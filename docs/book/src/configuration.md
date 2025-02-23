# Configuration

**Source:** [config.rs](https://github.com/wgsl-analyzer/wgsl-analyzer/blob/main/crates/wgsl-analyzer/src/config.rs)

The [Installation](./installation.md) section contains details on configuration for some of the editors.
In general, `wgsl-analyzer` is configured via LSP messages, which means that it is up to the editor to decide on the exact format and location of configuration files.

Some editors, such as [VS Code](./vs_code.md) or [COC plugin in Vim](./other_editors.md#coc-wgsl-analyzer), provide `wgsl-analyzer`-specific configuration UIs.
Other editors may require you to know a bit more about the interaction with `wgsl-analyzer`.

For the latter category, it might help to know that the initial configuration is specified as a value of the `initializationOptions` field of the [`InitializeParams` message, in the LSP protocol].
The spec says that the field type is `any?`, but `wgsl-analyzer` is looking for a JSON object that is constructed using settings from the list below.
The name of the setting, ignoring the `wgsl-analyzer.` prefix, is used as a path, and the value of the setting becomes the JSON property value.

Please consult your editor's documentation to learn more about how to configure [LSP servers](https://microsoft.github.io/language-server-protocol/).

To verify which configuration is actually used by `wgsl-analyzer`, set the `WA_LOG` environment variable to `wgsl_analyzer=info` and look for config-related messages.
Logs should show both the JSON that `wgsl-analyzer` sees as well as the updated config.

This is the list of config options `wgsl-analyzer` supports:

{{#include configuration_generated.md}}

[`InitializeParams` message, in the LSP protocol]: https://microsoft.github.io/language-server-protocol/specifications/specification-current/#initialize
