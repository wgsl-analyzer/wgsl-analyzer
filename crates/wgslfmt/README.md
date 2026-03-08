# wgslfmt

<!--
[![linux](https://github.com/wgsl-analyzer/wgslfmt/actions/workflows/linux.yml/badge.svg?event=push)](https://github.com/wgsl-analyzer/wgslfmt/actions/workflows/linux.yml)
[![mac](https://github.com/wgsl-analyzer/wgslfmt/actions/workflows/mac.yml/badge.svg?event=push)](https://github.com/wgsl-analyzer/wgslfmt/actions/workflows/mac.yml)
[![windows](https://github.com/wgsl-analyzer/wgslfmt/actions/workflows/windows.yml/badge.svg?event=push)](https://github.com/wgsl-analyzer/wgslfmt/actions/workflows/windows.yml)
[![crates.io](https://img.shields.io/crates/v/wgslfmt-nightly.svg)](https://crates.io/crates/wgslfmt-nightly)
-->

A tool for formatting [WGSL](https://www.w3.org/TR/WGSL/) and [WESL](https://wesl.thimoteus.dev/) shader code.

## Quick start

To install:

```bash
cargo install --git https://github.com/wgsl-analyzer/wgsl-analyzer wgslfmt
```

## Usage

`wgslfmt` accepts files, directories (recursively finds `.wgsl` and `.wesl`
files), glob patterns, or stdin (`-`).

```bash
# Format all .wgsl and .wesl files in the current directory (recursively)
wgslfmt .

# Format specific files in place
wgslfmt shader.wgsl utils.wesl

# Format from stdin (writes to stdout)
echo "fn     x() {}" | wgslfmt -

# Check formatting without modifying files (exit code 1 if changes needed)
wgslfmt --check .

# Use tabs instead of spaces
wgslfmt --tabs .

# Use 2-space indentation
wgslfmt --indent-width 2 .

# JSON output for scripting (single JSON object with all results)
wgslfmt --output-format json --check .
```

For all options, see `wgslfmt --help`.

### Checking that code is formatted

When running with `--check`, wgslfmt will exit with `0` if the input is
already formatted correctly, and `1` if formatting changes are needed.
A diff of the required changes is printed to stdout.

### JSON output

Use `--output-format json` to get machine-readable output. This emits a
single JSON object containing per-file results (changed status, duration,
parse errors, and diffs).

## WESL support

Both `.wgsl` and `.wesl` files are supported. The formatter recognizes all
WESL syntax extensions — including `import` statements and qualified paths
(e.g. `package::utils::math`) — regardless of file extension.

## Running wgslfmt from your editor

- [Visual Studio Code](https://marketplace.visualstudio.com/items?itemName=wgsl-analyzer.wgsl-analyzer)

## How to build and test

```bash
cargo build -p wgslfmt
cargo test -p wgsl-formatter
cargo run -p wgslfmt -- shader.wgsl
```

## License

wgslfmt is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).
