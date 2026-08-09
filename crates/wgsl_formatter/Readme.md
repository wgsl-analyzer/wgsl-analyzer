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



# TODOs
- [Some more thoughts](https://discord.com/channels/1289346613185351722/1341941812675481680/1475555853066047549)
- Ignoring Code [Issue](https://github.com/wgsl-analyzer/wgsl-analyzer/issues/93)
