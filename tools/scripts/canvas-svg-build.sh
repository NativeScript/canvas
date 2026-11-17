#!/bin/bash
# Builds CanvasSVG.xcframework into packages/canvas-svg/platforms/ios/CanvasSVGBinary.
#
#   canvas-svg-build.sh                 every platform (what the npm script runs)
#   canvas-svg-build.sh ios tvos        only those platforms' slices, left in ios/dist
#   canvas-svg-build.sh --xcframework   the xcframework from the slices already in ios/dist
cd "$(dirname "$0")/../../packages/canvas-svg/src-native/ios"
set -e

# build.sh copies the xcframework into platforms/ios/CanvasSVGBinary.
./build.sh "$@"
