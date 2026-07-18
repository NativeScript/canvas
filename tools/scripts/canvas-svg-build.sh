#!/bin/bash
cd ../../packages/canvas-svg/src-native/ios
set -e


# Replace (don't merge into) the existing framework so stale slices can't linger.
rm -rf ../../platforms/ios/CanvasSVG.xcframework || true
mkdir -p ../../platforms/ios

echo "Build iOS"
./build.sh
#cd ..
echo "Copy /dist/*.xcframework platforms/ios"

cp -R dist/CanvasSVG.xcframework ../../platforms/ios
