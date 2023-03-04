//
// Created by Osei Fortune on 20/04/2022.
//

#ifndef CANVAS_NATIVE_RAFIMPL_H
#define CANVAS_NATIVE_RAFIMPL_H

#include "OnRafCallback.h"
#include "canvas-android/src/lib.rs.h"

class RafImpl {
public:
    RafImpl(OnRafCallback *rafCallback, intptr_t callback, rust::Box<Raf> raf)
            : rafCallback_(rafCallback), callback_(callback), raf_(std::move(raf)) {}

    Raf &GetRaf() {
        return *this->raf_;
    }

private:
    OnRafCallback *rafCallback_;
    intptr_t callback_;
    rust::Box<::Raf> raf_;
};


#endif //CANVAS_NATIVE_RAFIMPL_H
