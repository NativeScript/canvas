//
// Created by Osei Fortune on 22/03/2022.
//

#include "ImageAssetImpl.h"
#include "OnImageAssetLoadCallbackHolder.h"
#include "rust/cxx.h"
#include "canvas-android/src/lib.rs.h"
#include "Configuration.h"
#include "Bridge.h"

std::atomic_int64_t ImageAssetImpl::Count_ = {0};

ImageAssetImpl::ImageAssetImpl(rust::Box<ImageAsset> asset)
        : asset_(std::move(asset)) {}

ImageAssetImpl::~ImageAssetImpl() {
    auto isolate = v8::Isolate::GetCurrent();
    if (isolate != nullptr) {
        auto previous = canvas_native_image_asset_get_size(this->GetImageAsset());
        if (previous != 0) {
            isolate->AdjustAmountOfExternalAllocatedMemory(-previous);
        }
    }
}

void ImageAssetImpl::Init(v8::Isolate *isolate) {
    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    global->Set(context, Helpers::ConvertToV8String(isolate, "ImageAsset"),
                ctor->GetFunction(context).ToLocalChecked());
}

ImageAssetImpl *ImageAssetImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<ImageAssetImpl *>(ptr);
}

void ImageAssetImpl::Create(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
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
        Helpers::SetInstanceType(isolate, ret, ObjectType::ImageAsset);
        auto *asset = new ImageAssetImpl(std::move(canvas_native_image_asset_create()));
        AddWeakListener(isolate, ret, asset);
        args.GetReturnValue().Set(ret);
    }
}

void ImageAssetImpl::GetLoadingState(v8::Local<v8::String> name,
                                     const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    info.GetReturnValue().Set(ptr->IsLoading_);
}

void ImageAssetImpl::SetLoadingState(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                     const v8::PropertyCallbackInfo<void> &info) {
    auto ptr = GetPointer(info.This());
    auto isolate = info.GetIsolate();
    ptr->IsLoading_ = value->BooleanValue(isolate);
}

void ImageAssetImpl::GetAddr(v8::Local<v8::String> name,
                             const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    info.GetReturnValue().Set((double) canvas_native_image_asset_addr(*ptr->asset_));
}

void ImageAssetImpl::GetWidth(v8::Local<v8::String> name,
                              const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    info.GetReturnValue().Set(canvas_native_image_asset_width(*ptr->asset_));
}

void ImageAssetImpl::GetHeight(v8::Local<v8::String> name,
                               const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    info.GetReturnValue().Set(canvas_native_image_asset_height(*ptr->asset_));
}

void ImageAssetImpl::GetError(v8::Local<v8::String> name,
                              const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.This());
    auto has_error = canvas_native_image_asset_has_error(*ptr->asset_);
    if (!has_error) {
        info.GetReturnValue().Set(v8::Undefined(isolate));
    } else {
        auto error = canvas_native_image_asset_get_error(*ptr->asset_);
        info.GetReturnValue().Set(Helpers::ConvertToV8String(isolate, error.c_str()));
    }
}

void ImageAssetImpl::Scale(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    auto x = args[0]->Int32Value(context).FromMaybe(0);
    auto y = args[1]->Int32Value(context).FromMaybe(0);
    if (x > 0 && y > 0) {
        canvas_native_image_asset_scale(*ptr->asset_, static_cast<uint32_t>(x),
                                        static_cast<uint32_t>(y));
    }
}

inline int64_t GetBytesChange(int64_t old_bytes, int64_t new_bytes) {
    if (old_bytes > new_bytes) {
        return -(old_bytes - new_bytes);
    }

    return new_bytes - old_bytes;
}

