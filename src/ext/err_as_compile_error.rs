use proc_macro2::TokenStream;
use quote::quote;

pub trait ResultErrAsCompileErrorExt<E: ToString> {
    fn err_as_compile_error(self) -> TokenStream;
}

impl<E: ToString> ResultErrAsCompileErrorExt<E> for Result<TokenStream, E> {
    fn err_as_compile_error(self) -> TokenStream {
        self.unwrap_or_else(|err| {
            let err = err.to_string();
            quote! {
                compile_error!(#err);
            }
        })
    }
}
