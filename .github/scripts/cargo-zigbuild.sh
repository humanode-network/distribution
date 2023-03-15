#!/bin/bash
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/../.."

URL="$(build-utils/cargo-zigbuild-download-url)"
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
  maybe_sudo mkdir -p "$INSTALL_PATH"
  if [[ "$TARGET_FILE" == *.zip ]]; then
    maybe_sudo unzip -o -d "$INSTALL_PATH" "$TARGET_FILE"
  else
    maybe_sudo tar -C "$INSTALL_PATH" -xvf "$TARGET_FILE"
  fi
}

case "$(uname -s)" in
"Win"* | "MINGW"*)
  INSTALL_PATH="$HOME/.cargo/bin"
  extract
  ;;
*)
  INSTALL_PATH="/usr/local/bin"
  USE_SUDO=true extract
  ;;
esac

rm "$TARGET_FILE"

"$INSTALL_PATH/cargo-zigbuild" --version
