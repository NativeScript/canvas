//
// Created by Osei Fortune on 28/04/2022.
//

#pragma once
#include "../../Common.h"
#include "../../Caches.h"
#include "../../Helpers.h"

class EXT_color_buffer_half_floatImpl {
public:
    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate);

private:
    static v8::Local<v8::Function> GetCtor(v8::Isolate *isolate);
};

