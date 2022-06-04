use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    self,
    parenthesized,
    parse::{Parser, ParseStream},
    Attribute, Data, DeriveInput, Generics, Ident,
};

// TODO: nicer error handling
    // maybe using syn::parse

/// Automatically implement `FromDiscriminant` for enums.
///
/// **Important:** Implementations can only be derived for types that are fieldless enums without generics,
/// and which specify a `#[repr(...)]` attribute.
///
/// # Example
/// ```rust
/// use discrim::FromDiscriminant;
///
/// #[derive(Debug, FromDiscriminant, PartialEq)]
/// #[repr(u8)]
/// enum Opcode {
///     Add, Sub, Mul, Div
/// }
///
/// assert_eq!(Opcode::from_discriminant(2), Ok(Opcode::Mul));
/// assert_eq!(Opcode::from_discriminant(5), Err(5));
/// ```
#[proc_macro_derive(FromDiscriminant)]
pub fn derive_from_discriminant(input: TokenStream) -> TokenStream {
    let input = syn::parse(input).expect("failed to parse macro input");
    let (ty, repr, variants) = unpack_input(input);

    // Declare a constant value per discriminant to match against
    let discriminants = variants.iter().map(|v| {
        let name = format_ident!("D_{}", v);
        quote! {
            const #name: #repr = #ty::#v as #repr;
        }
    });

    // Define match arms for each variant
    let match_arms = variants.iter().map(|v| {
        let name = format_ident!("D_{}", v);
        quote! {
            #name => Ok(#ty::#v),
        }
    });

    quote! {
        impl discrim::FromDiscriminant<#repr> for #ty {
            #[allow(non_upper_case_globals)]
            fn from_discriminant(tag: #repr) -> Result<Self, #repr> {
                #(#discriminants)*

                match tag {
                    #(#match_arms)*
                    other => Err(other),
                }
            }
        }
    }.into()
}

fn unpack_input(input: DeriveInput) -> (Ident, Ident, Vec<Ident>) {
    let data = match input.data {
        Data::Enum(data) => data,
        _ => panic!("input must be an enum"),
    };

    // check that there is at least one variant, and that they're all unit variants
    if data.variants.is_empty() {
        panic!("enum must have at least one variant");
    }

    let variants: Vec<_> = data.variants.into_iter().map(|v| v.ident).collect();

    // disallow generics
    if has_generics(&input.generics) {
        panic!("generic enums are not supported");
    }

    // find and require the repr attribute
    let repr = detect_repr(input.attrs).expect("#[repr(...)] attribute is required");

    // return (ty, repr, variants)
    (input.ident, repr, variants)
}

fn detect_repr(attrs: Vec<Attribute>) -> Option<Ident> {
    // if an attr is the ident "repr", extract its contents and parse them into an ident
    attrs.into_iter()
        .find_map(|attr| {
            if attr.path.is_ident("repr") {
                Some(extract_repr.parse2(attr.tokens).expect("failed to parse tokens in #[repr(...)] attribute"))
            } else {
                None
            }
        })
}

fn extract_repr(input: ParseStream) -> syn::parse::Result<Ident> {
    let repr;
    parenthesized!(repr in input);
    repr.parse()
}

fn has_generics(generics: &Generics) -> bool {
    !generics.params.is_empty() || generics.where_clause.is_some()
}
