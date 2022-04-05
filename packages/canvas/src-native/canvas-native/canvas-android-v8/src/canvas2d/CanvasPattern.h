//
// Created by Osei Fortune on 22/03/2022.
//

#ifndef CANVAS_NATIVE_CANVASPATTERN_H
#define CANVAS_NATIVE_CANVASPATTERN_H

#include "../Common.h"
#include "../Caches.h"
#include "../Helpers.h"

class CanvasPattern {
public:
    static void Init(v8::Isolate *isolate);

    static void Create(v8::Local <v8::Context> context, intptr_t pattern);

    static void CreateCallback(const v8::FunctionCallbackInfo <v8::Value> &args);

    static void SetTransform(const v8::FunctionCallbackInfo <v8::Value> &args);

private:
    static v8::Local <v8::Function> GetCtorFunc(v8::Isolate *isolate);
};


#endif //CANVAS_NATIVE_CANVASPATTERN_H
