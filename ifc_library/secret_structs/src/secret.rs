#![allow(non_snake_case)]

use crate::integrity_lattice as int_lat;
use crate::ternary_lattice as lattice;
use serde::{Deserialize, Deserializer};
use serde::ser::{Serialize, SerializeStruct};
use std::marker::PhantomData;
use std::borrow::Borrow;
use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, AtomicI64};
use std::time::Instant;
use std::iter::Fuse;
//Note: Code copied from static_assertions crate

#[macro_export]
macro_rules! assert_not_impl_all {
    ($x:ty: $($t:path),+ $(,)?) => {
        const _: fn() = || {
            // Generic trait with a blanket impl over `()` for all types.
            trait AmbiguousIfImpl<A> {
                // Required for actually being able to reference the trait.
                fn some_item() {}
            }

            impl<T: ?Sized> AmbiguousIfImpl<()> for T {}

            // Used for the specialized impl when *all* traits in
            // `$($t)+` are implemented.
            #[allow(dead_code)]
            struct Invalid;

            impl<T: ?Sized $(+ $t)+> AmbiguousIfImpl<Invalid> for T {}

            // If there is only one specialized trait impl, type inference with
            // `_` can be resolved and this can compile. Fails to compile if
            // `$x` implements `AmbiguousIfImpl<Invalid>`.
            let _ = <$x as AmbiguousIfImpl<_>>::some_item;
        };
    };
}

#[macro_export]
macro_rules! secret_block {
    ($t:ty $e:block) => {
        //secret_macros::secret_block!($t $e)
        secret_macros::untrusted_secure_block_static_all!($t, ::secret_structs::integrity_lattice::Label_All, $e)
    };
}
#[macro_export]
macro_rules! secret_block_no_return {
    ($t:ty $e:block) => {
        secret_macros::untrusted_secure_block_no_return_static_all!($t, ::secret_structs::integrity_lattice::Label_All, $e)
        //secret_macros::secret_block_no_return!($t $e)
    };
}

#[macro_export]
macro_rules! untrusted_secure_block_static_all {
    ($ts:ty, $ti:ty, $b:block) => {
        secret_macros::untrusted_secure_block_static_all!($ts, $ti, $b)
    };
}
#[macro_export]
macro_rules! untrusted_secure_block_dynamic_secrecy {
    ($ts:ty, $ti:ty, $e:expr, $b:block) => {
        secret_macros::untrusted_secure_block_dynamic_secrecy!($ts, $ti, $e, $b)
    };
}
#[macro_export]
macro_rules! untrusted_secure_block_dynamic_integrity {
    ($ts:ty, $ti:ty, $e:expr, $b:block) => {
        secret_macros::untrusted_secure_block_dynamic_integrity!($ts, $ti, $e, $b)
    };
}
#[macro_export]
macro_rules! untrusted_secure_block_dynamic_all {
    ($ts:ty, $ti:ty, $e1:expr, $e2:expr, $b:block) => {
        secret_macros::untrusted_secure_block_dynamic_all!($ts, $ti, $e1, $e2, $b)
    };
}

#[macro_export]
macro_rules! trusted_secure_block_static_all {
    ($ts:ty, $ti:ty, $b:block) => {
        secret_macros::trusted_secure_block_static_all!($ts, $ti, $b)
    };
}
#[macro_export]
macro_rules! trusted_secure_block_dynamic_secrecy {
    ($ts:ty, $ti:ty, $e:expr, $b:block) => {
        secret_macros::trusted_secure_block_dynamic_secrecy!($ts, $ti, $e, $b)
    };
}
#[macro_export]
macro_rules! trusted_secure_block_dynamic_integrity {
    ($ts:ty, $ti:ty, $e:expr, $b:block) => {
        secret_macros::trusted_secure_block_dynamic_integrity!($ts, $ti, $e, $b)
    };
}
#[macro_export]
macro_rules! trusted_secure_block_dynamic_all {
    ($ts:ty, $ti:ty, $e1:expr, $e2:expr, $b:block) => {
        secret_macros::trusted_secure_block_dynamic_all!($ts, $ti, $e1, $e2, $b)
    };
}

#[macro_export]
macro_rules! untrusted_secure_block_no_return_static_all {
    ($ts:ty, $ti:ty, $b:block) => {
        secret_macros::untrusted_secure_block_no_return_static_all!($ts, $ti, $b)
    };
}
#[macro_export]
macro_rules! untrusted_secure_block_no_return_dynamic_secrecy {
    ($ts:ty, $ti:ty, $e:expr, $b:block) => {
        secret_macros::untrusted_secure_block_no_return_dynamic_secrecy!($ts, $ti, $e, $b)
    };
}
#[macro_export]
macro_rules! untrusted_secure_block_no_return_dynamic_integrity {
    ($ts:ty, $ti:ty, $e:expr, $b:block) => {
        secret_macros::untrusted_secure_block_no_return_dynamic_integrity!($ts, $ti, $e, $b)
    };
}
#[macro_export]
macro_rules! untrusted_secure_block_no_return_dynamic_all {
    ($ts:ty, $ti:ty, $e1:expr, $e2:expr, $b:block) => {
        secret_macros::untrusted_secure_block_no_return_dynamic_all!($ts, $ti, $e1, $e2, $b)
    };
}

#[macro_export]
macro_rules! partial_trusted_secure_block_static_all {
    ($ts:ty, $ti:ty, $ts2:ty, $ti2:ty, $b:block) => {
        secret_macros::partial_trusted_secure_block_static_all!($ts, $ti, $ts2, $ti2, $b)
    };
}
#[macro_export]
macro_rules! partial_trusted_secure_block_dynamic_secrecy {
    ($ts:ty, $ti:ty, $e1:expr, $ts2:ty, $ti2:ty, $e2:expr, $b:block) => {
        secret_macros::partial_trusted_secure_block_dynamic_secrecy!($ts, $ti, $e1, $ts2, $ti2, $e2, $b)
    };
}
#[macro_export]
macro_rules! partial_trusted_secure_block_dynamic_integrity {
    ($ts:ty, $ti:ty, $e1:expr, $ts2:ty, $ti2:ty, $e2:expr, $b:block) => {
        secret_macros::partial_trusted_secure_block_dynamic_integrity!($ts, $ti, $e1, $ts2, $ti2, $e2, $b)
    };
}
#[macro_export]
macro_rules! partial_trusted_secure_block_dynamic_all {
    ($ts:ty, $ti:ty, $e1:expr, $e2:expr, $ts2:ty, $ti2:ty, $e3:expr, $e4:expr, $b:block) => {
        secret_macros::partial_trusted_secure_block_dynamic_all!($ts, $ti, $e1, $e2, $ts2, $ti2, $e3, $e4, $b)
    };
}

pub struct Counter {
    counter: AtomicI64
}

impl Counter {
    pub const fn initialize() -> Counter {
        Counter{counter: AtomicI64::new(0)}
    }
    pub fn increment(&mut self) {
        self.counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn get_counter(&self) -> i64 {
        self.counter.load(std::sync::atomic::Ordering::SeqCst)
    }
}

#[cfg(feature = "benchmarking")]
pub static mut TOTAL_BLOCK_COUNTER: Counter = Counter::initialize();

#[cfg(feature = "benchmarking")]
pub static mut TRUSTED_BLOCK_COUNTER: Counter = Counter::initialize();

#[cfg(feature = "benchmarking")]
pub static mut UNTRUSTED_BLOCK_COUNTER: Counter = Counter::initialize();

#[cfg(feature = "benchmarking")]
pub static mut WRAP_COUNTER: Counter = Counter::initialize();

#[cfg(feature = "benchmarking")]
pub static mut STRUCT_CLONE_COUNTER: Counter = Counter::initialize();

#[cfg(feature = "benchmarking")]
pub static mut UNWRAP_COUNTER: Counter = Counter::initialize();

#[cfg(feature = "benchmarking")]
pub static mut UNCHECKED_COUNTER: Counter = Counter::initialize();

#[cfg(feature = "benchmarking")]
pub static mut JOIN_COUNTER: Counter = Counter::initialize();

#[cfg(feature = "benchmarking")]
pub static mut LABEL_CLONE_COUNTER: Counter = Counter::initialize();

#[cfg(feature = "benchmarking")]
pub static mut NEW_TAG_COUNTER: Counter = Counter::initialize();

#[cfg(feature = "benchmarking")]
pub static mut NEW_LABEL_COUNTER: Counter = Counter::initialize();

#[cfg(feature = "benchmarking")]
pub static mut SUBSET_COUNTER: Counter = Counter::initialize();

#[cfg(feature = "benchmarking")]
pub static mut EQUAL_COUNTER: Counter = Counter::initialize();

#[cfg(feature = "benchmarking")]
pub static mut DIFFERENCE_COUNTER: Counter = Counter::initialize();

#[cfg(feature = "benchmarking")]
pub static mut CUSTOM_COUNTER_1: Counter = Counter::initialize();

#[cfg(feature = "benchmarking")]
pub static mut CUSTOM_COUNTER_2: Counter = Counter::initialize();

#[cfg(feature = "benchmarking")]
pub static mut CUSTOM_COUNTER_3: Counter = Counter::initialize();

#[cfg(feature = "benchmarking")]
pub static mut CUSTOM_COUNTER_4: Counter = Counter::initialize();

pub static mut TIMING: Option<Instant> = None;

#[cfg(feature = "benchmarking")]
pub unsafe fn print_counters() {
    eprintln!("total blocks: {}", TOTAL_BLOCK_COUNTER.get_counter());
    eprintln!("untrusted blocks: {}", UNTRUSTED_BLOCK_COUNTER.get_counter());
    eprintln!("trusted blocks: {}", TRUSTED_BLOCK_COUNTER.get_counter());
    eprintln!("wraps: {}", WRAP_COUNTER.get_counter());
    eprintln!("unwraps: {}", UNWRAP_COUNTER.get_counter());
    eprintln!("unchecked operations: {}", UNCHECKED_COUNTER.get_counter());
    eprintln!("joins: {}", JOIN_COUNTER.get_counter());
    eprintln!("new tags: {}", NEW_TAG_COUNTER.get_counter());
    eprintln!("new labels: {}", NEW_LABEL_COUNTER.get_counter());
    eprintln!("cloned labels: {}", LABEL_CLONE_COUNTER.get_counter());
    eprintln!("cloned ifc structs: {}", STRUCT_CLONE_COUNTER.get_counter());
    eprintln!("subset checks: {}", SUBSET_COUNTER.get_counter());
    eprintln!("equal checks: {}", EQUAL_COUNTER.get_counter());
    eprintln!("label difference calculations: {}", DIFFERENCE_COUNTER.get_counter());
    /*eprintln!("custom counter: {}", CUSTOM_COUNTER_1.get_counter());
    eprintln!("custom counter: {}", CUSTOM_COUNTER_2.get_counter());
    eprintln!("custom counter: {}", CUSTOM_COUNTER_3.get_counter());
    eprintln!("custom counter: {}", CUSTOM_COUNTER_4.get_counter());*/
}

#[cfg(feature = "benchmarking")]
pub extern "C" fn print_counters_c() {
    unsafe {print_counters();}
}

extern crate libc;
#[cfg(feature = "benchmarking")]
pub fn atexit_print_counters() {
    unsafe { libc::atexit(print_counters_c); }
}

pub unsafe fn print_timing() {
    let elapsed = unsafe { TIMING.unwrap().elapsed() };
    eprintln!("Elapsed time: {} s, {} ns", elapsed.as_secs(), elapsed.subsec_nanos());
}

pub extern "C" fn print_timing_c() {
    unsafe {print_timing();}
}

pub fn atexit_print_timing() {
    unsafe { libc::atexit(print_timing_c); }
}

// TODO: fix data races?
pub static mut PRINCIPAL_ADD_SECRECY: DynLabel<Sec> = DynLabel::<Sec>::new_default();
pub static mut PRINCIPAL_REMOVE_SECRECY: DynLabel<Sec> = DynLabel::<Sec>::new_default();
pub static mut PRINCIPAL_ADD_INTEGRITY: DynLabel<Int> = DynLabel::<Int>::new_default();
pub static mut PRINCIPAL_REMOVE_INTEGRITY: DynLabel<Int> = DynLabel::<Int>::new_default();
static mut NEXT_SECRECY_TAG: AtomicU32 = AtomicU32::new(Sec::DEFAULT_TAG + Sec::NEXT_TAG_INCREMENT);
static mut NEXT_INTEGRITY_TAG: AtomicU32 = AtomicU32::new(Int::DEFAULT_TAG + Int::NEXT_TAG_INCREMENT);

const DEFAULT_SECRECY_LABEL_REF: &DynLabel<Sec> = &DynLabel::<Sec>::new_default();
const DEFAULT_INTEGRITY_LABEL_REF: &DynLabel<Int> = &DynLabel::<Int>::new_default();

pub fn initialize_carapace() {
    #[cfg(feature = "benchmarking")]
    atexit_print_counters();
    unsafe {TIMING = Some(Instant::now());}
    atexit_print_timing();
    #[cfg(not(debug_assertions))]
    std::panic::always_abort();
}

pub fn get_new_secrecy_tag() -> DynamicTag<Sec> {
    unsafe {
        #[cfg(feature = "benchmarking")]
        unsafe { NEW_TAG_COUNTER.increment(); }
        let tag_val = NEXT_SECRECY_TAG.fetch_add(Sec::NEXT_TAG_INCREMENT, std::sync::atomic::Ordering::SeqCst);
        PRINCIPAL_ADD_SECRECY = DynLabel::<Sec>::new_size_one(DynamicTag::<Sec>{value: tag_val, p: PhantomData}).join(&PRINCIPAL_ADD_SECRECY); //OPTIMIZE
        PRINCIPAL_REMOVE_SECRECY = DynLabel::<Sec>::new_size_one(DynamicTag::<Sec>{value: tag_val, p: PhantomData}).join(&PRINCIPAL_REMOVE_SECRECY); //OPTIMIZE
        DynamicTag::<Sec>{value: tag_val, p: PhantomData}
    }
}

pub fn get_new_integrity_tag() -> DynamicTag<Int> {
    unsafe {
        #[cfg(feature = "benchmarking")]
        unsafe { NEW_TAG_COUNTER.increment(); }
        let tag_val = NEXT_INTEGRITY_TAG.fetch_add(Int::NEXT_TAG_INCREMENT, std::sync::atomic::Ordering::SeqCst);
        PRINCIPAL_ADD_INTEGRITY = DynLabel::<Int>::new_size_one(DynamicTag::<Int>{value: tag_val, p: PhantomData}).join(&PRINCIPAL_ADD_INTEGRITY); //OPTIMIZE
        PRINCIPAL_REMOVE_INTEGRITY = DynLabel::<Int>::new_size_one(DynamicTag::<Int>{value: tag_val, p: PhantomData}).join(&PRINCIPAL_REMOVE_INTEGRITY); //OPTIMIZE
        DynamicTag::<Int>{value: tag_val, p: PhantomData}
    }
}


pub struct Wrapped<T> {
    _pd: PhantomData<T>,
}

#[inline(always)]
pub fn call_info_flow_closure<L1, L2, F, R>(clos: F) -> R
where
    F: FnOnce() -> R + VisibleSideEffectFree,
    R: InfoFlowStaticTrait<L1, L2>,
{
    clos()
}

#[inline(always)]
pub fn call_info_flow_closure_declassify<L1, L2, F, R>(clos: F) -> R
where
    F: FnOnce() -> R + VisibleSideEffectFree,
{
    clos()
}

#[inline(always)]
pub fn call_info_flow_closure_no_return<L1, L2, F>(clos: F)
where
    F: FnOnce() + VisibleSideEffectFree,
{
    clos()
}

pub unsafe trait InfoFlowStaticTrait<L1, L2> {}
unsafe impl<T: SecureValueSafe, LS, LI, L1, L2, D1, D2> InfoFlowStaticTrait<LS, LI> for SecureValue<T, L1, L2, D1, D2> where
    LS: lattice::MoreSecretThan<L1>, LI: int_lat::LowerIntegrityThan<L2>, D1: DynField<Sec>, D2: DynField<Int>
{
}

unsafe impl<T: SecureValueSafe, LS, LI, L1, L2, D1, D2> InfoFlowStaticTrait<LS, LI> for &SecureValue<T, L1, L2, D1, D2> where
    LS: lattice::MoreSecretThan<L1>, LI: int_lat::LowerIntegrityThan<L2>, D1: DynField<Sec>, D2: DynField<Int>
{
}

unsafe impl<T1: SecureValueSafe, T2: SecureValueSafe, LS, LI, L1, L2, L3, L4, D1, D2, D3, D4> InfoFlowStaticTrait<LS, LI>
    for (SecureValue<T1, L1, L2, D1, D2>, SecureValue<T2, L3, L4, D3, D4>)
where
    LS: lattice::MoreSecretThan<L1> + lattice::MoreSecretThan<L3>,
    LI: int_lat::LowerIntegrityThan<L2> + int_lat::LowerIntegrityThan<L4>,
    D1: DynField<Sec>, D2: DynField<Int>, D3: DynField<Sec>, D4: DynField<Int>
{
}

unsafe impl<T1: SecureValueSafe, T2: SecureValueSafe, T3: SecureValueSafe, LS, LI, L1, L2, L3, L4, L5, L6, D1, D2, D3, D4, D5, D6> InfoFlowStaticTrait<LS, LI>
    for (SecureValue<T1, L1, L2, D1, D2>, SecureValue<T2, L3, L4, D3, D4>, SecureValue<T3, L5, L6, D5, D6>)
where
    LS: lattice::MoreSecretThan<L1> + lattice::MoreSecretThan<L3> + lattice::MoreSecretThan<L5>,
    LI: int_lat::LowerIntegrityThan<L2> + int_lat::LowerIntegrityThan<L4> + int_lat::LowerIntegrityThan<L6>,
    D1: DynField<Sec>, D2: DynField<Int>, D3: DynField<Sec>, D4: DynField<Int>, D5: DynField<Sec>, D6: DynField<Int>
{
}

pub unsafe auto trait NotSecret {}
impl<T: ?Sized, L> !NotSecret for Secret<T, L> {}
unsafe auto trait WrappedNotInvisibleSideEffectFree {}
impl<T: InvisibleSideEffectFree> !WrappedNotInvisibleSideEffectFree for Wrapped<T> {}

pub unsafe auto trait VisibleSideEffectFree {} // For limiting what secret block closures can capture
//unsafe impl VisibleSideEffectFree for dyn FnOnce() {} // This doesn't seem to have any purpose
impl<T> !VisibleSideEffectFree for &T where Wrapped<T>: WrappedNotInvisibleSideEffectFree {}
impl<T: NotSecret> !VisibleSideEffectFree for &mut T {}
impl<T: NotSecret> !VisibleSideEffectFree for *mut T {}
impl<T: NotSecret> !VisibleSideEffectFree for std::cell::UnsafeCell<T> {}
unsafe impl<T: InvisibleSideEffectFree> VisibleSideEffectFree for &T {}
unsafe impl VisibleSideEffectFree for &str {}
unsafe impl<T: SecureValueSafe, L1, L2, D1: DynField<Sec>, D2: DynField<Int>> VisibleSideEffectFree for &mut SecureValue<T, L1, L2, D1, D2> {}
unsafe impl<T: SecureValueSafe, L1, L2, D1: DynField<Sec>, D2: DynField<Int>> VisibleSideEffectFree for &mut &mut SecureValue<T, L1, L2, D1, D2> {}

#[inline(always)]
pub fn not_mut_secret<T>(x: &mut T) -> &mut T
    where T: NotSecret { x }

// This struct represents the return value of a function guaranteed not to leak certain kinds of information.
struct Vetted<T> where T: InvisibleSideEffectFree {
    item: T,
}

impl<T> Vetted<T> where T: InvisibleSideEffectFree {
    // Marks a return value as side-effect free.
    /*#[inline(always)]
    unsafe fn wrap(item: T) -> Self {
        Vetted::<T> { item }
    }*/

