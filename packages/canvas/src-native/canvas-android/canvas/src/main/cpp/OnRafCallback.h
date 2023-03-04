//
// Created by Osei Fortune on 19/04/2022.
//

#pragma once

#include <cstdint>
#include "canvas-android/src/lib.rs.h"
#include "./canvas2d/CanvasRenderingContext2DImpl.h"
#include "./webgl/WebGLRenderingContextBase.h"

class OnRafCallback {
    // 0 2d
    // 1 gl
    // 2 gl2
public:
    OnRafCallback(intptr_t context, uint32_t version) : context_(context), version_(version) {}

    void OnFrame(int64_t ts) const {
        if (this->version_ == 0) {
            CanvasRenderingContext2DImpl::Flush(this->context_);
        }

        if (this->version_ == 1 || this->version_ == 2) {
            WebGLRenderingContextBase::Flush(this->context_);
        }

    }

private:
    intptr_t context_;
    uint32_t version_;
};

void OnRafCallbackOnFrame(intptr_t callback, int64_t ts) {
    auto ptr = reinterpret_cast<OnRafCallback *>(reinterpret_cast<intptr_t *>(callback));
    ptr->OnFrame(ts);
}
