This file was generated from all the unit tests via a ripgrep search.

```
cd crates/wgsl_formatter/src/tests
rg -U 'check\([\s\n]*"(([^"]*\n*)+)",' -r '$1' -I . > ../../benches/large_file.wesl
```

And then going through the file manually to remove any non-parsable syntax.
