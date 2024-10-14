//
// Created by Osei Fortune on 25/02/2023.
//

#include "CanvasJSIModule.h"
#include "JSICallback.h"
#include "JSIRuntime.h"
#include "JSIReadFileCallback.h"
#include "Helpers.h"

struct GLOptions {
    int32_t version;
    bool alpha;
    bool antialias;
    bool depth;
    bool failIfMajorPerformanceCaveat;
    int32_t powerPreference;
    bool premultipliedAlpha;
    bool preserveDrawingBuffer;
    bool stencil;
    bool desynchronized;
    bool xrCompatible;

public:
    GLOptions() {
        this->version = 0;
        this->alpha = true;
        this->antialias = true;
        this->depth = true;
        this->failIfMajorPerformanceCaveat = false;
        this->powerPreference = 0;
        this->premultipliedAlpha = true;
        this->preserveDrawingBuffer = false;
        this->stencil = false;
        this->desynchronized = false;
        this->xrCompatible = false;
    }

    void parseGLOptions(const v8::FunctionCallbackInfo<v8::Value> &args) {
        auto configValue = args[0];

        if (!(!configValue->IsNullOrUndefined() && configValue->IsObject())) {
            return;
        }

        auto isolate = args.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto config = configValue.As<v8::Object>();

        v8::Local<v8::Value> versionValue;

        config->Get(context, ConvertToV8String(isolate, "version")).ToLocal(&versionValue);

        if (!versionValue.IsEmpty() && versionValue->IsInt32()) {
            versionValue->Int32Value(context).To(&this->version);
        }

        v8::Local<v8::Value> alphaValue;

        config->Get(context, ConvertToV8String(isolate, "alpha")).ToLocal(&alphaValue);
        if (!alphaValue.IsEmpty() && alphaValue->IsBoolean()) {
            this->alpha = alphaValue->BooleanValue(isolate);
        }

        v8::Local<v8::Value> antialiasValue;
        config->Get(context, ConvertToV8String(isolate, "antialias")).ToLocal(
                &antialiasValue);
        if (!antialiasValue.IsEmpty() && antialiasValue->IsBoolean()) {
            this->antialias = antialiasValue->BooleanValue(isolate);
        }

        v8::Local<v8::Value> failIfMajorPerformanceCaveatValue;
        config->Get(context, ConvertToV8String(isolate, "failIfMajorPerformanceCaveat")).ToLocal(
                &failIfMajorPerformanceCaveatValue);
        if (!failIfMajorPerformanceCaveatValue.IsEmpty() &&
            failIfMajorPerformanceCaveatValue->IsBoolean()) {
            this->failIfMajorPerformanceCaveat = failIfMajorPerformanceCaveatValue->BooleanValue(
                    isolate);
        }

        v8::Local<v8::Value> powerPreferenceValue;
        config->Get(context, ConvertToV8String(isolate, "powerPreference")).ToLocal(
                &powerPreferenceValue);
        if (!powerPreferenceValue.IsEmpty() && powerPreferenceValue->IsInt32()) {
            powerPreferenceValue->Int32Value(context).To(&this->powerPreference);
        }

        v8::Local<v8::Value> premultipliedAlphaValue;
        config->Get(context,
                    ConvertToV8String(isolate, "premultipliedAlpha")).ToLocal(
                &premultipliedAlphaValue);
        if (!premultipliedAlphaValue.IsEmpty() && premultipliedAlphaValue->IsBoolean()) {
            this->premultipliedAlpha = premultipliedAlphaValue->BooleanValue(isolate);
        }

        v8::Local<v8::Value> preserveDrawingBufferValue;
        config->Get(context,
                    ConvertToV8String(isolate, "preserveDrawingBuffer")).ToLocal(
                &preserveDrawingBufferValue);
        if (!preserveDrawingBufferValue.IsEmpty() && preserveDrawingBufferValue->IsBoolean()) {
            this->preserveDrawingBuffer = preserveDrawingBufferValue->BooleanValue(isolate);
        }

        v8::Local<v8::Value> stencilValue;
        config->Get(context, ConvertToV8String(isolate, "stencil")).ToLocal(&stencilValue);
        if (!stencilValue.IsEmpty() && stencilValue->IsBoolean()) {
            this->stencil = stencilValue->BooleanValue(isolate);
        }

        v8::Local<v8::Value> desynchronizedValue;
        config->Get(context, ConvertToV8String(isolate, "desynchronized")).ToLocal(
                &desynchronizedValue);
        if (!desynchronizedValue.IsEmpty() && desynchronizedValue->IsBoolean()) {
            this->desynchronized = desynchronizedValue->BooleanValue(isolate);
        }

        v8::Local<v8::Value> xrCompatibleValue;
        config->Get(context, ConvertToV8String(isolate, "xrCompatible")).ToLocal(
                &xrCompatibleValue);
        if (!xrCompatibleValue.IsEmpty() && xrCompatibleValue->IsBoolean()) {
            this->xrCompatible = xrCompatibleValue->BooleanValue(isolate);
        }
    }
};

