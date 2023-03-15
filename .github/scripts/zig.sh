#!/bin/bash
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/../.."

URL="$(build-utils/zig-download-url)"
TARGET_FILE="$(basename "$URL")"

printf "Downloading %s from %s\n" "$TARGET_FILE" "$URL"
curl -sSL "$URL" -o "$TARGET_FILE"

maybe_sudo() {
  if [[ "${USE_SUDO:-}" == true ]]; then
    sudo "$@"
  else
    "$@"
  fi
}

extract() {
  if [[ "$TARGET_FILE" == *.zip ]]; then
    EXTRACT_DIR="$(mktemp -d)"
    unzip -o -d "$EXTRACT_DIR" "$TARGET_FILE"
    maybe_sudo mkdir -p "$(dirname "$INSTALL_PATH")"
    maybe_sudo mv "$EXTRACT_DIR/$(basename "$TARGET_FILE" .zip)" "$INSTALL_PATH"
  else
    maybe_sudo mkdir -p "$INSTALL_PATH"
    maybe_sudo tar -C "$INSTALL_PATH" --strip-components=1 -xf "$TARGET_FILE"
  fi
}

case "$(uname -s)" in
"Win"* | "MINGW"*)
  INSTALL_PATH="C:/zig"
  extract
  ;;
*)
  INSTALL_PATH="/usr/local/zig"
  USE_SUDO=true extract
  ;;
esac

rm "$TARGET_FILE"

"$INSTALL_PATH/zig" version

# Add to PATH.
printf "%s\n" "$INSTALL_PATH" >>"$GITHUB_PATH"
