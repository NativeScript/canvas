#include "OffscreenSurfaceImpl.h"
#include "Caches.h"
#include "OneByteStringResource.h"
#include "canvas2d/CanvasRenderingContext2DImpl.h"
#include "webgl/WebGLRenderingContext.h"
#include "webgl2/WebGL2RenderingContext.h"
#include "webgpu/GPUImpl.h"
#include "webgpu/GPUCanvasContextImpl.h"

OffscreenSurfaceImpl::OffscreenSurfaceImpl(const CanvasOffscreenSurface *surface) : surface_(surface) {}

OffscreenSurfaceImpl::~OffscreenSurfaceImpl() {
    canvas_native_offscreen_surface_release(surface_);
    surface_ = nullptr;
}

void OffscreenSurfaceImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto context = isolate->GetCurrentContext();
    auto func = GetCtor(isolate)->GetFunction(context).ToLocalChecked();
    canvasModule->Set(context, ConvertToV8String(isolate, "OffscreenSurface"), func).FromJust();
}

OffscreenSurfaceImpl *OffscreenSurfaceImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0, ObjectWrapperImpl::kInternalFieldTag);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<OffscreenSurfaceImpl *>(ptr);
}

v8::Local<v8::Object> OffscreenSurfaceImpl::NewInstance(v8::Isolate *isolate, OffscreenSurfaceImpl *impl) {
    auto context = isolate->GetCurrentContext();
    v8::EscapableHandleScope scope(isolate);
    auto object = GetCtor(isolate)->GetFunction(context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
    SetNativeType(impl, NativeType::OffscreenSurface);
    object->SetAlignedPointerInInternalField(0, impl, ObjectWrapperImpl::kInternalFieldTag);
    impl->BindFinalizer(isolate, object);
    return scope.Escape(object);
}

/// Null after `dispose()`.
static const CanvasOffscreenSurface *SurfaceOf(const v8::Local<v8::Object> &object) {
    auto ptr = OffscreenSurfaceImpl::GetPointer(object);
    return ptr == nullptr ? nullptr : ptr->GetSurface();
}

v8::Local<v8::FunctionTemplate> OffscreenSurfaceImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->OffscreenSurfaceTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "OffscreenSurface"));
    ctorTmpl->Set(ConvertToV8String(isolate, "create"), v8::FunctionTemplate::New(isolate, Create));
    ctorTmpl->Set(ConvertToV8String(isolate, "fromPointer"), v8::FunctionTemplate::New(isolate, FromPointer));
    ctorTmpl->Set(ConvertToV8String(isolate, "adopt"), v8::FunctionTemplate::New(isolate, Adopt));
    ctorTmpl->Set(ConvertToV8String(isolate, "releaseHandle"), v8::FunctionTemplate::New(isolate, ReleaseHandle));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);
    tmpl->SetNativeDataProperty(ConvertToV8String(isolate, "width"), GetWidth);
    tmpl->SetNativeDataProperty(ConvertToV8String(isolate, "height"), GetHeight);
    tmpl->SetNativeDataProperty(ConvertToV8String(isolate, "density"), GetDensity);
    tmpl->SetNativeDataProperty(ConvertToV8String(isolate, "ppi"), GetPpi);
    tmpl->SetNativeDataProperty(ConvertToV8String(isolate, "direction"), GetDirection);
    tmpl->SetNativeDataProperty(ConvertToV8String(isolate, "hasView"), GetHasView);
    tmpl->Set(ConvertToV8String(isolate, "resize"), v8::FunctionTemplate::New(isolate, Resize));
    tmpl->Set(ConvertToV8String(isolate, "toHandle"), v8::FunctionTemplate::New(isolate, ToHandle));
    tmpl->Set(ConvertToV8String(isolate, "dispose"), v8::FunctionTemplate::New(isolate, Dispose));
    tmpl->Set(ConvertToV8String(isolate, "create2D"), v8::FunctionTemplate::New(isolate, Create2D));
    tmpl->Set(ConvertToV8String(isolate, "createWebGL"), v8::FunctionTemplate::New(isolate, CreateWebGL));
    tmpl->Set(ConvertToV8String(isolate, "createWebGPU"), v8::FunctionTemplate::New(isolate, CreateWebGPU));
    tmpl->Set(ConvertToV8String(isolate, "toDataURL"), v8::FunctionTemplate::New(isolate, ToDataURL));

    cache->OffscreenSurfaceTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

