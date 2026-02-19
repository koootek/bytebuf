use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, parse_macro_input};

#[proc_macro_derive(IntoBytes)]
pub fn derive_into_bytes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match input.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(named) => named.named.iter().cloned().collect::<Vec<Field>>(),
            Fields::Unnamed(_) | Fields::Unit => vec![],
        },
        Data::Enum(_) | Data::Union(_) => {
            return syn::Error::new_spanned(&input.ident, "incompatible data type")
                .to_compile_error()
                .into();
        }
    };
    let into_bytes_fields = fields.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        quote! {
            #ty::into_bytes(self.#ident, buf)
        }
    });
    TokenStream::from(quote! {
        impl bytebuf::IntoBytes for #name {
            fn into_bytes(self, buf: &mut bytebuf::ByteBuf) {
                #(#into_bytes_fields;)*
            }
        }
    })
}

#[proc_macro_derive(FromBytes)]
pub fn derive_from_bytes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = match input.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(named) => named.named.iter().cloned().collect::<Vec<Field>>(),
            Fields::Unnamed(_) | Fields::Unit => vec![],
        },
        Data::Enum(_) | Data::Union(_) => {
            return syn::Error::new_spanned(&input.ident, "incompatible data type")
                .to_compile_error()
                .into();
        }
    };
    let from_bytes_fields = fields.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        quote! {
            #ident: #ty::from_bytes(buf)?
        }
    });
    TokenStream::from(quote! {
        impl bytebuf::FromBytes for #name {
            fn from_bytes(buf: &mut bytebuf::ByteBuf) -> Option<Self> {
                Some(Self {
                    #(#from_bytes_fields,)*
                })
            }
        }
    })
}
