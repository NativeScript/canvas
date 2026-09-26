#!/bin/bash
# Builds a Node-API native module (crates/canvas-napi by default) for one target triple and
# copies it into the package's platforms/<platform>/<arch>/ as <name>.node.
#
# Usage: build-napi.sh TARGET [PROFILE]
#   TARGET   x86_64-pc-windows-msvc | aarch64-pc-windows-msvc
#   PROFILE  release-napi (default; panic=unwind so the host app survives a panic) or dev

set -e

TARGET="$1"
PROFILE=${2:-release-napi}

if [ "$TARGET" = "" ]; then
    echo "missing argument TARGET"
    echo "Usage: $0 TARGET [PROFILE]"
    exit 1
fi

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CRATE=canvas-napi
PACKAGE="$ROOT/packages/canvas"
NAME=canvasnative

case "$TARGET" in
  x86_64-pc-windows-msvc)  PLATFORM=windows; ARCH=x64;   LIB=canvas_napi.dll ;;
  aarch64-pc-windows-msvc) PLATFORM=windows; ARCH=arm64; LIB=canvas_napi.dll ;;
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
if [ "$PLATFORM" = "windows" ]; then
    ANGLE="$ROOT/.angle-prebuilt/angle-$ARCH/bin"
    [ -f "$ANGLE/libEGL.dll" ] || "$ROOT/tools/scripts/download-angle.sh" "$ARCH"
    cp "$ANGLE/libEGL.dll" "$ANGLE/libGLESv2.dll" "$DEST/"
    echo "$DEST/libEGL.dll, libGLESv2.dll"
fi
