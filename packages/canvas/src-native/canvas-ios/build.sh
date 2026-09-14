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


# tvOS needs its *platform* installed, not just the SDK -- a generic tvOS
# destination fails without it (Xcode > Settings > Components, or
# `xcodebuild -downloadPlatform tvOS`). Dropping -destination is not a
# workaround: xcodebuild then silently resolves to the first matching
# destination, which is iOS, and quietly produces no tvOS slice at all.
# Skip the tvOS slices when the platform is absent so the rest of the
# xcframework still builds, and say so.
BUILD_TVOS=true
# A tvOS destination is still *listed* when the platform is missing -- it just
# carries an "error:... is not installed" note -- so match a line that offers
# tvOS without one.
if ! xcodebuild -showdestinations -project CanvasNative.xcodeproj -scheme CanvasNative 2>/dev/null \
     | grep "platform:tvOS" | grep -qv "error:"; then
  BUILD_TVOS=false
  echo ""
  echo "warning: tvOS platform not installed -- skipping the tvOS slices."
  echo "         Install it with: xcodebuild -downloadPlatform tvOS"
  echo ""
fi

if $BUILD_TVOS; then
echo "Build for appletvsimulator (tvOS Simulator)"
xcodebuild \
    -project CanvasNative.xcodeproj \
    -scheme CanvasNative \
    -sdk appletvsimulator \
    -destination "generic/platform=tvOS Simulator" \
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
    -project CanvasNative.xcodeproj \
    -scheme CanvasNative \
    -sdk appletvos \
    -destination "generic/platform=tvOS" \
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

fi

TVOS_ARGS=""
if $BUILD_TVOS; then
  TVOS_ARGS="-framework $(PWD)/dist/Release-appletvos/CanvasNative.framework \
    -debug-symbols $(PWD)/dist/Release-appletvos/CanvasNative.framework.dSYM \
    -framework $(PWD)/dist/Release-appletvsimulator/CanvasNative.framework \
    -debug-symbols $(PWD)/dist/Release-appletvsimulator/CanvasNative.framework.dSYM"
fi

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
    $TVOS_ARGS \
    -output $(PWD)/dist/CanvasNative.xcframework