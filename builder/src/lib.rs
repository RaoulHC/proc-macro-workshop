extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, parse_macro_input, DeriveInput, Ident, Fields};
use proc_macro2::Span;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let id = ast.ident;

    let fields = match ast.data {
        Data::Struct(data) => data.fields,
        _ => panic!("Can't derive builder that isn't a struct"),
    };
    let named_fields = match fields {
        Fields::Named(x) => x.named,
        _ => panic!("can't derive builder for struct without named fields"),
    };
    let empty_fields = named_fields.iter().map(|x| {
        let name = &x.ident;
        quote!{#name: None}
    });

    // name of the builder structure
    let builder_name = Ident::new(&format!("{}Builder", id), Span::call_site());

    let builder = quote!{
        impl #id {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#empty_fields),*
                }
            }
        }
    };
    let builder_fields = named_fields.iter().map(|x| {
        let name = &x.ident;
        let ty = &x.ty;
        quote!{
            #name: Option<#ty>
        }
    });

    let builder_struct = quote! {
        pub struct #builder_name {
            #(#builder_fields),*
        }
    };

    let call_setters = named_fields.iter().map(|x| {
        let name = &x.ident;
        let ty = &x.ty;
        quote!{
            fn #name(&mut self, #name: #ty) -> &mut Self {
                self.#name = Some(#name);
                self
            }
        }
    });

    let build_checks = named_fields.iter().map(|x| {
        let name = &x.ident;
        let ty = &x.ty;
        quote! {
            let #name: #ty = match &self.#name {
                Some(x) => Ok(x.to_owned()),
                None => Err(Box::<dyn Error>::from(
                    "Missing")),
            }?;
        }
    });

    let build_construct = named_fields.iter().map(|x| {
        let name = &x.ident;
        quote!{ #name }
    });

    let build_method = quote! {
        pub fn build(&mut self) -> Result<#id, Box<dyn Error>> {
            #(#build_checks)*
            let built = #id {
                #(#build_construct),*
            };
            Ok(built)
        }
    };

    let builder_impl = quote! {
        impl #builder_name {
            #(#call_setters)*
            #build_method
        }
    };

    let total = quote!{
        #builder_struct
        #builder
        #builder_impl
    };

    TokenStream::from(total)
}
