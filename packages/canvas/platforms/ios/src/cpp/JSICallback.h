//
// Created by Osei Fortune on 10/03/2023.
//

#pragma once

#include "Common.h"
#ifdef __ANDROID__
#include <android/looper.h>
#endif
#include <unistd.h>
#include <fcntl.h>


#ifdef __APPLE__

struct JSICallback {
    v8::Isolate *isolate_;
    std::shared_ptr<v8::Persistent<v8::Function>> callback_;
    std::shared_ptr<v8::Persistent<v8::Value>> data_;

    JSICallback(v8::Isolate *isolate, v8::Local<v8::Function> callback,
                v8::Local<v8::Value> data = v8::Local<v8::Value>()) : isolate_(isolate),
                                                                             callback_(
                                                                                     std::make_shared<v8::Persistent<v8::Function>>(
                                                                                             isolate,
                                                                                             callback)),
                                                                             data_(std::make_shared<v8::Persistent<v8::Value>>(
                                                                                     isolate,
                                                                                     data)) {}
};


#endif


#ifdef __ANDROID__
struct JSICallback {
    int fd_[2];
    ALooper *looper_;
    v8::Isolate *isolate_;
    std::shared_ptr<v8::Persistent<v8::Function>> callback_;
    std::shared_ptr<v8::Persistent<v8::Value>> data_;

    JSICallback(v8::Isolate *isolate, v8::Local<v8::Function> callback,
                v8::Local<v8::Value> data = v8::Local<v8::Value>()) : isolate_(isolate),
                                                                             callback_(
                                                                                     std::make_shared<v8::Persistent<v8::Function>>(
                                                                                             isolate,
                                                                                             callback)),
                                                                             data_(std::make_shared<v8::Persistent<v8::Value>>(
                                                                                     isolate,
                                                                                     data)) {
        auto res = pipe(fd_);
        assert(res != -1);
        res = fcntl(fd_[1], F_SETFL, O_NONBLOCK);
        assert(res != -1);
        looper_ = ALooper_prepare(0);
        ALooper_acquire(looper_);
    }

    ~JSICallback() {
        ALooper_removeFd(looper_, fd_[0]);
        close(fd_[0]);
        ALooper_release(looper_);
        callback_ = nullptr;
        data_ = nullptr;
    }
};
#endif
