use cancellation::CancellationToken;
use lsp_types::CompletionParams;

use crate::{
    features::cursor::CursorContext,
    syntax::{latex, CstNode},
};

use super::types::{InternalCompletionItem, InternalCompletionItemData};

pub fn complete_acronyms<'a>(
    context: &'a CursorContext<CompletionParams>,
    items: &mut Vec<InternalCompletionItem<'a>>,
    cancellation_token: &CancellationToken,
) -> Option<()> {
    cancellation_token.result().ok()?;

    let (_, range, group) = context.find_curly_group_word()?;
    latex::AcronymReference::cast(group.syntax().parent()?)?;

    for document in &context.request.subset.documents {
        if let Some(data) = document.data.as_latex() {
            for name in data
                .root
                .descendants()
                .filter_map(latex::AcronymDefinition::cast)
                .filter_map(|node| node.name())
                .filter_map(|name| name.key())
                .map(|name| name.to_string())
            {
                items.push(InternalCompletionItem::new(
                    range,
                    InternalCompletionItemData::Acronym { name },
                ));
            }
        }
    }
    Some(())
}

#[cfg(test)]
mod tests {
    use cstree::TextRange;

    use crate::features::testing::FeatureTester;

    use super::*;

    #[test]
    fn test_empty_latex_document() {
        let request = FeatureTester::builder()
            .files(vec![("main.tex", "")])
            .main("main.tex")
            .line(0)
            .character(0)
            .build()
            .completion();

        let context = CursorContext::new(request);
        let mut actual_items = Vec::new();
        complete_acronyms(&context, &mut actual_items, CancellationToken::none());

        assert!(actual_items.is_empty());
    }

    #[test]
    fn test_empty_bibtex_document() {
        let request = FeatureTester::builder()
            .files(vec![("main.bib", "")])
            .main("main.bib")
            .line(0)
            .character(0)
            .build()
            .completion();

        let context = CursorContext::new(request);
        let mut actual_items = Vec::new();
        complete_acronyms(&context, &mut actual_items, CancellationToken::none());

        assert!(actual_items.is_empty());
    }

    #[test]
    fn test_simple() {
        let request = FeatureTester::builder()
            .files(vec![("main.tex", "\\newacronym[longplural={Frames per Second}]{fpsLabel}{FPS}{Frame per Second}\n\\acrshort{f}")])
            .main("main.tex")
            .line(1)
            .character(11)
            .build()
            .completion();

        let context = CursorContext::new(request);
        let mut actual_items = Vec::new();
        complete_acronyms(&context, &mut actual_items, CancellationToken::none());

        assert!(!actual_items.is_empty());
        for item in actual_items {
            assert_eq!(item.range, TextRange::new(87.into(), 88.into()));
        }
    }

    #[test]
    fn test_empty() {
        let request = FeatureTester::builder()
            .files(vec![("main.tex", "\\newacronym[longplural={Frames per Second}]{fpsLabel}{FPS}{Frame per Second}\n\\acrshort{}")])
            .main("main.tex")
            .line(1)
            .character(10)
            .build()
            .completion();

        let context = CursorContext::new(request);
        let mut actual_items = Vec::new();
        complete_acronyms(&context, &mut actual_items, CancellationToken::none());

        assert!(!actual_items.is_empty());
        for item in actual_items {
            assert_eq!(item.range, TextRange::new(87.into(), 87.into()));
        }
    }

    #[test]
    fn test_after_group() {
        let request = FeatureTester::builder()
            .files(vec![("main.tex", "\\newacronym[longplural={Frames per Second}]{fpsLabel}{FPS}{Frame per Second}\n\\acrshort{}")])
            .main("main.tex")
            .line(1)
            .character(11)
            .build()
            .completion();

        let context = CursorContext::new(request);
        let mut actual_items = Vec::new();
        complete_acronyms(&context, &mut actual_items, CancellationToken::none());

        assert!(actual_items.is_empty());
    }

    #[test]
    fn test_open_brace() {
        let request = FeatureTester::builder()
        .files(vec![("main.tex", "\\newacronym[longplural={Frames per Second}]{fpsLabel}{FPS}{Frame per Second}\n\\acrshort{f")])
        .main("main.tex")
        .line(1)
        .character(11)
        .build()
        .completion();

        let context = CursorContext::new(request);
        let mut actual_items = Vec::new();
        complete_acronyms(&context, &mut actual_items, CancellationToken::none());

        assert!(!actual_items.is_empty());
        for item in actual_items {
            assert_eq!(item.range, TextRange::new(87.into(), 88.into()));
        }
    }
}
