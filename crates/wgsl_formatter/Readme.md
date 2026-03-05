# Debugging the tests

When compiled with the `prefer-immediate-crash` feature, the formatter will crash immediately when encountering a formatting error. This can be useful for debugging the tests, as a proper backtrace can be enabled with `RUST_BACKTRACE=1`.

```
RUST_BACKTRACE=1 cargo test --features=prefer-immediate-crash
```

# TODOs
- Format Ranges
- Polish cli
