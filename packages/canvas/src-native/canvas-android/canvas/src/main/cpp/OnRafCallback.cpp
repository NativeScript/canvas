//
// Created by Osei Fortune on 19/04/2022.
//

#include "OnRafCallback.h"

OnRafCallback::OnRafCallback(intptr_t context, uint32_t version) : context_(context),
                                                                   version_(version) {}

void OnRafCallback::OnFrame(int64_t ts) const {
    if (this->version_ == 0) {
        CanvasRenderingContext2DImpl::Flush(this->context_);
    }

    if (this->version_ == 1 || this->version_ == 2) {
        WebGLRenderingContextBase::Flush(this->context_);
    }
}

void OnRafCallbackOnFrame(intptr_t callback, int64_t ts) {
    auto ptr = reinterpret_cast<OnRafCallback *>(reinterpret_cast<intptr_t *>(callback));
    ptr->OnFrame(ts);
}
