#!/bin/bash
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/../.."

URL="$(build-utils/zig-download-url)"
TARGET_FILE="zig.tar.xz"

printf "Downloading zig from %s\n" "$URL"
curl -sSL "$URL" -o "$TARGET_FILE"

INSTALL_PATH="/usr/local/zig"
sudo mkdir -p "$INSTALL_PATH"
sudo tar -C "$INSTALL_PATH" --strip-components=1 -xf "$TARGET_FILE"
rm "$TARGET_FILE"

"$INSTALL_PATH/zig" version

# Add to PATH.
printf "%s\n" "$INSTALL_PATH" >>"$GITHUB_PATH"
