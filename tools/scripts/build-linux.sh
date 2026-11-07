#!/bin/bash
# Builds the Linux Node-API modules into each package's platforms/linux/lib/<arch>/:
# canvasnative.node (@nativescript/canvas), canvassvg.node (@nativescript/canvas-svg),
# canvasmedia.node (@nativescript/canvas-media, GStreamer), canvasgamepad.node
# (@nativescript/canvas-gamepad) and audiocontext.node (@nativescript/audio-context).
#
# Needs the GTK 4 (>= 4.6), EGL/GLES, fontconfig, freetype and ALSA development files, or
# NS_SYSROOT pointing at a sysroot made by NativeScript/linux's
# metadata-generator/linux/fetch-sysroot.sh (headers and .pc files only; add libasound2-dev,
# libgstreamer1.0-dev, libgstreamer-plugins-base1.0-dev and libudev-dev).
# Skia is built from source: ninja must be on PATH (or SKIA_NINJA_COMMAND set).
#
# Usage: tools/scripts/build-linux.sh [--debug] [crate...]
#   crates: canvas-napi canvas-svg-napi canvas-media-napi canvas-gamepad-napi audio-context-napi
#           (default: all)
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

PROFILE=release-napi
if [ "${1:-}" = "--debug" ]; then
    PROFILE=dev
    shift
fi
CRATES=("$@")
[ ${#CRATES[@]} -eq 0 ] && CRATES=(canvas-napi canvas-svg-napi canvas-media-napi canvas-gamepad-napi audio-context-napi)

case "$(uname -m)" in
    x86_64) ARCH=x64; TRIPLE=x86_64-linux-gnu ;;
    aarch64) ARCH=arm64; TRIPLE=aarch64-linux-gnu ;;
    *) echo "unsupported architecture $(uname -m)" >&2; exit 1 ;;
esac

if ! pkg-config --exists 'gtk4 >= 4.6' egl glesv2 alsa fontconfig freetype2 2>/dev/null; then
    SYSROOT="${NS_SYSROOT:-/tmp/ns-sysroot}"
    for pc in gtk4 egl glesv2 alsa fontconfig freetype2; do
        [ -f "$SYSROOT/usr/lib/$TRIPLE/pkgconfig/$pc.pc" ] || {
            echo "$pc development files not found: install them or set NS_SYSROOT" >&2
            exit 1
        }
    done
    host_pc=$(pkg-config --variable pc_path pkg-config)
    export PKG_CONFIG_SYSROOT_DIR="$SYSROOT"
    export PKG_CONFIG_LIBDIR="$(echo "$host_pc" | tr ':' '\n' | sed "s|^|$SYSROOT|" | paste -sd:):$host_pc"
    # A sysroot's lib*.so links point at runtime libraries it doesn't have: link the host's.
    LINK="$ROOT/target/linux-link"
    mkdir -p "$LINK"
    for dev in "$SYSROOT/usr/lib/$TRIPLE"/lib*.so; do
        soname=$(readlink "$dev" || true)
        [ -n "$soname" ] && [ -e "/usr/lib/$TRIPLE/$soname" ] && ln -sf "/usr/lib/$TRIPLE/$soname" "$LINK/$(basename "$dev")"
    done
    export RUSTFLAGS="${RUSTFLAGS:-} -L $LINK"
    # Skia's GN build includes fontconfig and system freetype headers from the default paths.
    export C_INCLUDE_PATH="$SYSROOT/usr/include/freetype2:$SYSROOT/usr/include${C_INCLUDE_PATH:+:$C_INCLUDE_PATH}"
    export CPLUS_INCLUDE_PATH="$SYSROOT/usr/include/freetype2:$SYSROOT/usr/include${CPLUS_INCLUDE_PATH:+:$CPLUS_INCLUDE_PATH}"
fi

# Skia needs clang >= 16 (C++20 parenthesized aggregate init): the system's, CANVAS_CLANG_DIR,
# or an Android NDK's (its clang builds for the host too).
clang_major() { "$1" --version 2>/dev/null | sed -n 's/.*clang version \([0-9]*\).*/\1/p' | head -1; }
CLANG_DIR="${CANVAS_CLANG_DIR:-}"
if [ -z "$CLANG_DIR" ] && [ "$(clang_major clang)" -ge 16 ] 2>/dev/null; then
    CLANG_DIR="$(dirname "$(command -v clang)")"
