# Benchmarks

This is a crate with a collection of benchmarks, separate from the rest of the crates.

## Running the benchmarks

1. Setup everything you need to develop in rust.
2. `cd` into the `benches` directory (where this README is located).

    ```sh
    template_crate_name $ cd benches
    ```

3. Run the benchmarks with cargo (This will take a while)

    ```sh
    template_crate_name/benches $ cargo bench
    ```

    If you'd like to only compile the benchmarks (without running them), you can do that like this:

    ```sh
    template_crate_name/benches $ cargo bench --no-run
    ```

## Criterion

The benchmarks use [Criterion](https://crates.io/crates/criterion). If you want to learn more about using Criterion for comparing performance against a baseline or generating detailed reports, you can read the [Criterion.rs documentation](https://bheisler.github.io/criterion.rs/book/criterion_rs.html).
