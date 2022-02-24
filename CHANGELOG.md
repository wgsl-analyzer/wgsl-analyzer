# Changelog

### Unreleased
- support `#ifdef`, `#ifndef`, `#else`, `#endif` preprocessor macros
- add `Show full WGSL source` command that resolves imports
- fix reloading of configuration for imports
- some more code formatting ([#18](https://github.com/wgsl-analyzer/wgsl-analyzer/pull/18))

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