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

# Parse functions
TODO

# NodeWithTrivia
TODO

# Expecting SyntaxKinds
TODO

# Requests
