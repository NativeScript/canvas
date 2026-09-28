#!/bin/bash
# Fetches the pinned ANGLE build (libEGL/libGLESv2 on Direct3D 11) that WebGL uses on Windows,
# into .angle-prebuilt/angle-<arch>/. build-napi.sh ships its DLLs next to canvasnative.node.
#
# Usage: download-angle.sh [x64|arm64]   (default: both)
#
# Builds: https://github.com/mmozeiko/build-angle (ANGLE commit in angle-<arch>/commit.txt).
# ANGLE is BSD-licensed; see packages/canvas/platforms/windows/THIRD_PARTY_NOTICES.txt.

set -e

ANGLE_RELEASE="2026-09-20"
declare -A ANGLE_SHA256=(
  [x64]="55a076049a34096f24e364aadef2bfb0276d99d0a7c8642cdbf94f19346b676d"
  [arm64]="f6bfb386cc438357c6a11bc35ca968de6b06c61577e080db39207e958ba8f853"
)

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DEST="$ROOT/.angle-prebuilt"
ARCHS=${1:-"x64 arm64"}

mkdir -p "$DEST"
for ARCH in $ARCHS; do
  EXPECTED=${ANGLE_SHA256[$ARCH]}
  if [ -z "$EXPECTED" ]; then
    echo "unsupported arch: $ARCH"
    exit 1
  fi
  ZIP="$DEST/angle-$ARCH-$ANGLE_RELEASE.zip"
  if [ ! -f "$ZIP" ] || [ "$(sha256sum "$ZIP" | cut -d' ' -f1)" != "$EXPECTED" ]; then
    curl -fsSL -o "$ZIP" "https://github.com/mmozeiko/build-angle/releases/download/$ANGLE_RELEASE/angle-$ARCH-$ANGLE_RELEASE.zip"
  fi
  ACTUAL=$(sha256sum "$ZIP" | cut -d' ' -f1)
  if [ "$ACTUAL" != "$EXPECTED" ]; then
    echo "checksum mismatch for $ZIP: $ACTUAL"
    exit 1
  fi
  rm -rf "$DEST/angle-$ARCH"
  (cd "$DEST" && unzip -q "$ZIP")
  echo "$DEST/angle-$ARCH ($(cat "$DEST/angle-$ARCH/commit.txt"))"
done
