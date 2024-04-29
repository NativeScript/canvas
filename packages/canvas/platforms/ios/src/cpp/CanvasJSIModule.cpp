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
        CanvasGradient::Init(canvasMod, isolate);
        CanvasPattern::Init(canvasMod, isolate);
        MatrixImpl::Init(canvasMod, isolate);
        TextMetricsImpl::Init(canvasMod, isolate);

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
    auto width = (float) args[1]->NumberValue(context).ToChecked();
    auto height = (float) args[2]->NumberValue(context).ToChecked();
    auto density = (float) args[3]->NumberValue(context).ToChecked();
    auto samples = (int) args[4]->NumberValue(context).ToChecked();
    auto alpha = (bool) args[5]->BooleanValue(isolate);
    auto font_color = (int) args[6]->NumberValue(context).ToChecked();
    auto ppi = (float) args[7]->NumberValue(context).ToChecked();
    auto direction = (int) args[8]->NumberValue(context).ToChecked();

    auto context_2d = canvas_native_context_create_gl(width, height, density,
                                                      ptr,
                                                      samples, alpha,
                                                      font_color, ppi, direction);

    auto ret = CanvasRenderingContext2DImpl::NewInstance(isolate, new CanvasRenderingContext2DImpl(
            context_2d));

    args.GetReturnValue().Set(ret);
}

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
                                       v8::Undefined(isolate)};

        cb.As<v8::Function>()->Call(context, context->Global(), 2, ret);
        return;
    }

    if (len >= 4 && (sw->IsNumber() && sw->IsNumber() == 0)) {

        v8::Local<v8::Value> ret[2] = {ConvertToV8String(isolate,
                                                         "Failed to execute 'createImageBitmap' : The crop rect width is 0"),
                                       v8::Undefined(isolate)};

        cb.As<v8::Function>()->Call(context, context->Global(), 2, ret);

        return;
    }
    if (len >= 5 && (sh->IsNumber() && sh->IsNumber() == 0)) {
        v8::Local<v8::Value> ret[2] = {ConvertToV8String(isolate,
                                                         "Failed to execute 'createImageBitmap' : The crop rect height is 0"),
                                       v8::Undefined(isolate)};

        cb.As<v8::Function>()->Call(context, context->Global(), 2, ret);
        return;
    }


    if (image->IsObject()) {
        auto imageObject = image.As<v8::Object>();
        auto isArrayBuffer = imageObject->IsArrayBuffer();
        auto IsArrayBufferView = imageObject->IsArrayBufferView();
        if (isArrayBuffer || IsArrayBufferView) {

            if (len == 1 || len == 2) {
                if (len == 2) {
                    options = ImageBitmapImpl::HandleOptions(isolate, args[1]);
                }

                auto asset = canvas_native_image_asset_create();

                auto shared_asset = canvas_native_image_asset_shared_clone(asset);

                auto ret = new ImageBitmapImpl(asset);


                auto cbFunc = args[count - 1].As<v8::Function>();
                auto data = v8::External::New(isolate, ret);

                auto jsi_callback = new JSICallback(isolate, cbFunc, data);


#ifdef __ANDROID__
                ALooper_addFd(jsi_callback->looper_,
                              jsi_callback->fd_[0],
                              ALOOPER_POLL_CALLBACK,
                              ALOOPER_EVENT_INPUT,
                              [](int fd, int events,
                                 void *data) {
                                  auto cb = static_cast<JSICallback *>(data);
                                  bool done;
                                  read(fd, &done,
                                       sizeof(bool));


                                  v8::Isolate *isolate = cb->isolate_;
                                  v8::Locker locker(isolate);
                                  v8::Isolate::Scope isolate_scope(isolate);
                                  v8::HandleScope handle_scope(isolate);
                                  v8::Local<v8::Function> callback = cb->callback_->Get(isolate);
                                  v8::Local<v8::External> cbData = cb->data_->Get(
                                          isolate).As<v8::External>();
                                  v8::Local<v8::Context> context = callback->GetCreationContextChecked();
                                  v8::Context::Scope context_scope(context);

                                  auto ret = ImageBitmapImpl::NewInstance(isolate, cbData);

                                  v8::Local<v8::Value> args[2];

                                  if (done) {
                                      args[0] = v8::Null(isolate);
                                      args[1] = ret;

                                  } else {
                                      args[0] = v8::Exception::Error(
                                              ConvertToV8String(isolate, "Failed to load image"));
                                      args[1] = v8::Null(isolate);
                                  }


                                  callback->Call(context, context->Global(), 2, args);

                                  delete static_cast<JSICallback *>(data);
                                  return 0;
                              }, jsi_callback);

                ALooper_wake(jsi_callback->looper_);

                if (isArrayBuffer) {
                    auto arrayBuffer = imageObject.As<v8::ArrayBuffer>();
                    auto store = arrayBuffer->GetBackingStore();
                    auto dataBuffer = (uint8_t *) store->Data();
                    v8::Global<v8::ArrayBuffer> ab(isolate, arrayBuffer);
                    std::thread thread(
                            [&dataBuffer, jsi_callback, &options, store, shared_asset](
                                    v8::Global<v8::ArrayBuffer> ab, size_t size) {


                                auto done = canvas_native_image_bitmap_create_from_encoded_bytes_with_output(
                                        dataBuffer, size,
                                        options.flipY,
                                        options.premultiplyAlpha,
                                        options.colorSpaceConversion,
                                        options.resizeQuality,
                                        options.resizeWidth,
                                        options.resizeHeight,
                                        shared_asset);

                                canvas_native_image_asset_destroy(shared_asset);

                                write(jsi_callback->fd_[1],
                                      &done,
                                      sizeof(bool));


                            }, std::move(ab), arrayBuffer->ByteLength());

                    thread.detach();
                    return;
                }


                auto ta = imageObject.As<v8::ArrayBufferView>();


                auto array = ta->Buffer();
                auto offset = ta->ByteOffset();
                auto size = ta->ByteLength();
                auto data_ptr = static_cast<uint8_t *>(array->GetBackingStore()->Data()) + offset;


                v8::Global<v8::ArrayBufferView> ab(isolate, ta);

                std::thread thread(
                        [jsi_callback, &options, shared_asset, data_ptr, size](
                                v8::Global<v8::ArrayBufferView> ab) {

                            auto done = canvas_native_image_bitmap_create_from_encoded_bytes_with_output(
                                    data_ptr, size,
                                    options.flipY,
                                    options.premultiplyAlpha,
                                    options.colorSpaceConversion,
                                    options.resizeQuality,
                                    options.resizeWidth,
                                    options.resizeHeight,
                                    shared_asset);

                            canvas_native_image_asset_destroy(shared_asset);

                            write(jsi_callback->fd_[1],
                                  &done,
                                  sizeof(bool));


                        }, std::move(ab));
                thread.detach();

#endif


#ifdef __APPLE__

                auto current_queue = new NSOperationQueueWrapper(true);


                if (isArrayBuffer) {
                    auto arrayBuffer = imageObject.As<v8::ArrayBuffer>();
                    auto store = arrayBuffer->GetBackingStore();
                    auto dataBuffer = (uint8_t *) store->Data();
                    v8::Global<v8::ArrayBuffer> ab(isolate, arrayBuffer);

                    std::thread thread(
                            [&dataBuffer, jsi_callback, &options, store, shared_asset, current_queue](
                                    v8::Global<v8::ArrayBuffer> ab, size_t size
                            ) {

                                auto done = canvas_native_image_bitmap_create_from_encoded_bytes_with_output(
                                        dataBuffer, size,
                                        options.flipY,
                                        options.premultiplyAlpha,
                                        options.colorSpaceConversion,
                                        options.resizeQuality,
                                        options.resizeWidth,
                                        options.resizeHeight,
                                        shared_asset);

                                canvas_native_image_asset_destroy(shared_asset);


                                auto main_task = [jsi_callback, current_queue, done]() {


                                    v8::Isolate *isolate = jsi_callback->isolate_;
                                    v8::Locker locker(isolate);
                                    v8::Isolate::Scope isolate_scope(isolate);
                                    v8::HandleScope handle_scope(isolate);
                                    v8::Local<v8::Function> callback = jsi_callback->callback_->Get(isolate);
                                    v8::Local<v8::External> cbData = jsi_callback->data_->Get(
                                            isolate).As<v8::External>();
                                    v8::Local<v8::Context> context = callback->GetCreationContextChecked();
                                    v8::Context::Scope context_scope(context);

                                    auto ret = ImageBitmapImpl::NewInstance(isolate, cbData);

                                    v8::Local<v8::Value> args[2];

                                    if (done) {
                                        args[0] = v8::Null(isolate);
                                        args[1] = ret;

                                    } else {
                                        args[0] = v8::Exception::Error(
                                                ConvertToV8String(isolate, "Failed to load image"));
                                        args[1] = v8::Null(isolate);
                                    }


                                    callback->Call(context, context->Global(), 2, args);


                                    delete jsi_callback;
                                    delete current_queue;

                                };
                                current_queue->addOperation(main_task);

                            }, std::move(ab), arrayBuffer->ByteLength());

                    thread.detach();

                    return;
                }

                auto ta = imageObject.As<v8::ArrayBufferView>();


                auto array = ta->Buffer();
                auto offset = ta->ByteOffset();
                auto size = ta->ByteLength();
                auto data_ptr = static_cast<uint8_t *>(array->GetBackingStore()->Data()) + offset;


                v8::Global<v8::ArrayBufferView> ab(isolate, ta);


                std::thread thread(
                        [jsi_callback, &options, shared_asset, data_ptr, size, current_queue](
                                v8::Global<v8::ArrayBufferView> ab
                        ) {

                            auto done = canvas_native_image_bitmap_create_from_encoded_bytes_with_output(
                                    data_ptr, size,
                                    options.flipY,
                                    options.premultiplyAlpha,
                                    options.colorSpaceConversion,
                                    options.resizeQuality,
                                    options.resizeWidth,
                                    options.resizeHeight,
                                    shared_asset);

                            canvas_native_image_asset_destroy(shared_asset);

                            auto main_task = [jsi_callback, current_queue, done]() {


                                v8::Isolate *isolate = jsi_callback->isolate_;
                                v8::Locker locker(isolate);
                                v8::Isolate::Scope isolate_scope(isolate);
                                v8::HandleScope handle_scope(isolate);
                                v8::Local<v8::Function> callback = jsi_callback->callback_->Get(isolate);
                                v8::Local<v8::External> cbData = jsi_callback->data_->Get(
                                        isolate).As<v8::External>();
                                v8::Local<v8::Context> context = callback->GetCreationContextChecked();
                                v8::Context::Scope context_scope(context);

                                 auto ret = ImageBitmapImpl::NewInstance(isolate, cbData);

                                v8::Local<v8::Value> args[2];

                                if (done) {
                                    args[0] = v8::Null(isolate);
                                    args[1] = ret;
                                } else {
                                    args[0] = v8::Exception::Error(
                                            ConvertToV8String(isolate, "Failed to load image"));
                                    args[1] = v8::Null(isolate);
                                }

                                callback->Call(context, context->Global(), 2, args);


                                delete jsi_callback;
                                delete current_queue;

                            };
                            current_queue->addOperation(main_task);



                        }, std::move(ab));
                thread.detach();

#endif

                return;
            } else if (len == 5 || len == 6) {
                auto asset = canvas_native_image_asset_create();

                auto shared_asset = canvas_native_image_asset_shared_clone(asset);

                auto ret = new ImageBitmapImpl(asset);

                auto cbFunc = args[count - 1].As<v8::Function>();
                auto data = v8::External::New(isolate, ret);
                if (len == 6) {
                    options = ImageBitmapImpl::HandleOptions(isolate, args[5]);
                }

                auto jsi_callback = new JSICallback(isolate, cbFunc, data);


#ifdef __ANDROID__
                ALooper_addFd(jsi_callback->looper_,
                              jsi_callback->fd_[0],
                              ALOOPER_POLL_CALLBACK,
                              ALOOPER_EVENT_INPUT,
                              [](int fd, int events,
                                 void *data) {
                                  auto cb = static_cast<JSICallback *>(data);
                                  bool done;
                                  read(fd, &done,
                                       sizeof(bool));

                                  v8::Isolate *isolate = cb->isolate_;
                                  v8::Locker locker(isolate);
                                  v8::Isolate::Scope isolate_scope(isolate);
                                  v8::HandleScope handle_scope(isolate);
                                  v8::Local<v8::Function> callback = cb->callback_->Get(isolate);
                                  v8::Local<v8::External> cbData = cb->data_->Get(
                                          isolate).As<v8::External>();
                                  v8::Local<v8::Context> context = callback->GetCreationContextChecked();
                                  v8::Context::Scope context_scope(context);

                                  auto ret = ImageBitmapImpl::NewInstance(isolate, cbData);

                                  v8::Local<v8::Value> args[2];

                                  if (done) {
                                      args[0] = v8::Null(isolate);
                                      args[1] = ret;
                                  } else {
                                      args[0] = v8::Exception::Error(
                                              ConvertToV8String(isolate, "Failed to load image"));
                                      args[1] = v8::Null(isolate);
                                  }

                                  callback->Call(context, context->Global(), 2, args);

                                  delete static_cast<JSICallback *>(data);
                                  return 0;
                              }, jsi_callback);

                ALooper_wake(jsi_callback->looper_);

                auto bufferValue = args[0];

                if (isArrayBuffer) {
                    auto arrayBuffer = bufferValue.As<v8::ArrayBuffer>();
                    auto dataBuffer = (uint8_t *) arrayBuffer->GetBackingStore()->Data();
                    v8::Global<v8::ArrayBuffer> ab(isolate, arrayBuffer);
                    std::thread thread(
                            [&dataBuffer, jsi_callback, &options, shared_asset](
                                    float sx_or_options,
                                    float sy,
                                    float sw,
                                    float sh,
                                    v8::Global<v8::ArrayBuffer> ab,
                                    size_t size
                            ) {

                                auto done = canvas_native_image_bitmap_create_from_encoded_bytes_src_rect_with_output(
                                        dataBuffer, size,
                                        sx_or_options,
                                        sy,
                                        sw,
                                        sh,
                                        options.flipY,
                                        options.premultiplyAlpha,
                                        options.colorSpaceConversion,
                                        options.resizeQuality,
                                        options.resizeWidth,
                                        options.resizeHeight, shared_asset);

                                canvas_native_image_asset_destroy(shared_asset);


                                write(jsi_callback->fd_[1],
                                      &done,
                                      sizeof(bool));

                            },
                            (float) sx_or_options->NumberValue(context).ToChecked(),
                            (float) sy->NumberValue(context).ToChecked(),
                            (float) sw->NumberValue(context).ToChecked(),
                            (float) sh->NumberValue(context).ToChecked(), std::move(ab),
                            arrayBuffer->ByteLength());

                    thread.detach();

                    return;
                }

                auto ta = bufferValue.As<v8::ArrayBufferView>();

                auto array = ta->Buffer();
                auto offset = ta->ByteOffset();
                auto size = ta->ByteLength();
                auto data_ptr = static_cast<uint8_t *>(array->GetBackingStore()->Data()) + offset;


                v8::Global<v8::ArrayBufferView> ab(isolate, ta);
                std::thread thread(
                        [jsi_callback, &options, shared_asset, data_ptr, size](
                                float sx_or_options,
                                float sy,
                                float sw,
                                float sh,
                                v8::Global<v8::ArrayBufferView> ab
                        ) {

                            auto done = canvas_native_image_bitmap_create_from_encoded_bytes_src_rect_with_output(
                                    data_ptr, size,
                                    sx_or_options,
                                    sy,
                                    sw,
                                    sh,
                                    options.flipY,
                                    options.premultiplyAlpha,
                                    options.colorSpaceConversion,
                                    options.resizeQuality,
                                    options.resizeWidth,
                                    options.resizeHeight, shared_asset);

                            canvas_native_image_asset_destroy(shared_asset);


                            write(jsi_callback->fd_[1],
                                  &done,
                                  sizeof(bool));

                        },
                        (float) sx_or_options->NumberValue(context).ToChecked(),
                        (float) sy->NumberValue(context).ToChecked(),
                        (float) sw->NumberValue(context).ToChecked(),
                        (float) sh->NumberValue(context).ToChecked(), std::move(ab));
                thread.detach();

#endif

#ifdef __APPLE__

                auto current_queue = new NSOperationQueueWrapper(true);


                auto bufferValue = args[0];

                if (isArrayBuffer) {
                    auto arrayBuffer = bufferValue.As<v8::ArrayBuffer>();
                    auto dataBuffer = (uint8_t *) arrayBuffer->GetBackingStore()->Data();
                    v8::Global<v8::ArrayBuffer> ab(isolate, arrayBuffer);
                    std::thread thread(
                            [&dataBuffer, jsi_callback, &options, shared_asset, current_queue](
                                    float sx_or_options,
                                    float sy,
                                    float sw,
                                    float sh,
                                    v8::Global<v8::ArrayBuffer> ab,
                                    size_t size
                            ) {

                                auto done = canvas_native_image_bitmap_create_from_encoded_bytes_src_rect_with_output(
                                        dataBuffer, size,
                                        sx_or_options,
                                        sy,
                                        sw,
                                        sh,
                                        options.flipY,
                                        options.premultiplyAlpha,
                                        options.colorSpaceConversion,
                                        options.resizeQuality,
                                        options.resizeWidth,
                                        options.resizeHeight, shared_asset);

                                canvas_native_image_asset_destroy(shared_asset);


                                auto main_task = [jsi_callback, current_queue, done]() {


                                    v8::Isolate *isolate = jsi_callback->isolate_;
                                    v8::Locker locker(isolate);
                                    v8::Isolate::Scope isolate_scope(isolate);
                                    v8::HandleScope handle_scope(isolate);
                                    v8::Local<v8::Function> callback = jsi_callback->callback_->Get(isolate);
                                    v8::Local<v8::External> cbData = jsi_callback->data_->Get(
                                            isolate).As<v8::External>();
                                    v8::Local<v8::Context> context = callback->GetCreationContextChecked();
                                    v8::Context::Scope context_scope(context);



                                    auto ret = ImageBitmapImpl::NewInstance(isolate, cbData);

                                    v8::Local<v8::Value> args[2];

                                    if (done) {
                                        args[0] = v8::Null(isolate);
                                        args[1] = ret;
                                    } else {
                                        args[0] = v8::Exception::Error(
                                                ConvertToV8String(isolate, "Failed to load image"));
                                        args[1] = v8::Null(isolate);
                                    }

                                    callback->Call(context, context->Global(), 2, args);


                                    delete jsi_callback;
                                    delete current_queue;

                                };
                                current_queue->addOperation(main_task);

                            },
                            (float) sx_or_options->NumberValue(context).ToChecked(),
                            (float) sy->NumberValue(context).ToChecked(),
                            (float) sw->NumberValue(context).ToChecked(),
                            (float) sh->NumberValue(context).ToChecked(),
                                       std::move(ab),
                            arrayBuffer->ByteLength());

                    thread.detach();

                    return;
                }

                auto ta = bufferValue.As<v8::ArrayBufferView>();

                auto array = ta->Buffer();
                auto offset = ta->ByteOffset();
                auto size = ta->ByteLength();
                auto data_ptr = static_cast<uint8_t*>(array->GetBackingStore()->Data()) + offset;



                v8::Global<v8::ArrayBufferView> ab(isolate, ta);
                std::thread thread(
                        [jsi_callback, &options, data_ptr, size, shared_asset, current_queue](
                                float sx_or_options,
                                float sy,
                                float sw,
                                float sh,
                                v8::Global<v8::ArrayBufferView> ab
                        ) {

                            auto done = canvas_native_image_bitmap_create_from_encoded_bytes_src_rect_with_output(
                                    data_ptr, size,
                                    sx_or_options,
                                    sy,
                                    sw,
                                    sh,
                                    options.flipY,
                                    options.premultiplyAlpha,
                                    options.colorSpaceConversion,
                                    options.resizeQuality,
                                    options.resizeWidth,
                                    options.resizeHeight, shared_asset);

                            canvas_native_image_asset_destroy(shared_asset);

                            auto main_task = [jsi_callback, current_queue, done]() {


                                v8::Isolate *isolate = jsi_callback->isolate_;
                                v8::Locker locker(isolate);
                                v8::Isolate::Scope isolate_scope(isolate);
                                v8::HandleScope handle_scope(isolate);
                                v8::Local<v8::Function> callback = jsi_callback->callback_->Get(isolate);
                                v8::Local<v8::External> cbData = jsi_callback->data_->Get(
                                        isolate).As<v8::External>();
                                v8::Local<v8::Context> context = callback->GetCreationContextChecked();
                                v8::Context::Scope context_scope(context);

                                 auto ret = ImageBitmapImpl::NewInstance(isolate, cbData);

                                v8::Local<v8::Value> args[2];

                                if (done) {
                                    args[0] = v8::Null(isolate);
                                    args[1] = ret;
                                } else {
                                    args[0] = v8::Exception::Error(
                                            ConvertToV8String(isolate, "Failed to load image"));
                                    args[1] = v8::Null(isolate);
                                }

                                callback->Call(context, context->Global(), 2, args);


                                delete jsi_callback;
                                delete current_queue;

                            };
                            current_queue->addOperation(main_task);



                        },
                        (float) sx_or_options->NumberValue(context).ToChecked(),
                        (float) sy->NumberValue(context).ToChecked(),
                        (float) sw->NumberValue(context).ToChecked(),
                        (float) sh->NumberValue(context).ToChecked(), std::move(ab));
                thread.detach();



#endif


                return;
            }
        }
    }

    auto type = GetNativeType(image);

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

