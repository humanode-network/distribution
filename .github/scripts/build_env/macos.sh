#!/bin/bash
set -euo pipefail

brew install \
  coreutils

.github/scripts/zig.sh
.github/scripts/cargo-zigbuild.sh
.github/scripts/use-zig.sh
