#!/bin/bash
set -e

LANG_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd)
CARGO_TOML=$(cat "${LANG_DIR}/../../Cargo.toml")
PKG_NAME=$(echo "${CARGO_TOML}" | grep -m 1 -Po '(?<=^name = ")[^"]*(?=".*)')
FALLBACK_PO="${LANG_DIR}/en/${PKG_NAME}.po"

find "${LANG_DIR}" -maxdepth 2 -name \*.po -execdir mkdir -p LC_MESSAGES \;
find "${LANG_DIR}" -maxdepth 2 -name \*.po -execdir msgmerge --quiet --previous "${FALLBACK_PO}" "${PKG_NAME}.po" -o "LC_MESSAGES/${PKG_NAME}.po" \;
find "${LANG_DIR}" -maxdepth 2 -name \*.po -execdir msgfmt "${PKG_NAME}.po" -o "LC_MESSAGES/${PKG_NAME}.mo" \;
