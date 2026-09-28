#!/bin/bash
set -e
cd "$(dirname "$0")/../.."
REPO_ROOT="$(pwd)"

# xcodebuild only links the Rust static libs; it does not build them. The
# in-project pre-build.sh phase builds a single target per run, so a Release
# simulator build (arm64 + x86_64) links a slice that phase never produced and
# fails with a bare "ld: library 'canvasnative' not found". Check up front and
# name the make target instead.
# build.sh only adds the tvOS slices when a *usable* tvOS destination exists
# (the platform is listed even when not installed, with an "error:" note), so
# requiring those two triples unconditionally would block a build that never
# links them. Apply the same condition here.
TVOS_ENTRIES="aarch64-apple-tvos:tvos aarch64-apple-tvos-sim:tvos"
if ! xcodebuild -showdestinations \
       -project packages/canvas/src-native/canvas-ios/CanvasNative.xcodeproj \
       -scheme CanvasNative 2>/dev/null \
     | grep "platform:tvOS" | grep -qv "error:"; then
  TVOS_ENTRIES=""
fi

missing=""
for entry in \
  $TVOS_ENTRIES \
  "x86_64-apple-ios:ios" \
  "aarch64-apple-ios:ios" \
  "aarch64-apple-ios-sim:ios" \
  "aarch64-apple-visionos:visionos" \
  "aarch64-apple-visionos-sim:visionos"; do
  triple="${entry%%:*}"
  target="${entry##*:}"
  if [ ! -f "$REPO_ROOT/target/$triple/release/libcanvasnative.a" ]; then
    missing="$missing\n  $triple   (run: make $target)"
  fi
done

if [ -n "$missing" ]; then
  echo "error: the Rust static libraries this xcframework links are missing:" >&2
  printf "%b\n" "$missing" >&2
  echo "" >&2
  echo "Build them first, as .github/workflows/build-native.yml does:" >&2
  echo "  make ios && make visionos && make tvos" >&2
  exit 1
fi

cd packages/canvas/src-native/canvas-ios


# Replace (don't merge into) the existing framework so stale slices can't linger.
rm -rf ../../platforms/ios/CanvasNative.xcframework || true
mkdir -p ../../platforms/ios

echo "Build iOS"
./build.sh
#cd ..
echo "Copy /dist/*.xcframework platforms/ios"

cp -R dist/CanvasNative.xcframework ../../platforms/ios


# DEV_TEAM=${DEVELOPMENT_TEAM:-}
# DIST=$(PWD)/dist
# mkdir -p $DIST

# mkdir -p $DIST/intermediates

# echo "Cleanup"
# xcodebuild -project CanvasNative.xcodeproj -target "CanvasNative" -configuration Release clean


# echo "Building for Mac Catalyst"
# xcodebuild archive -project CanvasNative.xcodeproj \
#                    -scheme "CanvasNative" \
#                    -configuration Release \
#                    -destination "platform=macOS,variant=Mac Catalyst" \
#                    -quiet \
#                    SKIP_INSTALL=NO \
#                    -archivePath $DIST/intermediates/CanvasNative.maccatalyst.xcarchive



# echo "Building for iphone simulator"
# xcodebuild archive -project CanvasNative.xcodeproj \
#                    -scheme "CanvasNative" \
#                    -configuration Release \
#                    -sdk iphonesimulator IPHONEOS_DEPLOYMENT_TARGET=11.0 \
#                    -quiet \
#                    -arch x86_64 \
#                    DEVELOPMENT_TEAM=$DEV_TEAM \
#                    SKIP_INSTALL=NO \
#                    BUILD_LIBRARY_FOR_DISTRIBUTION=YES \
#                    ONLY_ACTIVE_ARCH=NO \
#                    -archivePath $DIST/intermediates/CanvasNative.iphonesimulator.xcarchive 



