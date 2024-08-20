//
// Created by Osei Fortune on 05/08/2024.
//

#pragma once

#include "Common.h"

#ifdef __ANDROID__

#include <android/looper.h>
#include <cassert>

#endif

#ifdef __APPLE__
#include "NSOperationQueueWrapper.h"
#endif

#include <unistd.h>
#include <fcntl.h>
#include <thread>

typedef void(*CompleteCallback)(bool success, void *data);


#ifdef __APPLE__
struct AsyncCallback {

    struct Inner {
        NSOperationQueueWrapper* current_queue = nullptr;
        v8::Isolate *isolate_;
        v8::Persistent<v8::Function> callback_;
        CompleteCallback completeCallback_;
        CompleteCallback completeCallbackWrapper_;
        bool isPrepared_ = false;
        void* data = nullptr;

        Inner(v8::Isolate *isolate, v8::Local<v8::Function> callback,
              CompleteCallback completeCallback) : isolate_(isolate),
                                                   callback_(v8::Persistent<v8::Function>(
                                                           isolate,
                                                           callback)) {
            this->completeCallback_ = completeCallback;

            this->completeCallbackWrapper_ = [](bool success, void *data){
                if(data != nullptr){
                    auto* callback = static_cast<AsyncCallback*>(data);
                    if(callback->inner_->current_queue == nullptr){
                        return;
                    }
                    
                    auto inner = std::shared_ptr<Inner>(callback->inner_);
                                        
                    std::thread thread([success, data, callback](std::shared_ptr<Inner> inner){
                        inner->current_queue = nullptr;
                        inner->completeCallback_(success, data);
                    }, std::move(inner));
                    
                    thread.detach();
                    
//                    callback->inner_->current_queue->addOperation([success, data, callback, inner_ptr](){
//                        inner_ptr->completeCallback_(success, data);
//                       // delete callback;
//                    });
                }
            };
        }

        void prepare(){
            current_queue = new NSOperationQueueWrapper(true);
            isPrepared_ = true;
        }

        void execute(bool complete, AsyncCallback* callback){
            completeCallbackWrapper_(complete, callback);
        }

        ~Inner() {
            callback_.Reset();
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


    explicit AsyncCallback(std::shared_ptr<Inner> inner) : inner_(std::move(inner)) {}


    AsyncCallback(v8::Isolate *isolate, v8::Local<v8::Function> callback,
                  CompleteCallback completeCallback) : inner_(
            std::make_shared<Inner>(isolate, callback, completeCallback)) {
    }


};
#endif

#ifdef __ANDROID__

struct AsyncCallback {

    struct Inner {
        int fd_[2];
        ALooper *looper_;
        v8::Isolate *isolate_;
        v8::Persistent<v8::Function> callback_;
        CompleteCallback completeCallback_;
        bool isPrepared_ = false;
        void* data;

        Inner(v8::Isolate *isolate, v8::Local<v8::Function> callback,
              CompleteCallback completeCallback) : isolate_(isolate),
                                                   callback_(v8::Persistent<v8::Function>(
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
            if(!isPrepared_){
                return;
            }
            ALooper_removeFd(looper_, fd_[0]);
            close(fd_[0]);
            ALooper_release(looper_);
            callback_.Reset();
        }
    };

    std::shared_ptr<Inner> inner_;

    void prepare() const {
        auto inner = this->inner_.get();
        if (inner == nullptr) { return; }
        inner->prepare();
        auto looper = inner->looper_;
        auto fd = inner->fd_[0];

        auto data = new AsyncCallback(this->inner_);
        ALooper_addFd(looper,
                      fd,
                      0,
                      ALOOPER_EVENT_INPUT,
                      [](int fd, int events,
                         void *data) {
                          auto cb = static_cast<AsyncCallback *>(data);
                          bool done = false;
                          read(fd, &done,
                               sizeof(bool));
                          cb->inner_->completeCallback_(done, data);
                          return 0;
                      }, (void *) data);
    }

    void execute(bool complete) const {
        auto inner = this->inner_.get();
        if (inner == nullptr) { return; }
        inner->execute(complete);
    }


    explicit AsyncCallback(std::shared_ptr<Inner> inner) : inner_(std::move(inner)) {}


    AsyncCallback(v8::Isolate *isolate, v8::Local<v8::Function> callback,
                  CompleteCallback completeCallback) : inner_(
            std::make_shared<Inner>(isolate, callback, completeCallback)) {
    }


};

#endif
