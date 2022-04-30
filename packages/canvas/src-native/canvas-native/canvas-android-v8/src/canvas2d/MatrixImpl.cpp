//
// Created by Osei Fortune on 03/04/2022.
//

#include "MatrixImpl.h"

MatrixImpl::MatrixImpl(rust::Box <Matrix> matrix) : matrix_(std::move(matrix)) {

}

MatrixImpl *MatrixImpl::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<MatrixImpl *>(ptr);
}

void MatrixImpl::Init(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    global->Set(context, Helpers::ConvertToV8String(isolate, "DOMMatrix"), ctor->GetFunction(context).ToLocalChecked());
}

void MatrixImpl::Create(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();

    if (!args.IsConstructCall()) {
        auto err = v8::Exception::Error(
                Helpers::ConvertToV8String(
                        isolate,
                        "Please use the 'new' operator, this object constructor cannot be called as a function."
                )
        );
        isolate->ThrowException(err);
        return;
    } else {
        v8::Local<v8::Object> ret = args.This();
        Helpers::SetInternalClassName(isolate, ret, "DOMMatrix");

        MatrixImpl *matrix = new MatrixImpl(std::move(canvas_native_matrix_create()));

        auto ext = v8::External::New(isolate, matrix);

        if (ret->InternalFieldCount() > 0) {
            ret->SetInternalField(0, ext);
        } else {
            ret->SetPrivate(context, v8::Private::New(isolate, Helpers::ConvertToV8String(isolate, "rustPtr")),
                            ext);
        }

        args.GetReturnValue().Set(ret);
    }
}

v8::Local<v8::FunctionTemplate> MatrixImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->MatrixTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    auto context = isolate->GetCurrentContext();

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, &Create);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "DOMMatrix"));

    auto func = ctorTmpl->GetFunction(context).ToLocalChecked();

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(1);

    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "a"),
            &GetA,
            &SetA
    );

    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "b"),
            &GetB,
            &SetB
    );

    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "c"),
            &GetC,
            &SetC
    );

    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "d"),
            &GetD,
            &SetD
    );


    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "e"),
            &GetE,
            &SetE
    );


    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "f"),
            &GetF,
            &SetF
    );

    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "m11"),
            &GetM11,
            &SetM11
    );

    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "m12"),
            &GetM12,
            &SetM12
    );

    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "m13"),
            &GetM13,
            &SetM13
    );

    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "m14"),
            &GetM14,
            &SetM14
    );


    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "m21"),
            &GetM21,
            &SetM21
    );

    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "m22"),
            &GetM22,
            &SetM22
    );

    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "m23"),
            &GetM23,
            &SetM23
    );

    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "m24"),
            &GetM24,
            &SetM24
    );


    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "m31"),
            &GetM31,
            &SetM31
    );

    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "m32"),
            &GetM32,
            &SetM32
    );

    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "m33"),
            &GetM33,
            &SetM33
    );

    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "m34"),
            &GetM34,
            &SetM34
    );

    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "m41"),
            &GetM41,
            &SetM41
    );

    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "m42"),
            &GetM42,
            &SetM42
    );

    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "m43"),
            &GetM43,
            &SetM43
    );

    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "m44"),
            &GetM44,
            &SetM44
    );

    cache->MatrixTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void MatrixImpl::GetA(v8::Local<v8::String> name,
                      const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_matrix_get_a(*ptr->matrix_));
    info.GetReturnValue().Set(ret);
}

void MatrixImpl::SetA(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                      const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_matrix_set_a(*ptr->matrix_, static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}


void MatrixImpl::GetB(v8::Local<v8::String> name,
                      const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_matrix_get_b(*ptr->matrix_));
    info.GetReturnValue().Set(ret);
}

void MatrixImpl::SetB(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                      const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_matrix_set_b(*ptr->matrix_, static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}


void MatrixImpl::GetC(v8::Local<v8::String> name,
                      const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_matrix_get_c(*ptr->matrix_));
    info.GetReturnValue().Set(ret);
}

void MatrixImpl::SetC(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                      const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_matrix_set_c(*ptr->matrix_, static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}


void MatrixImpl::GetD(v8::Local<v8::String> name,
                      const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_matrix_get_d(*ptr->matrix_));
    info.GetReturnValue().Set(ret);
}

void MatrixImpl::SetD(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                      const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_matrix_set_d(*ptr->matrix_, static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}


void MatrixImpl::GetE(v8::Local<v8::String> name,
                      const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_matrix_get_e(*ptr->matrix_));
    info.GetReturnValue().Set(ret);
}

