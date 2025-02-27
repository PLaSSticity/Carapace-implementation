#![feature(auto_traits, negative_impls, fn_traits, unboxed_closures, const_trait_impl, allocator_api, slice_index_methods, return_position_impl_trait_in_trait, panic_always_abort)]
//#![feature(min_specialization)]
pub mod secret;
pub mod ternary_lattice;
pub mod integrity_lattice;
pub mod allowlisted;

#[macro_use]
extern crate serde;