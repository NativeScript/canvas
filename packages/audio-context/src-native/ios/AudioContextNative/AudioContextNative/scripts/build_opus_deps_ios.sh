#!/usr/bin/env bash
set -euo pipefail

# Cross-compile libogg, libopus and libopusfile for iOS (device + simulator)
# Produces universal static libraries under third_party/build/universal/lib
# and headers under third_party/build/universal/include

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TP_DIR="$ROOT_DIR/third_party"
mkdir -p "$TP_DIR/build"

# Ensure sources exist
if [ ! -d "$TP_DIR/libogg" ] || [ ! -d "$TP_DIR/opus" ] || [ ! -d "$TP_DIR/opusfile" ]; then
  echo "Sources not found in $TP_DIR. Running fetch_opus_deps.sh"
  "$ROOT_DIR/scripts/fetch_opus_deps.sh"
fi

JOBS=$(getconf _NPROCESSORS_ONLN 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 1)
MIN_IOS_VERSION="${MIN_IOS_VERSION:-11.0}"

# Targets: SDK ARCH HOST_TRIPLE
MIN_XROS_VERSION="${MIN_XROS_VERSION:-1.0}"

# Targets: SDK ARCH HOST_TRIPLE. visionOS is Apple-Silicon only (arm64 device + arm64 simulator).
TARGETS=(
  "iphoneos arm64 arm-apple-darwin"
  "iphonesimulator x86_64 x86_64-apple-darwin"
  "iphonesimulator arm64 arm-apple-darwin"
  "xros arm64 arm-apple-darwin"
  "xrsimulator arm64 arm-apple-darwin"
)

LIBS=(libogg libvorbis libopus libopusfile)

build_for_target() {
  SDK="$1"; ARCH="$2"; HOST="$3";
  echo "\n=== Building for SDK=$SDK ARCH=$ARCH HOST=$HOST ==="
  SYSROOT=$(xcrun --sdk "$SDK" --show-sdk-path)
  CC_TOOL=$(xcrun --sdk "$SDK" --find clang)
  export CC="$CC_TOOL -arch $ARCH"
  # set min version flag depending on sdk. visionOS has no -m*-version-min flag and no
  # bitcode; it uses a target triple (-target <arch>-apple-xros[-simulator]).
  BITCODE="-fembed-bitcode"
  if [ "$SDK" = "iphoneos" ]; then
    MINFLAG="-miphoneos-version-min=${MIN_IOS_VERSION}"
  elif [ "$SDK" = "iphonesimulator" ]; then
    MINFLAG="-mios-simulator-version-min=${MIN_IOS_VERSION}"
  elif [ "$SDK" = "xros" ]; then
    MINFLAG="-target ${ARCH}-apple-xros${MIN_XROS_VERSION}"
    BITCODE=""
  elif [ "$SDK" = "xrsimulator" ]; then
    MINFLAG="-target ${ARCH}-apple-xros${MIN_XROS_VERSION}-simulator"
    BITCODE=""
  fi
  export CFLAGS="-isysroot $SYSROOT -arch $ARCH $MINFLAG $BITCODE"
  export LDFLAGS="-isysroot $SYSROOT -arch $ARCH $MINFLAG"
  export CPPFLAGS="$CFLAGS"

  PREFIX="$TP_DIR/build/${SDK}-${ARCH}"
  mkdir -p "$PREFIX"
  # per-target logs
  LOG_DIR="$TP_DIR/build/logs/${SDK}-${ARCH}"
  mkdir -p "$LOG_DIR"

  # Build libogg
  pushd "$TP_DIR/libogg" >/dev/null
  make distclean >/dev/null 2>&1 || true
  ./configure --host="$HOST" --disable-shared --enable-static --prefix="$PREFIX" CC="$CC" CFLAGS="$CFLAGS" LDFLAGS="$LDFLAGS" || true
  make -j${JOBS} || true
  make install || true
  popd >/dev/null

  # Build libvorbis (depends on libogg)
  pushd "$TP_DIR/libvorbis" >/dev/null
  make distclean >/dev/null 2>&1 || true
  export PKG_CONFIG_PATH="$PREFIX/lib/pkgconfig"
  echo "Configuring libvorbis for $SDK-$ARCH (logs: $LOG_DIR/libvorbis-configure.log)"
  ./configure --host="$HOST" --disable-shared --enable-static --with-ogg="$PREFIX" --prefix="$PREFIX" CC="$CC" CFLAGS="$CFLAGS" LDFLAGS="$LDFLAGS" 2>&1 | tee "$LOG_DIR/libvorbis-configure.log"
  # On newer Apple toolchains the generated libtool may include the
  # obsolete '-force_cpusubtype_ALL' token which newer 'ld' rejects.
  # Remove it from the generated libtool script so builds succeed.
  if [ -f "$TP_DIR/libvorbis/libtool" ]; then
    echo "Patching libvorbis/libtool to remove -force_cpusubtype_ALL"
    # BSD sed requires an explicit suffix for -i; use .bak then remove it
    sed -i.bak 's/ -force_cpusubtype_ALL//g' "$TP_DIR/libvorbis/libtool" || true
    rm -f "$TP_DIR/libvorbis/libtool.bak" || true
  fi
  # Also strip the same token from any generated Makefiles or other build files
  # so compiler/linker invocations don't pick it up.
  if grep -R --line-number --exclude-dir=.git "force_cpusubtype_ALL" "$TP_DIR/libvorbis" >/dev/null 2>&1; then
    echo "Stripping -force_cpusubtype_ALL from generated libvorbis files"
    # Use portable sed -i with .bak suffix then delete backups
    find "$TP_DIR/libvorbis" -type f -name "Makefile*" -o -name "*.mk" -o -name "libtool" | while read -r f; do
      sed -i.bak 's/ -force_cpusubtype_ALL//g' "$f" 2>/dev/null || true
      rm -f "$f.bak" 2>/dev/null || true
    done
    # also process configure-generated include/Makefile files
    find "$TP_DIR/libvorbis" -type f -exec grep -Il "force_cpusubtype_ALL" {} \; | while read -r f; do
      sed -i.bak 's/ -force_cpusubtype_ALL//g' "$f" 2>/dev/null || true
      rm -f "$f.bak" 2>/dev/null || true
    done
  fi
  echo "Building libvorbis for $SDK-$ARCH (logs: $LOG_DIR/libvorbis-make.log)"
  make -j${JOBS} 2>&1 | tee "$LOG_DIR/libvorbis-make.log"
  echo "Installing libvorbis for $SDK-$ARCH (logs: $LOG_DIR/libvorbis-install.log)"
  make install 2>&1 | tee "$LOG_DIR/libvorbis-install.log"
  popd >/dev/null

  # Build opus
  pushd "$TP_DIR/opus" >/dev/null
  make distclean >/dev/null 2>&1 || true
  # Disable assembly optimizations to avoid platform-specific assembler syntax
  # issues when cross-compiling with Apple's toolchain.
  ./configure --host="$HOST" --disable-shared --enable-static --disable-asm --prefix="$PREFIX" CC="$CC" CFLAGS="$CFLAGS" LDFLAGS="$LDFLAGS" || true
  make -j${JOBS} || true
  make install || true
  popd >/dev/null

  # Build opusfile
  pushd "$TP_DIR/opusfile" >/dev/null
  make distclean >/dev/null 2>&1 || true
  # Ensure pkg-config knows where to find built .pc files
  export PKG_CONFIG_PATH="$PREFIX/lib/pkgconfig"
  # Disable HTTP and examples to avoid linking host OpenSSL binaries and
  # unnecessary example programs when cross-compiling for iOS.
  ./configure --host="$HOST" --disable-shared --enable-static --disable-http --disable-examples --prefix="$PREFIX" CC="$CC" CFLAGS="$CFLAGS" LDFLAGS="$LDFLAGS" || true
  make -j${JOBS} || true
  make install || true
  popd >/dev/null

  echo "Installed to: $PREFIX"
}

