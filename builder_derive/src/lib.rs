extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, GenericArgument, Ident, PathArguments, Type,
};

#[proc_macro_derive(Builder)]
pub fn builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let option = "Option".to_owned();
    let name = input.ident;
    let data = match input.data {
        Data::Struct(data) => data,
        _ => unimplemented!(),
    };
    let fields = match data.fields {
        Fields::Named(fields) => fields,
        _ => unimplemented!(),
    };
    let results = fields
        .named
        .into_iter()
        .map(|f| {
            let ident = f.ident;
            let ty = f.ty.clone();
            let path = match f.ty {
                Type::Path(path) => path,
                _ => unimplemented!(),
            };
            let mut segs = path.path.segments.into_iter();
            let first_seg = segs.next().unwrap();
            let is_option = first_seg.ident.to_string() == option;
            let option_ty = if is_option {
                let args = match first_seg.arguments {
                    PathArguments::AngleBracketed(args) => args,
                    _ => unimplemented!(),
                };
                match args.args.into_iter().next().unwrap() {
                    GenericArgument::Type(ty) => Some(ty),
                    _ => unimplemented!(),
                }
            } else {
                None
            };
            (ident, ty, option_ty)
        })
        .map(|(n, t, o)| {
            if let Some(ty) = o {
                quote_spanned! { name.span() =>
                    pub fn #n<T: Into<#ty>>(mut self, val: T) -> Self {
                        self.node.#n = Some(val.into());
                        self
                    }
                }
            } else {
                quote_spanned! { name.span() =>
                    pub fn #n<T: Into<#t>>(mut self, val: T) -> Self {
                        self.node.#n = val.into();
                        self
                    }
                }
            }
        });
    let builder_name = Ident::new(&format!("{}Builder", name), name.span());
    let expanded = quote! {
        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name::new()
            }
        }

        pub struct #builder_name {
            node: #name
        }

        impl #builder_name {
            pub fn new() -> Self {
                #builder_name {
                    node: #name::default()
                }
            }

            #(#results)*

            pub fn build(self) -> #name {
                self.node
            }
        }
    };
    TokenStream::from(expanded)
}
