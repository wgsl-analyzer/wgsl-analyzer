use criterion::{Criterion, criterion_group, criterion_main};
use sha2::{Digest as _, Sha256};
use std::{fmt::Write as _, hint::black_box};
use syntax::{AstNode as _, ast::SourceFile};
use wgsl_formatter::{FormattingOptions, format_tree};

const SOURCE: &str = include_str!("large_file.wesl");
// DO NOT CHANGE!!
// unless you actually want to change this.
// In order to get meaningful readings over time, the input to the formatter in the benchmarks
// should remain constant. We hash it to ensure it does not accidentally get formatted or changed
// by save on Ctrl-S, `wgslfmt .`, or other means.
const SOURCE_SHA256: &str = "12d8e60e8bfcbdc6f4e3b61d65c64d6457c4ba35b60f1fab79c2f0b514236f1b";

fn large_file(criterion: &mut Criterion) {
    let parse = parser::parse_entrypoint(
        SOURCE,
        parser::ParseEntryPoint::File,
        parser::Edition::LATEST,
    );
    assert!(
        parse.errors().is_empty(),
        "Encountered parse errors in benchmark input: {:#?}",
        parse.errors()
    );

    {
        // Nail down the input source, so that it does not get accidentally formatted/changed on hooks or a CI run.
        let hash = Sha256::digest(SOURCE);
        let hex = hash.iter().fold(String::new(), |mut output, byte| {
            write!(output, "{byte:02x}").expect("should be able to convert hash to hex");
            output
        });
        assert_eq!(
            hex,
            // ARE YOU
            SOURCE_SHA256
        );
    }

    let tree = parse.syntax();
    let source = SourceFile::cast(tree).expect("The file should parse into a SourceFile");

    criterion.bench_function("large_file_default", |bench| {
        bench.iter(|| format_tree(&source, &FormattingOptions::default()));
    });
}

criterion_group!(benches, large_file);
criterion_main!(benches);