# Build for each target
for t in "${TARGETS[@]}"; do
  read -r SDK ARCH HOST <<< "$t"
  build_for_target "$SDK" "$ARCH" "$HOST"
done

# Create XCFrameworks (preferred) from per-arch libraries. This avoids
# lipo conflicts when device and simulator slices share the same arch.
UNIVERSAL_LIB_DIR="$TP_DIR/build/universal"
UNIVERSAL_INCLUDE_DIR="$TP_DIR/build/universal/include"
mkdir -p "$UNIVERSAL_LIB_DIR" "$UNIVERSAL_INCLUDE_DIR"

for lib in "${LIBS[@]}"; do
  xc_args=()
  any_found=0
  for t in "${TARGETS[@]}"; do
    read -r SDK ARCH HOST <<< "$t"
    p="$TP_DIR/build/${SDK}-${ARCH}/lib/${lib}.a"
    inc="$TP_DIR/build/${SDK}-${ARCH}/include"
    if [ -f "$p" ]; then
      xc_args+=("-library" "$p" "-headers" "$inc")
      any_found=1
    fi
  done
  if [ "$any_found" -eq 0 ]; then
    echo "Warning: no built archives found for $lib"
    continue
  fi
  OUT="$UNIVERSAL_LIB_DIR/${lib}.xcframework"
  echo "Creating XCFramework for $lib -> $OUT"
  xcodebuild -create-xcframework "${xc_args[@]}" -output "$OUT" || {
    echo "xcframework creation failed for $lib, copying first available static archive instead"
    # fallback: copy the first available .a into a flat lib folder
    mkdir -p "$UNIVERSAL_LIB_DIR/lib"
    cp "${xc_args[1]}" "$UNIVERSAL_LIB_DIR/lib/${lib}.a" || true
  }
done

# Copy includes from one of the built prefixes (prefer iphoneos-arm64)
SRC_INC="$TP_DIR/build/iphoneos-arm64/include"
if [ ! -d "$SRC_INC" ]; then
  # fallback to first available
  for t in "${TARGETS[@]}"; do
    read -r SDK ARCH HOST <<< "$t"
    if [ -d "$TP_DIR/build/${SDK}-${ARCH}/include" ]; then
      SRC_INC="$TP_DIR/build/${SDK}-${ARCH}/include"
      break
    fi
  done
fi

if [ -d "$SRC_INC" ]; then
  cp -R "$SRC_INC/"* "$UNIVERSAL_INCLUDE_DIR/"
else
  echo "Warning: no include dir found to copy to universal include"
fi

echo "\n=== iOS cross-build complete ==="
echo "XCFrameworks: $UNIVERSAL_LIB_DIR"
echo "Universal includes: $UNIVERSAL_INCLUDE_DIR"
