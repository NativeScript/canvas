//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once

#include "Common.h"
#include "ConcurrentMap.h"
#include "OnImageAssetLoadCallbackHolder.h"
#include "TextEncoderImplEntry.h"

class Caches {
public:
    Caches(v8::Isolate *isolate);

    ~Caches();

    static std::shared_ptr <Caches> Get(v8::Isolate *isolate);

    static void Remove(v8::Isolate *isolate);

    void SetContext(v8::Local<v8::Context> context);

    v8::Local<v8::Context> GetContext();

    std::unique_ptr <v8::Persistent<v8::Function>> TextMetricsTmpl = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> CanvasRenderingContext2DCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);
    std::unique_ptr <v8::Persistent<v8::Function>> CanvasGradientCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);
    std::unique_ptr <v8::Persistent<v8::Function>> CanvasPatternCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);
    std::unique_ptr <v8::Persistent<v8::Function>> ImageDataCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);
    std::unique_ptr <v8::Persistent<v8::Function>> Path2DCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);
    std::unique_ptr <v8::Persistent<v8::Function>> ImageAssetCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);
    std::unique_ptr <v8::Persistent<v8::Function>> MatrixCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::shared_ptr<ConcurrentMap<intptr_t, std::shared_ptr<OnImageAssetLoadCallbackHolder>>> OnImageAssetLoadCallbackHolder_ = std::make_shared<ConcurrentMap<intptr_t, std::shared_ptr<OnImageAssetLoadCallbackHolder>>>();

    std::unique_ptr <v8::Persistent<v8::Function>> TextEncoderCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::Function>> TextDecoderCtor = std::unique_ptr<v8::Persistent<v8::Function>>(
            nullptr);

    std::shared_ptr<ConcurrentMap<TextEncoderImplEntry *,
            std::shared_ptr<v8::Persistent<v8::Object>>>> TextEncoderData = std::make_shared<ConcurrentMap<TextEncoderImplEntry *,
            std::shared_ptr<v8::Persistent<v8::Object>>>>();
private:
    static std::shared_ptr <ConcurrentMap<v8::Isolate *,
            std::shared_ptr <Caches>>>
    perIsolateCaches_;
    v8::Isolate *isolate_;
    std::shared_ptr <v8::Persistent<v8::Context>> context_;
};
