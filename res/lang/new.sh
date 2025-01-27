#!/bin/bash
set -e

if [ "$#" -ne 1 ]; then
  echo "Usage: $0 <locale>"
  exit 1
fi

LANG_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd)
CARGO_TOML=$(cat "${LANG_DIR}/../../Cargo.toml")
PKG_NAME=$(echo "${CARGO_TOML}" | grep -m 1 -Po '(?<=^name = ")[^"]*(?=".*)')

mkdir -p "${LANG_DIR}/${1}"
msginit -i "${LANG_DIR}/${PKG_NAME}.pot" -l "${1}" -o "${LANG_DIR}/${1}/${PKG_NAME}.po" --no-translator
