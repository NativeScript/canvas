//
// Created by Osei Fortune on 20/04/2022.
//

#include "RafImpl.h"

RafImpl::RafImpl(OnRafCallback *rafCallback, intptr_t callback, Raf* raf) : rafCallback_(
        rafCallback), callback_(callback), raf_(raf) {}