void CanvasJSIModule::install(v8::Isolate *isolate) {

    auto context = isolate->GetCurrentContext();
    auto global = context->Global();

    if (!global->
            HasOwnProperty(context, ConvertToV8String(isolate, "CanvasJSIModule")).FromMaybe(
            false)) {


        v8::Locker locker(isolate);
        v8::Isolate::Scope isolate_scope(isolate);
        v8::HandleScope handle_scope(isolate);

        auto v8Global = context->Global();

        auto canvasMod = v8::Object::New(isolate);
        TextDecoderImpl::Init(canvasMod, isolate);
        TextEncoderImpl::Init(canvasMod, isolate);
        Path2D::Init(canvasMod, isolate);
        ImageDataImpl::Init(canvasMod, isolate);
        ImageAssetImpl::Init(canvasMod, isolate);
        ImageBitmapImpl::Init(canvasMod, isolate);
        CanvasGradient::Init(canvasMod, isolate);
        CanvasPattern::Init(canvasMod, isolate);
        MatrixImpl::Init(canvasMod, isolate);
        TextMetricsImpl::Init(canvasMod, isolate);


        GPUImpl::Init(canvasMod, isolate);
        GPUSupportedLimitsImpl::Init(canvasMod, isolate);
        GPUDeviceImpl::Init(canvasMod, isolate);
        GPUQueueImpl::Init(canvasMod, isolate);

        v8Global->Set(context, ConvertToV8String(isolate, "CanvasModule"), canvasMod).FromJust();
        canvasMod->Set(context, ConvertToV8String(isolate, "create2DContext"),
                       v8::FunctionTemplate::New(isolate, &Create2DContext)->GetFunction(
                               context).ToLocalChecked()).FromJust();
        canvasMod->Set(context, ConvertToV8String(isolate, "createImageBitmap"),
                       v8::FunctionTemplate::New(isolate, &CreateImageBitmap)->GetFunction(
                               context).ToLocalChecked()).FromJust();
        canvasMod->Set(context, ConvertToV8String(isolate, "create2DContextWithPointer"),
                       v8::FunctionTemplate::New(isolate, &Create2DContextWithPointer)->GetFunction(
                               context).ToLocalChecked()).FromJust();

        canvasMod->Set(context, ConvertToV8String(isolate, "createWebGPUContextWithPointer"),
                       v8::FunctionTemplate::New(isolate,
                                                 &CreateWebGPUContextWithPointer)->GetFunction(
                               context).ToLocalChecked()).FromJust();


        canvasMod->Set(context, ConvertToV8String(isolate, "readFile"),
                       v8::FunctionTemplate::New(isolate, &ReadFile)->GetFunction(
                               context).ToLocalChecked()).FromJust();

        canvasMod->Set(context, ConvertToV8String(isolate, "createWebGLContext"),
                       v8::FunctionTemplate::New(isolate, &CreateWebGLContext)->GetFunction(
                               context).ToLocalChecked()).FromJust();

        canvasMod->Set(context, ConvertToV8String(isolate, "createWebGL2Context"),
                       v8::FunctionTemplate::New(isolate, &CreateWebGL2Context)->GetFunction(
                               context).ToLocalChecked()).FromJust();


        canvasMod->Set(context, ConvertToV8String(isolate, "__addFontFamily"),
                       v8::FunctionTemplate::New(isolate, &AddFontFamily)->GetFunction(
                               context).ToLocalChecked()).FromJust();

        global->Set(context,
                    ConvertToV8String(isolate, "CanvasModule"), canvasMod).FromJust();

    }
}

