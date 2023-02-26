//
// Created by Osei Fortune on 25/02/2023.
//

#ifndef CANVAS_ANDROID_CANVASJSIMODULE_H
#define CANVAS_ANDROID_CANVASJSIMODULE_H

#import "v8runtime/V8Runtime.h"
#import "canvas-android/src/lib.rs.h"
#import "canvas-cxx/src/canvas2d.rs.h"
#import "canvas-cxx/src/webgl.rs.h"
#import "canvas-cxx/src/webgl2.rs.h"
#include "Helpers.h"

using namespace facebook::jsi;
using namespace std;


template<typename NativeFunc>
static void createGlobalFunc(Runtime &jsiRuntime, const char *prop, int paramCount, NativeFunc &&func) {
    auto f = Function::createFromHostFunction(jsiRuntime,
                                              PropNameID::forAscii(jsiRuntime, prop),
                                              paramCount,
                                              std::forward<NativeFunc>(func));
    jsiRuntime.global().setProperty(jsiRuntime, prop, std::move(f));
}

#define CREATE_GLOBAL_FUNC(prop, paramCount, func) \
    createFunc(jsiRuntime, prop, paramCount, func)



template<typename NativeFunc>
static void createFunc(Runtime &jsiRuntime, Object& object, const char *prop, int paramCount, NativeFunc &&func) {
auto f = Function::createFromHostFunction(jsiRuntime,
                                          PropNameID::forAscii(jsiRuntime, prop),
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

#endif //CANVAS_ANDROID_CANVASJSIMODULE_H
