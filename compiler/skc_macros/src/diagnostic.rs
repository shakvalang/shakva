use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{
    DeriveInput, Ident, LitStr, Token,
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

pub fn derive(input: TokenStream) -> TokenStream {
    expand(input).unwrap_or_else(syn::Error::into_compile_error)
}

fn expand(input: TokenStream) -> syn::Result<TokenStream> {
    let input: DeriveInput = syn::parse2(input)?;

    match &input.data {
        syn::Data::Struct(data_struct) => expand_struct(&input, data_struct),
        syn::Data::Enum(_) => unimplemented!(),
        syn::Data::Union(_) => unimplemented!(),
    }
}

/// Parsed contents of `#[diag(lvl, "message")]`.
#[derive(Debug)]
struct DiagHeader {
    /// `::skc_diag::Diagnostic::{error|warning|note}`
    lvl: TokenStream,
    /// The format string; bound field names can be used inline, e.g.
    /// `"{found}"`.
    msg: LitStr,
}

/// Raw syntax of the `diag` attribute body: an ident, a comma, a string
/// literal.
#[derive(Debug)]
struct DiagArgs {
    /// `error|warning|note`
    lvl: Ident,
    msg: LitStr,
}

impl Parse for DiagArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lvl: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let msg: LitStr = input.parse()?;
        Ok(Self { lvl, msg })
    }
}

fn parse_header(attrs: &[syn::Attribute]) -> syn::Result<DiagHeader> {
    let attr = attrs
        .iter()
        .find(|a| a.path().is_ident("diag"))
        .ok_or_else(|| {
            syn::Error::new(
                Span::call_site(),
                "missing `#[diag(lvl, \"message\")]` attribute",
            )
        })?;

    let DiagArgs { lvl, msg } = attr.parse_args()?;

    let variant = match lvl.to_string().as_str() {
        "error" | "warning" | "note" => lvl.to_token_stream(),
        other => {
            return Err(syn::Error::new_spanned(
                &lvl,
                format!(
                    "unknown diagnostic level `{other}`, expected `error`, `warning` or `note`"
                ),
            ));
        }
    };

    Ok(DiagHeader {
        lvl: quote!(::skc_diag::Diagnostic::#variant),
        msg,
    })
}

/// The role a field plays in the generated diagnostic, decided by which
/// marker attribute (if any) is present on it.
#[derive(Debug)]
enum Role {
    /// `#[primary_span]` - the field becomes the diagnostic's primary span.
    PrimarySpan,
    /// `#[label("...")]` - a secondary label pointing at this span field.
    Label { msg: LitStr },
    /// No marker; the field only contributes to message interpolation.
    Plain,
}

#[derive(Debug)]
struct Field {
    ident: Ident,
    role: Role,
}

fn parse_field(field: &syn::Field) -> syn::Result<Field> {
    let ident = field.ident.clone().ok_or_else(|| {
        syn::Error::new_spanned(field, "tuple structs are not supported; use a named struct")
    })?;

    let mut role = Role::Plain;
    let mut seen: Option<Span> = None;

    for attr in &field.attrs {
        let new_role = if attr.path().is_ident("primary_span") {
            attr.meta.require_path_only()?; // reject `#[primary_span(...)]`
            Role::PrimarySpan
        } else if attr.path().is_ident("label") {
            Role::Label {
                msg: attr.parse_args::<LitStr>()?,
            }
        } else {
            continue; // not one of ours, ignore
        };

        if let Some(prev) = seen {
            let mut err = syn::Error::new(
                attr.span(),
                "a field can have at most one diagnostic marker",
            );
            err.combine(syn::Error::new(prev, "previous marker here"));
            return Err(err);
        }
        seen = Some(attr.span());
        role = new_role;
    }

    Ok(Field { ident, role })
}

fn find_primary_span<'a>(
    input: &'a syn::DeriveInput,
    fields: &'a [Field],
) -> syn::Result<&'a Ident> {
    let primary_spans: Vec<&Ident> = fields
        .iter()
        .filter_map(|f| {
            if matches!(f.role, Role::PrimarySpan) {
                Some(&f.ident)
            } else {
                None
            }
        })
        .collect();

    match primary_spans.as_slice() {
        [span] => Ok(*span),
        [] => Err(syn::Error::new_spanned(
            input,
            "exactly one field must be marked `#[primary_span]`",
        )),
        _ => Err(syn::Error::new(
            primary_spans[1].span(),
            "only one field may be marked `#[primary_span]`",
        )),
    }
}

fn expand_struct(input: &syn::DeriveInput, data: &syn::DataStruct) -> syn::Result<TokenStream> {
    let header = parse_header(&input.attrs)?;
    let fields: Vec<Field> = data
        .fields
        .iter()
        .map(parse_field)
        .collect::<Result<_, _>>()?;

    let primary_span = find_primary_span(input, &fields)?;

    // Bind every field by name so the message format string can interpolate
    // them inline (e.g. `"unexpected token `{found}`"`).
    let all_idents = fields.iter().map(|f| &f.ident);

    let builder_calls = fields.iter().filter_map(|f| {
        let field_ident = &f.ident;
        match &f.role {
            Role::Label { msg } => Some(quote!(.label(#field_ident, ::std::format!(#msg)))),
            _ => None,
        }
    });

    let ident = &input.ident;
    let lvl = &header.lvl;
    let msg = &header.msg;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics ::core::convert::From<#ident #ty_generics> for ::skc_diag::Diagnostic #where_clause {
            fn from(value: #ident #ty_generics) -> Self {
                #[allow(unused_variables, unused_mut)]
                let #ident { #(#all_idents),* } = value;
                #lvl(#primary_span, ::std::format!(#msg))
                #(#builder_calls)*
            }
        }
    })
}
