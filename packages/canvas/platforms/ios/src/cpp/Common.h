//
// Created by Osei Fortune on 10/06/2022.
//

#pragma once
#include <stdint.h>
#include <string.h>

#ifdef __APPLE__
#include <NativeScript/include/v8.h>

#ifdef __cplusplus
extern "C" {
#endif

#include <CanvasNative/canvas_ios.h>
#include <CanvasNative/canvas_native.h>

#ifdef __cplusplus
}
#endif
#endif

#ifdef __ANDROID__
#include "include/v8.h"
#include <android/log.h>
#include <thread>
#endif

