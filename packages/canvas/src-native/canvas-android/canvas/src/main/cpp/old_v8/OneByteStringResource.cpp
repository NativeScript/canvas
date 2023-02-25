//
// Created by Osei Fortune on 10/07/2022.
//

#include "OneByteStringResource.h"

OneByteStringResource::OneByteStringResource() :
        data_(std::move(rust::Vec<uint8_t>())) {
}

OneByteStringResource::OneByteStringResource(rust::Vec<uint8_t> data) :
        data_(std::move(data)) {
}

OneByteStringResource::~OneByteStringResource() {}

const char *OneByteStringResource::data() const {
    return (char *) this->data_.data();
}

size_t OneByteStringResource::length() const {
    return this->data_.size();
}