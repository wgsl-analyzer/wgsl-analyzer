//! Various batch processing tasks, intended primarily for debugging.

// mod analysis_stats;
// mod diagnostics;
pub mod flags;
// mod highlight;
// mod lsif;
// mod parse;
// mod run_tests;
// mod rustc_tests;
// mod scip;
// mod ssr;
// mod symbols;
// mod unresolved_references;

// mod progress_report;

use std::io::Read as _;

use anyhow::Result;
use hir::Module;
use hir_def::module_data::Name;
use hir_ty::db::HirDatabase;
use itertools::Itertools;
use vfs::Vfs;

#[derive(Clone, Copy)]
pub enum Verbosity {
    Spammy,
    Verbose,
    Normal,
    Quiet,
}

impl Verbosity {
    #[must_use]
    #[inline]
    pub const fn is_verbose(self) -> bool {
        matches!(self, Self::Verbose | Self::Spammy)
    }

    #[must_use]
    #[inline]
    pub const fn is_spammy(self) -> bool {
        matches!(self, Self::Spammy)
    }
}

fn read_stdin() -> anyhow::Result<String> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

#[expect(clippy::print_stdout, reason = "CLI feature")]
fn report_metric(
    metric: &str,
    value: u64,
    unit: &str,
) {
    if std::env::var("WA_METRICS").is_err() {
        return;
    }
    println!("METRIC:{metric}:{value}:{unit}");
}

// fn print_memory_usage(mut host: AnalysisHost, vfs: Vfs) {
//     let mem = host.per_query_memory_usage();
//     let before = profile::memory_usage();
//     drop(vfs);
//     let vfs = before.allocated - profile::memory_usage().allocated;
//     let before = profile::memory_usage();
//     drop(host);
//     let unaccounted = before.allocated - profile::memory_usage().allocated;
//     let remaining = profile::memory_usage().allocated;
//     for (name, bytes, entries) in mem {
//         // NOTE: Not a debug print, so avoid going through the `eprintln` defined above.
//         eprintln!("{bytes:>8} {entries:>6} {name}");
//     }
//     eprintln!("{vfs:>8}        VFS");
//     eprintln!("{unaccounted:>8}        Unaccounted");
//     eprintln!("{remaining:>8}        Remaining");
// }
