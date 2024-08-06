- [ ] Active actions should have states and allow us to set them as Success/Failure/Executing accordingly
    - [ ] Is this really needed?
    - [ ] Open issue once public and ask for use cases that couldn't be solved without it

- [ ] `create_state!` to return `tuple` with components + state
- [ ] Improve derive so only `Component` + `ActionComponent` can be used, instead of also `Default` and `Clone`
    - Hmmmmmmm, not sure about this...
- [ ] Clear docs for macros about usage
- [ ] Figure out if we can abstract "creating the planner, inserting components, and adding the planner to entity" in a single step.
    - Addressed by some sort of DSL perhaps

## Documentation

- [ ] Document everything
- [ ] Write READMEs
- [ ] Compile reference list

## Misc

- [ ] Come up with a better name than "DOGOAP"?
    - This can come when it comes...

- [ ] Examples should have clear use cases we want to show off



--- Sort out the traits naming drama

    ActionBuilder
    ActionComponent
    ActionTrait

    DatumComponent
    EnumDatum

    GoalTrait
    MutatorTrait
    Precondition

    // Internal mostly
    InserterComponent



InserterComponent is really something we use to have our Datum inside of Bevy ECS, and syncing
the data from those to the LocalState of our planner

Should come up with a better name.





Why is there three action traits, ActionBuilder, ActionComponent and ActionTrait?


ActionTrait should just be impl directly on Action, need to replace some stuff

First replace cost [X]