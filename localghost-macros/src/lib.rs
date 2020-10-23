//! Async fn main support for WASM.

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![recursion_limit = "512"]

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

/// Initializes an `async` main function.
///
/// This enables the use of `async/.await` from the root of the program, logs any
/// errors returned from the program, and ensures [panics are
/// logged to the console](https://docs.rs/console_error_panic_hook/0.1.6/console_error_panic_hook/index.html).
///
/// # Examples
///
/// ```ignore
/// #[localghost::main]
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
    // let attrs = &input.attrs;

    if name != "main" {
        return TokenStream::from(quote_spanned! { name.span() =>
            compile_error!("only the main function can be tagged with #[localghost::main]"),
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
        pub fn main() {
            ::localghost::macro_export::set_panic_hook();
            ::localghost::log::Logger::new().start().unwrap_throw();

            async fn main(#inputs) #ret #body

            localghost::task::spawn_local(async {
                main().await#end;
            });
        }

    };

    result.into()
}
