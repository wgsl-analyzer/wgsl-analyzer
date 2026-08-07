# Generator Functions
Most of the heavy lifting is done via the `gen_`-functions.

They are split into a "parse" section and a "format" section.
The "parse" section parses the interesting bits of the ast into bindings. 
(We try to not "check" the ast for errors at this point, but be reasonably general)
The "format" section then uses the parsed bindings to construct a `PrintItemBuffer`.
```rust
pub fn gen_something(
    attribute: &ast::SomeNode
) -> FormatDocumentResult<PrintItemBuffer> {
    // === Parse
    let mut syntax = put_back(attribute.syntax().children_with_tokens());

    parse_token(&mut syntax, SyntaxKind::AttributeOperator)?;
    let identifier = parse_node::<Blabla>(&mut syntax)?;
    parse_end(&mut syntax)?;

    // === Format
    let mut formatted = PrintItemBuffer::default();
    formatted.push_sc(sc!("something"));
    formatted.extend(gen_blabla(&identifier));
    Ok(formatted)
}
```

This separation allows us to properly separate the "pattern matching" on the ast from how things will actually be printed.
Usually one will only touch the "parse" section when fixing bugs where the formatter doesn't recognize some syntax, and
one will only touch the "format" section when changing how the output will look.

Conceptually we could split the parsing and formatting into separate functions. 
The "parse" functions would consist of just the "parse" sections (pretty much unchanged) and return some clever structs that would form a sort of "Formatter-Syntax-Tree". 
Then we would recursively iterate this "Formatter-Syntax-Tree" via the "formatter" functions, which would consist of just the "format" sections. 
Through this iteration we would collapse the FST into the PrintItemBuffer.

However instead of having to keep the whole FST in memory, we instead immediately destructure a FST-node as soon as the parse-section would construct it.
Thus we only ever keep the "node"s we need in memory at the same time, and we don't even have to write structs for the FST-nodes, as the struct fields
are simply the bindings we have in scope.

As we still have to keep all the parent nodes in memory (because this procedure is recursive), there will be problematic situations where
a stack-overflow will occur, when the AST is particularly deeply nested. 
As far as I can tell the highly structured approach to the gen_ functions should make converting this to an iterative approach relatively straight forward, however I don't think such deeply nested ASTs will ever occur in practice.


# Parsing a list of many things
Oftentimes we iterate over a list of nodes (statements in a compound statement, fields of a struct, etc).

This is usually done with the following pattern:

```rust
    enum Item {
        Item(Baz),
        Comment(Comment),
        LineSpacing(LineSpacing),
    }

    let mut items = Vec::new();
    loop {
        if let Some(spacing) = parse_line_spacing(&mut syntax) {
            items.push(SourceFileItem::LineSpacing(spacing));
        } else if let Some(_statement) = parse_token_optional(&mut syntax, SyntaxKind::Blankspace) {
            // If its not a line_spacing blankspace, then we simply discard it
        } else if let Some(comment) = parse_comment_optional(&mut syntax) {
            items.push(SourceFileItem::Comment(comment));
        } else if let Some(item) = parse_node::<Baz>(&mut syntax) {
            items.push(SourceFileItem::Item(item));
        } else {
            break;
        }
    }
```

Conceptually we "filter" the list of children of the AST into a list of only the things that we are interested in. 
This usually yields a very clean "//== Format" section of the `gen_`-function.

If the list of many things is similar to function parameters, use `crate::helpers::separated_items`, which uses this pattern internally.
