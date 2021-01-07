extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::ParseBuffer;
use syn::Token;
use syn::{self, DeriveInput, Error, Ident};

struct LocoPacketPair(Ident, Ident);

impl syn::parse::Parse for LocoPacketPair {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        let content;
        syn::parenthesized!(content in input);
        let type1 = content.parse()?;
        content.parse::<Token![,]>()?;
        let type2 = content.parse()?;
        Ok(LocoPacketPair(type1, type2))
    }
}

#[proc_macro_derive(LocoPacketPair, attributes(loco_packet_pair))]
pub fn derive_loco_packet_pair(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();
    let attribute = derive_input
        .attrs
        .iter()
        .find(|a| a.path.segments.len() == 1 && a.path.segments[0].ident == "loco_packet_pair")
        .expect("attribute loco_packet_pair(request, response)");

    let command_pair: LocoPacketPair =
        syn::parse2(attribute.tokens.clone()).expect("Invalid loco_packet_pair attribute!");

    let name = derive_input.ident;
    let request = command_pair.0;
    let response = command_pair.1;
    let name_uppercase = Ident::new(&name.to_string().to_uppercase(), name.span());

    let token_stream = quote! {

        impl crate::protocol::LocoPacket for #request {
            const NAME: &'static str = &stringify!(#name_uppercase);
        }

        impl crate::protocol::LocoRequest for #request {}

        impl crate::protocol::LocoPacket for #response {
            const NAME: &'static str = &stringify!(#name_uppercase);
        }

        impl crate::protocol::LocoResponse for #response {}
    };

    token_stream.into()
}

#[proc_macro_derive(LocoPacket)]
pub fn derive_loco_packet(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();
    let name = derive_input.ident;
    let name_uppercase = Ident::new(&name.to_string().to_uppercase(), name.span());

    let token_stream = quote! {
        impl crate::protocol::LocoPacket for #name {
            const NAME: &'static str = &stringify!(#name_uppercase);
        }
    };

    token_stream.into()
}

#[proc_macro_derive(LocoRequest)]
pub fn derive_loco_request(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();
    let name = derive_input.ident;
    let name_uppercase = Ident::new(&name.to_string().to_uppercase(), name.span());

    let token_stream = quote! {
        impl crate::protocol::LocoPacket for #name {
            const NAME: &'static str = &stringify!(#name_uppercase);
        }

        impl crate::protocol::LocoRequest for #name {}
    };

    token_stream.into()
}

#[proc_macro_derive(LocoResponse)]
pub fn derive_loco_response(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();
    let name = derive_input.ident;
    let name_uppercase = Ident::new(&name.to_string().to_uppercase(), name.span());

    let token_stream = quote! {
        impl crate::protocol::LocoPacket for #name {
            const NAME: &'static str = &stringify!(#name_uppercase);
        }

        impl crate::protocol::LocoResponse for #name {}
    };

    token_stream.into()
}

#[proc_macro_derive(BsonData)]
pub fn bson_data(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_bson_data(&ast)
}

fn impl_bson_data(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl crate::protocol::BsonData<'_> for #name {} //for implementation checK

        impl crate::protocol::Encode for #name {
            fn encode<U: std::io::Write>(&self, buffer: &mut U) {
                bson::ser::to_document(&self).unwrap().to_writer(buffer).unwrap();
            }
        }

        impl crate::protocol::Decode for #name {
            fn decode<U: std::io::Read>(&self, buffer: &mut U) -> Self {
                bson::de::from_document(bson::Document::from_reader(buffer).unwrap()).unwrap()
            }
        }
    };

    gen.into()
}
