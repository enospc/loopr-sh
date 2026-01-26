# Loopr Test Templates

Use the appropriate template below based on the chosen testing strategy.

## Standard Test Template
```
# Test: <short title>

## Test ID
<test_id>

## Type
<Unit | Integration | E2E | Manual>

## Purpose

## Preconditions
- 

## Test Data
- 

## Steps
1. 

## Expected Results
- 

## Automation Notes
- 
```

## Property-based Test Template
```
# Test: <short title>

## Test ID
<test_id>

## Type
Property-based

## Purpose

## Properties
- 

## Generators
- 

## Preconditions
- 

## Test Data
- 

## Steps
1. Run the property tests with the configured budget and seed.

## Expected Results
- All properties hold across generated cases.

## Automation Notes
- Framework: <library>
- Budget: <iterations/time>
- Seed / replay: <how to reproduce a failure>
- Shrinking: <notes if supported>
```
