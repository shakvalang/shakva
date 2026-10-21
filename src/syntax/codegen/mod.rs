use std::{borrow::Cow, collections::HashSet};

use heck::ToShoutySnakeCase;
use quote::{format_ident, quote};
use ungrammar::Grammar;

pub fn generate(config: &KindsConfig, src: &str) -> Result<GeneratedSyntaxSources, Error> {
    let grammar: Grammar = src.parse()?;
    let kinds = lower_kinds(config, &grammar)?;

    let kinds_src = generate_kinds(&kinds)?;

    Ok(GeneratedSyntaxSources { kinds_src })
}

fn lower_kinds<'cfg>(config: &'cfg KindsConfig, grammar: &Grammar) -> Result<Kinds<'cfg>, Error> {
    let mut entries = Vec::new();

    append_config_kinds(&mut entries, &config.trivia, KindCategory::Trivia);
    append_config_kinds(&mut entries, &config.punct, KindCategory::Punct);
    append_config_kinds(&mut entries, &config.tokens, KindCategory::Token);
    append_config_kinds(
        &mut entries,
        &config.special_tokens,
        KindCategory::SpecialToken,
    );
    append_config_kinds(&mut entries, &config.extra_nodes, KindCategory::Node);

    append_grammar_nodes(&mut entries, grammar);

    check_kinds(&entries)?;
    check_grammar_tokens(&entries, grammar)?;

    Ok(Kinds { entries })
}

fn append_config_kinds<'cfg>(
    entries: &mut Vec<Kind<'cfg>>,
    configs: &'cfg [KindConfig],
    category: KindCategory,
) {
    for config in configs {
        entries.push(Kind {
            name: Cow::Borrowed(&config.name),
            category,
            doc: config.doc.as_deref(),
            text: config.text.as_deref(),
        });
    }
}

fn append_grammar_nodes(entries: &mut Vec<Kind<'_>>, grammar: &Grammar) {
    for node in grammar.iter() {
        let data = &grammar[node];
        if is_node_union(&data.rule) {
            continue;
        }

        entries.push(Kind {
            name: Cow::Owned(data.name.to_shouty_snake_case()),
            category: KindCategory::Node,
            doc: None,
            text: None,
        });
    }
}

fn is_node_union(rule: &ungrammar::Rule) -> bool {
    match rule {
        ungrammar::Rule::Alt(alternatives) => {
            !alternatives.is_empty()
                && alternatives
                    .iter()
                    .all(|rule| matches!(rule, ungrammar::Rule::Node(_)))
        }
        _ => false,
    }
}

