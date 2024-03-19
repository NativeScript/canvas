#!/bin/bash
cd ../../packages/canvas-svg/src-native/ios
set -e


rm -rf ../platforms/ios || true
mkdir -p ../platforms/ios

echo "Build iOS"
./build-debug.sh
#cd ..
echo "Copy /dist/*.xcframework platforms/ios"

cp -R dist/CanvasSVG.xcframework ../../platforms/ios
