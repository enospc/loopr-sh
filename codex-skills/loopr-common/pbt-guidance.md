# Property-based Testing Guidance

## Task generation
- If the feature marks PBT as Recommended or Optional, include task acceptance criteria for generators, properties, budgets, and seed logging.

## Test generation
- If PBT is Recommended and the framework is missing, stop and ask whether to select a framework or opt out.
- If PBT is Optional and the framework is missing, proceed with example-based tests and note the gap.
- If PBT is Not Suitable, use example-based tests only.
- If PBT is Recommended, include at least one property-based test referencing feature invariants and generator notes.
- If PBT is Optional, include a property-based test only if the framework is known; otherwise use example-based tests and note the optional PBT gap.
