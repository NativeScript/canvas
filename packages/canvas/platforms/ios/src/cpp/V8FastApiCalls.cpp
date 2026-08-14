#include "V8FastApiCalls.h"
#include <dlfcn.h>

namespace v8_helpers {

namespace {

template<typename T>
using CopyFn = bool (*)(v8::Local<v8::Array>, T *, uint32_t);

template<typename T>
CopyFn<T> ResolveRealSymbol(const char *mangled_name) {
    return reinterpret_cast<CopyFn<T>>(dlsym(RTLD_DEFAULT, mangled_name));
}

// Mirrors v8::TryToCopyAndConvertArrayToCppBuffer's documented contract: any
// element that isn't itself a JS number (not just number-convertible, e.g.
// no object/undefined/null coercion) makes the whole array unsupported.
template<typename T, typename Convert>
bool ManualCopy(v8::Local<v8::Array> src, T *dst, uint32_t max_length, Convert convert) {
    auto isolate = v8::Isolate::GetCurrent();
    auto context = isolate->GetCurrentContext();
    auto len = src->Length();
    auto count = len < max_length ? len : max_length;
    for (uint32_t i = 0; i < count; i++) {
        v8::Local<v8::Value> value;
        if (!src->Get(context, i).ToLocal(&value) || !value->IsNumber()) {
            return false;
        }

        T converted;
        if (!convert(context, value, &converted)) {
            return false;
        }

        if (dst != nullptr) {
            dst[i] = converted;
        }
    }
    return true;
}

} // namespace

bool TryToCopyAndConvertArrayToCppBufferInt32(v8::Local<v8::Array> src, int32_t *dst, uint32_t max_length) {
    static auto real = ResolveRealSymbol<int32_t>(
            "_ZN2v835TryToCopyAndConvertArrayToCppBufferILj768EiEEbNS_5LocalINS_5ArrayEEEPT0_j");
    if (real != nullptr) {
        return real(src, dst, max_length);
    }

    return ManualCopy<int32_t>(src, dst, max_length,
                                [](v8::Local<v8::Context> context, v8::Local<v8::Value> value, int32_t *out) {
                                    return value->Int32Value(context).To(out);
                                });
}

bool TryToCopyAndConvertArrayToCppBufferUint32(v8::Local<v8::Array> src, uint32_t *dst, uint32_t max_length) {
    static auto real = ResolveRealSymbol<uint32_t>(
            "_ZN2v835TryToCopyAndConvertArrayToCppBufferILj1024EjEEbNS_5LocalINS_5ArrayEEEPT0_j");
    if (real != nullptr) {
        return real(src, dst, max_length);
    }

    return ManualCopy<uint32_t>(src, dst, max_length,
                                 [](v8::Local<v8::Context> context, v8::Local<v8::Value> value, uint32_t *out) {
                                     return value->Uint32Value(context).To(out);
                                 });
}

bool TryToCopyAndConvertArrayToCppBufferFloat(v8::Local<v8::Array> src, float *dst, uint32_t max_length) {
    static auto real = ResolveRealSymbol<float>(
            "_ZN2v835TryToCopyAndConvertArrayToCppBufferILj1792EfEEbNS_5LocalINS_5ArrayEEEPT0_j");
    if (real != nullptr) {
        return real(src, dst, max_length);
    }

    return ManualCopy<float>(src, dst, max_length,
                              [](v8::Local<v8::Context> context, v8::Local<v8::Value> value, float *out) {
                                  double converted;
                                  if (!value->NumberValue(context).To(&converted)) {
                                      return false;
                                  }
                                  *out = static_cast<float>(converted);
                                  return true;
                              });
}

bool TryToCopyAndConvertArrayToCppBufferDouble(v8::Local<v8::Array> src, double *dst, uint32_t max_length) {
    static auto real = ResolveRealSymbol<double>(
            "_ZN2v835TryToCopyAndConvertArrayToCppBufferILj2048EdEEbNS_5LocalINS_5ArrayEEEPT0_j");
    if (real != nullptr) {
        return real(src, dst, max_length);
    }

    return ManualCopy<double>(src, dst, max_length,
                               [](v8::Local<v8::Context> context, v8::Local<v8::Value> value, double *out) {
                                   return value->NumberValue(context).To(out);
                               });
}

} // namespace v8_helpers