void CanvasJSIModule::AddFontFamily(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto aliasValue = args[0];
    auto familyValue = args[1];

    // todo improve
    if (familyValue->IsArray()) {
        auto family = familyValue.As<v8::Array>();
        auto len = family->Length();
        std::vector<std::string> buf;
        std::vector<const char *> buffer;
        buf.reserve(len);

        for (uint32_t i = 0; i < len; i++) {
            buf.emplace_back(
                    ConvertFromV8String(isolate, family->Get(context, i).ToLocalChecked()));
            buffer.emplace_back(buf[0].c_str());
        }

        if (aliasValue->IsString()) {
            auto alias = ConvertFromV8String(isolate, aliasValue);
            canvas_native_font_add_family(alias.c_str(), buffer.data(), buffer.size());
        } else {
            canvas_native_font_add_family(nullptr, buffer.data(), buffer.size());
        }

    }

}

void CanvasJSIModule::Create2DContext(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = args[0].As<v8::BigInt>()->Int64Value();

    auto context_2d = static_cast<CanvasRenderingContext2D *>((void *)ptr);

    auto ret = CanvasRenderingContext2DImpl::NewInstance(isolate, new CanvasRenderingContext2DImpl(
            context_2d));

    args.GetReturnValue().Set(ret);
}


struct Rect {
    float x;
    float y;
    float width;
    float height;
};

struct ImageBitmapData {
    char *error_;
    const ImageAsset *asset_;
    bool done_;
    v8::Isolate *isolate_;
    std::shared_ptr<v8::Persistent<v8::ArrayBufferView>> data_;

    ~ImageBitmapData() {
        if (error_ != nullptr) {
            canvas_native_string_destroy(error_);
            error_ = nullptr;
        }

        data_->Reset();
    }
};


