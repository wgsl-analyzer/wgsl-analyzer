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

If you provide a URL with a `http`, `https` or `file` scheme that resource will be downloaded and used. Keep in mind that this will slow down the LSP startup, so if you notice significant delays (the extension will warn if it took longer than a second) consider replacing resources on the network by file URLs or inline text.

```json
{
    "wgsl-analyzer.customImports": {
        "bevy_pbr::mesh_bindings": "https://raw.githubusercontent.com/bevyengine/bevy/v0.8.0/crates/bevy_pbr/src/render/mesh_bindings.wgsl",
        "bevy_pbr::mesh_functions": "https://raw.githubusercontent.com/bevyengine/bevy/v0.8.0/crates/bevy_pbr/src/render/mesh_functions.wgsl"
    }
}
```

<details>
<summary>
List of imports for bevy as of version 0.8
</summary>


_Generated using `rg define_import_path -g '*.wgsl' --sort path | sd '^([^:]*):#define_import_path (.*)' '        "$2": "https://raw.githubusercontent.com/bevyengine/bevy/v0.8.0/$1",'` inside the bevy folder_

```json
{
    "wgsl-analyzer.customImports": {
        "bevy_pbr::clustered_forward": "https://raw.githubusercontent.com/bevyengine/bevy/v0.8.0/crates/bevy_pbr/src/render/clustered_forward.wgsl",
        "bevy_pbr::mesh_bindings": "https://raw.githubusercontent.com/bevyengine/bevy/v0.8.0/crates/bevy_pbr/src/render/mesh_bindings.wgsl",
        "bevy_pbr::mesh_functions": "https://raw.githubusercontent.com/bevyengine/bevy/v0.8.0/crates/bevy_pbr/src/render/mesh_functions.wgsl",
        "bevy_pbr::mesh_types": "https://raw.githubusercontent.com/bevyengine/bevy/v0.8.0/crates/bevy_pbr/src/render/mesh_types.wgsl",
        "bevy_pbr::mesh_vertex_output": "https://raw.githubusercontent.com/bevyengine/bevy/v0.8.0/crates/bevy_pbr/src/render/mesh_vertex_output.wgsl",
        "bevy_pbr::mesh_view_bindings": "https://raw.githubusercontent.com/bevyengine/bevy/v0.8.0/crates/bevy_pbr/src/render/mesh_view_bindings.wgsl",
        "bevy_pbr::mesh_view_types": "https://raw.githubusercontent.com/bevyengine/bevy/v0.8.0/crates/bevy_pbr/src/render/mesh_view_types.wgsl",
        "bevy_pbr::pbr_bindings": "https://raw.githubusercontent.com/bevyengine/bevy/v0.8.0/crates/bevy_pbr/src/render/pbr_bindings.wgsl",
        "bevy_pbr::pbr_functions": "https://raw.githubusercontent.com/bevyengine/bevy/v0.8.0/crates/bevy_pbr/src/render/pbr_functions.wgsl",
        "bevy_pbr::lighting": "https://raw.githubusercontent.com/bevyengine/bevy/v0.8.0/crates/bevy_pbr/src/render/pbr_lighting.wgsl",
        "bevy_pbr::pbr_types": "https://raw.githubusercontent.com/bevyengine/bevy/v0.8.0/crates/bevy_pbr/src/render/pbr_types.wgsl",
        "bevy_pbr::shadows": "https://raw.githubusercontent.com/bevyengine/bevy/v0.8.0/crates/bevy_pbr/src/render/shadows.wgsl",
        "bevy_pbr::skinning": "https://raw.githubusercontent.com/bevyengine/bevy/v0.8.0/crates/bevy_pbr/src/render/skinning.wgsl",
        "bevy_pbr::utils": "https://raw.githubusercontent.com/bevyengine/bevy/v0.8.0/crates/bevy_pbr/src/render/utils.wgsl",
        "bevy_sprite::mesh2d_bindings": "https://raw.githubusercontent.com/bevyengine/bevy/v0.8.0/crates/bevy_sprite/src/mesh2d/mesh2d_bindings.wgsl",
        "bevy_sprite::mesh2d_functions": "https://raw.githubusercontent.com/bevyengine/bevy/v0.8.0/crates/bevy_sprite/src/mesh2d/mesh2d_functions.wgsl",
        "bevy_sprite::mesh2d_types": "https://raw.githubusercontent.com/bevyengine/bevy/v0.8.0/crates/bevy_sprite/src/mesh2d/mesh2d_types.wgsl",
        "bevy_sprite::mesh2d_vertex_output": "https://raw.githubusercontent.com/bevyengine/bevy/v0.8.0/crates/bevy_sprite/src/mesh2d/mesh2d_vertex_output.wgsl",
        "bevy_sprite::mesh2d_view_bindings": "https://raw.githubusercontent.com/bevyengine/bevy/v0.8.0/crates/bevy_sprite/src/mesh2d/mesh2d_view_bindings.wgsl",
        "bevy_sprite::mesh2d_view_types": "https://raw.githubusercontent.com/bevyengine/bevy/v0.8.0/crates/bevy_sprite/src/mesh2d/mesh2d_view_types.wgsl",

        // or alternatively for faster startup

        "bevy_pbr::clustered_forward": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/clustered_forward.wgsl",
        "bevy_pbr::mesh_bindings": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/mesh_bindings.wgsl",
        "bevy_pbr::mesh_functions": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/mesh_functions.wgsl",
        "bevy_pbr::mesh_types": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/mesh_types.wgsl",
        "bevy_pbr::mesh_vertex_output": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/mesh_vertex_output.wgsl",
        "bevy_pbr::mesh_view_bindings": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/mesh_view_bindings.wgsl",
        "bevy_pbr::mesh_view_types": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/mesh_view_types.wgsl",
        "bevy_pbr::pbr_bindings": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/pbr_bindings.wgsl",
        "bevy_pbr::pbr_functions": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/pbr_functions.wgsl",
        "bevy_pbr::lighting": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/pbr_lighting.wgsl",
        "bevy_pbr::pbr_types": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/pbr_types.wgsl",
        "bevy_pbr::shadows": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/shadows.wgsl",
        "bevy_pbr::skinning": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/skinning.wgsl",
        "bevy_pbr::utils": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/utils.wgsl",
        "bevy_sprite::mesh2d_bindings": "file:///path/to/your/local/bevy/clone/crates/bevy_sprite/src/mesh2d/mesh2d_bindings.wgsl",
        "bevy_sprite::mesh2d_functions": "file:///path/to/your/local/bevy/clone/crates/bevy_sprite/src/mesh2d/mesh2d_functions.wgsl",
        "bevy_sprite::mesh2d_types": "file:///path/to/your/local/bevy/clone/crates/bevy_sprite/src/mesh2d/mesh2d_types.wgsl",
        "bevy_sprite::mesh2d_vertex_output": "file:///path/to/your/local/bevy/clone/crates/bevy_sprite/src/mesh2d/mesh2d_vertex_output.wgsl",
        "bevy_sprite::mesh2d_view_bindings": "file:///path/to/your/local/bevy/clone/crates/bevy_sprite/src/mesh2d/mesh2d_view_bindings.wgsl",
        "bevy_sprite::mesh2d_view_types": "file:///path/to/your/local/bevy/clone/crates/bevy_sprite/src/mesh2d/mesh2d_view_types.wgsl"
    }
}
```
</details>

