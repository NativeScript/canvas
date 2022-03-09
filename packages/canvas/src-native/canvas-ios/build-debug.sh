#!/bin/sh

echo "Set exit on simple errors"
set -e

rm -rf $(PWD)/dist

echo "Build for iphonesimulator"
xcodebuild \
    -project CanvasNative.xcodeproj \
    -scheme CanvasNativeDebug \
    -sdk iphonesimulator \
    -destination "generic/platform=iOS Simulator" \
    -configuration Debug \
    clean build \
    BUILD_DIR=$(PWD)/dist \
    SKIP_INSTALL=NO \
    BUILD_LIBRARY_FOR_DISTRIBUTION=YES 

echo "Build for iphoneos"
xcodebuild \
    -project CanvasNative.xcodeproj \
    -scheme CanvasNativeDebug \
    -sdk iphoneos \
    -destination "generic/platform=iOS" \
    -configuration Debug \
    clean build \
    BUILD_DIR=$(PWD)/dist \
    CODE_SIGN_IDENTITY="" \
    CODE_SIGNING_REQUIRED=NO \
    SKIP_INSTALL=NO \
    BUILD_LIBRARY_FOR_DISTRIBUTION=YES 

echo "Creating XCFramework"
xcodebuild \
    -create-xcframework \
    -framework $(PWD)/dist/Debug-iphoneos/CanvasNative.framework \
    -framework $(PWD)/dist/Debug-iphonesimulator/CanvasNative.framework \
    -output $(PWD)/dist/CanvasNative.xcframework