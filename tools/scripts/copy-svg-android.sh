#! /bin/bash
COPY_DIR="packages/canvas-svg/src-native/android/canvassvg/src/main/jniLibs"
SRC_DIR="crates/canvas-svg"
NAME="canvassvg"
LIB_NAME="lib$NAME"
rm -rf $COPY_DIR

mkdir -p $COPY_DIR
mkdir -p $COPY_DIR/x86
mkdir -p $COPY_DIR/x86_64
mkdir -p $COPY_DIR/arm64-v8a
mkdir -p $COPY_DIR/armeabi-v7a

cp ./$SRC_DIR/target/i686-linux-android/release/$LIB_NAME.so $COPY_DIR/x86/$LIB_NAME.so
cp ./$SRC_DIR/target/x86_64-linux-android/release/$LIB_NAME.so $COPY_DIR/x86_64/$LIB_NAME.so
cp ./$SRC_DIR/target/aarch64-linux-android/release/$LIB_NAME.so $COPY_DIR/arm64-v8a/$LIB_NAME.so
cp ./$SRC_DIR/target/armv7-linux-androideabi/release/$LIB_NAME.so $COPY_DIR/armeabi-v7a/$LIB_NAME.so

echo "Dynamic libraries copied!"