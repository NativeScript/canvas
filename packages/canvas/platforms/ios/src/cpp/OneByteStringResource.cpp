//
// Created by Osei Fortune on 30/09/2023.
//

#include "OneByteStringResource.h"

OneByteStringResource::OneByteStringResource(char* string) : string_(string),length_(strlen(string)) {

}

OneByteStringResource::~OneByteStringResource() {
    canvas_native_string_destroy(this->string_);
}

const char *OneByteStringResource::data() const {
    return this->string_;
}

size_t OneByteStringResource::length() const {
    return this->length_;
}


