#!/bin/sh
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
BUILD_FLAG=""
ANDROID_ARMEABI_V7A_OUTPUT="$ANDROID_ARMEABI_V7A_OUTPUT_DIR/debug/$OUTPUT_LIB_NAME"
ANDROID_x86_OUTPUT="$ANDROID_x86_OUTPUT_DIR/debug/$OUTPUT_LIB_NAME"
ANDROID_AARCH_64_OUTPUT="$ANDROID_AARCH_64_OUTPUT_DIR/debug/$OUTPUT_LIB_NAME"
ANDROID_x86_64_OUTPUT="$ANDROID_x86_64_OUTPUT_DIR/debug/$OUTPUT_LIB_NAME"
ANDROID_ARMEABI_V7A_NDK_DIR="/tmp/ndk_arm"
ANDROID_AARCH_64_NDK_DIR="/tmp/ndk_arm64"
ANDROID_x86_64_NDK_DIR="/tmp/ndk_x86_64"
ANDROID_x86_NDK_DIR="/tmp/ndk_x86"
LIBCPLUSPLUS_NAME="libc++_shared.so"
LIBCPLUSPLUS_SHARED_ARMEABI_V7A="$ANDROID_ARMEABI_V7A_NDK_DIR/sysroot/usr/lib/arm-linux-androideabi/$LIBCPLUSPLUS_NAME"
LIBCPLUSPLUS_SHARED_x86="$ANDROID_x86_NDK_DIR/sysroot/usr/lib/i686-linux-android/$LIBCPLUSPLUS_NAME"
LIBCPLUSPLUS_SHARED_x86_64="$ANDROID_x86_64_NDK_DIR/sysroot/usr/lib/x86_64-linux-android/$LIBCPLUSPLUS_NAME"
LIBCPLUSPLUS_SHARED_AARCH_64="$ANDROID_AARCH_64_NDK_DIR/sysroot/usr/lib/aarch64-linux-android/$LIBCPLUSPLUS_NAME"
CARGO_FLAGS="-Z features=itarget"
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
    export RUSTFLAGS='-C link-arg=-s'
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

if [[ ! -d "$ANDROID_ARMEABI_V7A_NDK_DIR" ]];then
"$ANDROID_NDK/build/tools/make_standalone_toolchain.py" --arch arm --install-dir "$ANDROID_ARMEABI_V7A_NDK_DIR"
fi

if [[ ! -d "$ANDROID_x86_NDK_DIR" ]];then
"$ANDROID_NDK/build/tools/make_standalone_toolchain.py" --arch x86 --install-dir "$ANDROID_x86_NDK_DIR"
fi

if [[ ! -d "$ANDROID_AARCH_64_NDK_DIR" ]];then
"$ANDROID_NDK/build/tools/make_standalone_toolchain.py" --arch arm64 --install-dir "$ANDROID_AARCH_64_NDK_DIR"
fi

if [[ ! -d "$ANDROID_x86_64_NDK_DIR" ]];then
"$ANDROID_NDK/build/tools/make_standalone_toolchain.py" --arch x86_64 --install-dir "$ANDROID_x86_64_NDK_DIR"
fi

export PATH=$PATH:/"$ANDROID_AARCH_64_NDK_DIR/bin":"$ANDROID_x86_64_NDK_DIR/bin":"$ANDROID_ARMEABI_V7A_NDK_DIR/bin":"$ANDROID_x86_NDK_DIR/bin"

cd "$NATIVE_SRC"

# Build arm
cargo build --target armv7-linux-androideabi -vv $BUILD_FLAG $CARGO_FLAGS

if [[ -f "$ANDROID_ARMEABI_V7A_DIR/$OUTPUT_LIB_NAME" ]];then
rm "$ANDROID_ARMEABI_V7A_DIR/$OUTPUT_LIB_NAME"
fi

if [[ -f "$ANDROID_ARMEABI_V7A_DIR/$LIBCPLUSPLUS_NAME" ]];then
rm "$ANDROID_ARMEABI_V7A_DIR/$LIBCPLUSPLUS_NAME"
fi

ln -s "$ANDROID_ARMEABI_V7A_OUTPUT" "$ANDROID_ARMEABI_V7A_DIR/$OUTPUT_LIB_NAME"
ln -s "$LIBCPLUSPLUS_SHARED_ARMEABI_V7A" "$ANDROID_ARMEABI_V7A_DIR/$LIBCPLUSPLUS_NAME"


# Build arm64
cargo build --target aarch64-linux-android -vv $BUILD_FLAG $CARGO_FLAGS

if [[ -f "$ANDROID_AARCH_64_DIR/$OUTPUT_LIB_NAME" ]];then
rm "$ANDROID_AARCH_64_DIR/$OUTPUT_LIB_NAME"
fi

if [[ -f "$ANDROID_AARCH_64_DIR/$LIBCPLUSPLUS_NAME" ]];then
rm "$ANDROID_AARCH_64_DIR/$LIBCPLUSPLUS_NAME"
fi

ln -s "$ANDROID_AARCH_64_OUTPUT" "$ANDROID_AARCH_64_DIR/$OUTPUT_LIB_NAME"
ln -s "$LIBCPLUSPLUS_SHARED_AARCH_64" "$ANDROID_AARCH_64_DIR/$LIBCPLUSPLUS_NAME"



# Build x86

cargo build --target i686-linux-android -vv $BUILD_FLAG $CARGO_FLAGS

if [[ -f "$ANDROID_x86_DIR/$OUTPUT_LIB_NAME" ]];then
rm "$ANDROID_x86_DIR/$OUTPUT_LIB_NAME"
fi

ln -s "$ANDROID_x86_OUTPUT" "$ANDROID_x86_DIR/$OUTPUT_LIB_NAME"

if [[ -f "$ANDROID_x86_DIR/$LIBCPLUSPLUS_NAME" ]];then
rm "$ANDROID_x86_DIR/$LIBCPLUSPLUS_NAME"
fi

ln -s "$LIBCPLUSPLUS_SHARED_x86" "$ANDROID_x86_DIR/$LIBCPLUSPLUS_NAME"


# Build x86_64

cargo build --target x86_64-linux-android -vv $BUILD_FLAG $CARGO_FLAGS

if [[ -f "$ANDROID_x86_64_DIR/$OUTPUT_LIB_NAME" ]];then
rm "$ANDROID_x86_64_DIR/$OUTPUT_LIB_NAME"
fi

ln -s "$ANDROID_x86_64_OUTPUT" "$ANDROID_x86_64_DIR/$OUTPUT_LIB_NAME"

if [[ -f "$ANDROID_x86_64_DIR/$LIBCPLUSPLUS_NAME" ]];then
rm "$ANDROID_x86_64_DIR/$LIBCPLUSPLUS_NAME"
fi

ln -s "$LIBCPLUSPLUS_SHARED_x86_64" "$ANDROID_x86_64_DIR/$LIBCPLUSPLUS_NAME"