    // Extracts the return value.
    /*#[inline(always)]
    unsafe fn unwrap(self) -> T {
        self.item
    }*/
}

pub unsafe trait VettedTrait<T> where T: InvisibleSideEffectFree {
    unsafe fn unwrap(self) -> T;
}

unsafe impl<T: InvisibleSideEffectFree> VettedTrait<T> for Vetted<T> {
    unsafe fn unwrap(self) -> T {
        self.item
    }
}

pub const unsafe fn vetted_wrap<T: InvisibleSideEffectFree>(item: T) -> impl VettedTrait<T> {
    Vetted::<T> { item }
}
  
pub unsafe trait InvisibleSideEffectFree {
    // Limits what can be used in secret blocks
    #[inline(always)]
    unsafe fn check_all_types() {} // Overrided by #[derive(InvisibleSideEffectFree)]
}

#[inline(always)]
pub fn check_type_is_secret_block_safe<T: InvisibleSideEffectFree>() {}

#[inline(always)]
pub fn check_ISEF<T: InvisibleSideEffectFree>(x: T) -> T {
    //std::ptr::read(x)
    x
}

#[inline(always)]
pub unsafe fn check_ISEF_unsafe<T: InvisibleSideEffectFree>(x: &T) -> T {
    std::ptr::read(x)
}

#[inline(always)]
pub fn check_expr_secret_block_safe_ref<T: InvisibleSideEffectFree>(x: &T) -> &T
    where T: ?Sized {
    x
}

#[inline(always)]
pub fn check_ISEF_mut_ref<T: InvisibleSideEffectFree>(x: &mut T) -> &mut T
    where T: ?Sized {
    x
}

// Usage: check_safe_index_expr(e)[check_safe_index(i)]
// Checks that e and i have types such that std::ops::Index<I> for E
#[inline(always)]
pub fn check_safe_index_expr<E: SafeIndexExpr>(e: E) -> E {
    e
}

#[inline(always)]
pub fn check_safe_index<I: SafeIndex>(i: I) -> I {
    i
}

#[inline(always)]
pub fn check_safe_range_bounds<B: SafeRangeBounds>(b: B) -> B {
    b
}

unsafe impl InvisibleSideEffectFree for () {}
unsafe impl<T> InvisibleSideEffectFree for DynLabel<T> where T: LabelType {}
unsafe impl<T: SecureValueSafe, L1, L2, D1: DynField<Sec>, D2: DynField<Int>> InvisibleSideEffectFree for SecureValue<T, L1, L2, D1, D2> {}
unsafe impl<T: InvisibleSideEffectFree, U: InvisibleSideEffectFree> InvisibleSideEffectFree for (T, U) {}
unsafe impl<T: InvisibleSideEffectFree, U: InvisibleSideEffectFree, V: InvisibleSideEffectFree> InvisibleSideEffectFree for (T, U, V) {}
unsafe impl<T: InvisibleSideEffectFree> InvisibleSideEffectFree for Box<T> {}
unsafe impl<T: InvisibleSideEffectFree> InvisibleSideEffectFree for Vec<T> {}
unsafe impl<T: InvisibleSideEffectFree> InvisibleSideEffectFree for Fuse<std::vec::IntoIter<T>> {}
unsafe impl InvisibleSideEffectFree for f32 {}
unsafe impl InvisibleSideEffectFree for f64 {}
unsafe impl InvisibleSideEffectFree for isize {}
unsafe impl InvisibleSideEffectFree for i8 {}
unsafe impl InvisibleSideEffectFree for i16 {}
unsafe impl InvisibleSideEffectFree for i32 {}
unsafe impl InvisibleSideEffectFree for i64 {}
unsafe impl InvisibleSideEffectFree for i128 {}
unsafe impl InvisibleSideEffectFree for u8 {}
unsafe impl InvisibleSideEffectFree for u16 {}
unsafe impl InvisibleSideEffectFree for u32 {}
unsafe impl InvisibleSideEffectFree for u64 {}
unsafe impl InvisibleSideEffectFree for u128 {}
unsafe impl InvisibleSideEffectFree for usize {}
unsafe impl InvisibleSideEffectFree for String {}
unsafe impl InvisibleSideEffectFree for str {}
unsafe impl InvisibleSideEffectFree for &str {}
unsafe impl InvisibleSideEffectFree for core::num::NonZeroI32 {}
unsafe impl InvisibleSideEffectFree for std::str::Chars<'_> {}
unsafe impl InvisibleSideEffectFree for std::str::Split<'_, char> {}
unsafe impl InvisibleSideEffectFree for std::str::CharIndices<'_> {}
unsafe impl InvisibleSideEffectFree for std::num::ParseIntError {}
unsafe impl InvisibleSideEffectFree for std::num::ParseFloatError {}
unsafe impl InvisibleSideEffectFree for unicode_segmentation::UWordBounds<'_> {}
unsafe impl<T: InvisibleSideEffectFree> InvisibleSideEffectFree for *mut T {}
unsafe impl<T: InvisibleSideEffectFree> InvisibleSideEffectFree for Option<T> {}
unsafe impl<T: InvisibleSideEffectFree, E: InvisibleSideEffectFree> InvisibleSideEffectFree for Result<T, E> {}
unsafe impl<T: InvisibleSideEffectFree> InvisibleSideEffectFree for &T {}
unsafe impl<T: InvisibleSideEffectFree> InvisibleSideEffectFree for &mut T {}
unsafe impl InvisibleSideEffectFree for char {}
unsafe impl InvisibleSideEffectFree for bool {}
unsafe impl InvisibleSideEffectFree for PathBuf {}
unsafe impl InvisibleSideEffectFree for glam::Vec2 {}
unsafe impl<T: InvisibleSideEffectFree> InvisibleSideEffectFree for [T] {}
unsafe impl<T: InvisibleSideEffectFree> InvisibleSideEffectFree for &[T] {}
unsafe impl<T: InvisibleSideEffectFree> InvisibleSideEffectFree for &mut [T] {}
unsafe impl<T: InvisibleSideEffectFree, const N: usize> InvisibleSideEffectFree for [T; N] {}
unsafe impl<T: InvisibleSideEffectFree> InvisibleSideEffectFree for *const T {}
#[cfg(target_arch = "x86_64")]
unsafe impl InvisibleSideEffectFree for std::arch::x86_64::__m256d {}
#[cfg(target_arch = "x86_64")]
unsafe impl InvisibleSideEffectFree for std::arch::x86_64::__m128 {}
unsafe impl<T: InvisibleSideEffectFree> InvisibleSideEffectFree for std::iter::Copied<T> {}
unsafe impl<T: InvisibleSideEffectFree> InvisibleSideEffectFree for std::iter::Cycle<T> {}
unsafe impl<T: InvisibleSideEffectFree> InvisibleSideEffectFree for std::iter::Enumerate<T> {}
unsafe impl<T: InvisibleSideEffectFree> InvisibleSideEffectFree for std::iter::Rev<T> {}
unsafe impl<T: InvisibleSideEffectFree> InvisibleSideEffectFree for std::iter::Take<T> {}
unsafe impl<T: InvisibleSideEffectFree> InvisibleSideEffectFree for std::vec::IntoIter<T> {}
unsafe impl<'a> InvisibleSideEffectFree for unicode_segmentation::Graphemes<'a> {}
unsafe impl<'a, T: InvisibleSideEffectFree> InvisibleSideEffectFree for std::slice::Iter<'a, T> {}
unsafe impl<'a, F: FnMut(char) -> bool/*F: FnOnce() -> R + VisibleSideEffectFree*/> InvisibleSideEffectFree for std::str::Split<'a, F> {}
unsafe impl<T: InvisibleSideEffectFree> InvisibleSideEffectFree for std::mem::MaybeUninit<T> {}
unsafe impl<T: InvisibleSideEffectFree> InvisibleSideEffectFree for std::ops::Range<T> {}
unsafe impl<K: InvisibleSideEffectFree, V: InvisibleSideEffectFree> InvisibleSideEffectFree for std::collections::HashMap<K, V>  {}
unsafe impl<K: InvisibleSideEffectFree> InvisibleSideEffectFree for std::collections::HashSet<K>  {}
// TODO: lots more

/**
 * Struct wrapper containing a secret of type T with secrecy level L
 * Note: PhantomData<L> is just to fix issue that L is otherwise unused.
 * This may not be the best solution.
 */
#[derive(Serialize, Deserialize)]
pub struct SecureValue<T, L1, L2, D1, D2>
where
    T: SecureValueSafe, D1 : DynField<Sec>, D2: DynField<Int>
{
    val: T,
    _pd_secrecy: PhantomData<L1>,
    _pd_integrity: PhantomData<L2>,
    dynamic_secrecy: D1,
    dynamic_integrity: D2
}

impl<T: Clone, L1, L2, D1, D2> Clone for SecureValue<T, L1, L2, D1, D2>
where
    T: SecureValueSafe, D1 : DynField<Sec>, D2: DynField<Int>
{
    #[inline]
    fn clone(&self) -> Self {
        #[cfg(feature = "benchmarking")]
        unsafe { STRUCT_CLONE_COUNTER.increment(); CUSTOM_COUNTER_1.increment(); CUSTOM_COUNTER_1.increment(); }
        SecureValue {
            val: self.val.clone(),
            _pd_secrecy: PhantomData,
            _pd_integrity: PhantomData,
            dynamic_secrecy: self.dynamic_secrecy.clone(),
            dynamic_integrity: self.dynamic_integrity.clone(),
        }
    }
}

pub type StaticSecret<T, L> = SecureValue<T, L, int_lat::Label_All, (), ()>;
pub type DynamicSecret<T, D> = SecureValue<T, lattice::Label_Empty, int_lat::Label_All, D, ()>;
pub type StaticIntegrity<T, L> = SecureValue<T, lattice::Label_Empty, L, (), ()>;
pub type DynamicIntegrity<T, D> = SecureValue<T, lattice::Label_Empty, int_lat::Label_All, (), D>;
pub type StaticDynamicSecret<T, L, D> = SecureValue<T, L, int_lat::Label_All, D, ()>;
pub type StaticAll<T, L1, L2> = SecureValue<T, L1, L2, (), ()>;
pub type StaticSecretDynamicIntegrity<T, L, D> = SecureValue<T, L, int_lat::Label_All, (), D>;
pub type StaticDynamicIntegrity<T, L, D> = SecureValue<T, lattice::Label_Empty, L, (), D>;
pub type DynamicSecretStaticIntegrity<T, L, D> = SecureValue<T, lattice::Label_Empty, L, D, ()>;
pub type DynamicAll<T, D1, D2> = SecureValue<T, lattice::Label_Empty, int_lat::Label_All, D1, D2>;
pub type StaticDynamicSecretStaticIntegrity<T, L1, L2, D> = SecureValue<T, L1, L2, D, ()>;
pub type StaticDynamicSecretDynamicIntegrity<T, L, D1, D2> = SecureValue<T, L, int_lat::Label_All, D1, D2>;
pub type StaticSecretStaticDynamicIntegrity<T, L1, L2, D> = SecureValue<T, L1, L2, (), D>;
pub type DynamicSecretStaticDynamicIntegrity<T, L, D1, D2> = SecureValue<T, lattice::Label_Empty, L, D1, D2>;
pub type StaticDynamicAll<T, L1, L2, D1, D2> = SecureValue<T, L1, L2, D1, D2>;

pub type Secret<T, L> = StaticSecret<T, L>;

#[derive(Clone, Copy, Hash)]
pub struct Sec {}

#[derive(Clone, Copy, Hash)]
pub struct Int {}

pub trait LabelType: Clone + Copy + std::hash::Hash {
    const AM_SECRET_TYPE: bool;
    const DEFAULT_TAG: u32;
    const NEXT_TAG_INCREMENT: u32 = 0x4;
}

impl LabelType for Sec {
    const AM_SECRET_TYPE: bool = true;
    const DEFAULT_TAG: u32 = 0x1;
}

impl LabelType for Int {
    const AM_SECRET_TYPE: bool = false;
    const DEFAULT_TAG: u32 = 0x3;
}

pub trait Funcs<T> where T: LabelType {
    fn join<U>(&self, other: &U) -> DynLabel<T> where U: DynField<T>;
    unsafe fn set_minus<U>(&self, other: &U) -> DynLabel<T> where U: DynField<T>;
    unsafe fn subset_of<U>(&self, other: &U) -> bool where U: DynField<T>;
    unsafe fn is_equal_to<U>(&self, other: &U) -> bool where U: DynField<T>;
    unsafe fn is_equal_to_arc(&self, other: &Self) -> bool;
}

#[derive(Copy, Clone, Hash)]
pub struct DynamicTag<T> where T: LabelType {
    value: u32,
    p: PhantomData<T>
}

impl<T> DynamicTag<T> where T: LabelType {
    pub unsafe fn value_reference(&self) -> &u32 {
        &self.value
    }
}

impl<T> Serialize for DynamicTag<T> where T: LabelType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_u32(self.value)
    }
}