void CanvasJSIModule::CreateImageBitmap(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto count = args.Length();
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto image = args[0];
    auto sx_or_options = args[1];
    auto sy = args[2];
    auto sw = args[3];
    auto sh = args[4];

    auto len = count - 1;
    auto cb = args[count - 1];

    if ((len == 1 && !image->IsObject()) || image->IsFunction()) {
        isolate->ThrowError("Illegal constructor");
        return;
    }

    Options options;

    if (len == 0) {
        isolate->ThrowError("Illegal constructor");
        return;
    }

    if (!cb->IsObject() && !cb->IsFunction()) {
        isolate->ThrowError("Illegal constructor");
        return;
    }


    if (image->IsNullOrUndefined()) {
        v8::Local<v8::Value> ret[2] = {ConvertToV8String(isolate, "Failed to load image"),
                                       v8::Null(isolate)};

        cb.As<v8::Function>()->Call(context, context->Global(), 2, ret);
        return;
    }

    if (len >= 4 && (sw->IsNumber() && sw->IsNumber() == 0)) {

        v8::Local<v8::Value> ret[2] = {ConvertToV8String(isolate,
                                                         "Failed to execute 'createImageBitmap' : The crop rect width is 0"),
                                       v8::Null(isolate)};

        cb.As<v8::Function>()->Call(context, context->Global(), 2, ret);

        return;
    }
    if (len >= 5 && (sh->IsNumber() && sh->IsNumber() == 0)) {
        v8::Local<v8::Value> ret[2] = {ConvertToV8String(isolate,
                                                         "Failed to execute 'createImageBitmap' : The crop rect height is 0"),
                                       v8::Null(isolate)};

        cb.As<v8::Function>()->Call(context, context->Global(), 2, ret);
        return;
    }

    if (image->IsObject()) {
        auto imageObject = image.As<v8::Object>();
        auto isArrayBuffer = imageObject->IsArrayBuffer();
        auto IsArrayBufferView = imageObject->IsArrayBufferView();
        std::optional<Rect> rect = std::nullopt;

        if (isArrayBuffer || IsArrayBufferView) {
            if ((len == 1 || len == 2) || (len == 5 || len == 6)) {
                if (len == 2) {
                    options = ImageBitmapImpl::HandleOptions(isolate, args[1]);
                }

                if (len == 6) {
                    options = ImageBitmapImpl::HandleOptions(isolate, args[5]);
                }

                auto cbFunc = args[count - 1].As<v8::Function>();

                if (isArrayBuffer) {
                    auto arrayBuffer = imageObject.As<v8::ArrayBuffer>();
                    auto size = arrayBuffer->ByteLength();
                    imageObject = v8::Uint8Array::New(arrayBuffer, 0, size);
                }

                auto ta = imageObject.As<v8::ArrayBufferView>();

                auto image_bitmap_async_data = new ImageBitmapData();
                image_bitmap_async_data->isolate_ = isolate;
                image_bitmap_async_data->data_ = std::make_shared<v8::Persistent<v8::ArrayBufferView>>(
                        isolate, ta);

                auto callback = new AsyncCallback(isolate, cbFunc, [](bool done, void *data) {
                    if(data == nullptr){return;}
                    auto async_data = static_cast<AsyncCallback *>(data);
                    auto func = async_data->inner_.get();
                    if (func != nullptr && func->isolate_ != nullptr) {
                        v8::Isolate *isolate = func->isolate_;
                        v8::Locker locker(isolate);
                        v8::Isolate::Scope isolate_scope(isolate);
                        v8::HandleScope handle_scope(isolate);
                        v8::Local<v8::Function> callback = func->callback_.Get(
                                isolate);
                        v8::Local<v8::Context> context = callback->GetCreationContextChecked();
                        v8::Context::Scope context_scope(context);

                        if (func->data != nullptr) {
                            auto asset_data = static_cast<ImageBitmapData *>(func->data);

                            auto bitmap = new ImageBitmapImpl(asset_data->asset_);
                            auto external = v8::External::New(isolate, bitmap);

                            auto ret = ImageBitmapImpl::NewInstance(isolate, external);

                            v8::Local<v8::Value> args[2] = {
                                    v8::Null(isolate), ret};


                            callback->Call(context, context->Global(),
                                           2,
                                           args);  // ignore JS return value

                            delete asset_data;

                        } else {
                            v8::Local<v8::Value> args[1] = {
                                    v8::Null(isolate)};

                            callback->Call(context, context->Global(),
                                           1,
                                           args);  // ignore JS return value
                        }

                    }
                    delete static_cast<AsyncCallback *>(data);
                });

                callback->inner_->data = image_bitmap_async_data;
                callback->prepare();
                auto store = ta->Buffer()->GetBackingStore();
                auto offset = ta->ByteOffset();

                std::thread thread(
                        [callback, image_bitmap_async_data, offset, options, rect](
                                std::shared_ptr<v8::BackingStore> store) {

                            auto data = static_cast<uint8_t *>(store->Data()) + offset;
                            auto size = store->ByteLength();

                            auto asset = canvas_native_image_asset_create();
                            bool done;
                            if (rect.has_value()) {

                                done = canvas_native_image_bitmap_create_from_encoded_bytes_src_rect_with_output(
                                        data, size,
                                        rect.value().x,
                                        rect.value().y,
                                        rect.value().width,
                                        rect.value().height,
                                        options.flipY,
                                        options.premultiplyAlpha,
                                        options.colorSpaceConversion,
                                        options.resizeQuality,
                                        options.resizeWidth,
                                        options.resizeHeight, asset);
                            } else {
                                done = canvas_native_image_bitmap_create_from_encoded_bytes_with_output(
                                        data, size,
                                        options.flipY,
                                        options.premultiplyAlpha,
                                        options.colorSpaceConversion,
                                        options.resizeQuality,
                                        options.resizeWidth,
                                        options.resizeHeight, asset);
                            }
                            if (callback != nullptr) {
                                image_bitmap_async_data->asset_ = asset;
                                callback->execute(done);
                            }

                        }, std::move(store));
                thread.detach();

                return;
            }
        }
    }

    auto type = ObjectWrapperImpl::GetNativeType(image);

    if (len == 1 || len == 2) {
        if (len == 2) {
            options = ImageBitmapImpl::HandleOptions(isolate, args[1]);
        }

        v8::Local<v8::Value> retArgs[2];
        retArgs[0] = v8::Null(isolate);

        switch (type) {
            case NativeType::ImageAsset: {
                auto image_asset = ImageAssetImpl::GetPointer(image.As<v8::Object>());
                auto ret = canvas_native_image_bitmap_create_from_asset(
                        image_asset->GetImageAsset(),
                        options.flipY,
                        options.premultiplyAlpha,
                        options.colorSpaceConversion,
                        options.resizeQuality,
                        options.resizeWidth,
                        options.resizeHeight);


                auto bitmap = new ImageBitmapImpl(ret);
                auto data = v8::External::New(isolate, bitmap);

                auto object = ImageBitmapImpl::NewInstance(isolate, data);

                retArgs[1] = object;
            }
                break;
            case NativeType::ImageBitmap: {
                auto image_bitmap = ImageBitmapImpl::GetPointer(image.As<v8::Object>());
                auto ret = canvas_native_image_bitmap_create_from_asset(
                        image_bitmap->GetImageAsset(),
                        options.flipY,
                        options.premultiplyAlpha,
                        options.colorSpaceConversion,
                        options.resizeQuality,
                        options.resizeWidth,
                        options.resizeHeight);


                auto bitmap = new ImageBitmapImpl(ret);
                auto data = v8::External::New(isolate, bitmap);
                auto object = ImageBitmapImpl::NewInstance(isolate, data);

                retArgs[1] = object;

            }
                break;
            default:
                break;
        }

        cb.As<v8::Function>()->Call(context, context->Global(), 2, retArgs);

        return;
    } else if (len == 5 || len == 6) {

        if (len == 6) {
            options = ImageBitmapImpl::HandleOptions(isolate, args[5]);
        }

        v8::Local<v8::Value> retArgs[2];
        retArgs[0] = v8::Null(isolate);


        switch (type) {
            case NativeType::ImageAsset: {
                auto image_asset = ImageBitmapImpl::GetPointer(image.As<v8::Object>());
                auto ret = canvas_native_image_bitmap_create_from_asset_src_rect(
                        image_asset->GetImageAsset(),
                        (float) sx_or_options->NumberValue(context).ToChecked(),
                        (float) sy->NumberValue(context).ToChecked(),
                        (float) sw->NumberValue(context).ToChecked(),
                        (float) sh->NumberValue(context).ToChecked(),
                        options.flipY,
                        options.premultiplyAlpha,
                        options.colorSpaceConversion,
                        options.resizeQuality,
                        options.resizeWidth,
                        options.resizeHeight);


                auto bitmap = new ImageBitmapImpl(ret);
                auto data = v8::External::New(isolate, bitmap);
                auto object = ImageBitmapImpl::NewInstance(isolate, data);

                retArgs[1] = object;
            }
                break;
            case NativeType::ImageBitmap: {
                auto image_bitmap = ImageBitmapImpl::GetPointer(image.As<v8::Object>());
                auto ret = canvas_native_image_bitmap_create_from_asset_src_rect(
                        image_bitmap->GetImageAsset(),
                        (float) sx_or_options->NumberValue(context).ToChecked(),
                        (float) sy->NumberValue(context).ToChecked(),
                        (float) sw->NumberValue(context).ToChecked(),
                        (float) sh->NumberValue(context).ToChecked(),
                        options.flipY,
                        options.premultiplyAlpha,
                        options.colorSpaceConversion,
                        options.resizeQuality,
                        options.resizeWidth,
                        options.resizeHeight);


                auto bitmap = new ImageBitmapImpl(ret);
                auto data = v8::External::New(isolate, bitmap);
                auto object = ImageBitmapImpl::NewInstance(isolate, data);

                retArgs[1] = object;

            }
                break;
            default:
                break;
        }


        cb.As<v8::Function>()->Call(context, context->Global(), 2, retArgs);

        return;
    }


    args.GetReturnValue().SetUndefined();

}

