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
struct JSIReadFileCallback {
    v8::Isolate *isolate_;
    v8::Global<v8::Function> callback_;
    U8Buffer* data_;
    char* error_;
    std::mutex lock_;

    JSIReadFileCallback(v8::Isolate *isolate, v8::Local<v8::Function> callback) : isolate_(isolate),
                                                                                  callback_(
                                                                                          v8::Global<v8::Function>(
                                                                                                  isolate,
                                                                                                  callback)) {}

    void SetData(U8Buffer* data) {
        std::lock_guard<std::mutex> lock(lock_);
        data_ = data;
    }

    void SetError(char* error) {
        std::lock_guard<std::mutex> lock(lock_);
        error_ = error;
    }

    ~JSIReadFileCallback() {
        callback_.Reset();
        canvas_native_u8_buffer_release(data_);
        data_ = nullptr;

        canvas_native_string_destroy(error_);
        error_ = nullptr;
    }
};

#endif

#ifdef __ANDROID__
struct JSIReadFileCallback {
    int fd_[2];
    ALooper *looper_;
    v8::Isolate *isolate_;
    v8::Global<v8::Function> callback_;
    U8Buffer* data_;
    char* error_;
    std::mutex lock_;

    JSIReadFileCallback(v8::Isolate *isolate, v8::Local<v8::Function> callback) : isolate_(isolate),
                                                                                  callback_(
                                                                                          v8::Global<v8::Function>(
                                                                                                  isolate,
                                                                                                  callback)) {
        auto res = pipe(fd_);
        assert(res != -1);
        res = fcntl(fd_[1], F_SETFL, O_NONBLOCK);
        assert(res != -1);
        looper_ = ALooper_prepare(0);
        ALooper_acquire(looper_);
    }

    void SetData(U8Buffer* data) {
        std::lock_guard<std::mutex> lock(lock_);
        data_ = data;
    }

    void SetError(char* error) {
        std::lock_guard<std::mutex> lock(lock_);
        error_ = error;
    }

    ~JSIReadFileCallback() {
        ALooper_removeFd(looper_, fd_[0]);
        close(fd_[0]);
        ALooper_release(looper_);
        callback_.Reset();
        canvas_native_u8_buffer_release(data_);
        data_ = nullptr;

        canvas_native_string_destroy(error_);
        error_ = nullptr;
    }
};
#endif
