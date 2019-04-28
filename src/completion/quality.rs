use crate::feature::FeatureRequest;
use crate::range;
use crate::syntax::bibtex::ast::{BibtexToken, BibtexVisitor};
use crate::syntax::bibtex::finder::{BibtexFinder, BibtexNode};
use crate::syntax::latex::analysis::finder::{LatexCommandFinder, LatexFinder, LatexNode};
use crate::syntax::latex::ast::{LatexToken, LatexVisitor};
use crate::syntax::text::SyntaxNode;
use crate::workspace::{Document, SyntaxTree};
use lsp_types::{CompletionItem, CompletionParams, Position};

pub struct OrderByQualityCompletionProvider;

impl OrderByQualityCompletionProvider {
    pub async fn execute<'a, E, F>(
        request: &'a FeatureRequest<CompletionParams>,
        execute: E,
    ) -> Vec<CompletionItem>
    where
        E: Fn(&'a FeatureRequest<CompletionParams>) -> F,
        F: std::future::Future<Output = Vec<CompletionItem>>,
    {
        let query = Self::get_query(&request.document, request.params.position);
        let mut items = await!(execute(&request));
        items.sort_by_key(|item| -Self::get_quality(query, &item.label));
        items
    }

    fn get_query(document: &Document, position: Position) -> Option<&str> {
        match &document.tree {
            SyntaxTree::Latex(tree) => {
                let mut command_finder = LatexCommandFinder::new(position);
                command_finder.visit_root(&tree.root);
                let mut node = command_finder.result.map(LatexNode::Command).or_else(|| {
                    let mut finder = LatexFinder::new(position);
                    finder.visit_root(&tree.root);
                    finder.results.into_iter().last()
                })?;

                match node {
                    LatexNode::Root(_) | LatexNode::Group(_) => Some(""),
                    LatexNode::Command(command) => Some(&command.name.text()[1..]),
                    LatexNode::Text(text) => text.words.last().map(LatexToken::text),
                }
            }
            SyntaxTree::Bibtex(tree) => {
                fn get_kind_query(kind: &BibtexToken, position: Position) -> Option<&str> {
                    if range::contains(kind.range(), position) {
                        Some(&kind.text()[1..])
                    } else {
                        Some("")
                    }
                }
                let mut finder = BibtexFinder::new(position);
                finder.visit_root(&tree.root);
                match finder.results.pop()? {
                    BibtexNode::Root(_) => Some(""),
                    BibtexNode::Preamble(preamble) => get_kind_query(&preamble.kind, position),
                    BibtexNode::String(string) => get_kind_query(&string.kind, position),
                    BibtexNode::Entry(entry) => get_kind_query(&entry.kind, position),
                    BibtexNode::Comment(comment) => Some(comment.token.text()),
                    BibtexNode::Field(field) => {
                        if range::contains(field.name.range(), position) {
                            Some(field.name.text())
                        } else {
                            Some("")
                        }
                    }
                    BibtexNode::Word(word) => Some(word.token.text()),
                    BibtexNode::Command(command) => Some(&command.token.text()[1..]),
                    BibtexNode::QuotedContent(_)
                    | BibtexNode::BracedContent(_)
                    | BibtexNode::Concat(_) => Some(""),
                }
            }
        }
    }

    fn get_quality(query: Option<&str>, label: &str) -> i32 {
        if let Some(query) = query {
            if label == query {
                return 7;
            }

            if label.to_lowercase() == query.to_lowercase() {
                return 6;
            }

            if label.starts_with(query) {
                return 5;
            }

            if label.to_lowercase().starts_with(&query.to_lowercase()) {
                return 4;
            }

            if label.contains(query) {
                return 3;
            }

            if label.to_lowercase().contains(&query.to_lowercase()) {
                return 2;
            }

            return 1;
        } else {
            return 0;
        }
    }
}