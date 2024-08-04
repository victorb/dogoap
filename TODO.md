## Examples

- [X] bevy_dogoap example should use builders
- [X] Make sure we can increment I64 just as an example
- [X] Maybe figure out `Sensors` or alike? Be able to change goal depending on how closer the player is - Make this into an example
- [X] Show how Observers could be used to handle cancelled/successful Actions

## Compare

- [X] is_goal does comparison directly, should be via `Compare`

## ActionState

- [ ] Active actions should have states and allow us to set them as Success/Failure/Executing accordingly
    - [ ] Is this really needed?
- [X] I think we might need `ActionState`, how to signal failure otherwise?
    - Ended up removing it, use Changed<> with the component action to see when it was removed

## Actions

- [X] Arguments to Actions?
    - Nope

## Field / Datum

- [ ] Be able to create Datum from Enum without using `usize`
    - Would probably at least require a derive macro, worth it? Probably
- [X] Rename `Field`, it's actually a Value and Field has a meaning in Rust already
    - New name: `Datum`
- [X] `From` API is a footgun that doesn't handle type safety very well, mixing Fields will make the compiler alright, but panic on runtime...

## UX

- [ ] Do one last investigation if we can get rid of the String Constants for action/state keys
    - Kind of simpler now...
- [ ] Improve derive so only `Component` + `ActionComponent` can be used, instead of also `Default` and `Clone`
    - Hmmmmmmm, not sure about this...
- [ ] Ensure that the macros (create_goal!, simple_action, create_action_map!, create_state!, and register_components!) follow idiomatic practices and provide clear documentation for their usage and functionality.
- [ ] Consider providing a more streamlined way to set up the planner, goals, and actions. For example, you could create a builder pattern or a setup function that takes care of creating the planner, inserting components, and adding the planner to the entity in a single step.
    - Addressed by some sort of DSL perhaps
- [X] Fix so we don't have to pass in a vec + hashmap of actions
- [X] `Goal` has both `new` and `build`, maybe not needed
- [X] Make an example which involves Transforms and t1.distance(t2)
- [X] Rename `State` to something else... || is now `LocalState`
- [X] Get rid of users having to do RegisterExt
- [X] Maybe write a macro for deriving things at the same time easily

## Documentation

- [ ] Document everything
- [ ] Write READMEs
- [ ] Compile reference list

## Misc

- [ ] Come up with a better name than "DOGOAP"?
    - This can come when it comes...
- [X] We want to be able to Query for values in our Planner, without involving the Planner, how can we do this? Or does it make sense to be able to individually Query for those?
    - [X] What if the planner was a struct with derive
- [X] Remove graph generation stuff