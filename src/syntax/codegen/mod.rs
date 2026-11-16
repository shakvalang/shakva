use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

use either::Either;
use heck::{ToShoutySnakeCase, ToSnakeCase};
use indexmap::{IndexMap, map::Entry};
use quote::{format_ident, quote};

pub fn generate(config: &KindsConfig, src: &str) -> GeneratedSyntaxSources {
    let grammar: ungrammar::Grammar = src.parse().expect("invalid Shakva grammar");
    let kinds = lower_kinds(config, &grammar);
    let ast = lower_ast(&grammar, &kinds);

    GeneratedSyntaxSources {
        kinds_src: generate_kinds(&kinds),
        ast_src: generate_ast(&ast, &grammar, &kinds),
    }
}

#[derive(Debug)]
pub struct GeneratedSyntaxSources {
    pub kinds_src: String,
    pub ast_src: String,
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

fn lower_kinds<'cfg>(config: &'cfg KindsConfig, grammar: &ungrammar::Grammar) -> Kinds<'cfg> {
    let mut entries = vec![];

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

    check_kinds(&entries);
    let grammar_tokens = resolve_grammar_tokens(&entries, grammar);
    let idents = entries
        .iter()
        .map(|kind| format_ident!("{}", kind.name))
        .collect();
    let mut category_ranges = [None; 5];
    for (index, kind) in entries.iter().enumerate() {
        let range = category_ranges[kind.category as usize].get_or_insert((index, index));
        range.1 = index;
    }

    Kinds {
        entries,
        idents,
        category_ranges,
        grammar_tokens,
    }
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

