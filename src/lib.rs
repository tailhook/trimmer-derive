//! Derive implementation for trimmer template engine
//!
//! [Trimmer](https://crates.io/crates/trimmer/) |
//! [Docs](https://docs.rs/trimmer_derive/) |
//! [Github](https://github.com/tailhook/trimmer-derive) |
//! [Crate](https://crates.io/crates/trimmer_derive)
//!
//!
//! This crate allows to derive `trimmer::Variable` trait.
//!
//! Currently it supports two kinds of structures, a tuple structure with
//! a single field (i.e. a newtype pattern):
//!
//! ```rust
//! extern crate trimmer;
//! #[macro_use] extern crate trimmer_derive;
//!
//! #[derive(Debug, Variable)]
//! struct Variable(String);
//!
//! # fn main() {}
//! ```
//!
//! In this case, all methods of the variable implementation will be forwarded
//! to the enclosed type (and it must implement `Variable`)
//!
//! And for regular structures with named patterns:
//!
//! ```rust
//! extern crate trimmer;
//! #[macro_use] extern crate trimmer_derive;
//!
//! #[derive(Debug, Variable)]
//! struct Point {
//!     x: u32,
//!     y: u32,
//! }
//!
//! # fn main() {}
//! ```
//!
//! In this case, `Point` will implement `attr` method resolving `x` and `y`.
//! All fields must implement `Variable` trait themselves.
//!
//!
#![crate_type="proc-macro"]
#![recursion_limit="256"]
#![warn(missing_debug_implementations)]

extern crate syn;
extern crate proc_macro;
#[macro_use] extern crate quote;

mod new_types;
mod structs;

use proc_macro::TokenStream;


/// A derivation function for proc macro
#[proc_macro_derive(Variable, attributes(variable))]
pub fn derive_variable(input: TokenStream) -> TokenStream {
    let input: String = input.to_string();
    let ref ast = syn::parse_macro_input(&input).expect("Couldn't parse item");
    let str_data = match ast.body {
        syn::Body::Enum(..) => panic!("Only structs are supported"),
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
