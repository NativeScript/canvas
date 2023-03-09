//
// Created by Osei Fortune on 20/04/2022.
//

#pragma once
#pragma process_pending_includes

#include "rust/cxx.h"
#include "canvas-cxx/src/lib.rs.h"
#include <cstdint>
#include "OnRafCallback.h"

using namespace org::nativescript::canvas;

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
