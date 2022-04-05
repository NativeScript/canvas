//
// Created by Osei Fortune on 31/03/2022.
//

#ifndef CANVAS_NATIVE_ONIMAGEASSETLOADCALLBACKHOLDER_H
#define CANVAS_NATIVE_ONIMAGEASSETLOADCALLBACKHOLDER_H

#include "Common.h"
#include "Helpers.h"

class OnImageAssetLoadCallbackHolder {
public:
    OnImageAssetLoadCallbackHolder(v8::Isolate *isolate, v8::Local<v8::Context> context,
                                   v8::Local<v8::Promise::Resolver> resolver);

    ~OnImageAssetLoadCallbackHolder();

    void complete(bool done) const;

private:
    v8::Global<v8::Promise::Resolver> resolver_;
    v8::Isolate *isolate_;
    v8::Global<v8::Context> context_;
};

#endif //CANVAS_NATIVE_ONIMAGEASSETLOADCALLBACKHOLDER_H
