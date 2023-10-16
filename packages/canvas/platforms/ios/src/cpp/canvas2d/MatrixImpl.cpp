//
// Created by Osei Fortune on 03/04/2022.
//

#include <array>
#include "MatrixImpl.h"
#include "Caches.h"

MatrixImpl::MatrixImpl(Matrix* matrix) : matrix_(matrix) {}

void MatrixImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "DOMMatrix"), func);
}

MatrixImpl *MatrixImpl::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<MatrixImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> MatrixImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->MatrixTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, Ctor);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "DOMMatrix"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(1);

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "a"),
            GetA,
            SetA
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "b"),
            GetB,
            SetB
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "c"),
            GetC,
            SetC
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "d"),
            GetD,
            SetD
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "e"),
            GetE,
            SetE
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "f"),
            GetF,
            SetF
    );


    tmpl->SetAccessor(
            ConvertToV8String(isolate, "m11"),
            GetM11,
            SetM11
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "m12"),
            GetM12,
            SetM12
    );


    tmpl->SetAccessor(
            ConvertToV8String(isolate, "m13"),
            GetM13,
            SetM13
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "m14"),
            GetM14,
            SetM14
    );


    tmpl->SetAccessor(
            ConvertToV8String(isolate, "m21"),
            GetM21,
            SetM21
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "m22"),
            GetM22,
            SetM22
    );


    tmpl->SetAccessor(
            ConvertToV8String(isolate, "m23"),
            GetM23,
            SetM23
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "m24"),
            GetM24,
            SetM24
    );


    tmpl->SetAccessor(
            ConvertToV8String(isolate, "m31"),
            GetM31,
            SetM31
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "m32"),
            GetM32,
            SetM32
    );


    tmpl->SetAccessor(
            ConvertToV8String(isolate, "m33"),
            GetM33,
            SetM33
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "m34"),
            GetM34,
            SetM34
    );


    tmpl->SetAccessor(
            ConvertToV8String(isolate, "m41"),
            GetM41,
            SetM41
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "m42"),
            GetM42,
            SetM42
    );


    tmpl->SetAccessor(
            ConvertToV8String(isolate, "m43"),
            GetM43,
            SetM43
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "m44"),
            GetM44,
            SetM44
    );


    cache->MatrixTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void MatrixImpl::Ctor(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto count = args.Length();
    auto isolate = args.GetIsolate();

    auto ret = args.This();

    if (count > 0) {
        auto context = isolate->GetCurrentContext();
        auto value = args[0];
        if (value->IsArray()) {
            auto array = value.As<v8::Array>();
            auto size = array->Length();

            if (size == 6) {
                auto matrix = canvas_native_matrix_create();
                std::array<float, 6> buf{};
                for (int i = 0; i < size; i++) {
                    auto item = array->Get(context, i).ToLocalChecked()->NumberValue(
                            context).ToChecked();
                    buf[i] = (float) item;
                }

                canvas_native_matrix_update(matrix, buf.data(), buf.size());

                auto object = new MatrixImpl(matrix);

                auto ext = v8::External::New(isolate, object);

                ret->SetInternalField(0, ext);

                args.GetReturnValue().Set(ret);

                return;
            }

            if (size == 16) {
                auto matrix = canvas_native_matrix_create();
                std::array<float, 16> buf{};

                for (int i = 0; i < size; i++) {
                    auto item = array->Get(context, i).ToLocalChecked()->NumberValue(
                            context).ToChecked();
                    buf[i] = (float) item;
                }

                canvas_native_matrix_update_3d(matrix, buf.data(), buf.size());

                auto object = new MatrixImpl(matrix);

                auto ext = v8::External::New(isolate, object);

                ret->SetInternalField(0, ext);

                args.GetReturnValue().Set(ret);
                return;
            }

        }
    } else {
        auto matrix = canvas_native_matrix_create();
        auto object = new MatrixImpl(matrix);

        auto ext = v8::External::New(isolate, object);

        ret->SetInternalField(0, ext);

        args.GetReturnValue().Set(ret);
        return;
    }

    args.GetReturnValue().SetUndefined();
}

void MatrixImpl::GetA(v8::Local<v8::String> property,
                      const v8::PropertyCallbackInfo<v8::Value> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().SetUndefined();
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_matrix_get_a(ptr->GetMatrix()));

}

