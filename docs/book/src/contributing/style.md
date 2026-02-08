# Style

Our approach to "clean code" is two-fold:

- We generally do not block PRs on style changes.
- At the same time, all code in `wgsl-analyzer` is constantly refactored.

It is explicitly OK for a reviewer to flag only some nits in the PR, and then send a follow-up cleanup PR for things which are easier to explain by example, cc-ing the original author.
Sending small cleanup PRs (like renaming a single local variable) is encouraged.

When reviewing pull requests, prefer extending this document to leaving non-reusable comments on the pull request itself.

## General

### Scale of Changes

Everyone knows that it is better to send small & focused pull requests.
The problem is, sometimes you *have* to, for example, rewrite the whole compiler, and that just does not fit into a set of isolated PRs.

The main things to keep an eye on are the boundaries between various components.
There are three kinds of changes:

1. Internals of a single component are changed.
  Specifically, you do not change any `pub` items.
  A good example here would be an addition of a new assist.

2. API of a component is expanded.
  Specifically, you add a new `pub` function which was not there before.
  A good example here would be the expansion of the assist API, for example, to implement lazy assists or assist groups.

3. A new dependency between components is introduced.
  Specifically, you add a `pub use` re-export from another crate or you add a new line to the `[dependencies]` section of `Cargo.toml`.
  A good example here would be adding reference search capability to the assists crate.

For the first group, the change is generally merged as long as:

- it works for the happy case,
- it has tests,
- it does not panic for the unhappy case.

For the second group, the change would be subjected to quite a bit of scrutiny and iteration.
The new API needs to be right (or at least easy to change later).
The actual implementation does not matter that much.
It is very important to minimize the number of changed lines of code for changes of the second kind.
Often, you start doing a change of the first kind, only to realize that you need to elevate to a change of the second kind.
In this case, we will probably ask you to split API changes into a separate PR.

Changes of the third group should be pretty rare, so we do not specify any specific process for them.
That said, adding an innocent-looking `pub use` is a very simple way to break encapsulation, keep an eye on it!

Note: if you enjoyed this abstract hand-waving about boundaries, you might appreciate <https://www.tedinski.com/2018/02/06/system-boundaries.html>.

### Commit Style

