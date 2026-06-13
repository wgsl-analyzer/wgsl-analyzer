use std::{
    fmt::Display,
    io::Write as _,
    path::PathBuf,
    process::{Command, Stdio},
    range::Range,
};

use base_db::{EditionedFileId, SourceDatabase as _};
use hir::diagnostics::{AnyDiagnostic, Severity};
use ide_db::{FxHashMap, LineIndexDatabase as _, RootDatabase};
use line_index::{LineCol, LineIndex};
use rowan::{TextRange, TextSize};
use serde::Deserialize;
use vfs::AbsPathBuf;

use crate::DiagnosticsConfig;

pub(crate) fn tint_diagnostics(
    database: &RootDatabase,
    file_id: EditionedFileId,
    config: &DiagnosticsConfig,
    accumulator: &mut Vec<AnyDiagnostic>,
) {
    let raw_file_id = file_id.file_id(database);
    let source: &str = database.file_text(raw_file_id).text(database);
    let full_range = TextRange::up_to(TextSize::of(source));
    let line_index = database.line_index(raw_file_id);

    let tint_results = TintCommand {
        tint_path: config.tint_path.clone(),
    }
    .execute(source);

    let diagnostics = match tint_results {
        Ok(diagnostics) => diagnostics,
        Err(error) => {
            accumulator.push(AnyDiagnostic::TintValidationError {
                file_id,
                range: full_range,
                message: error.to_string(),
                severity: Severity::Error,
            });
            return;
        },
    };

    for diagnostic in diagnostics {
        let severity = match diagnostic.severity {
            TintDiagnosticSeverity::Error => Severity::Error,
            TintDiagnosticSeverity::Warning => Severity::Warning,
            TintDiagnosticSeverity::Note => Severity::Information,
        };
        let range = diagnostic.range.to_range(&line_index).unwrap_or(full_range);

        accumulator.push(AnyDiagnostic::TintValidationError {
            file_id,
            range,
            message: diagnostic.message,
            severity,
        });
    }
}

#[derive(Debug, Clone)]
pub struct TintCommand {
    pub tint_path: Option<AbsPathBuf>,
}

impl TintCommand {
    fn command(&self) -> Command {
        let path = self
            .tint_path
            .as_ref()
            .map_or_else(|| PathBuf::from("tint"), PathBuf::from);
        // let mut cmd = toolchain::command(path,,&FxHashMap::default());
        let mut cmd = Command::new(path);
        cmd.args([
            "--parse-only",
            "--input-format",
            "wgsl",
            "--diagnostics-format",
            "json",
        ]);
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::null());
        cmd.stderr(Stdio::piped());
        cmd
    }

    pub fn execute(
        &self,
        source: &str,
    ) -> Result<Vec<TintDiagnostic>, TintCommandError> {
        let mut child = self.command().spawn().map_err(TintCommandError::Io)?;
        let mut stdin = child.stdin.take().unwrap();
        let output = std::thread::scope(move |scope| {
            scope.spawn(move || {
                stdin.write_all(source.as_bytes()).unwrap();
            });
            child.wait_with_output().map_err(TintCommandError::Io)
        })?;

        // Tint seems to terminate with exit code 1 and output nothing when the program is correct
        if output.status.success() || output.stderr.len() == 0 {
            Ok(Vec::new())
        } else {
            parse(&output.stderr)
        }
    }
}

fn parse(data: &[u8]) -> Result<Vec<TintDiagnostic>, TintCommandError> {
    serde_json::from_slice::<Vec<TintDiagnostic>>(data).map_err(TintCommandError::Deserialize)
}

#[derive(Deserialize, Debug)]
pub struct TintDiagnostic {
    pub severity: TintDiagnosticSeverity,
    pub message: String,
    pub range: TintDiagnosticRange,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct TintDiagnosticRange {
    pub start: TintDiagnosticPosition,
    pub end: TintDiagnosticPosition,
}

impl TintDiagnosticRange {
    fn to_range(
        self,
        line_index: &LineIndex,
    ) -> Option<TextRange> {
        // Assuming that the end position could reasonably be missing
        let start = self.start.to_position(line_index)?;
        Some(match self.end.to_position(line_index) {
            Some(end) => TextRange::new(start, end),
            None => TextRange::at(start, TextSize::new(1)),
        })
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct TintDiagnosticPosition {
    pub line: u32,
    pub column: u32,
}

impl TintDiagnosticPosition {
    fn to_position(
        self,
        line_index: &LineIndex,
    ) -> Option<TextSize> {
        // Assuming that the column could reasonably be missing
        line_index.offset(LineCol {
            line: self.line.checked_sub(1)?,
            col: self.column.saturating_sub(1),
        })
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TintDiagnosticSeverity {
    Error,
    Warning,
    Note,
}

#[derive(Debug)]
pub enum TintCommandError {
    Io(std::io::Error),
    Tint { stderr: String },
    Deserialize(serde_json::Error),
}

impl Display for TintCommandError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(f, "failed to execute Tint: {error}"),
            Self::Tint { stderr } => write!(f, "Tint error: {stderr}"),
            Self::Deserialize(error) => {
                write!(f, "could not deserialize Tint output: {error}")
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use expect_test::{Expect, expect};

    use super::*;
    use core::assert_matches;
    use std::fmt::Write as _;

    #[expect(clippy::needless_pass_by_value, reason = "Matches expect! macro")]
    #[expect(clippy::use_debug, reason = "useful in tests")]
    fn check_tint_diagnostics(
        source_code: &str,
        tint_response: &str,
        expect: Expect,
    ) {
        let line_index = LineIndex::new(source_code);

        let diagnostics = parse(tint_response.as_bytes()).unwrap();

        let mut actual = String::new();

        for TintDiagnostic {
            severity,
            message,
            range,
        } in diagnostics
        {
            let severity_text = match severity {
                TintDiagnosticSeverity::Error => "Error",
                TintDiagnosticSeverity::Warning => "Warning",
                TintDiagnosticSeverity::Note => "Note",
            };
            let byte_range = range.to_range(&line_index).unwrap_or(TextRange::default());
            writeln!(actual, "{byte_range:?} {severity_text}: {message}",);
        }

        expect.assert_eq(&actual);
    }

    #[test]
    fn parse_tint() {
        check_tint_diagnostics(
            "fn foo() {}\nconst foo = foo;",
            r#"[
  {
    "severity": "error",
    "message": "redeclaration of 'foo'",
    "range": {
      "start": { "line": 2, "column": 1 },
      "end": { "line": 2, "column": 16 }
    }
  },
  {
    "severity": "note",
    "message": "'foo' previously declared here",
    "range": {
      "start": { "line": 1, "column": 1 },
      "end": { "line": 1, "column": 12 }
    }
  }
]"#,
            expect![[r#"
                12..27 Error: redeclaration of 'foo'
                0..11 Note: 'foo' previously declared here
            "#]],
        );
    }
}