void ImageAssetImpl::FromUrlSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ptr = GetPointer(args.This());
    if (Helpers::IsString(args[0])) {
        auto url = Helpers::GetString(isolate, args[0]);
        auto found = url.at(0);
        auto previous = canvas_native_image_asset_get_size(ptr->GetImageAsset());
        if (std::string(&found) == std::string("~")) {
            url.replace(0, 2, Configuration::GetAppBase() + "/");
        }
        auto done = canvas_native_image_asset_load_from_url(ptr->GetImageAsset(),
                                                            rust::Str(url.c_str(), url.size()));
        if (done) {
            auto new_bytes = canvas_native_image_asset_get_size(ptr->GetImageAsset());
            isolate->AdjustAmountOfExternalAllocatedMemory(GetBytesChange(previous, new_bytes));
        } else {
            isolate->AdjustAmountOfExternalAllocatedMemory(GetBytesChange(previous, 0));

        }
        args.GetReturnValue().Set(done);
    } else {
        args.GetReturnValue().Set(false);
    }
}

/*
void ImageAssetImpl::FromUrlCb(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    auto url = args[0];
    auto cb = args[1];
    if (Helpers::IsString(url) && cb->IsFunction()) {
        auto val = Helpers::GetString(isolate, url);
        auto found = val.at(0);
        if (std::string(&found) == std::string("~")) {
            val.replace(0, 2, Configuration::GetAppBase() + "/");
        }

        auto cbFunc = cb.As<v8::Function>();

         auto callback = std::make_shared<OnImageAssetLoadCallbackHolder>(isolate, context,
                                                                          cbFunc);

         auto cache = Caches::Get(isolate);

         int id = ++cache->taskId_;

         cache->OnImageAssetLoadCallbackHolder_->Insert(id, callback);

         auto asset = canvas_native_image_asset_shared_clone(*ptr->asset_);

         std::thread thread(
                 [](int fd, rust::Box<ImageAsset> ptr, const std::string &val, int id) {

                     auto done = canvas_native_image_asset_load_from_url(*ptr,
                                                                         rust::Str(val.c_str(),
                                                                                   val.size()));
                     auto ret = Caches::Task(1,
                                             id,
                                             done);

                     write(fd, &ret, sizeof(Caches::Task));

                 }, cache->GetAssetWriter(), std::move(asset), val, id);

         thread.detach();

    }
}

void ImageAssetImpl::FromUrlAsync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    auto url = args[0];
    auto resolver = v8::Promise::Resolver::New(context).ToLocalChecked();
    args.GetReturnValue().Set(resolver->GetPromise());
    if (Helpers::IsString(url)) {
        auto val = Helpers::GetString(isolate, url);
        auto found = val.at(0);
        if (std::string(&found) == std::string("~")) {
            val.replace(0, 2, Configuration::GetAppBase() + "/");
        }

        auto func = [](const v8::FunctionCallbackInfo<v8::Value> &info) {
            auto isolate = info.GetIsolate();
            auto context = isolate->GetCurrentContext();
            auto resolver = info.Data().As<v8::Promise::Resolver>();
            resolver->Resolve(context, info[0]);
        };

        auto cb = v8::Function::New(context, func, resolver).ToLocalChecked();

        auto callback = std::make_shared<OnImageAssetLoadCallbackHolder>(isolate, context, cb);

        std::thread thread([](ImageAssetImpl *ptr, const std::string &val,
                              const std::shared_ptr<OnImageAssetLoadCallbackHolder> &callback) {
            auto done = canvas_native_image_asset_load_from_url(*ptr->asset_,
                                                                rust::Str(val.c_str(),
                                                                          val.size()));
            if (callback != nullptr) {
                callback->done_ = done;
                callback->complete();
            }
        }, ptr, val, std::move(callback));

    }
}
*/

void ImageAssetImpl::FromFileSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ptr = GetPointer(args.This());
    if (Helpers::IsString(args[0])) {
        auto path = Helpers::GetString(isolate, args[0]);
        auto found = path.at(0);
        if (std::string(&found) == std::string("~")) {
            path = path.replace(0, 2, Configuration::GetAppBase() + "/");
        }
        auto previous = canvas_native_image_asset_get_size(ptr->GetImageAsset());
        auto done = canvas_native_image_asset_load_from_path(ptr->GetImageAsset(),
                                                             rust::Str(path.c_str(), path.size()));
        if (done) {
            auto new_bytes = canvas_native_image_asset_get_size(ptr->GetImageAsset());
            isolate->AdjustAmountOfExternalAllocatedMemory(GetBytesChange(previous, new_bytes));
        } else {
            isolate->AdjustAmountOfExternalAllocatedMemory(GetBytesChange(previous, 0));

        }
        args.GetReturnValue().Set(done);
    }
}

