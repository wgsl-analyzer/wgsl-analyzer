# TODOs
- [Some more thoughts](https://discord.com/channels/1289346613185351722/1341941812675481680/1475555853066047549)
- Ignoring Code [Issue](https://github.com/wgsl-analyzer/wgsl-analyzer/issues/93)


# Opinions
## The formatter should not unnecessarily move comments around
...if programmer wants them there, we will let them have it.

Generally the formatter should leave comments where they are - even if they are in weird places. 
If the programmer put a comment to be in a strange place, they will have had a reason for it.
The formatter should try to detect the programmer's intent (i.e what they wanted to comment) and then when
things shift around during formatting, keep the comment in a place where that intent is preserved.
The formatter should *not* prevent the programmer from putting comments in strange places and should not impose opinions on where comments should belong.

## The formatter should fail when it encounters unexpected syntax
In case the formatter at any point mistakenly diverges from how wesl syntax is structured or new syntax is simply not yet implemented, the formatter should not error out
instead of accidentally destroying the programmer's code.

In many places the formatter expects certain items to be of a certain `SyntaxKind`, even tho that is not needed for the formatting itself.

# Tests
The tests in this crate are differentiated into **normative** and **descriptive** tests.
The *descriptive* tests are meant to represent the current state of the formatter, regardless of whether that state is correct or not.
They are as comprehensive as possible, with as many edge cases present as possible.
They are noisy and might even cover many cases all at once.
They enable us to make changes to the formatter internals and be certain to not cause accidental formatting changes.
If changes to the formatter are made and a change of formatting is expected (= breaking change), the *descriptive* tests can be updated with relatively little thought.


The *normative* tests are meant to represent how the formatter should be.
They are the result of pondering alternatives, discussion, past issues and embody the opinions that flow into the formatter.
They are documentation about the choices made when implementing the formatter in some way.
They should be terse, purposeful and targeted, and contain documentation as to why they are the way they are.
If changes to the formatter are made that would require changes to the *normative* tests, it might be a good idea
to gather opinions first, research what the initial intent behind the old state was, and decide if the new state is actually
a better default.

When issues with the formatter arise, those decisions should be documented as a *normative* test in order to prevent regressions.

## Tricks for debugging the crate

When compiled with the `prefer-immediate-crash` feature, the formatter will crash immediately when encountering a formatting error. This can be useful for debugging the tests, as a proper backtrace can be enabled with `RUST_BACKTRACE=1`.

```
RUST_BACKTRACE=1 cargo test --features=prefer-immediate-crash
```

* If you insert a strategic `dbg!(&syntax)` call into `check_with_options` inside `test_util.rs` code, you can see the AST nodes being processed at runtime.
* When the formatter errors, you can see the source position of the problematic AST nodes and can then locate that within the printed ast (see above). From there on you might want to look for the responsible `gen_...` function (see `generators/node.rs`).
* `NodeWithTrivia` implements `Debug`, so it's very useful to just `dbg!(&items)` the items that were returned by `parse_node_with` to see what trivia (comments etc) gets attached to which nodes.
* Strategically inserting `formatted.push_sc(sc!("|"))` calls into the `==== Format` section of the `gen_` functions can help you visualize which part of the code is responsible for which part of the formatted output.


# Guidelines on implementing formatting for new syntax constructs
1. Create normative and descriptive tests. The normative tests should encapsulate ideas and opinions about how code *should* be formatted (e.g where linebreaks should go or if colons after a switch get removed.). The descriptive tests document how code *is* formatted (to make sure we don't accidentally regress one part of the formatter wen fixing another). Do not forget to add tests using `check_comments` to make sure the formatter never accidentally removes or reorders the comments around your syntax construct.
2. Create a generator function in `generators::` - the function should be called `gen_`, take either a `NodeWithTrivia`, `SyntaxNode` or `ast::TheNewSyntaxConstruct` and usually return a `FormatDocumentResult`. The generator function is split up in a "Parse" and a "Format" region (see [Patterns.md](Patterns.md)).
3. Register the new syntax construct in the big `match` within `generators/node.rs`.