impl<'d, T> Deserialize<'d> for DynamicTag<T> where T: LabelType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
       where D: Deserializer<'d> {
           let value = u32::deserialize(deserializer)?;
           Ok(DynamicTag{value, p: PhantomData{}})
    }
}

impl<T> PartialOrd for DynamicTag<T> where T: LabelType {
    fn partial_cmp(&self, other: &DynamicTag<T>) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T> PartialEq for DynamicTag<T> where T: LabelType {
    fn eq(&self, other: &DynamicTag<T>) -> bool {
        self.value == other.value
    }
}

impl<T> Eq for DynamicTag<T> where T: LabelType {}

impl<T> PartialEq<u32> for DynamicTag<T> where T: LabelType {
    fn eq(&self, other: &u32) -> bool {
        self.value == *other
    }
}

impl<T> DynamicTag<T> where T: LabelType {
    #[inline(always)]
    const fn empty_tag() -> DynamicTag<T> { //OPTIMIZE //QUESTION: SHOULD WE HAVE A COUNTER FOR THIS
        DynamicTag::<T>{ value: T::DEFAULT_TAG, p: PhantomData }
    }
}

pub struct SmallLabel<T> where T: LabelType {
    t1: DynamicTag<T>,
    t2: DynamicTag<T>
}

impl<T> SmallLabel<T> where T: LabelType {
    #[inline(always)]
    const fn new_default() -> SmallLabel<T> {
        SmallLabel::<T>{t1: DynamicTag::<T>::empty_tag(), t2: DynamicTag::<T>::empty_tag()}
    }

    #[inline(always)]
    fn new_size_one(t: DynamicTag<T>) -> SmallLabel<T> {
        SmallLabel::<T>{t1: DynamicTag::<T>::empty_tag(), t2: t}
    }
}

impl<T> Copy for SmallLabel<T> where T: LabelType {}

impl<T> Clone for SmallLabel<T> where T: LabelType {
    #[inline(always)]
    fn clone(&self) -> SmallLabel<T> {
        *self
    }
}

#[derive(/*Default, Serialize, Deserialize*/)]
pub union DynLabel<T> where T: LabelType {
    num: u64,
    small: SmallLabel<T>,
    large: *const HashSet<DynamicTag<T>>
}

unsafe impl<T> Sync for DynLabel<T> where T: LabelType {}

impl<T> Drop for DynLabel<T> where T: LabelType {
    fn drop(&mut self) {
        if !self.is_small() {
            let _arc = unsafe {
                Arc::from_raw(self.large())
            };
        }
    }
}

fn forget<T>(x: Arc<T>) {
    std::mem::forget(x);
}

impl<T> Serialize for DynLabel<T> where T: LabelType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        let flag = 0x1;
        if self.num() & flag == 0 {
            panic!("Serializing large labels is not supported.");
        }
        u64::serialize(&self.num(), serializer)
    }
}

/*impl<T> Serialize for DynLabel<T> where T: LabelType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        let mut st = serializer.serialize_struct("DynLabel", 2)?;
        st.serialize_field("num", &self.num())?;
        if !self.is_small() {
            let arc = unsafe {
                Arc::from_raw(self.large())
            };
            let h: &HashSet<DynamicTag<T>> = arc.borrow();
            let o: Option<&HashSet<DynamicTag<T>>> = Some(h);
            st.serialize_field("tags", &o)?;
        }
        st.end()
    }
}*/

impl<'d, T> Deserialize<'d> for DynLabel<T> where T: LabelType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'d> {
        let mut num = u64::deserialize(deserializer)?;
        Ok(DynLabel::<T>{num})
    }
}

/*impl<'d, T> Deserialize<'d> for DynLabel<T> where T: LabelType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
       where D: Deserializer<'d> {
            #[derive(Deserialize)]
            #[serde(field_identifier, rename_all = "lowercase")]
            enum LabelPseudoFields { Num, Tags }
            struct LabelVisitor<T> {
                p: PhantomData<T>
            }
            impl<T> LabelVisitor<T> where T: LabelType {
                fn make_value(num: u64, tags: Option<HashSet<DynamicTag<T>>>) -> Result<DynLabel<T>, &'static str> {
                    match (num & DynLabel::<T>::SMALL_MASK, tags) {
                        (0, None) => Err("None"),
                        (0, Some(t)) => {
                            let a = Arc::new(t);
                            Ok(DynLabel{large: Arc::into_raw(a)})
                        },
                        (_, None) => Ok(DynLabel{num}),
                        (_, Some(_)) => Err("Some")
                    }
                }
            }
            impl<'de, T> serde::de::Visitor<'de> for LabelVisitor<T> where T: LabelType {
                type Value = DynLabel<T>;
                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("struct DynLabel")
                }
                fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
                    where V: serde::de::SeqAccess<'de> {
                    let num = seq.next_element::<u64>()?.ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                    let tags = seq.next_element::<Option<HashSet<DynamicTag<T>>>>()?.ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                    Self::make_value(num, tags).map_err(|e| serde::de::Error::invalid_value(serde::de::Unexpected::Other(e), &self))
                }
                fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
                    where V: serde::de::MapAccess<'de> {
                    let mut num = None;
                    let mut tags = None;
                    while let Some(key) = map.next_key()? {
                        match key {
                            LabelPseudoFields::Num => {
                                if num.is_some() {
                                    return Err(serde::de::Error::duplicate_field("num"));
                                }
                                num = Some(map.next_value::<u64>()?);
                            }
                            LabelPseudoFields::Tags => {
                                if tags.is_some() {
                                    return Err(serde::de::Error::duplicate_field("tags"));
                                }
                                tags = Some(map.next_value::<Option<HashSet<DynamicTag<T>>>>()?);
                            }
                        }
                    }
                    let num = num.ok_or_else(|| serde::de::Error::missing_field("num"))?;
                    let tags = tags.ok_or_else(|| serde::de::Error::missing_field("tags"))?;
                    Self::make_value(num, tags).map_err(|e| serde::de::Error::invalid_value(serde::de::Unexpected::Other(e), &self))
                }
            }
           const FIELDS: &[&str] = &["num", "tags"];
           deserializer.deserialize_struct("DynLabel", FIELDS, LabelVisitor{p: PhantomData})
    }
}*/

impl<T> DynLabel<T> where T: LabelType {
    #[inline(always)]
    fn num(&self) -> u64 {
        unsafe { self.num }
    }
    #[inline(always)]
    fn small(&self) -> SmallLabel<T> {
        unsafe { self.small }
    }
    #[inline(always)]
    fn large(&self) -> *const HashSet<DynamicTag<T>> {
        unsafe { self.large }
    }
    const SMALL_MASK: u64 = 0x1; // LARGE otherwise
    #[inline(always)]
    fn is_small(&self) -> bool {
        (self.num() & Self::SMALL_MASK) != 0
    }
    fn is_empty_label(&self) -> bool {
      Self::new_default().num() == self.num()
    }
    // TODO: are the following two functions correct?
    fn into_raw(self) -> u64 {
      if !self.is_small() {
        forget(unsafe { Arc::from_raw(self.large()) });
      }
      self.num()
    }
    fn from_raw(num: u64) -> DynLabel<T> {
        let new_label = DynLabel::<T>{num};
        if !new_label.is_small() {
            unsafe { Arc::from_raw(new_label.large()); }
        }
        new_label
    }
  }

unsafe impl<T> Send for DynLabel<T> where T: LabelType {}

impl<T> Clone for DynLabel<T> where T: LabelType {
    #[inline]
    fn clone(&self) -> DynLabel<T> {
        #[cfg(feature = "benchmarking")]
        unsafe { LABEL_CLONE_COUNTER.increment(); }
        if !self.is_small() {
            self.arc_clone()
        } else {
            DynLabel::<T>{small: self.small()}
        }
    }
}

impl<T> DynLabel<T>  where T: LabelType {
    #[inline(never)]
    fn arc_clone(&self) -> DynLabel<T> {
        let arc = unsafe {
            Arc::from_raw(self.large())
        };
        let a2 = arc.clone();
        forget(arc);
        DynLabel::<T>::new_large(a2)
    }
}

impl<T> DynLabel<T> where T: LabelType {
    #[inline(always)]
    pub const fn new_default() -> DynLabel<T> {
        let new = SmallLabel::<T>::new_default();
        DynLabel::<T>{ small: new}
    }

