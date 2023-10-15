//
// Created by Osei Fortune on 30/09/2023.
//

#pragma once
#include "Common.h"

class OneByteStringResource : public v8::String::ExternalOneByteStringResource {
public:
    OneByteStringResource(char* string);
    
    ~OneByteStringResource();

    const char* data() const override;
    size_t length() const override;

private:
    const char* string_;
    size_t length_;
};

