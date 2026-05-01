#!/usr/bin/env bash
set -euo pipefail
set -x

echo "Building AudioContextNative.xcframework..."

# Resolve repository root (prefer git if available)
if git rev-parse --show-toplevel >/dev/null 2>&1; then
  REPO_ROOT=$(git rev-parse --show-toplevel)
else
  REPO_ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
fi

WORK_ROOT="$REPO_ROOT/packages/audio-context/src-native/ios/AudioContextNative/AudioContextNative"
PROJECT="$REPO_ROOT/packages/audio-context/src-native/ios/AudioContextNative/AudioContextNative.xcodeproj"
SCHEME="AudioContextNative"
BUILD_DIR="$WORK_ROOT/build_xc"
DIST="$WORK_ROOT/dist"

DEPS_SCRIPT="$WORK_ROOT/scripts/build_opus_deps_ios.sh"
if [ -f "$DEPS_SCRIPT" ]; then
  echo "Ensuring third-party deps are built (this may take a while)..."
  bash "$DEPS_SCRIPT"
else
  echo "No deps script found at $DEPS_SCRIPT; continuing."
fi

rm -rf "$BUILD_DIR" "$DIST"
mkdir -p "$BUILD_DIR" "$DIST"

## Temporarily remove doc CSS files that Xcode may auto-include and cause duplicate resource errors.
## Record deleted paths for restore from git after archiving.
TMP_DELETED_LIST="$BUILD_DIR/removed_docs.txt"
rm -f "$TMP_DELETED_LIST" || true
echo "Cleaning up doc files to avoid duplicate resource conflicts..."
# Remove documentation artifacts under any nested third_party doc locations.
# This covers both "$WORK_ROOT/third_party" and "$WORK_ROOT/AudioContextNative/third_party" (and similar).
find "$WORK_ROOT" -type f \( -path "*/third_party/*/doc/*" -o -path "*/third_party/*/share/doc/*" -o -path "*/third_party/doc/*" -o -path "*/third_party/share/doc/*" \) \( -name '*.css' -o -name '*.html' -o -name 'stamp-*' -o -name 'stamp_*' \) -print -exec rm -f {} \; >> "$TMP_DELETED_LIST" || true

echo "Archiving device build..."
xcodebuild archive \
  -project "$PROJECT" \
  -scheme "$SCHEME" \
  -configuration Release \
  -destination "generic/platform=iOS" \
  -archivePath "$BUILD_DIR/AudioContextNative-iOS.xcarchive" \
  SKIP_INSTALL=NO BUILD_LIBRARY_FOR_DISTRIBUTION=YES CODE_SIGNING_ALLOWED=NO CODE_SIGNING_REQUIRED=NO

echo "Archiving simulator build..."
xcodebuild archive \
  -project "$PROJECT" \
  -scheme "$SCHEME" \
  -configuration Release \
  -destination "generic/platform=iOS Simulator" \
  -archivePath "$BUILD_DIR/AudioContextNative-SIM.xcarchive" \
  SKIP_INSTALL=NO BUILD_LIBRARY_FOR_DISTRIBUTION=YES CODE_SIGNING_ALLOWED=NO CODE_SIGNING_REQUIRED=NO


SIM_BIN="$BUILD_DIR/AudioContextNative-SIM.xcarchive/Products/Library/Frameworks/AudioContextNative.framework/AudioContextNative"
NEED_SIM_X86=1
if [ -f "$SIM_BIN" ]; then
  if command -v lipo >/dev/null 2>&1; then
    ARCHS=$(lipo -archs "$SIM_BIN" 2>/dev/null || true)
  else
    ARCHS=""
  fi
  if [ -n "$ARCHS" ] && echo "$ARCHS" | grep -qw x86_64 && echo "$ARCHS" | grep -qw arm64; then
    echo "Simulator framework already contains both archs: $ARCHS; skipping explicit x86_64 archive."
    NEED_SIM_X86=0
  else
    echo "Simulator framework architectures: $ARCHS"
  fi
else
  echo "Simulator framework binary not found; will build explicit x86_64 archive."
fi

if [ "$NEED_SIM_X86" -eq 1 ]; then
  echo "Archiving simulator build (x86_64 slice)..."
  xcodebuild archive \
    -project "$PROJECT" \
    -scheme "$SCHEME" \
    -configuration Release \
    -destination "generic/platform=iOS Simulator" \
    -archivePath "$BUILD_DIR/AudioContextNative-SIM-x86_64.xcarchive" \
    SKIP_INSTALL=NO BUILD_LIBRARY_FOR_DISTRIBUTION=YES CODE_SIGNING_ALLOWED=NO CODE_SIGNING_REQUIRED=NO ARCHS="x86_64"
fi

echo "Creating XCFramework..."
FRAME_IOS="$BUILD_DIR/AudioContextNative-iOS.xcarchive/Products/Library/Frameworks/AudioContextNative.framework"
FRAME_SIM="$BUILD_DIR/AudioContextNative-SIM.xcarchive/Products/Library/Frameworks/AudioContextNative.framework"
FRAME_SIM_X86="$BUILD_DIR/AudioContextNative-SIM-x86_64.xcarchive/Products/Library/Frameworks/AudioContextNative.framework"
echo "Frameworks to combine:"
echo "  $FRAME_IOS"
echo "  $FRAME_SIM"
frames=("$FRAME_IOS" "$FRAME_SIM")
if [ "$NEED_SIM_X86" -eq 1 ]; then
  echo "  $FRAME_SIM_X86"
  frames+=("$FRAME_SIM_X86")
fi
for f in "${frames[@]}"; do
  if [ ! -d "$f" ]; then
    echo "ERROR: expected framework not found: $f" >&2
    exit 1
  fi
done

cf_args=()
for f in "${frames[@]}"; do
  cf_args+=( -framework "$f" )
done

xcodebuild -create-xcframework "${cf_args[@]}" -output "$DIST/AudioContextNative.xcframework"

DEST="$REPO_ROOT/packages/audio-context/platforms/ios"
mkdir -p "$DEST"
cp -R "$DIST/AudioContextNative.xcframework" "$DEST/"

echo "Done. XCFramework copied to: $DEST/AudioContextNative.xcframework"

# Restore any deleted tracked doc files so the repo isn't left modified.
if git rev-parse --is-inside-work-tree >/dev/null 2>&1 && [ -f "$TMP_DELETED_LIST" ]; then
  echo "Restoring deleted doc files from git (if tracked)..."
  # Convert absolute paths to repo-relative paths and restore
  sed "s#^$REPO_ROOT/##" "$TMP_DELETED_LIST" | tr '\n' ' ' | xargs -r git checkout -- || true
  rm -f "$TMP_DELETED_LIST" || true
fi
