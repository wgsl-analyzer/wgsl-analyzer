# WGSL/WESL Zed Extension

This extension provides language support for [WGSL](https://gpuweb.github.io/gpuweb/wgsl/), [WESL](https://github.com/wgsl-tooling-wg/wesl-spec) and a best effor support for Bevy ([naga_oil](https://github.com/bevyengine/naga_oil)) extensions for the [Zed editor](https://zed.dev).
It uses [wgsl-analyzer](https://github.com/wgsl-analyzer/wgsl-analyzer) for the language server and [tree-sitter-wesl](https://github.com/wgsl-tooling-wg/tree-sitter-wesl) for the tree-sitter grammar.

The syntax highlighting is heavily inspired by [wgsl-analyzer vscode extension](https://github.com/wgsl-analyzer/wgsl-analyzer/blob/main/editors/code/syntaxes/wgsl.tmLanguage.json).

There already is a Zed extension for WGSL, [wgsl-zed](https://github.com/luan/zed-wgsl), but it uses [glasgow](https://github.com/nolanderc/glasgow) for the LSP and [tree-sitter-wgsl](https://github.com/szebniok/tree-sitter-wgsl) which is outdated and doesn't support WESL.
Additionally, it lacks quality syntax highlighting and doesn't automatically download the lsp server if it's missing on your system.