    #[inline(always)]
    pub fn new_size_one(t: DynamicTag<T>) -> DynLabel<T> {
        let new = SmallLabel::<T>::new_size_one(t);
        DynLabel::<T>{ small: new }
    }
    #[inline(always)]
    fn new_small(s: SmallLabel<T>) -> DynLabel<T> {
        /*unsafe {
            #[cfg(feature = "benchmarking")]
            { NEW_LABEL_COUNTER.increment(); }
        }*/
        DynLabel::<T>{ small: s }
    }
    #[inline]
    fn new_large(a: Arc<HashSet<DynamicTag<T>>>) -> DynLabel<T> {
        /*unsafe {
            #[cfg(feature = "benchmarking")]
            { NEW_LABEL_COUNTER.increment(); }
        }*/
        DynLabel::<T>{ large: Arc::into_raw(a) as *const _ }
    }
    pub unsafe fn value_reference(&self) -> &u64 {
        &self.num
    }
}

impl<T> Default for DynLabel<T> where T: LabelType {
    fn default() -> DynLabel<T> {
        DynLabel::<T>::new_default()
    }
}

pub trait Printable {
    unsafe fn print_self(&self);
    unsafe fn to_string(&self) -> String;
}

impl Printable for () {
    unsafe fn print_self(&self) {
        println!("Empty");
    }
    unsafe fn to_string(&self) -> String {
        "Empty".to_string()
    }
}

impl<T> Printable for DynLabel<T> where T: LabelType {
    unsafe fn print_self(&self) {
        if !self.is_small() {
            let arc = unsafe {
                Arc::from_raw(self.large())
            };
            let set = (*arc).clone();
            for s in set.iter() {
                print!("{}, ", s.value);
            }
            forget(arc);
        } else {
            print!("{}, ", self.small().t1.value);
            print!("{}, ", self.small().t2.value);
        }
    }
    unsafe fn to_string(&self) -> String {
        let mut string = String::new();
        if !self.is_small() {
            let arc = unsafe {
                Arc::from_raw(self.large())
            };
            let set = (*arc).clone();
            for s in set.iter() {
                string.push_str(&s.value.to_string());
                string.push_str(", ");
            }
            forget(arc);
        } else {
            string.push_str(&self.small().t1.value.to_string());
            string.push_str(", ");
            string.push_str(&self.small().t2.value.to_string());
        }
        string
    }
}

pub trait DynField<T>: Funcs<T> + Printable + Clone + serde::Serialize
    where T: LabelType
{
    const AM_EMPTY_TYPE: bool;
    fn generate_dynamic_label(&self) -> &DynLabel<T>;
}

impl DynField<Sec> for () {
    const AM_EMPTY_TYPE: bool = true;
    fn generate_dynamic_label(&self) -> &DynLabel<Sec> { //QUESTION: WHAT COUNTER SHOULD BE INCREMENTED
        DEFAULT_SECRECY_LABEL_REF
    }
}

impl DynField<Int> for () {
    const AM_EMPTY_TYPE: bool = true;
    #[inline(always)]
    fn generate_dynamic_label(&self) -> &DynLabel<Int> { //QUESTION: WHAT COUNTER SHOULD BE INCREMENTED
        DEFAULT_INTEGRITY_LABEL_REF
    }
}

impl<T> DynField<T> for DynLabel<T> where T: LabelType + Clone {
    const AM_EMPTY_TYPE: bool = false;
    #[inline(always)]
    fn generate_dynamic_label(&self) -> &DynLabel<T> {
        self
    }
}

impl<T> Funcs<T> for () where T: LabelType {
    #[inline(always)]
    fn join<U>(&self, other: &U) -> DynLabel<T>
        where U: DynField<T>
    {
        #[cfg(feature = "benchmarking")]
        unsafe { JOIN_COUNTER.increment(); CUSTOM_COUNTER_3.increment(); }
        other.generate_dynamic_label().clone()
    }

    #[inline(always)]
    unsafe fn set_minus<U>(&self, _other: &U) -> DynLabel<T> 
        where U: DynField<T>
    {
        #[cfg(feature = "benchmarking")]
        unsafe { DIFFERENCE_COUNTER.increment();}
        DynLabel::<T>::new_default()
    }

    #[inline(always)]
    unsafe fn subset_of<U>(&self, _other: &U) -> bool 
        where U: DynField<T>
    {
        #[cfg(feature = "benchmarking")]
        unsafe { SUBSET_COUNTER.increment();}
        true
    }
    #[inline(always)]
    unsafe fn is_equal_to<U>(&self, other: &U) -> bool where U: DynField<T> {
        #[cfg(feature = "benchmarking")]
        unsafe { EQUAL_COUNTER.increment();}
        if U::AM_EMPTY_TYPE {
            true
        } else {
            return other.generate_dynamic_label().is_empty_label();
        }
    }

    unsafe fn is_equal_to_arc(&self, _other: &Self) -> bool {
        panic!("This should never be called on an empty type.")
    }
}

impl<T> Funcs<T> for DynLabel<T>  where T: LabelType {
    fn join<U>(&self, other: &U) -> DynLabel<T>
        where U: DynField<T>
    {
        #[cfg(feature = "benchmarking")]
        unsafe { JOIN_COUNTER.increment(); }
        if U::AM_EMPTY_TYPE {
            #[cfg(feature = "benchmarking")]
            unsafe { CUSTOM_COUNTER_3.increment(); }
            self.clone()
        } else {
            if self.num() == other.generate_dynamic_label().num() {
                #[cfg(feature = "benchmarking")]
                unsafe { CUSTOM_COUNTER_3.increment(); }
                return self.clone();
            }
            let DEFAULT_TAG: u32 = T::DEFAULT_TAG;
            let dynamic_other = other.generate_dynamic_label();
            let self_num = self.num();
            let other_num = dynamic_other.num();
            if self.is_empty_label() {
                #[cfg(feature = "benchmarking")]
                unsafe { CUSTOM_COUNTER_3.increment(); }
                return dynamic_other.clone();
            } else if dynamic_other.is_empty_label() {
                #[cfg(feature = "benchmarking")]
                unsafe { CUSTOM_COUNTER_3.increment(); }
                return self.clone();
            }
            if !((self_num & Self::SMALL_MASK & other_num) == 0) {
                let self_small = self.small();
                let other_small = dynamic_other.small();
                if (self_small.t1 == DEFAULT_TAG) && (other_small.t1 == DEFAULT_TAG) {
                    if self_small.t2 < other_small.t2 {
                        DynLabel::<T>::new_small(SmallLabel::<T>{t1: self_small.t2, t2: other_small.t2 })
                    } else {
                        DynLabel::<T>::new_small(SmallLabel::<T>{t1: other_small.t2, t2: self_small.t2 })
                    }
                } else if self_small.t1 == DEFAULT_TAG {
                    if self_small.t2 == other_small.t1 || self_small.t2 == other_small.t2 {
                        #[cfg(feature = "benchmarking")]
                        unsafe { CUSTOM_COUNTER_3.increment(); }
                        dynamic_other.clone()
                    } else {
                        let mut set: HashSet<DynamicTag<T>> = HashSet::new();
                        set.insert(self_small.t2);
                        set.insert(other_small.t1);
                        set.insert(other_small.t2);
                        let new = Arc::new(set);
                        DynLabel::<T>::new_large(new)
                    }
                } else if other_small.t1 == DEFAULT_TAG {
                    if other_small.t2 == self_small.t1 || other_small.t2 == self_small.t2 {
                        #[cfg(feature = "benchmarking")]
                        unsafe { CUSTOM_COUNTER_3.increment(); }
                        self.clone()
                    } else {
                        let mut set: HashSet<DynamicTag<T>> = HashSet::new();
                        set.insert(other_small.t2);
                        set.insert(self_small.t1);
                        set.insert(self_small.t2);
                        let new = Arc::new(set);
                        DynLabel::<T>::new_large(new)
                    }
                } else {
                    let mut set: HashSet<DynamicTag<T>> = HashSet::new();
                    set.insert(self_small.t1);
                    set.insert(self_small.t2);
                    set.insert(other_small.t1);
                    set.insert(other_small.t2);
                    let new = Arc::new(set);
                    DynLabel::<T>::new_large(new)
                }
            } else if self.is_small() || dynamic_other.is_small() {
                let small: &DynLabel<T>;
                let large: &DynLabel<T>;
                if self.is_small() {
                    small = self;
                    large = &dynamic_other;
                } else {
                    small = &dynamic_other;
                    large = self;
                }
                let small_small = small.small();
                let arc_from_raw: Arc<HashSet<DynamicTag<T>>> = unsafe { Arc::<HashSet<DynamicTag<T>>>::from_raw(large.large()) };
                if (small_small.t1  == DEFAULT_TAG) && (*arc_from_raw).contains(&small_small.t2) {
                    forget(arc_from_raw);
                    #[cfg(feature = "benchmarking")]
                    unsafe { CUSTOM_COUNTER_3.increment(); }
                    large.clone()
                } else if (*arc_from_raw).contains(&small_small.t1) && (*arc_from_raw).contains(&small_small.t2) {
                    forget(arc_from_raw);
                    #[cfg(feature = "benchmarking")]
                    unsafe { CUSTOM_COUNTER_3.increment(); }
                    large.clone()
                } else {
                    let mut set: HashSet<DynamicTag<T>> = (*arc_from_raw).clone();
                    forget(arc_from_raw);
                    if small_small.t1 == DEFAULT_TAG {
                        set.insert(small_small.t2);
                    } else {
                        set.insert(small_small.t1);
                        set.insert(small_small.t2);
                    }
                    let new = Arc::new(set);
                    DynLabel::<T>::new_large(new)
                }
            } else {
                let arc_from_raw_self: Arc<HashSet<DynamicTag<T>>> = unsafe { Arc::<HashSet<DynamicTag<T>>>::from_raw(self.large()) };
                let arc_from_raw_other: Arc<HashSet<DynamicTag<T>>> = unsafe { Arc::<HashSet<DynamicTag<T>>>::from_raw(dynamic_other.large()) };
                let union = (&*arc_from_raw_self) | (&*arc_from_raw_other);
                if union.len() == (*arc_from_raw_self).len() {
                    forget(arc_from_raw_self);
                    forget(arc_from_raw_other);
                    #[cfg(feature = "benchmarking")]
                    unsafe { CUSTOM_COUNTER_3.increment(); }
                    self.clone()
                } else if union.len() == (*arc_from_raw_other).len() {
                    forget(arc_from_raw_self);
                    forget(arc_from_raw_other);
                    #[cfg(feature = "benchmarking")]
                    unsafe { CUSTOM_COUNTER_3.increment(); }
                    dynamic_other.clone()
                } else {
                    forget(arc_from_raw_self);
                    forget(arc_from_raw_other);
                    let new = Arc::new(union);
                    DynLabel::<T>::new_large(new)
                }
            }
        }
    }

    unsafe fn set_minus<U>(&self, other: &U) -> DynLabel<T>
        where U: DynField<T>
    {
        #[cfg(feature = "benchmarking")]
        unsafe { DIFFERENCE_COUNTER.increment();}
        if U::AM_EMPTY_TYPE {
            #[cfg(feature = "benchmarking")]
            unsafe { CUSTOM_COUNTER_4.increment(); }
            return self.clone();
        } else {
            let dynamic_other = other.generate_dynamic_label();
            let self_num = self.num();
            let other_num = dynamic_other.num();
            if self_num == other_num {
                return DynLabel::<T>::new_default();
            }
            let DEFAULT_TAG: u32 = T::DEFAULT_TAG;
            if !(self_num & Self::SMALL_MASK & other_num == 0) {
                let self_small = self.small();
                let other_small = dynamic_other.small();
                if (self_small.t1 == DEFAULT_TAG) && (other_small.t1 == DEFAULT_TAG) {
                    if self_small.t2 == other_small.t2 {
                        DynLabel::<T>::new_default()
                    } else {
                        #[cfg(feature = "benchmarking")]
                        unsafe { CUSTOM_COUNTER_4.increment(); }
                        self.clone()
                    }
                } else if self_small.t1 == DEFAULT_TAG {
                    if self_small.t2 == other_small.t1 || self_small.t2 == other_small.t2 {
                        DynLabel::<T>::new_default()
                    } else {
                        #[cfg(feature = "benchmarking")]
                        unsafe { CUSTOM_COUNTER_4.increment(); }
                        self.clone()
                    }
                } else if other_small.t1 == DEFAULT_TAG {
                    if self_small.t1 == other_small.t2 {
                        DynLabel::<T>::new_size_one(self_small.t2)
                    } else if self_small.t2 == other_small.t2 {
                        DynLabel::<T>::new_size_one(self_small.t1)
                    } else {
                        #[cfg(feature = "benchmarking")]
                        unsafe { CUSTOM_COUNTER_4.increment(); }
                        self.clone()
                    }
                } else {
                    if (self_small.t1 == other_small.t1) || (self_small.t1 == other_small.t2) {
                        DynLabel::<T>::new_size_one(self_small.t2)
                    } else if (self_small.t2 == other_small.t1) || (self_small.t2 == other_small.t2) {
                        DynLabel::<T>::new_size_one(self_small.t1)
                    } else {
                        #[cfg(feature = "benchmarking")]
                        unsafe { CUSTOM_COUNTER_4.increment(); }
                        self.clone()
                    }
                }
            } else if self.is_small() {
                let self_small = self.small();
                let arc_from_raw_other = unsafe { Arc::from_raw(dynamic_other.large()) };
                let return_value: DynLabel<T>;
                if self_small.t1 == DEFAULT_TAG {
                    if (*arc_from_raw_other).contains(&self_small.t2) {
                        return_value = DynLabel::<T>::new_default();
                    } else {
                        #[cfg(feature = "benchmarking")]
                        unsafe { CUSTOM_COUNTER_4.increment(); }
                        return_value = self.clone();
                    }
                } else if (*arc_from_raw_other).contains(&self_small.t1) {
                    if (*arc_from_raw_other).contains(&self_small.t2) {
                        return_value = DynLabel::<T>::new_default();
                    } else {
                        return_value = DynLabel::<T>::new_size_one(self_small.t2);
                    }
                } else if (*arc_from_raw_other).contains(&self_small.t2) {
                    return_value = DynLabel::<T>::new_size_one(self_small.t1);
                } else {
                    #[cfg(feature = "benchmarking")]
                    unsafe { CUSTOM_COUNTER_4.increment(); }
                    return_value = self.clone();
                }
                forget(arc_from_raw_other);
                return_value
            } else if dynamic_other.is_small() {
                let arc_from_raw_self = unsafe { Arc::from_raw(self.large()) };
                let other_small = dynamic_other.small();
                if other_small.t1 == DEFAULT_TAG {
                    if (*arc_from_raw_self).contains(&other_small.t2) {
                        let mut set = (*arc_from_raw_self).clone();
                        set.remove(&other_small.t2);
                        let new = Arc::new(set);
                        forget(arc_from_raw_self);
                        DynLabel::<T>::new_large(new)
                    } else {
                        forget(arc_from_raw_self);
                        #[cfg(feature = "benchmarking")]
                        unsafe { CUSTOM_COUNTER_4.increment(); }
                        self.clone()
                    }
                } else if (*arc_from_raw_self).contains(&other_small.t1) {
                    if (*arc_from_raw_self).contains(&other_small.t2) {
                        let mut set = (*arc_from_raw_self).clone();
                        set.remove(&other_small.t1);
                        set.remove(&other_small.t2);
                        let new = Arc::new(set);
                        forget(arc_from_raw_self);
                        DynLabel::<T>::new_large(new)
                    } else {
                        let mut set = (*arc_from_raw_self).clone();
                        set.remove(&other_small.t1);
                        let new = Arc::new(set);
                        forget(arc_from_raw_self);
                        DynLabel::<T>::new_large(new)
                    }
                } else if (*arc_from_raw_self).contains(&other_small.t2) {
                    let mut set = (*arc_from_raw_self).clone();
                    set.remove(&other_small.t2);
                    let new = Arc::new(set);
                    forget(arc_from_raw_self);
                    DynLabel::<T>::new_large(new)
                } else {
                    forget(arc_from_raw_self);
                    #[cfg(feature = "benchmarking")]
                    unsafe { CUSTOM_COUNTER_4.increment(); }
                    self.clone()
                }
            } else {
                let arc_from_raw_self = unsafe { Arc::from_raw(self.large()) };
                let arc_from_raw_other = unsafe { Arc::from_raw(dynamic_other.large()) };
                let dif = (&*arc_from_raw_self) - (&*arc_from_raw_other);
                forget(arc_from_raw_other);
                if dif.len() == (*arc_from_raw_self).len() {
                    forget(arc_from_raw_self);
                    #[cfg(feature = "benchmarking")]
                    unsafe { CUSTOM_COUNTER_4.increment(); }
                    self.clone()
                } else {
                    let new = Arc::new(dif);
                    forget(arc_from_raw_self);
                    DynLabel::<T>::new_large(new)
                }
            }
        }
    }

