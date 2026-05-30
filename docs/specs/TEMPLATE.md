# Crate Spec: bonsai-CRATE_NAME

## Replaces
`external-crate-a`, `external-crate-b`

## Used By
(list workspace crates that currently import the replaced deps)

## API Surface

```rust
// Paste the key public types, traits, and functions here.
// The factory agent will implement these exactly as specified.

pub struct Foo {
    // ...
}

pub fn bar(x: &Foo) -> Result<Baz, BonsaiError>;

pub trait MyTrait {
    fn method(&self) -> u64;
}
```

## Invariants
- Invariant 1: (e.g., "All outputs are valid UTF-8")
- Invariant 2: (e.g., "bar() is deterministic given the same input")
- Invariant 3: (e.g., "No panics on valid input")

## Performance Target
- `bar()` must complete in < Xµs on typical input
- Memory: O(n) where n is input size

## External Dependencies Allowed
(list any crates this replacement is allowed to depend on — usually none)
- none (pure std)

## Test Vectors
(for crypto/codec/parser crates, provide input→output test cases from specs/RFCs)
- Input: `"abc"` → Output: `0xba7816bf...` (SHA-256 reference)

## Migration Notes
Replace all call sites:
- `old_crate::Foo` → `bonsai_crate_name::Foo`
- `old_crate::bar(x)?` → `bonsai_crate_name::bar(x)?`

## Notes
(any implementation notes, tricky parts, or references to the original crate's source)
