//
// Created by Osei Fortune on 30/09/2023.
//

#include "OneByteStringResource.h"

OneByteStringResource::OneByteStringResource(char *string) : string_(string),
                                                             length_(strlen(string)) {

}

OneByteStringResource::OneByteStringResource(U8Buffer *buffer) : buffer_(buffer) {
    string_ = (char *) canvas_native_u8_buffer_get_bytes(buffer);
    length_ = canvas_native_u8_buffer_get_length(buffer);
    usingBuffer_ = true;
}

OneByteStringResource::OneByteStringResource(CCow *cow) : cow_(cow) {
    string_ = (char *) canvas_native_ccow_get_bytes(cow);
    length_ = canvas_native_ccow_get_length(cow);
    usingCow_ = true;
}

OneByteStringResource::OneByteStringResource(std::string string) : stdString_(std::move(string)) {
    this->usingString = true;
}

OneByteStringResource::~OneByteStringResource() {
    if (usingBuffer_) {
        canvas_native_u8_buffer_destroy(buffer_);
        this->string_ = nullptr;
        this->length_ = 0;
    }else if(usingCow_){
        canvas_native_ccow_destroy(cow_);
        this->string_ = nullptr;
        this->length_ = 0;
    }else if(usingString){
        // noop
    } else {
        canvas_native_string_destroy((char *) this->string_);
        this->string_ = nullptr;
        this->length_ = 0;
    }
}

const char *OneByteStringResource::data() const {
    return this->string_;
}

size_t OneByteStringResource::length() const {
    return this->length_;
}



