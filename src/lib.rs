extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Arithmetic, attributes(add, sub, mul, div))]
pub fn arithmetic_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_data = if let syn::Data::Enum(ref data) = input.data {
        data
    } else {
        return syn::Error::new_spanned(input, "Arithmetic only support Enum type")
            .to_compile_error()
            .into();
    };

    let enum_name = &input.ident;

    let mut add_arms = Vec::new();
    let mut sub_arms = Vec::new();
    let mut mul_arms = Vec::new();
    let mut div_arms = Vec::new();

    // collect #[add] variant
    enum_data.variants.iter().for_each(|variant| {
        let variant_name = &variant.ident;

        // find #[add] attribute
        let has_add_attr = variant.attrs.iter().any(|attr| {
            attr.path().is_ident("add")
        });

        // find #[sub] attribute
        let has_sub_attr = variant.attrs.iter().any(|attr| {
            attr.path().is_ident("sub")
        });

        // find #[mul] attribute
        let has_mul_attr = variant.attrs.iter().any(|attr| {
            attr.path().is_ident("mul")
        });

        // find #[div] attribute
        let has_div_attr = variant.attrs.iter().any(|attr| {
            attr.path().is_ident("div")
        });

        if has_add_attr {
            add_arms.push(quote! {
                (#enum_name::#variant_name(a), #enum_name::#variant_name(b)) => #enum_name::#variant_name(a + b),
            })
        }
        if has_sub_attr {
            sub_arms.push(quote! {
                (#enum_name::#variant_name(a), #enum_name::#variant_name(b)) => #enum_name::#variant_name(a - b),
            })
        }
        if has_mul_attr {
            mul_arms.push(quote! {
                (#enum_name::#variant_name(a), #enum_name::#variant_name(b)) => #enum_name::#variant_name(a * b),
            })
        }
        if has_div_attr {
            div_arms.push(quote! {
                (#enum_name::#variant_name(a), #enum_name::#variant_name(b)) => #enum_name::#variant_name(a / b),
            })
        }
    });

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

        impl std::ops::Mul for #enum_name {
            type Output = #enum_name;

            fn mul(self, other: Self) -> Self::Output {
                match (self, other) {
                    #(#mul_arms)*
                    _ => panic!("Mul not supported"),
                }
            }
        }

        impl std::ops::Mul for &#enum_name {
            type Output = #enum_name;

            fn mul(self, other: Self) -> Self::Output {
                match (self, other) {
                    #(#mul_arms)*
                    _ => panic!("Mul not supported"),
                }
            }
        }

        impl std::ops::Div for #enum_name {
            type Output = #enum_name;

            fn div(self, other: Self) -> Self::Output {
                match (self, other) {
                    #(#div_arms)*
                    _ => panic!("Div not supported"),
                }
            }
        }

        impl std::ops::Div for &#enum_name {
            type Output = #enum_name;

            fn div(self, other: Self) -> Self::Output {
                match (self, other) {
                    #(#div_arms)*
                    _ => panic!("Div not supported"),
                }
            }
        }
    };

    TokenStream::from(expanded)
}
