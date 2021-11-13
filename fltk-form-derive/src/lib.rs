extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

mod implementation;
use crate::implementation::*;

use proc_macro::TokenStream;

#[proc_macro_derive(FltkForm)]
pub fn deser_widget_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_widget_deser_trait(&ast).unwrap()
}