/*

void ImageAssetImpl::FromFileCb(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    auto file = args[0];
    auto cb = args[1];
    if (Helpers::IsString(file) && cb->IsFunction()) {
        auto path = Helpers::GetString(isolate, file);
        auto found = path.at(0);
        if (std::string(&found) == std::string("~")) {
            path.replace(0, 2, Configuration::GetAppBase() + "/");
        }

        auto cbFunc = cb.As<v8::Function>();

        auto callback = std::make_shared<OnImageAssetLoadCallbackHolder>(isolate, context,
                                                                         cbFunc);

        std::thread thread([](ImageAssetImpl *ptr, const std::string &path,
                              const std::shared_ptr<OnImageAssetLoadCallbackHolder> &callback) {
            auto done = canvas_native_image_asset_load_from_path(*ptr->asset_,
                                                                 rust::Str(path.c_str(),
                                                                           path.size()));
            if (callback != nullptr) {
                callback->done_ = done;
                callback->complete();
            }
        }, ptr, path, std::move(callback));

    }
}

void ImageAssetImpl::FromFileAsync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    auto file = args[0];
    auto resolver = v8::Promise::Resolver::New(context).ToLocalChecked();
    args.GetReturnValue().Set(resolver->GetPromise());
    if (Helpers::IsString(file)) {
        auto path = Helpers::GetString(isolate, file);
        auto found = path.at(0);
        if (std::string(&found) == std::string("~")) {
            path.replace(0, 2, Configuration::GetAppBase() + "/");
        }

        auto func = [](const v8::FunctionCallbackInfo<v8::Value> &info) {
            auto isolate = info.GetIsolate();
            auto context = isolate->GetCurrentContext();
            auto resolver = info.Data().As<v8::Promise::Resolver>();
            resolver->Resolve(context, info[0]);
        };

        auto cb = v8::Function::New(context, func, resolver).ToLocalChecked();

        auto callback = std::make_shared<OnImageAssetLoadCallbackHolder>(isolate, context, cb);

        std::thread thread([](ImageAssetImpl *ptr, const std::string &path,
                              const std::shared_ptr<OnImageAssetLoadCallbackHolder> &callback) {
            auto done = canvas_native_image_asset_load_from_path(*ptr->asset_,
                                                                 rust::Str(path.c_str(),
                                                                           path.size()));
            if (callback != nullptr) {
                callback->done_ = done;
                callback->complete();
            }
        }, ptr, path, std::move(callback));

    }
}

 */

void ImageAssetImpl::FromBytesSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ptr = GetPointer(args.This());
    if (args[0]->IsUint8Array() || args[0]->IsUint8ClampedArray()) {
        auto buf = args[0].As<v8::TypedArray>();
        auto offset = buf->ByteOffset();
        auto size = buf->Length();
        auto store = buf->Buffer()->GetBackingStore();
        auto data = static_cast<uint8_t *>(store->Data()) + offset;
        auto buffer = rust::Slice<const uint8_t>(data, size);
        auto previous = canvas_native_image_asset_get_size(ptr->GetImageAsset());
        auto done = canvas_native_image_asset_load_from_raw(*ptr->asset_, buffer);
        if (done) {
            auto new_bytes = canvas_native_image_asset_get_size(ptr->GetImageAsset());
            isolate->AdjustAmountOfExternalAllocatedMemory(GetBytesChange(previous, new_bytes));
        } else {
            isolate->AdjustAmountOfExternalAllocatedMemory(GetBytesChange(previous, 0));

        }
        args.GetReturnValue().Set(done);
    }
}

