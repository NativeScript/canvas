ARCHS_IOS = x86_64-apple-ios aarch64-apple-ios aarch64-apple-ios-sim
ARCHS_VISIONOS = aarch64-apple-visionos aarch64-apple-visionos-sim
ARCHS_TVOS = aarch64-apple-tvos aarch64-apple-tvos-sim
ARCHS_ANDROID = i686-linux-android x86_64-linux-android aarch64-linux-android armv7-linux-androideabi

XCFRAMEWORK = CanvasNative.xcframework
RUST_LIB = canvasnative

all: GENERATE_HEADERS GENERATE_V8_HEADERS ios android

ios: $(XCFRAMEWORK)

visionos: $(ARCHS_VISIONOS)

tvos: $(ARCHS_TVOS)

android: GENERATE_ANDROID

ios-svg: GENERATE_IOS_SVG

visionos-svg: GENERATE_VISIONOS_SVG

tvos-svg: GENERATE_TVOS_SVG

svg: GENERATE_IOS_SVG GENERATE_VISIONOS_SVG GENERATE_TVOS_SVG

android-svg: GENERATE_ANDROID_SVG

# Host-side tests. The workspace pins `-C panic=abort` for the Apple host
# targets in .cargo/config.toml (needed by the macOS dylib build) and libtest
# cannot link against that, so the rustflags are replaced for this run.
.PHONY: test
test:
	python3 ./tools/tests/check-v8-bridge-invariants.py
	RUSTFLAGS="-C link-arg=-undefined -C link-arg=dynamic_lookup" \
	    cargo test -p canvas-c --features 2d,webgl,gl

.PHONY: GENERATE_HEADERS
GENERATE_HEADERS:
	./tools/scripts/build-headers.sh

.PHONY: GENERATE_V8_HEADERS
GENERATE_V8_HEADERS:
	./tools/scripts/download-v8.sh

.PHONY: GENERATE_ANDROID_V8_STUB
GENERATE_ANDROID_V8_STUB:
	./tools/scripts/build-android-v8-stub.sh

.PHONY: $(ARCHS_IOS)
$(ARCHS_IOS): %:
	RUSTFLAGS="-Zlocation-detail=none -Zunstable-options -Cpanic=immediate-abort" \
	cargo +nightly build -Z build-std='std,panic_abort' \
	    --target $@ --release -p canvas-ios

$(XCFRAMEWORK): $(ARCHS_IOS)

.PHONY: $(ARCHS_VISIONOS)
$(ARCHS_VISIONOS): %:
	RUSTFLAGS="-Zlocation-detail=none -Zunstable-options -Cpanic=immediate-abort" \
	cargo +nightly build -Z build-std='std,panic_abort' \
	    --target $@ --release -p canvas-ios

# TVOS_DEPLOYMENT_TARGET must be set explicitly: rustc defaults to 12.0 but the
# `cc` crate has no built-in tvOS default and falls back to the SDK version, so
# ring's C/asm objects come out tagged minos 26.4 and the framework link (12.0)
# warns on every one of them. iOS/visionOS need no equivalent -- cc knows those.
.PHONY: $(ARCHS_TVOS)
$(ARCHS_TVOS): %:
	TVOS_DEPLOYMENT_TARGET=12.0 \
	RUSTFLAGS="-Zlocation-detail=none -Zunstable-options -Cpanic=immediate-abort" \
	cargo +nightly build -Z build-std='std,panic_abort' \
	    --target $@ --release -p canvas-ios

.PHONY: $(ARCHS_ANDROID)
$(ARCHS_ANDROID): %:
	./tools/scripts/build-android.sh $@

.PHONY: GENERATE_ANDROID
GENERATE_ANDROID: $(ARCHS_ANDROID)

.PHONY: $(addsuffix _svg,$(ARCHS_IOS))
$(addsuffix _svg,$(ARCHS_IOS)): %_svg:
	RUSTFLAGS="-Zlocation-detail=none -Zunstable-options -Cpanic=immediate-abort" \
	cargo +nightly build -Z build-std='std,panic_abort' \
	    --target $* --release -p canvas-svg-ios

.PHONY: GENERATE_IOS_SVG
GENERATE_IOS_SVG: $(addsuffix _svg,$(ARCHS_IOS))

.PHONY: $(addsuffix _svg,$(ARCHS_VISIONOS))
$(addsuffix _svg,$(ARCHS_VISIONOS)): %_svg:
	RUSTFLAGS="-Zlocation-detail=none -Zunstable-options -Cpanic=immediate-abort" \
	cargo +nightly build -Z build-std='std,panic_abort' \
	    --target $* --release -p canvas-svg-ios

.PHONY: GENERATE_VISIONOS_SVG
GENERATE_VISIONOS_SVG: $(addsuffix _svg,$(ARCHS_VISIONOS))

.PHONY: $(addsuffix _svg,$(ARCHS_TVOS))
$(addsuffix _svg,$(ARCHS_TVOS)): %_svg:
	TVOS_DEPLOYMENT_TARGET=12.0 \
	RUSTFLAGS="-Zlocation-detail=none -Zunstable-options -Cpanic=immediate-abort" \
	cargo +nightly build -Z build-std='std,panic_abort' \
	    --target $* --release -p canvas-svg-ios

.PHONY: GENERATE_TVOS_SVG
GENERATE_TVOS_SVG: $(addsuffix _svg,$(ARCHS_TVOS))

.PHONY: ios-svg visionos-svg tvos-svg svg

.PHONY: $(addsuffix _svg,$(ARCHS_ANDROID))
$(addsuffix _svg,$(ARCHS_ANDROID)): %_svg:
	./tools/scripts/build-svg-android.sh $* svg

.PHONY: GENERATE_ANDROID_SVG
GENERATE_ANDROID_SVG: $(addsuffix _svg,$(ARCHS_ANDROID))

.PHONY: ios_debug
ios_debug: $(addsuffix _debug,$(ARCHS_IOS))

.PHONY: android_debug
android_debug: $(addsuffix _debug,$(ARCHS_ANDROID))

.PHONY: $(addsuffix _debug,$(ARCHS_IOS))
$(addsuffix _debug,$(ARCHS_IOS)): %_debug:
	cargo +nightly build --target $* -p canvas-ios

.PHONY: visionos_debug
visionos_debug: $(addsuffix _debug,$(ARCHS_VISIONOS))

.PHONY: $(addsuffix _debug,$(ARCHS_VISIONOS))
$(addsuffix _debug,$(ARCHS_VISIONOS)): %_debug:
	cargo +nightly build -Z build-std='std,panic_abort' --target $* -p canvas-ios

.PHONY: $(addsuffix _debug,$(ARCHS_ANDROID))
$(addsuffix _debug,$(ARCHS_ANDROID)): %_debug:
	./tools/scripts/build-android.sh $* debug

.PHONY: clean
clean:
	rm -rf target