use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::*;
use syn::*;

pub fn impl_widget_deser_trait(ast: &DeriveInput) -> Result<TokenStream> {
    Ok({
        let name = &ast.ident;
        let name_str = name.to_string();
        let data = &ast.data;
        let gen;
        match data {
            Data::Enum(variants) => {
                let data_expanded_members = variants.variants.iter().map(|field| {
                    let field_name = &field.ident;
                    let span = field_name.span();
                    let field_name_stringified = LitStr::new(&field_name.to_string(), span);
                    quote_spanned! {
                        span => {
                            #field_name_stringified
                        }
                    }
                });
                let data_expanded_members_c = data_expanded_members.clone();
                gen = quote! {
                    impl FltkForm for #name {
                        fn generate(&self) -> Box<dyn WidgetExt> {
                            let mut choice = menu::Choice::default();
                            let mems = vec![#(#data_expanded_members_c),*];
                            for mem in mems {
                                choice.add_choice(mem);
                            }
                            choice.set_value(*self as i32);
                            Box::new(choice)
                        }
                        fn view(&self) -> Box<dyn WidgetExt> {
                            let mut choice = output::Output::default();
                            choice.set_value(&format!("{:?}", *self));
                            Box::new(choice)
                        }
                    }
                };
            }

            Data::Struct(DataStruct {
                fields: Fields::Named(it),
                ..
            }) => {
                let data_expanded_members_gen = it.named.iter().map(|field| {
                    let field_name = field.ident.as_ref().expect("Unreachable");
                    let span = field_name.span();
                    let field_name_stringified = LitStr::new(&field_name.to_string(), span);
                    quote_spanned! {
                        span => {
                            let mut i = self.#field_name.generate();
                            i.set_align(fltk::enums::Align::Left);
                            i.set_label(#field_name_stringified);
                        }
                    }
                });
                let data_expanded_members_view = it.named.iter().map(|field| {
                    let field_name = field.ident.as_ref().expect("Unreachable");
                    let span = field_name.span();
                    let field_name_stringified = LitStr::new(&field_name.to_string(), span);
                    quote_spanned! {
                        span => {
                            let mut i = self.#field_name.view();
                            i.set_align(fltk::enums::Align::Left);
                            i.set_label(#field_name_stringified);
                        }
                    }
                });
                gen = quote! {
                    impl FltkForm for #name {
                        fn generate(&self) -> Box<dyn WidgetExt> {
                            let mut p = group::Pack::default()
                                .with_label(&format!("{}", #name_str))
                                .with_align(fltk::enums::Align::Left | fltk::enums::Align::Top);
                            p.set_spacing(5);
                            let mems = vec![#(#data_expanded_members_gen),*];
                            p.end();
                            let parent = p.parent().unwrap();
                            p.resize(
                                parent.x() + (parent.width()/2), parent.y() + parent.h() / 9, parent.width() / 3, (mems.len() * 30 + 5 * mems.len()) as i32
                            );
                            p.auto_layout();
                            Box::new(p)
                        }
                        fn view(&self) -> Box<dyn WidgetExt> {
                            let mut p = group::Pack::default()
                                .with_label(&format!("{}", #name_str))
                                .with_align(fltk::enums::Align::Left | fltk::enums::Align::Top);
                            p.set_spacing(5);
                            let mems = vec![#(#data_expanded_members_view),*];
                            p.end();
                            let parent = p.parent().unwrap();
                            p.resize(
                                parent.x() + (parent.width()/2), parent.y() + parent.h() / 9, parent.width() / 3, (mems.len() * 30 + 5 * mems.len()) as i32
                            );
                            p.auto_layout();
                            Box::new(p)
                        }
                    }
                };
            }

            _ => {
                return Err(Error::new(
                    Span::call_site(),
                    "Expected a `struct` with named fields",
                ));
            }
        };
        gen.into()
    })
}
