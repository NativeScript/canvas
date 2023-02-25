//
// Created by Osei Fortune on 31/03/2022.
//

#include "OnImageBitmapLoadCallbackHolder.h"
#include "Caches.h"
#include "canvas-android/src/lib.rs.h"
#include "ImageBitmapImpl.h"

OnImageBitmapLoadCallbackHolder::OnImageBitmapLoadCallbackHolder(v8::Isolate *isolate,
                                                                 const v8::Local<v8::Context> &context,
                                                                 const v8::Local<v8::Function> &callback,
                                                                 void *asset)
        : isolate_(
        isolate),
          context_(
                  isolate,
                  context),
          callback_(
                  isolate,
                  callback),
          asset_(asset) {

}


void OnImageBitmapLoadCallbackHolder::complete(bool done) const {
    auto isolate = this->isolate_;
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto context = this->context_.Get(isolate);
    v8::Context::Scope context_scope(context);

    auto func = this->callback_.Get(isolate);

    auto asset = rust::Box<ImageAsset>::from_raw((ImageAsset *)
                                                         this->asset_);

    if (!done) {
        v8::Local<v8::Value> args[2] = {Helpers::ConvertToV8String(isolate, "Failed to load image"),
                                        v8::Null(isolate).As<v8::Value>()};
        func->Call(context, context->Global(), 2, args);
    } else {
        auto ret = ImageBitmapImpl::NewInstance(isolate, std::move(asset));

        v8::Local<v8::Value> args[2] = {v8::Null(isolate), ret.As<v8::Value>()};
        func->Call(context, context->Global(), 2, args);
    }
}

OnImageBitmapLoadCallbackHolder::~OnImageBitmapLoadCallbackHolder() {
    this->callback_.Reset();
    this->context_.Reset();
    this->asset_ = nullptr;
}

v8::Isolate *OnImageBitmapLoadCallbackHolder::GetIsolate() {
    return this->isolate_;
}
