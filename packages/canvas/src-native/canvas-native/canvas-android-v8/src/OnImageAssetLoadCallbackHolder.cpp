//
// Created by Osei Fortune on 31/03/2022.
//

#include "OnImageAssetLoadCallbackHolder.h"

OnImageAssetLoadCallbackHolder::OnImageAssetLoadCallbackHolder(v8::Isolate *isolate,
                                                               v8::Local<v8::Context> context,
                                                               v8::Local<v8::Promise::Resolver> resolver) : isolate_(
        isolate),
                                                                                                             context_(
                                                                                                                     isolate,
                                                                                                                     context),
                                                                                                             resolver_(
                                                                                                                     isolate,
                                                                                                                     resolver) {

}


void OnImageAssetLoadCallbackHolder::complete(bool done) const {
    auto isolate = this->isolate_;
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto promise = resolver_.Get(isolate);
    auto context = context_.Get(isolate);
    v8::Context::Scope context_scope(context);
    v8::MicrotasksScope microtasksScope(isolate,
                                        v8::MicrotasksScope::kRunMicrotasks);
    promise->Resolve(context, v8::Boolean::New(isolate, done));
}

OnImageAssetLoadCallbackHolder::~OnImageAssetLoadCallbackHolder() {}
