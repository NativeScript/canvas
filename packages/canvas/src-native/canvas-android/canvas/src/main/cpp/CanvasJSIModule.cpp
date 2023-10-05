//
// Created by Osei Fortune on 25/02/2023.
//

#include "CanvasJSIModule.h"
#include "JSICallback.h"
#include "JSIRuntime.h"
#include "Helpers.h"
#include "JSIReadFileCallback.h"

void CanvasJSIModule::install(v8::Isolate *isolate) {
    auto canvas_module = facebook::jsi::Object(jsiRuntime);

    CREATE_FUNC("readFile", canvas_module, 2,
                ([](jsi::Runtime &runtime, const jsi::Value &thisValue,
                    const jsi::Value *arguments, size_t count) -> jsi::Value {

                    auto file = arguments[0].asString(runtime).utf8(runtime);


                    auto cbFunc = std::make_shared<jsi::Value>(
                            runtime, arguments[1]);

                    auto jsi_callback = new JSIReadFileCallback(
                            std::shared_ptr<jsi::Value>(
                                    cbFunc));


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

                                      jsi::Runtime &rt = *jsi_runtime;

                                      auto func = cb->value_->asObject(
                                              rt).asFunction(
                                              rt);

                                      while (cb->data_ == nullptr) {
                                          return 1;
                                      }

                                      if (done) {
                                          auto buf = cb->data_->asObject(rt).getArrayBuffer(rt);
                                          func.call(rt, {jsi::Value::null(), std::move(buf)});
                                      } else {
                                          auto error = cb->data_->asString(rt);
                                          func.call(rt, {std::move(error), jsi::Value::null()});
                                      }

                                      delete static_cast<JSIReadFileCallback *>(data);
                                      return 0;
                                  }, jsi_callback);

                    ALooper_wake(jsi_callback->looper_);

                    auto ab = std::make_shared<jsi::Value>(runtime,
                                                           std::move(arguments[0]));


                    std::thread thread(
                            [&runtime, jsi_callback, cbFunc](const std::string &file) {


                                bool done = false;
                                auto ret = canvas_native_helper_read_file(
                                        rust::Str(file.c_str()));

                                if (!canvas_native_helper_read_file_has_error(*ret)) {
                                    auto buf = canvas_native_helper_read_file_get_data(
                                            std::move(ret));

                                    auto vec_buffer = std::make_shared<VecMutableBuffer<uint8_t>>(
                                            std::move(buf));

                                    jsi_callback->data_ = std::move(
                                            std::make_shared<jsi::Value>(runtime,
                                                                         std::move(
                                                                                 jsi::ArrayBuffer(
                                                                                         runtime,
                                                                                         vec_buffer))));
                                    done = true;
                                } else {
                                    auto error = canvas_native_helper_read_file_get_error(
                                            *ret);

                                    jsi_callback->data_ = std::move(
                                            std::make_shared<jsi::Value>(runtime,
                                                                         std::move(
                                                                                 jsi::String::createFromAscii(
                                                                                         runtime,
                                                                                         error.c_str()))));
                                }


                                write(jsi_callback->fd_[1],
                                      &done,
                                      sizeof(bool));


                            }, std::move(file));

                    thread.detach();

                    return jsi::Value::undefined();
                }

                )

    );


    CREATE_FUNC("createWebGLContext", canvas_module, 7,
                [](jsi::Runtime &runtime, const jsi::Value &thisValue,
                   const jsi::Value *arguments, size_t count) -> jsi::Value {

                    if (arguments[0].isObject()) {
                        auto config = arguments[0].asObject(runtime);
                        std::string version("none");
                        auto alpha = true;
                        auto antialias = true;
                        auto depth = true;
                        auto fail_if_major_performance_caveat = false;
                        std::string power_preference("default");
                        auto premultiplied_alpha = true;
                        auto preserve_drawing_buffer = false;
                        auto stencil = false;
                        auto desynchronized = false;
                        auto xr_compatible = false;


                        auto versionValue = config.getProperty(runtime, "version");
                        if (versionValue.isString()) {
                            version = versionValue.asString(runtime).utf8(runtime);
                        }

                        auto alphaValue = config.getProperty(runtime, "alpha");
                        if (alphaValue.isBool()) {
                            alpha = alphaValue.asBool();
                        }

                        auto antialiasValue = config.getProperty(runtime, "antialias");
                        if (antialiasValue.isBool()) {
                            antialias = antialiasValue.asBool();
                        }

                        auto failIfMajorPerformanceCaveatValue = config.getProperty(runtime,
                                                                                    "failIfMajorPerformanceCaveat");
                        if (failIfMajorPerformanceCaveatValue.isBool()) {
                            fail_if_major_performance_caveat = failIfMajorPerformanceCaveatValue.asBool();
                        }

                        auto powerPreferenceValue = config.getProperty(runtime, "powerPreference");
                        if (powerPreferenceValue.isString()) {
                            power_preference = powerPreferenceValue.asString(runtime).utf8(runtime);
                        }

                        auto premultipliedAlphaValue = config.getProperty(runtime,
                                                                          "premultipliedAlpha");
                        if (premultipliedAlphaValue.isBool()) {
                            premultiplied_alpha = premultipliedAlphaValue.asBool();
                        }

                        auto preserveDrawingBufferValue = config.getProperty(runtime,
                                                                             "preserveDrawingBuffer");
                        if (preserveDrawingBufferValue.isBool()) {
                            preserve_drawing_buffer = preserveDrawingBufferValue.asBool();
                        }

                        auto stencilValue = config.getProperty(runtime, "stencil");
                        if (stencilValue.isBool()) {
                            stencil = stencilValue.asBool();
                        }

                        auto desynchronizedValue = config.getProperty(runtime, "desynchronized");
                        if (desynchronizedValue.isBool()) {
                            desynchronized = desynchronizedValue.asBool();
                        }

                        auto xrCompatibleValue = config.getProperty(
                                runtime,
                                "xrCompatible");
                        if (xrCompatibleValue.isBool()) {
                            xr_compatible = xrCompatibleValue.asBool();
                        }

                        if (version !=
                            "v1") {
                            return jsi::Value::undefined();
                        } else {
                            if (count == 6) {
                                auto context = getPointerValue(runtime, arguments[1]);
                                auto density = arguments[2].asNumber();
                                auto fontColor = arguments[3].asNumber();
                                auto ppi = arguments[4].asNumber();
                                auto direction = arguments[5].asNumber();
                                auto ctx = canvas_native_webgl_create(
                                        context,
                                        rust::Str(
                                                version.c_str()),
                                        alpha,
                                        antialias,
                                        depth,
                                        fail_if_major_performance_caveat,
                                        rust::Str(
                                                power_preference.c_str()),
                                        premultiplied_alpha,
                                        preserve_drawing_buffer,
                                        stencil,
                                        desynchronized,
                                        xr_compatible
                                );

                                auto renderingContext = std::make_shared<WebGLRenderingContext>(
                                        std::move(ctx));

                                return jsi::Object::createFromHostObject(
                                        runtime, renderingContext);

                            } else if (count == 7) {
                                auto width = arguments[1].asNumber();
                                auto height = arguments[2].asNumber();
                                auto density = arguments[3].asNumber();
                                auto fontColor = arguments[4].asNumber();
                                auto ppi = arguments[5].asNumber();
                                auto direction = arguments[6].asNumber();
                                auto ctx = canvas_native_webgl_create_no_window(
                                        (int32_t) width,
                                        (int32_t) height,
                                        rust::Str(
                                                version.c_str()),
                                        alpha,
                                        antialias,
                                        depth,
                                        fail_if_major_performance_caveat,
                                        rust::Str(
                                                power_preference.c_str()),
                                        premultiplied_alpha,
                                        preserve_drawing_buffer,
                                        stencil,
                                        desynchronized,
                                        xr_compatible,
                                        false
                                );

                                auto renderingContext = std::make_shared<WebGLRenderingContext>(
                                        std::move(ctx));

                                return jsi::Object::createFromHostObject(
                                        runtime, renderingContext);

                            } else {
                                auto width = (int32_t) arguments[1].asNumber();
                                auto height = (int32_t) arguments[2].asNumber();

                                auto ctx = canvas_native_webgl_create_no_window(
                                        width,
                                        height,
                                        rust::Str(
                                                version.c_str()),
                                        alpha,
                                        antialias,
                                        depth,
                                        fail_if_major_performance_caveat,
                                        rust::Str(
                                                power_preference.c_str()),
                                        premultiplied_alpha,
                                        preserve_drawing_buffer,
                                        stencil,
                                        desynchronized,
                                        xr_compatible,
                                        false
                                );

                                auto renderingContext = std::make_shared<WebGLRenderingContext>(
                                        std::move(
                                                ctx));


                                return jsi::Object::createFromHostObject(
                                        runtime,
                                        renderingContext);
                            }

                        }
                    }


                    return jsi::Value::undefined();
                }

    );

    CREATE_FUNC("createWebGL2Context", canvas_module, 7,
                [](jsi::Runtime &runtime, const jsi::Value &thisValue,
                   const jsi::Value *arguments, size_t count) -> jsi::Value {

                    if (arguments[0].isObject()) {
                        auto config = arguments[0].asObject(runtime);
                        std::string version("none");
                        auto alpha = true;
                        auto antialias = true;
                        auto depth = true;
                        auto fail_if_major_performance_caveat = false;
                        std::string power_preference("default");
                        auto premultiplied_alpha = true;
                        auto preserve_drawing_buffer = false;
                        auto stencil = false;
                        auto desynchronized = false;
                        auto xr_compatible = false;


                        auto versionValue = config.getProperty(runtime, "version");
                        if (versionValue.isString()) {
                            version = versionValue.asString(runtime).utf8(runtime);
                        }

                        auto alphaValue = config.getProperty(runtime, "alpha");
                        if (alphaValue.isBool()) {
                            alpha = alphaValue.asBool();
                        }

                        auto antialiasValue = config.getProperty(runtime, "antialias");
                        if (antialiasValue.isBool()) {
                            antialias = antialiasValue.asBool();
                        }

                        auto failIfMajorPerformanceCaveatValue = config.getProperty(runtime,
                                                                                    "failIfMajorPerformanceCaveat");
                        if (failIfMajorPerformanceCaveatValue.isBool()) {
                            fail_if_major_performance_caveat = failIfMajorPerformanceCaveatValue.asBool();
                        }

                        auto powerPreferenceValue = config.getProperty(runtime, "powerPreference");
                        if (powerPreferenceValue.isString()) {
                            power_preference = powerPreferenceValue.asString(runtime).utf8(runtime);
                        }

                        auto premultipliedAlphaValue = config.getProperty(runtime,
                                                                          "premultipliedAlpha");
                        if (premultipliedAlphaValue.isBool()) {
                            premultiplied_alpha = premultipliedAlphaValue.asBool();
                        }

                        auto preserveDrawingBufferValue = config.getProperty(runtime,
                                                                             "preserveDrawingBuffer");
                        if (preserveDrawingBufferValue.isBool()) {
                            preserve_drawing_buffer = preserveDrawingBufferValue.asBool();
                        }

                        auto stencilValue = config.getProperty(runtime, "stencil");
                        if (stencilValue.isBool()) {
                            stencil = stencilValue.asBool();
                        }

                        auto desynchronizedValue = config.getProperty(runtime, "desynchronized");
                        if (desynchronizedValue.isBool()) {
                            desynchronized = desynchronizedValue.asBool();
                        }

                        auto xrCompatibleValue = config.getProperty(
                                runtime,
                                "xrCompatible");
                        if (xrCompatibleValue.isBool()) {
                            xr_compatible = xrCompatibleValue.asBool();
                        }

                        if (version !=
                            "v2") {
                            return jsi::Value::undefined();
                        } else {
                            if (count == 6) {
                                auto context = getPointerValue(runtime, arguments[1]);
                                auto density = arguments[2].asNumber();
                                auto fontColor = arguments[3].asNumber();
                                auto ppi = arguments[4].asNumber();
                                auto direction = arguments[5].asNumber();
                                auto ctx = canvas_native_webgl_create(
                                        context,
                                        rust::Str(
                                                version.c_str()),
                                        alpha,
                                        antialias,
                                        depth,
                                        fail_if_major_performance_caveat,
                                        rust::Str(
                                                power_preference.c_str()),
                                        premultiplied_alpha,
                                        preserve_drawing_buffer,
                                        stencil,
                                        desynchronized,
                                        xr_compatible
                                );

                                auto renderingContext = std::make_shared<WebGL2RenderingContext>(
                                        std::move(ctx), WebGLRenderingVersion::V2);

                                return jsi::Object::createFromHostObject(
                                        runtime, renderingContext);

                            } else if (count ==
                                       7) {
                                auto width = arguments[1].asNumber();
                                auto height = arguments[2].asNumber();
                                auto density = arguments[3].asNumber();
                                auto fontColor = arguments[4].asNumber();
                                auto ppi = arguments[5].asNumber();
                                auto direction = arguments[6].asNumber();
                                auto ctx = canvas_native_webgl_create_no_window(
                                        (int32_t) width,
                                        (int32_t) height,
                                        rust::Str(
                                                version.c_str()),
                                        alpha,
                                        antialias,
                                        depth,
                                        fail_if_major_performance_caveat,
                                        rust::Str(
                                                power_preference.c_str()),
                                        premultiplied_alpha,
                                        preserve_drawing_buffer,
                                        stencil,
                                        desynchronized,
                                        xr_compatible,
                                        false
                                );
                                auto renderingContext = std::make_shared<WebGL2RenderingContext>(
                                        std::move(
                                                ctx), WebGLRenderingVersion::V2);

                                return jsi::Object::createFromHostObject(
                                        runtime,
                                        renderingContext);

                            } else {
                                auto width = (int32_t) arguments[1].asNumber();
                                auto height = (int32_t) arguments[2].asNumber();
                                auto ctx = canvas_native_webgl_create_no_window(
                                        width,
                                        height,
                                        rust::Str(
                                                version.c_str()),
                                        alpha,
                                        antialias,
                                        depth,
                                        fail_if_major_performance_caveat,
                                        rust::Str(
                                                power_preference.c_str()),
                                        premultiplied_alpha,
                                        preserve_drawing_buffer,
                                        stencil,
                                        desynchronized,
                                        xr_compatible,
                                        false
                                );

                                auto renderingContext = std::make_shared<WebGL2RenderingContext>(
                                        std::move(
                                                ctx), WebGLRenderingVersion::V2);

                                return jsi::Object::createFromHostObject(
                                        runtime,
                                        renderingContext);
                            }
                        }
                    }

                    return jsi::Value::undefined();
                }

    );

    auto global = jsiRuntime.global();

    if (!global.
            hasProperty(jsiRuntime,
                        "CanvasJSIModule")) {
        global.
                setProperty(jsiRuntime,
                            "CanvasJSIModule", canvas_module);


        v8::Locker locker(isolate);
        v8::Isolate::Scope isolate_scope(isolate);
        v8::HandleScope handle_scope(isolate);

        auto context = isolate->GetCurrentContext();
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
        v8Global->Set(context, ConvertToV8String(isolate, "CanvasModule"), canvasMod);
        canvasMod->Set(context, ConvertToV8String(isolate, "create2DContext"),
                       v8::FunctionTemplate::New(isolate, &Create2DContext)->GetFunction(
                               context).ToLocalChecked());
        canvasMod->Set(context, ConvertToV8String(isolate, "createImageBitmap"),
                       v8::FunctionTemplate::New(isolate, &CreateImageBitmap)->GetFunction(
                               context).ToLocalChecked());
        canvasMod->Set(context, ConvertToV8String(isolate, "create2DContextWithPointer"),
                       v8::FunctionTemplate::New(isolate, &Create2DContextWithPointer)->GetFunction(
                               context).ToLocalChecked());


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

    auto ctx = new CanvasRenderingContext2DImpl(
            std::move(context_2d));

    auto ret = CanvasRenderingContext2DImpl::GetCtor(isolate)->GetFunction(
            context).ToLocalChecked()->NewInstance(context).ToLocalChecked();

    auto data = v8::External::New(isolate, ctx);

    ret->SetInternalField(0, data);

    SetNativeType(isolate, ret, NativeType::CanvasRenderingContext2D);

    args.GetReturnValue().Set(ret);
}

