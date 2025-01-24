#!/bin/sh

cd "$(dirname "$(readlink -f "$0")")/.." || exit

VERSION=$(jq -r '.version' "editors/code/package.json")
REV=$(head -c 7 .git/refs/heads/main)

sd "const VERSION: &str = \"(.*)\"" \
   "const VERSION: &str = \"$VERSION\"" \
    crates/wgsl_analyzer/src/bin/main.rs

sd "const VERSION = \"(.*)\"" \
   "const VERSION = \"$VERSION\"" \
    editors/code/src/main.ts

sd "const REV = \"(.*)\"" \
   "const REV = \"$REV\"" \
    editors/code/src/main.ts
