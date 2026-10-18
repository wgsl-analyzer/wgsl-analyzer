# Project Discovery

`wgsl-analyzer` will attempt to discover your project structure and figure out where your shader files are.
To assist with this, it will need some metadata about your project called a WESL manifest.
This metadata tells the analyzer where to search for shader files, what files to include/exclude, what version of WESL is used, etc.

## WESL Manifest

There are a different options depending on the type of project you are working on.
See the appropriate section for your language.

### Javascript Projects

For Javascript projects, you will need to create a `wesl.toml` file along side your `package.json`.
If you have multiple packages within your workspace, you will need a `wesl.toml` file for each package.

### Rust Projects

There are two methods available for this:

- A `wesl.toml` file
- A section in your project's `Cargo.toml`

If your project is structured as a workspace with multiple packages, you will need one of the prior options in each package.

### `wesl.toml`

When using a `wesl.toml` file, you will need at least the following:

```toml
# Version of WESL used
edition = <WESL_VERSION> # For example, "2026_pre"
# Package manager of project
package_manager = <PACKAGE_MANAGER> # npm or cargo
root = <PATH_TO_SHADER_FILES>
```

**NOTE**: At this time, project discovery will fail if there is no `edition` field set or the analyzer cannot find the shader root directory.
`wgsl-analyzer` will report back an error message in this case.

### `Cargo.toml`

For `Cargo.toml`, use the same fields as in `wesl.toml` but include them within a table `[package.metadata.wgsl-analyzer]`.

```toml
[package.metadata.wgsl-analyzer]
# wesl.toml fields
```

### Further details

Please see WESL documentation for more [details](https://wesl-lang.dev/spec/WeslToml).
