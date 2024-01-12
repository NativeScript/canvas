//
//  PerIsolateData.cpp
//
//  Created by Osei Fortune on 10/01/2024.
//  Copyright © 2024 NativeScript. All rights reserved.
//

#include "PerIsolateData.h"

PerIsolateData::PerIsolateData(v8::Isolate *isolate) {
    auto type = v8::String::NewFromUtf8(isolate, "__type").ToLocalChecked();
    INSTANCE_TYPE_PERSISTENT = new v8::Persistent<v8::String>(isolate, type);

    auto version = v8::String::NewFromUtf8(isolate, "version").ToLocalChecked();
    VERSION_PERSISTENT = new v8::Persistent<v8::String>(isolate, version);

    auto alpha = v8::String::NewFromUtf8(isolate, "alpha").ToLocalChecked();
    ALPHA_PERSISTENT = new v8::Persistent<v8::String>(isolate, alpha);

    auto antialias = v8::String::NewFromUtf8(isolate, "antialias").ToLocalChecked();
    ANTIALIAS_PERSISTENT = new v8::Persistent<v8::String>(isolate, antialias);

    auto failIfMajorPerformanceCaveat = v8::String::NewFromUtf8(isolate,
                                                                "failIfMajorPerformanceCaveat").ToLocalChecked();
    FAIL_IF_MAJOR_PERFORMANCE_CAVEAT_PERSISTENT = new v8::Persistent<v8::String>(isolate,
                                                                                 failIfMajorPerformanceCaveat);

    auto powerPreference = v8::String::NewFromUtf8(isolate, "powerPreference").ToLocalChecked();
    POWER_PREFERENCE_PERSISTENT = new v8::Persistent<v8::String>(isolate, powerPreference);

    auto premultipliedAlpha = v8::String::NewFromUtf8(isolate,
                                                      "premultipliedAlpha").ToLocalChecked();
    PREMULTIPLIED_ALPHA_PERSISTENT = new v8::Persistent<v8::String>(isolate, premultipliedAlpha);

    auto preserveDrawingBuffer = v8::String::NewFromUtf8(isolate,
                                                         "preserveDrawingBuffer").ToLocalChecked();
    PRESERVE_DRAWING_BUFFER_PERSISTENT = new v8::Persistent<v8::String>(isolate,
                                                                        preserveDrawingBuffer);

    auto stencil = v8::String::NewFromUtf8(isolate, "stencil").ToLocalChecked();
    STENCIL_PERSISTENT = new v8::Persistent<v8::String>(isolate, stencil);

    auto desynchronized = v8::String::NewFromUtf8(isolate, "desynchronized").ToLocalChecked();
    DESYNCHRONIZED_PERSISTENT = new v8::Persistent<v8::String>(isolate, desynchronized);

    auto xrCompatible = v8::String::NewFromUtf8(isolate, "xrCompatible").ToLocalChecked();
    XR_COMPATIBLE_PERSISTENT = new v8::Persistent<v8::String>(isolate, xrCompatible);

}
