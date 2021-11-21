#!/bin/bash
cd ../../packages/canvas/src-native/canvas-ios
set -e


rm -rf ../platforms/ios || true
mkdir -p ../platforms/ios

echo "Build iOS"
./build.sh
#cd ..
echo "Copy /build/*.xcframework platforms/ios"

cp -R build/CanvasNative.xcframework ../../platforms/ios


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
