# VS Code

This is the best supported editor at the moment.
The `wgsl-analyzer` plugin for VS Code is maintained [in-tree].

You can install the latest release of the plugin from [the marketplace].

[in-tree]: <https://github.com/wgsl-analyzer/wgsl-analyzer/tree/master/editors/code>
[the marketplace]: <https://marketplace.visualstudio.com/items?itemName=wgsl-analyzer.wgsl-analyzer>

The server binary is stored in the extension install directory, which starts with `wgsl-analyzer.wgsl-analyzer-` and is located in:

- Linux: `~/.vscode/extensions`
- Linux (Remote, such as WSL): `~/.vscode-server/extensions`
- macOS: `~/.vscode/extensions`
- Windows: `%USERPROFILE%\.vscode\extensions`

As an exception, on NixOS, the extension makes a copy of the server and stores it in `~/.config/Code/User/globalStorage/wgsl-analyzer.wgsl-analyzer`.

Note that we only support the two most recent versions of VS Code.

## Updates

The extension will be updated automatically as new versions become available.
It will ask your permission to download the matching language server version binary if needed.

### Nightly

We ship nightly releases for VS Code. To help us out by testing the newest code, you can enable pre-release versions in the Code extension page.

## Manual installation

Alternatively, download a VSIX corresponding to your platform from the [releases] page.

[releases]: <https://github.com/wgsl-analyzer/wgsl-analyzer/releases>

Install the extension with the `Extensions: Install from VSIX` command within VS Code, or from the command line via:

```bash
code --install-extension /path/to/wgsl-analyzer.vsix
```

If you are running an unsupported platform, you can install `wgsl-analyzer-no-server.vsix` and compile or obtain a server binary.
Copy the server anywhere, then add the path to your `settings.json`.

For example:

```json
{ "wgsl-analyzer.server.path": "~/.local/bin/wgsl-analyzer-linux" }
```

## Building From Source

Both the server and the Code plugin can be installed from source:

```bash
git clone https://github.com/wgsl-analyzer/wgsl-analyzer.git && cd wgsl-analyzer
cargo xtask install
```

You will need [Cargo], [Node.js] (matching a supported version of VS Code) and [npm] for this.

[Cargo]: <https://doc.rust-lang.org/cargo/getting-started/installation.html>
[Node.js]: <https://nodejs.org/>
[npm]: <https://www.npmjs.com/get-npm>

Note that installing via `xtask install` does not work for VS Code Remote.
Instead, you will need to install the `.vsix` manually.

If you are not using Code, you can compile and install only the LSP
server:

```bash
cargo xtask install --server
```

Make sure that `.cargo/bin` is in `$PATH` and precedes paths where `wgsl-analyzer` may also be installed.

## VS Code or VSCodium in Flatpak

Setting up `wgsl-analyzer` with a Flatpak version of Code is not trivial because of the Flatpak sandbox. This prevents access to files you might want to import.