void MatrixImpl::SetA(v8::Local<v8::String> property,
                      v8::Local<v8::Value> value,
                      const v8::PropertyCallbackInfo<void> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    canvas_native_matrix_set_a(ptr->GetMatrix(), (float) value->NumberValue(context).ToChecked());
}


void MatrixImpl::GetB(v8::Local<v8::String> property,
                      const v8::PropertyCallbackInfo<v8::Value> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().SetUndefined();
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_matrix_get_b(ptr->GetMatrix()));

}

void MatrixImpl::SetB(v8::Local<v8::String> property,
                      v8::Local<v8::Value> value,
                      const v8::PropertyCallbackInfo<void> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    canvas_native_matrix_set_b(ptr->GetMatrix(), (float) value->NumberValue(context).ToChecked());
}


void MatrixImpl::GetC(v8::Local<v8::String> property,
                      const v8::PropertyCallbackInfo<v8::Value> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().SetUndefined();
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_matrix_get_c(ptr->GetMatrix()));

}

void MatrixImpl::SetC(v8::Local<v8::String> property,
                      v8::Local<v8::Value> value,
                      const v8::PropertyCallbackInfo<void> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    canvas_native_matrix_set_c(ptr->GetMatrix(), (float) value->NumberValue(context).ToChecked());
}


void MatrixImpl::GetD(v8::Local<v8::String> property,
                      const v8::PropertyCallbackInfo<v8::Value> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().SetUndefined();
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_matrix_get_d(ptr->GetMatrix()));

}

void MatrixImpl::SetD(v8::Local<v8::String> property,
                      v8::Local<v8::Value> value,
                      const v8::PropertyCallbackInfo<void> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    canvas_native_matrix_set_d(ptr->GetMatrix(), (float) value->NumberValue(context).ToChecked());
}


void MatrixImpl::GetE(v8::Local<v8::String> property,
                      const v8::PropertyCallbackInfo<v8::Value> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().SetUndefined();
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_matrix_get_e(ptr->GetMatrix()));

}

void MatrixImpl::SetE(v8::Local<v8::String> property,
                      v8::Local<v8::Value> value,
                      const v8::PropertyCallbackInfo<void> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    canvas_native_matrix_set_e(ptr->GetMatrix(), (float) value->NumberValue(context).ToChecked());
}


void MatrixImpl::GetF(v8::Local<v8::String> property,
                      const v8::PropertyCallbackInfo<v8::Value> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().SetUndefined();
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_matrix_get_f(ptr->GetMatrix()));

}

void MatrixImpl::SetF(v8::Local<v8::String> property,
                      v8::Local<v8::Value> value,
                      const v8::PropertyCallbackInfo<void> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    canvas_native_matrix_set_f(ptr->GetMatrix(), (float) value->NumberValue(context).ToChecked());
}

void MatrixImpl::GetM11(v8::Local<v8::String> property,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().SetUndefined();
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_matrix_get_m11(ptr->GetMatrix()));

}

void MatrixImpl::SetM11(v8::Local<v8::String> property,
                        v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    canvas_native_matrix_set_m11(ptr->GetMatrix(), (float) value->NumberValue(context).ToChecked());
}

void MatrixImpl::GetM12(v8::Local<v8::String> property,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().SetUndefined();
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_matrix_get_m12(ptr->GetMatrix()));

}

void MatrixImpl::SetM12(v8::Local<v8::String> property,
                        v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    canvas_native_matrix_set_m12(ptr->GetMatrix(), (float) value->NumberValue(context).ToChecked());
}

void MatrixImpl::GetM13(v8::Local<v8::String> property,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().SetUndefined();
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_matrix_get_m13(ptr->GetMatrix()));

}

void MatrixImpl::SetM13(v8::Local<v8::String> property,
                        v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    canvas_native_matrix_set_m13(ptr->GetMatrix(), (float) value->NumberValue(context).ToChecked());
}

void MatrixImpl::GetM14(v8::Local<v8::String> property,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().SetUndefined();
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_matrix_get_m14(ptr->GetMatrix()));

}

void MatrixImpl::SetM14(v8::Local<v8::String> property,
                        v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    canvas_native_matrix_set_m14(ptr->GetMatrix(), (float) value->NumberValue(context).ToChecked());
}

void MatrixImpl::GetM21(v8::Local<v8::String> property,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().SetUndefined();
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_matrix_get_m21(ptr->GetMatrix()));

}

