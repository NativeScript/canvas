//
// Created by Osei Fortune on 19/04/2022.
//

#pragma once

#include "Common.h"

struct TextEncoderImplEntry {
    TextEncoderImplEntry(rust::Vec<uint8_t> data) : data_(std::move(data)){}

    void *GetData() {
        return reinterpret_cast<void *>(this->data_.data());
    }

    std::size_t GetSize() {
        return this->data_.size();
    }

private:
    rust::Vec <std::uint8_t> data_;
};
