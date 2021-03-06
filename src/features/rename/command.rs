use std::collections::HashMap;

use cancellation::CancellationToken;
use cstree::{TextRange, TextSize};
use lsp_types::{Range, RenameParams, TextEdit, WorkspaceEdit};

use crate::{
    features::cursor::{CursorContext, HasPosition},
    LineIndexExt,
};

pub fn prepare_command_rename<P: HasPosition>(
    context: &CursorContext<P>,
    _cancellation_token: &CancellationToken,
) -> Option<Range> {
    Some(
        context
            .request
            .main_document()
            .line_index
            .line_col_lsp_range(context.cursor.command_range(context.offset)?),
    )
}

pub fn rename_command(
    context: &CursorContext<RenameParams>,
    cancellation_token: &CancellationToken,
) -> Option<WorkspaceEdit> {
    cancellation_token.result().ok()?;
    prepare_command_rename(context, cancellation_token)?;
    let name = context.cursor.as_latex()?.text();
    let mut changes = HashMap::new();
    for document in &context.request.subset.documents {
        cancellation_token.result().ok()?;

        if let Some(data) = document.data.as_latex() {
            let edits = data
                .root
                .descendants_with_tokens()
                .filter_map(|element| element.into_token())
                .filter(|token| token.kind().is_command_name() && token.text() == name)
                .map(|token| {
                    let range = token.text_range();
                    let range = document.line_index.line_col_lsp_range(TextRange::new(
                        range.start() + TextSize::from(1),
                        range.end(),
                    ));
                    TextEdit::new(range, context.request.params.new_name.clone())
                })
                .collect();

            changes.insert(document.uri.as_ref().clone().into(), edits);
        }
    }

    Some(WorkspaceEdit::new(changes))
}

#[cfg(test)]
mod tests {
    use crate::{features::testing::FeatureTester, RangeExt};

    use super::*;

    #[test]
    fn test_command() {
        let tester = FeatureTester::builder()
            .files(vec![
                ("foo.tex", r#"\baz\include{bar.tex}"#),
                ("bar.tex", r#"\baz"#),
            ])
            .main("foo.tex")
            .line(0)
            .character(2)
            .new_name("qux")
            .build();

        let uri1 = tester.uri("foo.tex");
        let uri2 = tester.uri("bar.tex");
        let req = tester.rename();

        let context = CursorContext::new(req);
        let actual_edit = rename_command(&context, CancellationToken::none()).unwrap();

        let mut expected_changes = HashMap::new();
        expected_changes.insert(
            uri1.as_ref().clone().into(),
            vec![TextEdit::new(Range::new_simple(0, 1, 0, 4), "qux".into())],
        );
        expected_changes.insert(
            uri2.as_ref().clone().into(),
            vec![TextEdit::new(Range::new_simple(0, 1, 0, 4), "qux".into())],
        );
        let expected_edit = WorkspaceEdit::new(expected_changes);

        assert_eq!(actual_edit, expected_edit);
    }
}
