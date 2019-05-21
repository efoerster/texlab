use crate::completion::factory;
use crate::completion::factory::LatexComponentId;
use crate::completion::latex::combinators::LatexCombinators;
use crate::data::symbols::DATABASE;
use crate::feature::FeatureRequest;
use lsp_types::{CompletionItem, CompletionParams};
use std::borrow::Cow;

pub struct LatexCommandSymbolCompletionProvider;

impl LatexCommandSymbolCompletionProvider {
    pub async fn execute(request: &FeatureRequest<CompletionParams>) -> Vec<CompletionItem> {
        await!(LatexCombinators::command(&request, async move |_| {
            let mut items = Vec::new();
            let components = request
                .component_database
                .related_components(&request.related_documents);

            for symbol in &DATABASE.commands {
                match &symbol.component {
                    Some(component) => {
                        if components.iter().any(|c| c.files.contains(&component)) {
                            let component = LatexComponentId::User(vec![Cow::from(component)]);
                            items.push(factory::create_command_symbol(
                                &symbol.command,
                                &component,
                                &symbol.image,
                            ));
                        }
                    }
                    None => {
                        items.push(factory::create_command_symbol(
                            &symbol.command,
                            &LatexComponentId::Kernel,
                            &symbol.image,
                        ));
                    }
                }
            }
            items
        }))
    }
}