#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    wgsl_fuzz::parse_compare(data);

    // fuzzed code goes here
});
