# Carapace: Static-Dynamic Information Flow Control in Rust
Carapace is a Rust library that provides types and mechanisms for enforcing static-dynamic information flow control in Rust programs. Carapace does not currently address OS integration or leaks caused by other means such as side-channel attacks.

## Folder structure
- `ifc_examples`: contains the Avail case study, the microbenchmark discussed in the paper, and code used in the paper.
- `ifc_library`: contains all code defining Carpace's operation.

## Keywords
Code using Carapace will commonly use at least one of the following strings: side_effect_free_attr, InvisibleSideEffectFree, SecureValue, or secure_block.

## Requirements
Carapace depends on the *nightly* version of Rust.

## Implementation Overview
### Types
Carapace provides a series of types for programmers to use to mark secure data. The following table describes each type and its location in the source code.

| Type | Description | Location |
| ---- | ----------- | -------- |
| `SecureValue<T, L1, L2, D1, D2>` | A secrety value of type `T` with static secrecy policy `L1`, static integrity policy `L2`, dynamic secrecy label type `D1`, and dynamic integrity label type `D2`, where `T` is constrained to be `SecureValueSafe`, `D1` is constrained to be either `DynLabel<Sec>` or `()`, and `D2` is constrained to be either `DynLabel<Int>` or `()`. | `ifc_library/secret_structs/src/secret.rs`|
| `Label_A` | A static secrecy label composed of the policies $\{a\}$. The other defined labels are `Label_Empty`, `Label_B`, `Label_C`, `Label_AB`, `Label_BC`, `Label_ABC` | `ifc_library/secret_structs/src/ternary_lattice.rs` |
| `Label_NotX` | A static integrity label composed of the policies $\{y,z\}$. The other defined labels are `Label_All`, `Label_NotY`, `Label_NotZ`, `Label_NotXY`, `Label_NotXZ`, `Label_NotYZ`, `Label_NotXYZ` | `ifc_library/secret_structs/src/integrity_lattice.rs` |
| `DynLabel<Sec>` | A dynamic secrecy label | `ifc_library/secret_structs/src/secret.rs` |
| `DynLabel<Int>` | A dynamic secrecy label | `ifc_library/secret_structs/src/secret.rs` |
| `DynTag<Sec>` | A dynamic secrecy tag | `ifc_library/secret_structs/src/secret.rs` |
| `DynTag<Int>` | A dynamic integrity tag | `ifc_library/secret_structs/src/secret.rs` |

### Traits
Carapace provides several traits which constrain the types that are allowable in a `SecureValue` or a `secure_block` (see below). The following table describes each trait and its location in the source code.

| Trait | Description | Definition | Location |
| ----- | ----------- | ---------- | -------- |
| `VisibleSideEffectFree`  | Limits mutably captured values of secure blocks | `(¬(&mut _) ∨ (&mut Secret<_,_>)) ∧ ¬&InvisibleSideEffectFree)` | `ifc_library/secret_structs/src/secret.rs` |
| `SecureValueSafe` | Restricts `T` in `SecureValue<T, L1, L2, D1, D2>` to interior-immutable, block-safe types |  `Immutable ∧ InvisibleSideEffectFree` | `ifc_library/secret_structs/src/secret.rs` |
| `Immutable` | Types without interior mutability | `¬(UnsafeCell<_>) ∧ ¬(&mut _)` | `ifc_library/secret_structs/src/secret.rs` |
| `InvisibleSideEffectFree` | Types that can be used in secure blocks | Implented individually for built-in and application types | `ifc_library/secret_structs/src/secret.rs` |
| `MoreSecretThan` | Enforces a partial order on static secrecy labels. For example, `Label_AB` is `MoreSecretThan<Label_A>` | `L1` is `MoreSecretThan<L2>` $\Leftrightarrow$ $L2 \subseteq L1$| `ifc_library/secret_structs/src/lattice.rs` |
| `LowerIntegrityThan` | Enforces a partial order on static integrity labels. For example, `Label_NotXY` is `LowerIntegrityThan<Label_X>` | `L1` is `MoreSecretThan<L2>` $\Leftrightarrow$ $L2 \supseteq L1$| `ifc_library/secret_structs/src/lattice.rs` |

### Macros & Functions
Carapace contains numerous macros which expands application code using Carapace to insert compile-time checks to ensure IFC. Specifically, programmers use one of the `secure_block!` macros when operating on `SecureValue` values and Carapace inserts calls to the other functions listed here to ensure IFC compliance. All listed macros and functions are defined in `ifc_library/macros/src/lib.rs`. 

Each macro follows the general form `block!(L1, L2, D1, D2, { e } )`. This defines a lexically-scoped block for operating on `SecureValue` values, where `L1` is the ultimate static secrecy label, `L2` is the ultimate static integrity label, `D1` is the ultimate dynamic secrecy label, and `D2` is the ultimate dynamic integrity label that the application code, `e`, evaluates to. Depending on whether or not dynamic secrecy or integrity policies are being enforced, D1 and D2 are optional, as described in the table below.

