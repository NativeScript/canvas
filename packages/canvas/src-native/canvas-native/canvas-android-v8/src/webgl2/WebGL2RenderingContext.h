//
// Created by Osei Fortune on 22/03/2022.
//

#ifndef CANVAS_NATIVE_WEBGL2RENDERINGCONTEXT_H
#define CANVAS_NATIVE_WEBGL2RENDERINGCONTEXT_H

#include "../Common.h"
#include "../Caches.h"
#include "../Helpers.h"
#include "../webgl/WebGLRenderingContext.h"

class WebGL2RenderingContext : WebGLRenderingContext {
public:
    static void Init(v8::Isolate *isolate);
};


#endif //CANVAS_NATIVE_WEBGL2RENDERINGCONTEXT_H
