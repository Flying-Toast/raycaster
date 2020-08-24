use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, parse};


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
        .map(|field| {
            let ident = field.ident.as_ref().unwrap();
            quote!{
                builder.add(self.#ident);
            }
        });
    let gets = data.fields
        .iter()
        .map(|field| {
            let ident = field.ident.as_ref().unwrap();
            quote!{
                #ident: pieces.get()?,
            }
        });

    (quote!{
        impl crate::protocol::payload::Encodable for &#ident {
            fn encode_to(self, builder: &mut crate::protocol::payload::PayloadBuilder) {
                #(#adds)*
            }
        }

        impl crate::protocol::payload::Decodable for #ident {
            fn decode_from(pieces: &mut crate::protocol::payload::Pieces) -> Result<Self, crate::error::CME> {
                Ok(Self {
                    #(#gets)*
                })
            }
        }
    }).into()
}
