//
// Created by Osei Fortune on 29/04/2022.
//

#pragma once

#include "../../Common.h"
#include "../../Caches.h"
#include "../../Helpers.h"

class OES_texture_floatImpl {
public:
    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate);

private:
    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);
};
