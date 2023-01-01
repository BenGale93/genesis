use proc_macro::TokenStream;

#[proc_macro_derive(BehaviourTracker)]
pub fn derive_behaviour_tracker(input: TokenStream) -> TokenStream {
    genesis_macros::derive_behaviour_tracker(input.into()).into()
}

#[proc_macro_derive(AttributeDisplay)]
pub fn derive_attribute_display(input: TokenStream) -> TokenStream {
    genesis_macros::derive_attribute_display(input.into()).into()
}
