use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn ters(_args: TokenStream, item: TokenStream) -> TokenStream {
    let mut s = parse_macro_input!(item as ItemStruct);

    let (impl_generics, ty_generics, where_clause) = s.generics.split_for_impl();

    let mut fields = Vec::new();

    for field in s.fields.iter_mut() {
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

        fields.push((field.ident.clone().unwrap(), field.ty.clone(), get, set));
    }

    let accessors = fields.iter().map(|(ident, ty, get, set)| {
        let set_ident = format_ident!("set_{ident}");

        let mut body = quote! {};

        if *get {
            body.extend(quote! {
                pub fn #ident(&self) -> &#ty {
                    &self.#ident
                }
            });
        }

        if *set {
            body.extend(quote! {
                pub fn #set_ident(&mut self, value: #ty) {
                    self.#ident = value;
                }
            });
        }

        body
    });

    let ident = &s.ident;

    quote! {
        #s

        impl #impl_generics #ident #ty_generics #where_clause {
            #(
                #accessors
            )*
        }
    }
    .into()
}
