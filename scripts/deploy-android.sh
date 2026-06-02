#!/usr/bin/env bash
set -euo pipefail

echo "Deploy Android placeholder script"
if [ -d "bonsai-buddy-android" ]; then
  (cd bonsai-buddy-android && ./gradlew assembleRelease)
else
  echo "bonsai-buddy-android not found"
  exit 1
fi
