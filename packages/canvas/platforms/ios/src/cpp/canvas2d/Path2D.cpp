//
// Created by Osei Fortune on 28/03/2022.
//

#include "Path2D.h"
#include "Caches.h"
#include "Helpers.h"
#include "OneByteStringResource.h"

Path2D::Path2D(Path *path)
        : path_(path) {}


void Path2D::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "Path2D"), func);
}

Path2D *Path2D::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<Path2D *>(ptr);
}

v8::CFunction Path2D::fast_arc_(v8::CFunction::Make(Path2D::FastArc));

v8::CFunction Path2D::fast_add_path_(v8::CFunction::Make(Path2D::FastAddPath));

v8::CFunction Path2D::fast_arc_to_(v8::CFunction::Make(Path2D::FastArcTo));

v8::CFunction Path2D::fast_bezier_curve_to_(v8::CFunction::Make(Path2D::FastBezierCurveTo));

v8::CFunction Path2D::fast_close_path_(v8::CFunction::Make(Path2D::FastClosePath));

v8::CFunction Path2D::fast_ellipse_(v8::CFunction::Make(Path2D::FastEllipse));

v8::CFunction Path2D::fast_line_to_(v8::CFunction::Make(Path2D::FastLineTo));

v8::CFunction Path2D::fast_move_to_(v8::CFunction::Make(Path2D::FastMoveTo));

v8::CFunction Path2D::fast_quadratic_curve_to_(v8::CFunction::Make(Path2D::FastQuadraticCurveTo));

v8::CFunction Path2D::fast_rect_(v8::CFunction::Make(Path2D::FastRect));

//v8::CFunction Path2D::fast_round_rect_(v8::CFunction::Make(Path2D::FastRoundRect));

//v8::CFunction Path2D::fast_to_svg_(v8::CFunction::Make(Path2D::FastToSVG));

void Path2D::Ctor(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto count = args.Length();
    auto value = args[0];
    auto isolate = args.GetIsolate();

    auto ret = args.This();

    SetNativeType(isolate, ret, NativeType::Path2D);

    if (count > 0) {
        if (value->IsString()) {
            auto d = ConvertFromV8String(isolate, value);
            auto path = canvas_native_path_create_with_string(d.c_str());
            auto object = new Path2D(path);

            ret->SetAlignedPointerInInternalField(0, object);

            object->BindFinalizer(isolate, ret);

            args.GetReturnValue().Set(ret);

            return;

        } else if (value->IsObject()) {
            auto path_to_copy = GetPointer(value.As<v8::Object>());
            if (path_to_copy != nullptr) {
                auto path = canvas_native_path_create_with_path(
                        path_to_copy->GetPath());
                auto object = new Path2D(path);

                ret->SetAlignedPointerInInternalField(0, object);

                object->BindFinalizer(isolate, ret);

                args.GetReturnValue().Set(ret);
                return;
            }
        }
    } else {
        auto path = new Path2D(canvas_native_path_create());


        ret->SetAlignedPointerInInternalField(0, path);

        path->BindFinalizer(isolate, ret);

        args.GetReturnValue().Set(ret);
        return;

    }

    args.GetReturnValue().SetUndefined();
}

void Path2D::AddPath(const v8::FunctionCallbackInfo<v8::Value> &args) {
    Path2D *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetUndefined();
        return;
    }

    auto value = args[0];
    if (value->IsObject()) {
        auto object = GetPointer(value.As<v8::Object>());
        if (object != nullptr) {
            AddPathImpl(ptr->GetPath(),
                        object->GetPath());
        }
    }
}

void Path2D::Arc(const v8::FunctionCallbackInfo<v8::Value> &args) {
    Path2D *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetUndefined();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto anti_clockwise = false;
    if (args.Length() == 6) {
        anti_clockwise = args[5]->BooleanValue(isolate);
    }

    canvas_native_path_arc(
            ptr->GetPath(),
            static_cast<float>(args[0]->NumberValue(context).ToChecked()),
            static_cast<float>(args[1]->NumberValue(context).ToChecked()),
            static_cast<float>(args[2]->NumberValue(context).ToChecked()),
            static_cast<float>(args[3]->NumberValue(context).ToChecked()),
            static_cast<float>(args[4]->NumberValue(context).ToChecked()),
            anti_clockwise
    );


    args.GetReturnValue().SetUndefined();
}

void Path2D::ArcTo(const v8::FunctionCallbackInfo<v8::Value> &args) {
    Path2D *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetUndefined();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    ArcToImpl(
            ptr->GetPath(),
            args[0]->NumberValue(context).ToChecked(),
            args[1]->NumberValue(context).ToChecked(),
            args[2]->NumberValue(context).ToChecked(),
            args[3]->NumberValue(context).ToChecked(),
            args[4]->NumberValue(context).ToChecked()
    );
}

