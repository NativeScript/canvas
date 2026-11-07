#!/usr/bin/env bash
# CI only (installed by the skia-cache action as `cargo`, first on PATH). Runs the real cargo,
# then files the Skia binaries skia-bindings exported during that run under their rust-skia key,
# where SKIA_BINARIES_URL finds them on the next run.
home="${CARGO_HOME:-$HOME/.cargo}"
command -v cygpath > /dev/null && home="$(cygpath -u "$home")"
"$home/bin/cargo" "$@"
status=$?

staging="$BUILD_ARTIFACTSTAGINGDIRECTORY"
cache="$SKIA_CACHE_DIR"
# GNU tar reads D:/... as host:path
command -v cygpath > /dev/null && staging="$(cygpath -u "$staging")" && cache="$(cygpath -u "$cache")"
export_dir="$staging/skia-binaries"
if [ "$status" -eq 0 ] && [ -f "$export_dir/key.txt" ]; then
    key="$(cat "$export_dir/key.txt")"
    archive="$cache/skia-binaries-$key.tar.gz"
    if [ ! -f "$archive" ]; then
        tar -czf "$archive" -C "$staging" skia-binaries
        echo "skia-cache: stored $(basename "$archive")"
    fi
    rm -rf "$export_dir"
fi
exit "$status"