void MatrixImpl::SetM21(v8::Local<v8::String> property,
                        v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    canvas_native_matrix_set_m21(ptr->GetMatrix(), (float) value->NumberValue(context).ToChecked());
}

void MatrixImpl::GetM22(v8::Local<v8::String> property,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().SetUndefined();
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_matrix_get_m22(ptr->GetMatrix()));

}

void MatrixImpl::SetM22(v8::Local<v8::String> property,
                        v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    canvas_native_matrix_set_m22(ptr->GetMatrix(), (float) value->NumberValue(context).ToChecked());
}

void MatrixImpl::GetM23(v8::Local<v8::String> property,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().SetUndefined();
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_matrix_get_m23(ptr->GetMatrix()));

}

void MatrixImpl::SetM23(v8::Local<v8::String> property,
                        v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    canvas_native_matrix_set_m23(ptr->GetMatrix(), (float) value->NumberValue(context).ToChecked());
}

void MatrixImpl::GetM24(v8::Local<v8::String> property,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().SetUndefined();
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_matrix_get_m24(ptr->GetMatrix()));

}

void MatrixImpl::SetM24(v8::Local<v8::String> property,
                        v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    canvas_native_matrix_set_m24(ptr->GetMatrix(), (float) value->NumberValue(context).ToChecked());
}


void MatrixImpl::GetM31(v8::Local<v8::String> property,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().SetUndefined();
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_matrix_get_m31(ptr->GetMatrix()));

}

void MatrixImpl::SetM31(v8::Local<v8::String> property,
                        v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    canvas_native_matrix_set_m31(ptr->GetMatrix(), (float) value->NumberValue(context).ToChecked());
}

void MatrixImpl::GetM32(v8::Local<v8::String> property,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().SetUndefined();
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_matrix_get_m32(ptr->GetMatrix()));

}

void MatrixImpl::SetM32(v8::Local<v8::String> property,
                        v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    canvas_native_matrix_set_m32(ptr->GetMatrix(), (float) value->NumberValue(context).ToChecked());
}

void MatrixImpl::GetM33(v8::Local<v8::String> property,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().SetUndefined();
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_matrix_get_m33(ptr->GetMatrix()));

}

void MatrixImpl::SetM33(v8::Local<v8::String> property,
                        v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    canvas_native_matrix_set_m33(ptr->GetMatrix(), (float) value->NumberValue(context).ToChecked());
}

void MatrixImpl::GetM34(v8::Local<v8::String> property,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().SetUndefined();
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_matrix_get_m34(ptr->GetMatrix()));

}

void MatrixImpl::SetM34(v8::Local<v8::String> property,
                        v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    canvas_native_matrix_set_m34(ptr->GetMatrix(), (float) value->NumberValue(context).ToChecked());
}

void MatrixImpl::GetM41(v8::Local<v8::String> property,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().SetUndefined();
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_matrix_get_m41(ptr->GetMatrix()));

}

void MatrixImpl::SetM41(v8::Local<v8::String> property,
                        v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    canvas_native_matrix_set_m41(ptr->GetMatrix(), (float) value->NumberValue(context).ToChecked());
}

void MatrixImpl::GetM42(v8::Local<v8::String> property,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().SetUndefined();
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_matrix_get_m42(ptr->GetMatrix()));

}

void MatrixImpl::SetM42(v8::Local<v8::String> property,
                        v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    canvas_native_matrix_set_m42(ptr->GetMatrix(), (float) value->NumberValue(context).ToChecked());
}

void MatrixImpl::GetM43(v8::Local<v8::String> property,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().SetUndefined();
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_matrix_get_m43(ptr->GetMatrix()));

}

void MatrixImpl::SetM43(v8::Local<v8::String> property,
                        v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    canvas_native_matrix_set_m43(ptr->GetMatrix(), (float) value->NumberValue(context).ToChecked());
}

void MatrixImpl::GetM44(v8::Local<v8::String> property,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().SetUndefined();
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_matrix_get_m44(ptr->GetMatrix()));

}

void MatrixImpl::SetM44(v8::Local<v8::String> property,
                        v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info) {
    MatrixImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    canvas_native_matrix_set_m44(ptr->GetMatrix(), (float) value->NumberValue(context).ToChecked());
}

Matrix* MatrixImpl::GetMatrix() {
    return this->matrix_;
}
