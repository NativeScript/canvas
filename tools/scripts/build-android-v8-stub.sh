#!/bin/bash
set -euo pipefail

# Wraps the prebuilt V8 static library (libv8_monolith.a) that
# NativeScript/v8-buildscripts publishes into a minimal libNativeScript.so
# per Android ABI, for Canvas's own native build to link against.
#
# This is NOT the real NativeScript Android runtime .so -- it's a
# build-time-only stand-in that exports the same v8:: symbols so CMake's
# linker is satisfied (see canvas/CMakeLists.txt's target_link_libraries).
# It's excluded from AAR packaging (see include.gradle's pickFirst rule):
# the app supplies the real NativeScript runtime's libNativeScript.so at
# install time, and Android's dynamic linker resolves the real symbols
# against that copy at runtime -- this stub is never loaded by a real app.
#
# Prefer the real @nativescript/android-published libNativeScript.so once
# NativeScript/android ships a V8 14.9 release (tracked: PR #1987) -- this
# script exists so Canvas's own build isn't blocked on that release landing.
#
# Env vars:
#   V8_PREBUILT_CACHE   override the download/extract cache location
#   NDK / NDK_HOST       override NDK path / host tag (same as build-android.sh)
# Flags:
#   --release <tag>      override the pinned V8_RELEASE tag
#   --abi <abi>           restrict to one ABI (repeatable)

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

UPSTREAM_V8="NativeScript/v8-buildscripts"
RELEASE_FILE="$REPO_ROOT/V8_RELEASE"
CACHE_DIR="${V8_PREBUILT_CACHE:-$REPO_ROOT/.v8-prebuilt}"
JNILIBS_DIR="$REPO_ROOT/packages/canvas/src-native/canvas-android/canvas/src/main/jniLibs"

NDK_VERSION="29.0.14206865"
API_VERSION="21"

SHASUM_CMD="shasum -a 256 -c -"
if ! command -v shasum >/dev/null 2>&1; then
  SHASUM_CMD="sha256sum -c -"
fi

RELEASE_OVERRIDE=""
ABIS=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    --release) RELEASE_OVERRIDE="$2"; shift 2 ;;
    --abi) ABIS+=("$2"); shift 2 ;;
    *) echo "unknown argument: $1" >&2; exit 1 ;;
  esac
done

if [[ ${#ABIS[@]} -eq 0 ]]; then
  ABIS=(armeabi-v7a x86 arm64-v8a x86_64)
fi

if [[ -n "$RELEASE_OVERRIDE" ]]; then
  RELEASE="$RELEASE_OVERRIDE"
elif [[ -f "$RELEASE_FILE" ]]; then
  RELEASE="$(<"$RELEASE_FILE")"
else
  echo "no V8 release pinned: pass --release <tag> or create $RELEASE_FILE" >&2
  exit 1
fi

if [[ -z "${NDK:-}" ]]; then
  NDK="$ANDROID_HOME/ndk/$NDK_VERSION"
fi
if [[ -z "${NDK_HOST:-}" ]]; then
  NDK_HOST="darwin-x86_64"
fi
TOOLCHAIN="$NDK/toolchains/llvm/prebuilt/$NDK_HOST"

if [[ ! -d "$TOOLCHAIN" ]]; then
  echo "NDK toolchain not found at $TOOLCHAIN (set NDK/NDK_HOST env vars, or install NDK $NDK_VERSION)" >&2
  exit 1
fi

case "$(uname -s)" in
  Darwin) NM=llvm-nm ;;
  *) NM=llvm-nm ;;
esac

abi_triple() {
  case "$1" in
    armeabi-v7a) echo "armv7a-linux-androideabi" ;;
    x86) echo "i686-linux-android" ;;
    arm64-v8a) echo "aarch64-linux-android" ;;
    x86_64) echo "x86_64-linux-android" ;;
    *) echo "unknown ABI: $1" >&2; exit 1 ;;
  esac
}

mkdir -p "$CACHE_DIR"
STAGE_DIR="$(mktemp -d)"
trap 'rm -rf "$STAGE_DIR"' EXIT

BASE_URL="https://github.com/$UPSTREAM_V8/releases/download/$RELEASE"

echo "==> Fetching checksums for V8 $RELEASE"
curl -fSL --retry 3 -o "$STAGE_DIR/SHA256SUMS" "$BASE_URL/SHA256SUMS"

for abi in "${ABIS[@]}"; do
  asset="$(grep -oE "[^[:space:]]*-android-${abi}\.tar\.gz" "$STAGE_DIR/SHA256SUMS" | head -n1)"
  if [[ -z "$asset" ]]; then
    echo "could not find android-$abi asset in SHA256SUMS for release $RELEASE" >&2
    exit 1
  fi

  cached="$CACHE_DIR/$asset"
  if [[ ! -f "$cached" ]]; then
    echo "==> Fetching $asset"
    curl -fSL --retry 3 -o "$cached.part" "$BASE_URL/$asset"
    mv "$cached.part" "$cached"
  else
    echo "==> Using cached $asset"
  fi

  echo "==> Verifying checksum for $asset"
  (cd "$CACHE_DIR" && grep -F "$asset" "$STAGE_DIR/SHA256SUMS" | $SHASUM_CMD)

  echo "==> Extracting $asset"
  abi_stage="$STAGE_DIR/$abi"
  mkdir -p "$abi_stage"
  tar -xzf "$cached" -C "$abi_stage"

  monolith="$(find "$abi_stage" -name libv8_monolith.a | head -n1)"
  if [[ -z "$monolith" ]]; then
    echo "libv8_monolith.a not found in $asset" >&2
    exit 1
  fi

  triple="$(abi_triple "$abi")"
  clangxx="$TOOLCHAIN/bin/${triple}${API_VERSION}-clang++"
  if [[ ! -x "$clangxx" ]]; then
    echo "missing NDK toolchain binary: $clangxx" >&2
    exit 1
  fi

  out_dir="$JNILIBS_DIR/$abi"
  mkdir -p "$out_dir"
  echo "==> Linking $abi stub .so from libv8_monolith.a"
  "$clangxx" -shared -o "$out_dir/libNativeScript.so" \
    -Wl,--whole-archive "$monolith" -Wl,--no-whole-archive \
    -Wl,--build-id -Wl,-soname,libNativeScript.so

  sym_count="$("$TOOLCHAIN/bin/$NM" -D --defined-only "$out_dir/libNativeScript.so" | wc -l | tr -d ' ')"
  echo "    -> $out_dir/libNativeScript.so ($sym_count exported symbols)"
done

echo "==> Done: stub libNativeScript.so written for: ${ABIS[*]}"
echo "    Reminder: this is a build-time-only stub (see header comment) -- replace with the"
echo "    real @nativescript/android runtime .so once NativeScript/android#1987 ships."