    unsafe fn subset_of<U>(&self, other: &U) -> bool
        where U: DynField<T>
    {
        #[cfg(feature = "benchmarking")]
        unsafe { SUBSET_COUNTER.increment();}
        let DEFAULT_TAG: u32 = T::DEFAULT_TAG;
        let self_num = self.num();
        if self.is_empty_label() {
            return true
        } else if U::AM_EMPTY_TYPE {
            return false
        } else {
            let dynamic_other = other.generate_dynamic_label();
            let other_num = dynamic_other.num();
            if self_num == other_num {
                return true;
            }
            if !(self_num & Self::SMALL_MASK & other_num == 0) {
                let self_small = self.small();
                let other_small = dynamic_other.small();
                if (self_small.t1 == DEFAULT_TAG) && (other_small.t1 == DEFAULT_TAG) {
                    if self_small.t2 == other_small.t2 {
                        true
                    } else {
                        false
                    }
                } else if self_small.t1 == DEFAULT_TAG {
                    if self_small.t2 == other_small.t1 || self_small.t2 == other_small.t2 {
                        true
                    } else {
                        false
                    }
                } else if other_small.t1 == DEFAULT_TAG {
                    false
                } else {
                    if (self_small.t1 == other_small.t1) && (self_small.t2 == other_small.t2) {
                        true
                    } else {
                        false
                    }
                }
            } else if self.is_small() {
                let self_small = self.small();
                let arc_from_raw_other = unsafe { Arc::from_raw(dynamic_other.large()) };
                let return_value: bool;
                if self_small.t1 == DEFAULT_TAG {
                    if (*arc_from_raw_other).contains(&self_small.t2) {
                        return_value = true;
                    } else {
                        return_value = false;
                    }
                } else if (*arc_from_raw_other).contains(&self_small.t1) {
                    if (*arc_from_raw_other).contains(&self_small.t2) {
                        return_value = true;
                    } else {
                        return_value = false;
                    }
                } else {
                    return_value = false;
                }
                forget(arc_from_raw_other);
                return_value
            } else if dynamic_other.is_small() {
                false
            } else {
                let arc_from_raw_self = unsafe { Arc::from_raw(self.large()) };
                let arc_from_raw_other = unsafe { Arc::from_raw(dynamic_other.large()) };
                let return_value = (*arc_from_raw_self).is_subset(&*arc_from_raw_other);
                forget(arc_from_raw_self);
                forget(arc_from_raw_other);
                return_value
            }
        }
    }

    #[inline]
    unsafe fn is_equal_to<U>(&self, other: &U) -> bool where U: DynField<T> {
        #[cfg(feature = "benchmarking")]
        unsafe { EQUAL_COUNTER.increment();}
        let self_num = self.num();
        //true
        if U::AM_EMPTY_TYPE {
            self.is_empty_label()
        } else {
            let dynamic_other = other.generate_dynamic_label();
            let other_num = dynamic_other.num();
            if self_num == other_num as u64 {
              true
            } else {
              unsafe { self.is_equal_to_arc(dynamic_other) }
            }
        }
    }

    #[inline(never)]
    unsafe fn is_equal_to_arc(&self, other: &Self) -> bool {
      if !other.is_small() { // self.num() & MASK should equal MASK if we reach here
        let arc_from_raw_self = unsafe { Arc::from_raw(self.large()) };
        let arc_from_raw_other = unsafe { Arc::from_raw(other.large()) };
        let return_value = &*arc_from_raw_self == &*arc_from_raw_other;
        forget(arc_from_raw_self);
        forget(arc_from_raw_other);
        return_value
      } else {
        false
      }
    }
}

pub unsafe auto trait Immutable {} // interior immutable
impl<T: ?Sized> !Immutable for std::cell::UnsafeCell<T> {}
//impl<T: ?Sized> !Immutable for &T {} // Isn't mutable, but we don't want references wrapped in Secrets?
impl<T: ?Sized> !Immutable for &mut T {}
//impl<T: ?Sized> !UniquePtr for std::rc::Rc<T> {}
//impl<T: ?Sized> !UniquePtr for Arc<T> {}

pub unsafe trait SecureValueSafe {} // For limiting what values can be wrapped in a Secret
unsafe impl<T> SecureValueSafe for T where T: Immutable + InvisibleSideEffectFree {}

/*
    * Code to restrict using unary, binary operators in secret closures to only primitive types.
    * Users can't overload (e.g. std::ops::Add) for primitive types so we can safely assume (+) hasn't
    * been overloaded in that case.
    */
// Extends a binary operator trait impl over refs
// Modified from forward_ref::forward_bin_op to use unsafe impls
macro_rules! unsafe_forward_ref_binop {
    // Equivalent to the non-const version, with the addition of `rustc_const_unstable`
    (unsafe impl const $imp:ident, $method:ident for $t:ty, $u:ty) => {
        unsafe impl<'a> const $imp<$u> for &'a $t {
            type Output = <$t as $imp<$u>>::Output;
            #[inline]
            fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, other)
            }
        }
        unsafe impl const $imp<&$u> for $t {
            type Output = <$t as $imp<$u>>::Output;
            #[inline]
            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(self, *other)
            }
        }
        unsafe impl const $imp<&$u> for &$t {
            type Output = <$t as $imp<$u>>::Output;
            #[inline]
            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, *other)
            }
        }
    };
    (unsafe impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        unsafe impl<'a> $imp<$u> for &'a $t {
            type Output = <$t as $imp<$u>>::Output;
            #[inline]
            fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, other)
            }
        }
        unsafe impl $imp<&$u> for $t {
            type Output = <$t as $imp<$u>>::Output;
            #[inline]
            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(self, *other)
            }
        }
        unsafe impl $imp<&$u> for &$t {
            type Output = <$t as $imp<$u>>::Output;
            #[inline]
            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, *other)
            }
        }
    };
}

macro_rules! unsafe_forward_ref_unop {
    // Equivalent to the non-const version
    (unsafe impl const $imp:ident, $method:ident for $t:ty) => {
        unsafe impl const $imp for &$t {
            type Output = <$t as $imp>::Output;
            #[inline]
            fn $method(self) -> <$t as $imp>::Output {
                $imp::$method(*self)
            }
        }
    };
    (unsafe impl $imp:ident, $method:ident for $t:ty) => {
        unsafe impl $imp for &$t {
            type Output = <$t as $imp>::Output;
            #[inline]
            fn $method(self) -> <$t as $imp>::Output {
                $imp::$method(*self)
            }
        }
    };
}

// implements "T op= &U", based on "T op= U"
// where U is expected to be `Copy`able
macro_rules! unsafe_forward_ref_op_assign {
    // Equivalent to the non-const version
    (unsafe impl const $imp:ident, $method:ident for $t:ty, $u:ty) => {
        unsafe impl const $imp<&$u> for $t {
            #[inline]
            fn $method(&mut self, other: &$u) {
                $imp::$method(self, *other);
            }
        }
    };
    (unsafe impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        unsafe impl $imp<&$u> for $t {
            #[inline]
            fn $method(&mut self, other: &$u) {
                $imp::$method(self, *other);
            }
        }
    };
}

/* Addition */
pub unsafe trait SafeAdd<Rhs = Self> {
    type Output;

    fn safe_add(self, rhs: Rhs) -> Self::Output;
}

// Auto implement SafeAdd for all primitive types
// Modified from source code for std::ops::Add with exception of const
// Const does not appear to behave the same way here as it does in std::ops
// so I'm not using it. This restricts use to non-const settings.
macro_rules! add_impl {
    ($($t:ty)*) => ($(
        unsafe impl SafeAdd for $t {
            type Output = $t;

            #[inline]
            fn safe_add(self, other: $t) -> $t { self + other }
        }
        unsafe_forward_ref_binop!{unsafe impl SafeAdd, safe_add for $t, $t}
    )*);
}
add_impl! {usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64}

// String concatenation
unsafe impl SafeAdd<&str> for String {
    type Output = String;

    #[inline]
    fn safe_add(self, other: &str) -> Self::Output {
        self + other
    }
}

pub unsafe trait SafeAddAssign<Rhs = Self> {
    fn safe_add_assign(&mut self, rhs: Rhs);
}
macro_rules! add_assign_impl {
    ($($t:ty)+) => ($(
        unsafe impl SafeAddAssign for $t {
            #[inline]
            fn safe_add_assign(&mut self, other: $t) { *self += other }
        }
        unsafe_forward_ref_op_assign! {unsafe impl SafeAddAssign, safe_add_assign for $t, $t }
    )+)
}
add_assign_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

/* Subtraction */
pub unsafe trait SafeSub<Rhs = Self> {
    type Output;
    fn safe_sub(self, rhs: Rhs) -> Self::Output;
}

macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        unsafe impl SafeSub for $t {
            type Output = $t;
            #[inline]
            fn safe_sub(self, other: $t) -> $t { self - other }
        }
        unsafe_forward_ref_binop! { unsafe impl SafeSub, safe_sub for $t, $t }
    )*)
}
sub_impl! {usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64}

pub unsafe trait SafeSubAssign<Rhs = Self> {
    fn safe_sub_assign(&mut self, rhs: Rhs);
}
macro_rules! sub_assign_impl {
    ($($t:ty)+) => ($(
        unsafe impl SafeSubAssign for $t {
            #[inline]
            fn safe_sub_assign(&mut self, other: $t) { *self -= other }
        }
        unsafe_forward_ref_op_assign! {unsafe impl SafeSubAssign, safe_sub_assign for $t, $t }
    )+)
}
sub_assign_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

/* Multiplication */
pub unsafe trait SafeMul<Rhs = Self> {
    type Output;

    fn safe_mul(self, rhs: Rhs) -> Self::Output;
}

macro_rules! mul_impl {
    ($($t:ty)*) => ($(
        unsafe impl SafeMul for $t {
            type Output = $t;

            #[inline]
            fn safe_mul(self, other: $t) -> $t {self * other}
        }

        unsafe_forward_ref_binop! { unsafe impl SafeMul, safe_mul for $t, $t }
    )*)
}
mul_impl! {usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64}

pub unsafe trait SafeMulAssign<Rhs = Self> {
    fn safe_mul_assign(&mut self, rhs: Rhs);
}
macro_rules! mul_assign_impl {
    ($($t:ty)+) => ($(
        unsafe impl SafeMulAssign for $t {
            #[inline]
            fn safe_mul_assign(&mut self, other: $t) { *self *= other }
        }
        unsafe_forward_ref_op_assign! {unsafe impl SafeMulAssign, safe_mul_assign for $t, $t }
    )+)
}
mul_assign_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

/* Division */
#[const_trait]
pub unsafe trait SafeDiv<Rhs = Self> {
    type Output;
    fn safe_div(self, rhs: Rhs) -> Self::Output;
}
macro_rules! div_impl_integer {
    ($(($($t:ty)*) => $panic:expr),*) => ($($(
        #[doc = $panic]
        //#[stable(feature = "rust1", since = "1.0.0")]
        //#[rustc_const_unstable(feature = "const_ops", issue = "90080")]
        unsafe impl const SafeDiv for $t {
            type Output = $t;

            #[inline]
            fn safe_div(self, other: $t) -> $t { self / other }
        }

        unsafe_forward_ref_binop! { unsafe impl const SafeDiv, safe_div for $t, $t }
    )*)*)

}

