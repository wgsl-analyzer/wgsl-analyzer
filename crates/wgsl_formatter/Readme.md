# WGSL Formatter (Library)

## TODOs

- Should we throw away the snapshot tests? they are kinda unnecessary
- Consider only allowing breaks before arithmetic expressions if both sides are not (function calls, paren-exprs, etc...)
  in order to avoid lonely + 1.0; broken on the next line

## Opinions

### The formatter should not unnecessarily move comments around

...if programmer wants them there, we will let them have it.

Generally the formatter should leave comments where they are -
even if they are in weird places.
If the programmer put a comment to be in a strange place,
they will have had a reason for it.
The formatter should try to detect the programmer's intent
(i.e what they wanted to comment) and then when
things shift around during formatting, keep the comment in a place
where that intent is preserved.
The formatter should _not_ prevent the programmer from putting comments
in strange places and should not impose opinions on where comments should belong.

### The formatter should fail when it encounters unexpected syntax

In case the formatter at any point mistakenly diverges from how WESL syntax
is structured or new syntax is simply not yet implemented,
the formatter should not error out
instead of accidentally destroying the programmer's code.

In many places the formatter expects certain items to be of a
certain `SyntaxKind`, even tho that is not needed for the formatting itself.

### The formatter should not rely on unstable equilibriums

When the formatter makes decisions based on existing formatting, the effort required
to "convince" the formatter to to reach either possible formatting should be comparably equal.

Example of an acceptable rule that takes existing formatting into account:

```wesl
// Those stick together
fn a() {}
fn b() {}

// Those stay apart
fn a() {}

fn b() {}
```

The effort to reach either possible formatting is comparable - its inserting or deleting a single newline

Example of an hypothetical unacceptable rule that would take existing formatting into account:

```wesl
struct A {
  a: u32,                // These comments stay aligned
  b: u32,                // The b
  long_name_blaaaa: u32, // The c
}

// This would get turned into B
struct A1 {
  a: u32,               // These comments stay snugly fit to the fields
  b: u32,               // The b
  long_name: u32, // The c
}

struct B {
  a: u32, // These comments stay snugly fit to the fields
  b: u32, // The b
  long_name_blaaaa: u32, // The c
}
```

The formatting in `struct A` is "unstable", because if the programmer changes the length of a single variable.
(Possibly even via a refactor-rename action from a distance)
suddenly the whole format collapses and gets turned into a different one `B`.
Restoring the new format back to how it was, takes a lot of effort, manually aligning the
comments - which is frustrating.
It is unstable because there are many source formats that get turned into `A` but only
a single specific source format that would get turned into `B`.

## Tests

The tests in this crate are differentiated into
**normative** and **descriptive** tests.
The _descriptive_ tests are meant to represent the
current state of the formatter, regardless of whether that state is correct or not.
They are as comprehensive as possible, with as many edge cases present as possible.
They are noisy and might even cover many cases all at once.
They enable us to make changes to the formatter internals and be certain
to not cause accidental formatting changes.
If changes to the formatter are made and a change of formatting is expected
(= breaking change), the _descriptive_ tests can be updated with relatively
little thought.

The _normative_ tests are meant to represent how the formatter should be.
They are the result of pondering alternatives, discussion, past issues and
embody the opinions that flow into the formatter.
They are documentation about the choices made when implementing
the formatter in some way.
They should be terse, purposeful and targeted, and contain documentation
as to why they are the way they are.
If changes to the formatter are made that would require changes
to the _normative_ tests, it might be a good idea
to gather opinions first, research what the initial intent behind the old state
was, and decide if the new state is actually a better default.

When issues with the formatter arise, those decisions should be documented
as a _normative_ test in order to prevent regressions.

## Tricks for debugging the crate

When compiled with the `prefer-immediate-crash` feature, the formatter
will crash immediately when encountering a formatting error. This can be useful
for debugging the tests, as a proper backtrace can be enabled with `RUST*BACKTRACE=1`.

```bash
RUST*BACKTRACE=1 cargo test --features=prefer-immediate-crash
```

- If you insert a strategic `dbg!(&syntax)` call into `check*with*options`
  inside `test*util.rs` code, you can see the AST nodes being processed at runtime.
- When the formatter errors, you can see the source position of the problematic
  AST nodes and can then locate that within the printed ast (see above).
  From there on you might want to look for the responsible
  `gen*...` function (see `generators/node.rs`).
- `NodeWithTrivia` implements `Debug`, so it's very useful to just
  `dbg!(&items)` the items that were returned by `parse*node*with`
  to see what trivia (comments etc) gets attached to which nodes.
- Strategically inserting `formatted.push*sc(sc!("|"))` calls into
  the `==== Format` section of the `gen*` functions can help you visualize which
  part of the code is responsible for which part of the formatted output.

## Guidelines on implementing formatting for new syntax constructs

1. Create normative and descriptive tests. The normative tests should encapsulate
   ideas and opinions about how code _should_ be formatted (e.g where linebreaks
   should go or if colons after a switch get removed.). The descriptive tests
   document how code _is_ formatted (to make sure we don't accidentally regress
   one part of the formatter wen fixing another). Do not forget to add tests
   using `check*comments` to make sure the formatter never accidentally removes
   or reorders the comments around your syntax construct.
2. Create a generator function in `generators::` - the function should be
   called `gen*`, take either a `NodeWithTrivia`, `SyntaxNode` or
   `ast::TheNewSyntaxConstruct` and usually return a `FormatDocumentResult`.
   The generator function is split up in a "Parse" and a "Format" region (see [Patterns.md](Patterns.md)).
3. Register the new syntax construct in the big `match` within `generators/node.rs`.

## Future Improvements

### Newlinegroups

Currently the handling of new-line-groups is a bit annoying.
Newline-groups are a way of increasing and decreasing precedence of
inserting a linebreak at a particular point.
dprint only offers "start newlinegroup" and "finish newlinegroup" which
works fine in many cases, but leads to a few places where we start 5 newlinegroups
just to artificially discourage a linebreak at that point.

Many parts of the formatting currently are a delicate balance between where newlinegroups are
started and finished. The semantics of what code breaks where is not defined anywhere but instead
are spread throughout the whole codebase and there is a great deal of spooky action at a distance.

An ideal solution would probably look something like this:

- Have a way of specifying precedence of possible "parallel" breaks.
  This would be used to set precedence of things like chains of infix operators
  in a "flat" manner, without affecting the layout of lower down in the hierarchy.
  These "precedences" would be attached to points where we offer a linebreak using `.or*newline()`.
  Possibly the API would look like `.or*newline(linebreak::OPERATOR*ADD)`
- Have a way of specifying a "hierarchy" of linebreaks - just like the current newlinegroup api.
  This would be used to set precedence between "inside" function call args and "outside" them.
  This hierarchy would always be more important than the parallel break precedences.
- Possibly the parallel linebreak api could also be used to express things like
  "all possible newlines with `linebreak::OPERATOR*ADD` that exist parallel to each other
  always break together". This could obsolete the `MultilineGroup` api.
