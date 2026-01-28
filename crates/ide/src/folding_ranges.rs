use base_db::{FileId, FileRange, TextRange};
use rowan::{NodeOrToken, TextSize};
use rustc_hash::FxHashSet;
use std::hash::Hash;
use syntax::{
    AstNode, AstToken as _, Direction,
    SyntaxKind::{self, *},
    SyntaxNode,
    ast::{self, SourceFile},
    match_ast,
};

const REGION_START: &str = "// region:";
const REGION_END: &str = "// endregion";

#[derive(Debug, PartialEq, Eq)]
pub enum FoldKind {
    Comment,
    Imports,
    Region,
    Block,
    ArgList,
    ReturnType,
    Function,
    // region: item runs
    Constants,
    Variables,
    Overrides,
    TypeAliases,
    // endregion: item runs
}

#[derive(Debug)]
pub struct Fold {
    pub range: TextRange,
    pub kind: FoldKind,
}

// Feature: Folding
//
// Defines folding regions for curly braced blocks, runs of consecutive use, mod, const or static
// items, and `region` / `endregion` comment markers.
pub(crate) fn folding_ranges(file: &SourceFile) -> Vec<Fold> {
    let mut result = vec![];
    let mut visited_comments = FxHashSet::default();
    let mut visited_nodes = FxHashSet::default();

    // regions can be nested, here is a LIFO buffer
    let mut region_starts: Vec<TextSize> = vec![];

    for element in file.syntax().descendants_with_tokens() {
        // Fold items that span multiple lines
        if let Some(kind) = fold_kind(element.kind()) {
            let is_multiline = match &element {
                NodeOrToken::Node(node) => node.text().contains_char('\n'),
                NodeOrToken::Token(token) => token.text().contains('\n'),
            };
            if is_multiline {
                // for the func with multiline param list
                if matches!(element.kind(), SyntaxKind::FunctionDeclaration)
                    && let NodeOrToken::Node(node) = &element
                    && let Some(fn_node) = ast::FunctionDeclaration::cast(node.clone())
                {
                    if !fn_node
                        .parameter_list()
                        .is_some_and(|param_list| param_list.syntax().text().contains_char('\n'))
                    {
                        continue;
                    }

                    if fn_node.body().is_some() {
                        // Get the actual start of the function (excluding doc comments)
                        let fn_start = fn_node.fn_token().map_or_else(
                            || node.text_range().start(),
                            |token| token.text_range().start(),
                        );
                        result.push(Fold {
                            range: TextRange::new(fn_start, node.text_range().end()),
                            kind: FoldKind::Function,
                        });
                        continue;
                    }
                }
                result.push(Fold {
                    range: element.text_range(),
                    kind,
                });
                continue;
            }
        }

        match element {
            NodeOrToken::Token(token) => {
                // Fold groups of comments
                if let Some(comment) = ast::Comment::cast(token) {
                    if visited_comments.contains(&comment) {
                        continue;
                    }
                    let text = comment.text().trim_start();
                    if text.starts_with(REGION_START) {
                        region_starts.push(comment.syntax().text_range().start());
                    } else if text.starts_with(REGION_END) {
                        if let Some(region) = region_starts.pop() {
                            result.push(Fold {
                                range: TextRange::new(region, comment.syntax().text_range().end()),
                                kind: FoldKind::Region,
                            });
                        }
                    } else if let Some(range) =
                        contiguous_range_for_comment(&comment, &mut visited_comments)
                    {
                        result.push(Fold {
                            range,
                            kind: FoldKind::Comment,
                        });
                    }
                }
            },
            NodeOrToken::Node(node) => {
                match_ast! {
                    match node {
                        ast::ConstantDeclaration(konst) => {
                            if let Some(range) = contiguous_range_for_item_group(&konst, &mut visited_nodes) {
                                result.push(Fold { range, kind: FoldKind::Constants });
                            }
                        },
                        ast::VariableDeclaration(variable) => {
                            if let Some(range) = contiguous_range_for_item_group(&variable, &mut visited_nodes) {
                                result.push(Fold { range, kind: FoldKind::Variables });
                            }
                        },
                        ast::OverrideDeclaration(r#override) => {
                            if let Some(range) = contiguous_range_for_item_group(&r#override, &mut visited_nodes) {
                                result.push(Fold { range, kind: FoldKind::Overrides });
                            }
                        },
                        ast::TypeAliasDeclaration(alias) => {
                            if let Some(range) = contiguous_range_for_item_group(&alias, &mut visited_nodes) {
                                result.push(Fold { range, kind: FoldKind::TypeAliases });
                            }
                        },
                        _ => (),
                    }
                }
            },
        }
    }

    result
}

