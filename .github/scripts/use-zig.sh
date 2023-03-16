#!/bin/bash
set -euo pipefail

setenv() {
  local KEY="$1"
  local VAL="$2"
  printf "%s=%s\n" "$KEY" "$VAL" >"$GITHUB_ENV"
}

case "$(uname -s)" in
"Linux")
  LINKER="ld.lld" # ELF
  ;;
"Darwin")
  LINKER="ld64.lld" # Mach-O
  ;;
*)
  # Windows
  LINKER="lld-link" # COFF
  ;;
esac

setenv CC "zig cc"
setenv CXX "zig c++"
setenv AR "zig ar"
setenv RANLIB "zig ranlib"
setenv LD "zig $LINKER"
