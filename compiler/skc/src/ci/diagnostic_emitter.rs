use std::{ops::Range, sync::Arc};

use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet, renderer::DecorStyle};
use skc_diag::{Diagnostic, DiagnosticEmitter, DiagnosticLevel};
use skc_ir::source::{SourceFile, Span};

use crate::ci::SourceMap;

#[derive(Debug)]
pub struct HumanReadableDiagnosticEmitter<'a> {
    sm: &'a SourceMap,
    renderer: Renderer,
}

impl<'a> HumanReadableDiagnosticEmitter<'a> {
    pub fn new(sm: &'a SourceMap) -> Self {
        Self {
            sm,
            renderer: Renderer::styled().decor_style(DecorStyle::Unicode),
        }
    }

    /// Render a single diagnostic to a string (without trailing newline).
    fn render(&self, diag: &Diagnostic) -> String {
        // One snippet per file.
        let mut files: Vec<FileSnippet> = Vec::new();

        // Primary span first so its file leads the report.
        self.push_annotation(
            &mut files,
            diag.primary_span,
            AnnotationKind::Primary,
            // TODO: Consider primary span label?
            String::new(),
        );

        for label in &diag.labels {
            self.push_annotation(
                &mut files,
                label.span,
                AnnotationKind::Context,
                label.msg.clone(),
            );
        }

        let mut group = Group::with_title(level_of(diag.lvl).primary_title(&diag.msg));

        for file in &files {
            let mut snippet = Snippet::source(&file.source.contents)
                .path(file.source.path.to_string_lossy())
                .line_start(1);

            for ann in &file.annotations {
                snippet = snippet.annotation(ann.kind.span(ann.range.clone()).label(&ann.label));
            }

            group = group.element(snippet);
        }

        self.renderer.render(&[group]).to_string()
    }

    fn push_annotation(
        &self,
        files: &mut Vec<FileSnippet>,
        span: Span,
        kind: AnnotationKind,
        label: String,
    ) {
        let Some((source, range)) = self.sm.resolve_span(span) else {
            return;
        };

        let anno = Annotation { kind, range, label };

        // Linear search is ok here, because files count per diagnostic is
        // small.
        if let Some(existing) = files.iter_mut().find(|f| f.lo == source.span.lo.to_u32()) {
            existing.annotations.push(anno);
        } else {
            files.push(FileSnippet {
                lo: source.span.lo.to_u32(),
                annotations: vec![anno],
                source,
            });
        }
    }
}

impl DiagnosticEmitter for HumanReadableDiagnosticEmitter<'_> {
    fn emit(&self, diag: &Diagnostic) {
        eprintln!("{}", self.render(diag));
    }
}

#[derive(Debug)]
struct FileSnippet {
    /// Global start offset of the file, used to dedup spans within one file.
    lo: u32,
    source: Arc<SourceFile>,
    annotations: Vec<Annotation>,
}

#[derive(Debug)]
struct Annotation {
    kind: AnnotationKind,
    range: Range<usize>,
    label: String,
}

fn level_of(lvl: DiagnosticLevel) -> Level<'static> {
    match lvl {
        DiagnosticLevel::Error => Level::ERROR,
        DiagnosticLevel::Warning => Level::WARNING,
        DiagnosticLevel::Note => Level::NOTE,
    }
}