static void ReturnSurface(const v8::FunctionCallbackInfo<v8::Value> &args, const CanvasOffscreenSurface *surface) {
    if (surface == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }
    auto isolate = args.GetIsolate();
    args.GetReturnValue().Set(OffscreenSurfaceImpl::NewInstance(isolate, new OffscreenSurfaceImpl(surface)));
}

void OffscreenSurfaceImpl::Create(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto width = args[0]->Uint32Value(context).FromMaybe(300);
    auto height = args[1]->Uint32Value(context).FromMaybe(150);
    auto density = (float) args[2]->NumberValue(context).FromMaybe(1);
    auto ppi = (float) args[3]->NumberValue(context).FromMaybe(160);
    auto direction = args[4]->Uint32Value(context).FromMaybe(0);
    auto colorSpace = args[5]->Uint32Value(context).FromMaybe(0) == 1 ? CanvasColorSpaceP3 : CanvasColorSpaceSrgb;
    ReturnSurface(args, canvas_native_offscreen_surface_create(width, height, density, ppi, direction, colorSpace));
}

/// Takes the reference the canvas view handed out.
void OffscreenSurfaceImpl::FromPointer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto context = args.GetIsolate()->GetCurrentContext();
    v8::Local<v8::BigInt> value;
    if (!args[0]->ToBigInt(context).ToLocal(&value)) {
        args.GetReturnValue().SetNull();
        return;
    }
    ReturnSurface(args, (const CanvasOffscreenSurface *) value->Int64Value());
}

void OffscreenSurfaceImpl::Adopt(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto context = args.GetIsolate()->GetCurrentContext();
    auto handle = args[0]->Uint32Value(context).FromMaybe(0);
    ReturnSurface(args, handle == 0 ? nullptr : canvas_native_offscreen_surface_adopt(handle));
}

void OffscreenSurfaceImpl::ReleaseHandle(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto context = args.GetIsolate()->GetCurrentContext();
    auto handle = args[0]->Uint32Value(context).FromMaybe(0);
    args.GetReturnValue().Set(handle != 0 && canvas_native_offscreen_surface_release_handle(handle));
}

void OffscreenSurfaceImpl::GetWidth(v8::Local<v8::Name>, const v8::PropertyCallbackInfo<v8::Value> &info) {
    info.GetReturnValue().Set(canvas_native_offscreen_surface_get_width(SurfaceOf(info.Holder())));
}

void OffscreenSurfaceImpl::GetHeight(v8::Local<v8::Name>, const v8::PropertyCallbackInfo<v8::Value> &info) {
    info.GetReturnValue().Set(canvas_native_offscreen_surface_get_height(SurfaceOf(info.Holder())));
}

void OffscreenSurfaceImpl::GetDensity(v8::Local<v8::Name>, const v8::PropertyCallbackInfo<v8::Value> &info) {
    info.GetReturnValue().Set((double) canvas_native_offscreen_surface_get_density(SurfaceOf(info.Holder())));
}

void OffscreenSurfaceImpl::GetPpi(v8::Local<v8::Name>, const v8::PropertyCallbackInfo<v8::Value> &info) {
    info.GetReturnValue().Set((double) canvas_native_offscreen_surface_get_ppi(SurfaceOf(info.Holder())));
}

void OffscreenSurfaceImpl::GetDirection(v8::Local<v8::Name>, const v8::PropertyCallbackInfo<v8::Value> &info) {
    info.GetReturnValue().Set(canvas_native_offscreen_surface_get_direction(SurfaceOf(info.Holder())));
}

void OffscreenSurfaceImpl::GetHasView(v8::Local<v8::Name>, const v8::PropertyCallbackInfo<v8::Value> &info) {
    info.GetReturnValue().Set(canvas_native_offscreen_surface_has_view(SurfaceOf(info.Holder())));
}

