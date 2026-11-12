#!/bin/bash
# Downloads the prebuilt iOS xcframeworks from a GitHub release into each plugin's
# platforms/ios/<Framework>Binary, where its local SwiftPM package expects them. They are
# not committed. To work on the native code, build them instead (see README).
#
# Usage: tools/scripts/download-ios-frameworks.sh [tag]
#   tag: a release tag such as 3.0.6. Defaults to the latest release.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="${1:-}"
REPO="${CANVAS_REPO:-NativeScript/canvas}"
if [ -n "$TAG" ]; then
    BASE="https://github.com/$REPO/releases/download/$TAG"
else
    BASE="https://github.com/$REPO/releases/latest/download"
fi
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

for pair in canvas:CanvasNative canvas-svg:CanvasSVG audio-context:AudioContextNative; do
    pkg=${pair%%:*}
    fw=${pair##*:}
    dest="$ROOT/packages/$pkg/platforms/ios/${fw}Binary"
    echo "==> $fw.xcframework (${TAG:-latest release})"
    if ! curl -fsSL --retry 3 -o "$WORK/$fw.zip" "$BASE/$fw.xcframework.zip"; then
        echo "error: $BASE/$fw.xcframework.zip not found." >&2
        echo "Releases made before the frameworks moved to SwiftPM have none: pass a newer tag or build it." >&2
        exit 1
    fi
    rm -rf "$dest/$fw.xcframework"
    unzip -q "$WORK/$fw.zip" -d "$dest"
    [ -f "$dest/$fw.xcframework/Info.plist" ] || { echo "error: $fw.xcframework.zip did not contain $fw.xcframework" >&2; exit 1; }
done
echo "done"
