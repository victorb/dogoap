use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::{collections::HashMap, fmt};

use bevy::prelude::*;
use bevy::tasks::futures_lite::future;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use dogoap::prelude::*;

use crate::{InserterComponent, LocalFieldComponent};

type ActionsMap = HashMap<String, Action>;
type ActionsComponentsMap = HashMap<String, Box<dyn InserterComponent>>;
type ActionsCombinedMap = HashMap<String, (Action, Box<dyn InserterComponent>)>;

type LocalFieldComponents = Vec<Box<dyn LocalFieldComponent>>;

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
    pub field_components: LocalFieldComponents,

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
        // state: LocalState,
        initial_state: LocalFieldComponents,
        goals: Vec<Goal>,
        combined_map: ActionsCombinedMap,
        // components_map: HashMap<String, Box<dyn MarkerComponent>>,
    ) -> Self {
        // let actions_vec: Vec<Action> = actions_map.values().map(|v| v.0).cloned().collect();

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
            field_components: initial_state,
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
        // println!("Based On:");
        for component in self.field_components.iter() {
            // println!(
            //     "{:?} => {:?}",
            //     component.field_key(),
            //     component.field_value()
            // );
            state
                .fields
                .insert(component.field_key(), component.field_value());
        }
        // println!("Constructed the following state:\n{:#?}", state);

        self.state = state;
    }

    pub fn insert_field_components(&self, commands: &mut Commands, entity: Entity) {
        for field in self.field_components.iter() {
            field.insert(commands, entity);
        }
    }

    // // Need a function that queries for all the field components, and our local state of them so the planning
    // // can be updated
    // pub fn update_localstate(&mut self, world: &mut World, entity: Entity) {
    //     for field in self.field_components.iter_mut() {
    //         // let new_val = field.latest_value(world, entity);
    //         // field.set_value(new_val);
    //     }
    // }

    pub fn make_and_execute_plan(&mut self, commands: &mut Commands, entity: Entity) -> usize {
        // Vec<Action> is what dogoap::make_plan needs, hence we reformat it here
        // TODO should come up with a better way for sure
        // let actions: Vec<Action> = self.actions.values().cloned().collect();
        // self.update_localstate();

        if let Some(goal) = &self.current_goal {
            // let state = self.field_components_to_localstate();

            // This is what we need to separate from the rest.
            let plan: Option<(Vec<dogoap::prelude::Node>, usize)> =
                make_plan(&self.state, &self.actions_for_dogoap[..], goal);

            // println!("New plan:");
            // println!("=====================");
            // print_plan(plan.clone().unwrap());
            // println!("=====================");
            debug!("Planned effects:");
            print_plan(plan.clone().unwrap());

            // let effects = get_effects_from_plan(plan.unwrap().0);
            let effects = match plan {
                Some(plan) => get_effects_from_plan(plan.0),
                None => {
                    println!("Tried to reach Goal:\n{:#?}", goal);
                    println!("From current State:\n{:#?}", self.state);
                    println!("Available Actions:\n{:#?}", self.actions_for_dogoap);
                    println!("Available Components:\n{:#?}", self.components_map);
                    panic!("Didn't find any plan for our goal in Entity {}!", entity)
                }
            };

            // debug!("{:#?}", effects);
            // for effect in &effects {
            //     debug!("{:?}", effect.action);
            // }
            // println!("First");

            match effects.first() {
                Some(first_effect) => {
                    let action_name = first_effect.action.clone();
                    debug!("First action found: {:?}", action_name);

                    let found_action = self.actions_map.get(&action_name).unwrap();

                    if self.current_action.is_some()
                        && Some(found_action) != self.current_action.as_ref()
                    {
                        // We used to work towards a different action, so lets remove that one first.
                        // TODO remove specific one, but for now, remove all of them?
                        let found_component = self
                            .components_map
                            .get(&self.current_action.clone().unwrap().key)
                            .unwrap();
                        found_component.remove(commands, entity);
                    }

                    // println!("Action to execute:");
                    // println!("{:#?}", found_action);

                    // TODO this is a bit horrible... Not only calling `.unwrap`, but the whole
                    // "do string match to find the right Component", slightly cursed
                    let found_component = self.components_map.get(&found_action.key).unwrap();
                    found_component.insert(commands, entity);
                    self.current_action = Some(found_action.clone());
                }
                None => {
                    if self.remove_goal_on_no_plan_found {
                        debug!("Seems there is nothing to be done, removing current goal");
                        self.current_goal = None;
                    }
                }
            }
            effects.len()
        } else {
            debug!("No goal set, no need for planner to create plan...");
            0
        }
    }
}

