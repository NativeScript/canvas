//
// Created by Osei Fortune on 10/06/2022.
//

#include "Caches.h"

std::shared_ptr <ConcurrentMap<v8::Isolate *, std::shared_ptr < Caches>>>
Caches::perIsolateCaches_ = std::make_shared < ConcurrentMap<v8::Isolate *, std::shared_ptr < Caches>>
>();

Caches::Caches(v8::Isolate *isolate) : isolate_(isolate) {}

Caches::~Caches() {}

std::shared_ptr<Caches> Caches::Get(v8::Isolate *isolate) {
    thread_local v8::Isolate* cached_isolate = nullptr;
    thread_local std::weak_ptr<Caches> cached_entry;

    if (cached_isolate == isolate) {
        if (auto cache = cached_entry.lock()) {
            return cache;
        }
    }

    auto cache = Caches::perIsolateCaches_->GetOrCreate(
        isolate, [isolate]{ return std::make_shared<Caches>(isolate); });

    cached_isolate = isolate;
    cached_entry   = cache;
    return cache;
}

void Caches::Remove(v8::Isolate *isolate) {
    Caches::perIsolateCaches_->Remove(isolate);
}

void Caches::SetContext(v8::Local<v8::Context> context) {
    this->context_ = std::make_shared<v8::Persistent<v8::Context >>(this->isolate_, context);
}

v8::Local<v8::Context> Caches::GetContext() {
    return this->context_->Get(this->isolate_);
}
