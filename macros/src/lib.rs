use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemStruct};

/// Generate getters and setters procedurally.
///
/// Annotate fields with `#[get]` to generate a getter method.
/// ```ignore
/// use ters::ters;
///
/// #[ters]
/// struct Foo {
///     a: i32,
///     #[get]
///     b: bool,
/// }
///
/// fn getters() {
///     let foo = Foo { a: 42, b: true };
///     assert_eq!(foo.b(), &true);
/// }
/// ```
///
/// Annotate fields with `#[set]` to generate a setter method.
/// ```ignore
/// use ters::ters;
///
/// #[ters]
/// struct Foo {
///     #[set]
///     a: i32,
///     b: bool,
/// }
///
/// fn setters() {
///     let mut foo = Foo { a: 42, b: true };
///     foo.set_a(31);
/// }
/// ```
///
/// Annotate fields with `#[get]` and `#[set]` to generate both a getter and a setter method.
/// ```ignore
/// use ters::ters;
///
/// #[ters]
/// struct Foo {
///     #[get]
///     #[set]
///     a: i32,
///     b: bool,
/// }
///
/// fn getters_and_setters() {
///     let mut foo = Foo { a: 42, b: true };
///     assert_eq!(foo.a(), &42);
///     foo.set_a(31);
///
///     assert_eq!(foo.a(), &31);
/// }
/// ```
///
/// Unannotated fields will not have generated getters or setters.
/// ```ignore
/// use ters::ters;
///
/// #[ters]
/// struct Foo {
///     a: i32,
///     #[get]
///     b: bool,
/// }
///
/// fn getters_not_generated() {
///     let foo = Foo { a: 42, b: true };
///     assert_eq!(foo.a(), &42); // this method doesn't exist
/// }
/// ```
#[proc_macro_attribute]
pub fn ters(_args: TokenStream, tokens: TokenStream) -> TokenStream {
    let item = parse_macro_input!(tokens as ItemStruct);

    ters_inner(item).into()
}

fn ters_inner(mut item: ItemStruct) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    let mut fields = Vec::new();

    for field in item.fields.iter_mut() {
        let mut get = false;
        let mut set = false;

        field.attrs.retain(|attr| {
            if attr.path().is_ident("get") {
                get = true;
                false
            } else if attr.path().is_ident("set") {
                set = true;
                false
            } else {
                true
            }
        });

        fields.push((
            field.ident.clone().unwrap(),
            field.ty.clone(),
            get,
            set,
            field
                .attrs
                .iter()
                .filter(|attr| {
                    attr.path()
                        .get_ident()
                        .map(|ident| ident == "doc")
                        .is_some_and(|is_doc| is_doc)
                })
                .cloned()
                .collect::<Vec<_>>(),
        ));
    }

    let accessors = fields
        .iter()
        .filter_map(|(ident, ty, get, set, docs)| {
            let set_ident = format_ident!("set_{ident}");
            let str_ident = ident.to_string();

            let mut body = quote! {};

            if *get {
                body.extend(quote! {
                    #[doc = "Getter for `"]
                    #[doc = #str_ident]
                    #[doc = "`.\n\n"]
                    #(#docs)*
                    #[inline]
                    pub fn #ident(&self) -> &#ty {
                        &self.#ident
                    }
                });
            }

            if *set {
                body.extend(quote! {
                    #[doc = "Setter for `"]
                    #[doc = #str_ident]
                    #[doc = "`.\n\n"]
                    #(#docs)*
                    #[inline]
                    pub fn #set_ident(&mut self, value: #ty) {
                        self.#ident = value;
                    }
                });
            }

            (!body.is_empty()).then_some(body)
        })
        .collect::<Vec<_>>();

    let ident = &item.ident;

    let impl_ = (!accessors.is_empty()).then_some(quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            #(
                #accessors
            )*
        }
    });

    quote! {
        #item
        #impl_
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::parse_quote;

    use crate::ters_inner;

    #[test]
    fn docs() {
        let input = parse_quote! {
            struct Foo {
                /// Baz.
                #[get]
                bar: u8,
            }
        };

        let expected = quote! {
            struct Foo {
                /// Baz.
                bar: u8,
            }

            impl Foo {
                #[doc = "Getter for `"]
                #[doc = "bar"]
                #[doc = "`.\n\n"]
                /// Baz.
                #[inline]
                pub fn bar(&self) -> &u8 {
                    &self.bar
                }
            }
        };

        let out: proc_macro2::TokenStream = ters_inner(input).into();

        assert_eq!(out.to_string(), expected.to_string());
    }
}
