use super::{Naga, NagaError, Range};

pub struct Naga29;

impl Naga for Naga29 {
    type Module = naga29::Module;
    type ParseError = naga29::front::wgsl::ParseError;
    type ValidationError = naga29::WithSpan<naga29::valid::ValidationError>;

    fn parse(source: &str) -> Result<Self::Module, Self::ParseError> {
        naga29::front::wgsl::parse_str(source)
    }

    fn validate(
        module: &Self::Module,
        native_features: bool,
    ) -> Result<(), Self::ValidationError> {
        let flags = naga29::valid::ValidationFlags::all();
        let mut capabilities = naga29::valid::Capabilities::all();
        if !native_features {
            use naga29::valid::Capabilities as Caps;

            capabilities.remove(
                Caps::SHADER_FLOAT32_ATOMIC
                    | Caps::SHADER_INT64
                    | Caps::SHADER_INT64_ATOMIC_MIN_MAX
                    | Caps::SHADER_INT64_ATOMIC_ALL_OPS
                    | Caps::FLOAT64,
            );
        }
        let mut validator = naga29::valid::Validator::new(flags, capabilities);
        validator.validate(module).map(drop)
    }
}

impl NagaError for naga29::front::wgsl::ParseError {
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

impl NagaError for naga29::WithSpan<naga29::valid::ValidationError> {
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

fn to_range(span: naga29::Span) -> Option<Range<usize>> {
    span.to_range().map(Range::from)
}
