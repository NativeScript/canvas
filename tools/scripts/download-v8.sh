#!/bin/bash
set -euo pipefail

# Vendors the V8 headers used by Canvas's V8 JS-binding layer
# (packages/canvas/platforms/ios/src/cpp/**), pinned to a specific
# NativeScript/v8-buildscripts release + NativeScript/ios commit, instead of
# git-committing the header tree. Mirrors the download_v8.sh mechanism
# NativeScript/android and NativeScript/ios themselves switched to for the
# V8 14.9 upgrade (checksum-verified pinned downloads, no committed binaries).
#
# Canvas only needs the public V8 API headers (v8.h, v8-isolate.h, ...) to
# compile against -- it never links against a raw V8 static lib itself
# (Android links its own .so against the NativeScript runtime's
# libNativeScript.so at build time; iOS defers all V8 symbol resolution to
# the app's final link against NativeScript.framework). Public V8 headers are
# identical across ABIs/variants for a given release, so a single archive per
# platform is enough -- the platform-specific .a/.so payloads in the release
# archives are never used here.
#
# The v8-inspector/crdtp headers historically vendored alongside the
# NativeScript runtime headers are not included by anything in this repo
# (verified by grep across packages/canvas) and are intentionally not
# re-vendored here -- no need to replicate NativeScript's
# vendor_inspector_sources.py closure computation for a dependency Canvas
# doesn't have.
#
# Env vars:
#   V8_SKIP_DOWNLOAD=1     skip entirely (e.g. testing a local V8 build)
#   V8_PREBUILT_CACHE=dir  override the download/extract cache location
# Flags:
#   --release <tag>        override the pinned V8_RELEASE tag
#   --runtime-ref <ref>    override the pinned NativeScript/ios ref
#   --force                re-vendor even if the stamp matches

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

UPSTREAM_V8="NativeScript/v8-buildscripts"
UPSTREAM_IOS="NativeScript/ios"

RELEASE_FILE="$REPO_ROOT/V8_RELEASE"
RUNTIME_REF_FILE="$REPO_ROOT/NATIVESCRIPT_IOS_RUNTIME_REF"

CACHE_DIR="${V8_PREBUILT_CACHE:-$REPO_ROOT/.v8-prebuilt}"
STAMP_FILE="$CACHE_DIR/.v8-release-stamp"

IOS_HEADERS_DEST="$REPO_ROOT/nativescript-v8/Headers"
ANDROID_HEADERS_DEST="$REPO_ROOT/packages/canvas/src-native/canvas-android/canvas/src/main/cpp/include"

SHASUM_CMD="shasum -a 256 -c -"
if ! command -v shasum >/dev/null 2>&1; then
  SHASUM_CMD="sha256sum -c -"
fi

FORCE=0
RELEASE_OVERRIDE=""
RUNTIME_REF_OVERRIDE=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --release) RELEASE_OVERRIDE="$2"; shift 2 ;;
    --runtime-ref) RUNTIME_REF_OVERRIDE="$2"; shift 2 ;;
    --force) FORCE=1; shift ;;
    *) echo "unknown argument: $1" >&2; exit 1 ;;
  esac
done

if [[ "${V8_SKIP_DOWNLOAD:-0}" == "1" ]]; then
  echo "V8_SKIP_DOWNLOAD=1 set - skipping V8 header vendoring"
  exit 0
fi

if [[ -n "$RELEASE_OVERRIDE" ]]; then
  RELEASE="$RELEASE_OVERRIDE"
elif [[ -f "$RELEASE_FILE" ]]; then
  RELEASE="$(<"$RELEASE_FILE")"
else
  echo "no V8 release pinned: pass --release <tag> or create $RELEASE_FILE" >&2
  exit 1
fi

if [[ -n "$RUNTIME_REF_OVERRIDE" ]]; then
  RUNTIME_REF="$RUNTIME_REF_OVERRIDE"
elif [[ -f "$RUNTIME_REF_FILE" ]]; then
  RUNTIME_REF="$(<"$RUNTIME_REF_FILE")"
else
  echo "no NativeScript/ios runtime-header ref pinned: pass --runtime-ref <ref> or create $RUNTIME_REF_FILE" >&2
  exit 1
fi

STAMP_CONTENT="$RELEASE|$RUNTIME_REF"
if [[ "$FORCE" != "1" && -f "$STAMP_FILE" ]] && grep -qxF "$STAMP_CONTENT" "$STAMP_FILE"; then
  echo "V8 $RELEASE (runtime headers @ $RUNTIME_REF) already vendored - skipping (use --force to reinstall)"
  exit 0
fi

mkdir -p "$CACHE_DIR"
STAGE_DIR="$(mktemp -d)"
trap 'rm -rf "$STAGE_DIR"' EXIT

