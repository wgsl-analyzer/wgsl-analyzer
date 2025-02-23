# Contributing Quick Start

`wgsl-analyzer` is an ordinary Rust project, which is organized as a Cargo workspace, builds on stable, and does not depend on C libraries.

Simply run the following to get started:

```bash
cargo test
```

To learn more about how `wgsl-analyzer` works, see [Architecture](architecture.md).
It also explains the high-level layout of the source code.
Do skim through that document.

We also publish rustdoc docs to pages: <https://wgsl-analyzer.github.io/wgsl-analyzer/ide>.
Note that the internal documentation is very incomplete.

Various organizational and process issues are discussed in this document.

## Getting in Touch

Discussion happens in this Discord server:

<https://discord.gg/3QUGyyz984>

<!-- toc -->

## Issue Labels

<https://github.com/wgsl-analyzer/wgsl-analyzer/labels>

- [A-Analyzer]: Affects the wgsl-analyzer crate
- [A-Base-DB]: Affects the base_db crate
- [A-Build-System]: CI stuff
- [A-Completion]: Affects the ide_completion crate
- [A-Cross-Cutting]: Affects many crates
- [A-Formatter]: Affects the wgsl-formatter crate
- [A-HIR]: Affects the hir or hir_def crate
- [A-IDE]: Affects the ide crate
- [A-Meta]: Affects non-code files such as documentation
- [A-wgslfmt]: Affects the wgslfmt crate
- [C-Bug]: Something isn't working
- [C-Dependencies]: Bump and migrate a dependency
- [C-Documentation]: Improvements or additions to documentation
- [C-Enhancement]: Improvement over an existing feature
- [C-Feature]: New feature or request
- [D-Complex]: Large implications, lots of changes, much thought
- [D-Modest]: "Normal" difficulty of solving
- [D-Straightforward]: Relatively easy to solve
- [D-Trivial]: Good for newcomers
- [S-Adopt-Me]: Extra attention is needed
- [S-Blocked]: Blocked on something else happening
- [S-Duplicate]: This issue or pull request already exists
- [S-Needs-Design]: The way this should be done is not yet clear
- [S-Needs-Investigation]: The cause of the issue is TBD
- [S-Needs-Triage]: Hasn't been triaged yet
- [S-Ready-to-Implement]: This issue is actionable and a solution can be proposed
- [S-Ready-to-Review]: This change is in a good state and needs someone (anyone!) to review it
- [S-Waiting-on-Author]: A change or a response from the author is needed
- [S-Won't-Fix]: This will not be worked on

## Code Style & Review Process

See the [Style Guide](style.md).

## Cookbook

### CI

We use GitHub Actions for CI.
Most of the things, including formatting, are checked by `cargo test`.
If `cargo test` passes locally, that is a good sign that CI will be green as well.
The only exception is that some long-running tests are skipped locally by default.
Use `env RUN_SLOW_TESTS=1 cargo test` to run the full suite.

We use bors to enforce the [not rocket science](https://graydon2.dreamwidth.org/1597.html) rule.

### Launching wgsl-analyzer

Debugging the language server can be tricky.
LSP is rather chatty, so driving it from the command line is not really feasible, driving it via VS Code requires interacting with two processes.

For this reason, the best way to see how `wgsl-analyzer` works is to **find a relevant test and execute it**.
VS Code & Emacs include an action for running a single test.

Launching a VS Code instance with a locally built language server is also possible.
There is **"Run Extension (Debug Build)"** launch configuration for this in VS Code.

In general, I use one of the following workflows for fixing bugs and implementing features:

If the problem concerns only internal parts of `wgsl-analyzer` (i.e. I do not need to touch the `wgsl-analyzer` crate or TypeScript code), there is a unit-test for it.
So, I use **wgsl-analyzer: Run** action in VS Code to run this single test, and then just do printf-driven development/debugging.
As a sanity check after I am done, I use `cargo xtask install --server` and **Reload Window** action in VS Code to verify that the thing works as I expect.

If the problem concerns only the VS Code extension, I use **Run Installed Extension** launch configuration from `launch.json`.
Notably, this uses the usual `wgsl-analyzer` binary from `PATH`.
For this, it is important to have the following in your `settings.json` file:

```json
{
    "wgsl-analyzer.server.path": "wgsl-analyzer"
}
```

After I am done with the fix, I use `cargo xtask install --client` to try the new extension for real.

If I need to fix something in the `wgsl-analyzer` crate, I feel sad because it is on the boundary between the two processes, and working there is slow.
I usually just `cargo xtask install --server` and poke changes from my live environment.
Note that this uses `--release`, which is usually faster overall, because loading stdlib into debug version of `wgsl-analyzer` takes a lot of time.
To speed things up, sometimes I open a temporary hello-world project which has `"wgsl-analyzer.cargo.sysroot": null` in `.code/settings.json`.
This flag causes `wgsl-analyzer` to skip loading the sysroot, which greatly reduces the amount of things `wgsl-analyzer` needs to do, and makes `printf`'s more useful.
Note that you should only use the `eprint!` family of macros for debugging: stdout is used for LSP communication, and `print!` would break it.

If I need to fix something simultaneously in the server and in the client, I feel even more sad.
I do not have a specific workflow for this case.

Additionally, I use `cargo run --release -p wgsl-analyzer -- analysis-stats path/to/some/wgsl/code` to run a batch analysis.
This is primarily useful for performance optimizations, or for bug minimization.

### TypeScript Tests

If you change files under `editors/code` and would like to run the tests and linter, install npm and run:

```bash
cd editors/code
npm ci
npm run lint
```

### How to

<!-- TODO: make wgsl-analyzer pulls as examples -->

- ... add an assist? [#7535](https://github.com/rust-lang/rust-analyzer/pull/7535)
- ... add a new protocol extension? [#4569](https://github.com/rust-lang/rust-analyzer/pull/4569)
- ... add a new configuration option? [#7451](https://github.com/rust-lang/rust-analyzer/pull/7451)
- ... add a new completion? [#6964](https://github.com/rust-lang/rust-analyzer/pull/6964)
- ... allow new syntax in the parser? [#7338](https://github.com/rust-lang/rust-analyzer/pull/7338)

### Logging

Logging is done by both `wgsl-analyzer` and VS Code, so it might be tricky to figure out where logs go.

Inside wgsl-analyzer, we use the [`tracing`](https://docs.rs/tracing/) crate for logging, and [`tracing-subscriber`](https://docs.rs/tracing-subscriber) for logging frontend.
By default, log goes to stderr, but the stderr itself is processed by VS Code.
`--log-file <PATH>` CLI argument allows logging to file.
Setting the `WA_LOG_FILE=<PATH>` environment variable will also log to file, it will also override `--log-file`.

To see stderr in the running VS Code instance, go to the "Output" tab of the panel and select `wgsl-analyzer`.
This shows `eprintln!` as well.
Note that `stdout` is used for the actual protocol, so `println!` will break things.

To log all communication between the server and the client, there are two choices:

- You can log on the server side, by running something like

  ```bash
  env WA_LOG=lsp_server=debug code .
  ```

- You can log on the client side, by the `wgsl-analyzer: Toggle LSP Logs` command or enabling `"wgsl-analyzer.trace.server": "verbose"` workspace setting.
  These logs are shown in a separate tab in the output and could be used with LSP inspector.
  Kudos to [@DJMcNab](https://github.com/DJMcNab) for setting this awesome infra up!

There are also several VS Code commands which might be of interest:

- `wgsl-analyzer: Status` shows some memory-usage statistics.

- `wgsl-analyzer: View Hir` shows the HIR expressions within the function containing the cursor.

- If `wgsl-analyzer.showSyntaxTree` is enabled in settings, `WGSL Syntax Tree: Focus on WGSL Syntax Tree View` shows the syntax tree of the current file.

  You can click on nodes in the WGSL editor to go to the corresponding syntax node.

  You can click on `Reveal Syntax Element` next to a syntax node to go to the corresponding WGSL code and highlight the proper text range.

  If you trigger Go to Definition in the inspected WGSL source file, the syntax tree view should scroll to and select the appropriate syntax node token.

  You can click on `Copy` next to a syntax node to copy a text representation of the node.

  ![demo](https://github.com/user-attachments/assets/2d20ae87-0abf-495f-bee8-54aa2494a00d)

### Profiling

We have a built-in hierarchical profiler, you can enable it by using `WA_PROFILE` env-var:

```bash
WA_PROFILE=*             // dump everything
WA_PROFILE=foo|bar|baz   // enabled only selected entries
WA_PROFILE=*@3>10        // dump everything, up to depth 3, if it takes more than 10 ms
```

Some `wgsl-analyzer` contributors have `export WA_PROFILE='*>10'` in my shell profile.

For machine-readable JSON output, we have the `WA_PROFILE_JSON` env variable.
We support filtering only by span name:

```bash
WA_PROFILE=* // dump everything
WA_PROFILE_JSON="vfs_load|parallel_prime_caches|discover_command" // dump selected spans
```

We also have a "counting" profiler which counts number of instances of popular structs.
It is enabled by `WA_COUNT=1`.

To measure time for from-scratch analysis, use something like this:

```bash
cargo run --release -p wgsl-analyzer -- analysis-stats ../chalk/
```

For measuring time of incremental analysis, use either of these:

```bash
cargo run --release -p wgsl-analyzer -- analysis-bench ../chalk/ --highlight ../chalk/chalk-engine/src/logic.rs
cargo run --release -p wgsl-analyzer -- analysis-bench ../chalk/ --complete ../chalk/chalk-engine/src/logic.rs:94:0
```

Look for `fn benchmark_xxx` tests for a quick way to reproduce performance problems.

### Release Process

Release process is handled by `release`, `dist`, `publish-release-notes` and `promote` xtasks, `release` being the main one.

`release` assumes that you have checkouts of `wgsl-analyzer`, `wgsl-analyzer.github.io`, and `wgsl-lang/wgsl` in the same directory:

```bash
./wgsl-analyzer
./wgsl-analyzer.github.io
./wgsl-wgsl-analyzer  # Note the name!
```

The remote for `wgsl-analyzer` must be called `upstream` (I use `origin` to point to my fork).
In addition, for `xtask promote` (see below), `wgsl-wgsl-analyzer` must have a `wgsl-analyzer` remote pointing to this repository on GitHub.

`release` calls the GitHub API calls to scrape pull request comments and categorize them in the changelog.
This step uses the `curl` and `jq` applications, which need to be available in `PATH`.
Finally, you need to obtain a GitHub personal access token and set the `GITHUB_TOKEN` environment variable.

Release steps:

1. Set the `GITHUB_TOKEN` environment variable.
2. Inside wgsl-analyzer, run `cargo xtask release`.
   This will:
   - checkout the `release` branch
   - reset it to `upstream/nightly`
   - push it to `upstream`.
    This triggers GitHub Actions which:
      - runs `cargo xtask dist` to package binaries and VS Code extension
      - makes a GitHub release
      - publishes the VS Code extension to the marketplace
      - call the GitHub API for PR details
      - create a new changelog in `wgsl-analyzer.github.io`
3. While the release is in progress, fill in the changelog.
4. Commit & push the changelog.
5. Run `cargo xtask publish-release-notes <CHANGELOG>` -- this will convert the changelog entry in AsciiDoc to Markdown and update the body of GitHub Releases entry.

Note: besides the `wgsl-wgsl-analyzer` clone, the Josh cache (stored under `~/.cache/wgsl-analyzer-josh`) will contain a bare clone of `wgsl-lang/wgsl`.
This currently takes about 3.5 GB.

This [HackMD](https://hackmd.io/7pOuxnkdQDaL1Y1FQr65xg) has details about how `josh` syncs work.

If the GitHub Actions release fails because of a transient problem like a timeout, you can re-run the job from the Actions console.
If it fails because of something that needs to be fixed, remove the release tag (if needed), fix the problem, then start over.
Make sure to remove the new changelog post created when running `cargo xtask release` a second time.

We release "nightly" every night automatically and promote the latest nightly to "stable" manually, every week.

We do not do "patch" releases, unless something truly egregious comes up.
To do a patch release, cherry-pick the fix on top of the current `release` branch and push the branch.
There is no need to write a changelog for a patch release, it is OK to include the notes about the fix into the next weekly one.
Note: we tag releases by dates, releasing a patch release on the same day should work (by overwriting a tag), but I am not 100% sure.

### Permissions

## Triage Team

We have a dedicated triage team that helps manage issues and pull requests on GitHub.
Members of the triage team have permissions to:

- Label issues and pull requests
- Close and reopen issues
- Assign issues and PRs to milestones

This team plays a crucial role in ensuring that the project remains organized and that contributions are properly reviewed and addressed.
