# `wgsl-analyzer`

`wgsl-analyzer` is a [language server](https://microsoft.github.io/language-server-protocol/) plugin for the [WGSL Shading language](https://www.w3.org/TR/WGSL).
It also supports [WESL] - a superset of WGSL.

## Features

Currently, `wgsl-analyzer` supports

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

### Custom imports

`wgsl-analyzer` supports `#import` directives in the flavor of [Bevy Engine](https://bevyengine.org)'s [shader preprocessor](https://bevyengine.org/news/bevy-0-6/#shader-imports).
You can define custom import snippet in the `wgsl-analyzer.customImports` section.

If you provide a URL with a `http`, `https` or `file` scheme that resource will be downloaded and used.
Keep in mind that this will slow down the LSP startup, so if you notice significant delays
(the extension will warn if it took longer than a second) consider replacing resources on the network by file URLs or inline text.

```json
{
	"wgsl-analyzer.customImports": {
		"bevy_pbr::mesh_bindings": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/render/mesh_bindings.wgsl",
		"bevy_pbr::mesh_functions": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/render/mesh_functions.wgsl"
	}
}
```

List of imports for bevy as of version 0.15.2

> Generated using:
>
> ```bash
> rg define_import_path -g '*.wgsl' --sort path | sd '^([^:]*):#define_import_path (.*)' ' "$2": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/$1",' >> ~/source/wgsl-analyzer/output.txt
> ```
>
> inside the bevy folder.

```json
{
	"wgsl-analyzer.customImports": {
		"bevy_core_pipeline::fullscreen_vertex_shader": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_core_pipeline/src/fullscreen_vertex_shader/fullscreen.wgsl",
		"bevy_core_pipeline::oit": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_core_pipeline/src/oit/oit_draw.wgsl",
		"bevy_core_pipeline::post_processing::chromatic_aberration": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_core_pipeline/src/post_process/chromatic_aberration.wgsl",
		"bevy_core_pipeline::tonemapping_lut_bindings": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_core_pipeline/src/tonemapping/lut_bindings.wgsl",
		"bevy_core_pipeline::tonemapping": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_core_pipeline/src/tonemapping/tonemapping_shared.wgsl",
		"bevy_pbr::atmosphere::bindings": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/atmosphere/bindings.wgsl",
		"bevy_pbr::atmosphere::bruneton_functions": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/atmosphere/bruneton_functions.wgsl",
		"bevy_pbr::atmosphere::functions": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/atmosphere/functions.wgsl",
		"bevy_pbr::atmosphere::types": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/atmosphere/types.wgsl",
		"bevy_pbr::decal::clustered": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/decal/clustered.wgsl",
		"bevy_pbr::decal::forward": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/decal/forward_decal.wgsl",
		"bevy_pbr::pbr_deferred_functions": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/deferred/pbr_deferred_functions.wgsl",
		"bevy_pbr::pbr_deferred_types": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/deferred/pbr_deferred_types.wgsl",
		"bevy_pbr::environment_map": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/light_probe/environment_map.wgsl",
		"bevy_pbr::irradiance_volume": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/light_probe/irradiance_volume.wgsl",
		"bevy_pbr::light_probe": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/light_probe/light_probe.wgsl",
		"bevy_pbr::lightmap": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/lightmap/lightmap.wgsl",
		"bevy_pbr::meshlet_visibility_buffer_resolve": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/meshlet/dummy_visibility_buffer_resolve.wgsl",
		"bevy_pbr::meshlet_bindings": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/meshlet/meshlet_bindings.wgsl",
		"bevy_pbr::meshlet_visibility_buffer_resolve": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/meshlet/visibility_buffer_resolve.wgsl",
		"bevy_pbr::prepass_bindings": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/prepass/prepass_bindings.wgsl",
		"bevy_pbr::prepass_io": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/prepass/prepass_io.wgsl",
		"bevy_pbr::prepass_utils": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/prepass/prepass_utils.wgsl",
		"bevy_pbr::clustered_forward": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/render/clustered_forward.wgsl",
		"bevy_pbr::fog": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/render/fog.wgsl",
		"bevy_pbr::forward_io": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/render/forward_io.wgsl",
		"bevy_pbr::mesh_bindings": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/render/mesh_bindings.wgsl",
		"bevy_pbr::mesh_functions": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/render/mesh_functions.wgsl",
		"bevy_pbr::mesh_types": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/render/mesh_types.wgsl",
		"bevy_pbr::mesh_view_bindings": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/render/mesh_view_bindings.wgsl",
		"bevy_pbr::mesh_view_types": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/render/mesh_view_types.wgsl",
		"bevy_pbr::morph": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/render/morph.wgsl",
		"bevy_pbr::occlusion_culling": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/render/occlusion_culling.wgsl",
		"bevy_pbr::parallax_mapping": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/render/parallax_mapping.wgsl",
		"bevy_pbr::ambient": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/render/pbr_ambient.wgsl",
		"bevy_pbr::pbr_bindings": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/render/pbr_bindings.wgsl",
		"bevy_pbr::pbr_fragment": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/render/pbr_fragment.wgsl",
		"bevy_pbr::pbr_functions": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/render/pbr_functions.wgsl",
		"bevy_pbr::lighting": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/render/pbr_lighting.wgsl",
		"bevy_pbr::pbr_prepass_functions": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/render/pbr_prepass_functions.wgsl",
		"bevy_pbr::transmission": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/render/pbr_transmission.wgsl",
		"bevy_pbr::pbr_types": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/render/pbr_types.wgsl",
		"bevy_pbr::rgb9e5": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/render/rgb9e5.wgsl",
		"bevy_pbr::shadow_sampling": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/render/shadow_sampling.wgsl",
		"bevy_pbr::shadows": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/render/shadows.wgsl",
		"bevy_pbr::skinning": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/render/skinning.wgsl",
		"bevy_pbr::utils": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/render/utils.wgsl",
		"bevy_pbr::view_transformations": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/render/view_transformations.wgsl",
		"bevy_pbr::ssao_utils": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/ssao/ssao_utils.wgsl",
		"bevy_pbr::raymarch": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/ssr/raymarch.wgsl",
		"bevy_pbr::ssr": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_pbr/src/ssr/ssr.wgsl",
		"bevy_render::color_operations": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_render/src/color_operations.wgsl",
		"bevy_pbr::mesh_preprocess_types": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_render/src/experimental/occlusion_culling/mesh_preprocess_types.wgsl",
		"bevy_render::globals": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_render/src/globals.wgsl",
		"bevy_render::maths": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_render/src/maths.wgsl",
		"bevy_render::view": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_render/src/view/view.wgsl",
		"bevy_sprite::mesh2d_bindings": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_sprite/src/mesh2d/mesh2d_bindings.wgsl",
		"bevy_sprite::mesh2d_functions": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_sprite/src/mesh2d/mesh2d_functions.wgsl",
		"bevy_sprite::mesh2d_types": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_sprite/src/mesh2d/mesh2d_types.wgsl",
		"bevy_sprite::mesh2d_vertex_output": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_sprite/src/mesh2d/mesh2d_vertex_output.wgsl",
		"bevy_sprite::mesh2d_view_bindings": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_sprite/src/mesh2d/mesh2d_view_bindings.wgsl",
		"bevy_sprite::mesh2d_view_types": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_sprite/src/mesh2d/mesh2d_view_types.wgsl",
		"bevy_sprite::sprite_view_bindings": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_sprite/src/render/sprite_view_bindings.wgsl",
		"bevy_ui::ui_vertex_output": "https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.2/crates/bevy_ui/src/render/ui_vertex_output.wgsl"
	}
}
```

For faster startup:

```json
{
	"wgsl-analyzer.customImports": {
		"bevy_core_pipeline::fullscreen_vertex_shader": "file:///path/to/your/local/bevy/clone/crates/bevy_core_pipeline/src/fullscreen_vertex_shader/fullscreen.wgsl",
		"bevy_core_pipeline::oit": "file:///path/to/your/local/bevy/clone/crates/bevy_core_pipeline/src/oit/oit_draw.wgsl",
		"bevy_core_pipeline::post_processing::chromatic_aberration": "file:///path/to/your/local/bevy/clone/crates/bevy_core_pipeline/src/post_process/chromatic_aberration.wgsl",
		"bevy_core_pipeline::tonemapping_lut_bindings": "file:///path/to/your/local/bevy/clone/crates/bevy_core_pipeline/src/tonemapping/lut_bindings.wgsl",
		"bevy_core_pipeline::tonemapping": "file:///path/to/your/local/bevy/clone/crates/bevy_core_pipeline/src/tonemapping/tonemapping_shared.wgsl",
		"bevy_pbr::atmosphere::bindings": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/atmosphere/bindings.wgsl",
		"bevy_pbr::atmosphere::bruneton_functions": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/atmosphere/bruneton_functions.wgsl",
		"bevy_pbr::atmosphere::functions": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/atmosphere/functions.wgsl",
		"bevy_pbr::atmosphere::types": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/atmosphere/types.wgsl",
		"bevy_pbr::decal::clustered": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/decal/clustered.wgsl",
		"bevy_pbr::decal::forward": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/decal/forward_decal.wgsl",
		"bevy_pbr::pbr_deferred_functions": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/deferred/pbr_deferred_functions.wgsl",
		"bevy_pbr::pbr_deferred_types": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/deferred/pbr_deferred_types.wgsl",
		"bevy_pbr::environment_map": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/light_probe/environment_map.wgsl",
		"bevy_pbr::irradiance_volume": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/light_probe/irradiance_volume.wgsl",
		"bevy_pbr::light_probe": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/light_probe/light_probe.wgsl",
		"bevy_pbr::lightmap": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/lightmap/lightmap.wgsl",
		"bevy_pbr::meshlet_visibility_buffer_resolve": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/meshlet/dummy_visibility_buffer_resolve.wgsl",
		"bevy_pbr::meshlet_bindings": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/meshlet/meshlet_bindings.wgsl",
		"bevy_pbr::meshlet_visibility_buffer_resolve": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/meshlet/visibility_buffer_resolve.wgsl",
		"bevy_pbr::prepass_bindings": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/prepass/prepass_bindings.wgsl",
		"bevy_pbr::prepass_io": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/prepass/prepass_io.wgsl",
		"bevy_pbr::prepass_utils": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/prepass/prepass_utils.wgsl",
		"bevy_pbr::clustered_forward": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/clustered_forward.wgsl",
		"bevy_pbr::fog": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/fog.wgsl",
		"bevy_pbr::forward_io": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/forward_io.wgsl",
		"bevy_pbr::mesh_bindings": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/mesh_bindings.wgsl",
		"bevy_pbr::mesh_functions": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/mesh_functions.wgsl",
		"bevy_pbr::mesh_types": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/mesh_types.wgsl",
		"bevy_pbr::mesh_view_bindings": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/mesh_view_bindings.wgsl",
		"bevy_pbr::mesh_view_types": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/mesh_view_types.wgsl",
		"bevy_pbr::morph": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/morph.wgsl",
		"bevy_pbr::occlusion_culling": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/occlusion_culling.wgsl",
		"bevy_pbr::parallax_mapping": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/parallax_mapping.wgsl",
		"bevy_pbr::ambient": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/pbr_ambient.wgsl",
		"bevy_pbr::pbr_bindings": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/pbr_bindings.wgsl",
		"bevy_pbr::pbr_fragment": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/pbr_fragment.wgsl",
		"bevy_pbr::pbr_functions": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/pbr_functions.wgsl",
		"bevy_pbr::lighting": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/pbr_lighting.wgsl",
		"bevy_pbr::pbr_prepass_functions": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/pbr_prepass_functions.wgsl",
		"bevy_pbr::transmission": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/pbr_transmission.wgsl",
		"bevy_pbr::pbr_types": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/pbr_types.wgsl",
		"bevy_pbr::rgb9e5": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/rgb9e5.wgsl",
		"bevy_pbr::shadow_sampling": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/shadow_sampling.wgsl",
		"bevy_pbr::shadows": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/shadows.wgsl",
		"bevy_pbr::skinning": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/skinning.wgsl",
		"bevy_pbr::utils": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/utils.wgsl",
		"bevy_pbr::view_transformations": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/render/view_transformations.wgsl",
		"bevy_pbr::ssao_utils": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/ssao/ssao_utils.wgsl",
		"bevy_pbr::raymarch": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/ssr/raymarch.wgsl",
		"bevy_pbr::ssr": "file:///path/to/your/local/bevy/clone/crates/bevy_pbr/src/ssr/ssr.wgsl",
		"bevy_render::color_operations": "file:///path/to/your/local/bevy/clone/crates/bevy_render/src/color_operations.wgsl",
		"bevy_pbr::mesh_preprocess_types": "file:///path/to/your/local/bevy/clone/crates/bevy_render/src/experimental/occlusion_culling/mesh_preprocess_types.wgsl",
		"bevy_render::globals": "file:///path/to/your/local/bevy/clone/crates/bevy_render/src/globals.wgsl",
		"bevy_render::maths": "file:///path/to/your/local/bevy/clone/crates/bevy_render/src/maths.wgsl",
		"bevy_render::view": "file:///path/to/your/local/bevy/clone/crates/bevy_render/src/view/view.wgsl",
		"bevy_sprite::mesh2d_bindings": "file:///path/to/your/local/bevy/clone/crates/bevy_sprite/src/mesh2d/mesh2d_bindings.wgsl",
		"bevy_sprite::mesh2d_functions": "file:///path/to/your/local/bevy/clone/crates/bevy_sprite/src/mesh2d/mesh2d_functions.wgsl",
		"bevy_sprite::mesh2d_types": "file:///path/to/your/local/bevy/clone/crates/bevy_sprite/src/mesh2d/mesh2d_types.wgsl",
		"bevy_sprite::mesh2d_vertex_output": "file:///path/to/your/local/bevy/clone/crates/bevy_sprite/src/mesh2d/mesh2d_vertex_output.wgsl",
		"bevy_sprite::mesh2d_view_bindings": "file:///path/to/your/local/bevy/clone/crates/bevy_sprite/src/mesh2d/mesh2d_view_bindings.wgsl",
		"bevy_sprite::mesh2d_view_types": "file:///path/to/your/local/bevy/clone/crates/bevy_sprite/src/mesh2d/mesh2d_view_types.wgsl",
		"bevy_sprite::sprite_view_bindings": "file:///path/to/your/local/bevy/clone/crates/bevy_sprite/src/render/sprite_view_bindings.wgsl",
		"bevy_ui::ui_vertex_output": "file:///path/to/your/local/bevy/clone/crates/bevy_ui/src/render/ui_vertex_output.wgsl"
	}
}
```

### Preprocessor defines

`wgsl-analyzer` supports `#ifdef`, `#ifndef`, `#else`, `#endif` directives in the flavor of [Bevy Engine](https://bevyengine.org)'s [shader preprocessor](https://bevyengine.org/news/bevy-0-6/#shader-imports).

```json
{
	"wgsl-analyzer.preprocessor.shaderDefs": ["VERTEX_TANGENTS"]
}
```

### Diagnostics

`wgsl-analyzer` will support diagnostics for parsing errors, and optionally (by default yes) type errors and naga-reported validation errors.
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

`wgsl-analyzer` can display read-only virtual text snippets interspersed with code, used to display the inferred types of variable declarations or the names of function parameters at the call site.

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
