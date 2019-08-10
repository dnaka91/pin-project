#![recursion_limit = "256"]
#![doc(html_root_url = "https://docs.rs/pin-project-internal/0.3.3")]
#![doc(test(attr(deny(warnings), allow(dead_code, unused_assignments, unused_variables))))]
#![warn(unsafe_code)]
#![warn(rust_2018_idioms, unreachable_pub)]
#![warn(single_use_lifetimes)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::use_self)]

extern crate proc_macro;

#[macro_use]
mod utils;

mod pin_projectable;
#[cfg(feature = "project_attr")]
mod project;

use proc_macro::TokenStream;

use crate::utils::Nothing;

#[cfg(feature = "project_attr")]
#[proc_macro_attribute]
pub fn project(args: TokenStream, input: TokenStream) -> TokenStream {
    let _: Nothing = syn::parse_macro_input!(args);
    project::attribute(input.into()).into()
}

/// This is a doc comment from the defining crate!
#[proc_macro]
pub fn pin_project(input: TokenStream) -> TokenStream {
    pin_projectable::pin_project(input.into()).unwrap_or_else(|e| e.to_compile_error()).into()
}

#[proc_macro_attribute]
pub fn pin_projectable(args: TokenStream, input: TokenStream) -> TokenStream {
    pin_projectable::attribute(args.into(), input.into())
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

#[cfg(feature = "renamed")]
lazy_static::lazy_static! {
    pub(crate) static ref PIN_PROJECT_CRATE: String = {
        proc_macro_crate::crate_name("pin-project")
            .expect("pin-project-internal was used without pin-project!")
    };
}