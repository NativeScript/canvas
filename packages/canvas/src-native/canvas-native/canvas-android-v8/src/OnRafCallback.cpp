//
// Created by Osei Fortune on 19/04/2022.
//

#include "OnRafCallback.h"
#include "canvas-android-v8/src/bridges/context.rs.h"
#include "./canvas2d/CanvasRenderingContext2DImpl.h"

OnRafCallback::OnRafCallback(intptr_t context) : context_(context) {}

void OnRafCallback::OnFrame(int64_t ts) const {
    CanvasRenderingContext2DImpl::Flush(this->context_);
}

void OnRafCallbackOnFrame(intptr_t callback, int64_t ts) {
    auto ptr = reinterpret_cast<OnRafCallback *>(reinterpret_cast<intptr_t *>(callback));
    ptr->OnFrame(ts);
}
