use std::collections::HashMap;

use proc_macro2::TokenStream as TokenStream2;
use proc_quote::quote;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse2, parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    token::Paren,
    Attribute, Data, DataEnum, DataStruct, Error, Fields, Ident, LitFloat, Result, Token,
};

mod a {
    // Base path in the attribute
    pub const BASE: &str = "mutagen";

    // Keys
    pub const GEN_WEIGHT: &str = "gen_weight";
    pub const MUT_REROLL: &str = "mut_reroll";

    // Allowed keys for each item
    pub const ENUM_VARIANT: &[&str] = &[GEN_WEIGHT, MUT_REROLL];
}

#[proc_macro_derive(Generatable, attributes(mutagen))]
pub fn derive_generatable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let body = match &input.data {
        Data::Struct(s) => generatable_struct(s),
        Data::Enum(e) => generatable_enum(e),
        Data::Union(_) => panic!("#[derive(Generatable)] is not yet implemented for unions"),
    }
    .unwrap_or_else(|e| e.to_compile_error());

    let ident = input.ident;

    let output: TokenStream2 = quote! {
        impl Generatable for #ident {
            fn generate_rng<R: ::mutagen::rand::Rng + ?Sized>(rng: &mut R) -> Self {
                #body
            }
        }
    };

    proc_macro::TokenStream::from(output)
}

fn generatable_struct(s: &DataStruct) -> Result<TokenStream2> {
    let fields = generatable_fields(&s.fields);

    Ok(quote! {
        Self #fields
    })
}

fn generatable_enum(e: &DataEnum) -> Result<TokenStream2> {
    if e.variants.is_empty() {
        panic!("Cannot derive Generatable for enum with no variants");
    }

    let attrs: Vec<_> = e
        .variants
        .iter()
        .map(|v| parse_attrs(&v.attrs, a::ENUM_VARIANT))
        .collect::<Result<_>>()?;

    let weights: Vec<_> = attrs
        .iter()
        .map(|a| a.get(a::GEN_WEIGHT).copied().unwrap_or(1.0))
        .collect();

    let (range_start, range_end): (Vec<_>, Vec<_>) = weights
        .iter()
        .scan(0.0, |top, w| {
            let item = (*top, *top + w);
            *top = item.1;
            Some(item)
        })
        .unzip();
    let total_weight: f32 = weights.iter().sum();
    let ident = e.variants.iter().map(|v| &v.ident);
    let fields = e.variants.iter().map(|v| generatable_fields(&v.fields));

    Ok(quote! {
        let roll: f32 = rng.gen_range(0.0, #total_weight);

        #(
            if roll >= #range_start && roll < #range_end {
                return Self::#ident #fields;
            }
        )*

        unreachable!();
    })
}

fn generatable_fields(fields: &Fields) -> TokenStream2 {
    match fields {
        Fields::Named(f) => {
            let name = f.named.iter().map(|f| &f.ident);

            quote! {
                {
                    #(#name: ::mutagen::Generatable::generate_rng(rng)),*
                }
            }
        }
        Fields::Unnamed(f) => {
            let item: Vec<TokenStream2> = f
                .unnamed
                .iter()
                .map(|_| quote! { ::mutagen::Generatable::generate_rng(rng) })
                .collect();

            quote! {
                (
                    #(#item),*
                )
            }
        }
        Fields::Unit => TokenStream2::new(),
    }
}

#[proc_macro_derive(Mutatable, attributes(mutagen))]
pub fn derive_mutatable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let body = match &input.data {
        Data::Struct(s) => mutatable_struct(s),
        Data::Enum(e) => mutatable_enum(e),
        Data::Union(_) => panic!("#[derive(Mutatable)] is not yet implemented for unions"),
    }
    .unwrap_or_else(|e| e.to_compile_error());

    let ident = input.ident;

    let output: TokenStream2 = quote! {
        impl Mutatable for #ident {
            fn mutate(&mut Self) {
                #body
            }
        }
    };

    proc_macro::TokenStream::from(output)
}

fn mutatable_struct(s: &DataStruct) -> Result<TokenStream2> {
    todo!()
}

fn mutatable_enum(e: &DataEnum) -> Result<TokenStream2> {
    if e.variants.is_empty() {
        panic!("Cannot derive Mutatable for enum with no variants");
    }

    let attrs: Vec<_> = e
        .variants
        .iter()
        .map(|v| parse_attrs(&v.attrs, a::ENUM_VARIANT))
        .collect::<Result<_>>()?;

    todo!()
}

fn mutatable_fields(fields: &Fields) -> TokenStream2 {
    match fields {
        Fields::Named(f) => {
            let name = f.named.iter().map(|f| &f.ident);

            quote! {
                {
                    #(#name: ::mutagen::Generatable::generate_rng(rng)),*
                }
            }
        }
        Fields::Unnamed(f) => {
            let item: Vec<TokenStream2> = f
                .unnamed
                .iter()
                .map(|_| quote! { ::mutagen::Generatable::generate_rng(rng) })
                .collect();

            quote! {
                (
                    #(#item),*
                )
            }
        }
        Fields::Unit => TokenStream2::new(),
    }
}

struct AttrsData {
    _paren: Paren,
    values: Punctuated<KeyValue, Token![,]>,
}

impl Parse for AttrsData {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            _paren: parenthesized!(content in input),
            values: content.parse_terminated(KeyValue::parse)?,
        })
    }
}

struct KeyValue {
    key: Ident,
    _eq: Token![=],
    value: LitFloat,
}

impl Parse for KeyValue {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            key: input.parse()?,
            _eq: input.parse()?,
            value: input.parse()?,
        })
    }
}

type Attrs = HashMap<String, f32>;

fn parse_attrs(attributes: &[Attribute], allowed: &[&str]) -> Result<Attrs> {
    Ok(attributes
        .iter()
        .filter(|attr| attr.path.is_ident(a::BASE))
        .map(|attr| {
            let data: AttrsData = parse2(attr.tokens.clone())?;
            data.values
                .iter()
                .map(|kv| {
                    let key = kv.key.to_string();
                    let value: f32 = kv.value.base10_parse()?;

                    if !allowed.contains(&key.as_str()) {
                        return Err(Error::new(
                            attr.span(),
                            format!(
                                "Attribute {} not allowed on this item. Allowed attributes: {:?}",
                                key, allowed
                            ),
                        ));
                    }

                    Ok((key, value))
                })
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect())
}
