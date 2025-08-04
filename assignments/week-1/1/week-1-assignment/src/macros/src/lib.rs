extern crate proc_macro;
use case::CaseExt;
use proc_macro::TokenStream;
use quote::quote;
use serde_json::{Value, json};
use syn::{parse_macro_input, DeriveInput};
#[proc_macro_attribute]
pub fn todo_app(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let expanded = quote! {
        #[derive(serde::Serialize)]
        #input
        impl serde::ser::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                let value = json!(self);
                if let Value::Object(mut map) = value {
                    let new_map = map.into_iter().map(|(k, v)| {
                        let new_key = format!("{}{}", "TodoApp", k.to_case(Case::Pascal));
                        (new_key, v)
                    }).collect();
                    serde::ser::Serialize::serialize(&Value::Object(new_map), serializer)
                } else {
                    serde::ser::Serialize::serialize(&value, serializer)
                }
            }
        }
    };
    TokenStream::from(expanded)
} 