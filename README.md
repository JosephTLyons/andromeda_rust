# andromeda

Quickly create large amounts of unique serial numbers.  Heavily
inspired by [random code generator](https://www.randomcodegenerator.com/en/generate-serial-numbers).

## Creating Unique Serial Numbers

Making a batch of unique licenses is very simple.  Let's say we want to create
1000 licenses, each having 20 characters and we want our licenses to make use of
numbers, lowercase letters, and uppercase letters.  We can simply call the
function:

```rust
generate_serial_numbers (1000, 20, true, true, true);
```

When we are done, we will find a new file within the same folder as this
application named `1000_unique_serials.txt`.  Opening the file, we see:

```text
dPzu8znblSPF1rmy4coW
djKD1ZxNLqAAo6ah6ZuA
dCZbvdHO9KIvm3CAnNCE
dzfN89poHQ7QMxZgGKH1
dqnj4G4A3mh23SpeoaZG
drsWemJsfg6aCzw0wHge
dYGS8jevEv5T0PFnd1bZ
dDSR4ZrQ2JgVZT2GmV4t
doO2eKULC3JLfuWBhQys
dp5yr9w1PrPpuAIWcGMH
dgFnW1Eppccc5gRNEFAR
dWosgmKFvdFtbKvCDB5X
dyTmrjnIhLK1pwhmXr9V
dMJQWVxqxRSjyt5lIOey
dwbPgKHTq5frkOzHJ0wP
dFheZibfSFeXVlPfzeSg
...
```
