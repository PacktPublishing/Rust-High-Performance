extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_derive(TypeName)]
pub fn type_name(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = syn::parse(input).unwrap();

    // Build the output
    let expanded = impl_type_name(&input);

    // Hand the output tokens back to the compiler
    expanded.into()
}

fn impl_type_name(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl TypeName for #name {
            fn type_name() -> &'static str {
                stringify!(#name)
            }
        }
    }
}