fn append_grammar_nodes(entries: &mut Vec<Kind<'_>>, grammar: &ungrammar::Grammar) {
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

fn check_kinds(entries: &[Kind<'_>]) {
    let mut problems = vec![];
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

    assert!(
        problems.is_empty(),
        "invalid syntax kinds:\n{}",
        problems.join("\n")
    );
}

fn resolve_grammar_tokens(
    entries: &[Kind<'_>],
    grammar: &ungrammar::Grammar,
) -> HashMap<ungrammar::Token, usize> {
    let mut problems = vec![];
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

    assert!(
        problems.is_empty(),
        "invalid grammar tokens:\n{}",
        problems.join("\n")
    );
    resolved
}

fn generate_kinds(kinds: &Kinds<'_>) -> String {
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

    let file = syn::parse2::<syn::File>(tokens).expect("generated kinds must be valid Rust");
    let code = prettyplease::unparse(&file);

    format!("// Generated by `shakva generate syntax`. DO NOT edit.\n\n{code}")
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

fn lower_ast(grammar: &ungrammar::Grammar, kinds: &Kinds<'_>) -> Ast {
    let kind_by_name: HashMap<&str, usize> = kinds
        .entries
        .iter()
        .enumerate()
        .filter(|(_, kind)| kind.category == KindCategory::Node)
        .map(|(index, kind)| (kind.name.as_ref(), index))
        .collect();

    let mut nodes = vec![];
    let mut unions = vec![];

    for node in grammar.iter() {
        let data = &grammar[node];
        let name = format_ident!("{}", data.name);

        if is_node_union(&data.rule) {
            let ungrammar::Rule::Alt(alternatives) = &data.rule else {
                unreachable!();
            };

            let variants = alternatives
                .iter()
                .map(|rule| {
                    let ungrammar::Rule::Node(variant) = rule else {
                        unreachable!();
                    };
                    *variant
                })
                .collect();

            unions.push(UnionDef {
                name,
                variants,
                kind_indices: vec![],
            });
            continue;
        }

        let kind_name = data.name.to_shouty_snake_case();
        let kind_index = *kind_by_name
            .get(kind_name.as_str())
            .unwrap_or_else(|| panic!("no syntax kind for node {}", data.name));

        let mut fields = IndexMap::new();
        lower_rule(&mut fields, grammar, kinds, &data.rule, None);

        nodes.push(NodeDef {
            name,
            kind_index,
            fields: fields.into_values().collect(),
        });
    }

    // Expand nested unions
    // Statement -> Binding, Assignment, Expr -> BinaryExpr, ...
    for union in &mut unions {
        union.kind_indices = resolve_union_kinds(&union.variants, grammar, &kind_by_name);
    }

    Ast { nodes, unions }
}

fn lower_rule(
    fields: &mut IndexMap<proc_macro2::Ident, Field>,
    grammar: &ungrammar::Grammar,
    kinds: &Kinds<'_>,
    rule: &ungrammar::Rule,
    label: Option<&str>,
) {
    if lower_separated_list(fields, grammar, kinds, rule, label) {
        return;
    }

    match rule {
        ungrammar::Rule::Labeled { label: l, rule } => {
            let manually_implemented = matches!(l.as_str(), "lhs" | "rhs" | "op");
            if manually_implemented {
                return;
            }
            lower_rule(fields, grammar, kinds, rule, Some(l))
        }
        ungrammar::Rule::Node(node) => {
            let ty = &grammar[*node].name;
            let name = label.map_or_else(|| ty.to_snake_case().into(), Cow::Borrowed);
            let field = Field {
                name: format_ident!("{name}"),
                target: FieldTarget::Node(*node),
                cardinality: Cardinality::Optional,
            };

            insert_field(fields, field)
        }
        ungrammar::Rule::Token(token) => {
            let index = kinds.grammar_tokens[token];
            let name = label.map_or_else(
                || format!("{}_token", kinds.entries[index].name.to_snake_case()).into(),
                Cow::Borrowed,
            );
            let field = Field {
                name: format_ident!("{name}"),
                target: FieldTarget::Token(index),
                cardinality: Cardinality::Optional,
            };

            insert_field(fields, field)
        }
        ungrammar::Rule::Seq(rules) | ungrammar::Rule::Alt(rules) => {
            for rule in rules {
                lower_rule(fields, grammar, kinds, rule, label);
            }
        }
        ungrammar::Rule::Opt(rule) => lower_rule(fields, grammar, kinds, rule, label),
        ungrammar::Rule::Rep(inner) => {
            if let ungrammar::Rule::Node(node) = &**inner {
                let ty = &grammar[*node].name;
                let name =
                    label.map_or_else(|| pluralize(&ty.to_snake_case()).into(), Cow::Borrowed);
                let field = Field {
                    name: format_ident!("{name}"),
                    target: FieldTarget::Node(*node),
                    cardinality: Cardinality::Many,
                };

                return insert_field(fields, field);
            }

            panic!("unhandled rule: {rule:?}");
        }
    }
}

/// T (separator T)* separator?
fn lower_separated_list(
    fields: &mut IndexMap<proc_macro2::Ident, Field>,
    grammar: &ungrammar::Grammar,
    kinds: &Kinds<'_>,
    rule: &ungrammar::Rule,
    label: Option<&str>,
) -> bool {
    let rule = match rule {
        ungrammar::Rule::Seq(it) => it,
        _ => return false,
    };

    let (nt, repeat, trailing_sep) = match rule.as_slice() {
        [
            ungrammar::Rule::Node(node),
            ungrammar::Rule::Rep(repeat),
            ungrammar::Rule::Opt(trailing_sep),
        ] => (Either::Left(node), repeat, trailing_sep),
        [
            ungrammar::Rule::Token(token),
            ungrammar::Rule::Rep(repeat),
            ungrammar::Rule::Opt(trailing_sep),
        ] => (Either::Right(token), repeat, trailing_sep),
        _ => return false,
    };

    let repeat = match &**repeat {
        ungrammar::Rule::Seq(it) => it,
        _ => return false,
    };

    if !matches!(
        repeat.as_slice(),
        [sep, nt_]
            if &**trailing_sep == sep && match (nt, nt_) {
                (Either::Left(node), ungrammar::Rule::Node(node_)) => node == node_,
                (Either::Right(token), ungrammar::Rule::Token(token_)) => token == token_,
                _ => false
            }
    ) {
        return false;
    }

    match nt {
        Either::Left(node) => {
            let ty = &grammar[*node].name;
            let name = label.map_or_else(|| pluralize(&ty.to_snake_case()).into(), Cow::Borrowed);
            let field = Field {
                name: format_ident!("{name}"),
                target: FieldTarget::Node(*node),
                cardinality: Cardinality::Many,
            };

            insert_field(fields, field);
        }
        Either::Right(token) => {
            let index = kinds.grammar_tokens[token];
            let name = label.map_or_else(
                || pluralize(&kinds.entries[index].name.to_snake_case()).into(),
                Cow::Borrowed,
            );
            let field = Field {
                name: format_ident!("{name}"),
                target: FieldTarget::Token(index),
                cardinality: Cardinality::Many,
            };

            insert_field(fields, field);
        }
    }

    true
}

fn insert_field(fields: &mut IndexMap<proc_macro2::Ident, Field>, field: Field) {
    match fields.entry(field.name.clone()) {
        Entry::Vacant(entry) => {
            entry.insert(field);
        }
        Entry::Occupied(entry) => {
            assert_eq!(
                entry.get(),
                &field,
                "conflicting fields named {}",
                entry.key(),
            );
        }
    }
}

fn pluralize(s: &str) -> String {
    format!("{s}s")
}

fn resolve_union_kinds(
    variants: &[ungrammar::Node],
    grammar: &ungrammar::Grammar,
    kind_by_name: &HashMap<&str, usize>,
) -> Vec<usize> {
    todo!()
}

fn generate_ast(ast: &Ast, grammar: &ungrammar::Grammar, kinds: &Kinds<'_>) -> String {
    todo!()
}

#[derive(Debug)]
struct Ast {
    nodes: Vec<NodeDef>,
    unions: Vec<UnionDef>,
}

#[derive(Debug)]
struct NodeDef {
    name: proc_macro2::Ident,
    kind_index: usize,
    fields: Vec<Field>,
}

#[derive(Debug, PartialEq, Eq)]
struct Field {
    name: proc_macro2::Ident,
    target: FieldTarget,
    cardinality: Cardinality,
}

#[derive(Debug, PartialEq, Eq)]
enum FieldTarget {
    Node(ungrammar::Node),
    Token(usize),
}

#[derive(Debug, PartialEq, Eq)]
enum Cardinality {
    Optional,
    Many,
}

#[derive(Debug)]
struct UnionDef {
    name: proc_macro2::Ident,
    variants: Vec<ungrammar::Node>,
    kind_indices: Vec<usize>,
}