# --- vendor_manifest: remembers exactly which relative paths under a dest dir
# were installed by this script on the previous run, so re-vendoring can
# remove headers that no longer exist in the new release without ever
# touching files this script didn't itself put there (canvas_native.h,
# robin_hood.h, zip.h, etc. all stay untouched). ---
clear_previous_vendor() {
  local dest="$1"
  local manifest="$dest/.vendored-files.list"
  if [[ -f "$manifest" ]]; then
    while IFS= read -r rel; do
      [[ -n "$rel" ]] && rm -f "$dest/$rel"
    done < "$manifest"
    find "$dest" -type d -empty -delete 2>/dev/null || true
  fi
}

write_vendor_manifest() {
  local src="$1" dest="$2"
  (cd "$src" && find . -type f -not -name '.vendored-files.list' | sed 's#^\./##') > "$dest/.vendored-files.list"
}

echo "==> Fetching V8 $RELEASE header archives"
BASE_URL="https://github.com/$UPSTREAM_V8/releases/download/$RELEASE"
curl -fSL --retry 3 -o "$STAGE_DIR/SHA256SUMS" "$BASE_URL/SHA256SUMS"

ANDROID_ASSET="$(grep -oE '[^[:space:]]*-android-arm64-v8a\.tar\.gz' "$STAGE_DIR/SHA256SUMS" | head -n1)"
IOS_ASSET="$(grep -oE '[^[:space:]]*-ios-arm64-device\.tar\.gz' "$STAGE_DIR/SHA256SUMS" | head -n1)"

if [[ -z "$ANDROID_ASSET" || -z "$IOS_ASSET" ]]; then
  echo "could not find expected asset names in SHA256SUMS for release $RELEASE" >&2
  exit 1
fi

for asset in "$ANDROID_ASSET" "$IOS_ASSET"; do
  echo "    - $asset"
  curl -fSL --retry 3 -o "$STAGE_DIR/$asset.part" "$BASE_URL/$asset"
  mv "$STAGE_DIR/$asset.part" "$STAGE_DIR/$asset"
done

echo "==> Verifying checksums"
(cd "$STAGE_DIR" && grep -E "$(printf '%s|%s' "$ANDROID_ASSET" "$IOS_ASSET")" SHA256SUMS | $SHASUM_CMD)

echo "==> Extracting"
mkdir -p "$STAGE_DIR/android" "$STAGE_DIR/ios"
tar -xzf "$STAGE_DIR/$ANDROID_ASSET" -C "$STAGE_DIR/android"
tar -xzf "$STAGE_DIR/$IOS_ASSET" -C "$STAGE_DIR/ios"

ANDROID_INCLUDE_SRC="$(find "$STAGE_DIR/android" -maxdepth 2 -type d -name include | head -n1)"
IOS_INCLUDE_SRC="$(find "$STAGE_DIR/ios" -maxdepth 2 -type d -name include | head -n1)"

if [[ -z "$ANDROID_INCLUDE_SRC" || -z "$IOS_INCLUDE_SRC" ]]; then
  echo "could not locate include/ directory inside extracted archives" >&2
  exit 1
fi

echo "==> Installing bare V8 headers"
clear_previous_vendor "$ANDROID_HEADERS_DEST"
mkdir -p "$ANDROID_HEADERS_DEST"
cp -R "$ANDROID_INCLUDE_SRC/." "$ANDROID_HEADERS_DEST/"
write_vendor_manifest "$ANDROID_INCLUDE_SRC" "$ANDROID_HEADERS_DEST"

clear_previous_vendor "$IOS_HEADERS_DEST/include"
mkdir -p "$IOS_HEADERS_DEST/include"
cp -R "$IOS_INCLUDE_SRC/." "$IOS_HEADERS_DEST/include/"
write_vendor_manifest "$IOS_INCLUDE_SRC" "$IOS_HEADERS_DEST/include"

echo "==> Fetching NativeScript/ios runtime headers @ $RUNTIME_REF"
curl -fSL --retry 3 -o "$STAGE_DIR/nativescript-ios.tar.gz" \
  "https://codeload.github.com/$UPSTREAM_IOS/tar.gz/$RUNTIME_REF"
mkdir -p "$STAGE_DIR/nativescript-ios"
tar -xzf "$STAGE_DIR/nativescript-ios.tar.gz" -C "$STAGE_DIR/nativescript-ios" --strip-components=1

RUNTIME_SRC="$STAGE_DIR/nativescript-ios/NativeScript/runtime"
if [[ ! -d "$RUNTIME_SRC" ]]; then
  echo "NativeScript/runtime not found at ref $RUNTIME_REF - repo layout may have changed" >&2
  exit 1
fi

echo "==> Installing NativeScript runtime headers (headers only, no .cpp/.mm)"
RUNTIME_DEST="$IOS_HEADERS_DEST/runtime"
clear_previous_vendor "$RUNTIME_DEST"
mkdir -p "$RUNTIME_DEST"
find "$RUNTIME_SRC" -maxdepth 1 -type f \( -name '*.h' -o -name '*.hpp' \) -exec cp {} "$RUNTIME_DEST/" \;
write_vendor_manifest "$RUNTIME_DEST" "$RUNTIME_DEST"

echo "$STAMP_CONTENT" > "$STAMP_FILE"
echo "==> Done: vendored V8 $RELEASE headers + NativeScript/ios runtime headers @ $RUNTIME_REF"