div_impl_integer! {
    (usize u8 u16 u32 u64 u128) => "This operation will panic if `other == 0`.",
    (isize i8 i16 i32 i64 i128) => "This operation will panic if `other == 0` or the division results in overflow."
}
macro_rules! div_impl_float {
    ($($t:ty)*) => ($(
        //#[stable(feature = "rust1", since = "1.0.0")]
        //#[rustc_const_unstable(feature = "const_ops", issue = "90080")]
        //NOTE: original code had unsafe impl const SafeDiv both here and in unsafe_forward_ref_binop
        unsafe impl SafeDiv for $t {
            type Output = $t;

            #[inline]
            fn safe_div(self, other: $t) -> $t { self / other }
        }

        unsafe_forward_ref_binop! { unsafe impl SafeDiv, safe_div for $t, $t }
    )*)
}
div_impl_float! { f32 f64 }

pub unsafe trait SafeDivAssign<Rhs = Self> {
    fn safe_div_assign(&mut self, rhs: Rhs);
}
macro_rules! div_assign_impl_integer {
    ($(($($t:ty)*) => $panic:expr),*) => ($($(
        #[doc = $panic]
        //#[stable(feature = "rust1", since = "1.0.0")]
        //#[rustc_const_unstable(feature = "const_ops", issue = "90080")]
        unsafe impl SafeDivAssign for $t {
            #[inline]
            fn safe_div_assign(&mut self, other: $t) { *self /= other }
        }

        unsafe_forward_ref_op_assign! { unsafe impl SafeDivAssign, safe_div_assign for $t, $t }
    )*)*)
}

div_assign_impl_integer! {
    (usize u8 u16 u32 u64 u128) => "This operation will panic if `other == 0`.",
    (isize i8 i16 i32 i64 i128) => "This operation will panic if `other == 0` or the division results in overflow."
}

macro_rules! div_assign_impl_float {
    ($($t:ty)*) => ($(
        //#[stable(feature = "rust1", since = "1.0.0")]
        //#[rustc_const_unstable(feature = "const_ops", issue = "90080")]
        //NOTE: original code had unsafe impl const SafeDiv both here and in unsafe_forward_ref_binop
        unsafe impl SafeDivAssign for $t {
            #[inline]
            fn safe_div_assign(&mut self, other: $t) { *self /= other }
        }

        unsafe_forward_ref_op_assign! { unsafe impl SafeDivAssign, safe_div_assign for $t, $t }
    )*)
}
div_assign_impl_float! { f32 f64 }

/* Remainder */
#[const_trait]
pub unsafe trait SafeRem<Rhs = Self> {
    type Output;
    fn safe_rem(self, rhs: Rhs) -> Self::Output;
}
macro_rules! rem_impl_integer {
    ($(($($t:ty)*) => $panic:expr),*) => ($($(
        #[doc = $panic]
        //#[stable(feature = "rust1", since = "1.0.0")]
        //#[rustc_const_unstable(feature = "const_ops", issue = "90080")]
        unsafe impl const SafeRem for $t {
            type Output = $t;

            #[inline]
            fn safe_rem(self, other: $t) -> $t { self % other }
        }

        unsafe_forward_ref_binop! { unsafe impl const SafeRem, safe_rem for $t, $t }
    )*)*)

}

rem_impl_integer! {
    (usize u8 u16 u32 u64 u128) => "This operation will panic if `other == 0`.",
    (isize i8 i16 i32 i64 i128) => "This operation will panic if `other == 0` or if `self / other` results in overflow."
}
macro_rules! rem_impl_float {
    ($($t:ty)*) => ($(
        //#[stable(feature = "rust1", since = "1.0.0")]
        //#[rustc_const_unstable(feature = "const_ops", issue = "90080")]
        //NOTE: original code had unsafe impl const SafeDiv both here and in unsafe_forward_ref_binop
        unsafe impl SafeRem for $t {
            type Output = $t;

            #[inline]
            fn safe_rem(self, other: $t) -> $t { self % other }
        }

        unsafe_forward_ref_binop! { unsafe impl SafeRem, safe_rem for $t, $t }
    )*)
}
rem_impl_float! { f32 f64 }

pub unsafe trait SafeRemAssign<Rhs = Self> {
    fn safe_rem_assign(&mut self, rhs: Rhs);
}

macro_rules! rem_assign_impl {
    ($($t:ty)+) => ($(
        //#[stable(feature = "op_assign_traits", since = "1.8.0")]
        unsafe impl SafeRemAssign for $t {
            #[inline]
            fn safe_rem_assign(&mut self, other: $t) { *self %= other }
        }

        unsafe_forward_ref_op_assign! { unsafe impl SafeRemAssign, safe_rem_assign for $t, $t }
    )+)
}

rem_assign_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

/* PartialEq */
pub unsafe trait SafePartialEq<Rhs: ?Sized = Self> {
    /// This method tests for `self` and `other` values to be equal, and is used
    /// by `==`.
    #[must_use]
    fn safe_eq(&self, other: &Rhs) -> bool;

    /// This method tests for `!=`.
    #[inline]
    #[must_use]
    fn safe_ne(&self, other: &Rhs) -> bool {
        !self.safe_eq(other)
    }
}

// had to implement manually since std::cmp uses a compiler built-in and so the macro is not available in
// Rust docs source code
macro_rules! parteq_impl {
    ($($t:ty)*) => ($(
        unsafe impl SafePartialEq for $t {
            #[inline]
            fn safe_eq(&self, other: &$t) -> bool { self == other }

            fn safe_ne(&self, other: &$t) -> bool {
                !self.safe_eq(other)
            }
        }
        //unsafe_forward_ref_binop! {unsafe impl SafePartialEq, safe_eq safe_ne for $t, $t}
    )*)
}

parteq_impl! { bool char f32 f64 i8 i16 i32 i64 i128 isize &str u8 u16 u32 u64 u128 usize }

pub unsafe fn safe_max_by<T, F: FnOnce(&T, &T) -> std::cmp::Ordering>(
    v1: T,
    v2: T,
    compare: F,
) -> T {
    match compare(&v1, &v2) {
        std::cmp::Ordering::Less | std::cmp::Ordering::Equal => v2,
        std::cmp::Ordering::Greater => v1,
    }
}

pub unsafe fn safe_min_by<T, F: FnOnce(&T, &T) -> std::cmp::Ordering>(
    v1: T,
    v2: T,
    compare: F,
) -> T {
    match compare(&v1, &v2) {
        std::cmp::Ordering::Less | std::cmp::Ordering::Equal => v1,
        std::cmp::Ordering::Greater => v2,
    }
}

pub unsafe trait SafeOrd: Eq + PartialOrd<Self> {
    #[must_use]
    fn safe_cmp(&self, other: &Self) -> std::cmp::Ordering;

    #[inline]
    #[must_use]
    unsafe fn safe_max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        safe_max_by(self, other, SafeOrd::safe_cmp)
    }

    #[inline]
    #[must_use]
    unsafe fn safe_min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        safe_min_by(self, other, SafeOrd::safe_cmp)
    }

    #[must_use]
    fn safe_clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        assert!(min <= max);
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}

unsafe impl SafeOrd for std::cmp::Ordering {
    #[inline]
    fn safe_cmp(&self, other: &std::cmp::Ordering) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}

unsafe impl SafePartialOrd for std::cmp::Ordering {
    #[inline]
    fn safe_partial_cmp(&self, other: &std::cmp::Ordering) -> Option<std::cmp::Ordering> {
        (*self as i32).partial_cmp(&(*other as i32))
    }
}

pub unsafe trait SafePartialOrd<Rhs: ?Sized = Self>: PartialEq<Rhs> {
    #[must_use]
    fn safe_partial_cmp(&self, other: &Rhs) -> Option<std::cmp::Ordering>;

    #[inline]
    #[must_use]
    fn safe_lt(&self, other: &Rhs) -> bool {
        matches!(self.safe_partial_cmp(other), Some(std::cmp::Ordering::Less))
    }

    #[inline]
    #[must_use]
    fn safe_le(&self, other: &Rhs) -> bool {
        !matches!(
            self.safe_partial_cmp(other),
            None | Some(std::cmp::Ordering::Greater)
        )
    }

    #[inline]
    #[must_use]
    fn safe_gt(&self, other: &Rhs) -> bool {
        matches!(
            self.safe_partial_cmp(other),
            Some(std::cmp::Ordering::Greater)
        )
    }

    #[inline]
    #[must_use]
    fn safe_ge(&self, other: &Rhs) -> bool {
        matches!(
            self.safe_partial_cmp(other),
            Some(std::cmp::Ordering::Greater | std::cmp::Ordering::Equal)
        )
    }
}
macro_rules! partial_ord_impl {
    ($($t:ty)*) => ($(
        unsafe impl SafePartialOrd for $t {
            fn safe_partial_cmp(&self, other: &$t) -> Option<std::cmp::Ordering> {
                match (*self <= *other, *self >= *other) {
                    (false, false) => None,
                    (false, true) => Some(std::cmp::Ordering::Greater),
                    (true, false) => Some(std::cmp::Ordering::Less),
                    (true, true) => Some(std::cmp::Ordering::Equal),
                }
            }
            #[inline]
            fn safe_lt(&self, other: &$t) -> bool { (*self) < (*other) }
            #[inline]
            fn safe_le(&self, other: &$t) -> bool { (*self) <= (*other) }
            #[inline]
            fn safe_ge(&self, other: &$t) -> bool { (*self) >= (*other) }
            #[inline]
            fn safe_gt(&self, other: &$t) -> bool { (*self) > (*other) }
        }
    )*)
}

partial_ord_impl! {f32 f64}

// Negation
pub unsafe trait SafeNeg {
    type Output;
    fn safe_neg(self) -> Self::Output;
}

macro_rules! ord_impl {
    ($($t:ty)*) => ($(
        unsafe impl SafePartialOrd for $t {
            #[inline]
            fn safe_partial_cmp(&self, other: &$t) -> Option<std::cmp::Ordering> {
                Some(self.safe_cmp(other))
            }
            #[inline]
            fn safe_lt(&self, other: &$t) -> bool { (*self) < (*other) }
            #[inline]
            fn safe_le(&self, other: &$t) -> bool { (*self) <= (*other) }
            #[inline]
            fn safe_ge(&self, other: &$t) -> bool { (*self) >= (*other) }
            #[inline]
            fn safe_gt(&self, other: &$t) -> bool { (*self) > (*other) }
        }

        unsafe impl SafeOrd for $t {
            #[inline]
            fn safe_cmp(&self, other: &$t) -> std::cmp::Ordering {
                if *self < *other { return std::cmp::Ordering::Less }
                else if *self == *other { return std::cmp::Ordering::Equal }
                else { return std::cmp::Ordering::Greater}
            }
        }
    )*)
}

ord_impl! { char usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128}

macro_rules! neg_impl {
    (f $($t:ty)*) => ($(
        unsafe impl SafeNeg for $t {
            type Output = $t;
            #[inline]
            fn safe_neg(self) -> $t { -self }
        }
        unsafe_forward_ref_unop! {unsafe impl SafeNeg, safe_neg for $t }
    )*);
    ($($t:ty)*) => ($(
        unsafe impl SafeNeg for $t {
            type Output = $t;
            #[inline]
            fn safe_neg(self) -> $t { -self }
        }
        unsafe_forward_ref_unop! {unsafe impl SafeNeg, safe_neg for $t }
    )*);
}

neg_impl! {isize i8 i16 i32 i64 i128}
neg_impl! {f f32 f64}

/* Not */
pub unsafe trait SafeNot {
    type Output;
    fn safe_not(self) -> Self::Output;
}

macro_rules! not_impl {
    ($($t:ty)*) => ($(
        unsafe impl SafeNot for $t {
            type Output = $t;
            #[inline]
            fn safe_not(self) -> $t { !self }
        }
        unsafe_forward_ref_unop! { unsafe impl SafeNot, safe_not for $t }
    )*)
}

not_impl! {bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128}

/* Traits for all other overloadable operators (not implemented = disallowed by macro)
TODO: implement these */
pub unsafe trait SafeBitAndAssign {}

pub unsafe trait SafeBitOr<Rhs = Self> {
    /// The resulting type after applying the `&` operator.
    type Output;
    fn safe_bitor(self, rhs: Rhs) -> Self::Output;
}

macro_rules! bitor_impl {
    ($($t:ty)*) => ($(
        unsafe impl SafeBitOr for $t {
            type Output = $t;

            #[inline]
            fn safe_bitor(self, rhs: $t) -> $t { self | rhs }
        }

        unsafe_forward_ref_binop! { unsafe impl SafeBitOr, safe_bitor for $t, $t }
    )*)
}

bitor_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

pub unsafe trait SafeBitOrAssign {}
pub unsafe trait SafeBitXorAssign {}
pub unsafe trait SafeDrop {}
pub unsafe trait SafeFn {}
pub unsafe trait SafeFnMut {}
pub unsafe trait SafeFnOnce {}
#[const_trait]

pub unsafe trait SafeBitXor<Rhs = Self> {
    type Output;
    fn safe_bitxor(self, rhs: Rhs) -> Self::Output;
}

macro_rules! bitxor_impl {
    ($($t:ty)*) => ($(
        unsafe impl SafeBitXor for $t {
            type Output = $t;

            #[inline]
            fn safe_bitxor(self, other: $t) -> $t { self ^ other }
        }

        unsafe_forward_ref_binop! { unsafe impl SafeBitXor, safe_bitxor for $t, $t }
    )*)
}

bitxor_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

pub unsafe trait SafeBitAnd<Rhs = Self> {
    /// The resulting type after applying the `&` operator.
    type Output;
    fn safe_bitand(self, rhs: Rhs) -> Self::Output;
}

