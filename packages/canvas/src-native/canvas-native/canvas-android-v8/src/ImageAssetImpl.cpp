//
// Created by Osei Fortune on 22/03/2022.
//

#include "ImageAssetImpl.h"
#include "OnImageAssetLoadCallbackHolder.h"

ImageAssetImpl::ImageAssetImpl(rust::Box <ImageAsset> asset)
        : asset_(std::move(asset)) {}

void ImageAssetImpl::Init(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    global->Set(context, Helpers::ConvertToV8String(isolate, "ImageAsset"), ctor);
}

ImageAssetImpl *ImageAssetImpl::GetPointer(v8::Local<v8::Object> object) {
    if (object->InternalFieldCount() > 0) {
        auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<ImageAssetImpl *>(ptr);
    } else {
        auto isolate = object->GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = object->GetPrivate(context,
                                      v8::Private::New(isolate, Helpers::ConvertToV8String(isolate, "rustPtr")));
        if (!ptr.IsEmpty()) {
            auto ret = ptr.ToLocalChecked().As<v8::External>()->Value();
            if (ret == nullptr) {
                return nullptr;
            }
            return static_cast<ImageAssetImpl *>(ret);
        }
        return nullptr;
    }
}

void ImageAssetImpl::Create(const v8::FunctionCallbackInfo<v8::Value> &args) {
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
        ret->SetPrivate(context, v8::Private::New(isolate, Helpers::ConvertToV8String(isolate, "class_name")),
                        Helpers::ConvertToV8String(isolate, "ImageAsset"));

        ImageAssetImpl *asset = new ImageAssetImpl(std::move(canvas_native_image_asset_create()));
        auto ext = v8::External::New(isolate, asset);

        if (ret->InternalFieldCount() > 0) {
            ret->SetInternalField(0, ext);
        } else {
            ret->SetPrivate(context, v8::Private::New(isolate, Helpers::ConvertToV8String(isolate, "rustPtr")),
                            ext);
        }

        args.GetReturnValue().Set(ret);
    }
}

void ImageAssetImpl::GetWidth(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto self = info.Holder();
    auto ptr = GetPointer(self);
    info.GetReturnValue().Set(canvas_native_image_asset_width(*ptr->asset_));
}

void ImageAssetImpl::GetHeight(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto self = info.Holder();
    auto ptr = GetPointer(self);
    info.GetReturnValue().Set(canvas_native_image_asset_height(*ptr->asset_));
}

void ImageAssetImpl::GetError(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto self = info.Holder();
    auto ptr = GetPointer(self);
    auto has_error = canvas_native_image_asset_has_error(*ptr->asset_);
    if (!has_error) {
        info.GetReturnValue().Set(v8::Undefined(isolate));
    } else {
        auto err = canvas_native_image_asset_get_error(*ptr->asset_);
        info.GetReturnValue().Set(Helpers::ConvertToV8String(isolate, std::string(err)));
    }
}

void ImageAssetImpl::FlipX(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto self = args.Holder();
    auto ptr = GetPointer(self);
    canvas_native_image_asset_flip_x(*ptr->asset_);
}

void ImageAssetImpl::FlipY(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto self = args.Holder();
    auto ptr = GetPointer(self);
    canvas_native_image_asset_flip_y(*ptr->asset_);
}

void ImageAssetImpl::Scale(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto self = args.Holder();
    auto ptr = GetPointer(self);
    auto x = args[0]->Int32Value(context).FromMaybe(0);
    auto y = args[1]->Int32Value(context).FromMaybe(0);
    if (x > 0 && y > 0) {
        canvas_native_image_asset_scale(*ptr->asset_, static_cast<uint32_t>(x), static_cast<uint32_t>(y));
    }
}

