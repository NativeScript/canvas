//
// Created by Osei Fortune on 28/03/2022.
//

#include "Path2D.h"
#include "canvas-android-v8/src/bridges/context.rs.h"


Path2D::Path2D(rust::Box<Path> path)
        : path_(std::move(path)) {}

void Path2D::Init(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    global->Set(context, Helpers::ConvertToV8String(isolate, "Path2D"), ctor->GetFunction(context).ToLocalChecked());
}

Path2D *Path2D::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<Path2D *>(ptr);
}

void Path2D::Create(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
//    v8::Locker locker(isolate);
//    v8::Isolate::Scope isolate_scope(isolate);
//    v8::HandleScope handle_scope(isolate);
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
        Helpers::SetInstanceType(isolate, ret, ObjectType::Path2D);
        if (args.Length() > 0) {
            auto obj = args[0];
            if (obj->IsString()) {
                auto d = Helpers::ConvertFromV8String(isolate, obj->ToString(context).ToLocalChecked());
                auto path_box = canvas_native_path_create_with_str(rust::Str(d.c_str(), d.size()));
                Path2D *path = new Path2D(std::move(path_box));
                AddWeakListener(isolate, ret, path);
                args.GetReturnValue().Set(ret);
                return;
            } else if (obj->IsObject()) {
                auto object = obj->ToObject(context).ToLocalChecked();
                if (Helpers::GetInstanceType(isolate, object) == ObjectType::Path2D) {
                    auto path_from = GetPointer(object);
                    Path2D *path = new Path2D(std::move(canvas_native_path_create_with_path(
                            *path_from->path_
                    )));
                    AddWeakListener(isolate, ret, path);
                    args.GetReturnValue().Set(ret);
                    return;
                }
            }
        } else {
            Path2D *path = new Path2D(std::move(canvas_native_path_create()));
            AddWeakListener(isolate, ret, path);
            args.GetReturnValue().Set(ret);
        }
    }
}

void Path2D::AddWeakListener(v8::Isolate *isolate, const v8::Local<v8::Object> &object, Path2D *path) {
    auto ext = v8::External::New(isolate, path);
    auto context = isolate->GetCurrentContext();
    if (object->InternalFieldCount() > 0) {
        object->SetInternalField(0, ext);
    } else {
        object->SetPrivate(context, v8::Private::New(isolate, Helpers::ConvertToV8String(isolate, "rustPtr")),
                           ext);
    }
    auto persistent = new v8::Persistent<v8::Object>(isolate, object);
    auto entry = new ObjectCacheEntry(static_cast<void *>(path), persistent);
    auto callback = [](const v8::WeakCallbackInfo<ObjectCacheEntry> &data) {
        auto value = data.GetParameter();
        auto path_ptr = static_cast<Path2D *>(value->data);
        if (path_ptr != nullptr) {
            delete path_ptr;
        }
        auto persistent_ptr = value->object;
        if (persistent_ptr != nullptr) {
            if (!persistent_ptr->IsEmpty()) {
                //persistent_ptr->ClearWeak();
                persistent_ptr->Reset();
            }
        }
        delete value;
    };
    persistent->SetWeak(entry, callback, v8::WeakCallbackType::kFinalizer);
}

void Path2D::AddPath(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (args[0]->IsObject()) {
        auto ptr = GetPointer(args.This());
        auto object = args[0]->ToObject(context).ToLocalChecked();
        if (Helpers::GetInstanceType(isolate, object) == ObjectType::Path2D) {
            auto to_add = GetPointer(object);
            canvas_native_path_add_path(*ptr->path_, *to_add->path_);
        }
    }

}

void Path2D::Arc(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());

    auto anti_clockwise = false;
    if (args.Length() == 6) {
        anti_clockwise = args[5]->BooleanValue(isolate);
    }

    canvas_native_path_arc(
            *ptr->path_,
            static_cast<float>(args[0]->NumberValue(context).ToChecked()),
            static_cast<float>(args[1]->NumberValue(context).ToChecked()),
            static_cast<float>(args[2]->NumberValue(context).ToChecked()),
            static_cast<float>(args[3]->NumberValue(context).ToChecked()),
            static_cast<float>(args[4]->NumberValue(context).ToChecked()),
            anti_clockwise
    );
}

void Path2D::ArcTo(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());

    canvas_native_path_arc_to(
            *ptr->path_,
            static_cast<float>(args[0]->NumberValue(context).ToChecked()),
            static_cast<float>(args[1]->NumberValue(context).ToChecked()),
            static_cast<float>(args[2]->NumberValue(context).ToChecked()),
            static_cast<float>(args[3]->NumberValue(context).ToChecked()),
            static_cast<float>(args[4]->NumberValue(context).ToChecked())
    );

}

