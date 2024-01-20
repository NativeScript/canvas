#!/bin/bash
cd ../..
CWD="$(pwd)/packages/canvas/src-native"
NATIVE_SRC="$CWD/canvas-native"
ANDROID_SRC="$CWD/canvas-android"
ANDROID_JNI_LIB_DIR="$ANDROID_SRC/canvas/src/main/jniLibs"
ANDROID_ARMEABI_V7A_DIR="$ANDROID_JNI_LIB_DIR/armeabi-v7a"
ANDROID_AARCH_64_DIR="$ANDROID_JNI_LIB_DIR/arm64-v8a"
ANDROID_x86_DIR="$ANDROID_JNI_LIB_DIR/x86"
ANDROID_x86_64_DIR="$ANDROID_JNI_LIB_DIR/x86_64"
ANDROID_ARMEABI_V7A_OUTPUT_DIR="$NATIVE_SRC/target/armv7-linux-androideabi"
ANDROID_x86_OUTPUT_DIR="$NATIVE_SRC/target/i686-linux-android"
ANDROID_AARCH_64_OUTPUT_DIR="$NATIVE_SRC/target/aarch64-linux-android"
ANDROID_x86_64_OUTPUT_DIR="$NATIVE_SRC/target/x86_64-linux-android"
OUTPUT_LIB_NAME="libcanvasnative.so"
IS_RELEASE=false
BUILD_FLAG="-p canvas-android --features gl"
ANDROID_ARMEABI_V7A_OUTPUT="$ANDROID_ARMEABI_V7A_OUTPUT_DIR/debug/$OUTPUT_LIB_NAME"
ANDROID_x86_OUTPUT="$ANDROID_x86_OUTPUT_DIR/debug/$OUTPUT_LIB_NAME"
ANDROID_AARCH_64_OUTPUT="$ANDROID_AARCH_64_OUTPUT_DIR/debug/$OUTPUT_LIB_NAME"
ANDROID_x86_64_OUTPUT="$ANDROID_x86_64_OUTPUT_DIR/debug/$OUTPUT_LIB_NAME"
ANDROID_NDK_SYSROOT_LIB="$ANDROID_NDK/sysroot/usr/lib"
LIBCPLUSPLUS_NAME="libc++_shared.so"
#CARGO_FLAGS="-C target-cpu=native"
CARGO_FLAGS="-Zlocation-detail=none -C target-cpu=native -C panic=abort"
for arg in "$@"
do
    if [[ "$arg" == "--help" ]] || [[ "$arg" == "-h" ]]
    then
        echo "Help argument detected."
    elif [[ "$arg" == "--release" ]] ||  [[ "$arg" == "-r" ]]
    then
    IS_RELEASE=true
    BUILD_FLAG="--release"
    ANDROID_ARMEABI_V7A_OUTPUT="$ANDROID_ARMEABI_V7A_OUTPUT_DIR/release/$OUTPUT_LIB_NAME"
    ANDROID_x86_OUTPUT="$ANDROID_x86_OUTPUT_DIR/release/$OUTPUT_LIB_NAME"
    ANDROID_AARCH_64_OUTPUT="$ANDROID_AARCH_64_OUTPUT_DIR/release/$OUTPUT_LIB_NAME"
    ANDROID_x86_64_OUTPUT="$ANDROID_x86_64_OUTPUT_DIR/release/$OUTPUT_LIB_NAME"
    CARGO_FLAGS="$CARGO_FLAGS -C link-arg=-s"
    continue
    fi
done

if [[ -z "$ANDROID_NDK" ]]
then
    echo "Android NDK not found"
    exit
fi

if ! cargo --version >/dev/null 2>&1; then
    echo  "Cargo not found"
    exit
fi


NAME="$(uname -s)"
if [[ "$NAME" == "Darwin" ]];then 
PRE_BUILT_PATH="$ANDROID_NDK/toolchains/llvm/prebuilt/darwin-x86_64"
export PATH=$PATH:/"$PRE_BUILT_PATH/bin"
ANDROID_NDK_SYSROOT_LIB="$PRE_BUILT_PATH/sysroot/usr/lib"
elif [[ "$NAME" == "Linux" ]];then
PRE_BUILT_PATH="$ANDROID_NDK/toolchains/llvm/prebuilt/linux-x86_64"
export PATH=$PATH:/"$PRE_BUILT_PATH/bin"
ANDROID_NDK_SYSROOT_LIB="$PRE_BUILT_PATH/sysroot/usr/lib"
fi