void OffscreenSurfaceImpl::Resize(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto context = args.GetIsolate()->GetCurrentContext();
    auto width = args[0]->Uint32Value(context).FromMaybe(0);
    auto height = args[1]->Uint32Value(context).FromMaybe(0);
    canvas_native_offscreen_surface_resize(SurfaceOf(args.This()), width, height);
}

void OffscreenSurfaceImpl::ToHandle(const v8::FunctionCallbackInfo<v8::Value> &args) {
    args.GetReturnValue().Set(canvas_native_offscreen_surface_to_handle(SurfaceOf(args.This())));
}

void OffscreenSurfaceImpl::Dispose(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr && ptr->surface_ != nullptr) {
        canvas_native_offscreen_surface_release(ptr->surface_);
        ptr->surface_ = nullptr;
    }
}

void OffscreenSurfaceImpl::Create2D(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto alpha = args[0]->BooleanValue(isolate);
    auto fontColor = args[1]->Int32Value(context).FromMaybe(-16777216);
    auto threaded = args[2]->BooleanValue(isolate);
    auto ctx = canvas_native_offscreen_surface_create_2d(SurfaceOf(args.This()), alpha, fontColor, threaded);
    if (ctx == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }
    args.GetReturnValue().Set(CanvasRenderingContext2DImpl::NewInstance(isolate, new CanvasRenderingContext2DImpl(ctx, true)));
}

void OffscreenSurfaceImpl::CreateWebGL(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    WebGLAttributes attributes{};
    attributes.version = args[0]->Int32Value(context).FromMaybe(1);
    attributes.alpha = args[1]->BooleanValue(isolate);
    attributes.antialias = args[2]->BooleanValue(isolate);
    attributes.depth = args[3]->BooleanValue(isolate);
    attributes.fail_if_major_performance_caveat = args[4]->BooleanValue(isolate);
    attributes.power_preference = args[5]->Int32Value(context).FromMaybe(0);
    attributes.premultiplied_alpha = args[6]->BooleanValue(isolate);
    attributes.preserve_drawing_buffer = args[7]->BooleanValue(isolate);
    attributes.stencil = args[8]->BooleanValue(isolate);
    attributes.desynchronized = args[9]->BooleanValue(isolate);
    attributes.xr_compatible = args[10]->BooleanValue(isolate);
    auto threaded = args[11]->BooleanValue(isolate);

    auto state = canvas_native_offscreen_surface_create_webgl(SurfaceOf(args.This()), &attributes, threaded);
    if (state == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }
    if (attributes.version == 2) {
        args.GetReturnValue().Set(WebGL2RenderingContext::NewInstance(isolate, new WebGL2RenderingContext(state, WebGLRenderingVersion::V2)));
    } else {
        args.GetReturnValue().Set(WebGLRenderingContext::NewInstance(isolate, new WebGLRenderingContext(state)));
    }
}

/// `gpu`: the calling isolate's `navigator.gpu.native`.
void OffscreenSurfaceImpl::CreateWebGPU(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    if (!args[0]->IsObject()) {
        args.GetReturnValue().SetNull();
        return;
    }
    auto gpu = GPUImpl::GetPointer(args[0].As<v8::Object>());
    auto ctx = gpu == nullptr ? nullptr : canvas_native_offscreen_surface_create_webgpu(SurfaceOf(args.This()), gpu->GetGPUInstance());
    if (ctx == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }
    args.GetReturnValue().Set(GPUCanvasContextImpl::NewInstance(isolate, new GPUCanvasContextImpl(ctx)));
}

void OffscreenSurfaceImpl::ToDataURL(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    std::string type("image/png");
    int quality = 92;
    if (args[0]->IsString()) {
        type = ConvertFromV8String(isolate, args[0]);
    }
    if (args[1]->IsNumber()) {
        quality = (int) (args[1]->NumberValue(context).ToChecked() * 100);
    }
    auto data = canvas_native_offscreen_surface_to_data_url(SurfaceOf(args.This()), type.c_str(), quality);
    if (data == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }
    auto value = new OneByteStringResource(data);
    args.GetReturnValue().Set(v8::String::NewExternalOneByte(isolate, value).ToLocalChecked());
}
