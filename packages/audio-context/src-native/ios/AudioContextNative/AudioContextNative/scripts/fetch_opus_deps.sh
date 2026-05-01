#!/usr/bin/env bash
set -euo pipefail

# Fetch libogg, libopus and libopusfile sources into the framework's third_party folder.
# Usage:
#   ./fetch_opus_deps.sh
# Override versions via env vars, e.g. LIBOGG_VERSION=1.3.5 ./fetch_opus_deps.sh

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TP_DIR="$ROOT_DIR/third_party"
mkdir -p "$TP_DIR"
cd "$TP_DIR"

LIBOGG_VERSION="${LIBOGG_VERSION:-1.3.5}"
OPUS_VERSION="${OPUS_VERSION:-1.5.2}"
OPUSFILE_VERSION="${OPUSFILE_VERSION:-0.12}"
LIBVORBIS_VERSION="${LIBVORBIS_VERSION:-1.3.7}"
LIBOGG_URL="https://downloads.xiph.org/releases/ogg/libogg-${LIBOGG_VERSION}.tar.gz"
OPUS_URL="https://github.com/xiph/opus/releases/download/v${OPUS_VERSION}/opus-${OPUS_VERSION}.tar.gz"
OPUSFILE_URL="https://github.com/xiph/opusfile/releases/download/v${OPUSFILE_VERSION}/opusfile-${OPUSFILE_VERSION}.tar.gz"
LIBVORBIS_URL="https://downloads.xiph.org/releases/vorbis/libvorbis-${LIBVORBIS_VERSION}.tar.gz"

echo "Fetching third-party sources into: $TP_DIR"

curl -L -f -o "libogg-${LIBOGG_VERSION}.tar.gz" "$LIBOGG_URL" || { echo "Failed to download libogg"; exit 1; }
curl -L -f -o "opus-${OPUS_VERSION}.tar.gz" "$OPUS_URL" || { echo "Failed to download opus"; exit 1; }
curl -L -f -o "opusfile-${OPUSFILE_VERSION}.tar.gz" "$OPUSFILE_URL" || { echo "Failed to download opusfile"; exit 1; }
curl -L -f -o "libvorbis-${LIBVORBIS_VERSION}.tar.gz" "$LIBVORBIS_URL" || { echo "Failed to download libvorbis"; exit 1; }

echo "Extracting..."
for f in libogg-${LIBOGG_VERSION}.tar.gz opus-${OPUS_VERSION}.tar.gz opusfile-${OPUSFILE_VERSION}.tar.gz; do
  tar xzf "$f"
done

tar xzf "libvorbis-${LIBVORBIS_VERSION}.tar.gz"

# Normalize extracted directory names (strip version suffixes)
echo "Normalizing extracted directories..."
for d in libogg-*; do
  if [ -d "$d" ]; then
    rm -rf libogg
    mv "$d" libogg
    break
  fi
done
for d in opus-*; do
  if [ -d "$d" ]; then
    rm -rf opus
    mv "$d" opus
    break
  fi
done
for d in opusfile-*; do
  if [ -d "$d" ]; then
    rm -rf opusfile
    mv "$d" opusfile
    break
  fi
done

for d in libvorbis-*; do
  if [ -d "$d" ]; then
    rm -rf libvorbis
    mv "$d" libvorbis
    break
  fi
done

echo "Done. Sources extracted to: $TP_DIR/{libogg,opus,opusfile,libvorbis}"

echo "Next steps: run the build script to compile static libraries, or integrate the sources into Xcode. See third_party/README.md for guidance."
