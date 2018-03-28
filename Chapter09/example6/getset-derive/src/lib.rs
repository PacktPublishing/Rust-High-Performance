extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Field, Fields};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Ident;
use quote::Tokens;

fn get_fields(ast: &DeriveInput) -> &Punctuated<Field, Comma> {
    match ast.data {
        Data::Struct(ref structure) => {
            if let Fields::Named(ref fields) = structure.fields {
                &fields.named
            } else {
                panic!(
                    "you cannot implement setters or getters \
                     for unit or tuple structs"
                );
            }
        }
        Data::Union(ref _union) => {
            unimplemented!(
                "sorry, setters and getters are not \
                 implemented for unions yet"
            );
        }
        Data::Enum(ref _enum) => {
            panic!(
                "you cannot derive setters or getters for \
                 enumerations"
            );
        }
    }
}

#[proc_macro_derive(Getters)]
pub fn derive_getters(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = syn::parse(input).unwrap();

    // Build the output
    let expanded = impl_methods(&input, generate_getter);

    // Hand the output tokens back to the compiler
    expanded.into()
}

#[proc_macro_derive(Setters)]
pub fn derive_setters(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = syn::parse(input).unwrap();

    // Build the output
    let expanded = impl_methods(&input, generate_setter);

    // Hand the output tokens back to the compiler
    expanded.into()
}

fn impl_methods<F>(ast: &DeriveInput, strategy: F) -> Tokens
where
    F: FnMut(&Field) -> Tokens,
{
    let methods: Vec<_> = get_fields(ast).iter().map(strategy).collect();
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #(#methods)*
        }
    }
}

fn generate_setter(field: &Field) -> Tokens {
    let name = field.ident.expect("named fields must have a name");
    let fn_name = Ident::from(format!("set_{}", name));
    let ty = &field.ty;

    quote! {
        fn #fn_name<T>(&mut self, value: T) where T: Into<#ty> {
            self.#name = value.into();
        }
    }
}

fn generate_getter(field: &syn::Field) -> Tokens {
    let name = field.ident.expect("named fields must have a name");
    let ty = &field.ty;

    quote! {
        fn #name(&self) -> &#ty {
            &self.#name
        }
    }
}
