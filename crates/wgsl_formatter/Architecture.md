# Architecture of the Formatter

## Generator Functions

Most of the heavy lifting is done via the `gen_`-functions in the [`generators`] module.

They are split into a "context", a "parse" and a "format" section.

- The "context" section is usually omitted, unless it is required. It
  checks if some context about the current node is true (e.g if a compound-
  statement is part of a conditional compilation `@if` construct).
- The "parse" section parses the interesting bits of the ast into bindings.
  (We try to not "check" the ast for errors at this point, but be reasonably general.
  If there is some structure of the ast that we expect, we can assert these things with `expect_kind`
  and similar functions.)
- The "format" section then uses the parsed bindings to construct a `PrintItemBuffer`.

An example of a `gen_` function:

```rust
# use dprint_core_macros::sc;
# use parser::SyntaxKind;
# use syntax::{AstNode as _, ast};
# use wgsl_formatter::{
#    ast_parse::{DiscardBlankspace, NoTrivia, parse_end, parse_node_with, syntax_iter},
#    context_policies::statement_needs_semicolon_policy,
#    generators::node::gen_node_with_trivia,
#    print_item_buffer::{
#        PrintItemBuffer,
#        spacing_request::{Request, RequestItem},
#    },
#    reporting::FormatDocumentResult,
# };
# pub fn belongs_to_clown(node: &parser::SyntaxNode) -> bool { false }
pub fn gen_example(
    node: &ast::DiscardStatement
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Context ====
    let needs_a_smiley = node.syntax().parent().is_some_and(|node| belongs_to_clown(&node));

    // ==== Parse ====
    let mut syntax = syntax_iter(node.syntax());
    let item_discard =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_kind(SyntaxKind::Discard)?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind_optional(SyntaxKind::Semicolon)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    if needs_a_smiley {
        formatted.push_sc(sc!(":)"));
    }
    formatted.extend(gen_node_with_trivia(&item_discard)?);
    if statement_needs_semicolon_policy(node.syntax()) {
        formatted.push_sc(sc!(";"));
    }
    formatted.request(Request::expect(RequestItem::LineBreak));
    Ok(formatted)
}
```

This separation between sections allows us to properly separate the "pattern matching" on the ast
from how things will actually be printed.
Usually one will only touch the "parse" section when fixing bugs where the
formatter doesn't recognize some syntax, and
one will only touch the "format" section when changing how the output will look.

### Sidenote

Conceptually we could split the parsing and formatting into separate functions
The "parse" functions would consist of just the "parse" sections
(pretty much unchanged) and return some clever structs that would form a sort of
"Formatter-Syntax-Tree".
Then we would recursively iterate this "Formatter-Syntax-Tree" via the
"formatter" functions, which would consist of just the "format" sections.
Through this iteration we would collapse the FST into the PrintItemBuffer.

However instead of having to keep the whole FST in memory, we instead
immediately destructure a FST-node as soon as the parse-section would
construct it.
Thus we only ever keep the "node"s we need in memory at the same time, and
we don't even have to write structs for the FST-nodes, as the struct fields
are simply the bindings we have in scope.

As we still have to keep all the parent nodes in memory (because this
procedure is recursive), there will be problematic situations where
a stack-overflow will occur, when the AST is particularly deeply nested.
As far as I can tell the highly structured approach to the gen_ functions
should make converting this to an iterative approach relatively straight
forward, however I don't think such deeply nested ASTs will ever occur in practice.

## The "Parse" Section

### SyntaxIter

The [`SyntaxIter`](ast_parse::SyntaxIter) is just a wrapper around `itertools::put_back_n(node.children_with_tokens())`, with the added benefit that
on development builds it checks if [`ast_parse::parse_end`] was called on it.

### Parse functions & NodeWithTrivia

The main parsing function is [`ast_parse::parse_node_with`], which retrieves the next "interesting"
node from the syntax iterator, and associates it with trivia, like attributes or comments.
It takes a ["Policy"](`ast_parse::ParseNodePolicy`) which specifies how trivia should be associated.
(e.g [`DiscardBlankspace`](ast_parse::DiscardBlankspace) specifies that blankspace like newlines are not interesting for this node
and should not be included in the trivia. This means that when the node is later generated and put
into the output, any blankspace will not be preserved.)

