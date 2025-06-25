# Setup Guide

This guide gives a simplified, opinionated setup for developers contributing to `wgsl-analyzer` using Visual Studio Code.
It enables developers to make changes and Visual Studio Code Insiders to test those changes.
This guide will assume you have Visual Studio Code and Visual Studio Code Insiders installed.

## Prerequisites

Since `wgsl-analyzer` is a Rust project, you will need to install Rust.
You can download and install the latest stable version of [Rust](https://www.rust-lang.org/tools/install).

## Step-by-Step Setup

1. Fork the [`wgsl-analyzer` repository](https://github.com/wgsl-analyzer/wgsl-analyzer) and clone the fork to your local machine.
2. Open the project in Visual Studio Code.
3. Open a terminal and run `cargo build` to build the project.
4. Install the language server locally by running the following command:

```bash
cargo xtask install --server --code-bin code-insiders --dev-rel
```

In the output of this command, there should be a file path provided to the installed binary on your local machine.
It should look something like the following output below:

```text
Installing <path-to-wgsl-analyzer-binary>
Installed package `wgsl-analyzer v0.0.0 (<path-to-wgsl-analyzer-binary>)` (executable `wgsl-analyzer.exe`)
```

In Visual Studio Code Insiders, you will want to open your User Settings (JSON) from the Command Palette.
From there, you should ensure that the `wgsl-analyzer.server.path` key is set to the `<path-to-wgsl-analyzer-binary>`.
This will tell Visual Studio Code Insiders to use the locally installed version that you can debug.

The User Settings (JSON) file should contain the following:

```json
{
    "wgsl-analyzer.server.path": "<path-to-wgsl-analyzer-binary>"
}
```

Now you should be able to make changes to `wgsl-analyzer` in Visual Studio Code and then view the changes in Visual Studio Code Insiders.

## Debugging `wgsl-analyzer`

The simplest way to debug `wgsl-analyzer` is to use the `eprintln!` macro.
The reason why we use `eprintln!` instead of `println!` is because the language server uses `stdout` to send messages.
Instead, debug using `stderr`.

An example debugging statement could go into the `main_loop.rs` file which can be found at `crates/wgsl-analyzer/src/main_loop.rs`.
Inside the `main_loop` add the following `eprintln!` to test debugging `wgsl-analyzer`:

```rust
eprintln!("Hello, world!");
```

Now we run `cargo build` and `cargo xtask install --server --code-bin code-insiders --dev-rel` to reinstall the server.

Now on Visual Studio Code Insiders, we should be able to open the Output tab on our terminal and switch to WGSL Analyzer Language Server to see the `eprintln!` statement we just wrote.

If you are able to see your output, you now have a complete workflow for debugging `wgsl-analyzer`.
