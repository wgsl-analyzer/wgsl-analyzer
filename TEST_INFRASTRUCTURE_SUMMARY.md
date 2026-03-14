# Test Infrastructure Summary for wgsl-analyzer IDE Features

## Overview
This document summarizes the existing test infrastructure for IDE features (hover, completions, signature help, and dot completions) in the wgsl-analyzer project.

---

## 1. Test Fixture System

### Location
- **Main fixture module**: `crates/test-fixture/src/lib.rs`
- **IDE fixture helpers**: `crates/ide/src/fixture.rs`

### Key Components

#### `ChangeFixture` (test-fixture crate)
- **Purpose**: Parses test fixture strings and creates a database with files
- **Usage**: 
  ```rust
  let fixture = ChangeFixture::parse(source);
  let mut database = ide_db::RootDatabase::new(None);
  database.apply_change(fixture.change);
  ```
- **Cursor Markers**: Use `$0` to mark cursor position for single-point tests
- **Multi-file Support**: Can define multiple files in one fixture string

#### `WithFixture` Trait (test-fixture crate)
- **Methods**:
  - `with_single_file(fixture: &str) -> (Self, EditionedFileId)` - Single file with cursor
  - `with_many_files(fixture: &str) -> (Self, Vec<EditionedFileId>)` - Multiple files
  - `with_files(fixture: &str) -> Self` - Multiple files without cursor
  - `with_position(fixture: &str) -> (Self, FilePosition)` - Single file with cursor position
  - `with_range(fixture: &str) -> (Self, FileRange)` - Single file with range
  - `with_range_or_offset(fixture: &str) -> (Self, FileId, RangeOrOffset)` - Flexible positioning

#### IDE Fixture Helpers (`crates/ide/src/fixture.rs`)
- `single_file_db(source: &str) -> (Analysis, FileId)` - Creates Analysis for single file
- `multi_file_db(source: &str) -> (Analysis, Vec<EditionedFileId>)` - Creates Analysis for multiple files

---

## 2. Existing Tests

### Completions Tests
**Location**: `crates/ide_completion/src/lib.rs` (lines 76-275)

#### Test Helper Functions
```rust
fn get_completion_items(source: &str) -> Vec<crate::CompletionItem>
fn get_completions(source: &str) -> Vec<(CompletionItemKind, String)>
fn check_completions_contain(source: &str, expected_kind: CompletionItemKind, expected_labels: &[&str])
fn check_completions_absent(source: &str, labels: &[&str])
```

#### Existing Tests
1. **Keyword completions**
   - `keyword_completions_at_top_level()` - Tests fn, struct, var, const, alias, enable, requires
   - `keyword_completions_inside_function()` - Tests let, var, const, if, for, while, loop, switch, return, break, continue, discard
   - `no_statement_keywords_at_top_level()` - Ensures statement keywords don't appear at top level

2. **Type completions**
   - `type_completions_at_top_level()` - Tests f32, i32, u32, bool, vec3, vec4f, mat4x4, array
   - `type_completions_inside_function()` - Tests f32, vec3f, mat4x4f, sampler, texture_2d

3. **Attribute completions**
   - `attribute_completions_before_fn()` - Tests @vertex, @fragment, @compute
   - `no_attribute_completions_inside_function()` - Ensures attributes don't appear in function body

4. **Builtin completions**
   - `builtin_completions_have_signature_detail()` - Tests that builtins like `abs` have signature details

#### Test Pattern
```rust
#[test]
fn test_name() {
    check_completions_contain(
        "source_code_with_$0_cursor",
        CompletionItemKind::Keyword,
        &["expected", "completions"],
    );
}
```

### Dot Completions Tests
**Location**: `crates/ide_completion/src/completions/dot.rs` (lines 191-279)

#### Existing Tests
1. **Swizzle validation tests**
   - `is_swizzleable_valid()` - Tests valid swizzle patterns (rgba, xyzw)
   - `is_swizzleable_invalid()` - Tests invalid swizzle patterns

2. **Swizzle generation tests**
   - `possible_swizzles_is_correct()` - Tests swizzle generation for different vector sizes
   - `swizzler_is_correct()` - Tests swizzle character generation

