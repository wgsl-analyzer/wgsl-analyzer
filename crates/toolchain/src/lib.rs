//! Discovery of `cargo` & `rustc` executables.

use std::{
    env,
    ffi::OsStr,
    iter,
    path::{Path, PathBuf},
    process::Command,
};

use camino::{Utf8Path, Utf8PathBuf};

#[derive(Copy, Clone)]
pub enum Tool {
    Cargo,
    Rustc,
    Rustup,
    Rustfmt,
}

impl Tool {
    #[must_use]
    pub fn proxy(self) -> Option<Utf8PathBuf> {
        cargo_proxy(self.name())
    }

    /// Return a `PathBuf` to use for the given executable.
    ///
    /// The current implementation checks three places for an executable to use:
    /// 1) `$CARGO_HOME/bin/<executable_name>`
    ///    where `$CARGO_HOME` defaults to ~/.cargo (see <https://doc.rust-lang.org/cargo/guide/cargo-home.html>)
    ///    example: for cargo, this tries `$CARGO_HOME/bin/cargo`, or ~/.cargo/bin/cargo if `$CARGO_HOME` is unset.
    ///    It seems that this is a reasonable place to try for cargo, rustc, and rustup
    /// 2) Appropriate environment variable (erroring if this is set but not a usable executable)
    ///    example: for cargo, this checks $CARGO environment variable; for rustc, $RUSTC; etc
    /// 3) $PATH/`<executable_name>`
    ///    example: for cargo, this tries all paths in $PATH with appended `cargo`, returning the
    ///    first that exists
    /// 4) If all else fails, we just try to use the executable name directly
    #[must_use]
    pub fn prefer_proxy(self) -> Utf8PathBuf {
        invoke(
            &[cargo_proxy, lookup_as_env_var, lookup_in_path],
            self.name(),
        )
    }

    /// Return a `PathBuf` to use for the given executable.
    ///
    /// The current implementation checks three places for an executable to use:
    /// 1) Appropriate environment variable (erroring if this is set but not a usable executable)
    ///    example: for cargo, this checks $CARGO environment variable; for rustc, $RUSTC; etc
    /// 2) $PATH/`<executable_name>`
    ///    example: for cargo, this tries all paths in $PATH with appended `cargo`, returning the
    ///    first that exists
    /// 3) `$CARGO_HOME/bin/<executable_name>`
    ///    where `$CARGO_HOME` defaults to ~/.cargo (see <https://doc.rust-lang.org/cargo/guide/cargo-home.html>)
    ///    example: for cargo, this tries `$CARGO_HOME/bin/cargo`, or ~/.cargo/bin/cargo if `$CARGO_HOME` is unset.
    ///    It seems that this is a reasonable place to try for cargo, rustc, and rustup
    /// 4) If all else fails, we just try to use the executable name directly
    #[must_use]
    pub fn path(self) -> Utf8PathBuf {
        invoke(
            &[lookup_as_env_var, lookup_in_path, cargo_proxy],
            self.name(),
        )
    }

    #[must_use]
    pub fn path_in(
        self,
        path: &Utf8Path,
    ) -> Option<Utf8PathBuf> {
        probe_for_binary(path.join(self.name()))
    }

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Cargo => "cargo",
            Self::Rustc => "rustc",
            Self::Rustup => "rustup",
            Self::Rustfmt => "rustfmt",
        }
    }
}

#[must_use]
#[expect(
    clippy::disallowed_types,
    reason = "generic parameter allows for FxHashMap"
)]
pub fn command<Hashy, OsStringy: AsRef<OsStr>, Pathy: AsRef<Path>>(
    command: OsStringy,
    working_directory: Pathy,
    extra_env: &std::collections::HashMap<String, Option<String>, Hashy>,
) -> Command {
    #[expect(clippy::disallowed_methods, reason = "we are `toolchain::command`")]
    let mut command = Command::new(command);
    command.current_dir(working_directory);
    for env in extra_env {
        match env {
            (key, Some(val)) => command.env(key, val),
            (key, None) => command.env_remove(key),
        };
    }
    command
}

fn invoke(
    list: &[fn(&str) -> Option<Utf8PathBuf>],
    executable: &str,
) -> Utf8PathBuf {
    list.iter()
        .find_map(|getter| getter(executable))
        .unwrap_or_else(|| executable.into())
}

/// Looks up the binary as its SCREAMING upper case in the env variables.
#[expect(
    clippy::disallowed_types,
    reason = "The value of the environment variable may be a relative path"
)]
fn lookup_as_env_var(executable_name: &str) -> Option<Utf8PathBuf> {
    env::var_os(executable_name.to_ascii_uppercase())
        .map(PathBuf::from)
        .map(Utf8PathBuf::try_from)
        .and_then(Result::ok)
}

/// Looks up the binary in the cargo home directory if it exists.
fn cargo_proxy(executable_name: &str) -> Option<Utf8PathBuf> {
    let mut path = get_cargo_home()?;
    path.push("bin");
    path.push(executable_name);
    probe_for_binary(path)
}

#[expect(
    clippy::disallowed_types,
    reason = "The value of the environment variable may be a relative path"
)]
fn get_cargo_home() -> Option<Utf8PathBuf> {
    if let Some(path) = env::var_os("CARGO_HOME") {
        return Utf8PathBuf::try_from(PathBuf::from(path)).ok();
    }

    if let Some(mut path) = home::home_dir() {
        path.push(".cargo");
        return Utf8PathBuf::try_from(path).ok();
    }

    None
}

fn lookup_in_path(executable_name: &str) -> Option<Utf8PathBuf> {
    let paths = env::var_os("PATH").unwrap_or_default();
    env::split_paths(&paths)
        .map(|path| path.join(executable_name))
        .map(Utf8PathBuf::try_from)
        .filter_map(Result::ok)
        .find_map(probe_for_binary)
}

#[must_use]
pub fn probe_for_binary(path: Utf8PathBuf) -> Option<Utf8PathBuf> {
    let with_extension = match env::consts::EXE_EXTENSION {
        "" => None,
        extension => Some(path.with_extension(extension)),
    };
    iter::once(path)
        .chain(with_extension)
        .find(|path| path.is_file())
}
