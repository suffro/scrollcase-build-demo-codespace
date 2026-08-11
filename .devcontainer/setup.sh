#!/usr/bin/env bash
set -euo pipefail

npm install --global scrollcase@0.9.1

if [[ ${CODESPACES:-} == true ]] && git remote get-url origin >/dev/null 2>&1; then
  git remote remove origin
fi
