Third-party sources for AudioContextNative

This folder is intended to hold the sources for libogg, libvorbis, libopus, and libopusfile used by the framework.

Recommended workflow

1. From the framework folder run the fetch script to download sources:

   ```bash
   cd packages/audio-context/src-native/ios/AudioContextNative/AudioContextNative
   ./scripts/fetch_opus_deps.sh
   ```

2. Build static libraries for iOS (arm64, x86_64 simulator, etc.). Cross-compiling autotools projects for iOS requires setting CC/CFLAGS/LDFLAGS and using a proper iOS toolchain. Example references:
   - Use `./configure --host` with `CC` set to an Xcode clang wrapper
   - Consider using `ios-cmake` or building on macOS and lipo-ing archives for multiple architectures

3. Add the resulting `libogg.a`, `libvorbis.a`, `libopus.a`, and `libopusfile.a` files to the `AudioContextNative` Xcode target "Link Binary With Libraries" phase.

4. Add header search paths to the target's build settings pointing to the extracted source `include`/`src` folders.

Notes

- The fetch script only downloads and extracts sources. Building for iOS is platform-specific and left to the integrator.
- If you prefer vendoring sources directly in the repo, copy the extracted folders into this directory and commit them.
- If you'd like, I can also add an Xcode project update (pbxproj edits) to add sources/headers automatically — tell me if you want me to do that.
