//
// Created by Osei Fortune on 25/02/2023.
//

#pragma once

#include <memory>
#include <array>
#include <array>
#include "v8runtime/JSIV8ValueConverter.h"
#include "canvas2d/CanvasRenderingContext2DImpl.h"

#import "v8runtime/V8Runtime.h"
#import "canvas-android/src/lib.rs.h"
#import "canvas-cxx/src/canvas2d.rs.h"
#import "canvas-cxx/src/webgl.rs.h"
#import "canvas-cxx/src/webgl2.rs.h"
#include "Helpers.h"

#include "ImageAssetImpl.h"
#include "ImageBitmapImpl.h"
#include "TextDecoderImpl.h"
#include "TextEncoderImpl.h"
#include "canvas2d/MatrixImpl.h"

#include "webgl/WebGLRenderingContextBase.h"
#include "webgl/WebGLRenderingContext.h"
#include "webgl2/WebGL2RenderingContext.h"

using namespace facebook;

template<typename NativeFunc>
static void
createGlobalFunc(jsi::Runtime &jsiRuntime, const char *prop, int paramCount, NativeFunc &&func) {
    auto f = jsi::Function::createFromHostFunction(jsiRuntime,
                                                   jsi::PropNameID::forAscii(jsiRuntime, prop),
                                                   paramCount,
                                                   std::forward<NativeFunc>(func));
    jsiRuntime.global().setProperty(jsiRuntime, prop, std::move(f));
}

#define CREATE_GLOBAL_FUNC(prop, paramCount, func) \
    createFunc(jsiRuntime, prop, paramCount, func)


template<typename NativeFunc>
static void
createFunc(jsi::Runtime &jsiRuntime, jsi::Object &object, const char *prop, int paramCount,
           NativeFunc &&func) {
    auto f = jsi::Function::createFromHostFunction(jsiRuntime,
                                                   jsi::PropNameID::forAscii(jsiRuntime, prop),
                                                   paramCount,
                                                   std::forward<NativeFunc>(func));
    object.setProperty(jsiRuntime, prop, std::move(f));
}

#define CREATE_FUNC(prop, object, paramCount, func) \
    createFunc(jsiRuntime, object, prop, paramCount, func)

class CanvasJSIModule {
public:
    static void install(facebook::jsi::Runtime &rt);
};

