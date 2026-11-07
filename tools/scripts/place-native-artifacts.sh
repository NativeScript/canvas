#!/bin/bash
# Replaces the committed native binaries under packages/*/platforms with the ones
# .github/workflows/build-native.yml built, so npm ships what CI built from the same commit.
#
# Usage: tools/scripts/place-native-artifacts.sh <dir>
#   <dir>: an actions/download-artifact download of every build-native artifact
#          (one subdirectory per artifact name).
#
# Fails if any artifact is missing, so a partial build can't publish stale binaries.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ARTIFACTS="$(cd "${1:?usage: $0 <artifacts dir>}" && pwd)"
cd "$ROOT/packages"

need() {
    [ -e "$ARTIFACTS/$1" ] || { echo "error: artifact $1 missing" >&2; exit 1; }
}

for pair in canvas-android/canvas-release.aar:canvas \
            canvas-svg-android/canvassvg-release.aar:canvas-svg \
            audio-context-android/audiocontext-release.aar:audio-context; do
    src=${pair%%:*}
    need "$src"
    cp "$ARTIFACTS/$src" "${pair##*:}/platforms/android/"
done

# xcframeworks come tarred (upload-artifact drops file modes). Remove the committed one first
# so a slice CI no longer builds can't linger.
for triple in canvas-ios:canvas:CanvasNative \
              canvas-svg-ios:canvas-svg:CanvasSVG \
              audio-context-ios:audio-context:AudioContextNative; do
    IFS=: read -r artifact pkg framework <<< "$triple"
    need "$artifact/$artifact.tar.gz"
    rm -rf "$pkg/platforms/ios/$framework.xcframework"
    tar -xzf "$ARTIFACTS/$artifact/$artifact.tar.gz" -C "$pkg/platforms/ios"
done

MODULES="canvas/canvasnative canvas-svg/canvassvg audio-context/audiocontext canvas-media/canvasmedia canvas-gamepad/canvasgamepad"
for arch in x64 arm64; do
    for module in $MODULES; do
        pkg=${module%%/*}
        name=${module##*/}.node
        # <pkg>-windows-<arch>: the module (+ ANGLE DLLs for canvas)
        need "$pkg-windows-$arch/$name"
        rm -rf "$pkg/platforms/windows/$arch"
        mkdir -p "$pkg/platforms/windows/$arch"
        cp "$ARTIFACTS/$pkg-windows-$arch"/* "$pkg/platforms/windows/$arch/"
        # linux-<arch> is rooted at packages/
        f="$pkg/platforms/linux/lib/$arch/$name"
        need "linux-$arch/$f"
        rm -rf "$pkg/platforms/linux/lib/$arch"
        mkdir -p "$pkg/platforms/linux/lib/$arch"
        install -m 755 "$ARTIFACTS/linux-$arch/$f" "$f"
    done
done

echo "placed:"
git -C "$ROOT" status --short --untracked-files=all -- packages | sed 's/^/  /'
