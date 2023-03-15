use std::ops::Add;

use darling::{FromDeriveInput, ToTokens};
use proc_macro::{self, TokenStream};
use quote::{quote, __private::ext::RepToTokensExt};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Symbol)]
pub fn symbol_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let DeriveInput { ident, .. } = input;
    let output = quote! {
        impl Symbol for #ident {
            fn new(symbol_data: SymbolData) -> Self{
                Self(symbol_data)
            }
            fn get_data(&self) -> &geometry::symbol::SymbolData {
                &self.0
            }
            fn get_data_mut(&mut self) -> &mut geometry::symbol::SymbolData {
                &mut self.0
            }
        }
    };
    output.into()
}

#[proc_macro_derive(RuleEvaluator, attributes(rules))]
pub fn my_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input into a `DeriveInput` struct
    let derive_input = input.clone();
    let derive_input = parse_macro_input!(derive_input as DeriveInput);

    // Extract the name of the struct or enum that the macro is being derived for
    let ident = &derive_input.ident;

    // Extract any types passed as attributes
    // let mut my_types = Vec::new();
    let mut rules_vec = Vec::with_capacity(5);
    for attr in derive_input.attrs.iter() {
        if let Ok(meta) = attr.parse_meta() {
            if let syn::Meta::List(mnv) = meta {
                if mnv.path.is_ident("rules") {
                    for nested_meta in mnv.nested.iter(){
                        if let syn::NestedMeta::Meta(meta) = nested_meta{
                            if let syn::Meta::Path(path) = meta {
                                // my_strings.push(lit.value());
                                let name = path.to_token_stream();
                                rules_vec.push(name);
                            }
                        }
                    }
                }
                // if let Ok(ty) = syn::parse::<syn::Type>(mnv.lit.clone()) {
                //     my_types.push(ty);
                // }
            }
            
        }
    }
    let count = rules_vec.len();
    let count_iter = (0..count);
    let count_iter2 = (0..count);

    // Generate the code for the derived implementation
    let expanded = quote! {
        impl geometry::rule::RuleEvaluator for #ident {
            fn evaluate_rules(&mut self) -> Option<Vec<Box<dyn geometry::rule::RuleEvaluator>>> {
                // #(println!("{:?}", <#my_types as std::fmt::Debug>::default());)*
                // println!("{}", #name)
                let mut probabilities : [f32; #count] = Default::default();
                #(probabilities[#count_iter] = <#ident as geometry::rule::Rule<#rules_vec>>::probability();)*

                let distribution = WeightedIndex::new(&probabilities).unwrap();
                let mut rng = thread_rng();

                match distribution.sample(&mut rng) {
                    #(#count_iter2 => {
                        geometry::rule::Rule::<#rules_vec>::evaluate(self)
                    }),*
                    _ => {None}
                }
            }
            fn get_symbol_data(&self) -> &geometry::symbol::SymbolData {
                self.get_data()
            }
        }
    };

    // Return the generated code as a `TokenStream`
    proc_macro::TokenStream::from(expanded)
    // TokenStream::new()
}

// #[proc_macro_derive(MyMacro, attributes(my_type))]
// pub fn my_macro_derive(input: TokenStream) -> TokenStream {
//     // Parse the input into a `DeriveInput` struct
//     let input = parse_macro_input!(input as DeriveInput);
//
//     // Extract any types passed as attributes
//     let mut my_types = Vec::new();
//     for attr in input.attrs.iter() {
//         if let Ok(meta) = attr.parse_meta() {
//             if let syn::Meta::NameValue(mnv) = meta {
//                 if mnv.path.is_ident("my_type") {
//                     if let Ok(ty) = syn::parse::<syn::Type>(mnv.lit.clone()) {
//                         my_types.push(ty);
//                     }
//                 }
//             }
//         }
//     }
//
//     // Generate the code for the derived implementation
//     let expanded = quote! {
//         impl MyTrait for #input {
//             fn my_method(&self) {
//                 #(println!("{:?}", <#my_types as std::fmt::Debug>::default());)*
//             }
//         }
//     };
//
//     // Return the generated code as a `TokenStream`
//     TokenStream::from(expanded)
// }

