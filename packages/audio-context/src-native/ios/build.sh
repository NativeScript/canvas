#!/bin/sh

echo "Set exit on simple errors"
set -e

cd "$(dirname "$0")/AudioContextNative"

rm -rf $(PWD)/dist

echo "Build for iphonesimulator"
xcodebuild \
    -project AudioContextNative.xcodeproj \
    -scheme AudioContextNative \
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
    -project AudioContextNative.xcodeproj \
    -scheme AudioContextNative \
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
# visionOS is Apple-Silicon only — pin ARCHS=arm64 (no x86_64 visionOS simulator slice).
xcodebuild \
    -project AudioContextNative.xcodeproj \
    -scheme AudioContextNative \
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
    -project AudioContextNative.xcodeproj \
    -scheme AudioContextNative \
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
    -framework $(PWD)/dist/Release-iphoneos/AudioContextNative.framework \
    -debug-symbols $(PWD)/dist/Release-iphoneos/AudioContextNative.framework.dSYM \
    -framework $(PWD)/dist/Release-iphonesimulator/AudioContextNative.framework \
    -debug-symbols $(PWD)/dist/Release-iphonesimulator/AudioContextNative.framework.dSYM \
    -framework $(PWD)/dist/Release-xros/AudioContextNative.framework \
    -debug-symbols $(PWD)/dist/Release-xros/AudioContextNative.framework.dSYM \
    -framework $(PWD)/dist/Release-xrsimulator/AudioContextNative.framework \
    -debug-symbols $(PWD)/dist/Release-xrsimulator/AudioContextNative.framework.dSYM \
    -output $(PWD)/dist/AudioContextNative.xcframework

echo "Publishing XCFramework to packages/audio-context/platforms/ios"
DEST="$(PWD)/../../../platforms/ios"
mkdir -p "$DEST"
# Replace (don't merge into) any existing framework so stale slices can't linger.
rm -rf "$DEST/AudioContextNative.xcframework"
cp -R "$(PWD)/dist/AudioContextNative.xcframework" "$DEST/"
echo "Done. XCFramework published to: $DEST/AudioContextNative.xcframework"
