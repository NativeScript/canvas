//
// Created by Osei Fortune on 10/06/2022.
//

#pragma once
#include <stdint.h>
#include <string.h>

#ifdef __APPLE__
#include <NativeScript/v8.h>
#include <CanvasNative/CanvasNative.h>
#endif

#ifdef __ANDROID__
#include "include/v8.h"
#include <android/log.h>
#include <thread>
#endif

