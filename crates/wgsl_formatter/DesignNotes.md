# Design Draft
For now this is an unstructured collection of rules and thoughts about how
WGSL should be formatted.
Eventually these thoughts get codified into Formatting.md

* [Some more thoughts](https://discord.com/channels/1289346613185351722/1341941812675481680/1475555853066047549)

# Ignoring
[Issue](https://github.com/wgsl-analyzer/wgsl-analyzer/issues/93)
There should be a comment or attribute
```
// wgslfmt-ignore
```

That behaves like prettier-ignore:
> I think their heuristic is something like that:
> * If the prettier-ignore comment is on the same line, but outside an ast-node, it ignores that node
> * If the prettier-ignore comment is not on the same line as any ast-node, it ignores the next ast node
> * If the prettier-ignore comment is inside an ast-node and its not clearly preceding or following an ast node, it doesn't go into effect
