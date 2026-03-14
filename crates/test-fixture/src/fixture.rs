//! Defines `Fixture` -- a convenient way to describe the initial state of
//! wgsl-analyzer database from a single string.
//!
//! Fixtures are strings containing wesl source code with optional metadata.
//! A fixture without metadata is parsed into a single source file.
//! Use this to test functionality local to one file.
//!
//! Simple Example:
//!
//! ```ignore
//! r#"
//! fn main() {
//!     let a = abs(-3.5);
//! }
//! "#
//! ```
//!
//! Metadata can be added to a fixture after a `//-` comment.
//! The basic form is specifying filenames,
//! which is also how to define multiple files in a single test fixture.
//!
//! Example using two files in the same package:
//!
//! ```ignore
//! "
//! //- /main.wgsl
//! import foo;
//! fn main() {
//!     foo::bar();
//! }
//!
//! //- /foo.wgsl
//! pub fn bar() {}
//! "
//! ```
//!
//! Example using two packages with one file each, with one package depending on the other:
//!
//! ```ignore
//! r#"
//! //- /main.wgsl package:a deps:b
//! fn main() {
//!     b::foo();
//! }
//! //- /lib.wgsl package:b
//! pub fn b() {
//!     let a = abs(-3.5);
//! }
//! "#
//! ```

use std::iter;

use rustc_hash::FxHashMap;
use stdx::trim_indent;

#[derive(Debug, Eq, PartialEq)]
pub struct Fixture {
    /// Specifies the path for this file. It must start with "/".
    pub path: String,
    /// Defines a new package and make this file its root module.
    ///
    /// Version and repository URL of the package can optionally be specified; if
    /// either one is specified, the other must also be specified.
    ///
    /// Syntax:
    /// - `package:my_awesome_lib`
    /// - `package:my_awesome_lib@0.0.1,https://example.com/repo.git`
    pub package: Option<String>,
    /// Specifies dependencies of this package. This must be used with `package` meta.
    ///
    /// Syntax: `deps:hir-def,ide-assists`.
    pub deps: Vec<String>,
    /// Specifies the edition of this package. This must be used with `package` meta. If
    /// this is not specified, the current default edition will be used.
    /// This must be used with `package` meta.
    ///
    /// Syntax: `edition:2021`.
    pub edition: Option<String>,

    /// Introduces a new [source root](base_db::input::SourceRoot). This file **and
    /// the following files** will belong the new source root. This must be used
    /// with `package` meta.
    ///
    /// Use this if you want to test something that uses `SourceRoot::is_library()`
    /// to check editability.
    ///
    /// Note that files before the first fixture with `new_source_root` meta will
    /// belong to an implicitly defined local source root.
    ///
    /// Syntax:
    /// - `new_source_root:library`
    /// - `new_source_root:local`
    pub introduce_new_source_root: Option<String>,
    /// Explicitly declares this package as a library outside current workspace. This
    /// must be used with `package` meta.
    ///
    /// This is implied if this file belongs to a library source root.
    ///
    /// Use this if you want to test something that checks if a package is a workspace
    /// member via `PackageOrigin`.
    ///
    /// Syntax: `library`.
    pub library: bool,
    /// Actual file contents. All meta comments are stripped.
    pub text: String,
    /// The line number in the original fixture of the beginning of this fixture.
    pub line: usize,
}

#[derive(Debug)]
pub struct FixtureWithProjectMeta {
    pub fixture: Vec<Fixture>,
}

impl FixtureWithProjectMeta {
    /// Parses text which looks like this:
    ///
    ///  ```text
    ///  //- some meta
    ///  line 1
    ///  line 2
    ///  //- other meta
    ///  ```
    ///
    /// # Panics
    /// Panics if an invalid fixture is passed to it. This function is used only in tests.
    #[must_use]
    pub fn parse(wa_fixture: &str) -> Self {
        let fixture = trim_indent(wa_fixture);
        let mut result: Vec<Fixture> = Vec::new();
        let first_row: i32 = 0;

        let default = if fixture.contains("//- /") {
            None
        } else {
            Some((first_row - 1, "//- /main.wgsl"))
        };

        for (ix, line) in default
            .into_iter()
            .chain((first_row..).zip(fixture.split_inclusive('\n')))
        {
            if line.contains("//-") {
                assert!(
                    line.starts_with("//-"),
                    "Metadata line {ix} has invalid indentation. \
                     All metadata lines need to have the same indentation.\n\
                     The offending line: {line:?}"
                );
            }

            if let Some(line) = line.strip_prefix("//-") {
                let meta = Self::parse_meta_line(line, (ix + 1).try_into().unwrap());
                result.push(meta);
            } else {
                if let Some(metadata_line) = line.strip_prefix("// ")
                    && metadata_line.trim().starts_with('/')
                {
                    panic!("looks like invalid metadata line: {line:?}");
                }

                if let Some(entry) = result.last_mut() {
                    entry.text.push_str(line);
                }
            }
        }

        Self { fixture: result }
    }

    //- /lib.rs package:foo deps:bar,baz
    fn parse_meta_line(
        meta: &str,
        line: usize,
    ) -> Fixture {
        let meta = meta.trim();
        let mut components = meta.split_ascii_whitespace();

        let path = components
            .next()
            .expect("fixture meta must start with a path")
            .to_owned();
        assert!(
            path.starts_with('/'),
            "fixture path does not start with `/`: {path:?}"
        );

        let mut package = None;
        let mut deps = Vec::new();
        let mut edition = None;
        let mut introduce_new_source_root = None;
        let mut library = false;
        for component in components {
            if component == "library" {
                library = true;
                continue;
            }

            let (key, value) = component
                .split_once(':')
                .unwrap_or_else(|| panic!("invalid meta line: {meta:?}"));
            match key {
                "package" => package = Some(value.to_owned()),
                "deps" => deps = value.split(',').map(ToOwned::to_owned).collect(),

                "edition" => edition = Some(value.to_owned()),
                "new_source_root" => introduce_new_source_root = Some(value.to_owned()),
                _ => panic!("bad component: {component:?}"),
            }
        }

        Fixture {
            path,
            package,
            deps,
            edition,
            introduce_new_source_root,
            library,
            text: String::new(),
            line,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(
        expected = "Metadata line 4 has invalid indentation. All metadata lines need to have the same indentation."
    )]
    fn parse_fixture_checks_further_indented_metadata() {
        FixtureWithProjectMeta::parse(
            r"
        //- /lib.rs
          mod bar;

          fn foo() {}
          //- /bar.rs
          pub fn baz() {}
          ",
        );
    }

    #[test]
    fn parse_fixture_gets_full_meta() {
        let FixtureWithProjectMeta { fixture: parsed } = FixtureWithProjectMeta::parse(
            r#"
//- /lib.rs package:foo deps:bar,baz
const a = 3;
"#,
        );

        assert_eq!(1, parsed.len());

        let meta = &parsed[0];
        assert_eq!("const a = 3;\n", meta.text);

        assert_eq!("foo", meta.package.as_ref().unwrap());
        assert_eq!("/lib.rs", meta.path);
    }
}
