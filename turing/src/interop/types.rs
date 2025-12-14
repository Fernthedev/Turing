use std::{
    ffi::{CStr, c_char, c_void},
    hash::Hash,
    ops::Deref,
    ptr,
};

use crate::ffi::Ext;

/// A string allocated externally, to be managed by the external environment.
pub struct ExtString {
    pub ptr: *const c_char,
}

impl ExtString {
    pub fn new(ptr: *const c_char) -> Self {
        ExtString { ptr }
    }
}

impl Drop for ExtString {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            Ext::free_string(self.ptr);
        }
    }
}

/// An externally managed pointer, to be freed by the external environment.
#[derive(Debug)]
pub struct ExtPointer<T> {
    pub ptr: *const T,
}

impl <T> ExtPointer<T> {
    pub fn new(ptr: *const T) -> Self {
        ExtPointer { ptr }
    }

    pub fn null() -> Self {
        ExtPointer { ptr: ptr::null() }
    }
}

unsafe impl<T> Send for ExtPointer<T> {}
unsafe impl<T> Sync for ExtPointer<T> {}

impl<T> Deref for ExtPointer<T> {
    type Target = *const T;

    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}

impl<T> From<*const T> for ExtPointer<T> {
    fn from(ptr: *const T) -> Self {
        ExtPointer { ptr }
    }
}

impl<T> Hash for ExtPointer<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.ptr as *const c_void).hash(state);
    }
}

impl<T> Clone for ExtPointer<T> {
    fn clone(&self) -> Self {
        ExtPointer { ptr: self.ptr }
    }
}
impl<T> Copy for ExtPointer<T> {}
impl<T> PartialEq for ExtPointer<T> {
    fn eq(&self, other: &Self) -> bool {
        ptr::addr_eq(self.ptr, other.ptr)
    }
}
impl<T> Eq for ExtPointer<T> {}
impl<T> Default for ExtPointer<T> {
    fn default() -> Self {
        ExtPointer { ptr: ptr::null() }
    }
}

impl From<&CStr> for ExtString {
    fn from(s: &CStr) -> Self {
        ExtString {
            ptr: s.as_ptr() as *mut c_char,
        }
    }
}

impl From<*const c_char> for ExtString {
    fn from(ptr: *const c_char) -> Self {
        ExtString {
            ptr: ptr as *mut c_char,
        }
    }
}

impl Deref for ExtString {
    type Target = CStr;

    fn deref(&self) -> &Self::Target {
        unsafe { CStr::from_ptr(self.ptr) }
    }
}

impl std::fmt::Display for ExtString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.deref().to_string_lossy())
    }
}
