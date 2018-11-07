// #![feature(optin_builtin_traits)]
// #![no_std]

#[macro_use]
extern crate partial_init_derive;
extern crate partial_init_core;

#[derive(PartialInit)]
struct Foo<T> {
    thing: (T,),
}

fn main() {
    let foo = <Foo<_> as partial_init_core::PartialInit>::uninit().build();
}

/*
#[doc = r" This module was created by the parital-init-derive crate, see that documentation"]
#[doc = r" for detail on how this operation works (it's long and detailed)"]
#[doc = r" "]
#[doc = " This module holds types that represent meta-data about `Foo` and `PartialFoo`"]
#[doc = r" for better error messages."]
#[allow(non_camel_case_types)]
mod __Foo__ {
    #[doc = r" This type was created by the parital-init-derive crate, see that documentation"]
    #[doc = r" for detail on how this operation works (it's long and detailed)"]
    #[doc = r" "]
    #[doc = " This type represents a field on `Foo`, and is not meant to be constructed."]
    #[doc = r" It is only used as a type-tag."]
    pub enum thing {}
    impl ::partial_init_core::FieldName for thing {}
    #[doc = r" This module was created by the parital-init-derive crate, see that documentation"]
    #[doc = r" for detail on how this operation works (it's long and detailed)"]
    #[doc = r" "]
    #[doc = " This module holds types that represents an uninitialized fields on `PartialFoo`."]
    pub mod uninit {
        use super::super::*;
        #[doc = r" This type was created by the parital-init-derive crate, see that documentation"]
        #[doc = r" for detail on how this operation works (it's long and detailed)"]
        #[doc = r" "]
        #[doc = " This type represents an uninitialized field on `Foo`."]
        pub type thing<T> = ::partial_init_core::Uninit<super::thing, T>;
    }
}
#[doc = r" This type was created by the parital-init-derive crate, see that documentation"]
#[doc = r" for detail on how this operation works (it's long and detailed)"]
#[doc = r" "]
#[doc = " This type represents a partially initialized `Foo`, each of the functions"]
#[doc = r" on this type is one of four things: part of the builder api,"]
#[doc = r" a field-initializing-function (FIF), a field-deinitializing-function (FDF), or a function defined by the author of"]
#[doc = " `Foo`. All builder api functions and FIFs will be marked as such."]
#[doc = " You can create a `PartialFoo` by calling `Foo::uninit()`, then using the relavant functions listed"]
#[doc = r" below to initialize the data."]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
struct PartialFoo<T, thing: ::partial_init_core::MaybeInit<__Foo__::thing, T>> {
    thing: thing,
    __phantom_data__partial_init_: ::partial_init_core::PhantomData<(T)>,
}
#[allow(non_camel_case_types)]
impl<T> ::partial_init_core::PartialInit for Foo<T> {
    type Uninitialized = PartialFoo<T, ::partial_init_core::Uninit<__Foo__::thing, T>>;
    #[inline(always)]
    fn uninit() -> Self::Uninitialized {
        Default::default()
    }
}
impl<T> Default for PartialFoo<T, ::partial_init_core::Uninit<__Foo__::thing, T>> {
    #[inline(always)]
    fn default() -> Self {
        PartialFoo {
            __phantom_data__partial_init_: Default::default(),
            thing: Default::default(),
        }
    }
}
#[allow(non_camel_case_types)]
impl<T, thing: ::partial_init_core::Init<__Foo__::thing, T>> PartialFoo<T, thing> {
    #[doc = r" This function is part of the builder api."]
    #[doc = " It is the final step in creating a `Foo`."]
    #[inline(always)]
    fn build(self) -> Foo<T> {
        Foo {
            thing: ::partial_init_core::Init::get(self.thing),
        }
    }
}
#[allow(non_camel_case_types)]
impl<T> PartialFoo<T, ::partial_init_core::Uninit<__Foo__::thing, T>> {
    #[doc = " This is a FIF, it takes a `T` and initializes `thing`"]
    #[doc = "and has no default value"]
    #[inline(always)]
    fn thing<thing: ::partial_init_core::Init<__Foo__::thing, T>>(
        self,
        thing: thing,
    ) -> PartialFoo<T, thing> {
        PartialFoo {
            __phantom_data__partial_init_: Default::default(),
            thing,
        }
    }
}
*/