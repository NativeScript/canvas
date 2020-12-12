#!/bin/bash
cd ../../packages/canvas/src-native/canvas-ios
set -e

DEV_TEAM=${DEVELOPMENT_TEAM:-}
DIST="CanvasNative/Dist"
mkdir -p $DIST

echo "Cleanup"
xcodebuild -project CanvasNative.xcodeproj -target "CanvasNative" -configuration Debug clean


echo "Building for iphone simulator"
xcodebuild archive -project CanvasNative.xcodeproj \
                   -scheme "CanvasNative" \
                   -configuration Debug \
                   -arch x86_64 \
                   -sdk iphonesimulator \
                   -quiet \
                   DEVELOPMENT_TEAM=$DEV_TEAM \
                   SKIP_INSTALL=NO \
                   BUILD_LIBRARY_FOR_DISTRIBUTION=YES \
                   -archivePath $DIST/CanvasNative.iphonesimulator.xcarchive

echo "Building for ARM64 device"
xcodebuild archive -project CanvasNative.xcodeproj \
                   -scheme "CanvasNative" \
                   -configuration Debug \
                   -arch arm64 \
                   -sdk iphoneos \
                   -quiet \
                   DEVELOPMENT_TEAM=$DEV_TEAM \
                   SKIP_INSTALL=NO \
                   BUILD_LIBRARY_FOR_DISTRIBUTION=YES \
                   -archivePath $DIST/CanvasNative.iphoneos.xcarchive

echo "Creating CanvasNative.xcframework"
OUTPUT_DIR="$DIST/CanvasNative.xcframework"
rm -rf $OUTPUT_DIR
xcodebuild -create-xcframework \
           -framework "$DIST/CanvasNative.iphonesimulator.xcarchive/Products/Library/Frameworks/CanvasNative.framework" \
           -framework "$DIST/CanvasNative.iphoneos.xcarchive/Products/Library/Frameworks/CanvasNative.framework" \
           -output "$OUTPUT_DIR"

DSYM_OUTPUT_DIR="$DIST/CanvasNative.framework.dSYM"
cp -r "$DIST/CanvasNative.iphoneos.xcarchive/dSYMs/CanvasNative.framework.dSYM/" $DSYM_OUTPUT_DIR
lipo -create \
    "$DIST/CanvasNative.iphonesimulator.xcarchive/dSYMs/CanvasNative.framework.dSYM/Contents/Resources/DWARF/CanvasNative" \
    "$DIST/CanvasNative.iphoneos.xcarchive/dSYMs/CanvasNative.framework.dSYM/Contents/Resources/DWARF/CanvasNative" \
    -output "$DSYM_OUTPUT_DIR/Contents/Resources/DWARF/CanvasNative"

pushd $DIST
zip -qr "CanvasNative.framework.dSYM.zip" "CanvasNative.framework.dSYM"
rm -rf "CanvasNative.framework.dSYM"
popd

rm -rf "$DIST/CanvasNative.iphonesimulator.xcarchive"
rm -rf "$DIST/CanvasNative.iphoneos.xcarchive"
