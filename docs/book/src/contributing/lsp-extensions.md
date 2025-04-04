# LSP Extensions

<!---
crates/wgsl-analyzer/src/lsp/extensions.rs hash: a2fcb0c9c05299e6

If you need to change the above hash to make the test pass, please check whether you
need to adjust this doc as well and ping this issue:

https://github.com/DavidAnson/markdownlint/blob/main/doc/md036.md
--->

<!-- markdownlint-disable no-duplicate-heading -->

This document describes LSP extensions used by wgsl-analyzer.
It is a best-effort document; when in doubt, consult the source (and send a PR with clarification).
We aim to upstream all non-WGSL-specific extensions to the protocol, but this is not a top priority.
All capabilities are enabled via the `experimental` field of `ClientCapabilities` or `ServerCapabilities`.
Requests which we hope to upstream live under the `experimental/` namespace.
Requests, which are likely to always remain specific to `wgsl-analyzer`, are under the `wgsl-analyzer/` namespace.

If you want to be notified about the changes to this document, subscribe to [#171](https://github.com/wgsl-analyzer/wgsl-analyzer/issues/171).

<!-- toc -->

## Configuration in `initializationOptions`

**Upstream Issue:** <https://github.com/microsoft/language-server-protocol/issues/567>

The `initializationOptions` field of the `InitializeParameters` of the initialization request should contain the `"wgsl-analyzer"` section of the configuration.

`wgsl-analyzer` normally sends a `"workspace/configuration"` request with `{ "items": ["wgsl-analyzer"] }` payload.
However, the server cannot do this during initialization.
At the same time, some essential configuration parameters are needed early on, before servicing requests.
For this reason, we ask that `initializationOptions` contain the configuration, as if the server did make a `"workspace/configuration"` request.

If a language client does not know about `wgsl-analyzer`'s configuration options, it can get sensible defaults by doing any of the following:

- Not sending `initializationOptions`
- Sending `"initializationOptions": null`
- Sending `"initializationOptions": {}`

## Snippet `TextEdit`

**Upstream Issue:** <https://github.com/microsoft/language-server-protocol/issues/724>

**Experimental Client Capability:** `{ "snippetTextEdit": boolean }`

If this capability is set, `WorkspaceEdit`s returned from `codeAction` requests
and `TextEdit`s returned from `textDocument/onTypeFormatting` requests might contain `SnippetTextEdit`s instead of the usual `TextEdit`s:

```typescript
interface SnippetTextEdit extends TextEdit {
  insertTextFormat?: InsertTextFormat;
  annotationId?: ChangeAnnotationIdentifier;
}
```

```typescript
export interface TextDocumentEdit {
  textDocument: OptionalVersionedTextDocumentIdentifier;
  edits: (TextEdit | SnippetTextEdit)[];
}
```

When applying such code action or text edit, the editor should insert a snippet, with tab stops and placeholders.
At the moment, wgsl-analyzer guarantees that only a single `TextDocumentEdit` will have edits which can be `InsertTextFormat.Snippet`.
Any additional `TextDocumentEdit`s will only have edits which are `InsertTextFormat.PlainText`.

### Example

<!-- TODO real example -->
<!-- "Add `derive`" code action transforms `struct S;` into `#[derive($0)] struct S;` -->

### Unresolved Questions

- Where exactly are `SnippetTextEdit`s allowed (only in code actions at the moment)?
- Can snippets span multiple files? (so far, no)

## `CodeAction` Groups

**Upstream Issue:** <https://github.com/microsoft/language-server-protocol/issues/994>

**Experimental Client Capability:** `{ "codeActionGroup": boolean }`

If this capability is set, `CodeAction`s returned from the server contain an additional field, `group`:

```typescript
interface CodeAction {
  title: string;
  group?: string;
  ...
}
```

All code actions with the same `group` should be grouped under a single (extendable) entry in the lightbulb menu.
The set of actions `[ { title: "foo" }, { group: "frobnicate", title: "bar" }, { group: "frobnicate", title: "baz" }]` should be rendered as

```text
💡
  +-------------+
  | foo         |
  +-------------+-----+
  | frobnicate >| bar |
  +-------------+-----+
                | baz |
                +-----+
```

Alternatively, selecting `frobnicate` could present a user with an additional menu to choose between `bar` and `baz`.

### Example

```wgsl
fn foo() {
    let x: Entry/*cursor here*/ = todo!();
}
```

Invoking code action at this position will yield two code actions for importing `Entry` from either `collections::HashMap` or `collection::BTreeMap`, grouped under a single "import" group.

### Unresolved Questions

- Is a fixed two-level structure enough?
- Should we devise a general way to encode custom interaction protocols for GUI refactorings?

## Parent Module

**Upstream Issue:** <https://github.com/microsoft/language-server-protocol/issues/1002>

**Experimental Server Capability:** `{ "parentModule": boolean }`

This request is sent from client to server to handle "Goto Parent Module" editor action.

**Method:** `experimental/parentModule`

**Request:** `TextDocumentPositionParameters`

**Response:** `Location | Location[] | LocationLink[] | null`

### Example

```wgsl
// src/foo.wgsl
mod bar;
// src/bar.wgsl

/* cursor here*/
```

`experimental/parentModule` returns a single `Link` to the `mod foo;` declaration.

### Unresolved Question

- An alternative would be to use a more general "gotoSuper" request, which would work for super methods, super classes, and super modules.
  This is the approach IntelliJ Rust is taking.
  However, experience shows that super module (which generally has a feeling of navigation between files) should be separate.
  If you want super module, but the cursor happens to be inside an overridden function, the behavior with a single "gotoSuper" request is surprising.

## Join Lines

**Upstream Issue:** <https://github.com/microsoft/language-server-protocol/issues/992>

**Experimental Server Capability:** `{ "joinLines": boolean }`

This request is sent from client to server to handle "Join Lines" editor action.

**Method:** `experimental/joinLines`

**Request:**

```typescript
interface JoinLinesParameters {
    textDocument: TextDocumentIdentifier,
    /// Currently active selections/cursor offsets.
    /// This is an array to support multiple cursors.
    ranges: Range[],
}
```

**Response:** `TextEdit[]`

### Example

```wgsl
fn main() {
    /*cursor here*/let x = {
        92
    };
}
```

`experimental/joinLines` yields (curly braces are automagically removed)

```wgsl
fn foo() {
    let x = 92;
}
```

### Unresolved Question

- What is the position of the cursor after `joinLines`?
  Currently, this is left to editor's discretion, but it might be useful to specify on the server via snippets.
  However, it then becomes unclear how it works with multi cursor.

## On Enter

**Upstream Issue:** <https://github.com/microsoft/language-server-protocol/issues/1001>

**Experimental Server Capability:** `{ "onEnter": boolean }`

This request is sent from client to server to handle the <kbd>Enter</kbd> key press.

**Method:** `experimental/onEnter`

**Request:** `TextDocumentPositionParameters`

**Response:**

```typescript
SnippetTextEdit[]
```

### Example

```wgsl
fn foo() {
    // Some /*cursor here*/ docs
    let x = 92;
}
```

`experimental/onEnter` returns the following snippet

```wgsl
fn foo() {
    // Some
    // $0 docs
    let x = 92;
}
```

The primary goal of `onEnter` is to handle automatic indentation when opening a new line.
This is not yet implemented.
The secondary goal is to handle fixing up syntax, like continuing doc strings and comments, and escaping `\n` in string literals.

As proper cursor positioning is the main purpose of `onEnter`, it uses `SnippetTextEdit`.

### Unresolved Question

- How to deal with synchronicity of the request?
  One option is to require the client to block until the server returns the response.
  Another option is to do an operational transforms style merging of edits from client and server.
  A third option is to do a record-replay: client applies heuristic on enter immediately, then applies all the user's keypresses.
  When the server is ready with the response, the client rollbacks all the changes and applies the recorded actions on top of the correct response.
- How to deal with multiple carets?
- Should we extend this to arbitrary typed events and not just `onEnter`?

## Structural Search Replace (SSR)

**Experimental Server Capability:** `{ "ssr": boolean }`

This request is sent from client to server to handle structural search replace -- automated syntax tree based transformation of the source.

**Method:** `experimental/ssr`

**Request:**

```typescript
interface SsrParameters {
    /// Search query.
    /// The specific syntax is specified outside of the protocol.
    query: string,
    /// If true, only check the syntax of the query and do not compute the actual edit.
    parseOnly: boolean,
    /// The current text document.
    /// This and `position` will be used to determine in what scope paths in `query` should be resolved.
    textDocument: TextDocumentIdentifier;
    /// Position where SSR was invoked.
    position: Position;
    /// Current selections.
    /// Search/replace will be restricted to these if non-empty.
    selections: Range[];
}
```

**Response:**

```typescript
WorkspaceEdit
```

### Example

SSR with query `foo($a, $b) ==>> ($a).foo($b)` will transform, eg `foo(y + 5, z)` into `(y + 5).foo(z)`.

### Unresolved Question

- Probably needs search without replace mode
- Needs a way to limit the scope to certain files.

## Matching Brace

**Upstream Issue:** <https://github.com/microsoft/language-server-protocol/issues/999>

**Experimental Server Capability:** `{ "matchingBrace": boolean }`

This request is sent from client to server to handle "Matching Brace" editor action.

**Method:** `experimental/matchingBrace`

**Request:**

```typescript
interface MatchingBraceParameters {
    textDocument: TextDocumentIdentifier,
    /// Position for each cursor
    positions: Position[],
}
```

**Response:**

```typescript
Position[]
```

### Example

```wgsl
fn main() {
  let x: array<()/*cursor here*/> = array();
}
```

`experimental/matchingBrace` yields the position of `<`.
In many cases, matching braces can be handled by the editor.
However, some cases (like disambiguating between generics and comparison operations) need a real parser.
Moreover, it would be cool if editors did not need to implement even basic language parsing.

### Unresolved Question

- Should we return a nested brace structure, to allow [paredit](https://paredit.org/)-like actions of jump *out* of the current brace pair?
  This is how `SelectionRange` request works.
- Alternatively, should we perhaps flag certain `SelectionRange`s as being brace pairs?

## Open External Documentation

This request is sent from the client to the server to obtain web and local URL(s) for documentation related to the symbol under the cursor, if available.

**Method:** `experimental/externalDocs`

**Request:** `TextDocumentPositionParameters`

**Response:** `string | null`

## Local Documentation

**Experimental Client Capability:** `{ "localDocs": boolean }`

If this capability is set, the `Open External Documentation` request returned from the server will have the following structure:

```typescript
interface ExternalDocsResponse {
    web?: string;
    local?: string;
}
```

## Analyzer Status

**Method:** `wgsl-analyzer/analyzerStatus`

**Request:**

```typescript
interface AnalyzerStatusParameters {
    textDocument?: TextDocumentIdentifier;
}
```

**Response:** `string`

Returns internal status message, mostly for debugging purposes.

## Reload Workspace

**Method:** `wgsl-analyzer/reloadWorkspace`

**Request:** `null`

**Response:** `null`

Reloads project information (that is, re-executes `cargo metadata`).

## Server Status

**Experimental Client Capability:** `{ "serverStatusNotification": boolean }`

**Method:** `experimental/serverStatus`

**Notification:**

```typescript
interface ServerStatusParameters {
    /// `ok` means that the server is completely functional.
    ///
    /// `warning` means that the server is partially functional.
    /// It can answer correctly to most requests, but some results
    /// might be wrong due to, for example, some missing dependencies.
    ///
    /// `error` means that the server is not functional.
    /// For example, there is a fatal build configuration problem.
    /// The server might still give correct answers to simple requests,
    /// but most results will be incomplete or wrong.
    health: "ok" | "warning" | "error",
    /// Is there any pending background work which might change the status?
    /// For example, are dependencies being downloaded?
    quiescent: boolean,
    /// Explanatory message to show on hover.
    message?: string,
}
```

This notification is sent from server to client.
The client can use it to display *persistent* status to the user (in modline).
It is similar to the `showMessage`, but is intended for states rather than point-in-time events.

Note that this functionality is intended primarily to inform the end user about the state of the server.
In particular, it is valid for the client to completely ignore this extension.
Clients are discouraged from but are allowed to use the `health` status to decide if it is worth sending a request to the server.

### Controlling Flycheck

The flycheck/checkOnSave feature can be controlled via notifications sent by the client to the server.

**Method:** `wgsl-analyzer/runFlycheck`

**Notification:**

```typescript
interface RunFlycheckParameters {
    /// The text document whose cargo workspace flycheck process should be started.
    /// If the document is null or does not belong to a cargo workspace all flycheck processes will be started.
    textDocument: lc.TextDocumentIdentifier | null;
}
```

Triggers the flycheck processes.

**Method:** `wgsl-analyzer/clearFlycheck`

**Notification:**

```typescript
interface ClearFlycheckParameters {}
```

Clears the flycheck diagnostics.

**Method:** `wgsl-analyzer/cancelFlycheck`

**Notification:**

```typescript
interface CancelFlycheckParameters {}
```

Cancels all running flycheck processes.

## Syntax Tree

**Method:** `wgsl-analyzer/syntaxTree`

**Request:**

```typescript
interface SyntaxTreeParameters {
    textDocument: TextDocumentIdentifier,
    range?: Range,
}
```

**Response:** `string`

Returns textual representation of a parse tree for the file/selected region.
Primarily for debugging, but very useful for all people working on wgsl-analyzer itself.

## View Syntax Tree

**Method:** `wgsl-analyzer/viewSyntaxTree`

**Request:**

```typescript
interface ViewSyntaxTreeParameters {
    textDocument: TextDocumentIdentifier,
}
```

**Response:** `string`

Returns json representation of the file's syntax tree.
Used to create a treeView for debugging and working on wgsl-analyzer itself.

## View File Text

**Method:** `wgsl-analyzer/viewFileText`

**Request:** `TextDocumentIdentifier`

**Response:** `string`

Returns the text of a file as seen by the server.
This is for debugging file sync problems.

## View ItemTree

**Method:** `wgsl-analyzer/viewItemTree`

**Request:**

```typescript
interface ViewItemTreeParameters {
    textDocument: TextDocumentIdentifier,
}
```

**Response:** `string`

Returns a textual representation of the `ItemTree` of the currently open file, for debugging.

## Hover Actions

**Experimental Client Capability:** `{ "hoverActions": boolean }`

If this capability is set, the `Hover` request returned from the server might contain an additional field, `actions`:

```typescript
interface Hover {
    ...
    actions?: CommandLinkGroup[];
}

interface CommandLink extends Command {
    /**
     * A tooltip for the command, when represented in the UI.
     */
    tooltip?: string;
}

interface CommandLinkGroup {
    title?: string;
    commands: CommandLink[];
}
```

Such actions on the client side are appended to a hover bottom as command links:

```text
  +-----------------------------+
  | Hover content               |
  |                             |
  +-----------------------------+
  | _Action1_ | _Action2_       |  <- first group, no TITLE
  +-----------------------------+
  | TITLE _Action1_ | _Action2_ |  <- second group
  +-----------------------------+
  ...
```

## Related tests

This request is sent from client to server to get the list of tests for the specified position.

**Method:** `wgsl-analyzer/relatedTests`

**Request:** `TextDocumentPositionParameters`

**Response:** `TestInfo[]`

```typescript
interface TestInfo {
    runnable: Runnable;
}
```

## Hover Range

**Upstream Issue:** <https://github.com/microsoft/language-server-protocol/issues/377>

**Experimental Server Capability:** { "hoverRange": boolean }

This extension allows passing a `Range` as a `position` field of `HoverParameters`.
The primary use-case is to use the hover request to show the type of the expression currently selected.

```typescript
interface HoverParameters extends WorkDoneProgressParameters {
    textDocument: TextDocumentIdentifier;
    position: Range | Position;
}
```

Whenever the client sends a `Range`, it is understood as the current selection and any hover included in the range will show the type of the expression if possible.

### Example

```wgsl
fn main() {
    let expression = $01 + 2 * 3$0;
}
```

Triggering a hover inside the selection above will show a result of `i32`.

## Move Item

**Upstream Issue:** <https://github.com/rust-lang/rust-analyzer/issues/6823>

This request is sent from client to server to move item under cursor or selection in some direction.

**Method:** `experimental/moveItem`

**Request:** `MoveItemParameters`

**Response:** `SnippetTextEdit[]`

```typescript
export interface MoveItemParameters {
    textDocument: TextDocumentIdentifier,
    range: Range,
    direction: Direction
}

export const enum Direction {
    Up = "Up",
    Down = "Down"
}
```

## Workspace Symbols Filtering

**Upstream Issue:** <https://github.com/microsoft/language-server-protocol/issues/941>

**Experimental Server Capability:** `{ "workspaceSymbolScopeKindFiltering": boolean }`

Extends the existing `workspace/symbol` request with ability to filter symbols by broad scope and kind of symbol.
If this capability is set, `workspace/symbol` parameter gains two new optional fields:

```typescript
interface WorkspaceSymbolParameters {
    /**
     * Return only the symbols of specified kinds.
     */
    searchKind?: WorkspaceSymbolSearchKind;
    ...
}

const enum WorkspaceSymbolSearchKind {
    OnlyTypes = "onlyTypes",
    AllSymbols = "allSymbols"
}
```

## Client Commands

**Upstream Issue:** <https://github.com/microsoft/language-server-protocol/issues/642>

**Experimental Client Capability:** `{ "commands?": ClientCommandOptions }`

Certain LSP types originating on the server, notably code lenses, embed commands.
Commands can be serviced either by the server or by the client.
However, the server does not know which commands are available on the client.

This extensions allows the client to communicate this info.

```typescript
export interface ClientCommandOptions {
    /**
     * The commands to be executed on the client
     */
    commands: string[];
}
```

## Colored Diagnostic Output

**Experimental Client Capability:** `{ "colorDiagnosticOutput": boolean }`

If this capability is set, the "full compiler diagnostics" provided by `checkOnSave`
will include ANSI color and style codes to render the diagnostic in a similar manner
as `cargo`. This is translated into `--message-format=json-diagnostic-rendered-ansi`
when flycheck is run, instead of the default `--message-format=json`.

The full compiler rendered diagnostics are included in the server response
regardless of this capability:

```typescript
// https://microsoft.github.io/language-server-protocol/specifications/specification-current#diagnostic
export interface Diagnostic {
    ...
    data?: {
        /**
         * The human-readable compiler output as it would be printed to a terminal.
         * Includes ANSI color and style codes if the client has set the experimental
         * `colorDiagnosticOutput` capability.
         */
        rendered?: string;
    };
}
```

## View Recursive Memory Layout

**Method:** `wgsl-analyzer/viewRecursiveMemoryLayout`

**Request:** `TextDocumentPositionParameters`

**Response:**

```typescript
export interface RecursiveMemoryLayoutNode = {
    /// Name of the item, or [ROOT], `.n` for tuples
    item_name: string;
    /// Full name of the type (type aliases are ignored)
    typename: string;
    /// Size of the type in bytes
    size: number;
    /// Alignment of the type in bytes
    alignment: number;
    /// Offset of the type relative to its parent (or 0 if its the root)
    offset: number;
    /// Index of the node's parent (or -1 if its the root)
    parent_index: number;
    /// Index of the node's children (or -1 if it does not have children)
    children_start: number;
    /// Number of child nodes (unspecified it does not have children)
    children_length: number;
};

export interface RecursiveMemoryLayout = {
    nodes: RecursiveMemoryLayoutNode[];
};
```

Returns a vector of nodes representing items in the datatype as a tree, `RecursiveMemoryLayout::nodes[0]` is the root node.

If `RecursiveMemoryLayout::nodes::length == 0` we could not find a suitable type.

Generic Types do not give anything because they are incomplete. Fully specified generic
types do not give anything if they are selected directly but do work when a child of
other types [this is consistent with other behavior](https://github.com/rust-lang/rust-analyzer/issues/15010).

### Unresolved questions

- How should enums/unions be represented? currently they do not produce any children because they have multiple distinct sets of children.
- Should niches be represented? currently they are not reported.
- A visual representation of the memory layout is not specified, see the provided implementation for an example, however it may not translate well to terminal based editors or other such things.
