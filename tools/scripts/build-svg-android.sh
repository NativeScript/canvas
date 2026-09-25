#!/bin/bash

TARGET="$1"

MODE=${2:-release}

if [ "$TARGET" = "" ]; then
    echo "missing argument TARGET"
    echo "Usage: $0 TARGET [debug|release]"
    exit 1
fi

# The NDK names the 32-bit ARM toolchain "armv7a-linux-androideabi"; Rust spells
# the same target "armv7-linux-androideabi" (and older setups "arm-linux-androideabi").
NDK_TARGET=$TARGET
case "$TARGET" in
  arm-linux-androideabi|armv7-linux-androideabi)
    NDK_TARGET="armv7a-linux-androideabi"
    ;;
esac

# Same as build-android.sh: Skia's ICU needs posix_madvise (API 23+), which NDK 29 dropped.
API_VERSION="23"
NDK_VERSION="28.2.13676358"

if [ -z "$ANDROID_HOME" ]; then
  ANDROID_HOME="${ANDROID_SDK_ROOT:-$HOME/Library/Android/sdk}"
fi

# needed so we can overwrite it in the CI
if [ -z "$NDK" ]; then
  NDK="$ANDROID_HOME/ndk/$NDK_VERSION"
fi

# needed so we can overwrite it in the CI ... defaults to mac
if [ -z "$NDK_HOST" ]; then
  NDK_HOST="darwin-x86_64"
fi

TOOLS="$NDK/toolchains/llvm/prebuilt/$NDK_HOST"

# --hash-style=sysv : older Android loaders
# --gc-sections     : drop unreferenced sections from Skia and the NDK
# -z,max-page-size  : Android 15+ 16KB page alignment
RUSTFLAGS="-C link-arg=-Wl,--hash-style=sysv"
RUSTFLAGS="$RUSTFLAGS -C link-arg=-Wl,--gc-sections"
RUSTFLAGS="$RUSTFLAGS -C link-arg=-Wl,-z,max-page-size=16384"

if [ "$TARGET" = "aarch64-linux-android" ]; then
    RUSTFLAGS="$RUSTFLAGS -C target-feature=-outline-atomics"
fi

if [ "$MODE" = "release" ]; then
    RUSTFLAGS="$RUSTFLAGS -Zlocation-detail=none -Zunstable-options -Cpanic=immediate-abort"
    EXTRA_ARGS="-Z build-std=std,panic_abort --release"
else
    EXTRA_ARGS=""
fi

CLANG="$TOOLS/bin/${NDK_TARGET}${API_VERSION}-clang"
CLANGXX="${CLANG}++"

for tool in "$CLANG" "$CLANGXX" "$TOOLS/bin/llvm-ar"; do
  if [ ! -x "$tool" ]; then
    echo "error: NDK tool not found: $tool" >&2
    echo "       NDK=$NDK NDK_HOST=$NDK_HOST TARGET=$TARGET (NDK_TARGET=$NDK_TARGET)" >&2
    exit 1
  fi
done

TRIPLE_ENV=$(echo "$TARGET" | tr '-' '_')
LINKER_ENV="CARGO_TARGET_$(echo "$TRIPLE_ENV" | tr '[:lower:]' '[:upper:]')_LINKER"

# skia-bindings reads ANDROID_NDK directly and panics without it; the linker has
# to be the NDK's clang or rustc links with Apple's ld, which rejects the flags
# above. No bare CC/CXX: those would also apply to host build scripts.
export ANDROID_NDK="$NDK"

RUSTFLAGS="$RUSTFLAGS" \
env "CC_${TRIPLE_ENV}=$CLANG" "CXX_${TRIPLE_ENV}=$CLANGXX" "AR_${TRIPLE_ENV}=$TOOLS/bin/llvm-ar" \
    "${LINKER_ENV}=$CLANG" \
cargo +nightly build $EXTRA_ARGS --target $TARGET -p canvas-svg-android

status=$?
if [ "$status" -ne 0 ]; then
  exit "$status"
fi

if [ "$MODE" = "release" ]; then
    LIB_PATH="target/$TARGET/release/libcanvassvg.so"
    if [ -f "$LIB_PATH" ]; then
        $TOOLS/bin/llvm-strip --strip-unneeded "$LIB_PATH"
    fi
fi