/*
void ImageAssetImpl::FromBytesCb(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    auto cb = args[1];
    if ((args[0]->IsUint8Array() || args[0]->IsUint8ClampedArray()) && cb->IsFunction()) {


        auto cbFunc = cb.As<v8::Function>();

        auto callback = std::make_shared<OnImageAssetLoadCallbackHolder>(isolate, context,
                                                                         cbFunc);

        auto buf = args[0].As<v8::TypedArray>();
        auto offset = buf->ByteOffset();
        auto size = buf->Length();
        auto store = buf->Buffer()->GetBackingStore();
        auto data = static_cast<uint8_t *>(store->Data()) + offset;
        auto buffer = rust::Slice<const uint8_t>(data, size);


        std::thread thread([](ImageAssetImpl *ptr, const rust::Slice<const uint8_t> buffer,
                              const std::shared_ptr<OnImageAssetLoadCallbackHolder> &callback) {
            auto done = canvas_native_image_asset_load_from_raw(*ptr->asset_, buffer);
            if (callback != nullptr) {
                callback->done_ = done;
                callback->complete();
            }
        }, ptr, buffer, std::move(callback));
    }
}

void ImageAssetImpl::FromBytesAsync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    auto resolver = v8::Promise::Resolver::New(context).ToLocalChecked();
    args.GetReturnValue().Set(resolver->GetPromise());
    if ((args[0]->IsUint8Array() || args[0]->IsUint8ClampedArray())) {


        auto buffer = Helpers::GetTypedArrayData<const uint8_t>(args[0].As<v8::TypedArray>());

        auto func = [](const v8::FunctionCallbackInfo<v8::Value> &info) {
            auto isolate = info.GetIsolate();
            auto context = isolate->GetCurrentContext();
            auto resolver = info.Data().As<v8::Promise::Resolver>();
            resolver->Resolve(context, info[0]);
        };

        auto cb = v8::Function::New(context, func, resolver).ToLocalChecked();

        auto callback = std::make_shared<OnImageAssetLoadCallbackHolder>(isolate, context, cb);

        std::thread thread([](ImageAssetImpl *ptr, const rust::Slice<const uint8_t> buffer,
                              const std::shared_ptr<OnImageAssetLoadCallbackHolder> &callback) {
            auto done = canvas_native_image_asset_load_from_raw(*ptr->asset_, buffer);
            if (callback != nullptr) {
                callback->done_ = done;
                callback->complete();
            }
        }, ptr, buffer, std::move(callback));
    };
}

 */

void ImageAssetImpl::SaveSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    auto path = Helpers::GetString(isolate, args[0]);
    auto found = path.at(0);
    if (std::string(&found) == std::string("~")) {
        path.replace(0, 2, Configuration::GetAppBase() + "/");
    }
    auto format = args[1]->Int32Value(context).FromMaybe(0);
    auto done = canvas_native_image_asset_save_path(*ptr->asset_,
                                                    rust::Str(path.c_str(), path.size()), format);
    args.GetReturnValue().Set(done);
}

