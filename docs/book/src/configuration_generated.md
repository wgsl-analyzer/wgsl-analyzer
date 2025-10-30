## wgsl-analyzer.cachePriming.numThreads

Default: `"physical"`

Number of worker threads used to warm caches when a project opens.
Use `0` to let the server choose automatically based on the machine.

## wgsl-analyzer.customImports

Default: `{}`

Custom `#import` directives in the flavor of [Bevy Engine](https://bevyengine.org)'s [shader preprocessor](https://bevyengine.org/news/bevy-0-6/#shader-imports). To use objects from an import, add `#import <name>` to your WGSL.

## wgsl-analyzer.diagnostics.nagaParsingErrors

Default: `true`

Controls whether to show naga's parsing errors.

## wgsl-analyzer.diagnostics.nagaValidationErrors

Default: `true`

Controls whether to show naga's validation errors.

## wgsl-analyzer.diagnostics.nagaVersion

Default: `"main"`

Naga version used for validation.

## wgsl-analyzer.diagnostics.typeErrors

Default: `true`

Controls whether to show type errors.

## wgsl-analyzer.inlayHints.enabled

Default: `true`

Whether to show inlay hints.

## wgsl-analyzer.inlayHints.parameterHints

Default: `true`

Whether to show inlay hints for the names of function parameters.

## wgsl-analyzer.inlayHints.renderColons

Default: `true`

Show colons.

## wgsl-analyzer.inlayHints.structLayoutHints

Default: `false`

Whether to show inlay hints for the layout of struct fields.

## wgsl-analyzer.inlayHints.typeHints

Default: `true`

Whether to show inlay hints for types of variable declarations.

## wgsl-analyzer.inlayHints.typeVerbosity

Default: `"compact"`

Verbosity of type hints: `"full"`, `"compact"`, or `"inner"`.

## wgsl-analyzer.numThreads

Default: `null`

Number of worker threads for the main analysis loop.
`null` lets the server choose automatically.

## wgsl-analyzer.preprocessor.shaderDefs

Default: `[]`

Shader defines used in `#ifdef` directives in the flavor of [Bevy Engine](https://bevyengine.org)'s [shader preprocessor](https://bevyengine.org/news/bevy-0-6/#shader-imports).

## wgsl-analyzer.trace.extension

Default: `false`

Enable logging of VS Code extensions itself.
This settings is now deprecated.
Log level is now controlled by the [Developer: Set Log Level...](command:workbench.action.setLogLevel) command. You can set the log level for the current session and also the default log level from there. This is also available by clicking the gear icon on the OUTPUT tab when wgsl-analyzer Client is visible or by passing the --log wgsl-analyzer.wgsl-analyzer:debug parameter to VS Code.

## wgsl-analyzer.trace.server

Default: `"off"`

Server trace verbosity.
One of: `"off"`, `"messages"`, or `"verbose"`.

