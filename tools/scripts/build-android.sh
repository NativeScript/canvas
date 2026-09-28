#!/bin/bash


TARGET="$1"

MODE=${2:-release}

if [ "$TARGET" = "" ]; then
    echo "missing argument TARGET"
    echo "Usage: $0 TARGET"
    exit 1
fi

# The NDK names the 32-bit ARM toolchain "armv7a-linux-androideabi"; Rust spells
# the same target "armv7-linux-androideabi" (and older setups "arm-linux-androideabi").
# Only the latter used to be remapped, so `make android` -- which passes the Rust
# triple -- looked for an armv7-...-clang++ the NDK has never shipped.
NDK_TARGET=$TARGET
case "$TARGET" in
  arm-linux-androideabi|armv7-linux-androideabi)
    NDK_TARGET="armv7a-linux-androideabi"
    ;;
esac

# 23, paired with the NDK pin below. Skia m152's ICU calls posix_madvise, which
# bionic declares __INTRODUCED_IN(23) -- so compiling at 21 guards it out even
# though the NDK ships it. This is the NDK *compile* level; the AAR's
# minSdkVersion (canvas/build.gradle) is unchanged at 21, and skia itself
# already builds with ndk_api=26 regardless.
API_VERSION="23"
# NDK 28, not 29: NDK 29 dropped the posix_madvise declaration from
# <sys/mman.h> (only the POSIX_MADV_* macros remain), and the ICU bundled with
# Skia m152 calls it -- so skia-bindings fails to compile against NDK 29
# regardless of API level. Revisit when rust-skia's ICU stops using it.
NDK_VERSION="28.2.13676358"

# ANDROID_HOME is set by CI and by Android Studio's shell integration, but not
# in a plain login shell -- fall back to the usual locations so the script works
# standalone.
if [ -z "$ANDROID_HOME" ]; then
  ANDROID_HOME="${ANDROID_SDK_ROOT:-$HOME/Library/Android/sdk}"
fi

# needed so we can overwrite it in the CI
if [ -z "$NDK" ]; then
  NDK="$ANDROID_HOME/ndk/$NDK_VERSION"
fi

# needed so we can overwrite it in the CI ... defaults to mac
if [ -z "$NDK_HOST" ]; then
  NDK_HOST="darwin-x86_64"
fi


TOOLS="$NDK/toolchains/llvm/prebuilt/$NDK_HOST"

# Base linker flags:
#   --hash-style=sysv    : compatibility with older Android loaders
#   --gc-sections        : remove unreferenced sections from native deps (Skia, NDK, etc.)
#   -z,max-page-size     : required for Android 15+ 16KB page alignment
RUSTFLAGS="-C link-arg=-Wl,--hash-style=sysv"
RUSTFLAGS="$RUSTFLAGS -C link-arg=-Wl,--gc-sections"
RUSTFLAGS="$RUSTFLAGS -C link-arg=-Wl,-z,max-page-size=16384"

if [ "$TARGET" = "aarch64-linux-android" ]; then
    RUSTFLAGS="$RUSTFLAGS -C target-feature=-outline-atomics"
fi

if [ "$MODE" = "release" ]; then
    # -Zlocation-detail=none              : remove file/line info from panic messages
    # -Cpanic=immediate-abort             : abort at codegen level on panic
    # build-std-features=panic_immediate_abort : removes panic formatting machinery from rebuilt stdlib
    RUSTFLAGS="$RUSTFLAGS -Zlocation-detail=none -Zunstable-options -Cpanic=immediate-abort"
    EXTRA_ARGS="-Z build-std=std,panic_abort --release"
else
    EXTRA_ARGS=""
fi

CLANG="$TOOLS/bin/${NDK_TARGET}${API_VERSION}-clang"
CLANGXX="${CLANG}++"

for tool in "$CLANG" "$CLANGXX" "$TOOLS/bin/llvm-ar"; do
  if [ ! -x "$tool" ]; then
    echo "error: NDK tool not found: $tool" >&2
    echo "       NDK=$NDK NDK_HOST=$NDK_HOST TARGET=$TARGET (NDK_TARGET=$NDK_TARGET)" >&2
    exit 1
  fi
done

# cc-rs also needs CC: C-only dependencies (ring, stb_image) otherwise fall back
# to a bare "<target>-clang" that modern NDKs do not ship. The per-target
# CC_<triple>/CXX_<triple> forms are what cc-rs consults first when
# cross-compiling, so set those too.
# No --target in CFLAGS/CXXFLAGS: the NDK driver below is already
# ${NDK_TARGET}${API_VERSION}-clang, which encodes both the triple and the API
# level. Passing a bare "--target=$NDK_TARGET" on top overrides that and drops
# the API level back to the default, hiding APIs the driver would otherwise
# expose -- Skia m152's ICU trips over exactly this with posix_madvise.
TRIPLE_ENV=$(echo "$TARGET" | tr '-' '_')
LINKER_ENV="CARGO_TARGET_$(echo "$TRIPLE_ENV" | tr '[:lower:]' '[:upper:]')_LINKER"

# Self-sufficiency: CI's setup-android-native action exports these, so the
# script used to work only inside a shell that had already been prepared.
#
#   ANDROID_NDK  : skia-bindings' build script reads it directly and panics
#                  ("ANDROID_NDK variable not set") without it.
#   CARGO_TARGET_<TRIPLE>_LINKER
#                : otherwise rustc links with the host `cc` (Apple ld), which
#                  rejects the --version-script and --hash-style=sysv flags
#                  above.
export ANDROID_NDK="$NDK"

# Only the per-triple CC_<triple>/CXX_<triple>/AR_<triple> below -- a bare
# CC/CXX also applies to *host* compilation (build scripts, proc macros, and
# host-built C deps like zune-jpeg), and the Android clang cannot link those.
RUSTFLAGS="$RUSTFLAGS" \
env "CC_${TRIPLE_ENV}=$CLANG" "CXX_${TRIPLE_ENV}=$CLANGXX" "AR_${TRIPLE_ENV}=$TOOLS/bin/llvm-ar" \
    "${LINKER_ENV}=$CLANG" \
cargo +nightly build $EXTRA_ARGS --target $TARGET -p canvas-android

# Don't let the trailing strip guard below swallow a failed build: without this
# a cargo failure still exited 0, so `make android` reported success having
# produced no .so at all.
status=$?
if [ "$status" -ne 0 ]; then
  exit "$status"
fi

# Post-build: strip unneeded symbols not covered by Rust's strip=true
# (native deps like Skia can carry debug exports through the link step)
if [ "$MODE" = "release" ]; then
    LIB_PATH="target/$TARGET/release/libcanvasnative.so"
    if [ -f "$LIB_PATH" ]; then
        $TOOLS/bin/llvm-strip --strip-unneeded "$LIB_PATH"
    fi
fi


