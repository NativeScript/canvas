#!/bin/bash

cbindgen --config crates/canvas-c/cbindgen.toml  crates/canvas-c/src/lib.rs -l c > packages/canvas/src-native/canvas-ios/CanvasNative/include/canvas_native.h
cbindgen --config crates/canvas-ios/cbindgen.toml  crates/canvas-ios/src/lib.rs -l c > packages/canvas/src-native/canvas-ios/CanvasNative/include/canvas_ios.h
cbindgen --config crates/canvas-c/cbindgen.toml  crates/canvas-c/src/lib.rs -l c > packages/canvas/src-native/canvas-android/canvas/src/main/cpp/include/canvas_native.h