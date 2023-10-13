//
// Created by Osei Fortune on 30/09/2023.
//

#pragma once
#include "rust/cxx.h"
#include "Common.h"
#include "canvas-cxx/src/lib.rs.h"

class OneByteStringResource : public v8::String::ExternalOneByteStringResource {
public:
    OneByteStringResource(rust::String string);

    const char* data() const override;
    size_t length() const override;

private:
    rust::String string_;
};

