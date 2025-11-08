use base_db::{SourceDatabase, TextRange};
use ide_db::RootDatabase;
use line_index::{LineCol, LineIndex};
use parser::{SyntaxNode, SyntaxToken};
use rowan::{NodeOrToken, TextSize, WalkEvent};
use stdx::format_to;
use syntax::ast;
use triomphe::Arc;
use vfs::FileId;

// Feature: Show Syntax Tree
//
// Shows a tree view with the syntax tree of the current file
//
// | Editor  | Panel Name |
// |---------|-------------|
// | VS Code | **WGSL Syntax Tree** |
pub(crate) fn view_syntax_tree(
    database: &RootDatabase,
    file_id: FileId,
) -> String {
    let syntax_node = database.parse(file_id).syntax();
    let line_index = database.line_index(file_id);

    let ctx = SyntaxTreeCtx {
        line_index,
        in_string: None,
    };

    syntax_node_to_json(&syntax_node, &ctx)
}

fn syntax_node_to_json(
    node: &SyntaxNode,
    ctx: &SyntaxTreeCtx,
) -> String {
    let mut result = String::new();
    for event in node.preorder_with_tokens() {
        match event {
            WalkEvent::Enter(it) => {
                let kind = it.kind();
                let (text_range, inner_range_str) = match &ctx.in_string {
                    Some(in_string) => {
                        let start_pos = TextPosition::new(&ctx.line_index, it.text_range().start());
                        let end_pos = TextPosition::new(&ctx.line_index, it.text_range().end());

                        let inner_start: u32 = it.text_range().start().into();
                        let inner_end: u32 = it.text_range().start().into();

                        let mut true_start = inner_start + in_string.offset;
                        let mut true_end = inner_end + in_string.offset;
                        for pos in &in_string.marker_positions {
                            if *pos >= inner_end {
                                break;
                            }

                            // We conditionally add to true_start in case
                            // the marker is between the start and end.
                            true_start += 2 * (*pos < inner_start) as u32;
                            true_end += 2;
                        }

                        let true_range = TextRange::new(true_start.into(), true_end.into());

                        (
                            true_range,
                            format!(r#","istart":{start_pos},"iend":{end_pos}"#,),
                        )
                    },
                    None => (it.text_range(), "".to_owned()),
                };

                let start = TextPosition::new(&ctx.line_index, text_range.start());
                let end = TextPosition::new(&ctx.line_index, text_range.end());

                match it {
                    NodeOrToken::Node(_) => {
                        format_to!(
                            result,
                            r#"{{"type":"Node","kind":"{kind:?}","start":{start},"end":{end}{inner_range_str},"children":["#
                        );
                    },
                    NodeOrToken::Token(token) => {
                        let comma = if token.next_sibling_or_token().is_some() {
                            ","
                        } else {
                            ""
                        };
                        format_to!(
                            result,
                            r#"{{"type":"Token","kind":"{kind:?}","start":{start},"end":{end}{inner_range_str}}}{comma}"#
                        )
                    },
                }
            },
            WalkEvent::Leave(it) => match it {
                NodeOrToken::Node(node) => {
                    let comma = if node.next_sibling_or_token().is_some() {
                        ","
                    } else {
                        ""
                    };
                    format_to!(result, "]}}{comma}")
                },
                NodeOrToken::Token(_) => (),
            },
        }
    }

    result
}

struct TextPosition {
    offset: TextSize,
    line: u32,
    col: u32,
}

impl std::fmt::Display for TextPosition {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "[{:?},{},{}]", self.offset, self.line, self.col)
    }
}

impl TextPosition {
    pub(crate) fn new(
        line_index: &LineIndex,
        offset: TextSize,
    ) -> Self {
        let LineCol { line, col } = line_index.line_col(offset);
        Self { offset, line, col }
    }
}

struct SyntaxTreeCtx {
    line_index: Arc<LineIndex>,
    in_string: Option<InStringCtx>,
}

struct InStringCtx {
    offset: u32,
    marker_positions: Vec<u32>,
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use crate::fixture;

    fn check(
        source: &str,
        expect: expect_test::Expect,
    ) {
        let (analysis, file_id) = fixture::single_file_db(source);
        let syn = analysis.view_syntax_tree(file_id).unwrap();
        expect.assert_eq(&syn)
    }

    #[test]
    fn view_syntax_tree() {
        // Basic syntax
        check(
            r#"fn foo() {}"#,
            expect![[
                r#"{"type":"Node","kind":"SourceFile","start":[0,0,0],"end":[11,0,11],"children":[{"type":"Node","kind":"FunctionDeclaration","start":[0,0,0],"end":[11,0,11],"children":[{"type":"Token","kind":"Fn","start":[0,0,0],"end":[2,0,2]},{"type":"Token","kind":"Blankspace","start":[2,0,2],"end":[3,0,3]},{"type":"Node","kind":"Name","start":[3,0,3],"end":[6,0,6],"children":[{"type":"Token","kind":"Identifier","start":[3,0,3],"end":[6,0,6]}]},{"type":"Node","kind":"FunctionParameters","start":[6,0,6],"end":[8,0,8],"children":[{"type":"Token","kind":"ParenthesisLeft","start":[6,0,6],"end":[7,0,7]},{"type":"Token","kind":"ParenthesisRight","start":[7,0,7],"end":[8,0,8]}]},{"type":"Token","kind":"Blankspace","start":[8,0,8],"end":[9,0,9]},{"type":"Node","kind":"CompoundStatement","start":[9,0,9],"end":[11,0,11],"children":[{"type":"Token","kind":"BraceLeft","start":[9,0,9],"end":[10,0,10]},{"type":"Token","kind":"BraceRight","start":[10,0,10],"end":[11,0,11]}]}]}]}"#
            ]],
        );

        check(
            r#"const bar: u32 = 3;"#,
            expect![[
                r#"{"type":"Node","kind":"SourceFile","start":[0,0,0],"end":[19,0,19],"children":[{"type":"Node","kind":"ConstantDeclaration","start":[0,0,0],"end":[19,0,19],"children":[{"type":"Token","kind":"Constant","start":[0,0,0],"end":[5,0,5]},{"type":"Token","kind":"Blankspace","start":[5,0,5],"end":[6,0,6]},{"type":"Node","kind":"Name","start":[6,0,6],"end":[9,0,9],"children":[{"type":"Token","kind":"Identifier","start":[6,0,6],"end":[9,0,9]}]},{"type":"Token","kind":"Colon","start":[9,0,9],"end":[10,0,10]},{"type":"Token","kind":"Blankspace","start":[10,0,10],"end":[11,0,11]},{"type":"Node","kind":"TypeSpecifier","start":[11,0,11],"end":[14,0,14],"children":[{"type":"Node","kind":"NameReference","start":[11,0,11],"end":[14,0,14],"children":[{"type":"Token","kind":"Identifier","start":[11,0,11],"end":[14,0,14]}]}]},{"type":"Token","kind":"Blankspace","start":[14,0,14],"end":[15,0,15]},{"type":"Token","kind":"Equal","start":[15,0,15],"end":[16,0,16]},{"type":"Token","kind":"Blankspace","start":[16,0,16],"end":[17,0,17]},{"type":"Node","kind":"Literal","start":[17,0,17],"end":[18,0,18],"children":[{"type":"Token","kind":"IntLiteral","start":[17,0,17],"end":[18,0,18]}]},{"type":"Token","kind":"Semicolon","start":[18,0,18],"end":[19,0,19]}]}]}"#
            ]],
        )
    }
}