void CanvasJSIModule::ReadFile(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto file = ConvertFromV8String(isolate, args[0]);
    auto cbFunc = args[1].As<v8::Function>();

    auto jsi_callback = new JSIReadFileCallback(isolate, cbFunc);

#ifdef __ANDROID__
    ALooper_addFd(jsi_callback->looper_,
                  jsi_callback->fd_[0],
                  ALOOPER_POLL_CALLBACK,
                  ALOOPER_EVENT_INPUT,
                  [](int fd, int events,
                     void *data) {

                      auto cb = static_cast<JSIReadFileCallback *>(data);
                      bool done;
                      read(fd, &done,
                           sizeof(bool));

                      if (cb->data_ == nullptr) {
                          return 1;
                      }

                      v8::Isolate *isolate = cb->isolate_;
                      v8::Locker locker(isolate);
                      v8::Isolate::Scope isolate_scope(isolate);
                      v8::HandleScope handle_scope(isolate);
                      v8::Local<v8::Function> callback = cb->callback_.Get(isolate);
                      v8::Local<v8::Context> context = callback->GetCreationContextChecked();
                      v8::Context::Scope context_scope(context);

                      v8::Local<v8::Value> args[2];

                      if (done) {
                          args[0] = v8::Null(isolate);

                          auto vec = cb->data_;

                          auto buf = (void *) canvas_native_u8_buffer_get_bytes_mut(vec);
                          auto size = (size_t) canvas_native_u8_buffer_get_length(vec);

                          auto store = v8::ArrayBuffer::NewBackingStore(buf, size,
                                                                        [](void *data,
                                                                           size_t length,
                                                                           void *deleter_data) {
                                                                            if (deleter_data !=
                                                                                nullptr) {
                                                                                // a little extreme :'D
                                                                                delete static_cast<JSIReadFileCallback *>(deleter_data);
                                                                            }
                                                                        },
                                                                        data);

                          args[1] = v8::ArrayBuffer::New(isolate, std::move(store));
                      } else {
                          auto error = cb->error_;
                          args[0] = v8::Exception::Error(ConvertToV8String(isolate, error));
                          args[1] = v8::Null(isolate);
                      }

                      v8::TryCatch tc(isolate);
                      v8::Local<v8::Value> result;
                      if (!callback->Call(context, context->Global(), 2, args).ToLocal(
                              &result)) {
                          if (tc.HasCaught()) {

                              v8::Local<v8::Value> stack;
                              bool success = tc.StackTrace(context).ToLocal(&stack);
                              if (!success || stack.IsEmpty()) {
                                  if (!done) {
                                      delete cb;
                                  }
                                  return 0;
                              }

                              v8::Local<v8::String> stackV8Str;
                              success = stack->ToDetailString(context).ToLocal(&stackV8Str);
                              if (!success || stackV8Str.IsEmpty()) {
                                  if (!done) {
                                      delete cb;
                                  }
                                  return 0;
                              }
                              LogToConsole(ConvertFromV8String(isolate, stackV8Str));

                          }
                      }

                      if (!done) {
                          delete cb;
                      }
                      return 0;
                  }, jsi_callback);

    ALooper_wake(jsi_callback->looper_);

    std::thread thread(
            [jsi_callback](const std::string &file) {
                bool done = false;
                auto ret = canvas_native_helper_read_file(file.c_str());

                if (!canvas_native_helper_read_file_has_error(ret)) {
                    auto buf = canvas_native_helper_read_file_get_data(ret);

                    jsi_callback->SetData(buf);

                    done = true;
                } else {
                    auto error = canvas_native_helper_read_file_get_error(ret);

                    jsi_callback->SetError(const_cast<char *>(error));
                }
                canvas_native_helper_destroy(ret);

                write(jsi_callback->fd_[1],
                      &done,
                      sizeof(bool));

            }, std::move(file));

    thread.detach();

#endif


#ifdef __APPLE__

    auto current_queue = new NSOperationQueueWrapper(true);

    auto queue = new NSOperationQueueWrapper(false);

    auto task = [jsi_callback, current_queue, queue, file]() {



        bool done = false;
        auto ret = canvas_native_helper_read_file(file.c_str());

        if (!canvas_native_helper_read_file_has_error(ret)) {
            auto buf = canvas_native_helper_read_file_get_data(ret);

            jsi_callback->SetData(buf);

            done = true;
        } else {
            auto error = canvas_native_helper_read_file_get_error(ret);

            jsi_callback->SetError((char*)error);
        }

        canvas_native_helper_destroy(ret);

        auto main_task = [jsi_callback, current_queue, queue, done]() {

            v8::Isolate *isolate = jsi_callback->isolate_;
            v8::Locker locker(isolate);
            v8::Isolate::Scope isolate_scope(isolate);
            v8::HandleScope handle_scope(isolate);
            v8::Local<v8::Function> callback = jsi_callback->callback_.Get(isolate);
            v8::Local<v8::Context> context = callback->GetCreationContextChecked();
            v8::Context::Scope context_scope(context);

            v8::Local<v8::Value> args[2];

            if (done) {
                args[0] = v8::Null(isolate);

                auto vec = jsi_callback->data_;


                auto buf = (void*)canvas_native_u8_buffer_get_bytes_mut(vec);

                auto size = (size_t)canvas_native_u8_buffer_get_length(vec);

                auto store = v8::ArrayBuffer::NewBackingStore(buf, size,
                                                              [](void *data,
                                                                 size_t length,
                                                                 void *deleter_data) {
                                                                  if (deleter_data !=
                                                                      nullptr) {
                                                                      // a little extreme :'D
                                                                      delete static_cast<JSIReadFileCallback *>(deleter_data);
                                                                  }
                                                              },
                                                              jsi_callback);

                args[1] = v8::ArrayBuffer::New(isolate, std::move(store));
            } else {
                auto error = jsi_callback->error_;
                args[0] = v8::Exception::Error(ConvertToV8String(isolate, error));
                args[1] = v8::Null(isolate);
            }

            v8::TryCatch tc(isolate);
            v8::Local<v8::Value> result;
            if (!callback->Call(context, context->Global(), 2, args).ToLocal(
                    &result)) {
                if (tc.HasCaught()) {

                    v8::Local<v8::Value> stack;
                    bool success = tc.StackTrace(context).ToLocal(&stack);
                    if (!success || stack.IsEmpty()) {
                        if (!done) {
                delete jsi_callback;
            }
                        return;
                    }

                    v8::Local<v8::String> stackV8Str;
                    success = stack->ToDetailString(context).ToLocal(&stackV8Str);
                    if (!success || stackV8Str.IsEmpty()) {
                        if (!done) {
                delete jsi_callback;
            }
                        return;
                    }
                    LogToConsole(ConvertFromV8String(isolate, stackV8Str));

                }
            }

            if (!done) {
                delete jsi_callback;
            }

            delete queue;
            delete current_queue;

        };
        current_queue->addOperation(main_task);
    };

    queue->addOperation(task);
#endif


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
        auto webgl = canvas_native_webgl_create(
                ctx,
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
                options.xrCompatible
        );

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
        auto ctx = args[1].As<v8::BigInt>()->Int64Value();
        auto webgl = canvas_native_webgl_create(
                ctx,
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
                options.xrCompatible
        );

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
