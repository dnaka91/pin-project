// Original code (./struct-default.rs):
//
// ```rust
// #![allow(dead_code)]
//
// use pin_project::pin_project;
//
// #[pin_project(project_replace)]
// struct Struct<T, U> {
//     #[pin]
//     pinned: T,
//     unpinned: U,
// }
//
// fn main() {}
// ```

#![allow(dead_code, unused_imports, unused_parens, unknown_lints, renamed_and_removed_lints)]

use pin_project::pin_project;

// #[pin_project(project_replace)]
struct Struct<T, U> {
    // #[pin]
    pinned: T,
    unpinned: U,
}

#[doc(hidden)]
#[allow(non_upper_case_globals)]
#[allow(clippy::used_underscore_binding)]
#[allow(box_pointers)]
#[allow(explicit_outlives_requirements)]
#[allow(single_use_lifetimes)]
#[allow(clippy::pattern_type_mismatch)]
const _: () = {
    #[allow(dead_code)]
    #[allow(clippy::mut_mut)]
    #[allow(clippy::type_repetition_in_bounds)]
    #[allow(box_pointers)]
    #[allow(explicit_outlives_requirements)]
    #[allow(single_use_lifetimes)]
    #[allow(clippy::pattern_type_mismatch)]
    struct __StructProjection<'pin, T, U>
    where
        Struct<T, U>: 'pin,
    {
        pinned: ::pin_project::__private::Pin<&'pin mut (T)>,
        unpinned: &'pin mut (U),
    }
    #[allow(dead_code)]
    #[allow(clippy::type_repetition_in_bounds)]
    #[allow(box_pointers)]
    #[allow(explicit_outlives_requirements)]
    #[allow(single_use_lifetimes)]
    #[allow(clippy::pattern_type_mismatch)]
    struct __StructProjectionRef<'pin, T, U>
    where
        Struct<T, U>: 'pin,
    {
        pinned: ::pin_project::__private::Pin<&'pin (T)>,
        unpinned: &'pin (U),
    }
    #[allow(dead_code)]
    #[allow(unreachable_pub)]
    #[allow(box_pointers)]
    #[allow(explicit_outlives_requirements)]
    #[allow(single_use_lifetimes)]
    #[allow(clippy::pattern_type_mismatch)]
    struct __StructProjectionOwned<T, U> {
        pinned: ::pin_project::__private::PhantomData<T>,
        unpinned: U,
    }

    impl<T, U> Struct<T, U> {
        fn project<'pin>(
            self: ::pin_project::__private::Pin<&'pin mut Self>,
        ) -> __StructProjection<'pin, T, U> {
            unsafe {
                let Self { pinned, unpinned } = self.get_unchecked_mut();
                __StructProjection {
                    pinned: ::pin_project::__private::Pin::new_unchecked(pinned),
                    unpinned,
                }
            }
        }
        fn project_ref<'pin>(
            self: ::pin_project::__private::Pin<&'pin Self>,
        ) -> __StructProjectionRef<'pin, T, U> {
            unsafe {
                let Self { pinned, unpinned } = self.get_ref();
                __StructProjectionRef {
                    pinned: ::pin_project::__private::Pin::new_unchecked(pinned),
                    unpinned,
                }
            }
        }
        fn project_replace(
            self: ::pin_project::__private::Pin<&mut Self>,
            __replacement: Self,
        ) -> __StructProjectionOwned<T, U> {
            unsafe {
                let __self_ptr: *mut Self = self.get_unchecked_mut();
                let Self { pinned, unpinned } = &mut *__self_ptr;

                // First, extract all the unpinned fields
                let __result = __StructProjectionOwned {
                    pinned: ::pin_project::__private::PhantomData,
                    unpinned: ::pin_project::__private::ptr::read(unpinned),
                };

                // Destructors will run in reverse order, so next create a guard to overwrite
                // `self` with the replacement value without calling destructors.
                let __guard = ::pin_project::__private::UnsafeOverwriteGuard {
                    target: __self_ptr,
                    value: ::pin_project::__private::ManuallyDrop::new(__replacement),
                };

                // Now create guards to drop all the pinned fields
                //
                // Due to a compiler bug (https://github.com/rust-lang/rust/issues/47949)
                // this must be in its own scope, or else `__result` will not be dropped
                // if any of the destructors panic.
                {
                    let __guard = ::pin_project::__private::UnsafeDropInPlaceGuard(pinned);
                }

                // Finally, return the result
                __result
            }
        }
    }

    // Automatically create the appropriate conditional `Unpin` implementation.
    //
    // See ./struct-default-expanded.rs and https://github.com/taiki-e/pin-project/pull/53.
    // for details.
    struct __Struct<'pin, T, U> {
        __pin_project_use_generics: ::pin_project::__private::AlwaysUnpin<
            'pin,
            (::pin_project::__private::PhantomData<T>, ::pin_project::__private::PhantomData<U>),
        >,
        __field0: T,
    }
    impl<'pin, T, U> ::pin_project::__private::Unpin for Struct<T, U> where
        __Struct<'pin, T, U>: ::pin_project::__private::Unpin
    {
    }
    // A dummy impl of `UnsafeUnpin`, to ensure that the user cannot implement it.
    #[doc(hidden)]
    unsafe impl<'pin, T, U> ::pin_project::UnsafeUnpin for Struct<T, U> where
        __Struct<'pin, T, U>: ::pin_project::__private::Unpin
    {
    }

    // Ensure that struct does not implement `Drop`.
    //
    // See ./struct-default-expanded.rs for details.
    trait StructMustNotImplDrop {}
    #[allow(clippy::drop_bounds, drop_bounds)]
    impl<T: ::pin_project::__private::Drop> StructMustNotImplDrop for T {}
    impl<T, U> StructMustNotImplDrop for Struct<T, U> {}
    // A dummy impl of `PinnedDrop`, to ensure that users don't accidentally
    // write a non-functional `PinnedDrop` impls.
    #[doc(hidden)]
    impl<T, U> ::pin_project::__private::PinnedDrop for Struct<T, U> {
        unsafe fn drop(self: ::pin_project::__private::Pin<&mut Self>) {}
    }

    // Ensure that it's impossible to use pin projections on a #[repr(packed)]
    // struct.
    //
    // See ./struct-default-expanded.rs and https://github.com/taiki-e/pin-project/pull/34
    // for details.
    #[forbid(safe_packed_borrows)]
    fn __assert_not_repr_packed<T, U>(this: &Struct<T, U>) {
        let _ = &this.pinned;
        let _ = &this.unpinned;
    }
};

fn main() {}
