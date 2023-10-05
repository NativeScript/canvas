//
// Created by Osei Fortune on 30/09/2023.
//

#include "OneByteStringResource.h"

OneByteStringResource::OneByteStringResource(rust::String string) : string_(std::move(string)) {

}

const char *OneByteStringResource::data() const {
    return this->string_.data();
}

size_t OneByteStringResource::length() const {
    return this->string_.size();
}


