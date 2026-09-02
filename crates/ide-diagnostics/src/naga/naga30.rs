use super::{Naga, NagaError, Range};

pub struct Naga30;

impl Naga for Naga30 {
    type Module = naga30::Module;
    type ParseError = naga30::front::wgsl::ParseError;
    type ValidationError = naga30::WithSpan<naga30::valid::ValidationError>;

    fn parse(source: &str) -> Result<Self::Module, Self::ParseError> {
        naga30::front::wgsl::parse_str(source)
    }

    fn validate(module: &Self::Module) -> Result<(), Box<Self::ValidationError>> {
        let flags = naga30::valid::ValidationFlags::all();
        let capabilities = naga30::valid::Capabilities::all();
        let mut validator = naga30::valid::Validator::new(flags, capabilities);
        Ok(validator.validate(module).map(drop)?)
    }
}

impl NagaError for naga30::front::wgsl::ParseError {
    fn spans(&self) -> Box<dyn Iterator<Item = (Option<Range<usize>>, String)> + '_> {
        Box::new(
            self.labels()
                .map(|(span, label)| (to_range(span), label.to_owned())),
        )
    }

    fn location(&self) -> Option<Range<usize>> {
        let (span, _) = self.labels().next()?;
        to_range(span)
    }
}

impl NagaError for naga30::WithSpan<naga30::valid::ValidationError> {
    fn spans(&self) -> Box<dyn Iterator<Item = (Option<Range<usize>>, String)> + '_> {
        Box::new(
            self.spans()
                .map(move |(span, label)| (to_range(*span), label.clone())),
        )
    }

    fn location(&self) -> Option<Range<usize>> {
        self.spans().next().and_then(|(span, _)| to_range(*span))
    }
}

fn to_range(span: naga30::Span) -> Option<Range<usize>> {
    span.to_range().map(Range::from)
}