void CanvasJSIModule::CreateImageBitmap(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto count = args.Length();
    auto value = args[0];
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto image = args[0];
    auto sx_or_options = args[1];
    auto sy = args[2];
    auto sw = args[3];
    auto sh = args[4];

    auto len = count - 1;
    auto cb = args[count - 1];

    if (len == 1 && !image->IsObject() || image->IsFunction()) {
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


    if (args[0]->IsObject()) {
        auto imageObject = args[0].As<v8::Object>();
        auto isArrayBuffer = imageObject->IsArrayBuffer();
        auto isTypedArray = imageObject->IsTypedArray();
        if (isArrayBuffer || isTypedArray) {

            if (len == 1 || len == 2) {
                if (len == 2) {
                    options = ImageBitmapImpl::HandleOptions(isolate, args[1]);
                }

                auto asset = canvas_native_image_asset_create();

                auto shared_asset = canvas_native_image_asset_shared_clone(*asset);


                auto ret = new ImageBitmapImpl(
                        std::move(asset));

                v8::Global<v8::Function> cbFunc(isolate, args[count - 1].As<v8::Function>());
                v8::Global<v8::Value> data(isolate, v8::External::New(isolate, ret));
                auto jsi_callback = new JSICallback(isolate, std::move(cbFunc), std::move(data));

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

                                  v8::HandleScope handle_scope(isolate);
                                  v8::Local<v8::Function> callback = cb->callback_.Get(isolate);
                                  v8::Local<v8::External> cbData = cb->data_.Get(
                                          isolate).As<v8::External>();
                                  v8::Local<v8::Context> context = callback->GetCreationContextChecked();
                                  v8::Context::Scope context_scope(context);

                                  auto ret = ImageBitmapImpl::GetCtor(isolate)->GetFunction(
                                          context).ToLocalChecked()->NewInstance(
                                          context).ToLocalChecked();

                                  SetNativeType(isolate, ret, NativeType::ImageBitmap);

                                  ret->SetInternalField(0, cbData);

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
                v8::Global<v8::Value> ab(isolate, bufferValue);

                if (isArrayBuffer) {

                    auto arrayBuffer = bufferValue.As<v8::ArrayBuffer>();
                    auto dataBuffer = (uint8_t *) arrayBuffer->GetBackingStore()->Data();

                    std::thread thread(
                            [&dataBuffer, jsi_callback, &options](
                                    rust::Box<ImageAsset> asset,
                                    v8::Global<v8::Value> ab, size_t size) {


                                auto done = canvas_native_image_bitmap_create_from_encoded_bytes_with_output(
                                        rust::Slice<const uint8_t>(dataBuffer, size),
                                        options.flipY,
                                        options.premultiplyAlpha,
                                        options.colorSpaceConversion,
                                        options.resizeQuality,
                                        options.resizeWidth,
                                        options.resizeHeight,
                                        *asset);

                                write(jsi_callback->fd_[1],
                                      &done,
                                      sizeof(bool));


                            }, std::move(shared_asset), std::move(ab), arrayBuffer->ByteLength());

                    thread.detach();
                    return;
                }

                if (isTypedArray) {

                    auto ta = bufferValue.As<v8::TypedArray>();
                    auto typedArray = GetTypedArrayData<const uint8_t>(ta);

                    std::thread thread(
                            [&typedArray, jsi_callback, &options](
                                    rust::Box<ImageAsset> asset, v8::Global<v8::Value> ab) {


                                auto done = canvas_native_image_bitmap_create_from_encoded_bytes_with_output(
                                        typedArray,
                                        options.flipY,
                                        options.premultiplyAlpha,
                                        options.colorSpaceConversion,
                                        options.resizeQuality,
                                        options.resizeWidth,
                                        options.resizeHeight,
                                        *asset);

                                write(jsi_callback->fd_[1],
                                      &done,
                                      sizeof(bool));


                            }, std::move(shared_asset), std::move(ab));
                    thread.detach();

                }
            } else if (len == 5 || len == 6) {
                auto asset = canvas_native_image_asset_create();

                auto shared_asset = canvas_native_image_asset_shared_clone(*asset);

                auto ret = new ImageBitmapImpl(
                        std::move(asset));

                auto cbFunc = args[count - 1].As<v8::Function>();
                v8::Global<v8::Function> globalFunc(isolate, cbFunc);
                v8::Global<v8::Value> data(isolate, v8::External::New(isolate, ret));
                if (len == 6) {
                    options = ImageBitmapImpl::HandleOptions(isolate, args[5]);
                }

                auto jsi_callback = new JSICallback(isolate, std::move(globalFunc),
                                                    std::move(data));


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

                                  v8::HandleScope handle_scope(isolate);
                                  v8::Local<v8::Function> callback = cb->callback_.Get(isolate);
                                  v8::Local<v8::External> cbData = cb->data_.Get(
                                          isolate).As<v8::External>();
                                  v8::Local<v8::Context> context = callback->GetCreationContextChecked();
                                  v8::Context::Scope context_scope(context);

                                  auto ret = ImageBitmapImpl::GetCtor(isolate)->GetFunction(
                                          context).ToLocalChecked()->NewInstance(
                                          context).ToLocalChecked();

                                  SetNativeType(isolate, ret, NativeType::ImageBitmap);

                                  ret->SetInternalField(0, cbData);

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
                v8::Global<v8::Value> ab(isolate, bufferValue);


                if (isArrayBuffer) {
                    auto arrayBuffer = bufferValue.As<v8::ArrayBuffer>();
                    auto dataBuffer = (uint8_t *) arrayBuffer->GetBackingStore()->Data();
                    std::thread thread(
                            [&dataBuffer, jsi_callback, &options](
                                    rust::Box<ImageAsset> asset,
                                    float sx_or_options,
                                    float sy,
                                    float sw,
                                    float sh,
                                    v8::Global<v8::Value> ab,
                                    size_t size
                            ) {

                                auto done = canvas_native_image_bitmap_create_from_encoded_bytes_src_rect_with_output(
                                        rust::Slice<const uint8_t>(dataBuffer, size),
                                        sx_or_options,
                                        sy,
                                        sw,
                                        sh,
                                        options.flipY,
                                        options.premultiplyAlpha,
                                        options.colorSpaceConversion,
                                        options.resizeQuality,
                                        options.resizeWidth,
                                        options.resizeHeight, *asset);


                                write(jsi_callback->fd_[1],
                                      &done,
                                      sizeof(bool));

                            }, std::move(shared_asset),
                            (float) sx_or_options->NumberValue(context).ToChecked(),
                            (float) sy->NumberValue(context).ToChecked(),
                            (float) sw->NumberValue(context).ToChecked(),
                            (float) sh->NumberValue(context).ToChecked(), std::move(ab),
                            arrayBuffer->ByteLength());

                    thread.detach();

                    return;
                }

                if (isTypedArray) {
                    auto ta = bufferValue.As<v8::TypedArray>();
                    auto typedArray = GetTypedArrayData<const uint8_t>(ta);

                    std::thread thread(
                            [&typedArray, jsi_callback, &options](
                                    rust::Box<ImageAsset> asset,
                                    float sx_or_options,
                                    float sy,
                                    float sw,
                                    float sh,
                                    v8::Global<v8::Value> ab
                            ) {

                                auto done = canvas_native_image_bitmap_create_from_encoded_bytes_src_rect_with_output(
                                        typedArray,
                                        sx_or_options,
                                        sy,
                                        sw,
                                        sh,
                                        options.flipY,
                                        options.premultiplyAlpha,
                                        options.colorSpaceConversion,
                                        options.resizeQuality,
                                        options.resizeWidth,
                                        options.resizeHeight, *asset);


                                write(jsi_callback->fd_[1],
                                      &done,
                                      sizeof(bool));

                            }, std::move(shared_asset),
                            (float) sx_or_options->NumberValue(context).ToChecked(),
                            (float) sy->NumberValue(context).ToChecked(),
                            (float) sw->NumberValue(context).ToChecked(),
                            (float) sh->NumberValue(context).ToChecked(), std::move(ab));

                    thread.detach();

                }


                return;
            }
        }
    }


    auto typeValue = GetPrivateValue(isolate, image.As<v8::Object>(),
                                     ConvertToV8String(isolate, "__type"));

    if (len == 1 || len == 2) {
        if (len == 2) {
            options = ImageBitmapImpl::HandleOptions(isolate, args[1]);
        }

        v8::Local<v8::Value> retArgs[2];
        retArgs[0] = v8::Null(isolate);


        if (!typeValue.IsEmpty()) {
            auto type = (NativeType) typeValue->Int32Value(context).ToChecked();
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


                    auto bitmap = new ImageBitmapImpl(std::move(ret));
                    auto data = v8::External::New(isolate, bitmap);
                    auto object = ImageBitmapImpl::GetCtor(isolate)->GetFunction(
                            context).ToLocalChecked()->NewInstance(context).ToLocalChecked();

                    object->SetInternalField(0, data);

                    SetNativeType(isolate, object, NativeType::ImageBitmap);

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


                    auto bitmap = new ImageBitmapImpl(std::move(ret));
                    auto data = v8::External::New(isolate, bitmap);
                    auto object = ImageBitmapImpl::GetCtor(isolate)->GetFunction(
                            context).ToLocalChecked()->NewInstance(context).ToLocalChecked();

                    object->SetInternalField(0, data);

                    SetNativeType(isolate, object, NativeType::ImageBitmap);

                    retArgs[1] = object;

                }
                    break;
                default:
                    break;
            }
        }


        cb.As<v8::Function>()->Call(context, context->Global(), 2, retArgs);

        return;
    } else if (len == 5 || len == 6) {

        if (len == 6) {
            options = ImageBitmapImpl::HandleOptions(isolate, args[5]);
        }

        v8::Local<v8::Value> retArgs[2];
        retArgs[0] = v8::Null(isolate);


        if (!typeValue.IsEmpty()) {
            auto type = (NativeType) typeValue->Int32Value(context).ToChecked();
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


                    auto bitmap = new ImageBitmapImpl(std::move(ret));
                    auto data = v8::External::New(isolate, bitmap);
                    auto object = ImageBitmapImpl::GetCtor(isolate)->GetFunction(
                            context).ToLocalChecked()->NewInstance(context).ToLocalChecked();

                    object->SetInternalField(0, data);

                    SetNativeType(isolate, object, NativeType::ImageBitmap);

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


                    auto bitmap = new ImageBitmapImpl(std::move(ret));
                    auto data = v8::External::New(isolate, bitmap);
                    auto object = ImageBitmapImpl::GetCtor(isolate)->GetFunction(
                            context).ToLocalChecked()->NewInstance(context).ToLocalChecked();

                    object->SetInternalField(0, data);

                    SetNativeType(isolate, object, NativeType::ImageBitmap);

                    retArgs[1] = object;

                }
                    break;
                default:
                    break;
            }
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

    auto ctx = new CanvasRenderingContext2DImpl(
            std::move(context_2d));

    auto ret = CanvasRenderingContext2DImpl::GetCtor(isolate)->GetFunction(
            context).ToLocalChecked()->NewInstance(context).ToLocalChecked();

    auto data = v8::External::New(isolate, ctx);

    ret->SetInternalField(0, data);

    args.GetReturnValue().Set(ret);
}
