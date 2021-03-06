extern crate proc_macro;

use std::{borrow::Cow, collections::HashMap};

use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};
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
        Data::Struct(s) => generatable_struct(&input.ident, s, &input.attrs, span),
        Data::Enum(e) => generatable_enum(&input.ident, e, &input.attrs, span),
        Data::Union(_) => panic!("#[derive(Generatable)] is not yet implemented for unions"),
    }
    .unwrap_or_else(|e| e.to_compile_error());

    let ident = input.ident;

    let output: TokenStream2 = quote! {
        impl ::mutagen::Generatable for #ident {
            fn generate_rng<R: ::mutagen::rand::Rng + ?Sized>(rng: &mut R, state: ::mutagen::State) -> Self {
                #body
            }
        }
    };

    proc_macro::TokenStream::from(output)
}

fn generatable_struct(
    ident: &Ident,
    s: &DataStruct,
    _attrs: &[Attribute],
    _span: Span,
) -> Result<TokenStream2> {
    let fields = generatable_fields(&s.fields);

    Ok(quote! {
        #ident #fields
    })
}

fn generatable_enum(
    enum_ident: &Ident,
    e: &DataEnum,
    _attrs: &[Attribute],
    span: Span,
) -> Result<TokenStream2> {
    if e.variants.is_empty() {
        return Err(Error::new(
            span,
            &format!(
                "Cannot derive Generatable for enum {}: no variants",
                enum_ident
            ),
        ));
    }

    roll(
        &e.variants.iter().collect::<Vec<_>>(),
        |variant| {
            Ok(parse_attrs(&variant.attrs, a::ENUM_VARIANT)?
                .get(a::GEN_WEIGHT)
                .cloned()
                .unwrap_or(Value::None))
        },
        |variant, _| {
            let ident = &variant.ident;
            let fields = generatable_fields(&variant.fields);
            quote! {
                    return #enum_ident::#ident #fields;
            }
        },
        &format!("Generation for {}", enum_ident),
    )
}

fn generatable_fields(fields: &Fields) -> TokenStream2 {
    match fields {
        Fields::Named(f) => {
            let name = f.named.iter().map(|f| &f.ident);

            quote! {
                {
                    #(#name: ::mutagen::Generatable::generate_rng(rng, state.deepen())),*
                }
            }
        }
        Fields::Unnamed(f) => {
            let item: Vec<TokenStream2> = f
                .unnamed
                .iter()
                .map(|_| quote! { ::mutagen::Generatable::generate_rng(rng, state.deepen()) })
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
            fn mutate_rng<R: ::mutagen::rand::Rng + ?Sized>(&mut self, rng: &mut R, state: ::mutagen::State) {
                #body
            }
        }
    };

    proc_macro::TokenStream::from(output)
}

fn mutatable_struct(
    ident: &Ident,
    s: &DataStruct,
    _attrs: &[Attribute],
    _span: Span,
) -> Result<TokenStream2> {
    let bindings = fields_bindings(&s.fields);
    let body = mutatable_fields(
        &flatten_fields(&s.fields),
        &ident.to_string(),
        s.fields.span(),
    )?;

    Ok(quote! {
        let #ident #bindings = self;
        #body
    })
}

