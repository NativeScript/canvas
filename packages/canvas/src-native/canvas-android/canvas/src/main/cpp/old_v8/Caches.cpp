//
// Created by Osei Fortune on 22/03/2022.
//

#include "Caches.h"

std::shared_ptr<ConcurrentMap<v8::Isolate *, std::shared_ptr<Caches>>>
        Caches::perIsolateCaches_ = std::make_shared<ConcurrentMap<v8::Isolate *, std::shared_ptr<Caches>>
>();

Caches::Caches(v8::Isolate *isolate) : isolate_(isolate), taskId_(0) {}

Caches::~Caches() {
    context_->Reset();
    ALooper_removeFd(looper_, assetFD_[0]);
    close(assetFD_[0]);
    ALooper_release(looper_);
}

std::shared_ptr<Caches> Caches::Get(v8::Isolate *isolate) {
    std::shared_ptr<Caches> cache = Caches::perIsolateCaches_->Get(isolate);
    if (cache == nullptr) {
        cache = std::make_shared<Caches>(isolate);

        pipe2(cache->assetFD_, O_NONBLOCK | O_CLOEXEC);

        auto looper = ALooper_prepare(0);
        assert(looper != nullptr);
        ALooper_acquire(looper);

        ALooper_addFd(looper, cache->assetFD_[0], ALOOPER_POLL_CALLBACK, ALOOPER_EVENT_INPUT,
                      HandleAssetLoopCallback, static_cast<void*>(new std::shared_ptr<Caches>(cache)));

        cache->looper_ = looper;

        Caches::perIsolateCaches_->Insert(isolate, cache);
    }

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

void Caches::SetPerformingMicrotaskCheckpoint(bool value) {
    this->performingMicrotaskCheckpoint_ = value;
}

bool Caches::GetPerformingMicrotaskCheckpoint() const {
    return this->performingMicrotaskCheckpoint_;
}

int Caches::HandleAssetLoopCallback(int fd, int events, void *data) {

    struct Task task;

    read(fd, &task, sizeof(Task));

    auto thiz = *reinterpret_cast<std::shared_ptr<Caches>*>(data);

    if (thiz == nullptr) {
        return 0;
    }
    v8::Isolate* isolate = thiz->isolate_;
    if (isolate == nullptr || isolate->IsDead()) {
        return 0;
    }

    if (task.type_ == 1) {
        auto it = thiz->OnImageAssetLoadCallbackHolder_->Get(task.id_);
        if (it != nullptr) {
            it->done_ = task.done_;
            it->complete();
            thiz->OnImageAssetLoadCallbackHolder_->Remove(task.id_);
        }
    } else if (task.type_ == 2) {
        auto it = thiz->OnImageBitmapLoadCallbackHolder_->Get(task.id_);
        if (it != nullptr) {
            it->complete(task.done_);
            thiz->OnImageBitmapLoadCallbackHolder_->Remove(task.id_);
        }
    }


    return 1;
}
