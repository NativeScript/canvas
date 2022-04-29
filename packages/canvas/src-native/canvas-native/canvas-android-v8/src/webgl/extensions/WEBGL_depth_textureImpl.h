//
// Created by Osei Fortune on 29/04/2022.
//

#pragma

#include "../../Common.h"
#include "../../Caches.h"
#include "../../Helpers.h"


class WEBGL_depth_textureImpl {
public:
    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate);

private:
    static v8::Local<v8::Function> GetCtor(v8::Isolate *isolate);
};
