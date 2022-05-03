# Changelog

### Unreleased
- add naga parsing/validation diagnostics (`wgsl-analyzer.diagnostics.nagaParsingErrors` enabled by default, `wgsl-analyzer.diagnostics.nagaValidationErrors` not).
- Naga version can be configured to be either `0.8` (default) or `main`: `wgsl-analyzer.diagnostics.nagaVersion": "0.8"`
- better spans in diagnostics
- parse push constants
- fix handling of `>>`, e.g. for `array<vec3<f32>>`
- support `binding_array`s
- verify that vscode extension and server binary match
- add `textureGather`/`textureGatherCompare` builtins
- fix an infinite loop in the parser

## Version 0.4.3
- experimental: struct layout hints with `wgsl-analyzer.inlayHints.structLayoutHints`
- handle `,` as struct field separator

## Version 0.4.2
- handle windows line endings
- implement goto definition for types

## Version 0.4.1
- ignore `#define_import_path`
- handle `switch` statement
- some formatting fixes
- include prebuilt `wgsl_analyzer` in github releases

## Version 0.4.0
- support `#ifdef`, `#ifndef`, `#else`, `#endif` preprocessor macros
- add `Show full WGSL source` command that resolves imports
- fix reloading of configuration for imports
- some more code formatting ([#18](https://github.com/wgsl-analyzer/wgsl-analyzer/pull/18))
- add inlay hints (configurable via `wgsl-analyzer.inlayHints.{enabled,typeHints,parameterHints,typeVerbosity}`)
- grey out `#ifdef`d code

## Version 0.3.1
- fix `smoothStep` to work with `vecN<f32>`

## Version 0.3.0
- implement `@`-attributes (`[[]]` still supported)
- add bit manipulation builtins
- support `file://` and `http[s]://` schemas in custom imports
- implement type aliases ([#14](https://github.com/wgsl-analyzer/wgsl-analyzer/pull/14))

## Version 0.2.0
- fix missing matrix multiplication type rules

## Version 0.1.4
- fix multisampled textures ([#9](https://github.com/wgsl-analyzer/wgsl-analyzer/issues/9))

## Version 0.1.3
- implement `i++`, `i--`
- better error message involving operator type errors

## Version 0.1.2
- better parser recovrey
- `elseif` -> `else if`
- remove `[[block]]` attribute lint

## Version 0.1.1
- implement `ptr` types
- fix `pack2x16` and `textureLoad` builtins
- add proper README
