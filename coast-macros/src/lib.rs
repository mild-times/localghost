//! Async fn main support for WASM.

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![recursion_limit = "512"]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

/// Enables an async main function.
///
/// # Examples
///
/// ```ignore
/// #[coast::main]
/// async fn main() -> std::io::Result<()> {
///     Ok(())
/// }
/// ```
#[cfg(not(test))] // NOTE: exporting main breaks tests, we should file an issue.
#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let ret = &input.sig.output;
    let inputs = &input.sig.inputs;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;

    if name != "main" {
        return TokenStream::from(quote_spanned! { name.span() =>
            compile_error!("only the main function can be tagged with #[coast::main]"),
        });
    }

    if input.sig.asyncness.is_none() {
        return TokenStream::from(quote_spanned! { input.span() =>
            compile_error!("the async keyword is missing from the function declaration"),
        });
    }

    let end = match ret {
        syn::ReturnType::Default => quote! {;},
        _ => quote! {.unwrap_throw();},
    };

    let result = quote! {
        #[wasm_bindgen(start)]
        pub fn main() #ret {
            #(#attrs)*
            async fn main(#inputs) #ret {
                #body
            }

            wasm_bindgen_futures::spawn_local(async {
                main().await#end;
            })
        }

    };

    result.into()
}
