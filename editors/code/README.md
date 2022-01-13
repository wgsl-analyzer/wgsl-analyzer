# wgsl-analyzer

wgsl-analyzer is a [language server](https://microsoft.github.io/language-server-protocol/) plugin for the [WGSL Shading language](https://gpuweb.github.io/gpuweb/wgsl/).

## Features

Currently, wgsl-analyzer supports
- syntax highlighting
- basic autocomplete
- type checking
- go to definition
- basic formatting

If you have any suggestions or bug reports, feel free to open an issue at https://github.com/wgsl-analyzer/wgsl-analyzer/issues.

## Configuration

In the `wgsl-analyzer` section in the vscode settings you can specify the following configuration options:

### Custom server path

```json
{
    "wgsl-analyzer.server.path": "~/.cargo/bin/wgsl_analyzer"
}
```

### Custom imports

wgsl-analyzer supports `#import` directives in the flavour of [Bevy Engine](https://bevyengine.org)'s [shader preprocessor](https://bevyengine.org/news/bevy-0-6/#shader-imports). You can define custom import snippet in the `wgsl-analyzer.customImports` section.

```json
{
    "wgsl-analyzer.customImports": {
        "bevy_pbr::mesh_view_bind_group": "struct View {\n    view_proj: mat4x4<f32>;\n    inverse_view: mat4x4<f32>; ...",
        "bevy_pbr::mesh_struct": "struct Mesh {\n    model: mat4x4<f32>;\n    inverse_transpose_model: mat4x4<f32>; ...",
    }
}
```