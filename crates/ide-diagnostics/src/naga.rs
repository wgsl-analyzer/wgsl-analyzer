mod naga27;
mod naga28;
mod naga29;
mod naga_main;

use std::{error, range::Range};

use base_db::{EditionedFileId, FileRange};
use hir::{HirDatabase, diagnostics::AnyDiagnostic};
pub(crate) use naga_main::NagaMain;
pub(crate) use naga27::Naga27;
pub(crate) use naga28::Naga28;
pub(crate) use naga29::Naga29;
use rowan::{TextRange, TextSize};

use crate::DiagnosticsConfig;

pub(crate) trait Naga {
    type Module;
    type ParseError: NagaError;
    type ValidationError: NagaError;

    fn parse(source: &str) -> Result<Self::Module, Self::ParseError>;
    fn validate(module: &Self::Module) -> Result<(), Self::ValidationError>;
}

pub(crate) trait NagaError: error::Error {
    fn spans(&self) -> Box<dyn Iterator<Item = (Option<Range<usize>>, String)> + '_>;
    fn location(&self) -> Option<Range<usize>>;
}

fn emit<Error>(
    database: &dyn HirDatabase,
    error: &Error,
    file_id: EditionedFileId,
    full_range: TextRange,
    accumulator: &mut Vec<AnyDiagnostic>,
) where
    Error: NagaError,
{
    let message = error_message_cause_chain(&error);
    let original_range = |range: Range<usize>| {
        TextRange::new(
            TextSize::from(u32::try_from(range.start).expect("indexes are small numbers")),
            TextSize::from(u32::try_from(range.end).expect("indexes are small numbers")),
        )
    };
    let location = error.location().map_or(full_range, original_range);

    let spans = error.spans().filter_map(|(span, label)| {
        let range = original_range(span?);
        Some((range, label))
    });

    let related: Vec<_> = spans
        .map(|(range, message)| {
            (
                message,
                FileRange {
                    range,
                    file_id: file_id.file_id(database),
                },
            )
        })
        .collect();

    accumulator.push(AnyDiagnostic::NagaValidationError {
        file_id,
        range: location,
        message,
        related,
    });
}

pub(crate) fn naga_diagnostics<Naga>(
    database: &dyn HirDatabase,
    file_id: EditionedFileId,
    config: &DiagnosticsConfig,
    accumulator: &mut Vec<AnyDiagnostic>,
) where
    Naga: self::Naga,
{
    let source: &str = database.file_text(file_id.file_id(database)).text(database);
    let full_range = TextRange::up_to(TextSize::of(source));

    match Naga::parse(source) {
        Ok(module) => {
            if !config.naga_validation_enabled {
                return;
            }
            if let Err(error) = Naga::validate(&module) {
                emit(database, &error, file_id, full_range, accumulator);
            }
        },
        Err(error) => {
            if !config.naga_parsing_enabled {
                return;
            }
            emit(database, &error, file_id, full_range, accumulator);
        },
    }
}

fn error_message_cause_chain(error: &dyn error::Error) -> String {
    let mut message = error.to_string();

    let mut error = error.source();
    if error.is_some() {
        message.push_str(": ");
    }

    while let Some(source) = error {
        message.push_str(&source.to_string());
        error = source.source();
    }

    message
}
