#!/bin/bash


TARGET="$1"

MODE=${2:-release}

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
NDK_VERSION="27.3.13750724"

# needed so we can overwrite it in the CI
if [ -z "$NDK" ]; then
  NDK="$ANDROID_HOME/ndk/$NDK_VERSION"
fi

# needed so we can overwrite it in the CI ... defaults to mac
if [ -z "$NDK_HOST" ]; then
  NDK_HOST="darwin-x86_64"
fi


TOOLS="$NDK/toolchains/llvm/prebuilt/$NDK_HOST"

RUSTFLAGS="-C link-arg=-Wl,--hash-style=sysv -C link-arg=-Wl,-z,max-page-size=16384"



if [ "$TARGET" = "aarch64-linux-android" ]; then
    RUSTFLAGS="$RUSTFLAGS -C target-feature=-outline-atomics"
fi

if [ "$MODE" = "release" ]; then
    RUSTFLAGS="$RUSTFLAGS -Zlocation-detail=none -Zunstable-options -Cpanic=immediate-abort"
    EXTRA_ARGS="-Z build-std=std,panic_abort --release"
else
    EXTRA_ARGS=""
fi

AR=$TOOLS/bin/llvm-ar \
CXX=$TOOLS/bin/${NDK_TARGET}${API_VERSION}-clang++ \
RANLIB=$TOOLS/bin/llvm-ranlib \
CXXFLAGS="--target=$NDK_TARGET" \
RUSTFLAGS="$RUSTFLAGS" \
cargo +nightly build $EXTRA_ARGS --target $TARGET -p canvas-android

# RUSTFLAGS="$RUSTFLAGS" cargo +nightly  build --target $TARGET -p canvas-android

#pushd packages/canvas/src-native/canvas-android
#./gradlew :canvas:assembleRelease
#popd


