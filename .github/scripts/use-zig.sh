#!/bin/bash
set -euo pipefail

setenv() {
  local KEY="$1"
  local VAL="$2"
  printf "%s=%s\n" "$KEY" "$VAL" >"$GITHUB_ENV"
}

setenv CC "zig cc"
setenv CXX "zig c++"
setenc AR "zig ar"
setenv RANLIB "zig ranlib"
