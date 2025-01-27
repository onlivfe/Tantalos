#!/bin/bash
set -e

LANG_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd)
CARGO_TOML=$(cat "${LANG_DIR}/../../Cargo.toml")
PKG_NAME=$(echo "${CARGO_TOML}" | grep -m 1 -Po '(?<=^name = ")[^"]*(?=".*)')
PKG_VERSION=$(echo "${CARGO_TOML}" | grep -m 1 -Po '(?<=^version = ")[^"]*(?=".*)')

cd "${LANG_DIR}/../ui"
find . -name \*.slint -print0 | xargs -0 slint-tr-extractor --package-name "${PKG_NAME}" --package-version "${PKG_VERSION}" -o "${LANG_DIR}/${PKG_NAME}.pot"
echo "Translations template update"