const fn fold_kind(kind: SyntaxKind) -> Option<FoldKind> {
    #[expect(clippy::wildcard_enum_match_arm, reason = "too many match arms")]
    match kind {
        SyntaxKind::BlockComment | SyntaxKind::LineEndingComment => Some(FoldKind::Comment),
        SyntaxKind::Arguments | SyntaxKind::FunctionParameters | SyntaxKind::TemplateList => {
            Some(FoldKind::ArgList)
        },
        SyntaxKind::ReturnType => Some(FoldKind::ReturnType),
        SyntaxKind::FunctionDeclaration => Some(FoldKind::Function),
        SyntaxKind::StructBody | SyntaxKind::CompoundStatement | SyntaxKind::SwitchBody => {
            Some(FoldKind::Block)
        },
        _ => None,
    }
}

fn contiguous_range_for_item_group<N>(
    first: &N,
    visited: &mut FxHashSet<SyntaxNode>,
) -> Option<TextRange>
where
    N: AstNode + Clone,
{
    if !visited.insert(first.syntax().clone()) {
        return None;
    }

    let (mut last) = (first.clone());
    for element in first.syntax().siblings_with_tokens(Direction::Next) {
        let node = match element {
            NodeOrToken::Token(token) => {
                if let Some(ws) = ast::Whitespace::cast(token)
                    && !ws.spans_multiple_lines()
                {
                    // Ignore whitespace without blank lines
                    continue;
                }
                // There is a blank line or another token, which means that the
                // group ends here
                break;
            },
            NodeOrToken::Node(node) => {
                if ast::Attribute::can_cast(node.kind()) {
                    // Ignore attributes
                    continue;
                }
                node
            },
        };

        if let Some(next) = N::cast(node) {
            // Rust-Analyzer has a check for equal visibility here
            // WGSL does not yet have a notion of visibility, so that check doesn't exist
            visited.insert(next.syntax().clone());
            last = next;
            continue;
        }
        // Stop if we find an item of a different kind.
        break;
    }

    if first.syntax() == last.syntax() {
        // The group consists of only one element, therefore it cannot be folded
        None
    } else {
        Some(TextRange::new(
            first.syntax().text_range().start(),
            last.syntax().text_range().end(),
        ))
    }
}