# echo "Building for iphone simulator m1"
# xcodebuild archive -project CanvasNative.xcodeproj \
#                    -scheme "CanvasNative" \
#                    -configuration Release \
#                    -sdk iphonesimulator IPHONEOS_DEPLOYMENT_TARGET=11.0 \
#                    -quiet \
#                    -arch x86_64 \
#                    DEVELOPMENT_TEAM=$DEV_TEAM \
#                    SKIP_INSTALL=NO \
#                    BUILD_LIBRARY_FOR_DISTRIBUTION=YES \
#                    ONLY_ACTIVE_ARCH=NO \
#                    -archivePath $DIST/intermediates/CanvasNative.iphonesimulator.xcarchive 




# echo "Building for ARM64 device"
# xcodebuild archive -project CanvasNative.xcodeproj \
#                    -scheme "CanvasNative" \
#                    -configuration Release \
#                    -arch arm64 \
#                    -sdk iphoneos IPHONEOS_DEPLOYMENT_TARGET=11.0 \
#                    -quiet \
#                    DEVELOPMENT_TEAM=$DEV_TEAM \
#                    SKIP_INSTALL=NO \
#                    BUILD_LIBRARY_FOR_DISTRIBUTION=YES \
#                    ONLY_ACTIVE_ARCH=NO \
#                    -archivePath $DIST/intermediates/CanvasNative.iphoneos.xcarchive


# echo "Creating CanvasNative.xcframework"
# OUTPUT_DIR="$DIST/CanvasNative.xcframework"
# rm -rf $OUTPUT_DIR

#    -framework "$DIST/intermediates/CanvasNative.maccatalyst.xcarchive/Products/Library/Frameworks/CanvasNative.framework" \
#    -debug-symbols "$DIST/intermediates/CanvasNative.maccatalyst.xcarchive/dSYMs/CanvasNative.framework.dSYM" \

# xcodebuild -create-xcframework \
#            -framework "$DIST/intermediates/CanvasNative.iphonesimulator.xcarchive/Products/Library/Frameworks/CanvasNative.framework" \
#            -debug-symbols "$DIST/intermediates/CanvasNative.iphonesimulator.xcarchive/dSYMs/CanvasNative.framework.dSYM" \
#            -framework "$DIST/intermediates/CanvasNative.iphoneos.xcarchive/Products/Library/Frameworks/CanvasNative.framework" \
#            -debug-symbols "$DIST/intermediates/CanvasNative.iphoneos.xcarchive/dSYMs/CanvasNative.framework.dSYM" \
#            -output "$OUTPUT_DIR"

# rm -rf "$DIST/intermediates"                   

# echo "Creating CanvasNative.xcframework"
# OUTPUT_DIR="$DIST/CanvasNative.xcframework"
# rm -rf $OUTPUT_DIR
# xcodebuild -create-xcframework \
#            -framework "$DIST/CanvasNative.iphonesimulator.xcarchive/Products/Library/Frameworks/CanvasNative.framework" \
#            -framework "$DIST/CanvasNative.iphoneos.xcarchive/Products/Library/Frameworks/CanvasNative.framework" \
#            -output "$OUTPUT_DIR"

# DSYM_OUTPUT_DIR="$DIST/CanvasNative.framework.dSYM"
# cp -r "$DIST/CanvasNative.iphoneos.xcarchive/dSYMs/CanvasNative.framework.dSYM/" $DSYM_OUTPUT_DIR
# lipo -create \
#     "$DIST/CanvasNative.iphonesimulator.xcarchive/dSYMs/CanvasNative.framework.dSYM/Contents/Resources/DWARF/CanvasNative" \
#     "$DIST/CanvasNative.iphoneos.xcarchive/dSYMs/CanvasNative.framework.dSYM/Contents/Resources/DWARF/CanvasNative" \
#     -output "$DSYM_OUTPUT_DIR/Contents/Resources/DWARF/CanvasNative"

# pushd $DIST
# zip -qr "CanvasNative.framework.dSYM.zip" "CanvasNative.framework.dSYM"
# rm -rf "CanvasNative.framework.dSYM"
# popd

# rm -rf "$DIST/CanvasNative.iphonesimulator.xcarchive"
# rm -rf "$DIST/CanvasNative.iphoneos.xcarch