void Path2D::BezierCurveTo(const v8::FunctionCallbackInfo<v8::Value> &args) {
    Path2D *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetUndefined();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    BezierCurveToImpl(
            ptr->GetPath(),
            static_cast<float>(args[0]->NumberValue(context).ToChecked()),
            static_cast<float>(args[1]->NumberValue(context).ToChecked()),
            static_cast<float>(args[2]->NumberValue(context).ToChecked()),
            static_cast<float>(args[3]->NumberValue(context).ToChecked()),
            static_cast<float>(args[4]->NumberValue(context).ToChecked()),
            static_cast<float>(args[5]->NumberValue(context).ToChecked())
    );

    args.GetReturnValue().SetUndefined();
}

void Path2D::ClosePath(const v8::FunctionCallbackInfo<v8::Value> &args) {
    Path2D *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetUndefined();
        return;
    }

    canvas_native_path_close_path(ptr->GetPath());
}

void Path2D::Ellipse(const v8::FunctionCallbackInfo<v8::Value> &args) {
    Path2D *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto anti_clockwise = false;
    if (args.Length() > 7) {
        anti_clockwise = args[7]->BooleanValue(isolate);
    }

    canvas_native_path_ellipse(
            ptr->GetPath(),
            static_cast<float>(args[0]->NumberValue(context).ToChecked()),
            static_cast<float>(args[1]->NumberValue(context).ToChecked()),
            static_cast<float>(args[2]->NumberValue(context).ToChecked()),
            static_cast<float>(args[3]->NumberValue(context).ToChecked()),
            static_cast<float>(args[4]->NumberValue(context).ToChecked()),
            static_cast<float>(args[5]->NumberValue(context).ToChecked()),
            static_cast<float>(args[6]->NumberValue(context).ToChecked()),
            anti_clockwise
    );


    args.GetReturnValue().SetUndefined();
}

void Path2D::LineTo(const v8::FunctionCallbackInfo<v8::Value> &args) {
    Path2D *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    canvas_native_path_line_to(ptr->GetPath(),
                               static_cast<float>(args[0]->NumberValue(context).ToChecked()),
                               static_cast<float>(args[1]->NumberValue(context).ToChecked()));


    args.GetReturnValue().SetUndefined();
}

void Path2D::MoveTo(const v8::FunctionCallbackInfo<v8::Value> &args) {
    Path2D *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    canvas_native_path_move_to(ptr->GetPath(),
                               static_cast<float>(args[0]->NumberValue(context).ToChecked()),
                               static_cast<float>(args[1]->NumberValue(context).ToChecked()));


    args.GetReturnValue().SetUndefined();
}

void Path2D::QuadraticCurveTo(const v8::FunctionCallbackInfo<v8::Value> &args) {
    Path2D *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    canvas_native_path_quadratic_curve_to(
            ptr->GetPath(),
            static_cast<float>(args[0]->NumberValue(context).ToChecked()),
            static_cast<float>(args[1]->NumberValue(context).ToChecked()),
            static_cast<float>(args[2]->NumberValue(context).ToChecked()),
            static_cast<float>(args[3]->NumberValue(context).ToChecked())
    );

    args.GetReturnValue().SetUndefined();
}

void Path2D::Rect(const v8::FunctionCallbackInfo<v8::Value> &args) {
    Path2D *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    canvas_native_path_rect(
            ptr->GetPath(),
            static_cast<float>(args[0]->NumberValue(context).ToChecked()),
            static_cast<float>(args[1]->NumberValue(context).ToChecked()),
            static_cast<float>(args[2]->NumberValue(context).ToChecked()),
            static_cast<float>(args[3]->NumberValue(context).ToChecked())
    );

    args.GetReturnValue().SetUndefined();
}

void Path2D::RoundRect(const v8::FunctionCallbackInfo<v8::Value> &args) {
    Path2D *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    if (args.Length() == 5) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto width = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto height = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        auto objectOrNumber = args[4];
        if (objectOrNumber->IsObject()) {
            auto radii = objectOrNumber.As<v8::Object>();
            if (radii->IsArray()) {
                auto array = radii.As<v8::Array>();
                auto size = array->Length();

                if (size > 1) {
                    std::vector<float> store;
                    store.reserve(size);
                    for (int i = 0;
                         i < size; i++) {
                        store[i] = (float) array->Get(context, i).ToLocalChecked()->NumberValue(
                                context).ToChecked();
                    }

                    canvas_native_path_round_rect(
                            ptr->GetPath(),
                            x, y,
                            width,
                            height, store.data(),
                            store.size());

                }
            }
        } else {
            auto radii = (float) objectOrNumber->NumberValue(context).ToChecked();
            canvas_native_path_round_rect_tl_tr_br_bl(
                    ptr->GetPath(), x, y,
                    width,
                    height, radii, radii,
                    radii, radii);

        }
    }

    args.GetReturnValue().SetUndefined();
}