macro_rules! bitand_impl {
    ($($t:ty)*) => ($(
        unsafe impl SafeBitAnd for $t {
            type Output = $t;

            #[inline]
            fn safe_bitand(self, rhs: $t) -> $t { self & rhs }
        }

        unsafe_forward_ref_binop! { unsafe impl SafeBitAnd, safe_bitand for $t, $t }
    )*)
}

bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

pub unsafe trait SafeIndex {}
unsafe impl SafeIndex for std::ops::Range<usize> {}
unsafe impl SafeIndex for std::ops::RangeFrom<usize> {}
unsafe impl SafeIndex for std::ops::RangeFull {}
unsafe impl SafeIndex for std::ops::RangeInclusive<usize> {}
unsafe impl SafeIndex for std::ops::RangeTo<usize> {}
// duplicate definition given Range types
//unsafe impl<T> SafeIndex for T where T: std::slice::SliceIndex<str>, {}
//unsafe impl<T> SafeIndex for T where T: std::slice::SliceIndex<[T]> {}
unsafe impl<Q> SafeIndex for &Q where Q: std::cmp::Ord + ?Sized, {}
unsafe impl SafeIndex for usize {}

pub unsafe trait SafeIndexExpr {}
unsafe impl SafeIndexExpr for std::string::String {}
unsafe impl SafeIndexExpr for &std::string::String {}
unsafe impl SafeIndexExpr for &mut std::string::String {}
unsafe impl SafeIndexExpr for std::ffi::CStr {}
unsafe impl SafeIndexExpr for &std::ffi::CStr {}
unsafe impl SafeIndexExpr for &mut std::ffi::CStr {}
unsafe impl SafeIndexExpr for std::ffi::CString {}
unsafe impl SafeIndexExpr for &std::ffi::CString {}
unsafe impl SafeIndexExpr for &mut std::ffi::CString{}
unsafe impl SafeIndexExpr for std::ffi::OsString {}
unsafe impl SafeIndexExpr for &std::ffi::OsString {}
unsafe impl SafeIndexExpr for &mut std::ffi::OsString{}
unsafe impl SafeIndexExpr for str {}
unsafe impl SafeIndexExpr for &str {}
unsafe impl SafeIndexExpr for &mut str {}
unsafe impl<K, V> SafeIndexExpr for std::collections::BTreeMap<K, V>
    where K: std::cmp::Ord, {}
unsafe impl<K, V> SafeIndexExpr for &std::collections::BTreeMap<K, V>
    where K: std::cmp::Ord, {}
unsafe impl<K, V> SafeIndexExpr for &mut std::collections::BTreeMap<K, V>
    where K: std::cmp::Ord, {}
unsafe impl<K, V, S> SafeIndexExpr for std::collections::HashMap<K, V, S>
    where K: std::cmp::Eq + std::hash::Hash, S: std::hash::BuildHasher, {}
unsafe impl<K, V, S> SafeIndexExpr for &std::collections::HashMap<K, V, S>
    where K: std::cmp::Eq + std::hash::Hash, S: std::hash::BuildHasher, {}
unsafe impl<K, V, S> SafeIndexExpr for &mut std::collections::HashMap<K, V, S>
        where K: std::cmp::Eq + std::hash::Hash, S: std::hash::BuildHasher, {}
unsafe impl<T, A> SafeIndexExpr for std::collections::VecDeque<T, A> where A: std::alloc::Allocator, {}
unsafe impl<T, A> SafeIndexExpr for &std::collections::VecDeque<T, A> where A: std::alloc::Allocator, {}
unsafe impl<T, A> SafeIndexExpr for &mut std::collections::VecDeque<T, A> where A: std::alloc::Allocator, {}
unsafe impl<T> SafeIndexExpr for [T] {}
unsafe impl<T> SafeIndexExpr for &[T] {}
unsafe impl<T> SafeIndexExpr for &mut [T] {}
unsafe impl<T, A> SafeIndexExpr for std::vec::Vec<T, A> where A: std::alloc::Allocator, {}
unsafe impl<T, A> SafeIndexExpr for &std::vec::Vec<T, A> where A: std::alloc::Allocator, {}
unsafe impl<T, A> SafeIndexExpr for &mut std::vec::Vec<T, A> where A: std::alloc::Allocator, {}
unsafe impl<T, const N: usize> SafeIndexExpr for [T; N] where [T]: SafeIndexExpr, {}
unsafe impl<T, const N: usize> SafeIndexExpr for &[T; N] where [T]: SafeIndexExpr, {}
unsafe impl<T, const N: usize> SafeIndexExpr for &mut [T; N] where [T]: SafeIndexExpr, {}

pub unsafe trait SafeRangeBounds {}

// only allow ranges to be of Rust numeric types
pub unsafe trait SafeRangeTypes {}
macro_rules! safe_range_types_impl {
    ($($t:ty)*) => ($(
        unsafe impl SafeRangeTypes for $t {}
    )*)
}
safe_range_types_impl! {usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64}

// unsafe impl<'a, T: SafeRangeTypes + 'a + ?Sized> SafeRangeBounds for (std::ops::Bound<&'a T>, std::ops::Bound<&'a T>) {}
unsafe impl<T: SafeRangeTypes> SafeRangeBounds for (std::ops::Bound<T>, std::ops::Bound<T>) {}
// unsafe impl<T: SafeRangeTypes> SafeRangeBounds for std::ops::Range<&T> {}
unsafe impl<T: SafeRangeTypes> SafeRangeBounds for std::ops::Range<T> {}
// unsafe impl<T: SafeRangeTypes> SafeRangeBounds for std::ops::RangeFrom<&T> {}
unsafe impl<T: SafeRangeTypes> SafeRangeBounds for std::ops::RangeFrom<T> {}
unsafe impl SafeRangeBounds for std::ops::RangeFull {}
//unsafe impl<T: SafeRangeTypes: SafeRangeTypes> SafeRangeBounds for std::ops::RangeInclusive<&T> {}
unsafe impl<T: SafeRangeTypes> SafeRangeBounds for std::ops::RangeInclusive<T> {}
//unsafe impl<T: SafeRangeTypes> SafeRangeBounds for std::ops::RangeTo<&T> {}
unsafe impl<T: SafeRangeTypes> SafeRangeBounds for std::ops::RangeTo<T> {}
// unsafe impl<T: SafeRangeTypes> SafeRangeBounds for std::ops::RangeToInclusive<&T> {}
unsafe impl<T: SafeRangeTypes> SafeRangeBounds for std::ops::RangeToInclusive<T> {}

pub unsafe trait SafeShl<Rhs = Self> {
    type Output;

    fn safe_shl(self, rhs: Rhs) -> Self::Output;
}

macro_rules! shl_impl {
    ($($t:ty)*) => ($(
        unsafe impl SafeShl for $t {
            type Output = $t;

            #[inline]
            fn safe_shl(self, other: $t) -> $t { self << other }
        }
        unsafe_forward_ref_binop!{unsafe impl SafeShl, safe_shl for $t, $t}
    )*);
}
shl_impl! {usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128}

pub unsafe trait SafeShlAssign {}

pub unsafe trait SafeShr<Rhs = Self> {
    type Output;

    fn safe_shr(self, rhs: Rhs) -> Self::Output;
}

macro_rules! shr_impl {
    ($($t:ty)*) => ($(
        unsafe impl SafeShr for $t {
            type Output = $t;

            #[inline]
            fn safe_shr(self, other: $t) -> $t { self >> other }
        }
        unsafe_forward_ref_binop!{unsafe impl SafeShr, safe_shr for $t, $t}
    )*);
}
shr_impl! {usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128}

pub unsafe trait SafeShrAssign {}

impl<T, L> StaticSecret<T, L>
    where T: SecureValueSafe
{
    //TODO: should be unsafe
    #[inline(always)]
    pub fn new(val: T) -> StaticSecret<T, L> {
        StaticSecret::<T, L> {
            val: val,
            _pd_secrecy: PhantomData,
            _pd_integrity: PhantomData,
            dynamic_secrecy: (),
            dynamic_integrity: ()
        }
    }

    #[inline(always)]
    pub unsafe fn unwrap_mut_unsafe<M>(&mut self) -> &mut T
    where
        M: lattice::MoreSecretThan<L>,
        L: lattice::MoreSecretThan<M>
    {
        &mut self.val
    }

    #[inline(always)]
    pub unsafe fn unwrap_consume_unsafe<M>(self) -> T
    where
        M: lattice::MoreSecretThan<L>
    {
        self.unwrap()
    }
    
    #[inline(always)]
    pub unsafe fn unwrap_unsafe<M>(&self) -> &T
    where
        M: lattice::MoreSecretThan<L>,
    {
        &self.val
    }
}

impl<T, L> StaticIntegrity<T, L>
    where T: SecureValueSafe
{
    #[inline(always)]
    pub unsafe fn new_static_integrity(val: T) -> StaticIntegrity<T, L> {
        StaticIntegrity::<T, L> {
            val: val,
            _pd_secrecy: PhantomData,
            _pd_integrity: PhantomData,
            dynamic_secrecy: (),
            dynamic_integrity: ()
        }
    }
}

impl<T, D> DynamicSecret<T, D>
    where T: SecureValueSafe, D: DynField<Sec>
{
    #[inline(always)]
    pub unsafe fn new_dynamic_secret(val: T, label: D) -> DynamicSecret<T, D> {
        DynamicSecret::<T, D> {
            val: val,
            _pd_secrecy: PhantomData,
            _pd_integrity: PhantomData,
            dynamic_secrecy: label,
            dynamic_integrity: ()
        }
    }
}

impl<T, D> DynamicIntegrity<T, D>
    where T: SecureValueSafe, D: DynField<Int>
{
    #[inline(always)]
    pub unsafe fn new_dynamic_integrity(val: T, label: D) -> DynamicIntegrity<T, D> {
        DynamicIntegrity::<T, D> {
            val: val,
            _pd_secrecy: PhantomData,
            _pd_integrity: PhantomData,
            dynamic_secrecy: (),
            dynamic_integrity: label
        }
    }
}

impl<T, L1, L2> StaticAll<T, L1, L2>
    where T: SecureValueSafe
{
    #[inline(always)]
    pub unsafe fn new_static_all(val: T) -> StaticAll<T, L1, L2> {
        StaticAll::<T, L1, L2> {
            val: val,
            _pd_secrecy: PhantomData,
            _pd_integrity: PhantomData,
            dynamic_secrecy: (),
            dynamic_integrity: ()
        }
    }

    #[inline(always)]
    pub unsafe fn unwrap_mut_unsafe_static_all<M1, M2>(&mut self) -> &mut T
    where
        M1: lattice::MoreSecretThan<L1>, M2: int_lat::LowerIntegrityThan<L2>,
        L1: lattice::MoreSecretThan<M1>, L2: int_lat::LowerIntegrityThan<M2>
    {
        &mut self.val
    }

    #[inline(always)]
    pub unsafe fn unwrap_consume_unsafe_static_all<M1, M2>(self) -> T
    where
        M1: lattice::MoreSecretThan<L1>, M2: int_lat::LowerIntegrityThan<L2>
    {
        self.unwrap()
    }
    #[inline(always)]
    pub unsafe fn unwrap_consume_unsafe_exact_static_all<M1, M2>(self) -> T
    where
        M1: lattice::MoreSecretThan<L1>, M2: int_lat::LowerIntegrityThan<L2>, L1: lattice::MoreSecretThan<M1>, L2: int_lat::LowerIntegrityThan<M2>
    {
        self.unwrap()
    }
}

impl<T, L, D> StaticDynamicSecret<T, L, D>
    where T: SecureValueSafe, D: DynField<Sec>
{
    #[inline(always)]
    pub unsafe fn new_static_dynamic_secret(val: T, label: D) -> StaticDynamicSecret<T, L, D> {
        StaticDynamicSecret::<T, L, D> {
            val: val,
            _pd_secrecy: PhantomData,
            _pd_integrity: PhantomData,
            dynamic_secrecy: label,
            dynamic_integrity: ()
        }
    }
}

impl<T, L, D> StaticSecretDynamicIntegrity<T, L, D>
    where T: SecureValueSafe, D: DynField<Int>
{
    #[inline(always)]
    pub unsafe fn new_static_secret_dynamic_integrity(val: T, label: D) -> StaticSecretDynamicIntegrity<T, L, D> {
        StaticSecretDynamicIntegrity::<T, L, D> {
            val: val,
            _pd_secrecy: PhantomData,
            _pd_integrity: PhantomData,
            dynamic_secrecy: (),
            dynamic_integrity: label
        }
    }
}

impl<T, L, D> DynamicSecretStaticIntegrity<T, L, D>
    where T: SecureValueSafe, D: DynField<Sec>
{
    #[inline(always)]
    pub unsafe fn new_dynamic_secret_static_integrity(val: T, label: D) -> DynamicSecretStaticIntegrity<T, L, D> {
        DynamicSecretStaticIntegrity::<T, L, D> {
            val: val,
            _pd_secrecy: PhantomData,
            _pd_integrity: PhantomData,
            dynamic_secrecy: label,
            dynamic_integrity: ()
        }
    }
}

impl<T, L, D> StaticDynamicIntegrity<T, L, D>
    where T: SecureValueSafe, D: DynField<Int>
{
    #[inline(always)]
    pub unsafe fn new_static_dynamic_integrity(val: T, label: D) -> StaticDynamicIntegrity<T, L, D> {
        StaticDynamicIntegrity::<T, L, D> {
            val: val,
            _pd_secrecy: PhantomData,
            _pd_integrity: PhantomData,
            dynamic_secrecy: (),
            dynamic_integrity: label
        }
    }
}

impl<T, D1, D2> DynamicAll<T, D1, D2>
    where T: SecureValueSafe, D1: DynField<Sec>, D2: DynField<Int>
{
    #[inline(always)]
    pub unsafe fn new_dynamic_all(val: T, label1: D1, label2: D2) -> DynamicAll<T, D1, D2> {
        DynamicAll::<T, D1, D2> {
            val: val,
            _pd_secrecy: PhantomData,
            _pd_integrity: PhantomData,
            dynamic_secrecy: label1,
            dynamic_integrity: label2
        }
    }
}

