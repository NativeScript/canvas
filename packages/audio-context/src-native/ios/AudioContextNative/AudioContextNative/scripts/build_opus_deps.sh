#!/usr/bin/env bash
set -euo pipefail

# Build script wrapper for AudioContextNative third_party libs.
# This script will fetch sources (if necessary) and attempt to build
# libogg, opus and opusfile into third_party/build.

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TP_DIR="$ROOT_DIR/third_party"

echo "Fetching sources (if missing)..."
"$ROOT_DIR/scripts/fetch_opus_deps.sh"

echo "Starting build in: $TP_DIR"
mkdir -p "$TP_DIR/build"
cd "$TP_DIR"

# Determine parallelism
JOBS=$(getconf _NPROCESSORS_ONLN 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 1)

# Delegate to Makefile (simple sequential builder)
if [ -f Makefile ]; then
  echo "Invoking Makefile (jobs=$JOBS)..."
  make -j${JOBS} all
  echo "Makefile build finished."
else
  echo "Makefile not found in $TP_DIR. Please create third_party/Makefile or implement custom build steps."
  exit 1
fi

if [ "${1-}" = "--ios" ] || [ "${IOS-}" = "1" ]; then
  echo "Delegating to iOS cross-build helper..."
  "$ROOT_DIR/scripts/build_opus_deps_ios.sh"
  exit $?
fi

# After build, libraries should be in $TP_DIR/build
echo "Third-party build completed. Artifacts are under: $TP_DIR/build"
