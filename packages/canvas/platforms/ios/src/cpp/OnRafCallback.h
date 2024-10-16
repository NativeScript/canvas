//
// Created by Osei Fortune on 19/04/2022.
//

#pragma once
#pragma process_pending_includes

#include <cstdint>

class OnRafCallback {
    // 0 2d
    // 1 gl
    // 2 gl2
public:
    OnRafCallback(intptr_t context, uint32_t version);

    void OnFrame(int64_t ts) const;

private:
    intptr_t context_;
    uint32_t version_;
};

void OnRafCallbackOnFrame(intptr_t callback, int64_t ts);
