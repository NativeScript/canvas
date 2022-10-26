//
// Created by Osei Fortune on 31/03/2022.
//

#pragma once

#include "Common.h"
#include "ObjectCacheEntry.h"
#include "Helpers.h"

class OnImageBitmapLoadCallbackHolder {
public:
    OnImageBitmapLoadCallbackHolder(v8::Isolate *isolate, const v8::Local<v8::Context>& context,
                                    const v8::Local<v8::Function>& callback, void *asset);

    ~OnImageBitmapLoadCallbackHolder();

    void complete(bool done) const;

    v8::Isolate *GetIsolate();

    struct Data {
        intptr_t callback;
        bool done;
        v8::Isolate *isolate_;
    };

private:
    v8::Global<v8::Function> callback_;
    v8::Isolate *isolate_;
    v8::Global<v8::Context> context_;
    void *asset_;
};
