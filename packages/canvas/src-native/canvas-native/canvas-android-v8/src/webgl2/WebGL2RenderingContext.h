//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once

#include "../Common.h"
#include "../Caches.h"
#include "../Helpers.h"
#include "../webgl/WebGLRenderingContext.h"
class WebGL2RenderingContext : WebGLRenderingContext {
public:
    static void Init(v8::Isolate *isolate);
};
