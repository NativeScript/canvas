#!/bin/sh

echo "Set exit on simple errors"
set -e

BUILD_DIR=$(PWD)/dist
rm -rf $BUILD_DIR

echo "Build for iphonesimulator"
xcodebuild \
    -project CanvasNative.xcodeproj \
    -scheme CanvasNative \
    -sdk iphonesimulator \
    -destination "generic/platform=iOS Simulator" \
    -configuration Release \
    clean build \
    BUILD_DIR=$BUILD_DIR \
    SKIP_INSTALL=NO \
    BUILD_LIBRARY_FOR_DISTRIBUTION=YES 

echo "Build for iphoneos"
xcodebuild \
    -project CanvasNative.xcodeproj \
    -scheme CanvasNative \
    -sdk iphoneos \
    -destination "generic/platform=iOS" \
    -configuration Release \
    clean build \
    BUILD_DIR=$BUILD_DIR \
    CODE_SIGN_IDENTITY="" \
    CODE_SIGNING_REQUIRED=NO \
    SKIP_INSTALL=NO \
    BUILD_LIBRARY_FOR_DISTRIBUTION=YES 

# Only available with Xcode >= 15.2
echo "Build for visionOS Simulator"
xcodebuild \
    -project CanvasNative.xcodeproj \
    -scheme CanvasNative \
    -configuration Release \
    -destination "generic/platform=visionOS Simulator" \
    clean build \
    BUILD_DIR=$BUILD_DIR \
    SKIP_INSTALL=NO \
    BUILD_LIBRARY_FOR_DISTRIBUTION=YES \
    -quiet

echo "Build for visionOS"
xcodebuild \
    -project CanvasNative.xcodeproj \
    -scheme CanvasNative \
    -configuration Release \
    -destination "generic/platform=visionOS" \
    clean build \
    BUILD_DIR=$BUILD_DIR \
    SKIP_INSTALL=NO \
    BUILD_LIBRARY_FOR_DISTRIBUTION=YES \
    -quiet

echo "Creating XCFramework"
BASE=$BUILD_DIR/Release
xcodebuild \
    -create-xcframework \
    -framework $BASE-iphoneos/CanvasNative.framework \
    -debug-symbols $BASE-iphoneos/CanvasNative.framework.dSYM \
    -framework $BASE-iphonesimulator/CanvasNative.framework \
    -debug-symbols $BASE-iphonesimulator/CanvasNative.framework.dSYM \
    -framework     $BASE-xrsimulator/CanvasNative.framework \
    -debug-symbols $BASE-xrsimulator/CanvasNative.framework.dSYM \
    -framework     $BASE-xros/CanvasNative.framework \
    -debug-symbols $BASE-xros/CanvasNative.framework.dSYM \
    -output $BASE/CanvasNative.xcframework