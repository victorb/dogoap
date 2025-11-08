//! Conveniece macros

/// Creates a [`Planner`](crate::prelude::Planner) from
/// - A list of actions
/// - A list of states
/// - A list of goals
#[macro_export]
macro_rules! create_planner {
    ({
        actions: [$(($action_type:ty, $action:expr)),* $(,)?],
        state: [$($state:expr),* $(,)?],
        goals: [$($goal:expr),* $(,)?],
    }) => {{
        use bevy::platform::collections::HashMap;
        use bevy_dogoap::prelude::InserterComponent;
        let actions_map: HashMap<String, (Action, Box<dyn InserterComponent>)> = HashMap::from([
            $(
                (
                    <$action_type>::key(),
                    (
                        $action.clone(),
                        Box::new(<$action_type>::default()) as Box<dyn InserterComponent>,
                    ),
                )
            ),*
        ]);

        let components = Vec::from([
            $(
                Box::new($state.clone()) as Box<dyn DatumComponent>,
            )*
        ]);

        let planner = Planner::new(components, vec![$($goal.clone()),*], actions_map);

        let component_entities = ($($state.clone()),*);

        (planner, component_entities)
    }};
}

/// Registers [`DatumComponent`](crate::prelude::DatumComponent)s into the type registry.
/// You need to call this macro and [`register_actions`] with all your relevant types on app startup for dogoap to function properly.
#[macro_export]
macro_rules! register_components {
    ($app:ident, [$($comp:ty),*]) => {
        $(
            $app.register_component_as::<dyn DatumComponent, $comp>();
        )*
    };
}

/// Registers [`ActionComponent`](crate::prelude::ActionComponent)s into the type registry.
/// You need to call this macro and [`register_components`] with all your relevant types on app startup for dogoap to function properly.
#[macro_export]
macro_rules! register_actions {
    ($app:ident, [$($comp:ty),*]) => {
        $(
            $app.register_component_as::<dyn ActionComponent, $comp>();
        )*
    };
}
