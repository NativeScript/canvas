//
// Created by Osei Fortune on 31/03/2022.
//

#include "OnImageAssetLoadCallbackHolder.h"
#include "Caches.h"
#include "canvas-android/src/lib.rs.h"

OnImageAssetLoadCallbackHolder::OnImageAssetLoadCallbackHolder(v8::Isolate *isolate,
                                                               const v8::Local<v8::Context> &context,
                                                               const v8::Local<v8::Function> &callback)
        : isolate_(
        isolate),
          context_(
                  isolate,
                  context),
          callback_(
                  isolate,
                  callback) {

}


void OnImageAssetLoadCallbackHolder::complete() const {
    auto isolate = this->isolate_;
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);


    auto context = this->context_.Get(isolate);
    v8::Context::Scope context_scope(context);


    auto func = this->callback_.Get(isolate);

    v8::Local<v8::Value> args[1] = {v8::Boolean::New(isolate, this->done_)};

    func->Call(context, context->Global(), 0, nullptr);

}

OnImageAssetLoadCallbackHolder::~OnImageAssetLoadCallbackHolder() {
    this->callback_.Reset();
    this->context_.Reset();
}

v8::Isolate *OnImageAssetLoadCallbackHolder::GetIsolate() {
    return this->isolate_;
}
