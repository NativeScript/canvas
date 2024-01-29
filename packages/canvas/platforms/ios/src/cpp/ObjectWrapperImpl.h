//
//  ObjectWrapperImpl.h
//
//  Created by Osei Fortune on 10/01/2024.
//  Copyright © 2024 NativeScript. All rights reserved.
//

#ifndef ObjectWrapperImpl_h
#define ObjectWrapperImpl_h

#include "Common.h"

class ObjectWrapperImpl {
public:

    static void Finalizer(const v8::WeakCallbackInfo<ObjectWrapperImpl> &data) {
        auto *pThis = data.GetParameter();
        pThis->weakHandle_.Reset();
        delete pThis;
    }

    void BindFinalizer(v8::Isolate *isolate, const v8::Local<v8::Object> &object) {
        v8::HandleScope scopedHandle(isolate);
        weakHandle_.Reset(isolate, object);
        weakHandle_.SetWeak(this, Finalizer, v8::WeakCallbackType::kParameter);
    }

private:
    v8::Global<v8::Object> weakHandle_;

};

#endif /* ObjectWrapperImpl_h */