LIBCPLUSPLUS_SHARED_ARMEABI_V7A="$ANDROID_NDK_SYSROOT_LIB/arm-linux-androideabi/$LIBCPLUSPLUS_NAME"
LIBCPLUSPLUS_SHARED_x86="$ANDROID_NDK_SYSROOT_LIB/i686-linux-android/$LIBCPLUSPLUS_NAME"
LIBCPLUSPLUS_SHARED_x86_64="$ANDROID_NDK_SYSROOT_LIB/x86_64-linux-android/$LIBCPLUSPLUS_NAME"
LIBCPLUSPLUS_SHARED_AARCH_64="$ANDROID_NDK_SYSROOT_LIB/aarch64-linux-android/$LIBCPLUSPLUS_NAME"

export RUSTFLAGS="$CARGO_FLAGS"

cd "$NATIVE_SRC"

# Build arm
cargo +nightly build -Z build-std='std,panic_abort'  -Z build-std-features='panic_immediate_abort' --target armv7-linux-androideabi -p canvas-android  $BUILD_FLAG

if [[ -f "$ANDROID_ARMEABI_V7A_DIR/$OUTPUT_LIB_NAME" ]];then
rm "$ANDROID_ARMEABI_V7A_DIR/$OUTPUT_LIB_NAME"
fi

if [[ -f "$ANDROID_ARMEABI_V7A_DIR/$LIBCPLUSPLUS_NAME" ]];then
rm "$ANDROID_ARMEABI_V7A_DIR/$LIBCPLUSPLUS_NAME"
fi

ln -s "$ANDROID_ARMEABI_V7A_OUTPUT" "$ANDROID_ARMEABI_V7A_DIR/$OUTPUT_LIB_NAME"
ln -s "$LIBCPLUSPLUS_SHARED_ARMEABI_V7A" "$ANDROID_ARMEABI_V7A_DIR/$LIBCPLUSPLUS_NAME"


# Build arm64
cargo +nightly build -Z build-std='std,panic_abort'  -Z build-std-features=panic_immediate_abort --target aarch64-linux-android -p canvas-android  $BUILD_FLAG

if [[ -f "$ANDROID_AARCH_64_DIR/$OUTPUT_LIB_NAME" ]];then
rm "$ANDROID_AARCH_64_DIR/$OUTPUT_LIB_NAME"
fi

if [[ -f "$ANDROID_AARCH_64_DIR/$LIBCPLUSPLUS_NAME" ]];then
rm "$ANDROID_AARCH_64_DIR/$LIBCPLUSPLUS_NAME"
fi

ln -s "$ANDROID_AARCH_64_OUTPUT" "$ANDROID_AARCH_64_DIR/$OUTPUT_LIB_NAME"
ln -s "$LIBCPLUSPLUS_SHARED_AARCH_64" "$ANDROID_AARCH_64_DIR/$LIBCPLUSPLUS_NAME"



# Build x86

cargo +nightly build -Z build-std='std,panic_abort'  -Z build-std-features=panic_immediate_abort --target i686-linux-android -p canvas-android  $BUILD_FLAG

if [[ -f "$ANDROID_x86_DIR/$OUTPUT_LIB_NAME" ]];then
rm "$ANDROID_x86_DIR/$OUTPUT_LIB_NAME"
fi

ln -s "$ANDROID_x86_OUTPUT" "$ANDROID_x86_DIR/$OUTPUT_LIB_NAME"

if [[ -f "$ANDROID_x86_DIR/$LIBCPLUSPLUS_NAME" ]];then
rm "$ANDROID_x86_DIR/$LIBCPLUSPLUS_NAME"
fi

ln -s "$LIBCPLUSPLUS_SHARED_x86" "$ANDROID_x86_DIR/$LIBCPLUSPLUS_NAME"


# Build x86_64

cargo +nightly build -Z build-std='std,panic_abort'  -Z build-std-features=panic_immediate_abort --target x86_64-linux-android -p canvas-android   $BUILD_FLAG

if [[ -f "$ANDROID_x86_64_DIR/$OUTPUT_LIB_NAME" ]];then
rm "$ANDROID_x86_64_DIR/$OUTPUT_LIB_NAME"
fi

ln -s "$ANDROID_x86_64_OUTPUT" "$ANDROID_x86_64_DIR/$OUTPUT_LIB_NAME"

if [[ -f "$ANDROID_x86_64_DIR/$LIBCPLUSPLUS_NAME" ]];then
rm "$ANDROID_x86_64_DIR/$LIBCPLUSPLUS_NAME"
fi

ln -s "$LIBCPLUSPLUS_SHARED_x86_64" "$ANDROID_x86_64_DIR/$LIBCPLUSPLUS_NAME"

