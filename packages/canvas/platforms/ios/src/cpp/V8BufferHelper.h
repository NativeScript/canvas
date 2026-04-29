#pragma once

#include "Common.h"

namespace v8_helpers {

v8::Local<v8::ArrayBuffer> ArrayBufferFromU8Buffer(v8::Isolate *isolate, const U8Buffer *buffer, bool cloneBuffer = true);

} // namespace v8_helpers