We do not have specific rules around git history hygiene.
Maintaining clean git history is strongly encouraged, but not enforced.
Use rebase workflow, it is OK to rewrite history during the PR review process.
After you are happy with the state of the code, please use [interactive rebase](https://git-scm.com/book/en/v2/Git-Tools-Rewriting-History) to squash fixup commits.

Avoid @mentioning people in commit messages and pull request descriptions.
Such messages create a lot of duplicate notification traffic during rebases.

If possible, write Pull Request titles and descriptions from the user's perspective:

```text
## GOOD
Make goto definition work inside macros

## BAD
Use original span for FileId
```

This makes it easier to prepare a changelog.

If the change adds a new user-visible functionality, consider recording a GIF with [peek](https://github.com/phw/peek) and pasting it into the PR description.

To make writing the release notes easier, you can mark a pull request as a feature, fix, internal change, or minor.
Minor changes are excluded from the release notes, while the other types are distributed in their corresponding sections.
There are two ways to mark this:

Use a `feature:`, `fix:`, `internal:`, or `minor:` prefix in the PR title OR write `changelog [feature|fix|internal|skip] [description]` in a comment or in the PR description;
the description is optional and will replace the title if included.
This is picked up by tooling to create the changelog.

These comments do not have to be added by the PR author.
Editing a comment or the PR description or title is also fine, as long as it happens before the release.

**Rationale:** clean history is potentially useful, but rarely used.
But many users read changelogs.
Including a description and GIF suitable for the changelog means less work for the maintainers on the release day.

### Clippy

We use Clippy to improve the code. If a lint is not relevant (especially a restriction lint),
then `#[expect]` it or suggest to allow it in the workspace.

## Code

### Minimal Tests

Most tests in `wgsl-analyzer` start with a snippet of WESL code.
These snippets should be minimal.
If you copy-paste a snippet of real code into the tests, make sure to remove everything which could be removed.
It also makes sense to format snippets more compactly so long as they are still readable.
When using multiline fixtures, use unindented raw string literals:

```wgsl
    #[test]
    fn inline_local_variable() {
        check_assist(
            inline_local_variable,
            r#"
@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let $0x = 1.0;
    return vec4(x, 0.0, 0.0, 1.0);
}
"#,
            r#"
@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(1.0, 0.0, 0.0, 1.0);
}
"#,
        );
    }
```

**Rationale:**

There are many benefits to this:

- less to read or to scroll past
- easier to understand what exactly is tested
- less stuff printed during printf-debugging
- less time to run tests

Formatting ensures that you can use your editor's "number of selected characters" feature to correlate offsets with tests' source code.

### Marked Tests

Use [`cov_mark::hit! / cov_mark::check!`](https://github.com/matklad/cov-mark) when testing specific conditions.
Do not place several marks into a single test or condition.
Do not reuse marks between several tests.

**Rationale:** marks provide an easy way to find the canonical test for each bit of code.
This makes it much easier to understand.
More than one mark per test / code branch does not add significantly to understanding.

### `#[should_panic]`

Do not use `#[should_panic]` tests.
Instead, explicitly check for `None`, `Err`, etc.

**Rationale:** `#[should_panic]` is a tool for library authors to make sure that the API does not fail silently when misused.
`wgsl-analyzer` is not a library.
We do not need to test for API misuse, and we have to handle any user input without panics.
Panic messages in the logs from the `#[should_panic]` tests are confusing.

### `#[ignore]`

Do not `#[ignore]` tests.
If the test currently does not work, assert the wrong behavior and add a fixme explaining why it is wrong.

**Rationale:** noticing when the behavior is fixed, making sure that even the wrong behavior is acceptable (i.e., not a panic).

### Function Preconditions

Express function preconditions in types and force the caller to provide them (rather than checking in callee):

```rust
// GOOD
fn frobnicate(walrus: Walrus) {
    ...
}

// BAD
fn frobnicate(walrus: Option<Walrus>) {
    let walrus = match walrus {
        Some(it) => it,
        None => return,
    };
    ...
}
```

**Rationale:** this makes control flow explicit at the call site.
Call-site has more context.
It often happens that the precondition falls out naturally or can be bubbled up higher in the stack.

Avoid splitting precondition check and precondition use across functions:

```rust
// GOOD
fn main() {
    let string: &str = ...;
    if let Some(contents) = string_literal_contents(string) {

    }
}

fn string_literal_contents(string: &str) -> Option<&str> {
    if string.starts_with('"') && string.ends_with('"') {
        Some(&string[1..string.len() - 1])
    } else {
        None
    }
}

// BAD
fn main() {
    let string: &str = ...;
    if is_string_literal(string) {
        let contents = &string[1..string.len() - 1];
    }
}

fn is_string_literal(string: &str) -> bool {
    string.starts_with('"') && string.ends_with('"')
}
```

In the "Not as good" version, the precondition that `1` is a valid char boundary is checked in `is_string_literal` and used in `foo`.
In the "Good" version, the precondition check and usage are checked in the same block, and then encoded in the types.

**Rationale:** non-local code properties degrade under change.

### Control Flow

As a special case of the previous rule, do not hide control flow inside functions, push it to the caller:

```rust
// GOOD
if cond {
    foo();
}

fn foo() {
  ...
}

// BAD
bar();

fn bar() {
    if !cond {
        return;
    }
    ...
}
```

### Assertions

Assert liberally.
Prefer [`stdx::never!`](https://docs.rs/always-assert/0.1.2/always_assert/macro.never.html) to standard `assert!`.

**Rationale:** See [cross cutting concern: error handling](https://github.com/wgsl-analyzer/wgsl-analyzer/blob/main/docs/book/src/contributing/architecture.md#error-handling).

### Getters & Setters

If a field can have any value without breaking invariants, make the field public.
Conversely, if there is an invariant, document it, enforce it in the "constructor" function, make the field private, and provide a getter.
Never provide setters.

Getters should return borrowed data:

```rust
struct Person {
    // Invariant: never empty
    first_name: String,
    middle_name: Option<String>
}

// GOOD
impl Person {
    fn first_name(&self) -> &str { self.first_name.as_str() }
    fn middle_name(&self) -> Option<&str> { self.middle_name.as_ref() }
}

// BAD
impl Person {
    fn first_name(&self) -> String { self.first_name.clone() }
    fn middle_name(&self) -> &Option<String> { &self.middle_name }
}
```

**Rationale:** we do not provide public API.
It is cheaper to refactor than to pay getters rent.
Non-local code properties degrade under change.
Privacy makes invariant local.
Borrowed owned types (`&String`) disclose irrelevant details about internal representation.
Irrelevant (neither right nor wrong) things obscure correctness.

### Useless Types

More generally, always prefer types on the left

```rust
// GOOD      BAD
&[T]         &Vec<T>
&str         &String
Option<&T>   &Option<T>
&Path        &PathBuf
```

**Rationale:** types on the left are strictly more general.
Even when generality is not required, consistency is important.
Use the least powerful tool that gets the job done.

### Constructors

Prefer `Default` to zero-argument `new` function.

```rust
// GOOD
##[derive(Default)]
struct Foo {
    bar: Option<Bar>
}

// BAD
struct Foo {
    bar: Option<Bar>
}

impl Foo {
    fn new() -> Foo {
        Foo { bar: None }
    }
}
```

Prefer `Default` even if it has to be implemented manually.

**Rationale:** less typing in the common case, uniformity.

Use `Vec::new` rather than `vec![]`.

**Rationale:** uniformity, strength reduction.

Avoid using "dummy" states to implement a `Default`.
If a type does not have a sensible default, empty value, do not hide it.
Let the caller explicitly decide what the right initial state is.

### Functions Over Objects

Avoid creating "doer" objects.
That is, objects which are created only to execute a single action.

```rust
// GOOD
do_thing(arg1, arg2);

// BAD
ThingDoer::new(arg1, arg2).do();
```

Note that this concerns only outward API.
When implementing `do_thing`, it might be very useful to create a context object.

```rust
pub fn do_thing(
  an_input: Argument1,
  another_input: Argument2,
) -> Result {
    let mut context = Context { an_input, another_input };
    context.run()
}

struct Context {
    an_input: Argument1,
    another_input: Argument2,
}

impl Context {
    fn run(self) -> Result {
        ...
    }
}
```

The difference is that `Context` is an implementation detail here.

Sometimes a middle ground is acceptable if this can save some busywork:

```rust
ThingDoer::do(an_input, another_input);

pub struct ThingDoer {
    an_input: Argument1,
    another_input: Argument2,
}

impl ThingDoer {
    pub fn do(
        an_input: Argument1,
        another_input: Argument2,
    ) -> Result {
        ThingDoer { an_input, another_input }.run()
    }

    fn run(self) -> Result {
        ...
    }
}
```

**Rationale:** not bothering the caller with irrelevant details, not mixing user API with implementor API.

### Functions with many parameters

Avoid creating functions with many optional or boolean parameters.
Introduce a `Config` struct instead.

```rust
// GOOD
pub struct AnnotationConfig {
    pub binary_target: bool,
    pub annotate_runnables: bool,
    pub annotate_impls: bool,
}

pub fn annotations(
    db: &RootDatabase,
    file_id: FileId,
    config: AnnotationConfig
) -> Vec<Annotation> {
    ...
}

// BAD
pub fn annotations(
    db: &RootDatabase,
    file_id: FileId,
    binary_target: bool,
    annotate_runnables: bool,
    annotate_impls: bool,
) -> Vec<Annotation> {
    ...
}
```

**Rationale:** reducing churn.
If the function has many parameters, they most likely change frequently.
By packing them into a struct we protect all intermediary functions from changes.

Do not implement `Default` for the `Config` struct, the caller has more context to determine better defaults.
Do not store `Config` as a part of the `state`, pass it explicitly.
This gives more flexibility for the caller.

If there is variation not only in the input parameters, but in the return type as well, consider introducing a `Command` type.

```rust
// MAYBE GOOD
pub struct Query {
    pub name: String,
    pub case_sensitive: bool,
}

impl Query {
    pub fn all(self) -> Vec<Item> { ... }
    pub fn first(self) -> Option<Item> { ... }
}

// MAYBE BAD
fn query_all(name: String, case_sensitive: bool) -> Vec<Item> { ... }
fn query_first(name: String, case_sensitive: bool) -> Option<Item> { ... }
```

### Prefer Separate Functions Over Parameters

If a function has a `bool` or an `Option` parameter, and it is always called with `true`, `false`, `Some` and `None` literals, split the function in two.

```rust
// GOOD
fn caller_a() {
    foo()
}

fn caller_b() {
    foo_with_bar(Bar::new())
}

fn foo() { ... }
fn foo_with_bar(bar: Bar) { ... }

// BAD
fn caller_a() {
    foo(None)
}

fn caller_b() {
    foo(Some(Bar::new()))
}

fn foo(bar: Option<Bar>) { ... }
```

**Rationale:** more often than not, such functions display "false sharing" - they have additional `if` branching inside for two different cases.
Splitting the two different control flows into two functions simplifies each path, and remove cross-dependencies between the two paths.
If there is common code between `foo` and `foo_with_bar`, extract *that* into a common helper.

### Appropriate String Types

When interfacing with OS APIs, use `OsString`, even if the original source of data is UTF-8 encoded.
**Rationale:** cleanly delineates the boundary when the data goes into the OS-land.

Use `AbsPathBuf` and `AbsPath` over `std::Path`.
**Rationale:** `wgsl-analyzer` is a long-lived process which handles several projects at the same time.
It is important not to leak cwd by accident.

## Premature Pessimization

### Avoid Allocations

Avoid writing code which is slower than it needs to be.
Do not allocate a `Vec` where an iterator would do, do not allocate strings needlessly.

```rust
// GOOD
use itertools::Itertools;

let (first_word, second_word) = match text.split_ascii_whitespace().collect_tuple() {
    Some(it) => it,
    None => return,
}

// BAD
let words = text.split_ascii_whitespace().collect::<Vec<_>>();
if words.len() != 2 {
    return
}
```

**Rationale:** not allocating is almost always faster.

### Push Allocations to the Call Site

If allocation is inevitable, let the caller allocate the resource:

```rust
// GOOD
fn frobnicate(string: String) {
    ...
}

// BAD
fn frobnicate(string: &str) {
    let string = string.to_string();
    ...
}
```

**Rationale:** reveals the costs.
It is also more efficient when the caller already owns the allocation.

### Collection Types

Prefer `rustc_hash::FxHashMap` and `rustc_hash::FxHashSet` instead of the ones in `std::collections`.

**Rationale:** they use a hasher that is significantly faster and using them consistently will reduce code size by some small amount.

### Avoid Intermediate Collections

When writing a recursive function to compute a set of things, use an accumulator parameter instead of returning a fresh collection.
The accumulator goes first in the list of arguments.

```rust
// GOOD
pub fn reachable_nodes(node: Node) -> FxHashSet<Node> {
    let mut result = FxHashSet::default();
    go(&mut result, node);
    result
}
fn go(acc: &mut FxHashSet<Node>, node: Node) {
    acc.insert(node);
    for n in node.neighbors() {
        go(acc, n);
    }
}

// BAD
pub fn reachable_nodes(node: Node) -> FxHashSet<Node> {
    let mut result = FxHashSet::default();
    result.insert(node);
    for n in node.neighbors() {
        result.extend(reachable_nodes(n));
    }
    result
}
```

**Rationale:** re-use allocations, accumulator style is more concise for complex cases.

### Avoid Monomorphization

Avoid making a lot of code type parametric, *especially* on the boundaries between crates.

```rust
// GOOD
fn frobnicate(function: impl FnMut()) {
    frobnicate_impl(&mut function)
}
fn frobnicate_impl(function: &mut dyn FnMut()) {
    // lots of code
}

// BAD
fn frobnicate(function: impl FnMut()) {
    // lots of code
}
```

Avoid `AsRef` polymorphism, it pays back only for widely used libraries:

```rust
// GOOD
fn frobnicate(foo: &Path) {
}

// BAD
fn frobnicate(foo: impl AsRef<Path>) {
}
```

**Rationale:** Rust uses monomorphization to compile generic code, meaning that for each instantiation of a generic function with concrete types, the function is compiled afresh, *per crate*.
This allows for exceptionally good performance, but leads to increased compile times.
Runtime performance obeys the 80%/20% rule -- only a small fraction of code is hot.
Compile time **does not** obey this rule -- all code has to be compiled.

## Code Style

### Order of Imports

Separate import groups with blank lines.
Use one `use` per crate.

Module declarations come before the imports.
Order them in "suggested reading order" for a person new to the code base.

```rust
mod x;
mod y;

// First std.
use std::{ ... }

// Second, external crates (both crates.io crates and other wgsl-analyzer crates).
use crate_foo::{ ... }
use crate_bar::{ ... }

// Then current crate.
use crate::{}

// Finally, parent and child modules, but prefer `use crate::`.
use super::{}

// Re-exports are treated as item definitions rather than imports, so they go
// after imports and modules. Use them sparingly.
pub use crate::x::Z;
```

**Rationale:** consistency.
Reading order is important for new contributors.
Grouping by crate allows spotting unwanted dependencies easier.

### Import Style

Qualify items from `hir` and `ast`.

```rust
// GOOD
use syntax::ast;

fn frobnicate(func: hir::Function, r#struct: ast::Struct) {}

// BAD
use hir::Function;
use syntax::ast::Struct;

fn frobnicate(func: Function, r#struct: Struct) {}
```

**Rationale:** avoids name clashes, makes the layer clear at a glance.

When implementing traits from `std::fmt` or `std::ops`, import the module:

```rust
// GOOD
use std::fmt;

impl fmt::Display for RenameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result { .. }
}

// BAD
impl std::fmt::Display for RenameError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { .. }
}

// BAD
use std::ops::Deref;

impl Deref for Widget {
    type Target = str;
    fn deref(&self) -> &str { .. }
}
```

**Rationale:** overall, less typing.
Makes it clear that a trait is implemented, rather than used.

Avoid local `use MyEnum::*` imports.
**Rationale:** consistency.

Prefer `use crate::foo::bar` to `use super::bar` or `use self::bar::baz`.
**Rationale:** consistency, this is the style which works in all cases.

By default, avoid re-exports.
**Rationale:** for non-library code, re-exports introduce two ways to use something and allow for inconsistency.

### Order of Items

Optimize for the reader who sees the file for the first time, and wants to get a general idea about what is going on.
People read things from top to bottom, so place most important things first.

Specifically, if all items except one are private, always put the non-private item on top.

```rust
// GOOD
pub(crate) fn frobnicate() {
    Helper::act()
}

##[derive(Default)]
struct Helper { stuff: i32 }

impl Helper {
    fn act(&self) {

    }
}

// BAD
##[derive(Default)]
struct Helper { stuff: i32 }

pub(crate) fn frobnicate() {
    Helper::act()
}

impl Helper {
    fn act(&self) {

    }
}
```

If there is a mixture of private and public items, put public items first.

Put `struct`s and `enum`s first, functions and impls last. Order type declarations in top-down manner.

```rust
// GOOD
struct Parent {
    children: Vec<Child>
}

struct Child;

impl Parent {
}

impl Child {
}

// BAD
struct Child;

impl Child {
}

struct Parent {
    children: Vec<Child>
}

impl Parent {
}
```

**Rationale:** easier to get the sense of the API by visually scanning the file.
If function bodies are folded in the editor, the source code should read as documentation for the public API.

### Context Parameters

Some parameters are threaded unchanged through many function calls.
They determine the "context" of the operation.
Pass such parameters first, not last.
If there are several context parameters, consider packing them into a `struct Ctx` and passing it as `&self`.

```rust
// GOOD
fn dfs(graph: &Graph, v: Vertex) -> usize {
    let mut visited = FxHashSet::default();
    return go(graph, &mut visited, v);

    fn go(graph: &Graph, visited: &mut FxHashSet<Vertex>, v: usize) -> usize {
        ...
    }
}

// BAD
fn dfs(v: Vertex, graph: &Graph) -> usize {
    fn go(v: usize, graph: &Graph, visited: &mut FxHashSet<Vertex>) -> usize {
        ...
    }

    let mut visited = FxHashSet::default();
    go(v, graph, &mut visited)
}
```

**Rationale:** consistency.
Context-first works better when non-context parameter is a lambda.

### Variable Naming

<https://www.youtube.com/watch?v=-J3wNP6u5YU>

Use boring and long names for local variables (yay code completion).
The default name is a lowercased name of the type: `global_state: GlobalState`.
Avoid all acronyms and contractions unless it is overwhelmingly appropriate.
Use American spelling (color, behavior).

Many names in `wgsl-analyzer` conflict with keywords.
We use `r#ident` syntax where necessary.

```text
crate  -> r#crate
enum   -> r#enum
fn     -> r#fn
impl   -> r#impl
mod    -> r#mod
struct -> r#struct
trait  -> r#trait
type   -> r#type
```

**Rationale:** idiomatic, clarity.

### Error Handling Trivia

Prefer `anyhow::Result` over `Result`.

**Rationale:** makes it immediately clear what result that is.

Prefer `anyhow::format_err!` over `anyhow::anyhow`.

**Rationale:** consistent, boring, avoids stuttering.

[Error messages are typically concise lowercase sentences without trailing punctuation.](https://doc.rust-lang.org/stable/std/error/trait.Error.html)

### Early Returns

*Do* use early returns

```rust
// GOOD
fn foo() -> Option<Bar> {
    if !condition() {
        return None;
    }

    Some(...)
}

// BAD
fn foo() -> Option<Bar> {
    if condition() {
        Some(...)
    } else {
        None
    }
}
```

**Rationale:** reduce cognitive stack usage.

Use `return Err(error)` to "throw" an error:

```rust
// GOOD
fn foo() -> Result<(), ()> {
    if condition {
        return Err(());
    }
    Ok(())
}

// BAD
fn foo() -> Result<(), ()> {
    if condition {
        Err(())?;
    }
    Ok(())
}
```

**Rationale:** `return` has type `!`, which allows the compiler to flag dead
code (`Err(...)?` is of unconstrained generic type `T`).

### Comparisons

When doing multiple comparisons use `<`/`<=`, avoid `>`/`>=`.

```rust
// GOOD
assert!(lo <= x && x <= hi);
assert!(r1 < l2 || r2 < l1);
assert!(x < y);
assert!(0 < x);

// BAD
assert!(x >= lo && x <= hi);
assert!(r1 < l2 || l1 > r2);
assert!(y > x);
assert!(x > 0);
```

**Rationale:** Less-then comparisons are more intuitive; they correspond spatially to [real line](https://en.wikipedia.org/wiki/Real_line).

### if-let

Avoid `if let ... { } else { }` construct; prefer `match`.

```rust
// GOOD
match context.expected_type.as_ref() {
    Some(expected_type) => completion_type == expected_type && !expected_type.is_unit(),
    None => false,
}

// BAD
if let Some(expected_type) = context.expected_type.as_ref() {
    completion_type == expected_type && !expected_type.is_unit()
} else {
    false
}
```

**Rationale:** `match` is almost always more compact.
The `else` branch can get a more precise pattern: `None` or `Err(_)` instead of `_`.

### Match Ergonomics

Do not use the `ref` keyword.

**Rationale:** consistency & simplicity.
`ref` was required before [match ergonomics](https://github.com/rust-lang/rfcs/blob/master/text/2005-match-ergonomics.md).
Today, it is redundant.
Between `ref` and mach ergonomics, the latter is more ergonomic in most cases, and is simpler (does not require a keyword).

### Empty Match Arms

Use `=> (),` when a match arm is intentionally empty:

```rust
// GOOD
match result {
    Ok(_) => (),
    Err(error) => error!("{}", error),
}

// BAD
match result {
    Ok(_) => {}
    Err(error) => error!("{}", error),
}
```

**Rationale:** consistency.

### Functional Combinators

Use high order monadic combinators like `map`, `then` when they are a natural choice; do not bend the code to fit into some combinator.
If writing a chain of combinators creates friction, replace them with control flow constructs: `for`, `if`, `match`.
Mostly avoid `bool::then` and `Option::filter`.

```rust
// GOOD
if !x.cond() {
    return None;
}
Some(x)

// BAD
Some(x).filter(|it| it.cond())
```

This rule is more "soft" then others, and boils down mostly to taste.
The guiding principle behind this rule is that code should be dense in computation, and sparse in the number of expressions per line.
The second example contains *less* computation -- the `filter` function is an indirection for `if`, it does not do any useful work by itself.
At the same time, it is more crowded -- it takes more time to visually scan it.

**Rationale:** consistency, playing to languages' strengths.
Rust has first-class support for imperative control flow constructs
like `for` and `if`, while functions are less first-class due to lack
of universal function type, currying, and non-first-class effects (`?`, `.await`).

### Turbofish

Prefer type ascription over the turbofish.
When ascribing types, avoid `_`

```rust
// GOOD
let mutable: Vec<T> = old.into_iter().map(|it| builder.make_mut(it)).collect();

// BAD
let mutable: Vec<_> = old.into_iter().map(|it| builder.make_mut(it)).collect();

// BAD
let mutable = old.into_iter().map(|it| builder.make_mut(it)).collect::<Vec<_>>();
```

**Rationale:** consistency, readability.
If compiler struggles to infer the type, the human would as well.
Having the result type specified up-front helps with understanding what the chain of iterator methods is doing.

### Helper Functions

Avoid creating single-use helper functions:

```rust
// GOOD
let buf = {
    let mut buf = get_empty_buf(&mut arena);
    buf.add_item(item);
    buf
};

// BAD
let buf = prepare_buf(&mut arena, item);

...

fn prepare_buf(arena: &mut Arena, item: Item) -> ItemBuf {
    let mut result = get_empty_buf(&mut arena);
    result.add_item(item);
    result
}
```

Exception: if you want to make use of `return` or `?`.

**Rationale:** single-use functions change frequently, adding or removing parameters adds churn.
A block serves just as well to delineate a bit of logic, but has access to all the context.
Re-using originally single-purpose function often leads to bad coupling.

### Local Helper Functions

Put nested helper functions at the end of the enclosing functions
(this requires using return statement).
Do not nest more than one level deep.

```rust
// GOOD
fn dfs(graph: &Graph, v: Vertex) -> usize {
    let mut visited = FxHashSet::default();
    return go(graph, &mut visited, v);

    fn go(graph: &Graph, visited: &mut FxHashSet<Vertex>, v: usize) -> usize {
        ...
    }
}

// BAD
fn dfs(graph: &Graph, v: Vertex) -> usize {
    fn go(graph: &Graph, visited: &mut FxHashSet<Vertex>, v: usize) -> usize {
        ...
    }

    let mut visited = FxHashSet::default();
    go(graph, &mut visited, v)
}
```

**Rationale:** consistency, improved top-down readability.

### Helper Variables

Introduce helper variables freely, especially for multiline conditions:

```rust
// GOOD
let wgslfmt_not_installed =
    captured_stderr.contains("not installed") || captured_stderr.contains("not available");

match output.status.code() {
    Some(1) if !wgslfmt_not_installed => Ok(None),
    _ => Err(format_err!("wgslfmt failed:\n{}", captured_stderr)),
};

// BAD
match output.status.code() {
    Some(1)
        if !captured_stderr.contains("not installed")
           && !captured_stderr.contains("not available") => Ok(None),
    _ => Err(format_err!("wgslfmt failed:\n{}", captured_stderr)),
};
```

**Rationale:** Like blocks, single-use variables are a cognitively cheap abstraction, as they have access to all the context.
Extra variables help during debugging, they make it easy to print/view important intermediate results.
Giving a name to a condition inside an `if` expression often improves clarity and leads to nicely formatted code.

### Token names

Use `T![foo]` instead of `SyntaxKind::FOO_KW`.

```rust
// GOOD
match p.current() {
    T![true] | T![false] => true,
    _ => false,
}

// BAD
match p.current() {
    SyntaxKind::TRUE_KW | SyntaxKind::FALSE_KW => true,
    _ => false,
}
```

**Rationale:** The macro uses the familiar Rust syntax, avoiding ambiguities like "is this a brace or bracket?".

### Documentation

Style inline code comments as proper sentences.
Start with a capital letter, end with a dot.

```rust
// GOOD

// Only simple single segment paths are allowed.
MergeBehavior::Last => {
    tree.use_tree_list().is_none() && tree.path().map(path_len) <= Some(1)
}

// BAD

// only simple single segment paths are allowed
MergeBehavior::Last => {
    tree.use_tree_list().is_none() && tree.path().map(path_len) <= Some(1)
}
```

**Rationale:** writing a sentence (or maybe even a paragraph) rather just "a comment" creates a more appropriate frame of mind.
It tricks you into writing down more of the context you keep in your head while coding.

For `.md` files, prefer a sentence-per-line format, do not wrap lines.
If the line is too long, you might want to split the sentence in two.

**Rationale:** much easier to edit the text and read the diff, see [this link](https://asciidoctor.org/docs/asciidoc-recommended-practices/#one-sentence-per-line).
