use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, parse, Fields, Index};


#[proc_macro_derive(Codable)]
pub fn codable_derive(ts: TokenStream) -> TokenStream {
    let input: syn::DeriveInput = parse(ts).expect("Can't parse DeriveInput");
    let data = match input.data {
        Data::Struct(data) => data,
        _ => panic!("Not a struct"),
    };

    let ident = input.ident;
    let adds = data.fields
        .iter()
        .enumerate()
        .map(|(i, field)| {
            if let Some(ref ident) = field.ident {
                quote!{
                    builder.add(&self.#ident);
                }
            } else {
                let index = Index::from(i);
                quote!{
                    builder.add(&self.#index);
                }
            }
        });
    let gets = data.fields
        .iter()
        .map(|field| {
            if let Some(ref ident) = field.ident {
                quote!{
                    #ident: pieces.get()?
                }
            } else {
                quote!{
                    pieces.get()?
                }
            }
        });
    let constructor = if let Fields::Named(_) = data.fields {
        quote!{
            Self {
                #(#gets),*
            }
        }
    } else {
        quote!{
            Self(#(#gets),*)
        }
    };

    (quote!{
        impl crate::protocol::payload::Encodable for &#ident {
            fn encode_to(self, builder: &mut crate::protocol::payload::PayloadBuilder) {
                #(#adds)*
            }
        }

        impl crate::protocol::payload::Decodable for #ident {
            fn decode_from(pieces: &mut crate::protocol::payload::Pieces) -> Result<Self, crate::error::CME> {
                Ok(#constructor)
            }
        }
    }).into()
}
