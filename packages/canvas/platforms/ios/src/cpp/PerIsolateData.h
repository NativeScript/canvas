//
//  PerIsolateData.h
//
//  Created by Osei Fortune on 10/01/2024.
//  Copyright © 2024 NativeScript. All rights reserved.
//

#ifndef PerIsolateData_h
#define PerIsolateData_h

#include "Common.h"

class PerIsolateData {
public:
    PerIsolateData(v8::Isolate *isolate);

    ~PerIsolateData() {
        INSTANCE_TYPE_PERSISTENT->Reset();
        VERSION_PERSISTENT->Reset();
        ALPHA_PERSISTENT->Reset();
        ANTIALIAS_PERSISTENT->Reset();
        FAIL_IF_MAJOR_PERFORMANCE_CAVEAT_PERSISTENT->Reset();
        POWER_PREFERENCE_PERSISTENT->Reset();
        PREMULTIPLIED_ALPHA_PERSISTENT->Reset();
        PRESERVE_DRAWING_BUFFER_PERSISTENT->Reset();
        STENCIL_PERSISTENT->Reset();
        DESYNCHRONIZED_PERSISTENT->Reset();
        XR_COMPATIBLE_PERSISTENT->Reset();
    }

    v8::Persistent<v8::String> *INSTANCE_TYPE_PERSISTENT;

    v8::Persistent<v8::String> *VERSION_PERSISTENT;

    v8::Persistent<v8::String> *ALPHA_PERSISTENT;

    v8::Persistent<v8::String> *ANTIALIAS_PERSISTENT;

    v8::Persistent<v8::String> *FAIL_IF_MAJOR_PERFORMANCE_CAVEAT_PERSISTENT;

    v8::Persistent<v8::String> *POWER_PREFERENCE_PERSISTENT;

    v8::Persistent<v8::String> *PREMULTIPLIED_ALPHA_PERSISTENT;

    v8::Persistent<v8::String> *PRESERVE_DRAWING_BUFFER_PERSISTENT;

    v8::Persistent<v8::String> *STENCIL_PERSISTENT;

    v8::Persistent<v8::String> *DESYNCHRONIZED_PERSISTENT;

    v8::Persistent<v8::String> *XR_COMPATIBLE_PERSISTENT;
};

#endif /* PerIsolateData_h */