An exception to the above format is macros used to partially declassify/endorse information, called `partial_trusted` blocks. These macros follow the form `block!(L1, L2, D1, D2, L3, L4, D3, D4, { e } )`. In this form, `L1` is the maximum static secrecy label, `L2` is the maximum static integrity label, `D1` is the maximum dynamic secrecy label, and `D1` is the maximum dynamic integrity label of data that can be accessed in the block. `L3` is the ultimate static secrecy label, `L4` is the ultimate static integrity label, `D3` is the ultimate dynamic secrecy label, and `D4` is the ultimate dynamic integrity label that the application code, `e`, evaluates to.

The appropriate macro to use is determined by what types of dynamic labels are in use, whether or not information is being fully or partially declassified, and whether information is being returned from the block. The table below shows how to choose which macro applies.

| ----- | No Dynamic Labels | Only Dynamic Secrecy | Only Dynamic Integrity | Both Dynamic Secrecy and Integrity |
| ----- | ----------- | ------- | -- | -- |
| **Return non-declassified values** | `untrusted_secure_block_static_all!(L1, L2, { e } )` | `untrusted_secure_block_dynamic_secrecy!(L1, L2, D1, { e } )` | `untrusted_secure_block_dynamic_integrity!(L1, L2, D2, { e } )` | `untrusted_secure_block_dynamic_all!(L1, L2, D1, D2 { e } )` |
| **Return no values** | `untrusted_secure_block_no_return_static_all!(L1, L2, { e } )` | `untrusted_secure_block_no_return_dynamic_secrecy!(L1, L2, D1, { e } )` | `untrusted_secure_block_no_return_dynamic_integrity!(L1, L2, D2, { e } )` | `untrusted_secure_block_no_return_dynamic_all!(L1, L2, D1, D2 { e } )` |
| **Return partially declassified/endorsed values** | `partial_trusted_secure_block_static_all!(L1, L2, L3, L4, { e } )` | `partial_trusted_secure_block_dynamic_secrecy!(L1, L2, D1, L3, L4, D3, { e } )` | `partial_trusted_secure_block_dynamic_integrity!(L1, L2, D2, L3, L4, D4, { e } )` | `partial_trusted_secure_block_dynamic_all!(L1, L2, D1, D2, L3, L4, D3, D4, { e } )` |
| **Return fully declassified/endorsed values** | `trusted_secure_block_static_all!(L1, L2, { e } )` | `trusted_secure_block_dynamic_secrecy!(L1, L2, D1, { e } )` | `trusted_secure_block_dynamic_integrity!(L1, L2, D2, { e } )` | `trusted_secure_block_dynamic_all!(L1, L2, D1, D2 { e } )` |

| Function | Description | 
| -------- | ----------- |
| `expand_and_check_expr(e, L)` | Transforms `e` to enforce that no side effects can occur | 
| `unwrap(e)`, `unwrap_ref(e)`, and `unwrap_mut_ref(e)` | Only callable from within a `secure_block`, returns the value of a `SecureValue` object
| `wrap(e)` | Creates a new `SecureValue<_,L1, L2, D1, D2>` with value `e` | 

## Examples & Case Studies
This repository provides several examples to demonstrate integration of Carapace with applications.

### Paper_Gradebook Example
Corresponding to the code used to motivate Carapace throughout the paper, this example demonstrates simple function calls from a hypothetical gradebook application.

This example can be found in `ifc_examples/paper_gradebook/src/main.rs` and `ifc_examples/paper_gradebook/src/grades.rs`. 

### Gradebook Microbenchmarks (Section 7)
The gradebook microbenchmark is a simple gradebook application inspired by the paper's motivating example and designed as a simple benchmark.

This example can be found in `ifc_examples/gradebook/src/main.rs` and `ifc_examples/gradebook/src/grades.rs`. 

### Avail Case Study (Section 8)
Avail is a terminal-based tool used to compute availabilities between two different calendars, one of which is Outlook, and one of which is Gmail. 

The original code for Avail is found in  `ifc_examples/avail_original`, and the version modified for Carapace is found in `ifc_examples/avail/avail_code`. Also, inside both `ifc_examples/avail_original` and `ifc_examples/avail` is the Chrono library used in Avail and modified to comply with Carapace requirements.

### Mk48.io Case Study (Section 9)
Mk48.io is a live multiplayer ship-combat game. At the time of writing, it has an active player base at the website mk48.io.

Due to its size, Mk48.io is not located within this repository. Instructions for accessing the Carapace-integrated Mk48.io code is located in `ifc_examples/mk48/README.md`. 

### Servo Case Study (Section 10)
Servo is Mozilla's browswer rendering engine written in Rust.

Due to its size, Servo is not located within this repository. Instructions for accessing the Carapace-integrated Servo code is located in `ifc_examples/servo/README.md`. 
