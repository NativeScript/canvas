#!/bin/sh

echo "Set exit on simple errors"
set -e

rm -rf $(PWD)/dist

echo "Build for iphonesimulator"
xcodebuild \
    -project CanvasNative.xcodeproj \
    -scheme CanvasNative \
    -sdk iphonesimulator \
    -destination "generic/platform=iOS Simulator" \
    -configuration Release \
    -quiet \
    clean build \
    BUILD_DIR=$(PWD)/dist \
    SKIP_INSTALL=NO \
    BUILD_LIBRARY_FOR_DISTRIBUTION=YES 

echo "Build for iphoneos"
xcodebuild \
    -project CanvasNative.xcodeproj \
    -scheme CanvasNative \
    -sdk iphoneos \
    -destination "generic/platform=iOS" \
    -configuration Release \
    -quiet \
    clean build \
    BUILD_DIR=$(PWD)/dist \
    CODE_SIGN_IDENTITY="" \
    CODE_SIGNING_REQUIRED=NO \
    SKIP_INSTALL=NO \
    BUILD_LIBRARY_FOR_DISTRIBUTION=YES

echo "Build for xrsimulator (visionOS Simulator)"
# visionOS is Apple-Silicon only — the simulator is arm64 (there is no x86_64-apple-visionos-sim
# Rust target). Pin ARCHS=arm64 so Xcode doesn't try to build an x86_64 slice it has no Rust lib for.
xcodebuild \
    -project CanvasNative.xcodeproj \
    -scheme CanvasNative \
    -sdk xrsimulator \
    -destination "generic/platform=visionOS Simulator" \
    -configuration Release \
    -quiet \
    clean build \
    BUILD_DIR=$(PWD)/dist \
    ARCHS=arm64 \
    ONLY_ACTIVE_ARCH=NO \
    SKIP_INSTALL=NO \
    BUILD_LIBRARY_FOR_DISTRIBUTION=YES

echo "Build for xros (visionOS)"
xcodebuild \
    -project CanvasNative.xcodeproj \
    -scheme CanvasNative \
    -sdk xros \
    -destination "generic/platform=visionOS" \
    -configuration Release \
    -quiet \
    clean build \
    BUILD_DIR=$(PWD)/dist \
    ARCHS=arm64 \
    ONLY_ACTIVE_ARCH=NO \
    CODE_SIGN_IDENTITY="" \
    CODE_SIGNING_REQUIRED=NO \
    SKIP_INSTALL=NO \
    BUILD_LIBRARY_FOR_DISTRIBUTION=YES

echo "Creating XCFramework"
xcodebuild \
    -create-xcframework \
    -framework $(PWD)/dist/Release-iphoneos/CanvasNative.framework \
    -debug-symbols $(PWD)/dist/Release-iphoneos/CanvasNative.framework.dSYM \
    -framework $(PWD)/dist/Release-iphonesimulator/CanvasNative.framework \
    -debug-symbols $(PWD)/dist/Release-iphonesimulator/CanvasNative.framework.dSYM \
    -framework $(PWD)/dist/Release-xros/CanvasNative.framework \
    -debug-symbols $(PWD)/dist/Release-xros/CanvasNative.framework.dSYM \
    -framework $(PWD)/dist/Release-xrsimulator/CanvasNative.framework \
    -debug-symbols $(PWD)/dist/Release-xrsimulator/CanvasNative.framework.dSYM \
    -output $(PWD)/dist/CanvasNative.xcframework