fn contiguous_range_for_comment(
    first: &ast::Comment,
    visited: &mut FxHashSet<ast::Comment>,
) -> Option<TextRange> {
    visited.insert(first.clone());

    // Only fold comments of the same flavor
    let group_kind = first.kind();
    if !group_kind.shape.is_line() {
        return None;
    }

    let mut last = first.clone();
    for element in first.syntax().siblings_with_tokens(Direction::Next) {
        match element {
            NodeOrToken::Token(token) => {
                if let Some(ws) = ast::Whitespace::cast(token.clone())
                    && !ws.spans_multiple_lines()
                {
                    // Ignore whitespace without blank lines
                    continue;
                }
                if let Some(comment) = ast::Comment::cast(token)
                    && comment.kind() == group_kind
                {
                    let text = comment.text().trim_start();
                    // regions are not real comments
                    if !(text.starts_with(REGION_START) || text.starts_with(REGION_END)) {
                        visited.insert(comment.clone());
                        last = comment;
                        continue;
                    }
                }
                // The comment group ends because either:
                // * An element of a different kind was reached
                // * A comment of a different flavor was reached
                break;
            },
            NodeOrToken::Node(_) => break,
        };
    }

    if *first == last {
        // The group consists of only one element, therefore it cannot be folded
        None
    } else {
        Some(TextRange::new(
            first.syntax().text_range().start(),
            last.syntax().text_range().end(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use test_utils::extract_tags;

    use super::*;

    #[track_caller]
    fn check(fixture: &str) {
        let (ranges, text) = extract_tags(fixture, "fold");

        let parse = syntax::parse(&text, syntax::Edition::LATEST);
        let mut folds = folding_ranges(&parse.tree());
        folds.sort_by_key(|fold| (fold.range.start(), fold.range.end()));

        assert_eq!(
            folds.len(),
            ranges.len(),
            "The amount of folds is different than the expected amount"
        );

        for (fold, (range, attr)) in folds.iter().zip(ranges.into_iter()) {
            assert_eq!(
                fold.range.start(),
                range.start(),
                "mismatched start of folding ranges"
            );
            assert_eq!(
                fold.range.end(),
                range.end(),
                "mismatched end of folding ranges"
            );

            let kind = match fold.kind {
                FoldKind::Comment => "comment",
                FoldKind::Imports => "imports",
                FoldKind::Block => "block",
                FoldKind::ArgList => "arglist",
                FoldKind::Region => "region",
                FoldKind::Constants => "consts",
                FoldKind::Variables => "variables",
                FoldKind::Overrides => "overrides",
                FoldKind::TypeAliases => "typealiases",
                FoldKind::ReturnType => "returntype",
                FoldKind::Function => "function",
            };
            assert_eq!(kind, &attr.unwrap());
        }
    }

    #[test]
    fn fold_func_with_multiline_param_list() {
        check(
            r#"
<fold function>fn func<fold arglist>(
    a: f32,
    b: f32,
    c: f32,
)</fold> <fold block>{



}</fold></fold>
"#,
        );
    }

    #[test]
    fn fold_comments() {
        check(
            r#"
<fold comment>// Hello
// this is a multiline
// comment
//</fold>

// But this is not

fn main() <fold block>{
    <fold comment>// We should
    // also
    // fold
    // this one.</fold>
    <fold comment>//! But this one is different
    //! because it has another flavor</fold>
    <fold comment>/* As does this
    multiline comment */</fold>
}</fold>
"#,
        );
    }

    #[test]
    fn folds_structs() {
        check(
            r#"
struct Foo <fold block>{
}</fold>
"#,
        );
    }

    #[test]
    fn fold_switch_arms() {
        check(
            r#"
fn main() <fold block>{
    switch foo <fold block>{
        case 0 { return 0; },
        default { return 1; },
    }</fold>
}</fold>
"#,
        );
    }

    #[test]
    fn fold_big_calls() {
        check(
            r#"
fn main() <fold block>{
    frobnicate<fold arglist>(
        1,
        2,
        3,
    )</fold>
}</fold>
"#,
        );
    }

    #[test]
    fn fold_multiline_params() {
        check(
            r#"
<fold function>fn foo<fold arglist>(
    x: i32,
    y: String,
)</fold> {}</fold>
"#,
        );
    }

    #[test]
    fn fold_region() {
        check(
            r#"
// 1. some normal comment
<fold region>// region: test
// 2. some normal comment
<fold region>// region: inner
fn f() {}
// endregion</fold>
fn f2() {}
// endregion: test</fold>
"#,
        );
    }

    #[test]
    fn fold_consecutive_const() {
        check(
            r#"
<fold consts>const FIRST_CONST: f32 = 1;
const SECOND_CONST: u32 = 2;</fold>
"#,
        );
    }

    #[test]
    fn fold_consecutive_override() {
        check(
            r#"
<fold overrides>override FIRST_OVERRIDE: f32 = 1;
override SECOND_OVERRIDE: vec3f = vec3f(2);</fold>
"#,
        );
    }

    #[test]
    fn fold_return_type() {
        check(
            r#"
fn foo()<fold returntype>-> array<fold arglist><
    f32,
    4
></fold></fold> {  }

fn bar() -> array<f32, 4> { (true, true) }
"#,
        );
    }

    #[test]
    fn fold_generics() {
        check(
            r#"
alias Foo = array<fold arglist><
    u32,
    2 + 3,
></fold>;
"#,
        );
    }

    #[test]
    fn fold_doc_comments_with_multiline_paramlist_function() {
        check(
            r#"
<fold comment>/// A very very very very very very very very very very very very very very very
/// very very very long description</fold>
<fold function>fn foo<fold arglist>(
    very_long_parameter_name: u32,
    another_very_long_parameter_name: u32,
    third_very_long_param: u32,
)</fold> <fold block>{
    // nothing
}</fold></fold>
"#,
        );
    }
}
