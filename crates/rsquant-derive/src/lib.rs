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
    let DeriveInput {
        ident, generics, ..
    } = parse_macro_input!(input);
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let ident_str = ident.to_string();
    let output = quote! {
        impl #impl_generics Name for #ident #type_generics #where_clause {
            fn get_name(&self) -> String {
                #ident_str.into()
            }
        }
    };
    output.into()
}
