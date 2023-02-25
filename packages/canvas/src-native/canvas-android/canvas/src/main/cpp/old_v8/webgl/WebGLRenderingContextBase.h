//
// Created by Osei Fortune on 30/04/2022.
//

#pragma once

#include "../Common.h"
#include "../Helpers.h"
#include "../Caches.h"
#include "../RafImpl.h"


enum class WebGLRenderingVersion : uint8_t {
    V1,
    V2
};

class WebGLRenderingContextBase {
public:
    WebGLRenderingContextBase(rust::Box<WebGLState> state, WebGLRenderingVersion version);

    ~WebGLRenderingContextBase();

    static WebGLRenderingContextBase *GetPointerBase(const v8::Local<v8::Object> &object) {
        auto ptrValue = object->GetInternalField(0);

        if (ptrValue.IsEmpty()) {
            return nullptr;
        }

        void *ptr = ptrValue.As<v8::External>()->Value();

        if (ptr == nullptr) {
            return nullptr;
        }

        return (WebGLRenderingContextBase *) (ptr);
    }

    void UpdateInvalidateState();

    int GetInvalidateState() const;

    void SetInvalidateState(int state);

    void Flush();

    static void Flush(intptr_t context);

    WebGLState &GetState();

    void SetRaf(std::shared_ptr <RafImpl> raf);

    RafImpl *GetRaf();

    WebGLRenderingVersion GetVersion() const;


private:
    rust::Box<WebGLState> state_;

    WebGLRenderingVersion version_;

    int invalidateState_ = static_cast<int>(InvalidateState::NONE);

    std::shared_ptr <RafImpl> raf_ = nullptr;
};

