//
//  PerIsolateData.cpp
//
//  Created by Osei Fortune on 10/01/2024.
//  Copyright © 2024 NativeScript. All rights reserved.
//

#include "PerIsolateData.h"

PerIsolateData::PerIsolateData(v8::Isolate * isolate){
    auto str = v8::String::NewFromUtf8(isolate, "__type").ToLocalChecked();
    INSTANT_TYPE_PERSISTENT = new v8::Persistent<v8::String>(isolate, str);
}
