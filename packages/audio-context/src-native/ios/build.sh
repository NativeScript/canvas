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
# No ARCHS/ONLY_ACTIVE_ARCH here: device SDKs are arm64-only already, so the
# override was redundant. The simulator steps do still need it, to keep an
# x86_64 slice out of a build whose third_party archives are arm64-only.
xcodebuild \
    -project AudioContextNative.xcodeproj \
    -scheme AudioContextNative \
    -sdk xros \
    -destination "generic/platform=visionOS" \
    -configuration Release \
    -quiet \
    clean build \
    BUILD_DIR=$(PWD)/dist \
    CODE_SIGN_IDENTITY="" \
    CODE_SIGNING_REQUIRED=NO \
    SKIP_INSTALL=NO \
    BUILD_LIBRARY_FOR_DISTRIBUTION=YES

echo "Build for appletvsimulator (tvOS Simulator)"
# tvOS slices are arm64-only, matching the third_party deps built by
# scripts/build_opus_deps_ios.sh (no x86_64 tvOS simulator archives exist).
# No -destination here, unlike the iOS/visionOS steps above: a generic tvOS
# destination requires the tvOS *platform* to be installed, not just its SDK,
# and .github/actions/setup-apple-native only downloads visionOS. -sdk alone
# builds the framework and keeps CI off a multi-GB platform download.
xcodebuild \
    -project AudioContextNative.xcodeproj \
    -scheme AudioContextNative \
    -sdk appletvsimulator \
    -configuration Release \
    -quiet \
    clean build \
    BUILD_DIR=$(PWD)/dist \
    ARCHS=arm64 \
    ONLY_ACTIVE_ARCH=NO \
    SKIP_INSTALL=NO \
    BUILD_LIBRARY_FOR_DISTRIBUTION=YES

echo "Build for appletvos (tvOS)"
xcodebuild \
    -project AudioContextNative.xcodeproj \
    -scheme AudioContextNative \
    -sdk appletvos \
    -configuration Release \
    -quiet \
    clean build \
    BUILD_DIR=$(PWD)/dist \
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
    -framework $(PWD)/dist/Release-appletvos/AudioContextNative.framework \
    -debug-symbols $(PWD)/dist/Release-appletvos/AudioContextNative.framework.dSYM \
    -framework $(PWD)/dist/Release-appletvsimulator/AudioContextNative.framework \
    -debug-symbols $(PWD)/dist/Release-appletvsimulator/AudioContextNative.framework.dSYM \
    -output $(PWD)/dist/AudioContextNative.xcframework

echo "Publishing XCFramework to packages/audio-context/platforms/ios"
DEST="$(PWD)/../../../platforms/ios"
mkdir -p "$DEST"
# Replace (don't merge into) any existing framework so stale slices can't linger.
rm -rf "$DEST/AudioContextNative.xcframework"
cp -R "$(PWD)/dist/AudioContextNative.xcframework" "$DEST/"
echo "Done. XCFramework published to: $DEST/AudioContextNative.xcframework"
