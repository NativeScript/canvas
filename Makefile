ARCHS_IOS = x86_64-apple-ios aarch64-apple-ios aarch64-apple-ios-sim
ARCHS_VISIONOS = aarch64-apple-visionos aarch64-apple-visionos-sim
ARCHS_ANDROID = i686-linux-android x86_64-linux-android aarch64-linux-android armv7-linux-androideabi

XCFRAMEWORK = CanvasNative.xcframework
RUST_LIB = canvasnative

all: GENERATE_HEADERS ios android

ios: $(XCFRAMEWORK)

visionos: $(ARCHS_VISIONOS)

android: GENERATE_ANDROID

ios-svg: GENERATE_IOS_SVG

visionos-svg: GENERATE_VISIONOS_SVG

svg: GENERATE_IOS_SVG GENERATE_VISIONOS_SVG

android-svg: GENERATE_ANDROID_SVG

.PHONY: GENERATE_HEADERS
GENERATE_HEADERS:
	./tools/scripts/build-headers.sh

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

.PHONY: ios-svg visionos-svg svg

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