/*

void ImageAssetImpl::SaveCb(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    auto val = args[0];
    auto formatVal = args[1];
    auto cb = args[2];
    if (Helpers::IsString(val) && Helpers::IsNumber(formatVal) && cb->IsFunction()) {
        auto path = Helpers::GetString(isolate, val->ToString(context).ToLocalChecked());
        auto found = path.at(0);
        if (std::string(&found) == std::string("~")) {
            path.replace(0, 2, Configuration::GetAppBase() + "/");
        }
        auto format = formatVal->Int32Value(context).FromMaybe(0);

        auto cbFunc = cb.As<v8::Function>();

        auto callback = std::make_shared<OnImageAssetLoadCallbackHolder>(isolate, context,
                                                                         cbFunc);

        std::thread thread([](ImageAssetImpl *ptr, const std::string &path, int format,
                              const std::shared_ptr<OnImageAssetLoadCallbackHolder> &callback) {
            auto done = canvas_native_image_asset_save_path(*ptr->asset_,
                                                            rust::Str(path.c_str(),
                                                                      path.size()), format);
            if (callback != nullptr) {
                callback->done_ = done;
                callback->complete();
            }
        }, ptr, path, format, std::move(callback));

    }
}

void ImageAssetImpl::SaveAsync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    auto val = args[0];
    auto formatVal = args[1];
    auto resolver = v8::Promise::Resolver::New(context).ToLocalChecked();
    args.GetReturnValue().Set(resolver->GetPromise());
    if (Helpers::IsString(val) && Helpers::IsNumber(formatVal)) {
        auto path = Helpers::GetString(isolate, val->ToString(context).ToLocalChecked());
        auto found = path.at(0);
        if (std::string(&found) == std::string("~")) {
            path.replace(0, 2, Configuration::GetAppBase() + "/");
        }
        auto format = formatVal->Int32Value(context).FromMaybe(0);


        auto func = [](const v8::FunctionCallbackInfo<v8::Value> &info) {
            auto isolate = info.GetIsolate();
            auto context = isolate->GetCurrentContext();
            auto resolver = info.Data().As<v8::Promise::Resolver>();
            resolver->Resolve(context, info[0]);
        };

        auto cb = v8::Function::New(context, func, resolver).ToLocalChecked();

        auto callback = std::make_shared<OnImageAssetLoadCallbackHolder>(isolate, context, cb);

        std::thread thread([](ImageAssetImpl *ptr, const std::string &path, int format,
                              const std::shared_ptr<OnImageAssetLoadCallbackHolder> &callback) {
            auto done = canvas_native_image_asset_save_path(*ptr->asset_,
                                                            rust::Str(path.c_str(),
                                                                      path.size()), format);
            if (callback != nullptr) {
                callback->done_ = done;
                callback->complete();
            }
        }, ptr, path, format, std::move(callback));

    }
}

 */

void ImageAssetImpl::Dispose(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    if (!ptr->IsLoading_.load() && ptr->entry_ != nullptr) {
        delete ptr->entry_;
        ptr->entry_ = nullptr;
    }
}


v8::Local<v8::FunctionTemplate> ImageAssetImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->ImageAssetTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, &Create);

    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "ImageAsset"));

    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);

    auto tmpl = ctorTmpl->PrototypeTemplate();

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

    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "__addr"),
            &GetAddr
    );


    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "__isLoading"),
            &GetLoadingState,
            &SetLoadingState
    );

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "__dispose"),
            v8::FunctionTemplate::New(isolate, &Dispose)
    );

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "scale"),
            v8::FunctionTemplate::New(isolate, &Scale)
    );
    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "loadUrlSync"),
            v8::FunctionTemplate::New(isolate, &FromUrlSync)
    );
    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "loadFileSync"),
            v8::FunctionTemplate::New(isolate, &FromFileSync)
    );

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "loadBytesSync"),
            v8::FunctionTemplate::New(isolate, &FromBytesSync)
    );

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "saveSync"),
            v8::FunctionTemplate::New(isolate, &SaveSync)
    );


    /*
    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "loadUrl"),
            v8::FunctionTemplate::New(isolate, &FromUrlCb)
    );

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "loadUrlAsync"),
            v8::FunctionTemplate::New(isolate, &FromUrlAsync)
    );


    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "loadFile"),
            v8::FunctionTemplate::New(isolate, &FromFileCb)
    );

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "loadFileAsync"),
            v8::FunctionTemplate::New(isolate, &FromFileAsync)
    );


    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "loadBytes"),
            v8::FunctionTemplate::New(isolate, &FromBytesCb)
    );

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "loadBytesAsync"),
            v8::FunctionTemplate::New(isolate, &FromBytesAsync)
    );


    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "save"),
            v8::FunctionTemplate::New(isolate, &SaveCb)
    );

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "saveAsync"),
            v8::FunctionTemplate::New(isolate, &SaveAsync)
    );

    */


    cache->ImageAssetTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate,
                                                                                   ctorTmpl);
    return ctorTmpl;
}

ImageAsset &ImageAssetImpl::GetImageAsset() {
    return *this->asset_;
}

