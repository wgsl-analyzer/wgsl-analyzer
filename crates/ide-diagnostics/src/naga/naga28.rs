use super::{Naga, NagaError, Range};

pub struct Naga28;

impl Naga for Naga28 {
    type Module = naga28::Module;
    type ParseError = naga28::front::wgsl::ParseError;
    type ValidationError = naga28::WithSpan<naga28::valid::ValidationError>;

    fn parse(source: &str) -> Result<Self::Module, Self::ParseError> {
        naga28::front::wgsl::parse_str(source)
    }

    fn validate(
        module: &Self::Module,
        native_features: bool,
    ) -> Result<(), Self::ValidationError> {
        let flags = naga28::valid::ValidationFlags::all();
        let mut capabilities = naga28::valid::Capabilities::all();
        if !native_features {
            use naga28::valid::Capabilities as Caps;

            capabilities.remove(
                Caps::SHADER_FLOAT32_ATOMIC
                    | Caps::SHADER_INT64
                    | Caps::SHADER_INT64_ATOMIC_MIN_MAX
                    | Caps::SHADER_INT64_ATOMIC_ALL_OPS
                    | Caps::FLOAT64,
            );
        }
        let mut validator = naga28::valid::Validator::new(flags, capabilities);
        validator.validate(module).map(drop)
    }
}

impl NagaError for naga28::front::wgsl::ParseError {
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

impl NagaError for naga28::WithSpan<naga28::valid::ValidationError> {
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

fn to_range(span: naga28::Span) -> Option<Range<usize>> {
    span.to_range().map(Range::from)
}
