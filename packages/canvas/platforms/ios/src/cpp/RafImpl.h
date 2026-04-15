//
// Created by Osei Fortune on 20/04/2022.
//

#pragma once
#pragma process_pending_includes

#include <cstdint>
#include "OnRafCallback.h"
#include "Common.h"

class RafImpl {
public:
    RafImpl(OnRafCallback *rafCallback, intptr_t callback, Raf* raf);

    ~RafImpl(){
        if (this->raf_ != nullptr) {
            // stop the raf, wait for any in-flight callback to finish, and clear the callback
            canvas_native_raf_stop_and_clear(this->GetRaf(), 100);
            canvas_native_raf_release(this->GetRaf());
            this->raf_ = nullptr;
        }
        delete rafCallback_;
    }

    Raf *GetRaf() {
        return this->raf_;
    }

private:
    OnRafCallback *rafCallback_;
    intptr_t callback_;
    Raf* raf_;
};
