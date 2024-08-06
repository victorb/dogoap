- [ ] Active actions should have states and allow us to set them as Success/Failure/Executing accordingly
    - [ ] Is this really needed?
    - [ ] Open issue once public and ask for use cases that couldn't be solved without it

- [ ] `create_state!` to return `tuple` with components + state
- [ ] Improve derive so only `Component` + `ActionComponent` can be used, instead of also `Default` and `Clone`
    - Hmmmmmmm, not sure about this...
- [ ] Ensure that the macros (create_goal!, simple_action, create_action_map!, create_state!, and register_components!) follow idiomatic practices and provide clear documentation for their usage and functionality.
- [ ] Consider providing a more streamlined way to set up the planner, goals, and actions. For example, you could create a builder pattern or a setup function that takes care of creating the planner, inserting components, and adding the planner to the entity in a single step.
    - Addressed by some sort of DSL perhaps

## Documentation

- [ ] Document everything
- [ ] Write READMEs
- [ ] Compile reference list

## Misc

- [ ] Come up with a better name than "DOGOAP"?
    - This can come when it comes...
