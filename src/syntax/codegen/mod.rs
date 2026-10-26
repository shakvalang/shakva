use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

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
    let grammar_tokens = resolve_grammar_tokens(&entries, grammar)?;
    let idents = entries
        .iter()
        .map(|kind| format_ident!("{}", kind.name))
        .collect();
    let mut category_ranges = [None; 5];
    for (index, kind) in entries.iter().enumerate() {
        let range = category_ranges[kind.category as usize].get_or_insert((index, index));
        range.1 = index;
    }

    Ok(Kinds {
        entries,
        idents,
        category_ranges,
        grammar_tokens,
    })
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
    let mut categories = HashSet::new();
    let mut previous_category = None;

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
        if previous_category != Some(kind.category) {
            if !categories.insert(kind.category) {
                problems.push(format!("category {:?} must be contiguous", kind.category));
            }
            previous_category = Some(kind.category);
        }
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

        if let Some(text) = kind.text {
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

fn resolve_grammar_tokens(
    entries: &[Kind<'_>],
    grammar: &Grammar,
) -> Result<HashMap<ungrammar::Token, usize>, Error> {
    let mut problems = Vec::new();
    let mut by_text = HashMap::new();
    let mut named_without_text = HashMap::new();
    for (index, kind) in entries.iter().enumerate() {
        if kind.category == KindCategory::Node {
            continue;
        }
        match kind.text {
            Some(text) => {
                by_text.insert(text, index);
            }
            None => {
                named_without_text.insert(kind.name.as_ref(), index);
            }
        }
    }
    let mut resolved = HashMap::new();

    for token in grammar.tokens() {
        let name = &grammar[token].name;

        let is_named =
            !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');

        let kind_name = is_named.then(|| name.to_shouty_snake_case());

        let text_match = by_text.get(name.as_str()).copied();
        let named_match = kind_name
            .as_deref()
            .and_then(|name| named_without_text.get(name))
            .copied();

        match (text_match, named_match) {
            (None, None) => {
                problems.push(format!("no configured kind for grammar token {name:?}",));
            }
            (Some(index), None) | (None, Some(index)) => {
                resolved.insert(token, index);
            }
            (Some(first), Some(second)) => {
                // Preserve configuration order in diagnostics.
                let (first, second) = (first.min(second), first.max(second));
                problems.push(format!(
                    "ambiguous grammar token {name:?}: {}, {}",
                    entries[first].name, entries[second].name,
                ));
            }
        }
    }

    if problems.is_empty() {
        Ok(resolved)
    } else {
        Err(Error::InvalidGrammarTokens(problems))
    }
}

fn generate_kinds(kinds: &Kinds<'_>) -> Result<String, Error> {
    let variants = kinds.entries.iter().zip(&kinds.idents).map(|(kind, name)| {
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

    let trivia_check = generate_category_check(kinds, KindCategory::Trivia);
    let node_check = generate_category_check(kinds, KindCategory::Node);
    let punct_check = generate_category_check(kinds, KindCategory::Punct);

    let text_arms = kinds
        .entries
        .iter()
        .zip(&kinds.idents)
        .filter_map(|(kind, variant)| {
            let text = kind.text?;

            Some(quote! {
                Self::#variant => Some(#text),
            })
        });

    let last_name = kinds.idents.last().expect("kinds must not be empty");
    let tokens = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[allow(non_camel_case_types)]
        #[repr(u16)]
        pub enum ShakvaKind {
            #(#variants)*
        }

        impl ShakvaKind {
            pub fn is_trivia(self) -> bool {
                #trivia_check
            }

            pub fn is_node(self) -> bool {
                #node_check
            }

            pub fn is_token(self) -> bool {
                !self.is_node()
            }

            pub fn is_punct(self) -> bool {
                #punct_check
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

                // SAFETY: variants are contiguous from zero and the upper bound was checked.
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

fn generate_category_check(kinds: &Kinds<'_>, category: KindCategory) -> proc_macro2::TokenStream {
    let Some((first, last)) = kinds.category_ranges[category as usize] else {
        return quote! { false };
    };
    let first_name = &kinds.idents[first];
    let last_name = &kinds.idents[last];
    // Omit bounds already guaranteed by the enum's valid discriminants.
    match (first == 0, last == kinds.entries.len() - 1) {
        (true, true) => quote! { true },
        (true, false) => quote! { (self as u16) <= Self::#last_name as u16 },
        (false, true) => quote! { (self as u16) >= Self::#first_name as u16 },
        (false, false) => quote! {
            (self as u16) >= Self::#first_name as u16
                && (self as u16) <= Self::#last_name as u16
        },
    }
}

#[derive(Debug)]
struct Kinds<'cfg> {
    entries: Vec<Kind<'cfg>>,
    idents: Vec<proc_macro2::Ident>,
    category_ranges: [Option<(usize, usize)>; 5],
    #[allow(dead_code)]
    grammar_tokens: HashMap<ungrammar::Token, usize>,
}

#[derive(Debug)]
struct Kind<'cfg> {
    name: Cow<'cfg, str>,
    category: KindCategory,
    doc: Option<&'cfg str>,
    text: Option<&'cfg str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum KindCategory {
    Trivia = 0,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_config() -> KindsConfig {
        KindsConfig {
            trivia: vec![],
            punct: vec![],
            tokens: vec![],
            special_tokens: vec![],
            extra_nodes: vec![],
        }
    }

    fn token(name: &str, text: Option<&str>) -> KindConfig {
        KindConfig {
            name: name.into(),
            doc: None,
            text: text.map(str::to_owned),
        }
    }

    #[test]
    fn resolves_fixed_and_named_tokens() {
        // arrange
        let mut config = empty_config();
        config.punct.push(token("PLUS", Some("+")));
        config.tokens.push(token("INT_LIT", None));
        let grammar: Grammar = "Expr = 'IntLit' '+' 'IntLit'".parse().unwrap();

        // act
        let kinds = lower_kinds(&config, &grammar).unwrap();

        // assert
        for token in grammar.tokens() {
            let index = kinds.grammar_tokens[&token];
            let expected = match grammar[token].name.as_str() {
                "+" => "PLUS",
                "IntLit" => "INT_LIT",
                other => panic!("unexpected token: {other}"),
            };
            assert_eq!(kinds.entries[index].name, expected);
            assert_eq!(kinds.idents[index].to_string(), expected);
        }
    }

    #[test]
    fn reports_ambiguous_and_missing_tokens_together() {
        // arrange
        let mut config = empty_config();
        config.tokens = vec![token("FOO", None), token("ALIAS", Some("Foo"))];
        let grammar: Grammar = "Expr = 'Foo' 'Missing'".parse().unwrap();

        // act
        let result = lower_kinds(&config, &grammar);

        // assert
        let Error::InvalidGrammarTokens(problems) = result.unwrap_err() else {
            panic!("expected grammar token errors");
        };
        assert_eq!(
            problems,
            [
                "ambiguous grammar token \"Foo\": FOO, ALIAS",
                "no configured kind for grammar token \"Missing\"",
            ]
        );
    }

    #[test]
    fn rejects_name_match_for_kind_with_different_fixed_text() {
        // arrange
        let mut config = empty_config();
        config.tokens.push(token("INT_LIT", Some("integer")));
        let grammar = "Expr = 'IntLit'";

        // act
        let result = generate(&config, grammar);

        // assert
        let Error::InvalidGrammarTokens(problems) = result.unwrap_err() else {
            panic!("a token with fixed text must only match that text");
        };
        assert_eq!(
            problems,
            ["no configured kind for grammar token \"IntLit\""]
        );
    }

    #[test]
    fn rejects_interleaved_categories() {
        // arrange
        let entries = [
            Kind {
                name: "FIRST".into(),
                category: KindCategory::Trivia,
                doc: None,
                text: None,
            },
            Kind {
                name: "SECOND".into(),
                category: KindCategory::Node,
                doc: None,
                text: None,
            },
            Kind {
                name: "THIRD".into(),
                category: KindCategory::Trivia,
                doc: None,
                text: None,
            },
        ];

        // act
        let result = check_kinds(&entries);

        // assert
        let Error::InvalidKinds(problems) = result.unwrap_err() else {
            panic!("expected category ordering error");
        };
        assert_eq!(problems, ["category Trivia must be contiguous"]);
    }

    #[test]
    fn computes_empty_token_ranges_and_singleton_node_range() {
        // arrange
        let config = empty_config();
        let grammar: Grammar = "Expr = Expr?".parse().unwrap();

        // act
        let kinds = lower_kinds(&config, &grammar).unwrap();
        let generated = generate_kinds(&kinds);

        // assert
        assert_eq!(kinds.category_ranges[KindCategory::Trivia as usize], None);
        assert_eq!(kinds.category_ranges[KindCategory::Punct as usize], None);
        assert_eq!(
            kinds.category_ranges[KindCategory::Node as usize],
            Some((0, 0))
        );
        generated.unwrap();
    }

    #[test]
    fn classifies_project_kinds_and_text_according_to_config() {
        // arrange
        let config: toml::Value =
            toml::from_str(include_str!("../../../syntax_codegen.toml")).unwrap();
        let config: KindsConfig = config["kinds"].clone().try_into().unwrap();
        let grammar: Grammar = include_str!("../shakva.ungram").parse().unwrap();
        let kinds = lower_kinds(&config, &grammar).unwrap();
        let expected_and_actual = kinds.entries.iter().enumerate().map(|(index, entry)| {
            let kind = crate::syntax::ShakvaKind::from_raw(rowan::SyntaxKind(index as u16));
            (entry, kind)
        });

        // act
        let classifications: Vec<_> = expected_and_actual
            .map(|(entry, kind)| {
                (
                    entry,
                    kind.is_trivia(),
                    kind.is_punct(),
                    kind.is_node(),
                    kind.is_token(),
                    kind.fixed_text(),
                )
            })
            .collect();

        // assert
        for (entry, is_trivia, is_punct, is_node, is_token, fixed_text) in classifications {
            assert_eq!(
                is_trivia,
                entry.category == KindCategory::Trivia,
                "{}",
                entry.name
            );
            assert_eq!(
                is_punct,
                entry.category == KindCategory::Punct,
                "{}",
                entry.name
            );
            assert_eq!(
                is_node,
                entry.category == KindCategory::Node,
                "{}",
                entry.name
            );
            assert_eq!(
                is_token,
                entry.category != KindCategory::Node,
                "{}",
                entry.name
            );
            assert_eq!(fixed_text, entry.text, "{}", entry.name);
        }
    }

    // Expand cases into separate tests so cargo reports each scenario by name.
    macro_rules! token_resolution_cases {
        ($($name:ident: ($kind:literal, $text:expr, $grammar:literal)),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    // arrange
                    let mut config = empty_config();
                    config.tokens.push(token($kind, $text));
                    let grammar: Grammar = $grammar.parse().unwrap();
                    let grammar_token = grammar.tokens().next().unwrap();

                    // act
                    let kinds = lower_kinds(&config, &grammar).unwrap();

                    // assert
                    let index = kinds.grammar_tokens[&grammar_token];
                    assert_eq!(kinds.entries[index].name, $kind);
                }
            )*
        };
    }

    token_resolution_cases! {
        resolves_punctuation_by_fixed_text:
            ("PLUS", Some("+"), "Expr = '+'"),
        resolves_keyword_alias_by_fixed_text:
            ("LET_KW", Some("let"), "Expr = 'let'"),
        resolves_unicode_fixed_text:
            ("LAMBDA", Some("λ"), "Expr = 'λ'"),
        resolves_camel_case_token_by_normalized_name:
            ("INT_LIT", None, "Expr = 'IntLit'"),
        resolves_uppercase_token_by_name:
            ("INT_LIT", None, "Expr = 'INT_LIT'"),
    }

    #[test]
    fn reports_ambiguity_in_config_order() {
        for alias_first in [false, true] {
            // arrange
            let mut config = empty_config();
            config.tokens = if alias_first {
                vec![token("ALIAS", Some("Foo")), token("FOO", None)]
            } else {
                vec![token("FOO", None), token("ALIAS", Some("Foo"))]
            };
            let expected = if alias_first {
                "ambiguous grammar token \"Foo\": ALIAS, FOO"
            } else {
                "ambiguous grammar token \"Foo\": FOO, ALIAS"
            };

            // act
            let result = generate(&config, "Expr = 'Foo'");

            // assert
            let Error::InvalidGrammarTokens(problems) = result.unwrap_err() else {
                panic!("alias_first={alias_first}: expected ambiguity");
            };
            assert_eq!(problems, [expected], "alias_first={alias_first}");
        }
    }

    #[test]
    fn computes_exact_boundaries_for_varied_category_sizes() {
        let cases = [
            (
                "only_node",
                [0, 0, 0],
                [None, None, None, None, Some((0, 0))],
            ),
            (
                "singleton_groups",
                [1, 1, 0],
                [Some((0, 0)), Some((1, 1)), None, None, Some((2, 2))],
            ),
            (
                "multiple_groups",
                [2, 3, 2],
                [Some((0, 1)), Some((2, 4)), None, None, Some((5, 7))],
            ),
        ];

        for (case_name, [trivia_count, punct_count, extra_node_count], expected) in cases {
            // arrange
            let mut config = empty_config();
            config.trivia = (0..trivia_count)
                .map(|index| token(&format!("TRIVIA_{index}"), None))
                .collect();
            config.punct = (0..punct_count)
                .map(|index| token(&format!("PUNCT_{index}"), Some(&format!("punct_{index}"))))
                .collect();
            config.extra_nodes = (0..extra_node_count)
                .map(|index| token(&format!("EXTRA_NODE_{index}"), None))
                .collect();
            let grammar: Grammar = "Expr = Expr?".parse().unwrap();

            // act
            let kinds = lower_kinds(&config, &grammar).unwrap();

            // assert
            assert_eq!(kinds.category_ranges, expected, "case={case_name}");
        }
    }

    #[test]
    fn matches_linear_lookup_for_all_alias_combinations() {
        // Each token can have no kind, a named kind, a fixed-text alias, or both.
        // Exhaust all 4^3 configurations without random seeds.
        for configuration in 0..64 {
            // arrange
            let mut config = empty_config();
            for index in 0..3 {
                let mode = (configuration >> (index * 2)) & 3;
                if mode & 1 != 0 {
                    config.tokens.push(token(&format!("T{index}"), None));
                }
                if mode & 2 != 0 {
                    config
                        .tokens
                        .push(token(&format!("ALIAS{index}"), Some(&format!("T{index}"))));
                }
            }
            let grammar: Grammar = "Expr = 'T0' 'T1' 'T2'".parse().unwrap();
            let mut entries = Vec::new();
            append_config_kinds(&mut entries, &config.tokens, KindCategory::Token);
            let mut expected_tokens = HashMap::new();
            let mut expected_problems = Vec::new();
            // Independent reference: scan all kinds instead of building indexes.
            for token in grammar.tokens() {
                let name = &grammar[token].name;
                let candidates: Vec<_> = entries
                    .iter()
                    .enumerate()
                    .filter(|(_, kind)| match kind.text {
                        Some(text) => text == name,
                        None => kind.name == name.as_str(),
                    })
                    .collect();
                match candidates.as_slice() {
                    [] => expected_problems
                        .push(format!("no configured kind for grammar token {name:?}")),
                    [(index, _)] => {
                        expected_tokens.insert(token, *index);
                    }
                    _ => expected_problems.push(format!(
                        "ambiguous grammar token {name:?}: {}",
                        candidates
                            .iter()
                            .map(|(_, kind)| kind.name.as_ref())
                            .collect::<Vec<_>>()
                            .join(", "),
                    )),
                }
            }

            // act
            let result = resolve_grammar_tokens(&entries, &grammar);

            // assert
            if expected_problems.is_empty() {
                assert_eq!(
                    result.unwrap(),
                    expected_tokens,
                    "configuration={configuration}"
                );
            } else {
                let Error::InvalidGrammarTokens(problems) = result.unwrap_err() else {
                    panic!("configuration={configuration}: expected token resolution errors");
                };
                assert_eq!(problems, expected_problems, "configuration={configuration}");
            }
        }
    }
}
