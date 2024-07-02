extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Arithmetic, attributes(add, sub))]
pub fn addable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_data = if let syn::Data::Enum(ref data) = input.data {
        data
    } else {
        return syn::Error::new_spanned(input, "Arithmetic only support Enum type")
            .to_compile_error()
            .into();
    };

    let enum_name = &input.ident;

    // collect #[add] variant
    let add_arms = enum_data.variants.iter().filter_map(|variant| {
        let variant_name = &variant.ident;

        // find #[add] attribute
        let has_add_attr = variant.attrs.iter().any(|attr| {
            attr.path().is_ident("add")
        });

        if has_add_attr {
            Some(quote! {
                (#enum_name::#variant_name(a), #enum_name::#variant_name(b)) => #enum_name::#variant_name(a + b),
            })
        } else {
            None
        }
    }).collect::<Vec<_>>();

    let sub_arms = enum_data.variants.iter().filter_map(|variant| {
        let variant_name = &variant.ident;

        // find #[sub] attribute
        let has_add_attr = variant.attrs.iter().any(|attr| {
            attr.path().is_ident("sub")
        });

        if has_add_attr {
            Some(quote! {
                (#enum_name::#variant_name(a), #enum_name::#variant_name(b)) => #enum_name::#variant_name(a - b),
            })
        } else {
            None
        }
    }).collect::<Vec<_>>();

    let expanded = quote! {
        impl std::ops::Add for #enum_name {
            type Output = #enum_name;

            fn add(self, other: Self) -> Self::Output {
                match (self, other) {
                    #(#add_arms)*
                    _ => panic!("Add not supported"),
                }
            }
        }

        impl std::ops::Add for &#enum_name {
            type Output = #enum_name;

            fn add(self, other: Self) -> Self::Output {
                match (self, other) {
                    #(#add_arms)*
                    _ => panic!("Add not supported"),
                }
            }
        }

        impl std::ops::Sub for #enum_name {
            type Output = #enum_name;

            fn sub(self, other: Self) -> Self::Output {
                match (self, other) {
                    #(#sub_arms)*
                    _ => panic!("Sub not supported"),
                }
            }
        }

        impl std::ops::Sub for &#enum_name {
            type Output = #enum_name;

            fn sub(self, other: Self) -> Self::Output {
                match (self, other) {
                    #(#sub_arms)*
                    _ => panic!("Sub not supported"),
                }
            }
        }
    };

    TokenStream::from(expanded)
}
