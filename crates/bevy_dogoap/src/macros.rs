#[macro_export]
macro_rules! create_planner {
    ({
        actions: [$(($action_type:ty, $action:expr)),* $(,)?],
        state: [$($state:expr),* $(,)?],
        goals: [$($goal:expr),* $(,)?],
    }) => {{
        let actions_map = create_action_map!($(($action_type, $action.clone())),*);

        let components = create_state!($($state.clone()),*);

        let planner = Planner::new(components, vec![$($goal.clone()),*], actions_map);

        let component_entities = ($($state.clone()),*);

        (planner, component_entities)
    }};
}

#[macro_export]
macro_rules! create_action_map {
    ($(($marker:ty, $action:expr)),* $(,)?) => {{
        use std::collections::HashMap;
        use bevy_dogoap::prelude::InserterComponent;
        let map: HashMap<String, (Action, Box<dyn InserterComponent>)> = HashMap::from([
            $(
                (
                    <$marker>::key(),
                    (
                        $action.clone(),
                        Box::new(<$marker>::default()) as Box<dyn InserterComponent>,
                    ),
                )
            ),*
        ]);
        map
    }};
}

#[macro_export]
macro_rules! create_state {
    ($( $x:expr ),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(Box::new($x) as Box<dyn DatumComponent>);
            )*
            temp_vec
        }
    };
}

#[macro_export]
macro_rules! register_components {
    ($app:ident, vec![$($comp:ty),*]) => {
        $(
            $app.register_component_as::<dyn DatumComponent, $comp>();
        )*
    };
}

#[macro_export]
macro_rules! create_goal {
    ($(($type:ident, $comp:path, $field:expr)),*) => {{
        let mut goal = Goal::new();

        $(
            goal = goal.with_req(&$type::key(), $comp($field));
        )*

        goal
    }};
}
