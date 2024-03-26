#!/bin/sh

echo "Set exit on simple errors"
set -e

rm -rf $(PWD)/dist

echo "Build for iphonesimulator"
xcodebuild \
    -project CanvasSVG.xcodeproj \
    -scheme CanvasSVGDebug \
    -sdk iphonesimulator \
    -destination "generic/platform=iOS Simulator" \
    -configuration Debug \
    clean build \
    BUILD_DIR=$(PWD)/dist \
    SKIP_INSTALL=NO \
    BUILD_LIBRARY_FOR_DISTRIBUTION=YES

echo "Build for iphoneos"
xcodebuild \
    -project CanvasSVG.xcodeproj \
    -scheme CanvasSVGDebug \
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
    -framework $(PWD)/dist/Debug-iphoneos/CanvasSVG.framework \
    -framework $(PWD)/dist/Debug-iphonesimulator/CanvasSVG.framework \
    -output $(PWD)/dist/CanvasSVG.xcframework
