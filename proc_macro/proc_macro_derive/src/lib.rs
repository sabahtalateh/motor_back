extern crate proc_macro;

use darling::FromMeta;
use proc_macro::{TokenStream, TokenTree};
use proc_macro_error::abort_call_site;
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn;
use syn::{parse_macro_input, AttributeArgs};

#[proc_macro_derive(Dependency)]
pub fn dep_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;

    let gen = quote! {
        impl Dependency for #name {
            fn service_name() -> String {
                return stringify!(#name).to_string();
            }
        }
    };
    gen.into()
}

fn applied_to_struct(input: TokenStream) -> bool {
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

fn parse_struct_name(input: TokenStream) -> String {
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

#[derive(Debug, FromMeta)]
struct ServiceArgs {
    name: String,
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn service(args: TokenStream, input: TokenStream) -> TokenStream {
    if !applied_to_struct(input.clone()) {
        abort_call_site!("`dep` macro can be used only on `struct`");
    }

    let struct_name = parse_struct_name(input.clone());
    let attr_args = parse_macro_input!(args as AttributeArgs);
    match ServiceArgs::from_list(&attr_args) {
        Ok(dep_args) => {
            let dep_name: String = dep_args.name;
            let dep_trait = format!(
                r#"impl Service2 for {struct_name} {{
                     fn name() -> String {{
                         return "{dep_name}".to_string();
                     }}
                 }}"#,
                struct_name = struct_name,
                dep_name = dep_name
            );
            (input.to_string() + &dep_trait.to_string())
                .parse()
                .unwrap()
        }
        Err(_) => {
            abort_call_site!("`dep` macro should have `name` attr");
        }
    }
}