void CanvasJSIModule::Create2DContextWithPointer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = args[0]->ToBigInt(context).ToLocalChecked()->Int64Value();

    auto context_2d = canvas_native_context_create_with_pointer(ptr);

    auto ret = CanvasRenderingContext2DImpl::NewInstance(isolate, new CanvasRenderingContext2DImpl(
            context_2d));
    args.GetReturnValue().Set(ret);
}

struct FileData {
    char *error_;
    U8Buffer* data;

    ~FileData() {
        if (error_ != nullptr) {
            canvas_native_string_destroy(error_);
            error_ = nullptr;
        }
    }
};

void CanvasJSIModule::ReadFile(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto file = ConvertFromV8String(isolate, args[0]);
    auto cbFunc = args[1].As<v8::Function>();


    auto callback = new AsyncCallback(isolate, cbFunc, [](bool done, void *data) {
        if(data == nullptr){return;}
        auto async_data = static_cast<AsyncCallback *>(data);
        auto func = async_data->inner_.get();
        if (func != nullptr && func->isolate_ != nullptr) {
            v8::Isolate *isolate = func->isolate_;
            v8::Locker locker(isolate);
            v8::Isolate::Scope isolate_scope(isolate);
            v8::HandleScope handle_scope(isolate);
            v8::Local<v8::Function> callback = func->callback_.Get(
                                                                   isolate);
            v8::Local<v8::Context> context = callback->GetCreationContextChecked();
            v8::Context::Scope context_scope(context);

            if (func->data != nullptr) {
                auto file_data = static_cast<FileData*>(func->data);

                v8::Local<v8::Value> args[2];

                if (done) {
                    args[0] = v8::Null(isolate);

                    auto buf = (void *) canvas_native_u8_buffer_get_bytes_mut(file_data->data);
                    auto size = (size_t) canvas_native_u8_buffer_get_length(file_data->data);

                    auto store = v8::ArrayBuffer::NewBackingStore(buf, size,
                                                                  [](void *data,
                                                                     size_t length,
                                                                     void *deleter_data) {
                                                                      if (deleter_data !=
                                                                          nullptr) {
                                                                          delete static_cast<FileData *>(deleter_data);

                                                                      }
                                                                  },
                                                                  func->data);

                    args[1] = v8::ArrayBuffer::New(isolate, std::move(store));
                } else {
                    auto error = file_data->error_;
                    args[0] = v8::Exception::Error(ConvertToV8String(isolate, error));
                    args[1] = v8::Null(isolate);
                    delete file_data;
                }

                v8::TryCatch tc(isolate);
                v8::Local<v8::Value> result;
                if (!callback->Call(context, context->Global(), 2, args).ToLocal(
                        &result)) {
                    if (tc.HasCaught()) {

//                        v8::Local<v8::Value> stack;
//                        bool success = tc.StackTrace(context).ToLocal(&stack);
//                        if (!success || stack.IsEmpty()) {
//                            if (!done) {
//                                delete async_data;
//                            }
//                            return;
//                        }
//
//                        v8::Local<v8::String> stackV8Str;
//                        success = stack->ToDetailString(context).ToLocal(&stackV8Str);
//                        if (!success || stackV8Str.IsEmpty()) {
//                            if (!done) {
//                                delete async_data;
//                            }
//                            return;
//                        }

                    }
                }

                delete async_data;

            }
        }
    });

    callback->prepare();

    std::thread thread(
            [callback, file]() {
                bool done = false;
                auto ret = canvas_native_helper_read_file(file.c_str());

                if (!canvas_native_helper_read_file_has_error(ret)) {
                    auto buf = canvas_native_helper_read_file_take_data(ret);
                    callback->inner_->data = new FileData {nullptr, buf};
                    done = true;
                } else {
                    auto error = canvas_native_helper_read_file_get_error(ret);
                    callback->inner_->data = new FileData {const_cast<char *>(error), nullptr};
                }
                canvas_native_helper_release(ret);
                callback->execute(done);
            });

    thread.detach();



}