#### Test Pattern
```rust
#[test]
fn test_name() {
    let swizzles: Vec<_> = possible_swizzles(2, "").collect();
    assert_eq!(swizzles, vec!["x", "y", "r", "g"]);
}
```

### Hover Tests
**Location**: `crates/ide/src/hover.rs`
- **Status**: No tests found in hover.rs itself
- **Note**: Tests should be added to test hover functionality

### Signature Help Tests
**Location**: `crates/ide/src/signature_help.rs`
- **Status**: No tests found in signature_help.rs itself
- **Note**: Tests should be added to test signature help functionality

---

## 3. Test Dependencies

### Required Crates
```toml
[dev-dependencies]
expect-test = "workspace"  # For expect! macro assertions
test-fixture = "workspace" # For fixture parsing and database creation
test-utils = "workspace"   # For test utilities (CURSOR_MARKER, etc.)
```

### Key Imports
```rust
use expect_test::{Expect, expect};
use test_fixture::{ChangeFixture, WithFixture};
use base_db::FilePosition;
use ide_db::RootDatabase;
```

---

## 4. How to Write Tests

### For Completions

#### Basic Pattern
```rust
#[test]
fn test_struct_field_completions() {
    check_completions_contain(
        "
struct Point {
    x: f32,
    y: f32,
}

fn test() {
    let p: Point;
    p.$0
}
",
        CompletionItemKind::Field,
        &["x", "y"],
    );
}
```

#### With Snippets
```rust
#[test]
fn test_function_completions_with_snippets() {
    let items = get_completion_items(
        "
fn my_func(a: f32, b: f32) -> f32 {
    a + b
}

fn test() {
    my_$0
}
",
    );
    let my_func = items
        .iter()
        .find(|item| item.label.primary == "my_func")
        .expect("Expected 'my_func' completion");
    
    // Check that it has a snippet
    assert!(my_func.insert_text.is_some());
}
```

### For Hover

#### Basic Pattern
```rust
#[test]
fn test_hover_struct_definition() {
    let (analysis, file_id) = single_file_db(
        "
struct Point {
    x: f32,
}

fn test() {
    let p: Point$0;
}
",
    );
    
    let file_range = FileRange {
        file_id,
        range: TextRange::at(TextSize::from(cursor_offset), TextSize::from(0)),
    };
    
    let config = HoverConfig {
        links_in_hover: false,
        memory_layout: None,
        documentation: true,
        keywords: true,
        format: HoverDocFormat::Markdown,
        max_fields_count: None,
        max_enum_variants_count: None,
        max_substitution_type_length: SubstitutionTypeLength::Unlimited,
    };
    
    let hover = analysis.hover(&config, file_range).unwrap();
    assert!(hover.is_some());
    let result = hover.unwrap();
    assert!(result.value.markup.as_str().contains("struct Point"));
}
```

### For Signature Help

#### Basic Pattern
```rust
#[test]
fn test_signature_help_builtin_function() {
    let (analysis, file_id) = single_file_db(
        "
fn test() {
    abs($0)
}
",
    );
    
    let position = FilePosition {
        file_id,
        offset: TextSize::from(cursor_offset),
    };
    
    let help = analysis.signature_help(position).unwrap();
    assert!(help.is_some());
    let sig_help = help.unwrap();
    assert!(!sig_help.signatures.is_empty());
    assert!(sig_help.signatures[0].label.contains("abs"));
}
```

### For Dot Completions

#### Basic Pattern
```rust
#[test]
fn test_dot_completion_struct_fields() {
    check_completions_contain(
        "
struct MyStruct {
    field_a: f32,
    field_b: vec3<f32>,
}

fn test() {
    let s: MyStruct;
    s.$0
}
",
        CompletionItemKind::Field,
        &["field_a", "field_b"],
    );
}

#[test]
fn test_dot_completion_vector_swizzles() {
    check_completions_contain(
        "
fn test() {
    let v: vec4<f32>;
    v.$0
}
",
        CompletionItemKind::Field,
        &["x", "y", "z", "w", "r", "g", "b", "a"],
    );
}
```

---

## 5. Key APIs and Structures

### CompletionItem
```rust
pub struct CompletionItem {
    pub kind: CompletionItemKind,
    pub label: CompletionLabel,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub insert_text: Option<String>,
    // ... other fields
}
```

