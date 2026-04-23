use super::{Naga, NagaError, Range};

pub struct Naga27;

impl Naga for Naga27 {
    type Module = naga27::Module;
    type ParseError = naga27::front::wgsl::ParseError;
    type ValidationError = naga27::WithSpan<naga27::valid::ValidationError>;

    fn parse(source: &str) -> Result<Self::Module, Self::ParseError> {
        naga27::front::wgsl::parse_str(source)
    }

    fn validate(module: &Self::Module) -> Result<(), Self::ValidationError> {
        let flags = naga27::valid::ValidationFlags::all();
        let capabilities = naga27::valid::Capabilities::all();
        let mut validator = naga27::valid::Validator::new(flags, capabilities);
        validator.validate(module).map(drop)
    }
}

impl NagaError for naga27::front::wgsl::ParseError {
    fn spans(&self) -> Box<dyn Iterator<Item = (Option<Range<usize>>, String)> + '_> {
        Box::new(
            self.labels()
                .map(|(span, label)| (span.to_range(), label.to_owned())),
        )
    }

    fn location(&self) -> Option<Range<usize>> {
        let (span, _) = self.labels().next()?;
        span.to_range()
    }
}

impl NagaError for naga27::WithSpan<naga27::valid::ValidationError> {
    fn spans(&self) -> Box<dyn Iterator<Item = (Option<Range<usize>>, String)> + '_> {
        Box::new(
            self.spans()
                .map(move |(span, label)| (span.to_range(), label.clone())),
        )
    }

    fn location(&self) -> Option<Range<usize>> {
        self.spans().next().and_then(|(span, _)| span.to_range())
    }
}
