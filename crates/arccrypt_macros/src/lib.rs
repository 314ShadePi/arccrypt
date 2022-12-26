use proc_macro::*;
use quote::quote;
use syn;

#[proc_macro_derive(Payload)]
pub fn payload_derive(item: TokenStream) -> TokenStream {
    let ast = syn::parse(item).unwrap();
    impl_payload(&ast)
}

fn impl_payload(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        #[typetag::serde]
        impl Payload for #name {}
    };
    gen.into()
}