pub fn update_planner_local_state(
    local_field_components: Query<(Entity, &dyn LocalFieldComponent)>,
    mut q_planner: Query<(Entity, &mut Planner)>,
) {
    // let mut system_state: SystemState<(
    //     Query<(Entity, &mut Planner)>,
    // )> = SystemState::new(world);

    // let mut res = system_state.get_mut(world);

    // for (entity, mut planner) in res.0.iter_mut() {
    //     // println!("I: {:#?}", i);
    //     planner.update_localstate(world, entity);
    // }

    // let mut query = world.query::<(Entity, &mut Planner)>();

    // for (entity, mut planner) in query.iter_mut(world) {
    //     // planner.update_localstate(world, entity);
    // }

    for (entity, mut planner) in q_planner.iter_mut() {
        let (_c_entity, components) = local_field_components.get(entity).expect("Didn't find any LocalFieldComponents, make sure you called register_components with all Components you want to use with the planner");
        for component in components {
            // println!(
            //     "\nComponent\n{:?} => {:?}\n",
            //     component.field_key(),
            //     component.field_value()
            // );
            planner
                .state
                .fields
                .insert(component.field_key(), component.field_value());
            // new_state = new_state.with_field(&component.field_key(), component.field_value());
        }
    }

    // for (entity, components) in &local_field_components {
    //     let mut planner = q_planner.get_mut(entity).unwrap();

    //     // let mut new_state = planner.state.clone();
    //     // let new_state

    //     println!("Updating local state from local_field_components");
    //     for component in components {
    //         println!("\nComponent\n{:?} => {:?}\n", component.field_key(), component.field_value());
    //         // for field in planner.field_components.iter_mut() {
    //             // println!("Field had value: {:?}", field.field_value());
    //         //     if field.field_key() == component.field_key() {
    //         //         field.set_value(component.field_value());
    //         //     }
    //         //     println!("Field now has value: {:?}", field.field_value());
    //         // }
    //         planner.state.fields.insert(component.field_key(), component.field_value());
    //         // new_state = new_state.with_field(&component.field_key(), component.field_value());
    //     }

    //     // planner.state = new_state;
    //     // planner.update_localstate();
    // }
}

pub fn update_planner(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Planner)>,
    mut last_micros: Local<VecDeque<u128>>,
) {
    for (entity, mut planner) in query.iter_mut() {
        // TODO implement some other heuristic for how often to make new plans
        if planner.always_plan {
            let start = Instant::now();

            // Simulate that this takes some time to calculate...
            std::thread::sleep(Duration::from_millis(250));

            let action_count = planner.make_and_execute_plan(&mut commands, entity);
            let duration = start.elapsed();

            last_micros.push_back(duration.as_micros());

            if last_micros.len() > 250 {
                last_micros.pop_front();
            }

            let avg_micros = if last_micros.is_empty() {
                0.0
            } else {
                last_micros.iter().sum::<u128>() as f64 / last_micros.len() as f64
            };

            debug!(
                "Time taken for planning {} actions: {} ms (avg: {:.0}μs) ({}μs)",
                action_count,
                duration.as_millis(),
                avg_micros,
                duration.as_micros()
            );
        }
    }
}

// First, query for planners
// Then, check if we have a goal
// Use that goal + state + actions to create a plan, this is the slow part
// Then, with the plan, perform a bunch of stuff

// One system that

pub fn create_planner_tasks(
    mut commands: Commands,
    query: Query<(Entity, &Planner), Without<ComputePlan>>,
    // mut query: Query<(Entity, &Planner)>,
    // mut in_progress: Local<HashMap<Entity, bool>>,
) {
    let thread_pool = AsyncComputeTaskPool::get();
    for (entity, planner) in query.iter() {
        if planner.always_plan {
            match planner.current_goal.clone() {
                // TODO shouldn't have clone here
                Some(goal) => {
                    // planner.update_localstate();
                    let state = planner.state.clone();
                    let actions = planner.actions_for_dogoap.clone();
                    // println!("Creating tasks");
                    let task = thread_pool.spawn(async move {
                        let start = Instant::now();
                        
                        // std::thread::sleep(Duration::from_millis(10));
                        // println!("Making plan");
                        let plan = make_plan(&state, &actions[..], &goal);
                        // std::thread::sleep(Duration::from_millis(1000));
                        // in_progress.insert(entity, false);
                        let duration_ms = start.elapsed().as_millis();
                        
                        if duration_ms > 10 {
                            let steps = plan.clone().expect("plan was empty?!").0.len(); // Not very clever to clone if things are slow...
                            warn!("Planning duration for Entity {entity} was {duration_ms}ms for {steps} steps");
                        }
                        
                        plan
                    });
                    commands.entity(entity).insert((IsPlanning, ComputePlan(task)));
                }
                None => {}
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
            // println!("Received task to handle");
            match p {
                Some((plan, cost)) => {
                    // println!("Plan with cost {} used", cost);
                    // debug!("Planned effects:");
                    // print_plan((plan.clone(), cost));

                    let effects = get_effects_from_plan(plan);
                    match effects.first() {
                        Some(first_effect) => {
                            let action_name = first_effect.action.clone();
                            // debug!("First action found: {:?}", action_name);

                            let found_action = planner.actions_map.get(&action_name).expect(&format!("Didn't find action {:?} registered in the Planner::actions_map", action_name));

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

                            // println!("Action to execute:");
                            // println!("{:#?}", found_action);

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
