// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Copyright (c) 2026 c1ph3rC4t

//! # UwUnsafe
//!
//! Wraps functions in unsafety so you dont gotta.
//!
//! Since edition 2024 `unsafe_op_in_unsafe_fn` warns by
//! default, so a simple `unsafe fn` is no longer enough.
//! You also have to wrap the function body in an `unsafe`
//! block which pisses me off, UwUnsafe solves that with a
//! proc macro that takes things like:
//!
//! ```
//! use uwunsafe::uwu;
//!
//! #[uwu]
//! fn nya() {
//!     println!("nya :3");
//! }
//! ```
//!
//! and expands it to:
//!
//! ```
//! unsafe fn nya() {
//!     unsafe {
//!         println!("nya :3");
//!     }
//! }
//! ```
//!
//! Note: `main` gets special treatment because the compiler wont accept
//! `unsafe fn main` as an entry point and simply rejects it at compile time.
//! Therefore all functions named `main` will retain their function signature
//! since you can still name other functions than the entry point `main`.

/// Marks function signatures as `unsafe` and wraps function body
/// in an `unsafe` block.
///
/// Note: functions named `main` dont have their signatures modified
/// as to not make the compiler reject the `main` entry point.
///
/// # Example
///
/// ```
/// use uwunsafe::uwu;
///
/// #[uwu]
/// fn owo() {
///     // `unsafe` code goes here
/// }
///
/// #[uwu]
/// fn main() {
///     owo(); // `owo` gets wrapped in `main`'s `unsafe` block
/// }
/// ```
#[proc_macro_attribute]
pub fn uwu(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let func = syn::parse_macro_input!(item as syn::ItemFn);

    let attrs = &func.attrs;
    let vis = &func.vis;
    let mut sig = func.sig;
    if sig.ident != "main" {
        sig.unsafety = Some(Default::default())
    };
    let block = &func.block;

    quote::quote! {
        #(#attrs)* #vis #sig {
            unsafe #block
        }
    }
    .into()
}