fi
if [ -z "$CLANG_DIR" ]; then
    for ndk in $(ls -d "${ANDROID_HOME:-$HOME/Android/Sdk}"/ndk/*/toolchains/llvm/prebuilt/linux-x86_64 2>/dev/null | sort -V -r); do
        if [ "$(clang_major "$ndk/bin/clang")" -ge 16 ] 2>/dev/null; then
            CLANG_DIR="$ndk/bin"
            break
        fi
    done
fi
[ -n "$CLANG_DIR" ] || { echo "clang >= 16 not found: set CANVAS_CLANG_DIR" >&2; exit 1; }

# Wrappers pin the target and, when clang would pick a GCC without C++ headers (a gcc-12
# runtime beside g++-11), a GCC that has them.
CC_DIR="$ROOT/target/linux-cc"
mkdir -p "$CC_DIR/bin"
CC_FLAGS="--target=$TRIPLE"
if ! echo '#include <cstddef>' | "$CLANG_DIR/clang++" $CC_FLAGS -x c++ -fsyntax-only - 2>/dev/null; then
    GCC_VERSION=$(ls /usr/include/c++ | sort -V | tail -1)
    mkdir -p "$CC_DIR/lib/gcc/$TRIPLE" "$CC_DIR/include/c++" "$CC_DIR/include/$TRIPLE/c++"
    ln -sfn "/usr/lib/gcc/$TRIPLE/$GCC_VERSION" "$CC_DIR/lib/gcc/$TRIPLE/$GCC_VERSION"
    ln -sfn "/usr/include/c++/$GCC_VERSION" "$CC_DIR/include/c++/$GCC_VERSION"
    ln -sfn "/usr/include/$TRIPLE/c++/$GCC_VERSION" "$CC_DIR/include/$TRIPLE/c++/$GCC_VERSION"
    CC_FLAGS="$CC_FLAGS --gcc-toolchain=$CC_DIR"
fi
for cc in clang clang++; do
    printf '#!/bin/sh\nexec %s %s "$@"\n' "$CLANG_DIR/$cc" "$CC_FLAGS" > "$CC_DIR/bin/$cc"
    chmod +x "$CC_DIR/bin/$cc"
done
export CLANGCC="$CC_DIR/bin/clang" CLANGCXX="$CC_DIR/bin/clang++"
export BINDGEN_EXTRA_CLANG_ARGS="${BINDGEN_EXTRA_CLANG_ARGS:-} $CC_FLAGS"
[ -f "$CLANG_DIR/../lib/libclang.so" ] && export LIBCLANG_PATH="$CLANG_DIR/../lib"

if [ -z "${SKIA_NINJA_COMMAND:-}" ] && ! command -v ninja >/dev/null; then
    echo "ninja not found: put it on PATH or set SKIA_NINJA_COMMAND (pip install ninja works)" >&2
    exit 1
fi

OUT_PROFILE=$PROFILE
[ "$PROFILE" = dev ] && OUT_PROFILE=debug

for CRATE in "${CRATES[@]}"; do
    case "$CRATE" in
        canvas-napi) PACKAGE=packages/canvas; NAME=canvasnative ;;
        canvas-svg-napi) PACKAGE=packages/canvas-svg; NAME=canvassvg ;;
        canvas-media-napi) PACKAGE=packages/canvas-media; NAME=canvasmedia ;;
        canvas-gamepad-napi) PACKAGE=packages/canvas-gamepad; NAME=canvasgamepad ;;
        audio-context-napi) PACKAGE=packages/audio-context; NAME=audiocontext ;;
        *) echo "unsupported crate: $CRATE" >&2; exit 1 ;;
    esac
    cargo build -p "$CRATE" --profile "$PROFILE"
    DEST="$PACKAGE/platforms/linux/lib/$ARCH"
    mkdir -p "$DEST"
    install -m 755 "target/$OUT_PROFILE/lib${CRATE//-/_}.so" "$DEST/$NAME.node"
    # Line tables stay in target/ for symbolication; the shipped module keeps only symbols.
    [ "$PROFILE" = dev ] || strip --strip-debug "$DEST/$NAME.node"
    echo "built $DEST/$NAME.node ($PROFILE)"
done
