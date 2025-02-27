use crate::secret::{InvisibleSideEffectFree, VettedTrait, vetted_wrap};
use std::vec::IntoIter;
use std::iter::Fuse;

pub trait AllowlistedLen {
    unsafe fn __len_secret_trampoline_wrapper(&self) -> impl VettedTrait<usize>;
}
  
impl<T> AllowlistedLen for ::std::collections::HashSet<T> {
    unsafe fn __len_secret_trampoline_wrapper(&self) -> impl VettedTrait<usize> {
        let result = self.len();
        unsafe { vetted_wrap(result) }
    }
}

impl<T> AllowlistedLen for ::std::vec::Vec<T> {
    unsafe fn __len_secret_trampoline_wrapper(&self) -> impl VettedTrait<usize> {
        let result = self.len();
        unsafe { vetted_wrap(result) }
    }
}
  
impl AllowlistedLen for String {
    unsafe fn __len_secret_trampoline_wrapper(&self) -> impl VettedTrait<usize> {
        let result = self.len();
        unsafe { vetted_wrap(result) }
    }
}
  
pub trait AllowlistedClone {
    unsafe fn __clone_secret_trampoline_wrapper(&self) -> impl VettedTrait<Self>
        where Self: Sized + InvisibleSideEffectFree;
}
  
impl AllowlistedClone for String {
    unsafe fn __clone_secret_trampoline_wrapper(&self) -> impl VettedTrait<String> {
        let result = self.clone();
        unsafe { vetted_wrap(result) }
    }
}
  
pub trait AllowlistedPush<T> {
    unsafe fn __push_secret_trampoline_wrapper(&mut self, val: T) -> impl VettedTrait<()>;
}
  
impl<T> AllowlistedPush<T> for Vec<T> {
    unsafe fn __push_secret_trampoline_wrapper(&mut self, val: T) -> impl VettedTrait<()> {
        self.push(val);
        unsafe { vetted_wrap(()) }
    }
}
  
impl AllowlistedPush<char> for String {
    unsafe fn __push_secret_trampoline_wrapper(&mut self, val: char) -> impl VettedTrait<()> {
        self.push(val);
        unsafe { vetted_wrap(()) }
    }
}
  
pub trait AllowlistIsDigit {
    unsafe fn __is_digit_secret_trampoline_wrapper(self, radix: u32) -> impl VettedTrait<bool>;
}
  
impl AllowlistIsDigit for char {
    unsafe fn __is_digit_secret_trampoline_wrapper(self, radix: u32) -> impl VettedTrait<bool> {
        let result = self.is_digit(radix);
        unsafe { vetted_wrap(result) }
    }
}
  
pub trait AllowlistToString {
    unsafe fn __to_string_secret_trampoline_wrapper(&self) -> impl VettedTrait<String>;
}
  
impl AllowlistToString for str {
    unsafe fn __to_string_secret_trampoline_wrapper(&self) -> impl VettedTrait<String> {
        let result = self.to_string();
        unsafe { vetted_wrap(result) }
    }
}


pub trait AllowlistedGet<'a,K,V> {
    unsafe fn __get_secret_trampoline_wrapper(&'a self, key: &K) -> impl VettedTrait<Option<&'a V>>
        where K: InvisibleSideEffectFree,
            V: InvisibleSideEffectFree, V: 'a;
}

impl<'a,K,V> AllowlistedGet<'a,K,V> for ::std::collections::HashMap<K,V>
    where K: Eq + ::std::hash::Hash + InvisibleSideEffectFree,
            V: InvisibleSideEffectFree, V: 'a {
    unsafe fn __get_secret_trampoline_wrapper(&'a self, key: &K) -> impl VettedTrait<Option<&'a V>> {
        let result = self.get(key);
        unsafe { vetted_wrap(result) }
    }
}

pub trait AllowlistedUnwrap<T> {
    unsafe fn __unwrap_secret_trampoline_wrapper(self) -> impl VettedTrait<T>
        where T: InvisibleSideEffectFree;
}

