//
// Created by Osei Fortune on 25/02/2023.
//

#pragma once

#include <memory>
#include <array>
#include "canvas2d/CanvasRenderingContext2DImpl.h"
#include "Helpers.h"

#include "ImageAssetImpl.h"
#include "ImageBitmapImpl.h"
#include "TextDecoderImpl.h"
#include "TextEncoderImpl.h"
#include "canvas2d/MatrixImpl.h"

#include "webgl/WebGLRenderingContextBase.h"
#include "webgl/WebGLRenderingContext.h"
#include "webgl2/WebGL2RenderingContext.h"

#include "webgpu/GPUImpl.h"
#include "webgpu/GPUSupportedLimitsImpl.h"
#include "webgpu/GPUDeviceImpl.h"
#include "webgpu/GPUQueueImpl.h"
#include "webgpu/GPUCanvasContextImpl.h"

class CanvasJSIModule {
public:
    static void install(v8::Isolate *isolate);

    static void CreateImageBitmap(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Create2DContext(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Create2DContextWithPointer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReadFile(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetMime(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreateWebGLContext(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreateWebGL2Context(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreateWebGPUContextWithPointer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void AddFontFamily(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void AddFontData(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Base64Encode(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Base64Decode(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Base64DecodeAsync(const v8::FunctionCallbackInfo<v8::Value> &args);
};

