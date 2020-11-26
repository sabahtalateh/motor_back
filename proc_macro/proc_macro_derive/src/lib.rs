mod util;

extern crate proc_macro;

use crate::util::{applied_to_struct, extract_helpers, parse_struct_name, str_to_ident};
use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::{Literal, Span, TokenTree};
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

    let helpers = extract_helpers(
        &ast,
        "logger",
        "`#[derive(HasLogger)]` must be applied to struct",
    );

    if helpers.len() != 1 {
        panic!("`#[derive(HasLogger)]` must have exactly 1 `#[logger]` helper");
    }

    let (logger_field, _) = helpers.get(0).unwrap();

    // let bb: Vec<String> = helpers.iter().map(|(x, _y)| format!("{}", x)).collect();
    // let _bbb: String = bb.join(",");

    let tt = quote! {
        impl HasLogger for #name {
            fn logger(&self) -> &Logger {
                self.#logger_field.logger()
            }
        }
    };
    tt.into()
}

#[derive(Debug)]
struct RepoArgs {
    collection: String,
    select: String,
    insert: String,
}

impl From<TokenStream> for RepoArgs {
    fn from(ts: TokenStream) -> Self {
        let panic_msg = "wrong repo args. should be #[repo(select = X, insert = Y, collection = Z)]";

        let mut select = "".to_string();
        let mut insert = "".to_string();
        let mut collection = "".to_string();

        let mut step = 0;

        for t in ts {
            // ну это ваше то можно нормально переписать но мне лень
            // можно переписать типо номер шага и братку-обратку передавать (колбек ёпта)
            // тогда вмест такого полотна будет просто семь строчек
            if step == 0 {
                match t.clone() {
                    proc_macro::TokenTree::Ident(i) => {
                        if "select" != &i.to_string() {
                            panic!(panic_msg)
                        }
                    }
                    _ => panic!(panic_msg),
                }
            }

            if step == 1 {
                match t.clone() {
                    proc_macro::TokenTree::Punct(p) => {
                        if '=' != p.as_char() {
                            panic!(panic_msg)
                        }
                    }
                    _ => panic!(panic_msg),
                }
            }

            if step == 2 {
                match t.clone() {
                    proc_macro::TokenTree::Ident(i) => {
                        select = i.to_string();
                    }
                    _ => panic!(panic_msg),
                }
            }

            if step == 3 {
                match t.clone() {
                    proc_macro::TokenTree::Punct(p) => {
                        if ',' != p.as_char() {
                            panic!(panic_msg)
                        }
                    }
                    _ => panic!(panic_msg),
                }
            }

            if step == 4 {
                match t.clone() {
                    proc_macro::TokenTree::Ident(i) => {
                        if "insert" != &i.to_string() {
                            panic!(panic_msg)
                        }
                    }
                    _ => panic!(panic_msg),
                }
            }

            if step == 5 {
                match t.clone() {
                    proc_macro::TokenTree::Punct(p) => {
                        if '=' != p.as_char() {
                            panic!(panic_msg)
                        }
                    }
                    _ => panic!(panic_msg),
                }
            }

            if step == 6 {
                match t.clone() {
                    proc_macro::TokenTree::Ident(i) => {
                        insert = i.to_string();
                    }
                    _ => panic!(panic_msg),
                }
            }

            if step == 7 {
                match t.clone() {
                    proc_macro::TokenTree::Punct(p) => {
                        if ',' != p.as_char() {
                            panic!(panic_msg)
                        }
                    }
                    _ => panic!(panic_msg),
                }
            }

            if step == 8 {
                match t.clone() {
                    proc_macro::TokenTree::Ident(i) => {
                        if "collection" != &i.to_string() {
                            panic!(panic_msg)
                        }
                    }
                    _ => panic!(panic_msg),
                }
            }

            if step == 9 {
                match t.clone() {
                    proc_macro::TokenTree::Punct(p) => {
                        if '=' != p.as_char() {
                            panic!(panic_msg)
                        }
                    }
                    _ => panic!(panic_msg),
                }
            }

            if step == 10 {
                match t.clone() {
                    proc_macro::TokenTree::Ident(i) => {
                        collection = i.to_string();
                    }
                    _ => panic!(panic_msg),
                }
            }

            step += 1
        }

        RepoArgs {
            select,
            insert,
            collection,
        }
    }
}

// #[proc_macro_error]
// #[proc_macro_attribute]
// pub fn repo(args: TokenStream, input: TokenStream) -> TokenStream {
//     let repo_args: RepoArgs = args.into();
//     let select = str_to_ident(&repo_args.select);
//     let insert = str_to_ident(&repo_args.insert);
//
//     let struct_name = str_to_ident(&parse_struct_name(input.clone()));
//
//     let tt = quote! {
//         #[async_trait]
//         impl Repo for #struct_name {
//             async fn find(&self) {
//
//             }
//         }
//     };
//
//     (input.to_string() + &tt.to_string()).parse().unwrap()
// }