void MatrixImpl::SetE(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                      const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_matrix_set_e(*ptr->matrix_, static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}


void MatrixImpl::GetF(v8::Local<v8::String> name,
                      const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_matrix_get_f(*ptr->matrix_));
    info.GetReturnValue().Set(ret);
}

void MatrixImpl::SetF(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                      const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_matrix_set_f(*ptr->matrix_, static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}

void MatrixImpl::GetM11(v8::Local<v8::String> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_matrix_get_m11(*ptr->matrix_));
    info.GetReturnValue().Set(ret);
}

void MatrixImpl::SetM11(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_matrix_set_m11(*ptr->matrix_, static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}

void MatrixImpl::GetM12(v8::Local<v8::String> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_matrix_get_m12(*ptr->matrix_));
    info.GetReturnValue().Set(ret);
}

void MatrixImpl::SetM12(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_matrix_set_m12(*ptr->matrix_, static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}

void MatrixImpl::GetM13(v8::Local<v8::String> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_matrix_get_m13(*ptr->matrix_));
    info.GetReturnValue().Set(ret);
}

void MatrixImpl::SetM13(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_matrix_set_m13(*ptr->matrix_, static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}

void MatrixImpl::GetM14(v8::Local<v8::String> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_matrix_get_m14(*ptr->matrix_));
    info.GetReturnValue().Set(ret);
}

void MatrixImpl::SetM14(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_matrix_set_m14(*ptr->matrix_, static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}

void MatrixImpl::GetM21(v8::Local<v8::String> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_matrix_get_m21(*ptr->matrix_));
    info.GetReturnValue().Set(ret);
}

void MatrixImpl::SetM21(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_matrix_set_m21(*ptr->matrix_, static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}

void MatrixImpl::GetM22(v8::Local<v8::String> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_matrix_get_m22(*ptr->matrix_));
    info.GetReturnValue().Set(ret);
}

void MatrixImpl::SetM22(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_matrix_set_m22(*ptr->matrix_, static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}

void MatrixImpl::GetM23(v8::Local<v8::String> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_matrix_get_m23(*ptr->matrix_));
    info.GetReturnValue().Set(ret);
}

void MatrixImpl::SetM23(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_matrix_set_m23(*ptr->matrix_, static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}

void MatrixImpl::GetM24(v8::Local<v8::String> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_matrix_get_m24(*ptr->matrix_));
    info.GetReturnValue().Set(ret);
}

void MatrixImpl::SetM24(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_matrix_set_m24(*ptr->matrix_, static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}

void MatrixImpl::GetM31(v8::Local<v8::String> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_matrix_get_m31(*ptr->matrix_));
    info.GetReturnValue().Set(ret);
}

void MatrixImpl::SetM31(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_matrix_set_m31(*ptr->matrix_, static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}

void MatrixImpl::GetM32(v8::Local<v8::String> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_matrix_get_m32(*ptr->matrix_));
    info.GetReturnValue().Set(ret);
}

void MatrixImpl::SetM32(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_matrix_set_m32(*ptr->matrix_, static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}

void MatrixImpl::GetM33(v8::Local<v8::String> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_matrix_get_m33(*ptr->matrix_));
    info.GetReturnValue().Set(ret);
}

void MatrixImpl::SetM33(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_matrix_set_m33(*ptr->matrix_, static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}

void MatrixImpl::GetM34(v8::Local<v8::String> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_matrix_get_m34(*ptr->matrix_));
    info.GetReturnValue().Set(ret);
}

void MatrixImpl::SetM34(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_matrix_set_m34(*ptr->matrix_, static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}

void MatrixImpl::GetM41(v8::Local<v8::String> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_matrix_get_m41(*ptr->matrix_));
    info.GetReturnValue().Set(ret);
}

void MatrixImpl::SetM41(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_matrix_set_m41(*ptr->matrix_, static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}

void MatrixImpl::GetM42(v8::Local<v8::String> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_matrix_get_m42(*ptr->matrix_));
    info.GetReturnValue().Set(ret);
}

void MatrixImpl::SetM42(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_matrix_set_m42(*ptr->matrix_, static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}

void MatrixImpl::GetM43(v8::Local<v8::String> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_matrix_get_m43(*ptr->matrix_));
    info.GetReturnValue().Set(ret);
}

void MatrixImpl::SetM43(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_matrix_set_m43(*ptr->matrix_, static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}

void MatrixImpl::GetM44(v8::Local<v8::String> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_matrix_get_m44(*ptr->matrix_));
    info.GetReturnValue().Set(ret);
}

void MatrixImpl::SetM44(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_matrix_set_m44(*ptr->matrix_, static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}

Matrix &MatrixImpl::GetMatrix() {
    return *this->matrix_;
}