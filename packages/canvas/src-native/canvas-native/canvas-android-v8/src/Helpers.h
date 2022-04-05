//
// Created by Osei Fortune on 22/03/2022.
//

#ifndef CANVAS_NATIVE_HELPERS_H
#define CANVAS_NATIVE_HELPERS_H

#include "Common.h"
#include "rust/cxx.h"
#include "canvas-android-v8/src/bridges/utils.rs.h"

class Helpers {
public:
    static void ThrowIllegalConstructor(v8::Isolate *isolate);

    static v8::Local<v8::String> ConvertToV8String(v8::Isolate *isolate, const std::string& string);

    static rust::String ConvertFromV8String(v8::Isolate *isolate, const v8::Local<v8::String>& value);
};


#endif //CANVAS_NATIVE_HELPERS_H
