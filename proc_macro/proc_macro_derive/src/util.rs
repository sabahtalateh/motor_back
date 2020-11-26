use proc_macro::{TokenStream, TokenTree};
use proc_macro2::Span;
use proc_macro_error::abort_call_site;
use syn;

pub(crate) fn applied_to_struct(input: TokenStream) -> bool {
    let mut is_struct = false;

    for t in input.into_iter() {
        match t {
            TokenTree::Ident(i) => {
                if i.to_string() == "struct" {
                    is_struct = true;
                    break;
                }
            }
            _ => (),
        }
    }

    is_struct
}

pub(crate) fn parse_struct_name(input: TokenStream) -> String {
    let it = input.into_iter();
    let mut prev_struct_keyword = false;
    for t in it {
        if prev_struct_keyword {
            if let TokenTree::Ident(struct_name) = t {
                return struct_name.to_string();
            } else {
                abort_call_site!("struct name should follow `struct` keyword");
            }
        }

        match t {
            TokenTree::Ident(i) => {
                if i.to_string() == "struct" {
                    prev_struct_keyword = true;
                }
            }
            _ => (),
        }
    }

    abort_call_site!("struct name should follow `struct` keyword");
}

pub(crate) fn extract_helpers(
    ast: &syn::DeriveInput,
    helper_name: &str,
    message_if_no_struct: &'static str,
) -> Vec<(proc_macro2::Ident, syn::Type)> {
    let mut helpers = vec![];

    match ast.data {
        syn::Data::Struct(ref data_struct) => match data_struct.fields {
            syn::Fields::Named(ref fields_named) => {
                for field in fields_named.named.iter() {
                    for attr in field.attrs.iter() {
                        match attr.parse_meta().unwrap() {
                            syn::Meta::Path(ref path)
                                if path.get_ident().unwrap().to_string() == helper_name =>
                            {
                                // Save helper
                                let item = field.clone();
                                helpers.push((item.ident.unwrap(), item.ty))
                            }
                            _ => (),
                        }
                    }
                }
            }
            _ => (),
        },
        _ => panic!(message_if_no_struct),
    }

    helpers
}

pub(crate) fn str_to_ident(str: &str) -> proc_macro2::TokenTree {
    proc_macro2::TokenTree::Ident(proc_macro2::Ident::new(str, Span::call_site()))
}
