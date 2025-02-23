# Installation

To use `wgsl-analyzer`, you need a `wgsl-analyzer` binary and a text editor that supports [LSP].

If you are [using VS Code](./vs_code.html), the extension bundles a copy of the `wgsl-analyzer` binary.
For other editors, you will need to [install the binary](./wgsl-analyzer_binary.html) and [configure your editor](./other_editors.html).

## Crates

There is a package named `wa_ap_wgsl-analyzer` available on [crates.io] for people who want to use `wgsl-analyzer` programmatically.

For more details, see [the publish workflow].

[LSP]: <https://microsoft.github.io/language-server-protocol>
[crates.io]: <https://crates.io/crates/wa_ap_wgsl-analyzer>
[the publish workflow]: <https://github.com/wgsl-analyzer/wgsl-analyzer/blob/main/.github/workflows/autopublish.yaml>
