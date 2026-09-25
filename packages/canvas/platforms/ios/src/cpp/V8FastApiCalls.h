#pragma once

#include "Common.h"

namespace v8_helpers {

// Wrappers around v8::TryToCopyAndConvertArrayToCppBuffer<Id, T>.
//
// That function is an explicit, V8_EXPORT template specialization -- its body
// lives inside V8 itself (one of int32_t/uint32_t/float/double), not in the
// header. Canvas's own build-time link stub (built from the raw
// libv8_monolith.a) always has it, but a real NativeScript/android runtime
// .so may not: their link step only pulls translation units from the V8
// static archive that their own runtime code references, and none of them
// call this particular fast-API helper, so it's absent from their final
// binary (verified via `nm -D` against @nativescript/android's V8 14.9 next
// build -- zero matches, whereas the build-time stub has all four).
//
// Each wrapper resolves the real mangled symbol at runtime via dlsym and
// delegates to it when present (so a future NativeScript/android runtime
// that does export it is used automatically, with V8's exact semantics).
// Only when the symbol is genuinely missing does it fall back to a manual
// element-by-element copy.
bool TryToCopyAndConvertArrayToCppBufferInt32(v8::Local<v8::Array> src, int32_t *dst, uint32_t max_length);

bool TryToCopyAndConvertArrayToCppBufferUint32(v8::Local<v8::Array> src, uint32_t *dst, uint32_t max_length);

bool TryToCopyAndConvertArrayToCppBufferFloat(v8::Local<v8::Array> src, float *dst, uint32_t max_length);

bool TryToCopyAndConvertArrayToCppBufferDouble(v8::Local<v8::Array> src, double *dst, uint32_t max_length);

} // namespace v8_helpers
