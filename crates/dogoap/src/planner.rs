//! Types related to planning.

use crate::{
    action::Action,
    compare::{check_preconditions, compare_values},
    effect::Effect,
    goal::Goal,
    localstate::LocalState,
    mutator::{apply_mutator, format_mutators},
};

/// A Node holds things can return a state, used for path finding
/// It's either the Initial [`LocalState`], or the [`LocalState`] after applying
/// the [`Effect`]
#[derive(Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub enum Node {
    /// The initial state of the planner
    State(LocalState),
    /// The state after applying an [`Effect`]
    Effect(Effect),
}

impl Node {
    /// Returns the state of the node
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

fn successors<'a>(
    node: &'a Node,
    actions: &'a [Action],
) -> impl Iterator<Item = (Node, usize)> + 'a {
    let state = node.state();
    actions.iter().filter_map(move |action| {
        if !check_preconditions(state, action) || action.effects.is_empty() {
            return None;
        }
        let new_state = state.clone();
        let first_effect = &action.effects[0];

        let mut new_data = new_state.data.clone();
        for mutator in &first_effect.mutators {
            apply_mutator(&mut new_data, mutator);
        }

        let new_effect = Effect {
            action: first_effect.action.clone(),
            mutators: first_effect.mutators.clone(),
            cost: first_effect.cost,
            state: LocalState { data: new_data },
        };
        Some((Node::Effect(new_effect), first_effect.cost))
    })
}

fn is_goal(node: &Node, goal: &Goal) -> bool {
    goal.requirements.iter().all(|(key, value)| {
        let state_val = node.state().data.get(key).unwrap_or_else(|| {
            panic!(
                "Couldn't find key {key:#?} in LocalState {:#?}",
                node.state().data
            )
        });
        compare_values(value, state_val, node.state().data.clone())
    })
}

/// Currently, only [`PlanningStrategy::StartToGoal`] is supported, which tries to find the chain of
/// [`Effect`]s that lead to our [`Goal`] state
#[derive(Default, Copy, Clone, Debug)]
pub enum PlanningStrategy {
    #[default]
    /// `StartToGoal` begins with our current state, and finds the most optimal path to the goal, based on the costs
    /// Might take longer time than `GoalToStart`, but finds the path with the lowest cost
    StartToGoal,
}

/// Use [`make_plan`] instead
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
    }
}

/// Returns a path of [`Node`]s that leads from our start [`LocalState`] to our
/// [`Goal`] state
pub fn make_plan(
    start: &LocalState,
    actions: &[Action],
    goal: &Goal,
) -> Option<(Vec<Node>, usize)> {
    // Default to using Start -> Goal planning
    make_plan_with_strategy(PlanningStrategy::StartToGoal, start, actions, goal)
}

/// Returns an iterator of all [`Effect`]s from a given plan
pub fn get_effects_from_plan(plan: impl IntoIterator<Item = Node>) -> impl Iterator<Item = Effect> {
    plan.into_iter().filter_map(|node| match node {
        Node::Effect(effect) => Some(effect),
        Node::State(_) => None,
    })
}

/// Formats a human-readable version of a plan from [`make_plan`] that shows
/// what [`Action`]s needs to be executed and what the results of each Action is
#[must_use]
pub fn format_plan(plan: (Vec<Node>, usize)) -> String {
    let mut output = String::new();
    let nodes = plan.0;
    let cost = plan.1;
    let mut last_state = LocalState::new();
    for node in nodes {
        match node {
            Node::Effect(effect) => {
                output.push_str(&format!("\t\t= DO ACTION {:#?}\n", effect.action));
                output.push_str("\t\tMUTATES:\n");
                output.push_str(&format_mutators(effect.mutators));
                last_state = effect.state.clone();
            }
            Node::State(s) => {
                output.push_str("\t\t= INITIAL STATE\n");
                for (k, v) in &s.data {
                    output.push_str(&format!("\t\t{k} = {v}\n"));
                }
                last_state = s.clone();
            }
        }
        output.push_str("\n\t\t---\n");
    }
    output.push_str(&format!("\t\t= FINAL STATE (COST: {cost})\n"));
    for (k, v) in &last_state.data {
        output.push_str(&format!("\t\t{k} = {v}\n"));
    }

    output
}
