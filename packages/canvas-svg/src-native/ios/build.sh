#!/bin/sh

echo "Set exit on simple errors"
set -e

rm -rf $(PWD)/dist

echo "Build for iphonesimulator"
xcodebuild \
    -project CanvasSVG.xcodeproj \
    -scheme CanvasSVG \
    -sdk iphonesimulator \
    -destination "generic/platform=iOS Simulator" \
    -configuration Release \
    clean build \
    BUILD_DIR=$(PWD)/dist \
    SKIP_INSTALL=NO \
    BUILD_LIBRARY_FOR_DISTRIBUTION=YES

echo "Build for iphoneos"
xcodebuild \
    -project CanvasSVG.xcodeproj \
    -scheme CanvasSVG \
    -sdk iphoneos \
    -destination "generic/platform=iOS" \
    -configuration Release \
    clean build \
    BUILD_DIR=$(PWD)/dist \
    CODE_SIGN_IDENTITY="" \
    CODE_SIGNING_REQUIRED=NO \
    SKIP_INSTALL=NO \
    BUILD_LIBRARY_FOR_DISTRIBUTION=YES

echo "Build for xrsimulator (visionOS Simulator)"
# visionOS is Apple-Silicon only — pin ARCHS=arm64 (no x86_64-apple-visionos-sim Rust target exists).
xcodebuild \
    -project CanvasSVG.xcodeproj \
    -scheme CanvasSVG \
    -sdk xrsimulator \
    -destination "generic/platform=visionOS Simulator" \
    -configuration Release \
    clean build \
    BUILD_DIR=$(PWD)/dist \
    ARCHS=arm64 \
    ONLY_ACTIVE_ARCH=NO \
    SKIP_INSTALL=NO \
    BUILD_LIBRARY_FOR_DISTRIBUTION=YES

echo "Build for xros (visionOS)"
xcodebuild \
    -project CanvasSVG.xcodeproj \
    -scheme CanvasSVG \
    -sdk xros \
    -destination "generic/platform=visionOS" \
    -configuration Release \
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
    -framework $(PWD)/dist/Release-iphoneos/CanvasSVG.framework \
    -debug-symbols $(PWD)/dist/Release-iphoneos/CanvasSVG.framework.dSYM \
    -framework $(PWD)/dist/Release-iphonesimulator/CanvasSVG.framework \
    -debug-symbols $(PWD)/dist/Release-iphonesimulator/CanvasSVG.framework.dSYM \
    -framework $(PWD)/dist/Release-xros/CanvasSVG.framework \
    -debug-symbols $(PWD)/dist/Release-xros/CanvasSVG.framework.dSYM \
    -framework $(PWD)/dist/Release-xrsimulator/CanvasSVG.framework \
    -debug-symbols $(PWD)/dist/Release-xrsimulator/CanvasSVG.framework.dSYM \
    -output $(PWD)/dist/CanvasSVG.xcframework

echo "Publishing XCFramework to packages/canvas-svg/platforms/ios"
DEST="$(PWD)/../../platforms/ios"
mkdir -p "$DEST"
# Replace (don't merge into) any existing framework so stale slices can't linger.
rm -rf "$DEST/CanvasSVG.xcframework"
cp -R "$(PWD)/dist/CanvasSVG.xcframework" "$DEST/"
echo "Done. XCFramework published to: $DEST/CanvasSVG.xcframework"
