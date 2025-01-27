#!/bin/bash
set -e

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd "${SCRIPT_DIR}"

slint-viewer ui/index.slint --auto-reload --style material --translation-domain tantalos --translation-dir lang
