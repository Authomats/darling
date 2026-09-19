use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;

/// Wraps all output inside of an anonymous `const` block
///
/// Because trait implementations are hoisted to the top-level, it is possible to
/// create them inside of the `const` block and take effect after
///
/// This is done to support users defining a custom location for the `darling` crate
///
/// If not wrapped in a `const` block, then multiple uses of `derive` on `darling`'s
/// macros will create errors, because there may be multiple `extern crate darling as _darling`
///
/// # Arguments
///
/// - `tokens`: The trait implementations to wrap inside of `const` block
/// - `krate`: Path to the darling crate, which defaults to `darling`
pub fn wrap_in_const<T: ToTokens>(tokens: &T, krate: Option<&syn::Path>) -> TokenStream {
    // Check if user depends on hicore
    let should_fake_original = match proc_macro_crate::crate_name("hicore") {
        Err(_) | Ok(proc_macro_crate::FoundCrate::Itself) => true,
        Ok(proc_macro_crate::FoundCrate::Name(name)) => {
            debug_assert_eq!(&name, "hicore");
            false
        }
    };

    let use_darling = if should_fake_original {
        krate.map_or_else(
            || quote! { use ::darling as _darling; },
            |krate| {
                quote_spanned! { krate.span() => use #krate as _darling; }
            },
        )
    } else {
        assert!(
            krate.is_none(),
            "Cannot change path of modified darling imported via hicore, import by yourself to do that!"
        );
        quote! { use ::hicore::darling as _darling; }
    };

    quote! {
        #[doc(hidden)]
        const _: () = {
            #use_darling

            #tokens
        };
    }
}
