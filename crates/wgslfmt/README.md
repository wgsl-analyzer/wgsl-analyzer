# wgslfmt

<!--
[![linux](https://github.com/wgsl-analyzer/wgslfmt/actions/workflows/linux.yml/badge.svg?event=push)](https://github.com/wgsl-analyzer/wgslfmt/actions/workflows/linux.yml)
[![mac](https://github.com/wgsl-analyzer/wgslfmt/actions/workflows/mac.yml/badge.svg?event=push)](https://github.com/wgsl-analyzer/wgslfmt/actions/workflows/mac.yml)
[![windows](https://github.com/wgsl-analyzer/wgslfmt/actions/workflows/windows.yml/badge.svg?event=push)](https://github.com/wgsl-analyzer/wgslfmt/actions/workflows/windows.yml)
[![crates.io](https://img.shields.io/crates/v/wgslfmt-nightly.svg)](https://crates.io/crates/wgslfmt-nightly)
-->

A tool for formatting Rust code according to style guidelines.

If you'd like to help out, see [Contributing.md](Contributing.md) and our [Code of Conduct](CODE_OF_CONDUCT.md).

You can use wgslfmt in Travis CI builds. We provide a minimal Travis CI configuration (see [here](#checking-style-on-a-ci-server)).

## Quick start

To install:

```bash
cargo install --git https://github.com/wgsl-analyzer/wgsl-analyzer wgslfmt
```

or

```bash
cargo install wgslfmt
```

To run on a cargo project in the current working directory:

```bash
wgslfmt
```

## Limitations

wgslfmt tries to work on as much Rust code as possible, even if there are syntax errors!
There are currently no stability guarantees, so updating wgslfmt may cause noise.

### Running `wgslfmt`

The `wgslfmt` binary supports input from `stdin` or by specifying a filename.
It can also format the current working directory.

Some examples follow:

- `wgslfmt` on its own will format the current working directory.
- `wgslfmt example1.wgsl example2.wgsl` will format `example1.wgsl` and `example2.wgsl` in place.
- `wgslfmt` will read a code from `stdin` and write formatting to `stdout`.
  - `echo "fn     x() {}" | wgslfmt` would emit "`fn x() {}`".

For more information, including arguments and emit options, see `wgslfmt --help`.

### Checking that code is formatted

When running with `--check`, wgslfmt will exit with `0` if wgslfmt would not
make any formatting changes to the input, and `1` if wgslfmt would make changes.
In other modes, wgslfmt will exit with `1` if there was some error during
formatting (for example a parsing or internal error) and `0` if formatting
completed without error (whether or not changes were made).

## Running wgslfmt from your editor

- [Visual Studio Code](https://marketplace.visualstudio.com/items?itemName=wgsl-analyzer.wgsl-analyzer)

## Checking style on a CI server

To keep your code base consistently formatted, it can be helpful to fail the CI build
when a pull request contains unformatted code. Using `--check` instructs
wgslfmt to exit with an error code if the input is not formatted correctly.
It will also print any found differences.

A minimal Travis setup could look like this:

```yaml
language: rust
before_script:
- cargo install --git https://github.com/wgsl-analyzer/wgsl-analyzer wgslfmt
script:
- wgslfmt --all -- --check
```

See [this blog post](https://medium.com/@ag_dubs/enforcing-style-in-ci-for-rust-projects-18f6b09ec69d) for more info.

## How to build and test

`cargo build` to build.

`cargo test` to run all tests.

To run wgslfmt after this, use `cargo run --bin wgslfmt -- filename`.

## Configuring wgslfmt

wgslfmt is designed to be very configurable. You can create a TOML file called
`wgslfmt.toml` or `.wgslfmt.toml`, place it in the project or any other parent
directory and it will apply the options in that file. See `wgslfmt --help=config`
for the options which are available, or if you prefer to see visual style previews,
[GitHub page](https://wgsl-analyzer.github.io/wgslfmt).

By default, wgslfmt uses a style which conforms to the [Rust style guide][style
guide] that has been formalized through the [style RFC
process][fmt rfcs].

Configuration options are either stable or unstable. Stable options can always
be used, while unstable ones are only available on a nightly toolchain, and opt-in.
See [GitHub page](https://wgsl-analyzer.github.io/wgslfmt) for details.

## Tips

- For things you do not want wgslfmt to mangle, use `// wgslfmt: skip` on the previous line.

- When you run wgslfmt, place a file named `wgslfmt.toml` or `.wgslfmt.toml` in
  target file directory or its parents to override the default settings of
  wgslfmt. You can generate a file containing the default configuration with
  `wgslfmt --print-config default wgslfmt.toml` and customize as needed.

- After successful compilation, a `wgslfmt` executable can be found in the target directory.
- If you're having issues compiling wgslfmt (or compile errors when trying to
  install), make sure you have the most recent version of Rust installed.

- You can change the way wgslfmt emits the changes with the `--emit` flag:

  Example:

  ```bash
  wgslfmt --emit files
  ```

  Options:

  | Flag       | Description                                       | Nightly Only |
  | ---------- | ------------------------------------------------- | ------------ |
  | files      | overwrites output to files                        | No           |
  | stdout     | writes output to stdout                           | No           |
  | coverage   | displays how much of the input file was processed | Yes          |
  | checkstyle | emits in a checkstyle format                      | Yes          |
  | json       | emits diffs in a json format                      | Yes          |

## License

wgslfmt is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.

[fmt rfcs]: https://github.com/rust-dev-tools/fmt-rfcs
[style guide]: https://doc.rust-lang.org/nightly/style-guide/
