//
// Created by Osei Fortune on 10/07/2022.
//

#ifndef CANVAS_NATIVE_ONEBYTESTRINGRESOURCE_H
#define CANVAS_NATIVE_ONEBYTESTRINGRESOURCE_H


#include "Common.h"

class OneByteStringResource : public v8::String::ExternalOneByteStringResource {
public:
    OneByteStringResource();
    OneByteStringResource(rust::Vec<uint8_t> data);
    ~OneByteStringResource() override;
    const char* data() const override;
    size_t length() const override;
private:
    rust::Vec<uint8_t> data_;
};


#endif //CANVAS_NATIVE_ONEBYTESTRINGRESOURCE_H
