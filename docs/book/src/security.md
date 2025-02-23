# Security

At the moment, `wgsl-analyzer` assumes that all code is trusted.
Here is a **non-exhaustive** list of ways to make `wgsl-analyzer` execute arbitrary code:

- VS Code plugin reads configuration from project directory, and that can be used to override paths to various executables, like `wgslfmt` or `wgsl-analyzer` itself.

- `wgsl-analyzer`'s syntax trees library uses a lot of `unsafe` and has not been properly audited for memory safety.
