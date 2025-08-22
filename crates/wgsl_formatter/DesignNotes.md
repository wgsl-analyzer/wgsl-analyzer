# Design Draft
For now this is an unstructured collection of rules and thoughts about how
WGSL should be formatted.
Eventually these thoughts get codified into Formatting.md

## Annotations
```
@group(0) @binding(0)
var foo: u32;
@group(0) @binding(1)
var bar: f32;
// Or
@group(0) @binding(1) var baz: f32;

@attribute1
@attribute2
x: Y

@compute @workgroup_size() fn...
// Or
@compute @workgroup_size()
fn...
```

* Group & Binding should be on the same rows, because they are related. Other things should be split up. ([Mathis](https://discord.com/channels/1289346613185351722/1341941812675481680/1406341477713576149))

* Things that are on the same line, if they are semantically tied to one another. If two unrelated changes could sensibly change both attributes, they should be on seperate lines as to not cause a merge conflict. ([Benjamin](https://discord.com/channels/1289346613185351722/1341941812675481680/1406350422104477917))

### Open Questions
```
@group(0) @binding(1)
var bar: f32;
// Vs
@group(0) @binding(1) var baz: f32;
```

```
@compute @workgroup_size() fn...
// Or
@compute @workgroup_size()
fn...
```
