use cancellation::CancellationToken;
use cstree::TextRange;
use lsp_types::CompletionParams;

use crate::{
    component_db::COMPONENT_DATABASE,
    features::cursor::CursorContext,
    syntax::{latex, CstNode},
};

use super::types::{InternalCompletionItem, InternalCompletionItemData};

pub fn complete_arguments<'a>(
    context: &'a CursorContext<CompletionParams>,
    items: &mut Vec<InternalCompletionItem<'a>>,
    cancellation_token: &CancellationToken,
) -> Option<()> {
    cancellation_token.result().ok()?;

    let token = context.cursor.as_latex()?;

    let range = if token.kind() == latex::WORD {
        token.text_range()
    } else {
        TextRange::empty(context.offset)
    };

    let group = latex::CurlyGroup::cast(token.parent())
        .or_else(|| token.parent().parent().and_then(latex::CurlyGroup::cast))
        .filter(|group| context.is_inside_latex_curly(group))?;

    let command = latex::GenericCommand::cast(group.syntax().parent()?)?;

    let index = command
        .syntax()
        .children()
        .filter_map(latex::CurlyGroup::cast)
        .position(|g| g.syntax().text_range() == group.syntax().text_range())?;

    let command_name = &command.name()?.text()[1..];

    for component in COMPONENT_DATABASE.linked_components(&context.request.subset) {
        for component_command in component
            .commands
            .iter()
            .filter(|command| command.name == command_name)
        {
            for (_, param) in component_command
                .parameters
                .iter()
                .enumerate()
                .filter(|(i, _)| *i == index)
            {
                for arg in &param.0 {
                    let item = InternalCompletionItem::new(
                        range,
                        InternalCompletionItemData::Argument {
                            name: &arg.name,
                            image: arg.image.as_deref(),
                        },
                    );
                    items.push(item);
                }
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
        complete_arguments(&context, &mut actual_items, CancellationToken::none());

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
        complete_arguments(&context, &mut actual_items, CancellationToken::none());

        assert!(actual_items.is_empty());
    }

    #[test]
    fn test_latex_simple() {
        let request = FeatureTester::builder()
            .files(vec![("main.tex", r#"\mathbb{}\usepackage{amsfonts}"#)])
            .main("main.tex")
            .line(0)
            .character(8)
            .build()
            .completion();

        let context = CursorContext::new(request);
        let mut actual_items = Vec::new();
        complete_arguments(&context, &mut actual_items, CancellationToken::none());

        assert!(!actual_items.is_empty());
        for item in actual_items {
            assert_eq!(item.range, TextRange::new(8.into(), 8.into()));
        }
    }

    #[test]
    fn test_latex_word() {
        let request = FeatureTester::builder()
            .files(vec![("main.tex", r#"\mathbb{foo}\usepackage{amsfonts}"#)])
            .main("main.tex")
            .line(0)
            .character(8)
            .build()
            .completion();

        let context = CursorContext::new(request);
        let mut actual_items = Vec::new();
        complete_arguments(&context, &mut actual_items, CancellationToken::none());

        assert!(!actual_items.is_empty());
        for item in actual_items {
            assert_eq!(item.range, TextRange::new(8.into(), 11.into()));
        }
    }

    #[test]
    fn test_latex_open_brace() {
        let request = FeatureTester::builder()
            .files(vec![("main.tex", "\\mathbb{ \\usepackage{amsfonts}")])
            .main("main.tex")
            .line(0)
            .character(8)
            .build()
            .completion();

        let context = CursorContext::new(request);
        let mut actual_items = Vec::new();
        complete_arguments(&context, &mut actual_items, CancellationToken::none());

        assert!(!actual_items.is_empty());
        for item in actual_items {
            assert_eq!(item.range, TextRange::new(8.into(), 8.into()));
        }
    }

    #[test]
    fn test_latex_open_brace_second() {
        let request = FeatureTester::builder()
            .files(vec![("main.tex", "\\mathbb{}{\\usepackage{amsfonts}")])
            .main("main.tex")
            .line(0)
            .character(10)
            .build()
            .completion();

        let context = CursorContext::new(request);
        let mut actual_items = Vec::new();
        complete_arguments(&context, &mut actual_items, CancellationToken::none());

        assert!(actual_items.is_empty());
    }
}
