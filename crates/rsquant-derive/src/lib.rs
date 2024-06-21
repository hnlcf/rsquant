use proc_macro::{
    self,
    TokenStream,
};
use quote::quote;
use syn::{
    parse_macro_input,
    DeriveInput,
};

#[proc_macro_derive(Name)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let ident_str = ident.to_string();
    let output = quote! {
        impl Name for #ident {
            fn get_name(&self) -> String {
                #ident_str.into()
            }
        }
    };
    output.into()
}
