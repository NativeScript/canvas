//
//  ObjectWrapperImpl.h
//
//  Created by Osei Fortune on 10/01/2024.
//  Copyright © 2024 NativeScript. All rights reserved.
//

#ifndef ObjectWrapperImpl_h
#define ObjectWrapperImpl_h

#include "Common.h"
#include "NativeType.h"

class ObjectWrapperImpl {
public:
    NativeType type_ = NativeType::None;

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


    /*
    static void SetNativeType(const v8::Local<v8::Object> &obj, NativeType type) {
        if (!obj.IsEmpty() && !obj->IsNullOrUndefined() && obj->IsObject() &&
            obj.As<v8::Object>()->InternalFieldCount() > 1) {
            auto wrapper = obj.As<v8::Object>()->GetAlignedPointerFromInternalField(0);
            if (wrapper != nullptr) {
                ((ObjectWrapperImpl *) wrapper)->type_ = type;
            }
        }
    }
     */

    inline static NativeType GetNativeType(const v8::Local<v8::Value> &obj) {
        if (!obj.IsEmpty() && !obj->IsNullOrUndefined() && obj->IsObject() &&
            obj.As<v8::Object>()->InternalFieldCount() > 1) {
            auto info = obj.As<v8::Object>()->GetAlignedPointerFromInternalField(0);

            if (info != nullptr) {
                auto value = static_cast<ObjectWrapperImpl *>(info);
                return value->type_;
            }
        }

        return NativeType::None;
    }

    static void SetNativeType(ObjectWrapperImpl *obj, NativeType type) {
        if (obj != nullptr) {
            obj->type_ = type;
        }
    }

    static NativeType GetNativeType(const ObjectWrapperImpl *obj) {
        if (obj != nullptr) {
            return obj->type_;
        }
        return NativeType::None;
    }

    static void SetNativeType(ObjectWrapperImpl &obj, NativeType type) {
        obj.type_ = type;
    }

    static NativeType GetNativeType(const ObjectWrapperImpl &obj) {
        return obj.type_;
    }

private:
    v8::Global<v8::Object> weakHandle_;

};

#endif /* ObjectWrapperImpl_h */
