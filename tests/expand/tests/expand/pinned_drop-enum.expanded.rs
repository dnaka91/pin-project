use pin_project::{pin_project, pinned_drop};
use std::pin::Pin;
#[pin(__private(PinnedDrop))]
enum Enum<T, U> {
    Struct {
        #[pin]
        pinned: T,
        unpinned: U,
    },
    Tuple(#[pin] T, U),
    Unit,
}
#[doc(hidden)]
#[allow(clippy::mut_mut)]
#[allow(dead_code)]
#[allow(single_use_lifetimes)]
enum __EnumProjection<'pin, T, U>
where
    Enum<T, U>: 'pin,
{
    Struct {
        pinned: ::pin_project::__reexport::pin::Pin<&'pin mut (T)>,
        unpinned: &'pin mut (U),
    },
    Tuple(
        ::pin_project::__reexport::pin::Pin<&'pin mut (T)>,
        &'pin mut (U),
    ),
    Unit,
}
#[doc(hidden)]
#[allow(dead_code)]
#[allow(single_use_lifetimes)]
enum __EnumProjectionRef<'pin, T, U>
where
    Enum<T, U>: 'pin,
{
    Struct {
        pinned: ::pin_project::__reexport::pin::Pin<&'pin (T)>,
        unpinned: &'pin (U),
    },
    Tuple(::pin_project::__reexport::pin::Pin<&'pin (T)>, &'pin (U)),
    Unit,
}
#[doc(hidden)]
#[allow(non_upper_case_globals)]
#[allow(single_use_lifetimes)]
const __SCOPE_Enum: () = {
    impl<T, U> Enum<T, U> {
        fn project<'pin>(
            self: ::pin_project::__reexport::pin::Pin<&'pin mut Self>,
        ) -> __EnumProjection<'pin, T, U> {
            unsafe {
                match self.get_unchecked_mut() {
                    Enum::Struct { pinned, unpinned } => __EnumProjection::Struct {
                        pinned: ::pin_project::__reexport::pin::Pin::new_unchecked(pinned),
                        unpinned,
                    },
                    Enum::Tuple(_0, _1) => __EnumProjection::Tuple(
                        ::pin_project::__reexport::pin::Pin::new_unchecked(_0),
                        _1,
                    ),
                    Enum::Unit => __EnumProjection::Unit,
                }
            }
        }
        fn project_ref<'pin>(
            self: ::pin_project::__reexport::pin::Pin<&'pin Self>,
        ) -> __EnumProjectionRef<'pin, T, U> {
            unsafe {
                match self.get_ref() {
                    Enum::Struct { pinned, unpinned } => __EnumProjectionRef::Struct {
                        pinned: ::pin_project::__reexport::pin::Pin::new_unchecked(pinned),
                        unpinned,
                    },
                    Enum::Tuple(_0, _1) => __EnumProjectionRef::Tuple(
                        ::pin_project::__reexport::pin::Pin::new_unchecked(_0),
                        _1,
                    ),
                    Enum::Unit => __EnumProjectionRef::Unit,
                }
            }
        }
    }
    struct __Enum<'pin, T, U> {
        __pin_project_use_generics: ::pin_project::__private::AlwaysUnpin<'pin, (T, U)>,
        __field0: T,
        __field1: T,
    }
    impl<'pin, T, U> ::pin_project::__reexport::marker::Unpin for Enum<T, U> where
        __Enum<'pin, T, U>: ::pin_project::__reexport::marker::Unpin
    {
    }
    impl<T, U> ::pin_project::__reexport::ops::Drop for Enum<T, U> {
        fn drop(&mut self) {
            let pinned_self = unsafe { ::pin_project::__reexport::pin::Pin::new_unchecked(self) };
            unsafe {
                ::pin_project::__private::PinnedDrop::drop(pinned_self);
            }
        }
    }
};
impl<T, U> ::pin_project::__private::PinnedDrop for Enum<T, U> {
    unsafe fn drop(self: Pin<&mut Self>) {
        #[allow(clippy::needless_pass_by_value)]
        fn __drop_inner<T, U>(__self: Pin<&mut Enum<T, U>>) {
            let _this = __self;
        }
        __drop_inner(self);
    }
}
fn main() {}