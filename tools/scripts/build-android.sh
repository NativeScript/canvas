#!/bin/bash


TARGET="$1"

if [ "$TARGET" = "" ]; then
    echo "missing argument TARGET"
    echo "Usage: $0 TARGET"
    exit 1
fi

NDK_TARGET=$TARGET

 if [ "$TARGET" = "arm-linux-androideabi" ]; then
     NDK_TARGET="armv7a-linux-androideabi"
 fi

API_VERSION="21"
NDK_VERSION="23.2.8568313"

# needed so we can overwrite it in the CI
if [ -z "$NDK" ]; then
  NDK="$ANDROID_HOME/ndk/$NDK_VERSION"
fi

# needed so we can overwrite it in the CI ... defaults to mac
if [ -z "$NDK_HOST" ]; then
  NDK_HOST="darwin-x86_64"
fi


TOOLS="$NDK/toolchains/llvm/prebuilt/$NDK_HOST"

AR=$TOOLS/bin/llvm-ar \
CXX=$TOOLS/bin/${NDK_TARGET}${API_VERSION}-clang++ \
RANLIB=$TOOLS/bin/llvm-ranlib \
CXXFLAGS="--target=$NDK_TARGET" \

RUSTFLAGS="-Zlocation-detail=none -C panic=abort"


if [ "$TARGET" = "aarch64-linux-android" ]; then
    RUSTFLAGS="-Zlocation-detail=none -C panic=abort -C target-feature=-outline-atomics -C target-cpu=native -DANDROID_SUPPORT_FLEXIBLE_PAGE_SIZES=ON"
fi

if [ "$TARGET" = "x86_64-linux-android" ]; then
    RUSTFLAGS="-Zlocation-detail=none -C panic=abort -DANDROID_SUPPORT_FLEXIBLE_PAGE_SIZES=ON"
fi


RUSTFLAGS="$RUSTFLAGS" cargo +nightly build -Z build-std='std,panic_abort' -Z build-std-features=panic_immediate_abort --target $TARGET $EXTRA_ARGS -p canvas-android --release