### Preprocessor defines

wgsl-analyzer supports `#ifdef`, `#ifndef`, `#else`, `#endif` directives in the flavour of [Bevy Engine](https://bevyengine.org)'s [shader preprocessor](https://bevyengine.org/news/bevy-0-6/#shader-imports).

```json
{
    "wgsl-analyzer.preprocessor.shaderDefs": [
        "VERTEX_TANGENTS"
    ]
}
```


### Diagnostics

wgsl-analyer will support diagnostics for parsing errors, and optionally (by default yes) type errors and naga-reported validation errors.
You can also additionally enable diagnostics for naga parsing errors.

```json
{
    "wgsl-analyzer.diagnostics.typeErrors": true,
    "wgsl-analyzer.diagnostics.nagaParsing": false,
    "wgsl-analyzer.diagnostics.nagaValidation": true,
    "wgsl-analyzer.diagnostics.nagaVersion": "0.8", // either "0.8" or "main"
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
    "wgsl-analyzer.inlayHints.typeVerbosity": "compact",
}
```

The `typeVerbosity` argument can be either `full`, `compact` or `inner`, which will correspond to
```rust
var x: ref<function, f32, read_write> = 0.0;
var x: ref<f32> = 0.0;
var x: f32 = 0.0;
```
respectively. For more information, check out references and the "Load Rule" in the [WGSL Spec](https://gpuweb.github.io/gpuweb/wgsl/#load-rule).