void Path2D::BezierCurveTo(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    canvas_native_path_bezier_curve_to(
            *ptr->path_,
            static_cast<float>(args[0]->NumberValue(context).ToChecked()),
            static_cast<float>(args[1]->NumberValue(context).ToChecked()),
            static_cast<float>(args[2]->NumberValue(context).ToChecked()),
            static_cast<float>(args[3]->NumberValue(context).ToChecked()),
            static_cast<float>(args[4]->NumberValue(context).ToChecked()),
            static_cast<float>(args[5]->NumberValue(context).ToChecked())
    );
}

void Path2D::ClosePath(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    canvas_native_path_close_path(*ptr->path_);
}

void Path2D::Ellipse(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    auto anti_clockwise = false;
    if (args.Length() == 6) {
        anti_clockwise = args[5]->BooleanValue(isolate);
    }
    canvas_native_path_ellipse(
            *ptr->path_,
            static_cast<float>(args[0]->NumberValue(context).ToChecked()),
            static_cast<float>(args[1]->NumberValue(context).ToChecked()),
            static_cast<float>(args[2]->NumberValue(context).ToChecked()),
            static_cast<float>(args[3]->NumberValue(context).ToChecked()),
            static_cast<float>(args[4]->NumberValue(context).ToChecked()),
            static_cast<float>(args[5]->NumberValue(context).ToChecked()),
            static_cast<float>(args[6]->NumberValue(context).ToChecked()),
            anti_clockwise
    );
}

void Path2D::LineTo(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    canvas_native_path_line_to(*ptr->path_,
                               static_cast<float>(args[0]->NumberValue(context).ToChecked()),
                               static_cast<float>(args[1]->NumberValue(context).ToChecked()));
}

void Path2D::MoveTo(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    canvas_native_path_move_to(*ptr->path_,
                               static_cast<float>(args[0]->NumberValue(context).ToChecked()),
                               static_cast<float>(args[1]->NumberValue(context).ToChecked()));
}

void Path2D::QuadraticCurveTo(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    canvas_native_path_quadratic_curve_to(
            *ptr->path_,
            static_cast<float>(args[0]->NumberValue(context).ToChecked()),
            static_cast<float>(args[1]->NumberValue(context).ToChecked()),
            static_cast<float>(args[2]->NumberValue(context).ToChecked()),
            static_cast<float>(args[3]->NumberValue(context).ToChecked())
    );
}

void Path2D::Rect(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    canvas_native_path_rect(
            *ptr->path_,
            static_cast<float>(args[0]->NumberValue(context).ToChecked()),
            static_cast<float>(args[1]->NumberValue(context).ToChecked()),
            static_cast<float>(args[2]->NumberValue(context).ToChecked()),
            static_cast<float>(args[3]->NumberValue(context).ToChecked())
    );
}

v8::Local<v8::FunctionTemplate> Path2D::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->Path2DTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, &Create);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "Path2D"));

    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
    auto tmpl = ctorTmpl->PrototypeTemplate();

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "addPath"), v8::FunctionTemplate::New(isolate, &AddPath)
    );
    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "arc"), v8::FunctionTemplate::New(isolate, &Arc)
    );
    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "arcTo"), v8::FunctionTemplate::New(isolate, &ArcTo)
    );
    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "bezierCurveTo"), v8::FunctionTemplate::New(isolate, &BezierCurveTo)
    );
    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "closePath"), v8::FunctionTemplate::New(isolate, &ClosePath)
    );
    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "ellipse"), v8::FunctionTemplate::New(isolate, &Ellipse)
    );
    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "lineTo"), v8::FunctionTemplate::New(isolate, &LineTo)
    );
    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "moveTo"), v8::FunctionTemplate::New(isolate, &MoveTo)
    );
    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "quadraticCurveTo"),
            v8::FunctionTemplate::New(isolate, &QuadraticCurveTo)
    );

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "rect"), v8::FunctionTemplate::New(isolate, &Rect)
    );

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "__toSVG"), v8::FunctionTemplate::New(isolate, &ToSVG)
    );

    cache->Path2DTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void Path2D::ToSVG(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ptr = GetPointer(args.This());
    rust::String string = canvas_native_path_to_string(*ptr->path_);
    args.GetReturnValue().Set(Helpers::ConvertToV8String(isolate, string.c_str()));
}

Path2D::~Path2D() {}

Path &Path2D::GetPath() {
    return *this->path_;
}

