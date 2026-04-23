use super::{Naga, NagaError, Range};

pub struct Naga29;

impl Naga for Naga29 {
    type Module = naga29::Module;
    type ParseError = naga29::front::wgsl::ParseError;
    type ValidationError = naga29::WithSpan<naga29::valid::ValidationError>;

    fn parse(source: &str) -> Result<Self::Module, Self::ParseError> {
        naga29::front::wgsl::parse_str(source)
    }

    fn validate(module: &Self::Module) -> Result<(), Self::ValidationError> {
        let flags = naga29::valid::ValidationFlags::all();
        let capabilities = naga29::valid::Capabilities::all();
        let mut validator = naga29::valid::Validator::new(flags, capabilities);
        validator.validate(module).map(drop)
    }
}

impl NagaError for naga29::front::wgsl::ParseError {
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

impl NagaError for naga29::WithSpan<naga29::valid::ValidationError> {
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
