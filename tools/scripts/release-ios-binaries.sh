#!/bin/bash
# Moves the iOS xcframeworks out of the built npm packages and onto the GitHub release for
# <tag>, then points each package's <Framework>Binary/Package.swift at the release zip. The
# frameworks are too large to ship in the npm tarballs (canvas alone is over npm's limit).
#
# Usage: tools/scripts/release-ios-binaries.sh <tag>
#   Run after the packages are built into dist/packages. Needs GH_TOKEN with contents: write.
#   Creates the release (notes generated from the tag) if it does not exist yet.
#
# An asset already on the release is reused, never replaced: a package published by an
# earlier run of the same tag pins that asset's checksum.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="${1:?usage: $0 <tag>}"
REPO="${GITHUB_REPOSITORY:-NativeScript/canvas}"
SERVER="${GITHUB_SERVER_URL:-https://github.com}"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

if ! gh release view "$TAG" -R "$REPO" > /dev/null 2>&1; then
    flags=()
    if [ "$TAG" != "${TAG#*-}" ]; then
        flags+=(--prerelease)
    fi
    echo "==> creating release $TAG"
    gh release create "$TAG" -R "$REPO" --verify-tag --title "v$TAG" --generate-notes ${flags[@]+"${flags[@]}"}
fi
existing="$(gh release view "$TAG" -R "$REPO" --json assets -q '.assets[].name')"

for pair in canvas:CanvasNative canvas-svg:CanvasSVG audio-context:AudioContextNative; do
    pkg=${pair%%:*}
    fw=${pair##*:}
    dir="$ROOT/dist/packages/$pkg/platforms/ios/${fw}Binary"
    manifest="$dir/Package.swift"
    asset="$fw.xcframework.zip"
    [ -d "$dir/$fw.xcframework" ] || { echo "error: $dir/$fw.xcframework missing (build missing?)" >&2; exit 1; }

    if grep -qxF "$asset" <<< "$existing"; then
        echo "==> $asset is already on release $TAG, reusing it"
        gh release download "$TAG" -R "$REPO" -p "$asset" -D "$WORK"
    else
        echo "==> uploading $asset to release $TAG"
        # -y keeps the symlinks in the Catalyst slice's versioned framework layout
        (cd "$dir" && zip -qry "$WORK/$asset" "$fw.xcframework")
        gh release upload "$TAG" -R "$REPO" "$WORK/$asset"
    fi
    checksum="$(sha256sum "$WORK/$asset" | cut -d' ' -f1)"
    url="$SERVER/$REPO/releases/download/$TAG/$asset"

    # Consumers download without a token, so check the public URL serves the pinned bytes.
    served="$(curl -fsSL --retry 5 --retry-all-errors "$url" | sha256sum | cut -d' ' -f1)"
    if [ "$served" != "$checksum" ]; then
        echo "error: $url serves $served, expected $checksum" >&2
        exit 1
    fi

    sed "s#path: \"$fw.xcframework\"#url: \"$url\", checksum: \"$checksum\"#" "$manifest" > "$manifest.tmp"
    mv "$manifest.tmp" "$manifest"
    grep -qF "checksum: \"$checksum\"" "$manifest" || { echo "error: could not stamp $manifest" >&2; exit 1; }
    rm -rf "$dir/$fw.xcframework"
    echo "ok: $pkg -> $url ($checksum)"
done

left="$(find "$ROOT/dist/packages" -name '*.xcframework' -prune -print)"
if [ -n "$left" ]; then
    echo "error: xcframeworks still in the npm packages:" >&2
    echo "$left" >&2
    exit 1
fi
