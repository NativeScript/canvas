#pragma once

#include "Common.h"
#include "Helpers.h"
#include "ObjectWrapperImpl.h"

class OffscreenSurfaceImpl : public ObjectWrapperImpl {
public:
    /// Takes `surface`'s reference.
    explicit OffscreenSurfaceImpl(const CanvasOffscreenSurface *surface);

    ~OffscreenSurfaceImpl() override;

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static OffscreenSurfaceImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, OffscreenSurfaceImpl *impl);

    const CanvasOffscreenSurface *GetSurface() { return surface_; }

private:
    const CanvasOffscreenSurface *surface_;

    static void Create(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FromPointer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Adopt(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReleaseHandle(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetWidth(v8::Local<v8::Name> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetHeight(v8::Local<v8::Name> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetDensity(v8::Local<v8::Name> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetPpi(v8::Local<v8::Name> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetDirection(v8::Local<v8::Name> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetHasView(v8::Local<v8::Name> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void Resize(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ToHandle(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Dispose(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Create2D(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreateWebGL(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreateWebGPU(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ToDataURL(const v8::FunctionCallbackInfo<v8::Value> &args);
};
