# Glossary

The `dogoap` API is for the standalone Rust library, it has nothing to do with Bevy.

The `bevy_dogoap` API however is all about Bevy, and integrates `dogoap` with various Bevy concepts.

## `dogoap` API

In order of appearance in a basic example.

### `LocalState`

This represents someone's/something's local state that they are aware of. This is considered the "starting state" for the planner when trying to figure out what actions to take to reach the Goal state. Also used within the `Effect` during planning to define the resulting state from applying the `Mutator`s from the `Effect`

### `Datum`

A Datum represents a type + value. Currently supports `bool`, `i64`, `f64` and `usize` (which represents `Enum`s).

### `Goal`

A Goal is the final state we want the planner to plan for.

### `Compare`

A Compare is used in Preconditions and Goals to indicate what we want a Datum in our LocalState to be. `Compare::GreaterThan(Datum::I64(10))` would mean we're looking to have a i64 result that is greater than 10. 

### `Action`

Action is built to be able to tell the planner what they could do to reach the final Goal state. The Action has Preconditions and Effects (currently only one `Effect` per `Action`).

#### `Precondition`

The Precondition tells the planner what the LocalState must look like before the Planner could take this Action.

#### `Effect`

The Effect encapsulates the `Mutator`s of applying the `Action`, has a `cost` associated with it and carries the `LocalState` of applying all the `Mutator`s during planning.

#### `Mutator`

Mutators are responsible for deciding what value should change, and by how much. The API currently supports `Set`, `Increment` and `Decrement`. `Set` works for all `Datum` types while `Incremnent`/`Decrement` only works for `I64` and `F64`.

#### `make_plan`

Main function responsible for actually coming up with a plan (list of Actions to reach the Goal).

##### Plan nodes `Node::Effect` and `Node::State`

Both of these structs encapsulate being able to get the state, in order to use path finding for finding the list of actions that reach to the goal state.

## `bevy_dogoap` API

### `DatumComponent`

Has to be a struct with arity 1, where argument is either a `bool`, `int64`, `f64` or `usize`.

```rust
#[derive(Component, Clone, DatumComponent)]
struct Hunger(f64);
```

Allows you to create `Compare` and `Mutator` structs from itself.

```rust
// Used as a Mutator:
assert_eq!(
    Hunger::increase(1.0),
    Mutator::Increment("hunger".to_string(), Datum::F64(2.0))
);
// Used as a Precondition:
assert_eq!(
    Hunger::is_less(10.0),
    ("hunger".to_string(), Compare::LessThanEquals(Datum::F64(10.0)))
)
```

### `ActionComponent`

A struct that is used to create a `Action` from it's own snake_cased name, to simplify usage with Bevy.

```rust
#[derive(ActionComponent)]
struct EatAction;

assert_eq!(EatAction::new(), Action::new("eat_action"));
```

### `EnumDatum`

Used to mark enums to be used with a `EnumComponent`

### `EnumComponent`

Basically the same as `DatumComponent` but allows you to transparently set/compare with the enum directly instead of having to do `myenum as usize` to work with `dogoap`. Meant to be used with `EnumDatum`

```rust
#[derive(EnumDatum)]
struct Location {
    Home,
    Outside
}

#[derive(EnumComponent)]
struct AtLocation(Location);

// Used as a Mutator:
assert_eq!(
    AtLocation::set(Location::Home),
    Mutator::Increment("at_location".to_string(), Datum::Enum(Location::Home as usize))
);

// Used as a Precondition:
assert_eq!(
    AtLocation::is(Location::Outside),
    ("at_location".to_string(), Compare::Equals(Datum::Enum(Location::Outside as usize)))
)
```

### `create_planner!` macro

This macro is responsible for setting everything up and return a `planner` that does the planning itself and `components` who are your `DatumComponent`s, basically the state of your Entity. These needs to be added to the Entity you want to control with dogoap. 

```rust
    let (mut planner, components) = create_planner!({
        actions: [
            (EatAction, eat_action),
            (GoToFoodAction, go_to_food_action),
            (ReplicateAction, replicate_action)
        ],
        state: [Hunger(starting_hunger), AtFood(false), IsReplicating(false)],
        goals: [goal],
    });
```

### `DogoapPlugin` Bevy Plugin

This plugin has to be added to your Bevy application as it's what actually makes the steps from the plan active in your Entities. 

### `register_components!` macro

In order for `DogoapPlugin` to be able to find all `DatumComponent`s, you'll need to register the components after doing `App::new()` but before calling `app.run()`.

```rust
let mut app = App::new();
register_components!(
    app,
    vec![Energy, AtLemonadeMaker, ServedOrder, ShouldGoToOrderDesk]
);
app.run();
```