#pragma once
#include <stdint.h>
#include <string.h>

#ifdef __APPLE__
#include "include/v8.h"

#ifdef __cplusplus
extern "C" {
#endif

// The framework is `CanvasSVG` and cbindgen writes `canvas_svg.h` into it; the
// `CanvasSVGNative/canvas_native_svg.h` this used to name never existed.
#include <CanvasSVG/canvas_svg.h>

#ifdef __cplusplus
}
#endif
#endif

#ifdef __ANDROID__
#include "include/v8.h"

#ifdef __cplusplus
extern "C" {
#endif

#include "include/canvas_native_svg.h"

#ifdef __cplusplus
}
#endif

#include <android/log.h>
#endif
