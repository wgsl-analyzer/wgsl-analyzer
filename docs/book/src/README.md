# wgsl-analyzer

At its core, `wgsl-analyzer` is a **library** for semantic analysis of WGSL and WESL code as it changes over time.
This manual focuses on a specific usage of the library - running it as part of a server that implements the [Language Server Protocol](https://microsoft.github.io/language-server-protocol) (LSP).
The LSP allows various code editors, such as VS Code, Emacs, or Vim to implement semantic features such as completion or goto definition by talking to an external language server process.

To improve this document, send a pull request: [https://github.com/wgsl-analyzer/wgsl-analyzer](https://github.com/wgsl-analyzer/wgsl-analyzer/blob/master/docs/book/README.md).

The manual is written in markdown and includes some extra files which are generated from the source code.
Run `cargo test` and `cargo xtask codegen` to create these.

If you have a question about using `wgsl-analyzer`, please read the documentation.
If your question is not addressed, then ask it in the ["discord"](https://discord.gg/3QUGyyz984).
Ideally, the documentation should address all usage questions.
