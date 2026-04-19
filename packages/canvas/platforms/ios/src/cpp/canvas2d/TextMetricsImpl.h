//
// Created by Osei Fortune on 18/04/2022.
//

#pragma once

#include <vector>
#include "Helpers.h"
#include "ObjectWrapperImpl.h"


struct TextMetricsData {
    float width                  = 0.f;
    float actualBoundingBoxLeft  = 0.f;
    float actualBoundingBoxRight = 0.f;
    float actualBoundingBoxAscent  = 0.f;
    float actualBoundingBoxDescent = 0.f;
    float fontBoundingBoxAscent  = 0.f;
    float fontBoundingBoxDescent = 0.f;
    float emHeightAscent         = 0.f;
    float emHeightDescent        = 0.f;
    float hangingBaseline        = 0.f;
    float alphabeticBaseline     = 0.f;
    float ideographicBaseline    = 0.f;
};

class TextMetricsImpl: public ObjectWrapperImpl {
public:
    explicit TextMetricsImpl(TextMetrics* metrics);

    explicit TextMetricsImpl(const TextMetricsData& data) : data_(data) {}

    ~TextMetricsImpl() = default;

    const TextMetricsData& GetData() const { return data_; }

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static TextMetricsImpl *GetPointer(v8::Local<v8::Object> object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void GetWidth(v8::Local<v8::Name> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetActualBoundingBoxLeft(v8::Local<v8::Name> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetActualBoundingBoxRight(v8::Local<v8::Name> name,
                                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetActualBoundingBoxAscent(v8::Local<v8::Name> name,
                                          const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetActualBoundingBoxDescent(v8::Local<v8::Name> name,
                                           const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetFontBoundingBoxAscent(v8::Local<v8::Name> name,
                                           const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetFontBoundingBoxDescent(v8::Local<v8::Name> name,
                                            const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetEmHeightAscent(v8::Local<v8::Name> name,
                                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetEmHeightDescent(v8::Local<v8::Name> name,
                                          const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetHangingBaseline(v8::Local<v8::Name> name,
                                   const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetAlphabeticBaseline(v8::Local<v8::Name> name,
                                   const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetIdeographicBaseline(v8::Local<v8::Name> name,
                                   const v8::PropertyCallbackInfo<v8::Value> &info);

private:
    TextMetricsData data_;
};
