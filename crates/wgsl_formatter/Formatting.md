# Formatting
This document strives to be the one source of truth for wgsl formatting rules.
The examples of the rules are enforced via doctests.

## Code Spacing
### Newlines at the start of a file
Wgslfmt removes newlines at the start of a file
```
# use wgsl_formatter::test_util::check;
check("\n\n\nfn a() {}\n", "fn a() {}\n");
```

### One Newline at the end of a file
Wgslfmt removes newlines at the start of a file
```
# use wgsl_formatter::test_util::check;
check("fn a() {}", "fn a() {}\n");
check("fn a() {}\n\n", "fn a() {}\n");
```
