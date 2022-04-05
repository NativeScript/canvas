//
// Created by Osei Fortune on 22/03/2022.
//

#ifndef CANVAS_NATIVE_CACHES_H
#define CANVAS_NATIVE_CACHES_H

#include "Common.h"
#include "ConcurrentMap.h"
class Caches {
public:
    Caches(v8::Isolate *isolate);

    ~Caches();

    static std::shared_ptr <Caches> Get(v8::Isolate *isolate);

    static void Remove(v8::Isolate *isolate);

    void SetContext(v8::Local<v8::Context> context);

    v8::Local<v8::Context> GetContext();

    std::unique_ptr<v8::Persistent<v8::Function>> CanvasRenderingContext2DCtor = std::unique_ptr<v8::Persistent<v8::Function>>(nullptr);
    std::unique_ptr<v8::Persistent<v8::Function>> CanvasGradientCtor = std::unique_ptr<v8::Persistent<v8::Function>>(nullptr);
    std::unique_ptr<v8::Persistent<v8::Function>> CanvasPatternCtor = std::unique_ptr<v8::Persistent<v8::Function>>(nullptr);
    std::unique_ptr<v8::Persistent<v8::Function>> ImageDataCtor = std::unique_ptr<v8::Persistent<v8::Function>>(nullptr);
    std::unique_ptr<v8::Persistent<v8::Function>> Path2DCtor = std::unique_ptr<v8::Persistent<v8::Function>>(nullptr);
    std::unique_ptr<v8::Persistent<v8::Function>> ImageAssetCtor = std::unique_ptr<v8::Persistent<v8::Function>>(nullptr);
    std::unique_ptr<v8::Persistent<v8::Function>> MatrixCtor = std::unique_ptr<v8::Persistent<v8::Function>>(nullptr);

private:
    static std::shared_ptr<ConcurrentMap<v8::Isolate*, std::shared_ptr<Caches>>> perIsolateCaches_;
    v8::Isolate* isolate_;
    std::shared_ptr<v8::Persistent<v8::Context>> context_;
};

#endif //CANVAS_NATIVE_CACHES_H