void ImageAssetImpl::FromUrl(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();;
    auto ptr = GetPointer(args.Holder());
    if (args[0]->IsString()) {
        auto val = args[0]->ToString(context).ToLocalChecked();
        v8::String::Utf8Value utf8(isolate, val);
        rust::Str url(*utf8, utf8.length());
        auto done = canvas_native_image_asset_load_from_url(*ptr->asset_, url);
        args.GetReturnValue().Set(done);
    } else {
        args.GetReturnValue().Set(false);
    }
}

void ImageAssetImpl::FromUrlAsync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    auto resolver = v8::Promise::Resolver::New(context).ToLocalChecked();
    args.GetReturnValue().Set(resolver->GetPromise());
    if (args[0]->IsString()) {
        auto val = args[0]->ToString(context).ToLocalChecked();
        v8::String::Utf8Value utf8(isolate, val);
        rust::Str url(*utf8, utf8.length());
        auto callback = std::make_shared<OnImageAssetLoadCallbackHolder>(isolate, context, resolver);
        auto cache = Caches::Get(isolate);
        auto key = reinterpret_cast<intptr_t>(reinterpret_cast<intptr_t *>(callback.get()));
        cache->OnImageAssetLoadCallbackHolder_->Insert(key, callback);
        if(!cache->GetPerformingMicrotaskCheckpoint()){
            cache->SetPerformingMicrotaskCheckpoint(true);
            isolate->PerformMicrotaskCheckpoint();
            cache->SetPerformingMicrotaskCheckpoint(false);
        }
        canvas_native_image_asset_load_from_url_async(*ptr->asset_, url, key);
    };
}

void ImageAssetImpl::FromFile(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto self = args.Holder();
    auto ptr = GetPointer(self);
    if (args[0]->IsString()) {
        auto val = args[0]->ToString(context).ToLocalChecked();
        v8::String::Utf8Value utf8(isolate, val);
        rust::Str path(*utf8, utf8.length());
        auto done = canvas_native_image_asset_load_from_path(*ptr->asset_, path);
        args.GetReturnValue().Set(done);
    }
}

void ImageAssetImpl::FromFileAsync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto self = args.Holder();
    auto ptr = GetPointer(self);
    auto resolver = v8::Promise::Resolver::New(context).ToLocalChecked();
    args.GetReturnValue().Set(resolver->GetPromise());
    if (args[0]->IsString()) {
        auto val = args[0]->ToString(context).ToLocalChecked();
        v8::String::Utf8Value utf8(isolate, val);
        rust::Str path(*utf8, utf8.length());
        auto callback = std::make_shared<OnImageAssetLoadCallbackHolder>(isolate, context, resolver);
        auto cache = Caches::Get(isolate);
        auto key = reinterpret_cast<intptr_t>(reinterpret_cast<intptr_t *>(callback.get()));
        cache->OnImageAssetLoadCallbackHolder_->Insert(key, callback);
        canvas_native_image_asset_load_from_path_async(*ptr->asset_, path, key);
    };
}

void ImageAssetImpl::FromBytes(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto self = args.Holder();
    auto ptr = GetPointer(self);
    if (args[0]->IsUint8Array() || args[0]->IsUint8ClampedArray()) {
        auto buf = args[0].As<v8::TypedArray>();
        auto offset = buf->ByteOffset();
        auto size = buf->Length();
        auto store = buf->Buffer()->GetBackingStore();
        auto data = static_cast<uint8_t *>(store->Data()) + offset;
        auto buffer = rust::Slice<const uint8_t>(data, size);
        auto done = canvas_native_image_asset_load_from_raw(*ptr->asset_, buffer);
        args.GetReturnValue().Set(done);
    }
}