fn check_kinds(entries: &[Kind<'_>]) -> Result<(), Error> {
    let mut problems = Vec::new();
    let mut names = HashSet::new();
    let mut texts = HashSet::new();

    if entries.is_empty() {
        problems.push("the list must not be empty".into());
    }

    let max_count = usize::from(u16::MAX) + 1;
    if entries.len() > max_count {
        problems.push(format!(
            "{} kinds exceed the u16 capacity of {max_count}",
            entries.len(),
        ));
    }

    for kind in entries {
        let mut chars = kind.name.chars();
        let valid_name = chars.next().is_some_and(|c| c.is_ascii_uppercase())
            && chars.all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_');

        if !valid_name {
            problems.push(format!(
                "invalid kind name {:?}; expected [A-Z][A-Z0-9_]*",
                kind.name,
            ));
        }

        if !names.insert(kind.name.as_ref()) {
            problems.push(format!("duplicate kind name: {}", kind.name));
        }

        match kind.category {
            KindCategory::Punct if kind.text.is_none() => {
                problems.push(format!("punctuation kind {} requires text", kind.name,));
            }
            KindCategory::Node if kind.text.is_some() => {
                problems.push(format!("node kind {} must not have text", kind.name,));
            }
            _ => {}
        }

        if let Some(text) = &kind.text {
            if text.is_empty() {
                problems.push(format!("kind {} has empty text", kind.name));
            }

            if !matches!(kind.category, KindCategory::Node) && !texts.insert(text) {
                problems.push(format!("duplicate token text: {text:?}"));
            }
        }
    }

    if problems.is_empty() {
        Ok(())
    } else {
        Err(Error::InvalidKinds(problems))
    }
}

fn check_grammar_tokens(entries: &[Kind<'_>], grammar: &Grammar) -> Result<(), Error> {
    let mut problems = Vec::new();

    for token in grammar.tokens() {
        let name = &grammar[token].name;

        let is_named =
            !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');

        let kind_name = is_named.then(|| name.to_shouty_snake_case());

        let mut candidates = entries
            .iter()
            .filter(|kind| !matches!(kind.category, KindCategory::Node))
            .filter(|kind| match &kind.text {
                Some(text) => text == name,
                None => kind_name.as_deref() == Some(kind.name.as_ref()),
            })
            .map(|kind| kind.name.as_ref());

        match (candidates.next(), candidates.next()) {
            (None, _) => {
                problems.push(format!("no configured kind for grammar token {name:?}",));
            }
            (Some(_), None) => {}
            (Some(first), Some(second)) => {
                let mut matches = vec![first, second];
                matches.extend(candidates);

                problems.push(format!(
                    "ambiguous grammar token {name:?}: {}",
                    matches.join(", "),
                ));
            }
        }
    }

    if problems.is_empty() {
        Ok(())
    } else {
        Err(Error::InvalidGrammarTokens(problems))
    }
}

fn generate_kinds(kinds: &Kinds<'_>) -> Result<String, Error> {
    let variants = kinds.entries.iter().map(|kind| {
        let name = format_ident!("{}", kind.name);
        let docs = kind.doc.iter().flat_map(|text| {
            text.trim().lines().map(|line| {
                let line = if line.is_empty() {
                    String::new()
                } else {
                    format!(" {line}")
                };

                quote! { #[doc = #line] }
            })
        });

        quote! {
            #(#docs)*
            #name,
        }
    });

    let trivia_variants = kinds
        .entries
        .iter()
        .filter(|kind| matches!(kind.category, KindCategory::Trivia))
        .map(|kind| format_ident!("{}", kind.name));

    let node_variants = kinds
        .entries
        .iter()
        .filter(|kind| matches!(kind.category, KindCategory::Node))
        .map(|kind| format_ident!("{}", kind.name));

    let punct_variants = kinds
        .entries
        .iter()
        .filter(|kind| matches!(kind.category, KindCategory::Punct))
        .map(|kind| format_ident!("{}", kind.name));

    let text_arms = kinds.entries.iter().filter_map(|kind| {
        let text = kind.text?;
        let variant = format_ident!("{}", kind.name);

        Some(quote! {
            Self::#variant => Some(#text),
        })
    });

    let last = kinds.entries.last().expect("kinds must not be empty");
    let last_name = format_ident!("{}", last.name);
    let tokens = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[allow(non_camel_case_types)]
        #[repr(u16)]
        pub enum ShakvaKind {
            #(#variants)*
        }

        impl ShakvaKind {
            pub fn is_trivia(self) -> bool {
                match self {
                    #(Self::#trivia_variants => true,)*
                    _ => false,
                }
            }

            pub fn is_node(self) -> bool {
                match self {
                    #(Self::#node_variants => true,)*
                    _ => false,
                }
            }

            pub fn is_token(self) -> bool {
                !self.is_node()
            }

            pub fn is_punct(self) -> bool {
                match self {
                    #(Self::#punct_variants => true,)*
                    _ => false,
                }
            }

            pub fn fixed_text(self) -> Option<&'static str> {
                match self {
                    #(#text_arms)*
                    _ => None,
                }
            }

            pub(crate) fn from_raw(raw: rowan::SyntaxKind) -> Self {
                assert!(
                    raw.0 <= Self::#last_name as u16,
                    "invalid syntax kind: {}",
                    raw.0,
                );

                unsafe { std::mem::transmute::<u16, Self>(raw.0) }
            }
        }

        impl From<ShakvaKind> for rowan::SyntaxKind {
            fn from(kind: ShakvaKind) -> Self {
                Self(kind as u16)
            }
        }
    };

    let file = syn::parse2::<syn::File>(tokens)?;
    let code = prettyplease::unparse(&file);

    Ok(format!(
        "// Generated by `shakva generate syntax`. DO NOT edit.\n\n{code}"
    ))
}

#[derive(Debug)]
struct Kinds<'cfg> {
    entries: Vec<Kind<'cfg>>,
}

#[derive(Debug)]
struct Kind<'cfg> {
    name: Cow<'cfg, str>,
    category: KindCategory,
    doc: Option<&'cfg str>,
    text: Option<&'cfg str>,
}

#[derive(Debug, Clone, Copy)]
enum KindCategory {
    Trivia,
    Punct,
    Token,
    SpecialToken,
    Node,
}

#[derive(Debug)]
pub struct GeneratedSyntaxSources {
    pub kinds_src: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to parse grammar")]
    GrammarParseFailed(#[from] ungrammar::Error),
    #[error("failed to generate source file from tokens")]
    GenerateSourceFileFailed(#[from] syn::Error),
    #[error("invalid grammar tokens:\n{}", .0.join("\n"))]
    InvalidGrammarTokens(Vec<String>),
    #[error("invalid syntax kinds:\n{}", .0.join("\n"))]
    InvalidKinds(Vec<String>),
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct KindsConfig {
    trivia: Vec<KindConfig>,
    punct: Vec<KindConfig>,
    tokens: Vec<KindConfig>,
    special_tokens: Vec<KindConfig>,
    extra_nodes: Vec<KindConfig>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KindConfig {
    name: String,
    doc: Option<String>,
    text: Option<String>,
}
