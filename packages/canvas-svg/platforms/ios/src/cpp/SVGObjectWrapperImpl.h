#ifndef SVGObjectWrapperImpl_h
#define SVGObjectWrapperImpl_h

#include "SVGCommon.h"
#include "SVGNativeType.h"

class SVGObjectWrapperImpl {
public:
    SVGNativeType type_ = SVGNativeType::None;

    static constexpr v8::EmbedderDataTypeTag kInternalFieldTag = v8::kEmbedderDataTypeTagDefault;

    virtual ~SVGObjectWrapperImpl() = default;

    static void Finalizer(const v8::WeakCallbackInfo<SVGObjectWrapperImpl> &data) {
        auto *pThis = data.GetParameter();
        pThis->weakHandle_.Reset();
        delete pThis;
    }

    void BindFinalizer(v8::Isolate *isolate, const v8::Local<v8::Object> &object) {
        v8::HandleScope scopedHandle(isolate);
        weakHandle_.Reset(isolate, object);
        weakHandle_.SetWeak(this, Finalizer, v8::WeakCallbackType::kParameter);
    }

    inline static SVGNativeType GetNativeType(const v8::Local<v8::Value> &obj) {
        if (!obj.IsEmpty() && !obj->IsNullOrUndefined() && obj->IsObject() &&
            obj.As<v8::Object>()->InternalFieldCount() > 1) {
            auto info = obj.As<v8::Object>()->GetAlignedPointerFromInternalField(0, kInternalFieldTag);
            if (info != nullptr) {
                return static_cast<SVGObjectWrapperImpl *>(info)->type_;
            }
        }
        return SVGNativeType::None;
    }

    static void SetNativeType(SVGObjectWrapperImpl *obj, SVGNativeType type) {
        if (obj != nullptr) {
            obj->type_ = type;
        }
    }

private:
    v8::Global<v8::Object> weakHandle_;
};

#endif /* SVGObjectWrapperImpl_h */
