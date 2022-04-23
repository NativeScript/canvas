//
// Created by Osei Fortune on 31/03/2022.
//

#pragma once

#include "Common.h"
#include "Helpers.h"

class OnImageAssetLoadCallbackHolder {
public:
    OnImageAssetLoadCallbackHolder(v8::Isolate *isolate, v8::Local<v8::Context> context,
                                   v8::Local<v8::Promise::Resolver> callback);

    ~OnImageAssetLoadCallbackHolder();

    void complete(bool done, intptr_t callback) const;

    v8::Isolate * GetIsolate();
    struct Data {
        intptr_t callback;
        bool done;
        v8::Isolate *isolate_;
    };

private:
    v8::Global<v8::Promise::Resolver> callback_;
    v8::Isolate *isolate_;
    v8::Global<v8::Context> context_;
};


void OnImageAssetLoadCallbackHolderComplete(intptr_t callback, bool  done);