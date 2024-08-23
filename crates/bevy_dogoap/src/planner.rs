#![cfg_attr(rustfmt, rustfmt_skip)]
use std::collections::VecDeque;

use std::{collections::HashMap, fmt};

use bevy::prelude::*;

#[cfg(feature = "compute-pool")]
use {
    std::time::Instant,
    bevy::tasks::futures_lite::future,
    bevy::tasks::{AsyncComputeTaskPool, Task}
};

use crate::prelude::*;
use dogoap::prelude::*;

// TODO can we replace this with ActionComponent perhaps? Should be able to
type ActionsMap = HashMap<String, (Action, Box<dyn InserterComponent>)>;

type DatumComponents = Vec<Box<dyn DatumComponent>>;

/// Our main struct for handling the planning within Bevy, keeping track of added
/// [`Action`]s, [`DatumComponent`]s, and some options for controlling the execution
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Planner {
    /// Our current state used for planning, updated by [`update_planner_local_state`] which reads
    /// the current state from our Bevy world and updates it accordingly
    pub state: LocalState,
    /// A Vector of all possible [`Goal`]s
    pub goals: Vec<Goal>,
    /// What [`Goal`] we're currently planning towards
    pub current_goal: Option<Goal>,
    /// What [`Action`] we're currrently trying to execute
    pub current_action: Option<Action>,

    // queue of action keys, first is current
    pub current_plan: VecDeque<String>,

    // TODO figure out how to get reflect to work, if possible
    #[reflect(ignore)]
    pub actions_map: ActionsMap,
    #[reflect(ignore)]
    pub datum_components: DatumComponents,

    // Some additional fields to control the execution
    /// If the Planner should try to always come up with new plans based on the current goal
    pub always_plan: bool,
    /// If the Planner should remove the current goal if it cannot find any plan to reach it
    pub remove_goal_on_no_plan_found: bool,
    /// plan_next_tick works like a toggle, that once you've set it to true, it'll make a new plan once
    /// then turn it to false. Combine with always_plan set to false and you can manually decide when
    /// new plans should be made.
    pub plan_next_tick: bool,

    /// Internal prepared vector of just [`Action`]
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

/// When we're not using AsyncComputeTaskPool + Task, we define our own so we can replace less code later
#[cfg(not(feature = "compute-pool"))]
struct Task<T>(T);

/// This Component holds to-be-processed data for make_plan
/// We do it in a asyncronous manner as make_plan blocks and if it takes 100ms, we'll delay frames
/// by 100ms...
#[derive(Component)]
pub struct ComputePlan(Task<Option<(Vec<dogoap::prelude::Node>, usize)>>);

/// This Component gets added when the planner for an Entity is currently planning,
/// and removed once a plan has been created. Normally this will take under 1ms,
/// but if you have lots of actions and possible states, it can take longer
#[derive(Component)]
pub struct IsPlanning;

impl Planner {
    pub fn new(components: DatumComponents, goals: Vec<Goal>, actions_map: ActionsMap) -> Self {
        let mut actions_for_dogoap: Vec<Action> = vec![];
        // let mut actions_map: ActionsMap = HashMap::new();

        for (_key, (action, _component)) in actions_map.iter() {
            actions_for_dogoap.push(action.clone());
        }

        let mut state = LocalState::new();

        for component in components.iter() {
            state
                .data
                .insert(component.field_key(), component.field_value());
        }

        Self {
            state,
            datum_components: components,
            current_goal: goals.first().cloned(),
            goals,
            actions_map,
            current_action: None,
            current_plan: VecDeque::new(),
            always_plan: true,
            remove_goal_on_no_plan_found: true,
            plan_next_tick: false,
            actions_for_dogoap,
        }
    }
}

/// This system "syncs" our [`DatumComponent`]s with the LocalState in the [`Planner`]
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

/// This system is responsible for finding [`Planner`]s that aren't alreay computing a new plan,
/// and creates a new task for generating a new plan
pub fn create_planner_tasks(
    mut commands: Commands,
    query: Query<(Entity, &Planner), Without<ComputePlan>>,
) {
    #[cfg(feature = "compute-pool")]
    let thread_pool = AsyncComputeTaskPool::get();

    for (entity, planner) in query.iter() {
        if planner.always_plan {
            if let Some(goal) = planner.current_goal.clone() {
                let state = planner.state.clone();
                let actions = planner.actions_for_dogoap.clone();

                #[cfg(feature = "compute-pool")]
                let task = thread_pool.spawn(async move {
                    let start = Instant::now();

                    // WARN this is the part that can be slow for large search spaces and why we use AsyncComputePool
                    let plan = make_plan(&state, &actions[..], &goal);
                    let duration_ms = start.elapsed().as_millis();

                    if duration_ms > 10 {
                        let steps = plan.clone().expect("plan was empty?!").0.len(); // Not very clever to clone if things are slow?
                        warn!("Planning duration for Entity {entity} was {duration_ms}ms for {steps} steps");
                    }

                    plan
                });

                #[cfg(not(feature = "compute-pool"))]
                let task = Task(make_plan(&state, &actions[..], &goal));

                commands
                    .entity(entity)
                    .insert((IsPlanning, ComputePlan(task)));
            }
        }
    }
}

#[cfg(not(feature = "compute-pool"))]
fn grab_plan_from_task(
    task: &mut Task<Option<(Vec<dogoap::prelude::Node>, usize)>>,
) -> Option<(Vec<dogoap::prelude::Node>, usize)> {
    task.0.clone()
}

/// This system is responsible for polling active [`ComputePlan`]s and switch the `current_action` if it changed
/// since last time. It'll add the [`ActionComponent`] as a Component to the same Entity the [`Planner`] is on, and
/// remove all the others, signalling that [`Action`] is currently active.
pub fn handle_planner_tasks(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ComputePlan, &mut Planner)>,
) {
    for (entity, mut task, mut planner) in query.iter_mut() {
        #[cfg(not(feature = "compute-pool"))]
        let p = grab_plan_from_task(&mut task.0);
        #[cfg(feature = "compute-pool")]
        let p = match future::block_on(future::poll_once(&mut task.0)) {
            Some(r) => r,
            None => continue,
        };

        commands.entity(entity).remove::<ComputePlan>();
        match p {
            Some((plan, _cost)) => {
                let effects = get_effects_from_plan(plan);

                let effect_names: VecDeque<String> =
                    effects.iter().map(|i| i.action.to_string()).collect();

                if planner.current_plan != effect_names {
                    planner.current_plan = effect_names.clone();
                    debug!(
                        "Current plan changed to: \n{:#?}\n(steps:{})",
                        effect_names,
                        effects.len()
                    );
                }

                match effects.first() {
                    Some(first_effect) => {
                        let action_name = first_effect.action.clone();

                        let (found_action, action_component) = planner.actions_map.get(&action_name).unwrap_or_else(|| panic!("Didn't find action {:?} registered in the Planner::actions_map", action_name));

                        if planner.current_action.is_some()
                            && Some(found_action) != planner.current_action.as_ref()
                        {
                            // We used to work towards a different action, so lets remove that one first.
                            // action_component.remove(&mut commands, entity);
                            // WARN remove all possible actions in order to avoid race conditions for now
                            for (_, (_, component)) in planner.actions_map.iter() {
                                component.remove(&mut commands, entity);
                            }
                        }

                        action_component.insert(&mut commands, entity);
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
