#!/bin/bash
set -euo pipefail

# Prevent issues with linking.
rm -f "C:\\Program Files\\Git\\usr\\bin\\link.exe"

.github/scripts/zig.sh
.github/scripts/cargo-zigbuild.sh