[`ast_parse::parse_node_with`] returns a [`trivia::NodeWithTrivia`] that associates some kind of "content" (which can be a syntax node, empty, or the end of the input)
with trivia preceding, and trivia succeeding it.

Per default `parse_node_with` treats all comments, linebreaks, and preceding attributes as trivia,
but using policies we can adapt the behavior and decide what nodes are treated as trivia, as content, discarded or
if a node signals to us to stop consuming from the `SyntaxIter`, regardless of if we found content already.

Policies can be thought of as functions, that get a reference to the newly parsed node, and return an action. Multiple
policies can be composed together via tuples (in which case they get applied in order, and the first one to return an action
gets applied), or they can be specified if they apply to only preceding trivia, only succeeding trivia or both.
The most commonly used Policies are defined in [`ast_parse`], and in edge cases a function can be turned into one too.

### Expecting SyntaxKinds

We take no special care to "validate" the AST - that is the job of the parser.
However oftentimes deciding on how a node should be formatted makes certain assumptions about the shape of the AST.
"Are we inside of the brackets?" etc. To document these assumptions, and make sure we don't "misinterpret" the AST
(either due to bugs in the formatter or due to syntax changes that haven't been implemented yet), we often
assert the kind of parsed nodes with [`expect_kind`](trivia::NodeWithTrivia::expect_kind) and similar functions.

Even though they are named "expect", they do not panic, but instead gracefully fail with a [`reporting::FormatDocumentError`].
This error is not meant to show the user if they wrote wrong WGSL (again, that is the job of the parser), but contain just enough
information to give us a starting point when debugging issues.

## The "Format" Section

### [`PrintItemBuffer`] and [`Request`]s

In order to keep the generator functions as streamlined as possible, and cut down on repetitive noise,
things like making sure that there are no spaces between the indentation and the code, or that
at no point two spaces can follow each other are handled by the [`PrintItemBuffer`].

The [`PrintItemBuffer`] can be thought of as a linked-list that we can very easily append text to.
Each `gen_` function returns one, and using [`PrintItemBuffer::extend`], one can easily be appended to another.

Every time we want to emit spaces, newlines or empty lines, we can do so using [`PrintItemBuffer::request`], which
automatically takes care of all of these weird edge-cases, like merging consecutive spaces.
When we request a space, before generating another node, we don't have to worry about if that other node possibly
starts with a newline or emits its own space due to e.g a block comment.

Additionally the [`PrintItemBuffer`] also allows some basic re-ordering of emitted items - for example if
one generator function generates an item that always wants to be indented, this function can start with
[`PrintItemBuffer::start_indent_before_requests`], to make sure that the indent starts before any newlines that
might have been issued by the generator function that generated its parent.

[`PrintItemBuffer`]: print_item_buffer::PrintItemBuffer
[`PrintItemBuffer::extend`]: print_item_buffer::PrintItemBuffer::extend
[`PrintItemBuffer::request`]: print_item_buffer::PrintItemBuffer::request
[`Request`]: print_item_buffer::spacing_request::Request
[`PrintItemBuffer::start_indent_before_requests`]: print_item_buffer::PrintItemBuffer::start_indent_before_requests

## [`gen_node_with_trivia`](generators::node::gen_node_with_trivia)

This is more or less the internal entrypoint for formatting any node, it takes care of emitting any applicable
trivia around the node, dispatching to the correct `gen_` function, and emitting the source code verbatim, if it
was ignored using `// @wgslfmt(ignore)`

## The "Context" Section

Because generator function need to work on their own (without relying on other generator functions calling them and providing
helpful arguments), they do not know anything about the context that the node is in.
(This is required for range-formatting, where any node could be the entry point to the formatting).

This context needs to be reconstructed by walking around in the AST.
If the logic for the context is completely self contained to the node, its fine to just put it in the generator function, however
if the logic depends on details of how other generator functions are implemented, then a [context-policy](crate::context_policies) would be better.
