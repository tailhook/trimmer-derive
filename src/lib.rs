#![crate_type = "proc-macro"]

extern crate syn;
extern crate proc_macro;
#[macro_use] extern crate quote;

mod new_types;
mod structs;

use proc_macro::TokenStream;


#[proc_macro_derive(Variable, attributes(variable))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input: String = input.to_string();
    let ref ast = syn::parse_macro_input(&input).expect("Couldn't parse item");
    let str_data = match ast.body {
        syn::Body::Enum(ref variants) => panic!("Only structs are supported"),
        syn::Body::Struct(ref str_data) => str_data,
    };
    let result = match *str_data {
        syn::VariantData::Struct(ref fields) => {
            structs::derive(ast, fields)
        },
        syn::VariantData::Unit => {
            panic!("Can't derive variable for unit struct");
        },
        syn::VariantData::Tuple(ref fields) => {
            if fields.len() == 1 {
                new_types::derive(ast, &fields[0])
            } else {
                unimplemented!();
            }
        },
    };
    result.to_string().parse().expect("Couldn't parse string to tokens")
}
