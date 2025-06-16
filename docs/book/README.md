# `wgsl-analyzer` Documentation

The `wgsl-analyzer` manual uses [mdBook](https://rust-lang.github.io/mdBook).

## Quick start

To run the documentation site locally:

```bash
cargo install mdbook
cd docs/book
mdbook serve
# make changes to documentation files in doc/book/src
# ...
```

mdBook will rebuild the documentation as changes are made.

## Making updates

While not required, installing the `mdbook` binary can be helfpul in order to see the changes.
Start with the mdBook [User Guide](https://rust-lang.github.io/mdBook/guide/installation.html) to familiarize yourself with the tool.

## Generated documentation

Four sections are generated dynamically: assists, configuration, diagnostics, and features.
Their content is found in the `generated.md` files of the respective book section.
For example, `src/configuration_generated.md`, and are included in the book via mdBook's
[include](https://rust-lang.github.io/mdBook/format/mdbook.html#including-files) functionality.
Generated files can be rebuilt by running the various test cases that generate them, or by simply running all of the `wgsl-analyzer` tests with `cargo test` and `cargo xtask codegen`.
