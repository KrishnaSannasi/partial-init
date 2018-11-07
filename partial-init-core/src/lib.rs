#![forbid(unsafe_code, missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
// #![feature(optin_builtin_traits)]

//! Partial Init
//! ---
//!
//! This crate exists to provide type information necessary to make partial-init-derive.
//!
//! This crate is used by partial-init-derive, please use that crate to implement `PartialInit`,
//! please see partial-init-derive for usage and more information.

#[cfg(not(feature = "std"))]
extern crate core as std;

mod init_impl;

pub use std::marker::PhantomData;

/// An identity function.
/// This is a function to hold over till
/// [std::convert::identity](https://doc.rust-lang.org/std/convert/fn.identity.html)
/// becomes stable
pub fn identity<T>(x: T) -> T { x }

/// This trait specifies what uninitialized value to
/// make uninitialized values without conflicting with
/// inherent functions on a type
pub trait PartialInit {
    /// The uninitialized value to be created.
    ///
    /// *note*: All uninitialized created by partial-init-derive are zero-sized
    type Uninitialized;

    /// makes an uninialized value
    fn uninit() -> Self::Uninitialized;
}

/// This is used by partial-init-derive to specify
/// field names for better error messages
pub trait FieldName { }

/// This is used by partial-init-derive to specify
/// that a field must be initialized by the user
/// i.e. that it is *not* marked `#[default]`
/// 
/// This trait helps produce better error messages
/// 
/// **Note** `get` will be called when `build` is called
/// on types created by partial-init-derive
pub trait Init<F: FieldName, T>: MaybeInit<F, T> {
    /// Gets the value of the field
    fn get(self) -> T;
}

/// This is used by partial-init-derive to specify
/// that a field must not have to be initialized by the user
/// i.e. that it *is* marked `#[default]`
/// 
/// This trait helps produce better error messages
/// 
/// **Note** `get` will be called when `build` is called
/// on types created by partial-init-derive
pub trait MaybeInit<F: FieldName, T> {
    /// Gets the value of the field if it
    /// is initialized, other-wise returns None
    fn get(self) -> Option<T>;
}

/// A zero-sized type that represents uninitailzed values
/// In the type system
pub struct Uninit<U: FieldName, T>(std::marker::PhantomData<(U, T)>);

impl<U: FieldName, T> Default for Uninit<U, T> {
    fn default() -> Self {
        Uninit(std::marker::PhantomData)
    }
}

impl<U: FieldName, T> Copy for Uninit<U, T> {}
impl<U: FieldName, T> Clone for Uninit<U, T> {
    fn clone(&self) -> Self {
        *self
    }
}

#[macro_export]
macro_rules! init {
    ($name:ident {
        $($field:ident: $value:expr),*
    }) => {
        $name::uninit()$(.$field($value))*.build()
    };
    ($(
        $name:ident {
            $($field:ident: $value:expr),*
        }
    ),*) => {
        ($(
            init! { $name { $($field: $value),* } }
        ),*)
    };
}

#[macro_export]
macro_rules! new_partial {
    ($name:ident {
        $($field:ident: $value:expr),*
    }) => {
        $name {
            $($field: $value,)*
            __phantom_data__partial_init_: Default::default()
        }
    };
    ($(
        $name:ident {
            $($field:ident: $value:expr),*
        }
    ),*) => {
        ($(
            new_partial! { $name { $($field: $value),* } }
        ),*)
    };
}
