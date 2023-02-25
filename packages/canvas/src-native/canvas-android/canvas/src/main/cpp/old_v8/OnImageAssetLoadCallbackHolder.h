//
// Created by Osei Fortune on 31/03/2022.
//

#pragma once

#include "Common.h"
#include "ObjectCacheEntry.h"
#include "Helpers.h"

class OnImageAssetLoadCallbackHolder {
public:
    OnImageAssetLoadCallbackHolder(v8::Isolate *isolate, const v8::Local<v8::Context> &context,
                                   const v8::Local<v8::Function> &callback);

    ~OnImageAssetLoadCallbackHolder();

    void complete() const;

    v8::Isolate *GetIsolate();

    struct Data {
        intptr_t callback;
        bool done;
        v8::Isolate *isolate_;
    };

    bool done_ = false;

    v8::Local<v8::Context> GetContext() {
        return this->context_.Get(this->isolate_);
    }

    v8::Local<v8::Function> GetCallback() {
        return this->callback_.Get(this->isolate_);
    }


private:
    v8::Persistent<v8::Function> callback_;
    v8::Isolate *isolate_;
    v8::Persistent<v8::Context> context_;
};
