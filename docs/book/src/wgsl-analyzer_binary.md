# wgsl-analyzer Binary

Text editors require the `wgsl-analyzer` binary to be in `$PATH`.
You can download pre-built binaries from the [releases] page.
You will need to uncompress and rename the binary for your platform.

For example,  on Mac OS:

1. extract `wgsl-analyzer-aarch64-apple-darwin.gz` to `wgsl-analyzer`
2. make it executable
3. move it into a directory in your `$PATH`

[releases]: <https://github.com/wgsl-analyzer/wgsl-analyzer/releases>

On Linux, to install the `wgsl-analyzer` binary into `~/.local/bin`, these commands should work:

```bash
mkdir -p ~/.local/bin
curl -L https://github.com/wgsl-analyzer/wgsl-analyzer/releases/latest/download/wgsl-analyzer-x86_64-unknown-linux-gnu.gz | gunzip -c - > ~/.local/bin/wgsl-analyzer
chmod +x ~/.local/bin/wgsl-analyzer
```

Make sure that `~/.local/bin` is listed in the `$PATH` variable and use the appropriate URL if you are not on a `x86-64` system.

You do not have to use `~/.local/bin`, any other path like `~/.cargo/bin` or `/usr/local/bin` will work just as well.

Alternatively, you can install it from source using the command below.
You will need the latest stable version of the Rust toolchain.

```bash
git clone https://github.com/wgsl-analyzer/wgsl-analyzer.git && cd wgsl-analyzer
cargo xtask install --server
```

If your editor cannot find the binary even though the binary is on your `$PATH`, the likely explanation is that it does not see the same `$PATH` as the shell.
On Unix, running the editor from a shell or changing the `.desktop` file to set the environment should help.

## Arch Linux

The `wgsl-analyzer` binary can be installed from the repos or AUR (Arch User Repository):

- [`wgsl-analyzer`](https://www.archlinux.org/packages/extra/x86_64/wgsl-analyzer) (built from latest tagged source)

- [`wgsl-analyzer-git`](https://aur.archlinux.org/packages/wgsl-analyzer-git) (latest Git version)

Install it with `pacman`, for example:

```bash
pacman -S wgsl-analyzer
```

## Gentoo Linux

<!-- TODO make this real -->

## macOS

<!-- TODO publish to brew -->

The `wgsl-analyzer` binary can be installed via [Homebrew](https://brew.sh).

```zsh
brew install wgsl-analyzer
```

## Windows

<!-- TODO publish to winget -->
<!-- TODO publish to choco -->

The `wgsl-analyzer` binary can be installed via [WinGet](https://github.com/microsoft/winget-cli) or [Chocolatey](https://github.com/chocolatey/choco).

```powershell
winget install wgsl-analyzer
```

```powershell
choco install wgsl-analyzer
```
