//
// Created by Osei Fortune on 19/04/2022.
//

#pragma once
#include "Common.h"

class OnRafCallback {
public:
    OnRafCallback(intptr_t context);
    void OnFrame(int64_t ts) const;
private:
    intptr_t context_;
};

void OnRafCallbackOnFrame(intptr_t callback, int64_t ts);