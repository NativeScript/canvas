#!/bin/bash
# Builds a Node-API native module for one target triple and copies it into its package's
# platforms/<platform>/<arch>/ as <name>.node.
#
# Usage: build-napi.sh TARGET [PROFILE] [CRATE]
#   TARGET   x86_64-pc-windows-msvc | aarch64-pc-windows-msvc
#   PROFILE  release-napi (default; panic=unwind so the host app survives a panic) or dev
#   CRATE    canvas-napi (default: @nativescript/canvas, canvasnative.node) or canvas-svg-napi
#            (@nativescript/canvas-svg, canvassvg.node)

set -e

TARGET="$1"
PROFILE=${2:-release-napi}
CRATE=${3:-canvas-napi}

if [ "$TARGET" = "" ]; then
    echo "missing argument TARGET"
    echo "Usage: $0 TARGET [PROFILE] [CRATE]"
    exit 1
fi

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
case "$CRATE" in
  canvas-napi)     PACKAGE="$ROOT/packages/canvas";     NAME=canvasnative ;;
  canvas-svg-napi) PACKAGE="$ROOT/packages/canvas-svg"; NAME=canvassvg ;;
  *)
    echo "unsupported crate: $CRATE"
    exit 1
    ;;
esac
LIB="${CRATE//-/_}.dll"

case "$TARGET" in
  x86_64-pc-windows-msvc)  PLATFORM=windows; ARCH=x64 ;;
  aarch64-pc-windows-msvc) PLATFORM=windows; ARCH=arm64 ;;
  *)
    echo "unsupported target: $TARGET"
    exit 1
    ;;
esac

# Local toolchain for Skia (LLVM/libclang, ninja, a Skia source checkout), when there is one.
if [ -f "$ROOT/.tools/env.sh" ]; then
    source "$ROOT/.tools/env.sh"
fi

cd "$ROOT"
# Cargo names the dev profile's output directory "debug".
OUT_PROFILE=$PROFILE
[ "$PROFILE" = "dev" ] && OUT_PROFILE=debug

# For the host triple, build without --target: that reuses the plain `cargo build` artifacts
# (Skia included) instead of a second copy under target/<triple>/.
if [ "$(rustc -vV | sed -n 's/^host: //p')" = "$TARGET" ]; then
    cargo build -p "$CRATE" --profile "$PROFILE"
    OUT="target/$OUT_PROFILE"
else
    cargo build -p "$CRATE" --target "$TARGET" --profile "$PROFILE"
    OUT="target/$TARGET/$OUT_PROFILE"
fi

DEST="$PACKAGE/platforms/$PLATFORM/$ARCH"
mkdir -p "$DEST"
cp "$OUT/$LIB" "$DEST/$NAME.node"
echo "$DEST/$NAME.node"

# WebGL on Windows runs on ANGLE, loaded from next to the module (d3dcompiler_47 ships with
# Windows 10+).
if [ "$PLATFORM" = "windows" ] && [ "$CRATE" = "canvas-napi" ]; then
    ANGLE="$ROOT/.angle-prebuilt/angle-$ARCH/bin"
    [ -f "$ANGLE/libEGL.dll" ] || "$ROOT/tools/scripts/download-angle.sh" "$ARCH"
    cp "$ANGLE/libEGL.dll" "$ANGLE/libGLESv2.dll" "$DEST/"
    echo "$DEST/libEGL.dll, libGLESv2.dll"
fi
