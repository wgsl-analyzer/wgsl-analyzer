# `wgsl-analyzer`

`wgsl-analyzer` is a [language server](https://microsoft.github.io/language-server-protocol/) plugin for the [WGSL Shading language](https://www.w3.org/TR/WGSL).
It also supports [WESL] - a superset of WGSL.

## Features

Currently, wgsl-analyzer supports

- syntax highlighting
- basic autocomplete
- type checking
- go to definition
- basic formatting

If you have any suggestions or bug reports, feel free to open an issue at <https://github.com/wgsl-analyzer/wgsl-analyzer/issues>.

## Configuration

In the `wgsl-analyzer` section in the vscode settings you can specify the following configuration options:

### Custom server path

```json
{
	"wgsl-analyzer.server.path": "~/.cargo/bin/wgsl-analyzer"
}
```

### Diagnostics

wgsl-analyzer will support diagnostics for parsing errors, and optionally (by default yes) type errors and naga-reported validation errors.
You can also additionally enable diagnostics for naga parsing errors.

```json
{
	"wgsl-analyzer.diagnostics.typeErrors": true,
	"wgsl-analyzer.diagnostics.nagaParsing": false,
	"wgsl-analyzer.diagnostics.nagaValidation": true,
	"wgsl-analyzer.diagnostics.nagaVersion": "0.22" // one of the supported versions or 'main'
}
```

### Inlay hints

wgsl-analyzer can display read-only virtual text snippets interspersed with code, used to display the inferred types of variable declarations or the names of function parameters at the call site.

```json
{
	"wgsl-analyzer.inlayHints.enabled": true,
	"wgsl-analyzer.inlayHints.typeHints": true,
	"wgsl-analyzer.inlayHints.parameterHints": true,
	"wgsl-analyzer.inlayHints.structLayoutHints": false,
	"wgsl-analyzer.inlayHints.typeVerbosity": "compact"
}
```

The `typeVerbosity` argument can be either `full`, `compact` or `inner`, which will correspond to

```rust
var x: ref<function, f32, read_write> = 0.0;
var x: ref<f32> = 0.0;
var x: f32 = 0.0;
```

respectively. For more information, check out references and the "Load Rule" in the [WGSL Spec](https://www.w3.org/TR/WGSL/#load-rule).
