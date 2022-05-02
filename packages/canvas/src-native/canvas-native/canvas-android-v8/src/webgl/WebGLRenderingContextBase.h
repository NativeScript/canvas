//
// Created by Osei Fortune on 30/04/2022.
//

#pragma once

#include "../Common.h"
#include "../Helpers.h"
#include "../Caches.h"
#include "../RafImpl.h"


class WebGLRenderingContextBase {
public:
    WebGLRenderingContextBase(rust::Box<WebGLState> state);

    void UpdateInvalidateState();

    InvalidateState GetInvalidateState() const;

    void SetInvalidateState(InvalidateState state);

    void Flush();

    static void Flush(intptr_t context);

    WebGLState& GetPointer();

    void SetRaf(std::shared_ptr <RafImpl> raf);
protected:
    rust::Box<WebGLState> state_;

    InvalidateState invalidateState_ = InvalidateState::NONE;

    std::shared_ptr <RafImpl> raf_;
};

