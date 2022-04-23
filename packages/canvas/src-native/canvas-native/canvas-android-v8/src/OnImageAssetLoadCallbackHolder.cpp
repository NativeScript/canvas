//
// Created by Osei Fortune on 31/03/2022.
//

#include "OnImageAssetLoadCallbackHolder.h"
#include "Caches.h"

OnImageAssetLoadCallbackHolder::OnImageAssetLoadCallbackHolder(v8::Isolate *isolate,
                                                               v8::Local<v8::Context> context,
                                                               v8::Local<v8::Promise::Resolver> callback)
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
    auto queueCallback = [](void* data){
        auto value = static_cast<Data*>(data);
        auto isolate = value->isolate_;
        v8::Locker locker(isolate);
        v8::Isolate::Scope isolate_scope(isolate);
        v8::HandleScope handle_scope(isolate);
        auto context = isolate->GetCurrentContext();
        auto ptr = reinterpret_cast<OnImageAssetLoadCallbackHolder *>(reinterpret_cast<intptr_t *>(value->callback));
        auto async_callback = ptr->callback_.Get(isolate);
        async_callback->Resolve(context, v8::Boolean::New(isolate, value->done));
        auto cache = Caches::Get(isolate);
        cache->OnImageAssetLoadCallbackHolder_->Remove(value->callback);
        delete value;
    };
    auto data = new Data{callback,done, isolate};
    isolate->EnqueueMicrotask(queueCallback,data);
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
