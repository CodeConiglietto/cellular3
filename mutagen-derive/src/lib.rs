use std::{borrow::Cow, collections::HashMap};

use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse2, parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    token::Paren,
    Attribute, Data, DataEnum, DataStruct, Error, Field, Fields, Ident, LitFloat, Result, Token,
};

mod a {
    // Base path in the attribute
    pub const BASE: &str = "mutagen";

    // Keys
    pub const GEN_WEIGHT: &str = "gen_weight";
    pub const MUT_REROLL: &str = "mut_reroll";
    pub const MUT_WEIGHT: &str = "mut_weight";

    // Allowed keys for each item
    pub const ENUM: &[&str] = &[MUT_REROLL];
    pub const ENUM_VARIANT: &[&str] = &[GEN_WEIGHT, MUT_REROLL];
    pub const FIELD: &[&str] = &[MUT_WEIGHT];
}

#[proc_macro_derive(Generatable, attributes(mutagen))]
pub fn derive_generatable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let span = input.span();

    let body = match &input.data {
        Data::Struct(s) => generatable_struct(s, &input.attrs, span),
        Data::Enum(e) => generatable_enum(e, &input.attrs, span),
        Data::Union(_) => panic!("#[derive(Generatable)] is not yet implemented for unions"),
    }
    .unwrap_or_else(|e| e.to_compile_error());

    let ident = input.ident;

    let output: TokenStream2 = quote! {
        impl ::mutagen::Generatable for #ident {
            fn generate_rng<R: ::mutagen::rand::Rng + ?Sized>(rng: &mut R) -> Self {
                #body
            }
        }
    };

    proc_macro::TokenStream::from(output)
}

fn generatable_struct(s: &DataStruct, _attrs: &[Attribute], _span: Span) -> Result<TokenStream2> {
    let fields = generatable_fields(&s.fields);

    Ok(quote! {
        Self #fields
    })
}

fn generatable_enum(e: &DataEnum, _attrs: &[Attribute], span: Span) -> Result<TokenStream2> {
    if e.variants.is_empty() {
        return Err(Error::new(
            span,
            "Cannot derive Generatable for enum with no variants",
        ));
    }

    let weights: Vec<_> = e
        .variants
        .iter()
        .map(|v| {
            let attrs = parse_attrs(&v.attrs, a::ENUM_VARIANT)?;
            let weight = attrs.get(a::GEN_WEIGHT).copied().unwrap_or(1.0);

            if weight < 0.0 {
                return Err(Error::new(
                    v.span(),
                    format!("Invalid variant gen_weight: {}", weight),
                ));
            }

            Ok(weight)
        })
        .collect::<Result<_>>()?;

    let total_weight: f64 = weights.iter().sum();

    if total_weight <= 0.0 {
        return Err(Error::new(span, "Sum of variant gen_weight values is 0"));
    }

    let mut top = 0.0;

    let mut out: TokenStream2 = e
        .variants
        .iter()
        .zip(weights.iter())
        .filter(|(_, weight)| **weight > 0.0)
        .map(|(variant, weight)| {
            let range_start = top;
            let range_end = top + weight;

            top += weight;

            let ident = &variant.ident;
            let fields = generatable_fields(&variant.fields);

            let out: TokenStream2 = quote! {
                let roll: f64 = rng.gen_range(0.0, #total_weight);

                if roll >= #range_start && roll < #range_end {
                    return Self::#ident #fields;
                }
            };

            out
        })
        .collect();

    out.extend(quote! {
        unreachable!()
    });

    Ok(out)
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
    let span = input.span();

    let body = match &input.data {
        Data::Struct(s) => mutatable_struct(&input.ident, s, &input.attrs, span),
        Data::Enum(e) => mutatable_enum(&input.ident, e, &input.attrs, span),
        Data::Union(_) => panic!("#[derive(Mutatable)] is not yet implemented for unions"),
    }
    .unwrap_or_else(|e| e.to_compile_error());

    let ident = input.ident;

    let output: TokenStream2 = quote! {
        impl ::mutagen::Mutatable for #ident {
            fn mutate_rng<R: ::mutagen::rand::Rng + ?Sized>(&mut self, rng: &mut R) {
                #body
            }
        }
    };

    proc_macro::TokenStream::from(output)
}

fn mutatable_struct(
    _ident: &Ident,
    s: &DataStruct,
    _attrs: &[Attribute],
    _span: Span,
) -> Result<TokenStream2> {
    let bindings = fields_bindings(&s.fields);
    let body = mutatable_fields(&flatten_fields(&s.fields), s.fields.span())?;

    Ok(quote! {
        let Self #bindings = self;
        #body
    })
}

