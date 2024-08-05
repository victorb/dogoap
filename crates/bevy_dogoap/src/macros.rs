
#[macro_export]
macro_rules! create_action_map {
    ($(($key:expr, $action:expr, $marker:ty)),* $(,)?) => {{
        use std::collections::HashMap;
        use bevy_dogoap::prelude::InserterComponent;
        let map: HashMap<String, (Action, Box<dyn InserterComponent>)> = HashMap::from([
            $(
                (
                    $key.to_string(),
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
macro_rules! create_action_map_v2 {
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