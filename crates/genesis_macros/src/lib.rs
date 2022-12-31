use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse2, ItemStruct};

pub fn derive_behaviour_tracker(input: TokenStream2) -> TokenStream2 {
    let ast = parse2::<ItemStruct>(input).unwrap();

    let struct_name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();

    quote! {
        impl #impl_generics genesis_traits::BehaviourTracker for #struct_name #type_generics #where_clause {
            fn new() -> Self where Self: Sized {
                Self(0.0)
            }

            fn add_time(&mut self, time: f32, cost: f32) {
                self.0 += time * cost
            }

            fn uint_portion(&mut self) -> usize {
                let floor = self.0.floor();
                self.0 -= floor;
                floor as usize
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream as TokenStream2;

    use super::*;

    fn assert_tokens_eq(expected: &TokenStream2, actual: &TokenStream2) {
        let expected = expected.to_string();
        let actual = actual.to_string();

        if expected != actual {
            println!("expected: {}", &expected);
            println!("actual  : {}", &actual);
            panic!("expected != actual");
        }
    }
    #[test]
    fn derive_behaviour_tracker_trait() {
        let before = quote! {
            pub struct EatingSum(f32);
        };

        let expected = quote! {
            impl genesis_traits::BehaviourTracker for EatingSum {
                fn new() -> Self where Self: Sized {
                    Self(0.0)
                }

                fn add_time(&mut self, time: f32, cost: f32) {
                    self.0 += time * cost
                }

                fn uint_portion(&mut self) -> usize {
                    let floor = self.0.floor();
                    self.0 -= floor;
                    floor as usize
                }
            }
        };

        let after = derive_behaviour_tracker(before);
        assert_tokens_eq(&expected, &after);
    }
}
