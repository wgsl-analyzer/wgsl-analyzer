# WGSL/WESL Zed Extension

This extension provides language support for [WGSL], [WESL] and a best effor support for Bevy ([naga_oil]) extensions for the [Zed editor].
It uses [wgsl-analyzer] for the language server and [tree-sitter-WESL] for the tree-sitter grammar.

The syntax highlighting is heavily inspired by [wgsl-analyzer VS Code extension].

There another Zed extension for WGSL, [WGSL-zed], but it uses [glasgow] for the LSP and [tree-sitter-WGSL] which is outdated and does not support WESL.
Additionally, it lacks quality syntax highlighting and does not automatically download the lsp server if it is missing on your system.

[WGSL]: https://gpuweb.github.io/gpuweb/wgsl/
[WESL]: https://github.com/wgsl-tooling-wg/wesl-spec
[naga_oil]: https://github.com/bevyengine/naga_oil
[wgsl-analyzer]: https://github.com/wgsl-analyzer/wgsl-analyzer
[tree-sitter-WESL]: https://github.com/wgsl-tooling-wg/tree-sitter-wesl
[wgsl-analyzer VS Code extension]: https://github.com/wgsl-analyzer/wgsl-analyzer/blob/main/editors/code/syntaxes/wgsl.tmLanguage.json
[WGSL-zed]: https://github.com/luan/zed-wgsl
[glasgow]: https://github.com/nolanderc/glasgow
[tree-sitter-WGSL]: https://github.com/szebniok/tree-sitter-wgsl
[Zed editor]: https://zed.dev
