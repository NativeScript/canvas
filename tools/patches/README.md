# Vendored upstream patches

Changes that belong in a dependency's own repository, kept here so the reason
they exist is recorded and so a fresh `cargo` checkout can be re-patched.

## `rust-skia-android-32bit-and-api-level.patch`

Against [triniwiz/rust-skia](https://github.com/triniwiz/rust-skia) at the rev
pinned in the root `Cargo.toml`. Apply from the repo root:

```sh
git apply tools/patches/rust-skia-android-32bit-and-api-level.patch
```

Two independent Android fixes:

1. **`build_support/platform.rs` + `platform/android.rs`** — `into_gn_args()`
   passed GN a bare `--target=<triple>`, which resets the effective min-SDK to
   the platform default and hides anything bionic guards with
   `__INTRODUCED_IN`. Skia m152's bundled ICU calls `posix_madvise` (API 23), so
   the C++ build failed to link. The API level Skia already builds against
   (`android::API_LEVEL`) is now appended to the triple.

2. **`build_support/skia_bindgen.rs`** — bindgen renders an opaque C++ type as
   `__BindgenOpaqueArray<T, N>`, choosing `T` so `size_of::<T>()` equals the
   alignment clang reported. That assumes Rust and C agree on `T`'s alignment.
   They don't for 64-bit integers on 32-bit x86: the i386 ABI aligns
   `long long`/`double`/`uint64_t` to 4, so Rust's `u64` is 4-aligned — while
   the types in question are 8-aligned because Skia declares their storage
   `alignas(8)` (`SkAnySubclass`, `SkCodec::GetPixelsCallback`, ...). The alias
   silently under-aligned, and five layout assertions failed to compile on
   `i686-linux-android`:

   | type | clang | rustc |
   | --- | --- | --- |
   | `GrBackendFormat` | 104 | 100 |
   | `GrBackendTexture` | 224 | 220 |
   | `GrBackendRenderTarget` | 456 | 440 |
   | `GrBackendSemaphore` | 40 | 36 |
   | `skgpu::MutableTextureState` | 8 | 4 |

   The generated aliases now point at a `#[repr(C, align(8))]` wrapper that
   states the alignment instead of inferring it. A `u64` element always means
   clang computed an alignment of 8, so this is a no-op on every target where
   `u64` is already 8-aligned (arm64, armv7, x86_64).

Once both land in the fork, bump the `rev` in `Cargo.toml` and delete this
patch.
