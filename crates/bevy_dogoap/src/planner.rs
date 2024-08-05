use std::time::Instant;
use std::{collections::HashMap, fmt};

use bevy::prelude::*;
use bevy::tasks::futures_lite::future;
use bevy::tasks::{AsyncComputeTaskPool, Task};

use dogoap::prelude::*;

use crate::prelude::*;

type ActionsMap = HashMap<String, Action>;
type ActionsComponentsMap = HashMap<String, Box<dyn InserterComponent>>;
type ActionsCombinedMap = HashMap<String, (Action, Box<dyn InserterComponent>)>;

type DatumComponents = Vec<Box<dyn DatumComponent>>;

#[derive(Component, Reflect)]
pub struct Planner {
    pub state: LocalState,
    pub goals: Vec<Goal>,

    pub current_goal: Option<Goal>,

    pub current_action: Option<Action>,

    pub actions_map: ActionsMap,

    #[reflect(ignore)]
    pub components_map: ActionsComponentsMap,

    #[reflect(ignore)]
    pub datum_components: DatumComponents,

    // Some additional fields to control the execution
    /// If the Planner should try to always come up with new plans based on the current goal
    pub always_plan: bool,
    /// If the Planner should remove the current goal if it cannot find any plan to reach it
    pub remove_goal_on_no_plan_found: bool,
    actions_for_dogoap: Vec<Action>,
}

impl fmt::Debug for Planner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "State: {:#?}\nGoals: {:#?}\nActions: {:#?}\nCurrent Goal:{:#?}\n",
            self.state, self.goals, self.actions_for_dogoap, self.current_goal
        )
    }
}

/// This Component holds to-be-processed data for make_plan
/// We do it in a asyncronous manner as make_plan blocks and if it takes 100ms, we'll delay frames
/// by 100ms...
#[derive(Component)]
pub struct ComputePlan(Task<Option<(Vec<dogoap::prelude::Node>, usize)>>);

#[derive(Component)]
pub struct IsPlanning;

impl Planner {
    pub fn new(
        initial_state: DatumComponents,
        goals: Vec<Goal>,
        combined_map: ActionsCombinedMap,
    ) -> Self {
        let mut actions_for_dogoap: Vec<Action> = vec![];
        let mut components_map: ActionsComponentsMap = HashMap::new();
        let mut actions_map: ActionsMap = HashMap::new();

        for (key, (action, component)) in combined_map.iter() {
            actions_map.insert(key.clone().to_string(), action.clone());
            components_map.insert(key.clone().to_string(), component.clone_box());
            actions_for_dogoap.push(action.clone());
        }

        let mut ret = Self {
            state: LocalState::new(),
            datum_components: initial_state,
            current_goal: goals.first().cloned(),
            goals,
            actions_map,
            components_map,
            current_action: None,
            always_plan: true,
            remove_goal_on_no_plan_found: true,
            actions_for_dogoap,
        };
        ret.update_localstate();
        ret
    }

    pub fn update_localstate(&mut self) {
        let mut state = LocalState::new();
        for component in self.datum_components.iter() {
            state
                .data
                .insert(component.field_key(), component.field_value());
        }

        self.state = state;
    }

    pub fn insert_datum_components(&self, commands: &mut Commands, entity: Entity) {
        for datum in self.datum_components.iter() {
            datum.insert(commands, entity);
        }
    }
}

pub fn update_planner_local_state(
    local_field_components: Query<(Entity, &dyn DatumComponent)>,
    mut q_planner: Query<(Entity, &mut Planner)>,
) {
    for (entity, mut planner) in q_planner.iter_mut() {
        let (_c_entity, components) = local_field_components.get(entity).expect("Didn't find any DatumComponents, make sure you called register_components with all Components you want to use with the planner");
        for component in components {
            planner
                .state
                .data
                .insert(component.field_key(), component.field_value());
        }
    }
}

pub fn create_planner_tasks(
    mut commands: Commands,
    query: Query<(Entity, &Planner), Without<ComputePlan>>,
) {
    let thread_pool = AsyncComputeTaskPool::get();
    for (entity, planner) in query.iter() {
        if planner.always_plan {
            if let Some(goal) = planner.current_goal.clone() {
                let state = planner.state.clone();
                let actions = planner.actions_for_dogoap.clone();
                let task = thread_pool.spawn(async move {
                    let start = Instant::now();

                    // WARN this is the part that can sometimes be slow and why we use AsyncComputePool
                    let plan = make_plan(&state, &actions[..], &goal);
                    let duration_ms = start.elapsed().as_millis();

                    if duration_ms > 10 {
                        let steps = plan.clone().expect("plan was empty?!").0.len(); // Not very clever to clone if things are slow?
                        warn!("Planning duration for Entity {entity} was {duration_ms}ms for {steps} steps");
                    }

                    plan
                });
                commands
                    .entity(entity)
                    .insert((IsPlanning, ComputePlan(task)));
            }
        }
    }
}

pub fn handle_planner_tasks(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ComputePlan, &mut Planner)>,
) {
    for (entity, mut task, mut planner) in query.iter_mut() {
        if let Some(p) = future::block_on(future::poll_once(&mut task.0)) {
            commands.entity(entity).remove::<ComputePlan>();
            match p {
                Some((plan, _cost)) => {
                    let effects = get_effects_from_plan(plan);
                    match effects.first() {
                        Some(first_effect) => {
                            let action_name = first_effect.action.clone();

                            let found_action = planner.actions_map.get(&action_name).unwrap_or_else(|| panic!("Didn't find action {:?} registered in the Planner::actions_map", action_name));

                            if planner.current_action.is_some()
                                && Some(found_action) != planner.current_action.as_ref()
                            {
                                // We used to work towards a different action, so lets remove that one first.
                                // TODO remove specific one, but for now, remove all of them?
                                let found_component = planner
                                    .components_map
                                    .get(&planner.current_action.clone().unwrap().key)
                                    .unwrap();
                                found_component.remove(&mut commands, entity);
                            }

                            // TODO this is a bit horrible... Not only calling `.unwrap`, but the whole
                            // "do string match to find the right Component", slightly cursed
                            let found_component =
                                planner.components_map.get(&found_action.key).unwrap();
                            found_component.insert(&mut commands, entity);
                            planner.current_action = Some(found_action.clone());
                        }
                        None => {
                            if planner.remove_goal_on_no_plan_found {
                                debug!("Seems there is nothing to be done, removing current goal");
                                planner.current_goal = None;
                            }
                        }
                    }
                }
                None => {
                    warn!("Didn't find any plan for our goal in Entity {}!", entity);
                    // warn!("No plan found");
                }
            }
            commands.entity(entity).remove::<IsPlanning>();
        }
    }
}
