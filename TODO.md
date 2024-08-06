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
    InserterComponent - Allows a Component to insert itself dynamically
    MutatorTrait
    Precondition