fn mutatable_enum(
    enum_ident: &Ident,
    e: &DataEnum,
    attrs: &[Attribute],
    span: Span,
) -> Result<TokenStream2> {
    if e.variants.is_empty() {
        panic!("Cannot derive Mutatable for enum with no variants");
    }

    let attrs = parse_attrs(attrs, a::ENUM)?;
    let mut_reroll_enum = attrs.get(a::MUT_REROLL).copied().unwrap_or(0.5);

    if mut_reroll_enum < 0.0 || mut_reroll_enum > 1.0 {
        return Err(Error::new(
            span,
            &format!(
                "Invalid mut_reroll attribute on enum {} ({}). Should be between 0.0 and 1.0 inclusive.",
                enum_ident, mut_reroll_enum
            ),
        ));
    }

    let variants: Vec<_> = e
        .variants
        .iter()
        .map(|variant| {
            let variant_attrs = parse_attrs(&variant.attrs, a::ENUM_VARIANT)?;
            let mut_reroll = variant_attrs
                .get(a::MUT_REROLL)
                .copied()
                .unwrap_or(mut_reroll_enum);

            if mut_reroll < 0.0 || mut_reroll > 1.0 {
                return Err(Error::new(
                    span,
                    &format!(
                        "Invalid mut_reroll attribute on enum variant {}::{} ({}). Should be between 0.0 and 1.0 inclusive.",
                        enum_ident, variant.ident, mut_reroll,
                    ),
                ));
            }

            let ident = &variant.ident;
            let bindings = fields_bindings(&variant.fields);
            let fields_body =
                mutatable_fields(&flatten_fields(&variant.fields), variant.fields.span())?;

            let out: TokenStream2 = if mut_reroll > 0.0 {
                quote! {
                    Self::#ident #bindings => {
                        if rng.sample(::rand::distributions::Bernoulli::new(#mut_reroll).unwrap()) {
                            *self = ::mutagen::Generatable::generate();
                        } else {
                            #fields_body
                        }
                    }
                }
            } else {
                quote! {
                    Self::#ident #bindings => { #fields_body }
                }
            };

            Ok(out)
        })
        .collect::<Result<_>>()?;

    Ok(quote! {
        match self {
            #( #variants )*
        }
    })
}

fn fields_bindings(fields: &Fields) -> TokenStream2 {
    match fields {
        Fields::Named(fields) => {
            let fields = fields.named.iter().map(|field| &field.ident);
            quote! {
                { #(#fields),* }
            }
        }
        Fields::Unnamed(fields) => {
            let fields = fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, _)| tuple_field_ident(i));
            quote! {
                ( #(#fields),* )
            }
        }
        Fields::Unit => TokenStream2::new(),
    }
}

fn mutatable_fields(fields: &[&Field], _span: Span) -> Result<TokenStream2> {
    if fields.is_empty() {
        return Ok(TokenStream2::new());
    }

    let weights: Vec<_> = fields
        .iter()
        .map(|field| {
            let attrs = parse_attrs(&field.attrs, a::FIELD)?;
            let weight = attrs.get(a::MUT_WEIGHT).copied().unwrap_or(1.0);

            if weight < 0.0 {
                return Err(Error::new(
                    field.span(),
                    format!("Invalid field mut_weight: {}", weight),
                ));
            }

            Ok(weight)
        })
        .collect::<Result<_>>()?;

    let total_weight: f64 = weights.iter().sum();

    if total_weight <= 0.0 {
        return Ok(TokenStream2::new());
    }

    let mut top = 0.0;

    let mut out: TokenStream2 = fields
        .iter()
        .zip(weights.iter())
        .filter(|(_, weight)| **weight > 0.0)
        .enumerate()
        .map(|(i, (field, weight))| {
            let range_start = top;
            let range_end = top + weight;

            top += weight;

            let ident = field_ident(field, i);

            let out: TokenStream2 = quote! {
                let roll: f64 = rng.gen_range(0.0, #total_weight);

                if roll >= #range_start && roll < #range_end {
                    ::mutagen::Mutatable::mutate_rng(#ident, rng);
                }
            };

            out
        })
        .collect();

    out.extend(quote! {
        unreachable!()
    });

    Ok(out)
}

fn flatten_fields(fields: &Fields) -> Vec<&Field> {
    match fields {
        Fields::Named(fields) => fields.named.iter().collect(),
        Fields::Unnamed(fields) => fields.unnamed.iter().collect(),
        Fields::Unit => Vec::new(),
    }
}

fn field_ident(field: &Field, i: usize) -> Cow<'_, Ident> {
    if let Some(ident) = field.ident.as_ref() {
        Cow::Borrowed(ident)
    } else {
        Cow::Owned(tuple_field_ident(i))
    }
}

fn tuple_field_ident(i: usize) -> Ident {
    Ident::new(&format!("_{}", i), Span::call_site())
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

type Attrs = HashMap<String, f64>;

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
                    let value: f64 = kv.value.base10_parse()?;

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