void ImageAssetImpl::FromBytesAsync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto self = args.Holder();
    auto ptr = GetPointer(self);
    auto resolver = v8::Promise::Resolver::New(context).ToLocalChecked();
    args.GetReturnValue().Set(resolver->GetPromise());
    if ((args[0]->IsUint8Array() || args[0]->IsUint8ClampedArray())) {
        auto callback = std::make_shared<OnImageAssetLoadCallbackHolder>(isolate, context, resolver);

        auto cache = Caches::Get(isolate);
        auto key = reinterpret_cast<intptr_t>(reinterpret_cast<intptr_t *>(callback.get()));
        cache->OnImageAssetLoadCallbackHolder_->Insert(key, callback);

        auto buf = args[0].As<v8::TypedArray>();
        auto offset = buf->ByteOffset();
        auto size = buf->Length();
        auto store = buf->Buffer()->GetBackingStore();
        auto data = static_cast<uint8_t *>(store->Data()) + offset;
        auto buffer = rust::Slice<const uint8_t>(data, size);
        canvas_native_image_asset_load_from_raw_async(*ptr->asset_, buffer, key);
    };
}

void ImageAssetImpl::Save(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto self = args.Holder();
    auto ptr = GetPointer(self);
    auto path = Helpers::ConvertFromV8String(isolate, args[0]->ToString(context).ToLocalChecked());
    auto format = args[1]->Int32Value(context).FromMaybe(0);
    auto done = canvas_native_image_asset_save_path(*ptr->asset_, path, format);
    args.GetReturnValue().Set(done);
}

void ImageAssetImpl::SaveAsync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto self = args.Holder();
    auto ptr = GetPointer(self);
    auto resolver = v8::Promise::Resolver::New(context).ToLocalChecked();
    args.GetReturnValue().Set(resolver->GetPromise());
    auto path = Helpers::ConvertFromV8String(isolate, args[0]->ToString(context).ToLocalChecked());
    auto format = args[1]->Int32Value(context).FromMaybe(0);
    auto callback = std::make_shared<OnImageAssetLoadCallbackHolder>(isolate, context, resolver);
    auto cache = Caches::Get(isolate);
    auto key = reinterpret_cast<intptr_t>(reinterpret_cast<intptr_t *>(callback.get()));
    cache->OnImageAssetLoadCallbackHolder_->Insert(key, callback);
    canvas_native_image_asset_save_path_async(*ptr->asset_, path, format, key);
}

v8::Local<v8::Function> ImageAssetImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->ImageAssetCtor.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    auto context = isolate->GetCurrentContext();

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, &Create);
    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "ImageAsset"));

    auto tmpl = ctorTmpl->InstanceTemplate();

    tmpl->SetInternalFieldCount(1);

    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "width"),
            &GetWidth
    );

    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "height"),
            &GetHeight
    );

    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "error"),
            &GetError
    );

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "flipX"), v8::FunctionTemplate::New(isolate, &FlipX)
    );
    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "flipY"), v8::FunctionTemplate::New(isolate, &FlipY)
    );
    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "scale"), v8::FunctionTemplate::New(isolate, &Scale)
    );
    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "loadFromUrlSync"), v8::FunctionTemplate::New(isolate, &FromUrl)
    );
    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "loadFromUrl"), v8::FunctionTemplate::New(isolate, &FromUrlAsync)
    );
    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "loadFileSync"),
            v8::FunctionTemplate::New(isolate, &FromFile)
    );

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "loadFile"), v8::FunctionTemplate::New(isolate, &FromFileAsync)
    );


    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "loadFromBytesSync"),
            v8::FunctionTemplate::New(isolate, &FromBytes)
    );

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "loadFromBytes"),
            v8::FunctionTemplate::New(isolate, &FromBytesAsync)
    );

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "saveSync"),
            v8::FunctionTemplate::New(isolate, &Save)
    );

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "save"),
            v8::FunctionTemplate::New(isolate, &SaveAsync)
    );

    auto func = ctorTmpl->GetFunction(context).ToLocalChecked();


    cache->ImageAssetCtor = std::make_unique<v8::Persistent<v8::Function>>(isolate, func);
    return func;
}

ImageAsset &ImageAssetImpl::GetImageAsset() {
    return *this->asset_;
}