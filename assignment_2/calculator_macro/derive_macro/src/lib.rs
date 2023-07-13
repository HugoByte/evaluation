use std::collections::HashMap;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
mod add;
use add::*;
mod sub;
use sub::*;
mod mul;
use mul::*;
mod modul;
use modul::*;


#[proc_macro_derive(Calculator, attributes(Operation))]
pub fn calculator(input: TokenStream) -> TokenStream {
let ast = parse_macro_input!(input as DeriveInput);
impl_calculator(ast)
}

fn impl_calculator(ast: DeriveInput) -> TokenStream {
    let calc = ast.ident.clone();
    let attribute_args = ast.attrs;

    let mut property_map: HashMap<String, String> = HashMap::new();

    for attribute in attribute_args.into_iter() {
        let (path, value) = match attribute.parse_meta().unwrap() {
            syn::Meta::NameValue(syn::MetaNameValue {
                path,
                lit: syn::Lit::Str(s),
                ..
            }) => (path, s.value()),
            _ => (syn::Path::into(attribute.path), "".to_string()),
        };

        for segment in path.segments {
            property_map.insert(segment.ident.to_string(), value.clone());
        }
    }

    let operation = property_map["Operation"].clone();

    let mut methods = quote! {};

    if operation == "addition" {
        methods = impl_add(calc).into();
    } else if  operation == "subtraction" {
        methods = impl_sub(calc).into();
    } else if operation == "multiplication" {
        methods = impl_mul(calc).into();
    } else if operation == "modulus" {
        methods = impl_modul(calc).into();
    } else {  
    }
        
    // methods.into()
    
    let ast= quote! {
        #methods
    };
    ast.into()


}
