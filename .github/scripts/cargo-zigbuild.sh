#!/bin/bash
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/../.."

URL="$(build-utils/cargo-zigbuild-download-url)"
TARGET_FILE="$(basename "$URL")"

printf "Downloading %s from %s\n" "$TARGET_FILE" "$URL"
curl -sSL "$URL" -o "$TARGET_FILE"

INSTALL_PATH="/usr/local/bin"
sudo mkdir -p "$INSTALL_PATH"

if [[ "$TARGET_FILE" == *.zip ]]; then
  sudo unzip -o -d "$INSTALL_PATH" "$TARGET_FILE"
else
  sudo tar -C "$INSTALL_PATH" -xvf "$TARGET_FILE"
fi

rm "$TARGET_FILE"

"$INSTALL_PATH/cargo-zigbuild" --version
