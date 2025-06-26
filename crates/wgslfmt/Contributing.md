# Contributing

There are many ways to contribute to `wgslfmt`.
This document lays out what they are and has information on how to get started.
If you have any questions about contributing or need help with anything, please ask in the on [Discord](https://discord.gg/Fk9FQWx28k).
Feel free to also ask questions on issues, or file new issues specifically to get help.

All contributors are expected to follow our [Code of Conduct](CODE_OF_CONDUCT.md).

## Test and file issues

It would be really useful to have people use `wgslfmt` on their projects and file issues where it does something you do not expect.

## Create test cases

Having a strong test suite for a tool like this is essential.
It is very easy to create regressions.
Any tests you can add are very much appreciated.

The tests can be run with `cargo test`.
This does a number of things:

- runs the unit tests for a number of internal functions;
- makes sure that `wgslfmt` run on every file in `./tests/source/` is equal to its associated file in `./tests/target/`;
- runs idempotence tests on the files in `./tests/target/`. These files should not be changed by `wgslfmt`;
- checks that `wgslfmt`'s code is not changed by running on itself. This ensures that the project bootstraps.

Creating a test is as easy as creating a new file in `./tests/source/` and an equally named one in `./tests/target/`.
If it is only required that `wgslfmt` leaves a piece of code unformatted, it may suffice to only create a target file.

Whenever there is a discrepancy between the expected output when running tests, a colorised diff will be printed so that the offending line(s) can quickly be identified.

Without explicit settings, the tests will be run using `wgslfmt`'s default configuration.
It is possible to run a test using non-default settings in several ways:

- You can include configuration parameters in comments at the top of the file.
    For example: to use 3 spaces per tab, start your test with `// wgslfmt-tab_spaces: 3`.
    Just remember that the comment is part of the input, so include in both the source and target files! It is also possible to explicitly specify the name of the expected output file in the target directory.
    Use `// wgslfmt-target: filename.rs` for this.

- You can also specify a custom configuration by using the `wgslfmt-config` directive.
    `wgslfmt` will then use that toml file located in `./tests/config/` for its configuration.
    Including `// wgslfmt-config: small_tabs.toml` will run your test with the configuration file found at `./tests/config/small_tabs.toml`.

- The final option is used when the test source file contains no configuration parameter comments.
    In this case, the test harness looks for a configuration file with the same filename as the test file in the `./tests/config/` directory.
    A test source file named `test-indent.wesl` would need a configuration file named `test-indent.toml` in that directory.
    As an example, the `issue-1111.wesl` test file is configured by the file `./tests/config/issue-1111.toml`.

## Debugging

Some `rewrite_*` methods use the `debug!` macro for printing useful information.
These messages can be printed by using the environment variable `WGSLFMT_LOG=debug`.
These traces can be helpful in understanding which part of the code was used and get a better grasp on the execution flow.

## Hack

Here are some [good starting issues].

<!-- [good starting issues]: <https://github.com/wgsl-analyzer/wgslfmt/issues?q=is%3Aopen%20is%3Aissue%20label%3AD-Trivial> -->
[good starting issues]: <https://github.com/wgsl-analyzer/wgsl-analyzer/issues?q=is%3Aopen%20is%3Aissue%20label%3AD-Trivial%20label%3AA-wgslfmt>

If you have found areas which need polish and don not have issues, please submit a PR.
Do not feel that there needs to be a corresponding issue.

### Guidelines

If you add a new feature or fix a bug, also add a test.
Run `cargo test` before submitting a PR to ensure your patch passes all tests.

`wgslfmt` is pre-1.0 and makes no backward-compatibility commitment.

Avoid leaving `TODO`s in the code.
There are a few around, but I wish there were not.
If you leave a `TODO`, you must create and link an issue by its issue number.

### Run `wgslfmt` from source code

You may want to run a version of `wgslfmt` from source code as part of a test or while hacking on the `wgslfmt` codebase.

To run `wgslfmt` on a file:

```bash
cargo run --bin `wgslfmt` -- path/to/file.wgsl
```

### A quick tour of `wgslfmt`

`wgslfmt` is basically a pretty printer - that is, its mode of operation is to take an AST (abstract syntax tree) and print it in a nice way.
This includes staying under the maximum permitted width for a line.
In order to get that AST, we first have to parse the source text.
The implementation does not do anything too fancy, such as algebraic approaches to pretty printing.
Instead, it uses a heuristic approach, "manually" crafting a string for each AST node.
This results in a lot of code, but it is relatively simple.

The AST is a tree view of source code.
It carries all the semantic information about the code, but not all of the syntax.
In particular, we lose white space and comments (although doc comments are preserved).

There are different nodes for every kind of item and expression in WGSL/WESL.
For more details, see the source code: [ast.rs].

[ast.rs]: <../../crates/syntax/src/ast.rs>

Many nodes in the AST (but not all, annoyingly) have a `Span`.
A `Span` is a range in the source code.
It can easily be converted to a snippet of source text.
When the AST does not contain enough information for us, we rely heavily on `Span`s.
For example, we can look between spans to try and find comments, or parse a snippet to see how the user wrote their source code.

The downside of using the AST is that we miss some information - primarily white space and comments.
White space is sometimes significant, although mostly we want to ignore it and make our own.
We strive to reproduce all comments, but this is sometimes difficult.
The crufty corners of `wgslfmt` are where we hack around the absence of comments in the AST and try to recreate them as best we can.

Our primary tool here is to look between spans for text we have missed.
For example, in a function call `foo(a, b)`, we have spans for `a` and `b`.
In this case, there is only a comma and a single space between the end of `a` and the start of `b`, so there is nothing much to do.
But if we look at `foo(a /* a comment */, b)`, then between `a` and `b` we find the comment.

At a higher level, `wgslfmt` has machinery so that we account for text between "top level" items.
Then we can reproduce that text pretty much verbatim.
We only count spans we actually reformat, so if we cannot format a span it is not missed completely but is reproduced in the output without being formatted.
This is mostly handled in [src/missed_spans.rs](src/missed_spans.rs).
See also `FmtVisitor::last_pos` in [src/visitor.rs](src/visitor.rs).

#### Some important elements

At the highest level, `wgslfmt` uses a `Visitor` implementation called `FmtVisitor` to walk the AST.
This is in [src/visitor.rs](src/visitor.rs).
This is really just used to walk items, rather than the bodies of functions.
We also cover macros and attributes here.
Most methods of the visitor call out to `Rewrite` implementations that then walk their own children.

The `Rewrite` trait is defined in [src/rewrite.rs](src/rewrite.rs).
It is implemented for many things that can be rewritten, mostly AST nodes.
It has a single function, `rewrite`, which is called to rewrite `self` into an `Option<String>`.
The arguments are `width` which is the horizontal space we write into and `offset` which is how much we are currently indented from the lhs of the page.
We also take a context which contains information used for parsing, the current block indent, and a configuration (see below).

##### Rewrite and Indent

To understand the indents, consider

```wesl
fn foo(...) {
    bar(argument_one,
        baz());
}
```

When formatting the `bar` call we will format the arguments in order, after the first one we know we are working on multiple lines (imagine it is longer than written).
When it comes to the second argument, the indent we pass to `rewrite` is 8, which brings it under the first argument.
The current block indent (stored in the context) is 8.
The former is used for visual indenting (when objects are vertically aligned with some marker), the latter is used for block indenting (when objects are tabbed in from the lhs).
The width available for `baz()` will be the maximum width, minus the space used for indenting, minus the space used for the `);`.
(Note that actual argument formatting does not quite work like this, but it is close enough).

The `rewrite` function returns an `Option` - either we successfully rewrite and return the rewritten string for the caller to use, or we fail to rewrite and return `None`.
This could be because `wgslfmt` encounters something it does not know how to reformat, but more often it is because `wgslfmt` cannot fit the item into the required width.
How to handle this is up to the caller.
Often the caller just gives up, ultimately relying on the missed spans system to paste in the un-formatted source.
A better solution (although not performed in many places) is for the caller to shuffle around some of its other items to make more width, then call the function again with more space.

Since it is common for callers to bail out when a callee fails, we often use a `?` operator to make this pattern more succinct.

One way we might find out that we do not have enough space is when computing how much space we have.
Something like `available_space = budget - overhead`.
Since widths are unsized integers, this would cause underflow.
Therefore we use checked subtraction: `available_space = budget.checked_sub(overhead)?`.
`checked_sub` returns an `Option`, and if we would underflow `?` returns `None`, otherwise, we proceed with the computed space.

##### Rewrite of list-like expressions

Much of the syntax in WGSL and WESL is lists. For example: lists of arguments, lists of fields, and lists of array elements.
We have some generic code to handle lists, including how to space them in horizontal and vertical space, indentation, comments between items, and trailing separators.
However, since there are so many options, the code is a bit complex.
Look in [src/lists.rs](src/lists.rs).
`write_list` is the key function, and `ListFormatting` the key structure for configuration.
You will need to make a `ListItems` for input, this is usually done using `itemize_list`.

##### Configuration

`wgslfmt` strives to be highly configurable.
Often the first part of a patch is creating a configuration option for the feature you are implementing.
All handling of configuration options is done in [src/config/mod.rs](src/config/mod.rs).
Look for the `create_config!` macro at the end of the file for all the options.
The rest of the file defines a bunch of enums used for options and the machinery to produce the config struct and parse a config file.
Checking an option is done by accessing the correct field on the config struct, for example, `config.max_width()`.
Most functions have a `Config`, or one can be accessed via a visitor or context of some kind.