void CanvasJSIModule::CreateWebGLContext(const v8::FunctionCallbackInfo<v8::Value> &args) {

    auto options = GLOptions();
    options.parseGLOptions(args);

    if (options.version != 1) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto count = args.Length();
    if (count == 6) {
        auto ctx = args[1].As<v8::BigInt>()->Int64Value();
        auto webgl = static_cast<WebGLState*>((void *)ctx);

        auto renderingContext = WebGLRenderingContext::NewInstance(isolate,
                                                                   new WebGLRenderingContext(
                                                                           webgl));

        args.GetReturnValue().Set(renderingContext);
        return;
    } else if (count == 7) {
        auto width = args[1]->NumberValue(context).ToChecked();
        auto height = args[2]->NumberValue(context).ToChecked();
        auto ctx = canvas_native_webgl_create_no_window(
                (int32_t) width,
                (int32_t) height,
                options.version,
                options.alpha,
                options.antialias,
                options.depth,
                options.failIfMajorPerformanceCaveat,
                options.powerPreference,
                options.premultipliedAlpha,
                options.preserveDrawingBuffer,
                options.stencil,
                options.desynchronized,
                options.xrCompatible,
                false
        );

        auto renderingContext = WebGLRenderingContext::NewInstance(isolate,
                                                                   new WebGLRenderingContext(
                                                                           ctx));

        args.GetReturnValue().Set(renderingContext);
        return;

    } else {
        auto width = (int32_t) args[1]->NumberValue(context).ToChecked();
        auto height = (int32_t) args[2]->NumberValue(context).ToChecked();

        auto ctx = canvas_native_webgl_create_no_window(
                width,
                height,
                options.version,
                options.alpha,
                options.antialias,
                options.depth,
                options.failIfMajorPerformanceCaveat,
                options.powerPreference,
                options.premultipliedAlpha,
                options.preserveDrawingBuffer,
                options.stencil,
                options.desynchronized,
                options.xrCompatible,
                false
        );

        auto renderingContext = WebGLRenderingContext::NewInstance(isolate,
                                                                   new WebGLRenderingContext(
                                                                           ctx));

        args.GetReturnValue().Set(renderingContext);
        return;
    }
}

