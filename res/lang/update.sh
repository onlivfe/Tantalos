#!/bin/bash
set -e

LANG_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd)
LANG_DIR=$(echo "$LANG_DIR" | xargs realpath --relative-to=.)
CARGO_TOML=$(cat "${LANG_DIR}/../../Cargo.toml")
PKG_NAME=$(echo "${CARGO_TOML}" | grep -m 1 -Po '(?<=^name = ")[^"]*(?=".*)')

find "${LANG_DIR}" -maxdepth 2 -name \*.po -execdir msgmerge --quiet --update --suffix="" "${PKG_NAME}.po" "${LANG_DIR}/${PKG_NAME}.pot" \;
echo "Translations merged from template"
