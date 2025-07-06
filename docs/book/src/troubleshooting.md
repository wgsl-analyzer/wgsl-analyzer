# Troubleshooting

Start with looking at the `wgsl-analyzer` version.
Try the **wgsl-analyzer: Show WA Version** in VS Code (using **Command Palette** feature.
It is typically activated by <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>P</kbd>) or `wgsl-analyzer --version` in the command line. <!-- spellchecker:disable-line -->
If the date is more than a week ago, it is better to update your installation of wgsl-analyzer to the newest version.

The next thing to check would be panic messages in `wgsl-analyzer`'s log.
Log messages are printed to stderr, in VS Code you can see them in the `Output > wgsl-analyzer Language Server` tab of the panel.
To see more logs, set the `WA_LOG=info` environment variable, this can be done either by setting the environment variable manually or by using `wgsl-analyzer.server.extraEnv`.
Note that both of these approaches require the server to be restarted.

To fully capture LSP messages between the editor and the server, run the `wgsl-analyzer: Toggle LSP Logs` command and check `Output > wgsl-analyzer Language Server Trace`.

The root cause for many "nothing works" problems is that `wgsl-analyzer` fails to understand the project structure.
To debug that, first note the `wgsl-analyzer` section in the status bar.
If it has an error icon and red, that is the problem (hover will have somewhat helpful error message).
**wgsl-analyzer: Status** prints dependency information for the current file.
Finally, `WA_LOG=project_model=debug` enables verbose logs during project loading.

If wgsl-analyzer outright crashes, try running `wgsl-analyzer analysis-stats /path/to/project/directory/` on the command line.
This command type checks the whole project in batch mode bypassing LSP machinery.

When filing issues, it is useful (but not necessary) to try to minimize examples.

<!--
An ideal bug reproduction looks like this:

```bash
$ git clone https://github.com/username/repo.git && cd repo && git switch --detach commit-hash
$ wgsl-analyzer --version
wgsl-analyzer dd12184e4 2021-05-08 dev
$ wgsl-analyzer analysis-stats .
💀 💀 💀
```

It is especially useful when the `repo` does not use external crates or the standard library.
-->

If you want to go as far as to modify the source code to debug the problem, be sure to take a look at the [dev docs](https://github.com/wgsl-analyzer/wgsl-analyzer/tree/master/docs/dev)!
