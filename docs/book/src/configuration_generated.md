## wgsl-analyzer.cachePriming.numThreads

Default: `"physical"`

Number of worker threads used to warm caches when a project opens.
Use `0` to let the server choose automatically based on the machine.

## wgsl-analyzer.customImports

Default: `{}`

Additional import aliases the server should resolve as if they were built-ins.
Keys are the import names as they appear in source; values are their resolved targets.

## wgsl-analyzer.diagnostics.nagaParsingErrors

Default: `true`

Report WGSL parsing errors emitted by Naga.

## wgsl-analyzer.diagnostics.nagaValidationErrors

Default: `true`

Report WGSL validation errors emitted by Naga.

## wgsl-analyzer.diagnostics.nagaVersion

Default: `"main"`

Naga version used for validation.

## wgsl-analyzer.diagnostics.typeErrors

Default: `true`

Report type errors from wgsl-analyzer.

## wgsl-analyzer.inlayHints.enabled

Default: `true`

Master switch for inlay hints.

## wgsl-analyzer.inlayHints.parameterHints

Default: `true`

Show function parameter name hints at call sites.

## wgsl-analyzer.inlayHints.renderColons

Default: `true`

Show colons

## wgsl-analyzer.inlayHints.structLayoutHints

Default: `false`

Show inlay hints for struct/array layout (offsets, sizes).

## wgsl-analyzer.inlayHints.typeHints

Default: `true`

Show inlay type hints for variables.

## wgsl-analyzer.inlayHints.typeVerbosity

Default: `"compact"`

Verbosity of type hints: `"full"`, `"compact"`, or `"inner"`.

## wgsl-analyzer.numThreads

Default: `null`

Number of worker threads for the main analysis loop.
`None` lets the server choose automatically.

## wgsl-analyzer.preprocessor.shaderDefs

Default: `[]`

Preprocessor shader `#define`s to apply during analysis.
Each entry enables a conditional compilation symbol as if passed on the command line.

## wgsl-analyzer.trace.extension

Default: `false`

Emit extension-level trace logs to the client log.

## wgsl-analyzer.trace.server

Default: `"off"`

Server trace verbosity.
One of: `"off"`, `"messages"`, or `"verbose"`.

