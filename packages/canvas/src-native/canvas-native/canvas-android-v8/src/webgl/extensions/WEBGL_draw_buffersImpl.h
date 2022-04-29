//
// Created by Osei Fortune on 29/04/2022.
//

#pragma once

#include "../../Common.h"
#include "../../Caches.h"
#include "../../Helpers.h"

class WEBGL_draw_buffersImpl {
public:
    WEBGL_draw_buffersImpl(rust::Box<WEBGL_draw_buffers> buffers);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, rust::Box<WEBGL_draw_buffers> buffers);

    static v8::Local<v8::Function> GetCtor(v8::Isolate *isolate);

    static void DrawBuffersWEBGL(const v8::FunctionCallbackInfo<v8::Value> &args);

private:
    rust::Box<WEBGL_draw_buffers> buffers_;

    static WEBGL_draw_buffersImpl *GetPointer(v8::Local<v8::Object> object);
};
