//
// Created by Osei Fortune on 05/08/2024.
//

#pragma once

#include "Common.h"
#include <thread>

#ifdef __ANDROID__

#include <android/looper.h>
#include <cassert>

#endif

#ifdef __APPLE__
#include "NSOperationQueueWrapper.h"
#endif


#include <unistd.h>
#include <fcntl.h>

typedef void(*CompleteCallback)(bool success, void *data);

#ifdef __APPLE__
struct PromiseCallback {
    struct Inner {
        NSOperationQueueWrapper* current_queue = nullptr;
        v8::Isolate *isolate_;
        v8::Persistent<v8::Promise::Resolver> callback_;
        v8::Persistent<v8::Value> error_;
        CompleteCallback completeCallback_;
        CompleteCallback completeCallbackWrapper_;
        bool isPrepared_ = false;
        void* data;

        Inner(v8::Isolate *isolate, v8::Local<v8::Promise::Resolver> callback,
              CompleteCallback completeCallback) : isolate_(isolate),
                                                   callback_(v8::Persistent<v8::Promise::Resolver>(
                                                           isolate,
                                                           callback)) {
            this->completeCallback_ = completeCallback;
            this->completeCallbackWrapper_ = [](bool success, void *data){
                if(data != nullptr){
                    auto* callback = static_cast<PromiseCallback*>(data);
                    auto inner = callback->inner_.get();
                    if(inner == nullptr || inner->current_queue != nullptr){
                        return;
                    }

                    inner->current_queue->addOperation([success, data, inner, callback](){
                        inner->completeCallback_(success, data);
                     //   delete callback;
                    });
                }
            };
        }

        void prepare(){
            current_queue = new NSOperationQueueWrapper(true);
            isPrepared_ = true;
        }

        void execute(bool complete, PromiseCallback* callback){
           completeCallbackWrapper_(complete, callback);
        }

        ~Inner(){
            callback_.Reset();
            error_.Reset();
        }
    };

    std::shared_ptr<Inner> inner_;

    void prepare(){
       auto inner = this->inner_.get();
       if(inner == nullptr){return;}
        inner->prepare();
    }

        void execute(bool complete) {
        auto inner = this->inner_.get();
        if (inner == nullptr) { return; }
        inner->execute(complete, this);
    }

    explicit PromiseCallback(std::shared_ptr<Inner> inner) : inner_(std::move(inner)) {}

    PromiseCallback(v8::Isolate *isolate, v8::Local<v8::Promise::Resolver> callback,
                    CompleteCallback completeCallback) : inner_(
            std::make_shared<Inner>(isolate, callback, completeCallback)) {}
};
#endif

#ifdef __ANDROID__

struct PromiseCallback {
    struct Inner {
        int fd_[2];
        ALooper *looper_ = nullptr;
        v8::Isolate *isolate_;
        v8::Persistent<v8::Promise::Resolver> callback_;
        void *data = nullptr;
        CompleteCallback completeCallback_;
        bool isPrepared_ = false;

        Inner(v8::Isolate *isolate, v8::Local<v8::Promise::Resolver> callback,
              CompleteCallback completeCallback) : isolate_(isolate),
                                                   callback_(v8::Persistent<v8::Promise::Resolver>(
                                                           isolate,
                                                           callback)) {
            this->completeCallback_ = completeCallback;
        }

        void prepare() {
            // release previous looper
            if (isPrepared_) {
                ALooper_removeFd(looper_, fd_[0]);
                close(fd_[0]);
                fd_[0] = -1;
                fd_[1] = -1;
                ALooper_release(looper_);
            }

            auto res = pipe(fd_);
            assert(res != -1);
            res = fcntl(fd_[1], F_SETFL, O_NONBLOCK);
            assert(res != -1);
            looper_ = ALooper_prepare(0);
            ALooper_acquire(looper_);

            isPrepared_ = true;
        }

        void execute(bool complete) const {
            if (!isPrepared_) { return; }
            write(fd_[1],
                  &complete,
                  sizeof(bool));
        }


        ~Inner() {
            if (!isPrepared_) {
                return;
            }
            ALooper_removeFd(looper_, fd_[0]);
            close(fd_[0]);
            ALooper_release(looper_);
            callback_.Reset();
        }
    };

    void prepare() const {
        auto inner = this->inner_.get();
        if (inner == nullptr) { return; }
        inner->prepare();
        auto looper = inner->looper_;
        auto fd = inner->fd_[0];
        auto data = new PromiseCallback(this->inner_);
        ALooper_addFd(looper,
                      fd,
                      ALOOPER_POLL_CALLBACK,
                      ALOOPER_EVENT_INPUT,
                      [](int fd, int events,
                         void *data) {
                          auto cb = static_cast<PromiseCallback *>(data);
                          bool done = false;
                          read(fd, &done,
                               sizeof(bool));
                          cb->inner_->completeCallback_(done, data);
                          return 0;
                      }, (void *) data);

        ALooper_wake(looper);
        inner->isPrepared_ = true;
    }

    void execute(bool complete) const {
        auto inner = this->inner_.get();
        if (inner == nullptr) { return; }
        inner->execute(complete);
    }

    std::shared_ptr<Inner> inner_;

    explicit PromiseCallback(std::shared_ptr<Inner> inner) : inner_(std::move(inner)) {}

    PromiseCallback(v8::Isolate *isolate, v8::Local<v8::Promise::Resolver> callback,
                    CompleteCallback completeCallback) : inner_(
            std::make_shared<Inner>(isolate, callback, completeCallback)) {}

};

#endif
