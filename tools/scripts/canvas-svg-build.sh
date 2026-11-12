#!/bin/bash
cd ../../packages/canvas-svg/src-native/ios
set -e


# Replace (don't merge into) the existing framework so stale slices can't linger.
rm -rf ../../platforms/ios/CanvasSVGBinary/CanvasSVG.xcframework || true
mkdir -p ../../platforms/ios/CanvasSVGBinary

echo "Build iOS"
./build.sh
#cd ..
echo "Copy dist/CanvasSVG.xcframework to platforms/ios/CanvasSVGBinary"

cp -R dist/CanvasSVG.xcframework ../../platforms/ios/CanvasSVGBinary
