use crate::{
    generators::comments::Comment,
    trivia::{NodeTriviaItem, NodeWithTrivia},
};

#[must_use]
pub fn is_ignored(node: &NodeWithTrivia) -> bool {
    let ignore_candidate = node
        .preceding_trivia
        .iter()
        .rev()
        .find(|item| !matches!(item, NodeTriviaItem::LineSpacing(_)));

    //dbg!(&node);

    ignore_candidate.is_some_and(|item| match item {
        NodeTriviaItem::Comment(comment) | NodeTriviaItem::NewlinedComment(comment) => {
            //dbg!(&comment);
            match comment {
                Comment::LineEnding(syntax_token) => {
                    syntax_token.text().trim() == "// @wgslfmt(ignore)"
                },
                Comment::Block(syntax_token) => {
                    syntax_token.text().trim() == "/* @wgslfmt(ignore) */"
                },
            }
        },

        NodeTriviaItem::LineSpacing(_) | NodeTriviaItem::AttributeList(_) => false,
    })
}
