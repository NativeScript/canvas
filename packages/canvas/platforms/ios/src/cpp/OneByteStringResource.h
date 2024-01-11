//
// Created by Osei Fortune on 30/09/2023.
//

#pragma once

#include "Common.h"

class OneByteStringResource : public v8::String::ExternalOneByteStringResource {
public:
    OneByteStringResource(char *string);

    OneByteStringResource(U8Buffer *buffer);
    
    OneByteStringResource(CCow *cow);
    
    OneByteStringResource(std::string string);

    ~OneByteStringResource();

    const char *data() const override;

    size_t length() const override;

private:
    const char *string_;
    size_t length_;
    U8Buffer *buffer_ = nullptr;
    CCow *cow_ = nullptr;
    std::string stdString_;
    // todo enum
    bool usingBuffer_ = false;
    bool usingCow_ = false;
    bool usingString = false;
};