### HoverResult
```rust
pub struct HoverResult {
    pub markup: Markup,
    pub actions: Vec<HoverAction>,
}
```

### SignatureHelp
```rust
pub struct SignatureHelp {
    pub signatures: Vec<SignatureInformation>,
    pub active_signature: Option<usize>,
    pub active_parameter: Option<u32>,
}

pub struct SignatureInformation {
    pub label: String,
    pub documentation: Option<String>,
    pub parameters: Vec<ParameterInformation>,
}
```

---

## 6. Test Configuration

### CompletionConfig for Tests
```rust
let config = CompletionConfig {
    enable_postfix_completions: false,
    enable_imports_on_the_fly: false,
    enable_self_on_the_fly: false,
    enable_auto_iter: false,
    enable_auto_await: false,
    enable_private_editable: false,
    enable_term_search: false,
    term_search_fuel: 400,
    full_function_signatures: false,
    callable: None,  // Set to Some(CallableSnippets::FillArguments) for snippet tests
    add_semicolon_to_unit: false,
    prefer_no_std: false,
    prefer_prelude: false,
    prefer_absolute: false,
    limit: None,
    fields_to_resolve: CompletionFieldsToResolve::empty(),
    exclude_flyimport: vec![],
};
```

### HoverConfig for Tests
```rust
let config = HoverConfig {
    links_in_hover: false,
    memory_layout: None,
    documentation: true,
    keywords: true,
    format: HoverDocFormat::Markdown,
    max_fields_count: None,
    max_enum_variants_count: None,
    max_substitution_type_length: SubstitutionTypeLength::Unlimited,
};
```

---

## 7. Running Tests

### Run all tests
```bash
cargo test
```

### Run specific test file
```bash
cargo test --lib ide_completion::tests
```

### Run specific test
```bash
cargo test --lib ide_completion::tests::keyword_completions_at_top_level
```

### Run with output
```bash
cargo test -- --nocapture
```

---

## 8. Best Practices

1. **Use `$0` marker** for cursor position in single-point tests
2. **Use `check_completions_contain`** for positive assertions
3. **Use `check_completions_absent`** for negative assertions
4. **Test both positive and negative cases**
5. **Include doc comments** in test code explaining what's being tested
6. **Use descriptive test names** that explain the scenario
7. **Keep fixtures minimal** - only include necessary code
8. **Test edge cases** - empty input, invalid syntax, etc.
9. **Group related tests** with comments like `// --- Category ---`
10. **Use `expect!` macro** for complex output validation

---

## 9. Multi-File Test Fixtures

### Example
```rust
#[test]
fn test_with_multiple_files() {
    let (analysis, files) = multi_file_db(
        "
//- /main.wgsl
struct Point {
    x: f32,
}

fn test() {
    let p: Point;
    p.$0
}

//- /other.wgsl
fn other_func() {}
",
    );
    
    // files[0] is main.wgsl
    // files[1] is other.wgsl
}
```

---

## 10. Summary of Test Locations

| Feature | Test Location | Status |
|---------|---------------|--------|
| Completions (general) | `crates/ide_completion/src/lib.rs` | ✅ Has tests |
| Dot completions | `crates/ide_completion/src/completions/dot.rs` | ✅ Has tests (swizzles only) |
| Hover | `crates/ide/src/hover.rs` | ❌ No tests |
| Signature Help | `crates/ide/src/signature_help.rs` | ❌ No tests |
| Expression completions | `crates/ide_completion/src/completions/expression.rs` | ❌ No tests |

---

## 11. Key Files to Reference

1. **Test fixture system**: `crates/test-fixture/src/lib.rs`
2. **IDE fixture helpers**: `crates/ide/src/fixture.rs`
3. **Completion tests**: `crates/ide_completion/src/lib.rs` (lines 76-275)
4. **Dot completion tests**: `crates/ide_completion/src/completions/dot.rs` (lines 191-279)
5. **Hover implementation**: `crates/ide/src/hover.rs`
6. **Signature help implementation**: `crates/ide/src/signature_help.rs`
7. **Completions implementation**: `crates/ide_completion/src/completions/expression.rs`
8. **Dot completions implementation**: `crates/ide_completion/src/completions/dot.rs`
