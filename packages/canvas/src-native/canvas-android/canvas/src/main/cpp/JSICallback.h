//
// Created by Osei Fortune on 10/03/2023.
//

#pragma once

#include "Common.h"
#include <android/looper.h>
#include <unistd.h>
#include <fcntl.h>


struct JSICallback {
    int fd_[2];
    ALooper *looper_;
    v8::Isolate *isolate_;
    v8::Global<v8::Function> callback_;
    v8::Global<v8::Value> data_;

    JSICallback(v8::Isolate *isolate, v8::Global<v8::Function> callback, v8::Global<v8::Value> data = v8::Global<v8::Value>() ) : isolate_(isolate),
                                                                           callback_(std::move(
                                                                                   callback)), data_(std::move(data)) {
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
        callback_.Reset();
    }
};
