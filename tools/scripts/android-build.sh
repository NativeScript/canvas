#!/bin/bash
set -e
cd "$(dirname "$0")/../.."
REPO_ROOT="$(pwd)"

# Restores the nx targets build.native.android.{debug,release}, whose script was
# removed in "chore: v2 final" (#120) without the project.json entries being
# updated -- so `npm run build.canvas.android.release` has failed with
# "sh: android-build.sh: No such file or directory" ever since.
#
# The per-arch cargo invocations the old script hardcoded now live in
# tools/scripts/build-android.sh, driven by ARCHS_ANDROID in the root Makefile;
# this only sequences make -> gradle -> copy.

MODE="release"
GRADLE_TASK=":canvas:assembleRelease"
MAKE_TARGET="android"
for arg in "$@"; do
  case "$arg" in
    --release|-r) ;;
    --debug|-d)
      MODE="debug"
      GRADLE_TASK=":canvas:assembleDebug"
      MAKE_TARGET="android_debug"
      ;;
    --help|-h)
      echo "Usage: $0 [--release|--debug]"
      exit 0
      ;;
  esac
done

echo "Building Rust libs (make $MAKE_TARGET)"
make "$MAKE_TARGET"

echo "Assembling AAR ($MODE)"
cd packages/canvas/src-native/canvas-android
./gradlew "$GRADLE_TASK"
cd "$REPO_ROOT"

# The abiFilters in canvas/build.gradle and ARCHS_ANDROID in the Makefile are
# maintained separately; a mismatch silently ships an AAR missing an ABI rather
# than failing, so compare them here.
AAR="packages/canvas/src-native/canvas-android/canvas/build/outputs/aar/canvas-$MODE.aar"
for abi in $(sed -n 's/.*abiFilters //p' packages/canvas/src-native/canvas-android/canvas/build.gradle | tr -d "'" | tr ',' ' '); do
  if ! unzip -l "$AAR" | grep -q "jni/$abi/libcanvasnative.so"; then
    echo "error: $AAR has no jni/$abi/libcanvasnative.so" >&2
    echo "       abiFilters lists '$abi' but ARCHS_ANDROID did not build it." >&2
    exit 1
  fi
  echo "  ✓ $abi"
done

if [ "$MODE" = "release" ]; then
  sh tools/scripts/copy-android.sh
fi
