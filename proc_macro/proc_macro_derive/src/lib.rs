mod util;

extern crate proc_macro;

use crate::util::{applied_to_struct, extract_helpers, parse_struct_name};
use darling::FromMeta;
use proc_macro::{TokenStream, TokenTree};
use proc_macro_error::{abort_call_site, proc_macro_error};
use quote::quote;
use syn::{parse_macro_input, AttributeArgs};

#[proc_macro_derive(Dependency)]
pub fn dep_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;

    let tt = quote! {
        impl Dependency for #name {
            fn service_name() -> String {
                return stringify!(#name).to_string();
            }
        }
    };
    tt.into()
}

#[derive(Debug, FromMeta)]
struct Service2Args {
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
    match Service2Args::from_list(&attr_args) {
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

#[proc_macro_derive(HasLogger, attributes(logger))]
pub fn has_logger(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;

    let mut helpers = extract_helpers(
        &ast,
        "logger",
        "`#[derive(HasLogger)]` must be applied to struct",
    );

    if helpers.len() != 1 {
        panic!("`#[derive(HasLogger)]` must have exactly 1 `#[logger]` helper");
    }

    let (logger_field, _) = helpers.get(0).unwrap();

    let bb: Vec<String> = helpers.iter().map(|(x, y)| format!("{}", x)).collect();
    let bbb: String = bb.join(",");

    let tt = quote! {
        impl HasLogger for #name {
            fn logger(&self) -> &Logger {
                self.#logger_field.logger()
                // println!("{}", #bbb);
                // return "123".to_string();
            }
        }
    };
    tt.into()
}