void Path2D::__toSVG(const v8::FunctionCallbackInfo<v8::Value> &args) {
    Path2D *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetEmptyString();
        return;
    }

    auto isolate = args.GetIsolate();

    auto d = canvas_native_path_to_string(
            ptr->GetPath());

//    args.GetReturnValue().Set(ConvertToV8String(isolate, d.c_str()));

    auto value = new OneByteStringResource((char *) d);
    auto ret = v8::String::NewExternalOneByte(isolate, value);
    args.GetReturnValue().Set(ret.ToLocalChecked());
}

Path *Path2D::GetPath() {
    return this->path_;
}


v8::Local<v8::FunctionTemplate> Path2D::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->Path2DTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, Ctor);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "Path2D"));

    auto tmpl = ctorTmpl->InstanceTemplate();

    tmpl->SetInternalFieldCount(1);

//    tmpl->Set(
//            ConvertToV8String(isolate, "addPath"),
//            v8::FunctionTemplate::New(isolate, &AddPath));

//    tmpl->Set(
//            ConvertToV8String(isolate, "arc"),
//            v8::FunctionTemplate::New(isolate, &Arc));

    tmpl->Set(
            ConvertToV8String(isolate, "arcTo"),
            v8::FunctionTemplate::New(isolate, &ArcTo));

    SetFastMethod(isolate, tmpl, "addPath", AddPath, &fast_add_path_, v8::Local<v8::Value>());
    SetFastMethod(isolate, tmpl, "arc", Arc, &fast_arc_, v8::Local<v8::Value>());
    SetFastMethod(isolate, tmpl, "arcTo", ArcTo, &fast_arc_to_, v8::Local<v8::Value>());
    SetFastMethod(isolate, tmpl, "bezierCurveTo", BezierCurveTo, &fast_bezier_curve_to_,
                  v8::Local<v8::Value>());
    SetFastMethod(isolate, tmpl, "closePath", ClosePath, &fast_close_path_, v8::Local<v8::Value>());
    SetFastMethod(isolate, tmpl, "ellipse", Ellipse, &fast_ellipse_, v8::Local<v8::Value>());
    SetFastMethod(isolate, tmpl, "lineTo", LineTo, &fast_line_to_, v8::Local<v8::Value>());
    SetFastMethod(isolate, tmpl, "moveTo", MoveTo, &fast_move_to_, v8::Local<v8::Value>());
    SetFastMethod(isolate, tmpl, "quadraticCurveTo", QuadraticCurveTo, &fast_quadratic_curve_to_,
                  v8::Local<v8::Value>());
    SetFastMethod(isolate, tmpl, "rect", Rect, &fast_rect_, v8::Local<v8::Value>());
//    SetFastMethod(isolate, tmpl, "roundRect", RoundRect, &fast_round_rect_, v8::Local<v8::Value>());
//    SetFastMethod(isolate, tmpl, "__toSVG", __toSVG, &fast_to_svg_, v8::Local<v8::Value>());


//    tmpl->Set(
//            ConvertToV8String(isolate, "bezierCurveTo"),
//            v8::FunctionTemplate::New(isolate, &BezierCurveTo));


//    tmpl->Set(
//            ConvertToV8String(isolate, "closePath"),
//            v8::FunctionTemplate::New(isolate, &ClosePath));


//    tmpl->Set(
//            ConvertToV8String(isolate, "ellipse"),
//            v8::FunctionTemplate::New(isolate, &Ellipse));

//    tmpl->Set(
//            ConvertToV8String(isolate, "lineTo"),
//            v8::FunctionTemplate::New(isolate, &LineTo));


//    tmpl->Set(
//            ConvertToV8String(isolate, "moveTo"),
//            v8::FunctionTemplate::New(isolate, &MoveTo));


//    tmpl->Set(
//            ConvertToV8String(isolate, "quadraticCurveTo"),
//            v8::FunctionTemplate::New(isolate, &QuadraticCurveTo));

//    tmpl->Set(
//            ConvertToV8String(isolate, "rect"),
//            v8::FunctionTemplate::New(isolate, &Rect));

    tmpl->Set(
            ConvertToV8String(isolate, "roundRect"),
            v8::FunctionTemplate::New(isolate, &RoundRect));

    tmpl->Set(
            ConvertToV8String(isolate, "__toSVG"),
            v8::FunctionTemplate::New(isolate, &__toSVG));


    cache->Path2DTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}
