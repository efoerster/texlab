package texlab.completion.latex

import org.eclipse.lsp4j.CompletionItem
import org.eclipse.lsp4j.CompletionParams
import texlab.completion.CompletionItemFactory
import texlab.provider.FeatureRequest
import texlab.syntax.latex.LatexCommandSyntax

object TikzLibraryProvider : LatexArgumentProvider() {
    private val libraries = arrayOf(
            "3d",
            "angles",
            "arrows",
            "automata",
            "babel",
            "backgrounds",
            "bending",
            "calc",
            "calendar",
            "chains",
            "circuits",
            "circuits.ee",
            "circuits.ee.IEC",
            "circuits.logic.CDH",
            "circuits.logic",
            "circuits.logic.IEC",
            "circuits.logic.US",
            "datavisualization.3d",
            "datavisualization.barcharts",
            "datavisualization",
            "datavisualization.formats.functions",
            "datavisualization.polar",
            "datavisualization.sparklines",
            "decorations",
            "decorations.footprints",
            "decorations.fractals",
            "decorations.markings",
            "decorations.pathmorphing",
            "decorations.pathreplacing",
            "decorations.shapes",
            "decorations.text",
            "er",
            "fadings",
            "fit",
            "fixedpointarithmetic",
            "folding",
            "fpu",
            "graphs",
            "graphs.standard",
            "intersections",
            "lindenmayersystems",
            "math",
            "matrix",
            "mindmap",
            "patterns",
            "patterns.meta",
            "petri",
            "plothandlers",
            "plotmarks",
            "positioning",
            "quotes",
            "scopes",
            "shadings",
            "shadows",
            "shapes.arrows",
            "shapes.callouts",
            "shapes",
            "shapes.gates.logic.IEC",
            "shapes.gates.logic.US",
            "shapes.geometric",
            "shapes.misc",
            "shapes.multipart",
            "shapes.symbols",
            "snakes",
            "spy",
            "svg.path",
            "through",
            "topaths",
            "trees",
            "turtle"
    )

    private val items = libraries.map { CompletionItemFactory.createTikzLibrary(it) }

    override val commandNames: List<String> = listOf("\\usetikzlibrary")

    override val argumentIndex: Int = 0

    override fun complete(request: FeatureRequest<CompletionParams>,
                          command: LatexCommandSyntax): List<CompletionItem> {
        return items
    }
}