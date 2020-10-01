#!/bin/sh
cd ../..
CWD="$(pwd)/packages/canvas/src-native"
NATIVE_SRC="$CWD/canvas-native"
IOS_SRC_DIR="$CWD/canvas-ios"
IOS_LIB_DIR="$IOS_SRC_DIR/CanvasNative"
IOS_LIB_INCLUDE="$IOS_LIB_DIR/include"
IOS_LIB_LIBS="$IOS_LIB_DIR/libs"
IOS_LIB_X86_64_SIM="$IOS_LIB_LIBS/x86_64-iphonesimulator"
IOS_LIB_ARM_64_PHONE="$IOS_LIB_LIBS/arm64-iphoneos"
OUTPUT_LIB_NAME="libcanvasnative.a"
IS_RELEASE=false
BUILD_FLAG=""
BITCODE_ENABLED=false
FEATURE_FLAGS="-Z features=itarget"
##CARGO_FLAGS="-C link-arg=-s -Z embed-bitcode features=itarget target-cpu=native"
CARGO_FLAGS="-C target-cpu=native"
##CARGO_FLAGS=""
IOS_X86_64_SIM_OUTPUT_DEBUG_DIR="$NATIVE_SRC/target/x86_64-apple-ios/debug/$OUTPUT_LIB_NAME"
IOS_X86_64_SIM_OUTPUT_RELEASE_DIR="$NATIVE_SRC/target/x86_64-apple-ios/release/$OUTPUT_LIB_NAME"

IOS_ARM_64_PHONE_OUTPUT_DEBUG_DIR="$NATIVE_SRC/target/aarch64-apple-ios/debug/$OUTPUT_LIB_NAME"
IOS_ARM_64_PHONE_OUTPUT_RELEASE_DIR="$NATIVE_SRC/target/aarch64-apple-ios/release/$OUTPUT_LIB_NAME"
if ! cargo --version >/dev/null 2>&1; then
  echo "Cargo not found"
  exit
fi

if ! cargo lipo --version >/dev/null 2>&1; then
  echo "Lipo not found"
  exit
fi

for arg in "$@"; do
  if [[ "$arg" == "--help" ]] || [[ "$arg" == "-h" ]]; then
    echo "Help argument detected."
  elif [[ "$arg" == "--release" ]] || [[ "$arg" == "-r" ]]; then
    IS_RELEASE=true
    BUILD_FLAG="--release"
  elif [[ "$arg" == "--bitcode" ]] || [[ "$arg" == "--bc" ]]; then
    BITCODE_ENABLED=true
    CARGO_FLAGS="$CARGO_FLAGS -Z embed-bitcode"
    continue
  fi
done

export RUSTFLAGS="$CARGO_FLAGS"

cd canvas-native

if [[ -f "$IOS_LIB_INCLUDE/canvas_native.h" ]]; then
  rm "$IOS_LIB_INCLUDE/canvas_native.h"
fi

# TODO fix header generation .... ignore android
cbindgen "$CWD/canvas-native/canvas-core/src/lib.rs" -l c >"$IOS_LIB_INCLUDE/canvas_native.h"


if [[ -f "$IOS_LIB_X86_64_SIM/$OUTPUT_LIB_NAME" ]]; then
  rm "$IOS_LIB_X86_64_SIM/$OUTPUT_LIB_NAME"
fi

if [[ $IS_RELEASE == true ]]; then
  cd "$NATIVE_SRC"
  cargo build --target x86_64-apple-ios $BUILD_FLAG $FEATURE_FLAGS
  cp "$IOS_X86_64_SIM_OUTPUT_RELEASE_DIR" "$IOS_LIB_X86_64_SIM/$OUTPUT_LIB_NAME"
else
  cd "$NATIVE_SRC"
  cargo build --target x86_64-apple-ios $FEATURE_FLAGS
  cp "$IOS_X86_64_SIM_OUTPUT_DEBUG_DIR" "$IOS_LIB_X86_64_SIM/$OUTPUT_LIB_NAME"
fi


if [[ -f "I$OS_LIB_ARM_64_PHONE/$OUTPUT_LIB_NAME" ]]; then
  rm "$IOS_LIB_ARM_64_PHONE/$OUTPUT_LIB_NAME"
fi

if [[ $IS_RELEASE == true ]]; then
  cd "$NATIVE_SRC"
  cargo build --target aarch64-apple-ios $BUILD_FLAG $FEATURE_FLAGS
  cp "$IOS_ARM_64_PHONE_OUTPUT_RELEASE_DIR" "$IOS_LIB_ARM_64_PHONE/$OUTPUT_LIB_NAME"
else
  cd "$NATIVE_SRC"
  cargo build --target aarch64-apple-ios $FEATURE_FLAGS
  cp "$IOS_ARM_64_PHONE_OUTPUT_DEBUG_DIR" "$IOS_LIB_ARM_64_PHONE/$OUTPUT_LIB_NAME"
fi


