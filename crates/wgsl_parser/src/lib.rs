mod grammar;
mod syntax_kind;

pub use syntax_kind::SyntaxKind;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum WgslLanguage {}
impl rowan::Language for WgslLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= SyntaxKind::Error as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

pub struct ParserDefinition;
impl parser::ParserDefinition for ParserDefinition {
    type Language = WgslLanguage;
    type TokenKind = SyntaxKind;
    type SyntaxKind = SyntaxKind;

    const DEFAULT_RECOVERY_SET: &'static [SyntaxKind] = &[SyntaxKind::Fn];
}

pub type SyntaxNode = rowan::SyntaxNode<WgslLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<WgslLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<WgslLanguage>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<WgslLanguage>;
pub type SyntaxElementChildren = rowan::SyntaxElementChildren<WgslLanguage>;
pub type PreorderWithTokens = rowan::api::PreorderWithTokens<WgslLanguage>;

pub type Parse = parser::Parse<ParserDefinition>;
pub type ParseError = parser::ParseError<ParserDefinition>;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub enum ParseEntryPoint {
    File,
    Expression,
    Statement,
    Type,
    AttributeList,
    FnParamList,
}

pub fn parse_entrypoint(input: &str, entrypoint: ParseEntryPoint) -> Parse {
    match entrypoint {
        ParseEntryPoint::File => parser::parse::<ParserDefinition, _>(input, grammar::file),
        ParseEntryPoint::Expression => parser::parse::<ParserDefinition, _>(input, grammar::expr),
        ParseEntryPoint::Statement => {
            parser::parse::<ParserDefinition, _>(input, grammar::statement)
        }
        ParseEntryPoint::Type => parser::parse::<ParserDefinition, _>(input, |p| {
            grammar::type_decl(p);
        }),
        ParseEntryPoint::AttributeList => {
            parser::parse::<ParserDefinition, _>(input, grammar::attribute_list)
        }
        ParseEntryPoint::FnParamList => {
            parser::parse::<ParserDefinition, _>(input, grammar::inner_param_list)
        }
    }
}

pub fn parse_file(input: &str) -> Parse {
    parse_entrypoint(input, ParseEntryPoint::File)
}

#[cfg(test)]
fn check_entrypoint(input: &str, entry_point: ParseEntryPoint, expected_tree: expect_test::Expect) {
    let parse = crate::parse_entrypoint(input, entry_point);
    expected_tree.assert_eq(&parse.debug_tree());
}

#[cfg(test)]
mod tests;
