## Examples

- [X] bevy_dogoap example should use builders
- [X] Make sure we can increment I64 just as an example
- [X] Maybe figure out `Sensors` or alike? Be able to change goal depending on how closer the player is - Make this into an example
- [X] Show how Observers could be used to handle cancelled/successful Actions

## Compare

- [X] is_goal does comparison directly, should be via `Compare`

## ActionState

- [X] I think we might need `ActionState`, how to signal failure otherwise?
    - Ended up removing it, use Changed<> with the component action to see when it was removed
- [ ] Active actions should have states and allow us to set them as Success/Failure/Executing accordingly
    - [ ] Is this really needed?

## Actions

- [X] Arguments to Actions?
    - Nope

## Field / Datum

- [X] Rename `Field`, it's actually a Value and Field has a meaning in Rust already
    - New name: `Datum`
- [ ] `From` API is a footgun that doesn't handle type safety very well, mixing Fields will make the compiler alright, but panic on runtime...
- [ ] Be able to create Datum from Enum without using `usize`

## UX

- [X] Fix so we don't have to pass in a vec + hashmap of actions
- [ ] `Goal` has both `new` and `build`, maybe not needed
- [ ] Make an example which involves Transforms and t1.distance(t2)
- [ ] Do one last investigation if we can get rid of the String Constants for action/state keys
- [X] Maybe write a macro for deriving things at the same time easily
- [X] Rename `State` to something else... || is now `LocalState`
- [X] Get rid of users having to do RegisterExt
    - Kind of simpler now...
- [ ] Improve derive so only `Component` + `ActionComponent` can be used, instead of also `Default` and `Clone`
- [ ] Ensure that the macros (create_goal!, simple_action, create_action_map!, create_state!, and register_components!) follow idiomatic practices and provide clear documentation for their usage and functionality.
- [ ] Consider providing a more streamlined way to set up the planner, goals, and actions. For example, you could create a builder pattern or a setup function that takes care of creating the planner, inserting components, and adding the planner to the entity in a single step.

## Documentation

- [ ] Document everything
- [ ] Write READMEs
- [ ] Compile reference list

## Misc

- [X] We want to be able to Query for values in our Planner, without involving the Planner, how can we do this? Or does it make sense to be able to individually Query for those?
    - [X] What if the planner was a struct with derive
- [ ] Come up with a better name than "DOGOAP"?
- [ ] Remove graph generation stuff

## Tests

- [ ]