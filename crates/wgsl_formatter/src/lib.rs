#[cfg(test)]
mod tests;

mod format;
mod util;

use rowan::WalkEvent;
use syntax::{AstNode, SyntaxKind, SyntaxNode, ast};

#[must_use]
pub fn format_str(
    input: &str,
    options: &FormattingOptions,
) -> String {
    let parse = parser::parse_file(input);
    let node = parse.syntax().clone_for_update();
    format_recursive(&node, options);
    node.to_string()
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FormattingOptions {
    #[cfg_attr(feature = "serde", serde(alias = "trailingCommas"))]
    pub trailing_commas: Policy,
    #[cfg_attr(feature = "serde", serde(alias = "indentSymbol"))]
    pub indent_symbol: String,
}

impl Default for FormattingOptions {
    fn default() -> Self {
        Self {
            trailing_commas: Policy::Ignore,
            indent_symbol: "    ".to_owned(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Policy {
    Ignore,
    Remove,
    Insert,
}

impl std::str::FromStr for Policy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ignore" => Ok(Self::Ignore),
            "insert" => Ok(Self::Insert),
            "remove" => Ok(Self::Remove),
            _ => Err(format!("invalid policy: {s}")),
        }
    }
}

pub fn format_recursive(
    syntax: &SyntaxNode,
    options: &FormattingOptions,
) {
    let preorder = syntax.preorder();

    let mut indentation: usize = 0;

    for event in preorder {
        match event {
            WalkEvent::Enter(node) => {
                if is_indent_kind(&node) {
                    indentation += 1;
                }
                format::format_syntax_node(node, indentation, options);
            },
            WalkEvent::Leave(node) => {
                if is_indent_kind(&node) {
                    indentation = indentation.saturating_sub(1);
                }
            },
        }
    }
}

pub(crate) fn is_indent_kind(node: &SyntaxNode) -> bool {
    // NOTE: LoopStatement is intentionally excluded here. Its body is a
    // CompoundStatement which already increments indentation; including
    // LoopStatement would double-indent the loop contents.
    if matches!(
        node.kind(),
        SyntaxKind::CompoundStatement | SyntaxKind::SwitchBody
    ) {
        return true;
    }

    let param_list_left_paren = ast::FunctionParameters::cast(node.clone())
        .and_then(|list| list.left_parenthesis_token())
        .or_else(|| {
            let list = ast::Arguments::cast(node.clone())?;
            list.left_parenthesis_token()
        });

    if param_list_left_paren
        .and_then(|token| token.next_token())
        .as_ref()
        .is_some_and(util::is_whitespace_with_newline)
    {
        return true;
    }

    false
}

#[cfg(test)]
mod policy_tests {
    use super::*;
    use std::str::FromStr as _;
    #[test]
    fn policy_from_str_valid_values() {
        assert!(matches!(Policy::from_str("ignore"), Ok(Policy::Ignore)));
        assert!(matches!(Policy::from_str("insert"), Ok(Policy::Insert)));
        assert!(matches!(Policy::from_str("remove"), Ok(Policy::Remove)));
    }
    #[test]
    fn policy_from_str_invalid_value() {
        let result = Policy::from_str("invalid_value");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "invalid policy: invalid_value");
    }
}
