package texlab.completion.latex

import org.eclipse.lsp4j.CompletionItem
import org.eclipse.lsp4j.CompletionParams
import texlab.BibtexDocument
import texlab.completion.CompletionItemFactory
import texlab.provider.FeatureRequest
import texlab.syntax.bibtex.BibtexEntrySyntax
import texlab.syntax.latex.LatexCitation
import texlab.syntax.latex.LatexCommandSyntax

object LatexCitationProvider : LatexArgumentProvider() {
    override val commandNames: List<String> = LatexCitation.COMMAND_NAMES

    override val argumentIndex: Int = 0

    override fun complete(request: FeatureRequest<CompletionParams>,
                          command: LatexCommandSyntax): List<CompletionItem> {
        return request.relatedDocuments
                .filterIsInstance<BibtexDocument>()
                .flatMap { it.tree.root.children }
                .filterIsInstance<BibtexEntrySyntax>()
                .filter { it.name != null }
                .map { CompletionItemFactory.createCitation(it) }
    }
}