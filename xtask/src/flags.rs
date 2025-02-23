use std::{fmt, str::FromStr};

use crate::install::{ClientOptions, ServerOptions};

xflags::xflags! {
    src "./src/flags.rs"

    /// Run custom build command.
    cmd xtask {

        /// Install wgsl-analyzer server or editor plugin.
        cmd install {
            /// Install only VS Code plugin.
            optional --client
            /// One of `code`, `code-exploration`, `code-insiders`, `codium`, or `code-oss`.
            optional --code-bin name: String

            /// Install only the language server.
            optional --server
            /// Use mimalloc allocator for server.
            optional --mimalloc
            /// Use jemalloc allocator for server.
            optional --jemalloc

            /// build in release with debug info set to 2.
            optional --dev-rel
        }

        cmd fuzz-tests {}

        cmd release {
            optional --dry-run
        }

        cmd dist {
            /// Use mimalloc allocator for server
            optional --mimalloc
            /// Use jemalloc allocator for server
            optional --jemalloc
            optional --client-patch-version version: String
        }
        /// Read a changelog AsciiDoc file and update the GitHub Releases entry in Markdown.
        cmd publish-release-notes {
            /// Only run conversion and show the result.
            optional --dry-run
            /// Target changelog file.
            required changelog: String
        }

        /// Builds a benchmark version of wgsl-analyzer and puts it into `./target`.
        cmd bb {
            required suffix: String
        }

        cmd tidy {}
    }
}

// generated start
// The following code is generated by `xflags` macro.
// Run `env UPDATE_XFLAGS=1 cargo build` to regenerate.
#[derive(Debug)]
pub struct Xtask {
    pub subcommand: XtaskCmd,
}

#[derive(Debug)]
pub enum XtaskCmd {
    Install(Install),
    FuzzTests(FuzzTests),
    Release(Release),
    Dist(Dist),
    PublishReleaseNotes(PublishReleaseNotes),
    Bb(Bb),
    Tidy(Tidy),
}

#[derive(Debug)]
pub struct Tidy;

#[derive(Debug)]
pub struct Install {
    pub client: bool,
    pub code_bin: Option<String>,
    pub server: bool,
    pub mimalloc: bool,
    pub jemalloc: bool,
    pub dev_rel: bool,
}

#[derive(Debug)]
pub struct FuzzTests;

#[derive(Debug)]
pub struct Release {
    pub dry_run: bool,
}

#[derive(Debug)]
pub struct RustcPull {
    pub commit: Option<String>,
}

#[derive(Debug)]
pub struct RustcPush {
    pub rust_path: String,
    pub rust_fork: String,
    pub branch: Option<String>,
}

#[derive(Debug)]
pub struct Dist {
    pub mimalloc: bool,
    pub jemalloc: bool,
    pub client_patch_version: Option<String>,
}

#[derive(Debug)]
pub struct PublishReleaseNotes {
    pub changelog: String,

    pub dry_run: bool,
}

#[derive(Debug)]
pub struct Metrics {
    pub measurement_type: Option<MeasurementType>,
}

#[derive(Debug)]
pub struct Bb {
    pub suffix: String,
}

#[derive(Debug)]
pub struct Codegen {
    pub codegen_type: Option<CodegenType>,

    pub check: bool,
}

impl Xtask {
    pub fn from_env_or_exit() -> Self {
        Self::from_env_or_exit_()
    }

    pub fn from_env() -> xflags::Result<Self> {
        Self::from_env_()
    }

    pub fn from_vec(args: Vec<std::ffi::OsString>) -> xflags::Result<Self> {
        Self::from_vec_(args)
    }
}
// generated end

#[derive(Debug, Default)]
pub enum CodegenType {
    #[default]
    All,
    Grammar,
    AssistsDocTests,
    DiagnosticsDocs,
    LintDefinitions,
    ParserTests,
    FeatureDocs,
}

impl fmt::Display for CodegenType {
    #[expect(clippy::min_ident_chars, reason = "trait impl")]
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Grammar => write!(f, "grammar"),
            Self::AssistsDocTests => write!(f, "assists-doc-tests"),
            Self::DiagnosticsDocs => write!(f, "diagnostics-docs"),
            Self::LintDefinitions => write!(f, "lint-definitions"),
            Self::ParserTests => write!(f, "parser-tests"),
            Self::FeatureDocs => write!(f, "feature-docs"),
        }
    }
}

impl FromStr for CodegenType {
    type Err = String;
    #[expect(clippy::min_ident_chars, reason = "trait impl")]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" => Ok(Self::All),
            "grammar" => Ok(Self::Grammar),
            "assists-doc-tests" => Ok(Self::AssistsDocTests),
            "diagnostics-docs" => Ok(Self::DiagnosticsDocs),
            "lint-definitions" => Ok(Self::LintDefinitions),
            "parser-tests" => Ok(Self::ParserTests),
            "feature-docs" => Ok(Self::FeatureDocs),
            _ => Err("Invalid option".to_owned()),
        }
    }
}

#[derive(Debug)]
pub enum MeasurementType {
    Build,
    RustcTests,
    AnalyzeSelf,
    AnalyzeRipgrep,
    AnalyzeWebRender,
    AnalyzeDiesel,
    AnalyzeHyper,
}

impl FromStr for MeasurementType {
    type Err = String;
    #[expect(clippy::min_ident_chars, reason = "trait impl")]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "build" => Ok(Self::Build),
            "rustc_tests" => Ok(Self::RustcTests),
            "self" => Ok(Self::AnalyzeSelf),
            "ripgrep-13.0.0" => Ok(Self::AnalyzeRipgrep),
            "webrender-2022" => Ok(Self::AnalyzeWebRender),
            "diesel-1.4.8" => Ok(Self::AnalyzeDiesel),
            "hyper-0.14.18" => Ok(Self::AnalyzeHyper),
            _ => Err("Invalid option".to_owned()),
        }
    }
}
impl AsRef<str> for MeasurementType {
    fn as_ref(&self) -> &str {
        match self {
            Self::Build => "build",
            Self::RustcTests => "rustc_tests",
            Self::AnalyzeSelf => "self",
            Self::AnalyzeRipgrep => "ripgrep-13.0.0",
            Self::AnalyzeWebRender => "webrender-2022",
            Self::AnalyzeDiesel => "diesel-1.4.8",
            Self::AnalyzeHyper => "hyper-0.14.18",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Malloc {
    System,
    Mimalloc,
    Jemalloc,
}

impl Malloc {
    pub(crate) fn to_features(self) -> &'static [&'static str] {
        match self {
            Self::System => &[][..],
            Self::Mimalloc => &["--features", "mimalloc"],
            Self::Jemalloc => &["--features", "jemalloc"],
        }
    }
}

impl Install {
    pub(crate) const fn server(&self) -> Option<ServerOptions> {
        if self.client && !self.server {
            return None;
        }
        let malloc = if self.mimalloc {
            Malloc::Mimalloc
        } else if self.jemalloc {
            Malloc::Jemalloc
        } else {
            Malloc::System
        };
        Some(ServerOptions {
            malloc,
            dev_rel: self.dev_rel,
        })
    }

    pub(crate) fn client(&self) -> Option<ClientOptions> {
        if self.server && !self.client {
            return None;
        }
        Some(ClientOptions {
            code_binary: self.code_bin.clone(),
        })
    }
}

impl Dist {
    pub(crate) const fn allocator(&self) -> Malloc {
        if self.mimalloc {
            Malloc::Mimalloc
        } else if self.jemalloc {
            Malloc::Jemalloc
        } else {
            Malloc::System
        }
    }
}