void CanvasJSIModule::CreateWebGL2Context(const v8::FunctionCallbackInfo<v8::Value> &args) {


    auto options = GLOptions();
    options.parseGLOptions(args);

    if (options.version != 2) {
        args.GetReturnValue().SetNull();
        return;
    }


    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto count = args.Length();
    if (count == 6) {
        auto ctx = args[0].As<v8::BigInt>()->Int64Value();
        auto webgl = static_cast<WebGLState*>((void *)ctx);
        auto renderingContext = WebGL2RenderingContext::NewInstance(isolate,
                                                                    new WebGL2RenderingContext(
                                                                            webgl,
                                                                            WebGLRenderingVersion::V2));

        args.GetReturnValue().Set(renderingContext);
        return;

    } else if (count ==
               7) {
        auto width = args[1]->NumberValue(context).ToChecked();
        auto height = args[2]->NumberValue(context).ToChecked();
        auto ctx = canvas_native_webgl_create_no_window(
                (int32_t) width,
                (int32_t) height,
                options.version,
                options.alpha,
                options.antialias,
                options.depth,
                options.failIfMajorPerformanceCaveat,
                options.powerPreference,
                options.premultipliedAlpha,
                options.preserveDrawingBuffer,
                options.stencil,
                options.desynchronized,
                options.xrCompatible,
                false
        );
        auto renderingContext = WebGL2RenderingContext::NewInstance(isolate,
                                                                    new WebGL2RenderingContext(
                                                                            ctx,
                                                                            WebGLRenderingVersion::V2));

        args.GetReturnValue().Set(renderingContext);
        return;

    } else {
        auto width = (int32_t) args[1]->NumberValue(context).ToChecked();
        auto height = (int32_t) args[2]->NumberValue(context).ToChecked();
        auto ctx = canvas_native_webgl_create_no_window(
                width,
                height,
                options.version,
                options.alpha,
                options.antialias,
                options.depth,
                options.failIfMajorPerformanceCaveat,
                options.powerPreference,
                options.premultipliedAlpha,
                options.preserveDrawingBuffer,
                options.stencil,
                options.desynchronized,
                options.xrCompatible,
                false
        );

        auto renderingContext = WebGL2RenderingContext::NewInstance(isolate,
                                                                    new WebGL2RenderingContext(
                                                                            ctx,
                                                                            WebGLRenderingVersion::V2));

        args.GetReturnValue().Set(renderingContext);
        return;
    }
}

void
CanvasJSIModule::CreateWebGPUContextWithPointer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = args[0]->ToBigInt(context).ToLocalChecked()->Int64Value();

    auto wgpu = static_cast<CanvasGPUCanvasContext *>((void *) ptr);

    auto ret = GPUCanvasContextImpl::NewInstance(isolate, new GPUCanvasContextImpl(
            wgpu));
    args.GetReturnValue().Set(ret);
}
