//
// Created by Osei Fortune on 30/04/2022.
//

#pragma once
#pragma process_pending_includes

#include <vector>
#include <cstdint>
#include <memory>

#include "Helpers.h"
#include "RafImpl.h"
#include "OnRafCallback.h"
#include "Common.h"
#include "ObjectWrapperImpl.h"

enum class WebGLRenderingVersion : uint8_t {
    V1,
    V2
};

class WebGLRenderingContextBase: public ObjectWrapperImpl {
public:
    WebGLRenderingContextBase(WebGLState* state, WebGLRenderingVersion version);

    static WebGLRenderingContextBase *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<WebGLRenderingContextBase *>(ptr);
    }

    ~WebGLRenderingContextBase();

    void UpdateInvalidateState();

    int GetInvalidateState() const;

    void SetInvalidateState(int state);

    void SetInvalidateState(InvalidateState state);

    void Flush();

    static void Flush(intptr_t context);

    WebGLState* GetState();

    void SetRaf(std::shared_ptr<RafImpl> raf);

    RafImpl *GetRaf();

    WebGLRenderingVersion GetVersion() const;

    void StartRaf();

    void StopRaf();

private:
    WebGLState* state_;

    WebGLRenderingVersion version_;

    int invalidateState_ = static_cast<int>(InvalidateState::InvalidateStateNone);

    std::shared_ptr<RafImpl> raf_;
};

