#!/bin/bash
set -e

LANG_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd)
LANG_DIR=$(echo "$LANG_DIR" | xargs realpath --relative-to=.)

find "${LANG_DIR}" -maxdepth 3 -type f -name "*.po" | while read -r filename; do 
    missing_translations=$(msggrep --msgstr --invert-match -e "" "$filename")
    if [ -n "$missing_translations" ]; then
            echo "$missing_translations" | grep --label="${filename}" -HnP "(msgid \"(.+)\"|msgstr.* \"\")"  | grep -v ":2:msgstr "
            printf '\n'
    fi
done
