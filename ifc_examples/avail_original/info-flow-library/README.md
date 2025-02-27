# Carapace: Static-Dynamic Information Flow Control in Rust
Carapace is a Rust library that provides types and mechanisms for enforcing static-dynamic information flow control in Rust programs.

## Folder structure
- `ifc_examples`: contains the Avail case study, the microbenchmark discussed in the paper, and code used in the paper.
- `ifc_library`: contains all code defining Carpace's operation.

## Keywords
Code using Carapace will commonly use at least one of the following strings: side_effect_free_attr, InvisibleSideEffectFree, SecureValue, or secure_block.

## Requirements
Carapace depends on the *nightly* version of Rust.