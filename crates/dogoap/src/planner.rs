use crate::{
    action::Action,
    compare::{check_preconditions, compare_values},
    effect::Effect,
    goal::Goal,
    mutator::{print_mutators, Mutator},
    state::LocalState,
};

use bevy_reflect::Reflect;

#[derive(Reflect, Clone, Eq, PartialEq, Hash)]
pub enum Node {
    Effect(Effect),
    State(LocalState),
}

impl Node {
    pub fn state(&self) -> &LocalState {
        match self {
            Node::Effect(effect) => &effect.state,
            Node::State(state) => state,
        }
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Effect(effect) => effect.fmt(f),
            Node::State(state) => state.fmt(f),
        }
    }
}

fn heuristic(node: &Node, goal: &Goal) -> usize {
    node.state().distance_to_goal(goal) as usize
}

// TODO This function is fucking horrible
// Why I did the whole `action.options[0].0.action.clone()` thing?
fn successors<'a>(
    node: &'a Node,
    actions: &'a [Action],
) -> impl Iterator<Item = (Node, usize)> + 'a {
    let state = node.state();
    actions.iter().filter_map(move |action| {
        if check_preconditions(state, action) {
            let mut new_state = state.clone();
            if action.effects.len() == 0 {
                return None;
            }
            for mutator in &action.effects[0].mutators {
                match mutator {
                    Mutator::Set(key, value) => {
                        new_state.data.insert(key.to_string(), *value);
                    }
                    Mutator::Increment(key, value) => {
                        if let Some(current_value) = new_state.data.get_mut(key) {
                            *current_value += *value;
                        }
                    }
                    Mutator::Decrement(key, value) => {
                        if let Some(current_value) = new_state.data.get_mut(key) {
                            *current_value -= *value;
                        }
                    }
                }
            }
            let new_effect = Effect {
                action: action.effects[0].action.clone(),
                mutators: action.effects[0].mutators.clone(),
                cost: action.effects[0].cost,
                state: new_state,
            };
            Some((Node::Effect(new_effect), action.effects[0].cost))
        } else {
            None
        }
    })
}

fn is_goal(node: &Node, goal: &Goal) -> bool {
    goal.requirements.iter().all(|(key, value)| {
        if let Some(state_val) = node.state().data.get(key) {
            compare_values(value, state_val)
        } else {
            panic!("Couldn't find key {:#?} in LocalState", key);
        }
    })
}

/// We implement two different strategies for finding a solution
#[derive(Default)]
pub enum PlanningStrategy {
    #[default]
    /// StartToGoal begins with our current state, and finds the most optimal path to the goal, based on the costs
    /// Might take longer time than GoalToStart, but finds the path with the lowest cost
    StartToGoal,
    /// GoalToStart begins with the goal state, and works backwards from there, in order to find a path as quick as possible
    /// Might lead to less-than-optimial paths, but should find a valid path quicker
    GoalToStart,
}

pub fn make_plan_with_strategy(
    strategy: PlanningStrategy,
    start: &LocalState,
    actions: &[Action],
    goal: &Goal,
) -> Option<(Vec<Node>, usize)> {
    match strategy {
        PlanningStrategy::StartToGoal => {
            let start_node = Node::State(start.clone());
            pathfinding::directed::astar::astar(
                &start_node,
                |node| successors(node, actions).collect::<Vec<_>>().into_iter(),
                |node| heuristic(node, goal),
                |node| is_goal(node, goal),
            )
        }
        PlanningStrategy::GoalToStart => {
            panic!("PlanningStrategy::GoalToStart hasn't been implemented yet!");
        }
    }
}

pub fn make_plan(
    start: &LocalState,
    actions: &[Action],
    goal: &Goal,
) -> Option<(Vec<Node>, usize)> {
    // Default to using Start -> Goal planning
    make_plan_with_strategy(PlanningStrategy::StartToGoal, start, actions, goal)
}

pub fn get_effects_from_plan(plan: Vec<Node>) -> Vec<Effect> {
    let mut nodes = vec![];

    for node in plan {
        match node {
            Node::Effect(c) => nodes.push(c),
            Node::State(_s) => {}
        }
    }

    nodes
}

pub fn print_plan(plan: (Vec<Node>, usize)) {
    let nodes = plan.0;
    let cost = plan.1;
    let mut last_state = LocalState::new();
    for node in nodes {
        match node {
            Node::Effect(effect) => {
                println!("\t\t= DO ACTION {:#?}", effect.action);
                println!("\t\tMUTATES:");
                print_mutators(effect.mutators);
                last_state = effect.state.clone();
            }
            Node::State(s) => {
                println!("\t\t= INITIAL STATE");
                for (k, v) in &s.data {
                    println!("\t\t{} = {}", k, v);
                }
                last_state = s.clone();
            }
        }
        println!("\n\t\t---\n");
    }
    println!("\t\t= FINAL STATE (COST: {})", cost);
    for (k, v) in &last_state.data {
        println!("\t\t{} = {}", k, v);
    }
}
