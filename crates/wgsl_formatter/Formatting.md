# Formatting
This document strives to be the one source of truth for wgsl formatting rules.
The examples of the rules are enforced via doctests.

## Code Spacing
### Empty lines at the start of a file
Wgslfmt removes empty lines at the start of a file
```
# use wgsl_formatter::test_util::check;
check("\n\n\nfn a() {}\n", "fn a() {}\n");
```

### One Newline at the end of a file
Wgslfmt enforces one newline at the end of a file
```
# use wgsl_formatter::test_util::check;
check("fn a() {}", "fn a() {}\n");
check("fn a() {}\n\n", "fn a() {}\n");
```

### Empty lines between module items
Wgsl allows 0-1 empty lines between module items.

TODO Example Doctests

### Newlines within structs
Wgslfmt removes newlines at the start of a struct body

TODO Example Doctests

Wgslfmt removes newlines at the end of a struct body

TODO Example Doctests

Wgslfmt allows between 0 and 1 newlines between struct fields

TODO Example Doctests

### Newlines within functions
Wgslfmt removes newlines at the start of a function body

TODO Example Doctests

Wgslfmt removes newlines at the end of a function body

TODO Example Doctests

Wgslfmt allows between 0 and 1 newlines between statements

TODO Example Doctests

### Newlines within statements
Wgslfmt removes newlines at the start of a function body

TODO Example Doctests

Wgslfmt removes newlines at the end of a function body

TODO Example Doctests

Wgslfmt allows between 0 and 1 newlines between statements

TODO Example Doctests
