//
// Created by Osei Fortune on 22/03/2022.
//

#ifndef CANVAS_NATIVE_CANVASGRADIENT_H
#define CANVAS_NATIVE_CANVASGRADIENT_H

#include "../Common.h"
#include "../Helpers.h"
#include "../Caches.h"

class CanvasGradient {
public:
    static void Init(v8::Isolate *isolate);

    static v8::Local <v8::Object> Create(v8::Local <v8::Context>, PaintStyleValue *gradient);

    static void CreateCallback(const v8::FunctionCallbackInfo <v8::Value> &args);

    static void AddColorStop(const v8::FunctionCallbackInfo <v8::Value> &args);

private:
    static v8::Local <v8::Function> GetCtorFunc(v8::Isolate *isolate);
};


#endif //CANVAS_NATIVE_CANVASGRADIENT_H
