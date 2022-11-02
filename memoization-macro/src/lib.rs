
use std::ops::Deref;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, ItemFn, FnArg};
//use proc_macro2::{Span, Ident};

#[proc_macro_attribute]
pub fn memoize(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast : ItemFn = syn::parse(input).unwrap();

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = ast;

    let mut input_idents = Vec::new();
    let mut input_types = Vec::new();
    for item in &sig.inputs {
        if let FnArg::Typed( pat_type ) = item {

            input_types.push(pat_type.ty.deref().clone());
            if let syn::Pat::Ident(pat_ident) = &pat_type.pat.deref() {
                input_idents.push(pat_ident.ident.clone());
            } else {
                panic!("expected a PatIdent");
            }
        }
    }
    let input_tuple_type = quote!((#(#input_types,)*));
    let input_tuple = quote!((#(#input_idents, )*));

    let output_type = match &sig.output {
        syn::ReturnType::Type(_, r_type) => {
            r_type.deref().clone()
        },
        syn::ReturnType::Default => {
            panic!("Didn't expect default");
        }
    };

    let out = quote!(
        #(#attrs)*
        #vis #sig {

            use std::collections::HashMap;
            use once_cell::sync::Lazy;

            //println!("input is {:?}", #input_tuple);

            static mut LOOKUP_TABLE : Lazy<HashMap<#input_tuple_type, #output_type>> = Lazy::new(|| { HashMap::new() });

            let result = match unsafe { LOOKUP_TABLE.get(&#input_tuple) } {
                Some(value) => *value,
                None => {
                    //println!("was not cached");
                    let new_result = #block;
                    unsafe { LOOKUP_TABLE.insert(#input_tuple, new_result.clone()) };
                    new_result
                }
            };

            result
        }
    );

    return out.into();
}