use super::*;

macro_rules! impl_init {
    ($(
        impl($($gen:tt)*) Init($U:ty) for $T:ty {
            get($($self:tt)*) { $($func:tt)* }
        }
    )*) => {$(
        impl<$($gen)*, ImplInitF: FieldName> Init<ImplInitF, $U> for $T {
            #[inline(always)]
            fn get($($self)*) -> $U {
                $($func)*
            }
        }

        impl<$($gen)*, ImplInitF: FieldName> MaybeInit<ImplInitF, $U> for $T {
            #[inline(always)]
            fn get(self) -> Option<$U> {
                Some(Init::<ImplInitF, _>::get(self))
            }
        }
    )*};
}

impl<U: FieldName, T> MaybeInit<U, T> for Uninit<U, T> {
    #[inline(always)]
    fn get(self) -> Option<T> {
        None
    }
}

impl<U: FieldName, T> MaybeInit<U, (T,)> for T {
    #[inline(always)]
    fn get(self) -> Option<(T,)> {
        Some(Init::<U, (T,)>::get(self))
    }
}

impl<U: FieldName, T> Init<U, (T,)> for T {
    #[inline(always)]
    fn get(self) -> (T,) {
        (self,)
    }
}

// impl<U: FieldName, T> MaybeInit<U, T> for Option<T> {
//     #[inline(always)]
//     fn get(self) -> Option<T> {
//         self
//     }
// }

impl_init! {
    // impl(T) Init(T) for T {
    //     get(self) { self }
    // }

    // impl(T) Init((T,)) for T {
    //     get(self) { (self,) }
    // }

    // impl(T) Init([T; 1]) for T {
    //     get(self) { [self] }
    // }

    // impl(T) Init(Option<T>) for T {
    //     get(self) { Some(self) }
    // }
}

/*
impl_init! {
    impl('a, T) Init(T) for &'a mut dyn (FnMut() -> T) {
        get(self) { self() }
    }
    
    impl('a, T) Init(T) for &'a dyn Fn() -> T {
        get(self) { self() }
    }
    
    impl(T) Init(T) for fn() -> T {
        get(self) { self() }
    }
}

macro_rules! fn_impl {
    ($($bounds:ident)*) => {
        #[cfg(feature = "std")]
        impl_init! {
            impl(T) Init(T) for std::sync::Arc<dyn Fn() -> T $(+ $bounds)*> {
                get(self) { self() }
            }
            
            impl(T) Init(T) for std::rc::Rc<dyn Fn() -> T $(+ $bounds)*> {
                get(self) { self() }
            }
            
            impl(T) Init(T) for Box<dyn FnMut() -> T $(+ $bounds)*> {
                get(mut self) { self() }
            }
            
            impl(T) Init(T) for Box<dyn Fn() -> T $(+ $bounds)*> {
                get(self) { self() }
            }
        }
    };
}

fn_impl! {  }
fn_impl! { Send }
fn_impl! { Sync }
fn_impl! { Send Sync }
*/

/*
    #[cfg(feature = "std")]
    impl_init! {
        impl(T) Init(Box<T>) for T {
            get(self) { Box::new(self) }
        }
    }

    #[cfg(feature = "std")]
    impl_init! {
        impl('a) Init(String) for &'a str {
            get(self) { self.to_owned() }
        }
    }

    #[cfg(feature = "std")]
    impl_init! {
        impl('a) Init(std::ffi::CString) for &'a std::ffi::CStr {
            get(self) { self.to_owned() }
        }
    }

    #[cfg(feature = "std")]
    impl_init! {
        impl('a) Init(std::ffi::OsString) for &'a std::ffi::OsStr {
            get(self) { self.to_owned() }
        }
    }

    #[cfg(feature = "std")]
    impl_init! {
        impl('a) Init(std::path::PathBuf) for &'a std::path::Path {
            get(self) { self.to_owned() }
        }
    }

    #[cfg(feature = "std")]
    impl_init! {
        impl('a, T: Clone) Init(Vec<T>) for &'a [T] {
            get(self) { self.to_vec() }
        }
    }
*/