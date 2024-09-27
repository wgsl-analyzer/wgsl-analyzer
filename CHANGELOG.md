# Changelog

## Version 0.9
- Add Naga 19 and 22 support, remove old 0.12 and 0.13
- fix issues around packaging script on windows
- make colons optional in default switch statement

## Version 0.8
- add `workgroupUniformLoad` builtin and missing `dot` builtins
- formatter: preserve linebreaks in function calls
- ignore `#if` directives
- use proper inlay hint API (thanks to @themcat https://github.com/wgsl-analyzer/wgsl-analyzer/pull/98)
- fix `textureDimensions` to return `u32` instead of `i32`
- allow predeclared type aliases in constructors (`vec32f(0.0)`)
- `wgslfmt`: add `--check` mode and inplace file writing

## Version 0.7
- add more missing builtins
- support identifiers using correct XID\_Start/XID\_Continue sets
- parse override declaraionts
- improve parser recovery

## Version 0.6.2
- fixes version mismatch between wgsl-analyzer server and binary

## Version 0.6.1
 - Show pre-existing diagnostics on file open
 - properly report that only `file`-schemed paths are supported, to prevent overwriting diffs

### Version 0.6.0
 - lint for parsing precedence issues
 - support type construction expressions properly
 - avoid panicking in cases where there are more `endif`s than `if`s
 - handle the scopes of bindings properly (this enables shadowing)

### Version 0.5.3
- clear diagnostics on file close
- format variables
- remove unused parenthesis on format 
- avoid panic on number with suffix
- update naga versions (now supports `0.9`, `0.10` and `main`)

### Version 0.5.2
- add `ceil` builtin
- fix panic due to wrong file in diagnostic
- allow exponentials without decimal point
- support `bitcast`
- replace `${workspaceFolder}` in import paths (other variables not supported yet)

### Version 0.5.1
- support while
- rename `firstLeadingBit`/`firstTrailingBit` builtins
- add instructions for vim usage
- fix parser hang

### Version 0.5.0
- implement #import's in function param list

### Version 0.4.6
- add naga 0.9 option
- update naga main

### Version 0.4.5
- fix issue where the `wgsl_analyzer` binary wasn't included in the `.vsix` extension file

### Version 0.4.4
- add naga parsing/validation diagnostics (`wgsl-analyzer.diagnostics.nagaValidationErrors` enabled by default, `wgsl-analyzer.diagnostics.nagaParsingErrors` not).
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
