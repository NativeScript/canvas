//
// Created by Osei Fortune on 17/03/2023.
//

#pragma once

#include <android/looper.h>
#include <unistd.h>
#include <fcntl.h>
#import "v8runtime/V8Runtime.h"

using namespace facebook;

struct JSIReadFileCallback {
    int fd_[2];
    ALooper *looper_;
    std::shared_ptr<jsi::Value> value_;
    std::shared_ptr<jsi::Value> data_;

    JSIReadFileCallback(std::shared_ptr<jsi::Value> value) : value_(std::move(value)) {
        auto res = pipe(fd_);
        assert(res != -1);
        res = fcntl(fd_[1], F_SETFL, O_NONBLOCK);
        assert(res != -1);
        looper_ = ALooper_prepare(0);
        ALooper_acquire(looper_);
    }

    ~JSIReadFileCallback() {
        ALooper_removeFd(looper_, fd_[0]);
        close(fd_[0]);
        ALooper_release(looper_);
    }
};

