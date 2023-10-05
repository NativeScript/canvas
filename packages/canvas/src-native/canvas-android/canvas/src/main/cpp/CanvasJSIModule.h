//
// Created by Osei Fortune on 25/02/2023.
//

#pragma once

#include <memory>
#include <array>
#include "canvas2d/CanvasRenderingContext2DImpl.h"
#include "canvas-cxx/src/lib.rs.h"
#include "Helpers.h"

#include "ImageAssetImpl.h"
#include "ImageBitmapImpl.h"
#include "TextDecoderImpl.h"
#include "TextEncoderImpl.h"
#include "canvas2d/MatrixImpl.h"

#include "webgl/WebGLRenderingContextBase.h"
#include "webgl/WebGLRenderingContext.h"
#include "webgl2/WebGL2RenderingContext.h"

using namespace org::nativescript::canvas;



class CanvasJSIModule {
public:
    static void install(v8::Isolate * isolate);
    static void CreateImageBitmap(const v8::FunctionCallbackInfo<v8::Value> &args);
    static void Create2DContext(const v8::FunctionCallbackInfo<v8::Value> &args);
    static void Create2DContextWithPointer(const v8::FunctionCallbackInfo<v8::Value> &args);
};

