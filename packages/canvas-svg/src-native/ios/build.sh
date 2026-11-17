#!/bin/sh
# Builds CanvasSVG.xcframework from the Rust static libs `make svg` produced, and copies it to
# packages/canvas-svg/platforms/ios/CanvasSVGBinary.
#
#   ./build.sh                 every platform's slices, then the xcframework
#   ./build.sh ios tvos        only those platforms' slices, into dist/
#   ./build.sh --xcframework   only the xcframework, from the slices already in dist/
#
# Platforms: ios, catalyst, visionos, tvos. CI builds each platform on its own runner and
# joins the slices with --xcframework.

set -e

DIST="$(pwd)/dist"
FRAMEWORK=CanvasSVG

# build_slice <sdk, or "" for Catalyst> <destination> [build settings...]
build_slice() {
  sdk="$1"
  destination="$2"
  shift 2
  echo "Build for $destination"
  xcodebuild \
    -project $FRAMEWORK.xcodeproj \
    -scheme $FRAMEWORK \
    ${sdk:+-sdk "$sdk"} \
    -destination "$destination" \
    -configuration Release \
    -quiet \
    clean build \
    BUILD_DIR="$DIST" \
    SKIP_INSTALL=NO \
    BUILD_LIBRARY_FOR_DISTRIBUTION=YES \
    "$@"
}

build_ios() {
  build_slice iphonesimulator "generic/platform=iOS Simulator"
  build_slice iphoneos "generic/platform=iOS" CODE_SIGN_IDENTITY="" CODE_SIGNING_REQUIRED=NO
}

# visionOS is Apple Silicon only: pin ARCHS=arm64 (no x86_64-apple-visionos-sim Rust target exists).
build_visionos() {
  build_slice xrsimulator "generic/platform=visionOS Simulator" ARCHS=arm64 ONLY_ACTIVE_ARCH=NO
  build_slice xros "generic/platform=visionOS" ARCHS=arm64 ONLY_ACTIVE_ARCH=NO \
    CODE_SIGN_IDENTITY="" CODE_SIGNING_REQUIRED=NO
}

build_tvos() {
  build_slice appletvsimulator "generic/platform=tvOS Simulator" ARCHS=arm64 ONLY_ACTIVE_ARCH=NO
  build_slice appletvos "generic/platform=tvOS" ARCHS=arm64 ONLY_ACTIVE_ARCH=NO \
    CODE_SIGN_IDENTITY="" CODE_SIGNING_REQUIRED=NO
}

# Xcode runs pre-build.sh once, for arm64: build x86_64-apple-ios-macabi first (make catalyst-svg).
build_catalyst() {
  build_slice "" "generic/platform=macOS,variant=Mac Catalyst" \
    ONLY_ACTIVE_ARCH=NO CODE_SIGN_IDENTITY="" CODE_SIGNING_REQUIRED=NO
}

create_xcframework() {
  set --
  for sdk in iphoneos iphonesimulator maccatalyst xros xrsimulator appletvos appletvsimulator; do
    fw="$DIST/Release-$sdk/$FRAMEWORK.framework"
    [ -d "$fw" ] || { echo "error: missing slice $fw" >&2; exit 1; }
    set -- "$@" -framework "$fw" -debug-symbols "$fw.dSYM"
  done
  echo "Creating XCFramework"
  rm -rf "$DIST/$FRAMEWORK.xcframework"
  xcodebuild -create-xcframework "$@" -output "$DIST/$FRAMEWORK.xcframework"

  DEST="$(pwd)/../../platforms/ios/CanvasSVGBinary"
  mkdir -p "$DEST"
  # Replace (don't merge into) any existing framework so stale slices can't linger.
  rm -rf "$DEST/$FRAMEWORK.xcframework"
  cp -R "$DIST/$FRAMEWORK.xcframework" "$DEST/"
  echo "Done. XCFramework published to: $DEST/$FRAMEWORK.xcframework"
}

if [ "$1" = "--xcframework" ]; then
  create_xcframework
  exit 0
fi

if [ $# -eq 0 ]; then
  rm -rf "$DIST"
  build_ios
  build_visionos
  build_tvos
  build_catalyst
  create_xcframework
  exit 0
fi

for platform in "$@"; do
  case $platform in
    ios | catalyst | visionos | tvos) "build_$platform" ;;
    *) echo "error: unknown platform '$platform' (ios, catalyst, visionos, tvos)" >&2; exit 1 ;;
  esac
done
