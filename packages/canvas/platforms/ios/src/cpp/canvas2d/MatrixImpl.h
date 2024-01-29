//
// Created by Osei Fortune on 03/04/2022.
//

#pragma once

#include <vector>
#include "Helpers.h"
#include "ObjectWrapperImpl.h"

class MatrixImpl: public ObjectWrapperImpl {
public:
    MatrixImpl(Matrix* matrix);
    ~MatrixImpl(){
        canvas_native_matrix_destroy(this->GetMatrix());
        this->matrix_ = nullptr;
    }

    Matrix* GetMatrix();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static MatrixImpl *GetPointer(v8::Local<v8::Object> object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void Ctor(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetA(v8::Local<v8::String> property,
                     const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetA(v8::Local<v8::String> property,
                     v8::Local<v8::Value> value,
                     const v8::PropertyCallbackInfo<void> &info);


    static void GetB(v8::Local<v8::String> property,
                     const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetB(v8::Local<v8::String> property,
                     v8::Local<v8::Value> value,
                     const v8::PropertyCallbackInfo<void> &info);

    static void GetC(v8::Local<v8::String> property,
                     const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetC(v8::Local<v8::String> property,
                     v8::Local<v8::Value> value,
                     const v8::PropertyCallbackInfo<void> &info);

    static void GetD(v8::Local<v8::String> property,
                     const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetD(v8::Local<v8::String> property,
                     v8::Local<v8::Value> value,
                     const v8::PropertyCallbackInfo<void> &info);

    static void GetE(v8::Local<v8::String> property,
                     const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetE(v8::Local<v8::String> property,
                     v8::Local<v8::Value> value,
                     const v8::PropertyCallbackInfo<void> &info);

    static void GetF(v8::Local<v8::String> property,
                     const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetF(v8::Local<v8::String> property,
                     v8::Local<v8::Value> value,
                     const v8::PropertyCallbackInfo<void> &info);


    static void GetM11(v8::Local<v8::String> property,
                     const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetM11(v8::Local<v8::String> property,
                     v8::Local<v8::Value> value,
                     const v8::PropertyCallbackInfo<void> &info);


    static void GetM12(v8::Local<v8::String> property,
                       const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetM12(v8::Local<v8::String> property,
                       v8::Local<v8::Value> value,
                       const v8::PropertyCallbackInfo<void> &info);


    static void GetM13(v8::Local<v8::String> property,
                       const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetM13(v8::Local<v8::String> property,
                       v8::Local<v8::Value> value,
                       const v8::PropertyCallbackInfo<void> &info);

    static void GetM14(v8::Local<v8::String> property,
                       const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetM14(v8::Local<v8::String> property,
                       v8::Local<v8::Value> value,
                       const v8::PropertyCallbackInfo<void> &info);

    static void GetM21(v8::Local<v8::String> property,
                       const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetM21(v8::Local<v8::String> property,
                       v8::Local<v8::Value> value,
                       const v8::PropertyCallbackInfo<void> &info);

    static void GetM22(v8::Local<v8::String> property,
                       const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetM22(v8::Local<v8::String> property,
                       v8::Local<v8::Value> value,
                       const v8::PropertyCallbackInfo<void> &info);

    static void GetM23(v8::Local<v8::String> property,
                       const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetM23(v8::Local<v8::String> property,
                       v8::Local<v8::Value> value,
                       const v8::PropertyCallbackInfo<void> &info);

    static void GetM24(v8::Local<v8::String> property,
                       const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetM24(v8::Local<v8::String> property,
                       v8::Local<v8::Value> value,
                       const v8::PropertyCallbackInfo<void> &info);

    static void GetM31(v8::Local<v8::String> property,
                       const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetM31(v8::Local<v8::String> property,
                       v8::Local<v8::Value> value,
                       const v8::PropertyCallbackInfo<void> &info);

    static void GetM32(v8::Local<v8::String> property,
                       const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetM32(v8::Local<v8::String> property,
                       v8::Local<v8::Value> value,
                       const v8::PropertyCallbackInfo<void> &info);

    static void GetM33(v8::Local<v8::String> property,
                       const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetM33(v8::Local<v8::String> property,
                       v8::Local<v8::Value> value,
                       const v8::PropertyCallbackInfo<void> &info);

    static void GetM34(v8::Local<v8::String> property,
                       const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetM34(v8::Local<v8::String> property,
                       v8::Local<v8::Value> value,
                       const v8::PropertyCallbackInfo<void> &info);

    static void GetM41(v8::Local<v8::String> property,
                       const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetM41(v8::Local<v8::String> property,
                       v8::Local<v8::Value> value,
                       const v8::PropertyCallbackInfo<void> &info);

    static void GetM42(v8::Local<v8::String> property,
                       const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetM42(v8::Local<v8::String> property,
                       v8::Local<v8::Value> value,
                       const v8::PropertyCallbackInfo<void> &info);

    static void GetM43(v8::Local<v8::String> property,
                       const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetM43(v8::Local<v8::String> property,
                       v8::Local<v8::Value> value,
                       const v8::PropertyCallbackInfo<void> &info);

    static void GetM44(v8::Local<v8::String> property,
                       const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetM44(v8::Local<v8::String> property,
                       v8::Local<v8::Value> value,
                       const v8::PropertyCallbackInfo<void> &info);

private:
    Matrix* matrix_;
};
