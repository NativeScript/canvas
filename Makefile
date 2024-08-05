ARCHS_IOS = x86_64-apple-ios aarch64-apple-ios aarch64-apple-ios-sim
ARCHS_ANDROID = i686-linux-android x86_64-linux-android aarch64-linux-android armv7-linux-androideabi
# LIB = libcanvasative.dylib
XCFRAMEWORK = CanvasNative.xcframework
RUST_LIB = canvasnative

all:GENERATE_HEADERS ios android

ios: $(XCFRAMEWORK)

android: GENERATE_ANDROID

android_svg: GENERATE_ANDROID_SVG

ios_svg: GENERATE_IOS_SVG

.PHONY: GENERATE_HEADERS
GENERATE_HEADERS:
	./tools/scripts/build-headers.sh

# PHONY keyword on make means this is not a file, just an identifier for a target
.PHONY: $(ARCHS_IOS)
$(ARCHS_IOS): %:
	RUSTFLAGS="-Zlocation-detail=none -C panic=abort" cargo +nightly build -Z build-std='std,panic_abort'  -Z build-std-features=panic_immediate_abort --target $@ --release -p canvas-svg

$(XCFRAMEWORK): $(ARCHS_IOS)

.PHONY: $(ARCHS_ANDROID)
$(ARCHS_ANDROID): %:
	./tools/scripts/build-android.sh $@ && ./tools/scripts/copy-android.sh $@

.PHONY: GENERATE_ANDROID
GENERATE_ANDROID: $(ARCHS_ANDROID)

# .PHONY: $(ARCHS_IOS)_SVG
# $(ARCHS_IOS)_SVG: %_svg:
# 	./tools/scripts/build-svg-ios.sh $* && ./tools/scripts/copy-svg-ios.sh $*

# .PHONY: GENERATE_IOS_SVG
# GENERATE_IOS_SVG: $(addsuffix _svg,$(ARCHS_IOS))

# .PHONY: $(ARCHS_ANDROID)_SVG
# $(ARCHS_ANDROID)_SVG: %_svg:
# 	./tools/scripts/build-svg-android.sh $* && ./tools/scripts/copy-svg-android.sh $*

# .PHONY: GENERATE_ANDROID_SVG
# GENERATE_ANDROID_SVG: $(addsuffix _svg,$(ARCHS_ANDROID))

.PHONY: clean
clean:
	rm -rf target