fn mutatable_enum(
    enum_ident: &Ident,
    e: &DataEnum,
    attrs: &[Attribute],
    _span: Span,
) -> Result<TokenStream2> {
    if e.variants.is_empty() {
        panic!("Cannot derive Mutatable for enum with no variants");
    }

    let attrs = parse_attrs(attrs, a::ENUM)?;
    let mut_reroll_enum = attrs
        .get(a::MUT_REROLL)
        .cloned()
        .unwrap_or(Value::None)
        .to_prob()?;

    let variants: Vec<_> = e
        .variants
        .iter()
        .map(|variant| {
            let variant_attrs = parse_attrs(&variant.attrs, a::ENUM_VARIANT)?;
            let mut_reroll = if let Some(v) = variant_attrs.get(a::MUT_REROLL) {
                v.to_prob()?
            } else {
                mut_reroll_enum.clone()
            };

            let ident = &variant.ident;
            let bindings = fields_bindings(&variant.fields);
            let fields_body = mutatable_fields(
                &flatten_fields(&variant.fields),
                &format!("{}::{}", &enum_ident, &variant.ident),
                variant.fields.span(),
            )?;

            let out: TokenStream2 = if let Some(mut_reroll) = mut_reroll {
                quote! {
                    #enum_ident::#ident #bindings => {
                        if rng.sample(::rand::distributions::Bernoulli::new(#mut_reroll).unwrap()) {
                            *self = ::mutagen::Generatable::generate_rng(rng, state);
                        } else {
                            #fields_body
                        }
                    }
                }
            } else {
                quote! {
                    #enum_ident::#ident #bindings => { #fields_body }
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

fn mutatable_fields(fields: &[&Field], path: &str, _span: Span) -> Result<TokenStream2> {
    if fields.is_empty() {
        return Ok(TokenStream2::new());
    }

    roll(
        fields,
        |field| {
            Ok(parse_attrs(&field.attrs, a::FIELD)?
                .get(a::MUT_WEIGHT)
                .cloned()
                .unwrap_or(Value::None))
        },
        |field, i| {
            let ident = field_ident(field, i);
            quote! {
                ::mutagen::Mutatable::mutate_rng(#ident, rng, state.deepen());
                return;
            }
        },
        &format!("mutation for {}", path),
    )
}

#[proc_macro_derive(Updatable, attributes(mutagen))]
pub fn derive_updatable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let span = input.span();

    let body = match &input.data {
        Data::Struct(s) => updatable_struct(&input.ident, s, &input.attrs, span),
        Data::Enum(e) => updatable_enum(&input.ident, e, &input.attrs, span),
        Data::Union(_) => panic!("#[derive(Updatable)] is not implemented for unions"),
    }
    .unwrap_or_else(|e| e.to_compile_error());

    let ident = input.ident;

    let output: TokenStream2 = quote! {
        impl ::mutagen::Updatable for #ident {
            fn update(&mut self, state: ::mutagen::State) {
                #body
            }
        }
    };

    proc_macro::TokenStream::from(output)
}

fn updatable_struct(
    ident: &Ident,
    s: &DataStruct,
    _attrs: &[Attribute],
    _span: Span,
) -> Result<TokenStream2> {
    let bindings = fields_bindings(&s.fields);
    let body = updatable_fields(
        &flatten_fields(&s.fields),
        &ident.to_string(),
        s.fields.span(),
    )?;

    Ok(quote! {
        let #ident #bindings = self;
        #body
    })
}

fn updatable_enum(
    enum_ident: &Ident,
    e: &DataEnum,
    _attrs: &[Attribute],
    _span: Span,
) -> Result<TokenStream2> {
    if e.variants.is_empty() {
        panic!("Cannot derive Mutatable for enum with no variants");
    }

    let variants: Vec<_> = e
        .variants
        .iter()
        .map(|variant| {
            let ident = &variant.ident;
            let bindings = fields_bindings(&variant.fields);
            let fields_body = updatable_fields(
                &flatten_fields(&variant.fields),
                &format!("{}::{}", &enum_ident, &variant.ident),
                variant.fields.span(),
            )?;

            let out: TokenStream2 = quote! {
                #enum_ident::#ident #bindings => { #fields_body }
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

fn updatable_fields(fields: &[&Field], _path: &str, _span: Span) -> Result<TokenStream2> {
    if fields.is_empty() {
        return Ok(TokenStream2::new());
    }

    Ok(fields
        .iter()
        .enumerate()
        .map(|(i, field)| {
            let ident = field_ident(field, i);
            quote! {
                ::mutagen::Updatable::update(#ident, state.deepen());
            }
        })
        .collect())
}

fn roll<T, Wf, Bf>(choices: &[T], weight_fn: Wf, body_fn: Bf, err: &str) -> Result<TokenStream2>
where
    Wf: Fn(&T) -> Result<Value>,
    Bf: Fn(&T, usize) -> TokenStream2,
{
    let n = choices.len();

    if n == 0 {
        panic!("roll was called with 0 choices");
    } else if n == 1 {
        return Ok(body_fn(&choices[0], 0));
    }

    let weight_values: Vec<_> = choices.iter().map(weight_fn).collect::<Result<_>>()?;

    let weights_opt: Vec<Option<TokenStream2>> = weight_values
        .iter()
        .map(Value::to_weight)
        .collect::<Result<_>>()?;

    let weights: Vec<TokenStream2> = weights_opt
        .iter()
        .map(|w| w.as_ref().cloned().unwrap_or(quote!(0.0)))
        .collect();

    let cumul_idents: Vec<Ident> = (0..n)
        .map(|i| format_ident!("cumul_weights_{}", i))
        .collect();

    let cumul_sum: TokenStream2 = cumul_idents
        .windows(2)
        .enumerate()
        .map(|(i, w)| {
            let pre = &w[0];
            let ident = &w[1];
            quote! {
                let #ident: f64 = #pre + weights[#i + 1];
            }
        })
        .collect();

    let checks: TokenStream2 = choices
        .iter()
        .zip(weights_opt.iter())
        .enumerate()
        .filter(|(_, (_, weight))| weight.is_some())
        .map(|(i, (choice, _))| {
            let body = body_fn(choice, i);
            quote! { if roll < cumul_weights[#i] { #body }}
        })
        .collect();

    Ok(quote! {
        let weights: [f64; #n] = [
            #(#weights),*
        ];

        let cumul_weights_0: f64 = weights[0];
        #cumul_sum

        let cumul_weights: [f64; #n] = [
            #(#cumul_idents),*
        ];


        let total_weight = cumul_weights[#n - 1];
        assert!(total_weight > 0.0, "Failed to roll {}. Total weight was {} (should be > 0).", #err, total_weight);
        let roll: f64 = rng.gen_range(0.0, total_weight);

        #checks

        unreachable!("Failed to roll {}. Rolled {}, total weight is {}", #err, roll, total_weight)
    })
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
    value: Value,
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

#[derive(Clone)]
enum Value {
    Lit(LitFloat),
    FnIdent(Ident),
    None,
}

impl Value {
    fn to_weight(&self) -> Result<Option<TokenStream2>> {
        match self {
            Value::Lit(lit) => {
                let v: f64 = lit.base10_parse()?;

                if v < 0.0 {
                    Err(Error::new(lit.span(), "Invalid weight"))
                } else if v == 0.0 {
                    Ok(None)
                } else {
                    Ok(Some(lit.to_token_stream()))
                }
            }
            Value::FnIdent(ident) => {
                let ident_s = ident.to_string();

                Ok(Some(quote! {
                {
                    let value = #ident (&state);
                    assert!(value >= 0.0, "{} returned invalid weight {}", #ident_s, value);
                    value
                }
                }))
            }
            Value::None => Ok(Some(quote!(1.0))),
        }
    }

    fn to_prob(&self) -> Result<Option<TokenStream2>> {
        match self {
            Value::Lit(lit) => {
                let v: f64 = lit.base10_parse()?;
                if v < 0.0 || v > 1.0 {
                    Err(Error::new(lit.span(), "Invalid probability"))
                } else if v == 0.0 {
                    Ok(None)
                } else {
                    Ok(Some(lit.to_token_stream()))
                }
            }
            Value::FnIdent(ident) => {
                let ident_s = ident.to_string();

                Ok(Some(quote! {
                    {
                        let value = #ident (&state);
                        assert!(value >= 0.0, "{} returned invalid probability {}", #ident_s, value);
                        value
                    }
                }))
            }
            Value::None => Ok(Some(quote!(0.5))),
        }
    }
}

impl Parse for Value {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitFloat) {
            input.parse().map(Value::Lit)
        } else if lookahead.peek(Ident) {
            input.parse().map(Value::FnIdent)
        } else {
            Err(lookahead.error())
        }
    }
}

type Attrs = HashMap<String, Value>;

fn parse_attrs(attributes: &[Attribute], allowed: &[&str]) -> Result<Attrs> {
    Ok(attributes
        .iter()
        .filter(|attr| attr.path.is_ident(a::BASE))
        .map(|attr| {
            let data: AttrsData = parse2(attr.tokens.clone())?;
            data.values
                .into_iter()
                .map(|kv| {
                    let key = kv.key.to_string();

                    if !allowed.contains(&key.as_str()) {
                        return Err(Error::new(
                            attr.span(),
                            format!(
                                "Attribute {} not allowed on this item. Allowed attributes: {:?}",
                                key, allowed
                            ),
                        ));
                    }

                    Ok((key, kv.value))
                })
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect())
}