impl<T, L1, L2, D> StaticDynamicSecretStaticIntegrity<T, L1, L2, D>
    where T: SecureValueSafe, D: DynField<Sec>
{
    #[inline(always)]
    pub unsafe fn new_static_dynamic_secret_static_integrity(val: T, label: D) -> StaticDynamicSecretStaticIntegrity<T, L1, L2, D> {
        StaticDynamicSecretStaticIntegrity::<T, L1, L2, D> {
            val: val,
            _pd_secrecy: PhantomData,
            _pd_integrity: PhantomData,
            dynamic_secrecy: label,
            dynamic_integrity: ()
        }
    }

    #[inline(always)]
    pub unsafe fn unwrap_mut_unsafe_dynamic_secret<M1, M2>(&mut self) -> &mut T
    where
        M1: lattice::MoreSecretThan<L1>, M2: int_lat::LowerIntegrityThan<L2>,
        L1: lattice::MoreSecretThan<M1>, L2: int_lat::LowerIntegrityThan<M2>
    {
        &mut self.val
    }

    #[inline(always)]
    pub unsafe fn unwrap_consume_unsafe_dynamic_secret<M1, M2>(self) -> T
    where
        M1: lattice::MoreSecretThan<L1>, M2: int_lat::LowerIntegrityThan<L2>
    {
        self.unwrap()
    }
}

impl<T, L1, L2, D> StaticSecretStaticDynamicIntegrity<T, L1, L2, D>
    where T: SecureValueSafe, D: DynField<Int>
{
    #[inline(always)]
    pub unsafe fn new_static_secret_static_dynamic_integrity(val: T, label: D) -> StaticSecretStaticDynamicIntegrity<T, L1, L2, D> {
        StaticSecretStaticDynamicIntegrity::<T, L1, L2, D> {
            val: val,
            _pd_secrecy: PhantomData,
            _pd_integrity: PhantomData,
            dynamic_secrecy: (),
            dynamic_integrity: label
        }
    }

    #[inline(always)]
    pub unsafe fn unwrap_mut_unsafe_dynamic_integrity<M1, M2>(&mut self) -> &mut T
    where
        M1: lattice::MoreSecretThan<L1>, M2: int_lat::LowerIntegrityThan<L2>,
        L1: lattice::MoreSecretThan<M1>, L2: int_lat::LowerIntegrityThan<M2>
    {
        &mut self.val
    }

    #[inline(always)]
    pub unsafe fn unwrap_consume_unsafe_dynamic_integrity<M1, M2>(self) -> T
    where
        M1: lattice::MoreSecretThan<L1>, M2: int_lat::LowerIntegrityThan<L2>
    {
        self.unwrap()
    }
}

impl<T, L, D1, D2> StaticDynamicSecretDynamicIntegrity<T, L, D1, D2>
    where T: SecureValueSafe, D1: DynField<Sec>, D2: DynField<Int>
{
    #[inline(always)]
    pub unsafe fn new_static_dynamic_secret_dynamic_integrity(val: T, label1: D1, label2: D2) -> StaticDynamicSecretDynamicIntegrity<T, L, D1, D2> {
        StaticDynamicSecretDynamicIntegrity::<T, L, D1, D2> {
            val: val,
            _pd_secrecy: PhantomData,
            _pd_integrity: PhantomData,
            dynamic_secrecy: label1,
            dynamic_integrity: label2
        }
    }
}

impl<T, L, D1, D2> DynamicSecretStaticDynamicIntegrity<T, L, D1, D2>
    where T: SecureValueSafe, D1: DynField<Sec>, D2: DynField<Int>
{
    #[inline(always)]
    pub unsafe fn new_dynamic_secret_static_dynamic_integrity(val: T, label1: D1, label2: D2) -> DynamicSecretStaticDynamicIntegrity<T, L, D1, D2> {
        DynamicSecretStaticDynamicIntegrity::<T, L, D1, D2> {
            val: val,
            _pd_secrecy: PhantomData,
            _pd_integrity: PhantomData,
            dynamic_secrecy: label1,
            dynamic_integrity: label2
        }
    }
}

impl<T, L1, L2, D1, D2> SecureValue<T, L1, L2, D1, D2>
    where T: SecureValueSafe, D1: DynField<Sec>, D2: DynField<Int>
{
    #[inline(always)]
    pub unsafe fn new_info_flow_struct(val: T, label1: D1, label2: D2) -> SecureValue<T, L1, L2, D1, D2> {
        SecureValue::<T, L1, L2, D1, D2> {
            val: val,
            _pd_secrecy: PhantomData,
            _pd_integrity: PhantomData,
            dynamic_secrecy: label1,
            dynamic_integrity: label2
        }
    }

    #[inline(always)]
    pub unsafe fn unwrap_unsafe2<M1, M2>(&self) -> &T
    where
        M1: lattice::MoreSecretThan<L1>, M2: int_lat::LowerIntegrityThan<L2>
    {
        &self.val
    }

    #[inline(always)]
    pub unsafe fn unwrap_mut_unsafe_dynamic_all<M1, M2>(&mut self) -> &mut T
    where
        M1: lattice::MoreSecretThan<L1>, M2: int_lat::LowerIntegrityThan<L2>,
        L1: lattice::MoreSecretThan<M1>, L2: int_lat::LowerIntegrityThan<M2>
    {
        &mut self.val
    }

    #[inline(always)]
    pub unsafe fn unwrap_mut_unsafe2<M1, M2>(&mut self) -> &mut T
    where
        M1: lattice::MoreSecretThan<L1>, M2: int_lat::LowerIntegrityThan<L2>,
        L1: lattice::MoreSecretThan<M1>, L2: int_lat::LowerIntegrityThan<M2>
    {
        &mut self.val
    }

    #[inline(always)]
    pub unsafe fn unwrap_consume_unsafe_dynamic_all<M1, M2>(self) -> T
    where
        M1: lattice::MoreSecretThan<L1>, M2: int_lat::LowerIntegrityThan<L2>
    {
        self.unwrap()
    }

    #[inline(always)]
    pub unsafe fn unwrap_consume_unsafe2<M1, M2>(self) -> T
    where
        M1: lattice::MoreSecretThan<L1>, M2: int_lat::LowerIntegrityThan<L2>
    {
        self.unwrap()
    }

    /*pub unsafe fn print_dynamic_secrecy(&self) {
        self.dynamic_secrecy.print_self();
    }

    pub unsafe fn print_dynamic_integrity(&self) {
        self.dynamic_integrity.print_self();
    }*/

    #[inline(always)]
    pub fn get_dyn_sec_label_ref(&self) -> &D1 {
        &self.dynamic_secrecy
    }

    #[inline(always)]
    pub fn get_dyn_int_label_ref(&self) -> &D2 {
        &self.dynamic_integrity
    }

    #[inline]
    pub fn get_dyn_sec_label(&self) -> DynLabel<Sec> {
        #[cfg(feature = "benchmarking")]
        unsafe { LABEL_CLONE_COUNTER.increment(); }
        self.get_dyn_sec_label_ref().generate_dynamic_label().clone()
    }

    #[inline]
    pub fn get_dyn_int_label(&self) -> DynLabel<Int> {
        #[cfg(feature = "benchmarking")]
        unsafe { LABEL_CLONE_COUNTER.increment(); }
        self.get_dyn_int_label_ref().generate_dynamic_label().clone()
    }

    #[inline(always)]
    fn unwrap(self) -> T {
        self.val
    }
}

// Note: Cannot print value if L: lattice::Label_Empty since it results in conflicting
// implementations and negative impls aren't supported
impl<T: SecureValueSafe, L1, L2, D1: DynField<Sec>, D2: DynField<Int>> fmt::Display for SecureValue<T, L1, L2, D1, D2> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(info flow protected)")
    }
}

impl<T: SecureValueSafe, L1, L2, D1: DynField<Sec>, D2: DynField<Int>> fmt::Debug for SecureValue<T, L1, L2, D1, D2> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(info flow protected)")
    }
}

impl<T> Secret<T, lattice::Label_Empty>
where
    T: SecureValueSafe,
{
    /**
     * Returns a borrow of the interior value of self.
     * Only valid on public data.
     */
    pub fn get_value_ref(&self) -> &T {
        &self.val
    }

    /**
     * Returns the interior value of self, consuming self in the process.
     * Only valid on public data.
     */
    pub fn get_value_consume(self) -> T {
        self.val
    }
}

impl<T: SecureValueSafe, L2: lattice::Label, D1: DynField<Sec>, D2: DynField<Int>> SecureValue<T, lattice::Label_Empty, L2, D1, D2> {
    pub unsafe fn print_static_secret_label(&self) {
        println!("sec_lat::Label_Empty");
    }
}
impl<T: SecureValueSafe, L2: lattice::Label, D1: DynField<Sec>, D2: DynField<Int>> SecureValue<T, lattice::Label_A, L2, D1, D2> {
    pub unsafe fn print_static_secret_label(&self) {
        println!("sec_lat::Label_A");
    }
}
impl<T: SecureValueSafe, L2: lattice::Label, D1: DynField<Sec>, D2: DynField<Int>> SecureValue<T, lattice::Label_B, L2, D1, D2> {
    pub unsafe fn print_static_secret_label(&self) {
        println!("sec_lat::Label_B");
    }
}
impl<T: SecureValueSafe, L2: lattice::Label, D1: DynField<Sec>, D2: DynField<Int>> SecureValue<T, lattice::Label_C, L2, D1, D2> {
    pub unsafe fn print_static_secret_label(&self) {
        println!("sec_lat::Label_C");
    }
}
impl<T: SecureValueSafe, L2: lattice::Label, D1: DynField<Sec>, D2: DynField<Int>> SecureValue<T, lattice::Label_AB, L2, D1, D2> {
    pub unsafe fn print_static_secret_label(&self) {
        println!("sec_lat::Label_AB");
    }
}
impl<T: SecureValueSafe, L2: lattice::Label, D1: DynField<Sec>, D2: DynField<Int>> SecureValue<T, lattice::Label_BC, L2, D1, D2> {
    pub unsafe fn print_static_secret_label(&self) {
        println!("sec_lat::Label_BC");
    }
}
impl<T: SecureValueSafe, L2: lattice::Label, D1: DynField<Sec>, D2: DynField<Int>> SecureValue<T, lattice::Label_AC, L2, D1, D2> {
    pub unsafe fn print_static_secret_label(&self) {
        println!("sec_lat::Label_AC");
    }
}
impl<T: SecureValueSafe, L2: lattice::Label, D1: DynField<Sec>, D2: DynField<Int>> SecureValue<T, lattice::Label_ABC, L2, D1, D2> {
    pub unsafe fn print_static_secret_label(&self) {
        println!("sec_lat::Label_ABC");
    }
}

impl<T: SecureValueSafe, L1: lattice::Label, D1: DynField<Sec>, D2: DynField<Int>> SecureValue<T, L1, int_lat::Label_All, D1, D2> {
    pub unsafe fn print_static_integrity_label(&self) {
        println!("int_lat::Label_All");
    }
}
impl<T: SecureValueSafe, L1: lattice::Label, D1: DynField<Sec>, D2: DynField<Int>> SecureValue<T, L1, int_lat::Label_NotX, D1, D2> {
    pub unsafe fn print_static_integrity_label(&self) {
        println!("int_lat::Label_NotX");
    }
}
impl<T: SecureValueSafe, L1: lattice::Label, D1: DynField<Sec>, D2: DynField<Int>> SecureValue<T, L1, int_lat::Label_NotY, D1, D2> {
    pub unsafe fn print_static_integrity_label(&self) {
        println!("int_lat::Label_NotY");
    }
}
impl<T: SecureValueSafe, L1: lattice::Label, D1: DynField<Sec>, D2: DynField<Int>> SecureValue<T, L1, int_lat::Label_NotZ, D1, D2> {
    pub unsafe fn print_static_integrity_label(&self) {
        println!("int_lat::Label_NotZ");
    }
}
impl<T: SecureValueSafe, L1: lattice::Label, D1: DynField<Sec>, D2: DynField<Int>> SecureValue<T, L1, int_lat::Label_NotXY, D1, D2> {
    pub unsafe fn print_static_integrity_label(&self) {
        println!("int_lat::Label_NotXY");
    }
}
impl<T: SecureValueSafe, L1: lattice::Label, D1: DynField<Sec>, D2: DynField<Int>> SecureValue<T, L1, int_lat::Label_NotYZ, D1, D2> {
    pub unsafe fn print_static_integrity_label(&self) {
        println!("int_lat::Label_NotYZ");
    }
}
impl<T: SecureValueSafe, L1: lattice::Label, D1: DynField<Sec>, D2: DynField<Int>> SecureValue<T, L1, int_lat::Label_NotXZ, D1, D2> {
    pub unsafe fn print_static_integrity_label(&self) {
        println!("int_lat::Label_NotXZ");
    }
}
impl<T: SecureValueSafe, L1: lattice::Label, D1: DynField<Sec>, D2: DynField<Int>> SecureValue<T, L1, int_lat::Label_NotXYZ, D1, D2> {
    pub unsafe fn print_static_integrity_label(&self) {
        println!("int_lat::Label_NotXYZ");
    }
}

impl<T: SecureValueSafe, L1: lattice::Label, L2: lattice::Label, D1: DynField<Sec>, D2: DynField<Int>> SecureValue<T, L1, L2, D1, D2> {
    pub unsafe fn print_dynamic_secret_label(&self) {
        print!("dyn_sec: ");
        self.dynamic_secrecy.print_self();
    }
    pub unsafe fn print_dynamic_integrity_label(&self) {
        print!("dyn_int: ");
        self.dynamic_integrity.print_self();
    }
}
