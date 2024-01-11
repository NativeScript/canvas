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
        INSTANT_TYPE_PERSISTENT->Reset();
    }

    v8::Persistent<v8::String> *INSTANT_TYPE_PERSISTENT;
};

#endif /* PerIsolateData_h */