impl<T> AllowlistedUnwrap<T> for Option<T> {
    unsafe fn __unwrap_secret_trampoline_wrapper(self) -> impl VettedTrait<T>
        where T: InvisibleSideEffectFree {
        let result = self.unwrap();
        unsafe { vetted_wrap(result) }
    }
}

pub trait AllowlistedSortByKey<F, K, T> {
    unsafe fn __sort_by_key_secret_trampoline_wrapper(&mut self, f: F) -> impl VettedTrait<()>
        where T: InvisibleSideEffectFree, F: FnMut(&T) -> K, K: Ord;
}

impl<F, K, T: InvisibleSideEffectFree> AllowlistedSortByKey<F, K, T> for Vec<T> {
    unsafe fn __sort_by_key_secret_trampoline_wrapper(&mut self, f: F) -> impl VettedTrait<()> 
        where F: FnMut(&T) -> K, K: Ord {
        let result = self.sort_by_key(f);
        unsafe { vetted_wrap(result) }
    }
}

pub trait AllowlistedIntoIter<T2> {
    unsafe fn __into_iter_secret_trampoline_wrapper(self) -> impl VettedTrait<T2>
        where T2: InvisibleSideEffectFree;
}

impl<T1: InvisibleSideEffectFree> AllowlistedIntoIter<IntoIter<T1>> for Vec<T1> {
    unsafe fn __into_iter_secret_trampoline_wrapper(self) -> impl VettedTrait<IntoIter<T1>> {
        let result = self.into_iter();
        unsafe { vetted_wrap(result) }
    }
}

pub trait AllowlistedFuse<T1> {
    unsafe fn __fuse_secret_trampoline_wrapper(self) -> impl VettedTrait<Fuse<IntoIter<T1>>>
        where T1: InvisibleSideEffectFree;
}

impl<T1: InvisibleSideEffectFree> AllowlistedFuse<T1> for IntoIter<T1> {
    unsafe fn __fuse_secret_trampoline_wrapper(self) -> impl VettedTrait<Fuse<IntoIter<T1>>> {
        let result = self.fuse();
        unsafe { vetted_wrap(result) }
    }
}

pub trait AllowlistedExtend<T1, T2> {
    unsafe fn __extend_secret_trampoline_wrapper(&mut self, iter: T2) -> impl VettedTrait<()>
        where T1: InvisibleSideEffectFree, T2: InvisibleSideEffectFree + std::iter::IntoIterator<Item=T1>;
}

impl<T1: InvisibleSideEffectFree, T2: InvisibleSideEffectFree + std::iter::IntoIterator<Item=T1>> AllowlistedExtend<T1, T2> for Vec<T1> {
    unsafe fn __extend_secret_trampoline_wrapper(&mut self, iter: T2) -> impl VettedTrait<()> {
        let result = self.extend(iter);
        unsafe { vetted_wrap(result) }
    }
}

pub trait AllowlistedUnwrapOr<T> {
    unsafe fn __unwrap_or_secret_trampoline_wrapper(self, default: T) -> impl VettedTrait<T>
        where T: InvisibleSideEffectFree;
}

impl<T> AllowlistedUnwrapOr<T> for Option<T> {
    unsafe fn __unwrap_or_secret_trampoline_wrapper(self, default: T) -> impl VettedTrait<T>
        where T: InvisibleSideEffectFree {
        let result = self.unwrap_or(default);
        unsafe { vetted_wrap(result) }
    }
}

pub trait AllowlistedExpect<T> {
    unsafe fn __expect_secret_trampoline_wrapper(self, msg: &str) -> impl VettedTrait<T>
        where T: InvisibleSideEffectFree;
}

impl<T> AllowlistedExpect<T> for Option<T> {
    unsafe fn __expect_secret_trampoline_wrapper(self, msg: &str) -> impl VettedTrait<T>
        where T: InvisibleSideEffectFree {
        let result = self.expect(msg);
        unsafe { vetted_wrap(result) }
    }
}