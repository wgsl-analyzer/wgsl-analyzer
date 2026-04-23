use super::{Naga, NagaError, Range};

pub struct NagaMain;

impl Naga for NagaMain {
    type Module = nagamain::Module;
    type ParseError = nagamain::front::wgsl::ParseError;
    type ValidationError = nagamain::WithSpan<nagamain::valid::ValidationError>;

    fn parse(source: &str) -> Result<Self::Module, Self::ParseError> {
        nagamain::front::wgsl::parse_str(source)
    }

    fn validate(module: &Self::Module) -> Result<(), Self::ValidationError> {
        let flags = nagamain::valid::ValidationFlags::all();
        let capabilities = nagamain::valid::Capabilities::all();
        let mut validator = nagamain::valid::Validator::new(flags, capabilities);
        validator.validate(module).map(drop)
    }
}

impl NagaError for nagamain::front::wgsl::ParseError {
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

impl NagaError for nagamain::WithSpan<nagamain::valid::ValidationError> {
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
