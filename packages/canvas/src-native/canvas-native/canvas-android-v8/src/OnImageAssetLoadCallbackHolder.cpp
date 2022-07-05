//
// Created by Osei Fortune on 31/03/2022.
//

#include "OnImageAssetLoadCallbackHolder.h"
#include "Caches.h"
#include "canvas-android-v8/src/bridges/context.rs.h"

OnImageAssetLoadCallbackHolder::OnImageAssetLoadCallbackHolder(v8::Isolate *isolate,
                                                               v8::Local<v8::Context> context,
                                                               v8::Local<v8::Function> callback)
        : isolate_(
        isolate),
          context_(
                  isolate,
                  context),
          callback_(
                  isolate,
                  callback) {

}


void OnImageAssetLoadCallbackHolder::complete(bool done, intptr_t callback) const {
    auto isolate = this->isolate_;
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto context = this->context_.Get(isolate);
    v8::Context::Scope context_scope(context);
    auto ptr = reinterpret_cast<OnImageAssetLoadCallbackHolder *>(reinterpret_cast<intptr_t *>(callback));
    auto func = ptr->callback_.Get(isolate);
    v8::Local<v8::Value> args[1] = {v8::Boolean::New(isolate, done)};
    Helpers::FunctionCall(func, context, context->Global(), 1, args);
    auto cache = Caches::Get(isolate);
    cache->OnImageAssetLoadCallbackHolder_->Remove(callback);
}

void OnImageAssetLoadCallbackHolderComplete(intptr_t callback, bool done) {
    auto ptr = reinterpret_cast<OnImageAssetLoadCallbackHolder *>(reinterpret_cast<intptr_t *>(callback));
    ptr->complete(done, callback);
}


OnImageAssetLoadCallbackHolder::~OnImageAssetLoadCallbackHolder() {
    this->callback_.Reset();
    this->context_.Reset();
}

v8::Isolate *OnImageAssetLoadCallbackHolder::GetIsolate() {
    return this->isolate_;
}
