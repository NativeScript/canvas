//
// Created by Osei Fortune on 20/04/2022.
//

#pragma once

#include <cstdint>
#include "rust/cxx.h"
#include "OnRafCallback.h"
#include "canvas-android/src/lib.rs.h"

class RafImpl {
public:
    RafImpl(OnRafCallback *rafCallback, intptr_t callback, rust::Box<Raf> raf);

    Raf &GetRaf() {
        return *this->raf_;
    }

private:
    OnRafCallback *rafCallback_;
    intptr_t callback_;
    rust::Box<Raf> raf_;
};
