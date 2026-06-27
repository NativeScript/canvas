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

## Temporarily move aside doc CSS/HTML files that Xcode may auto-include and cause
## duplicate resource errors during the framework archive.
## IMPORTANT: the third_party sources (libvorbis in particular) are NOT tracked in git,
## and several of these files (e.g. libvorbis/doc/index.html) are *source* files the
## autotools build needs. We therefore back them up to a directory OUTSIDE $WORK_ROOT
## (so the find below can never re-match them) and restore from that backup after
## archiving — never via `git checkout`, which silently fails for untracked files.
DOC_BACKUP_DIR="$WORK_ROOT/../.doc_backup"
# Self-heal: if a previous run crashed after deleting but before restoring, put the
# backed-up docs back before we start so the deps build can find its sources.
if [ -d "$DOC_BACKUP_DIR" ]; then
  echo "Restoring doc files left over from a previous interrupted run..."
  (cd "$DOC_BACKUP_DIR" && find . -type f -print0 | while IFS= read -r -d '' f; do
    dest="$WORK_ROOT/${f#./}"
    mkdir -p "$(dirname "$dest")"
    cp "$f" "$dest"
  done)
fi
echo "Cleaning up doc files to avoid duplicate resource conflicts..."
# Move documentation artifacts under any nested third_party doc locations into the backup.
# This covers both "$WORK_ROOT/third_party" and "$WORK_ROOT/AudioContextNative/third_party" (and similar).
rm -rf "$DOC_BACKUP_DIR"
find "$WORK_ROOT" -type f \( -path "*/third_party/*/doc/*" -o -path "*/third_party/*/share/doc/*" -o -path "*/third_party/doc/*" -o -path "*/third_party/share/doc/*" \) \( -name '*.css' -o -name '*.html' -o -name 'stamp-*' -o -name 'stamp_*' \) -print0 | while IFS= read -r -d '' f; do
  rel="${f#$WORK_ROOT/}"
  bak="$DOC_BACKUP_DIR/$rel"
  mkdir -p "$(dirname "$bak")"
  mv "$f" "$bak"
done

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

# visionOS is Apple-Silicon only — pin ARCHS=arm64 (no x86_64-apple-visionos-sim slice).
echo "Archiving visionOS device build..."
xcodebuild archive \
  -project "$PROJECT" \
  -scheme "$SCHEME" \
  -configuration Release \
  -destination "generic/platform=visionOS" \
  -archivePath "$BUILD_DIR/AudioContextNative-XROS.xcarchive" \
  ARCHS=arm64 ONLY_ACTIVE_ARCH=NO SKIP_INSTALL=NO BUILD_LIBRARY_FOR_DISTRIBUTION=YES CODE_SIGNING_ALLOWED=NO CODE_SIGNING_REQUIRED=NO

echo "Archiving visionOS simulator build..."
xcodebuild archive \
  -project "$PROJECT" \
  -scheme "$SCHEME" \
  -configuration Release \
  -destination "generic/platform=visionOS Simulator" \
  -archivePath "$BUILD_DIR/AudioContextNative-XRSIM.xcarchive" \
  ARCHS=arm64 ONLY_ACTIVE_ARCH=NO SKIP_INSTALL=NO BUILD_LIBRARY_FOR_DISTRIBUTION=YES CODE_SIGNING_ALLOWED=NO CODE_SIGNING_REQUIRED=NO

echo "Creating XCFramework..."
FRAME_IOS="$BUILD_DIR/AudioContextNative-iOS.xcarchive/Products/Library/Frameworks/AudioContextNative.framework"
FRAME_SIM="$BUILD_DIR/AudioContextNative-SIM.xcarchive/Products/Library/Frameworks/AudioContextNative.framework"
FRAME_SIM_X86="$BUILD_DIR/AudioContextNative-SIM-x86_64.xcarchive/Products/Library/Frameworks/AudioContextNative.framework"
FRAME_XROS="$BUILD_DIR/AudioContextNative-XROS.xcarchive/Products/Library/Frameworks/AudioContextNative.framework"
FRAME_XRSIM="$BUILD_DIR/AudioContextNative-XRSIM.xcarchive/Products/Library/Frameworks/AudioContextNative.framework"
echo "Frameworks to combine:"
echo "  $FRAME_IOS"
echo "  $FRAME_SIM"
frames=("$FRAME_IOS" "$FRAME_SIM")
if [ "$NEED_SIM_X86" -eq 1 ]; then
  echo "  $FRAME_SIM_X86"
  frames+=("$FRAME_SIM_X86")
fi
echo "  $FRAME_XROS"
echo "  $FRAME_XRSIM"
frames+=("$FRAME_XROS" "$FRAME_XRSIM")
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
# Replace (don't merge into) any existing framework so stale slices can't linger.
rm -rf "$DEST/AudioContextNative.xcframework"
cp -R "$DIST/AudioContextNative.xcframework" "$DEST/"

echo "Done. XCFramework copied to: $DEST/AudioContextNative.xcframework"

# Restore the doc files we moved aside (these are untracked source files the autotools
# build needs, so restore from our backup dir — git checkout cannot restore them).
if [ -d "$DOC_BACKUP_DIR" ]; then
  echo "Restoring doc files moved aside before archiving..."
  (cd "$DOC_BACKUP_DIR" && find . -type f -print0 | while IFS= read -r -d '' f; do
    dest="$WORK_ROOT/${f#./}"
    mkdir -p "$(dirname "$dest")"
    mv "$f" "$dest"
  done)
  rm -rf "$DOC_BACKUP_DIR"
fi
