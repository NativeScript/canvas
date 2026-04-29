#include "V8BufferHelper.h"

namespace v8_helpers {

v8::Local<v8::ArrayBuffer> ArrayBufferFromU8Buffer(v8::Isolate *isolate, const U8Buffer *buffer, bool cloneBuffer) {
    if (buffer == nullptr) {
        return v8::ArrayBuffer::New(isolate, 0);
    }

    const U8Buffer *owner = buffer;
    if (cloneBuffer) {
        owner = canvas_native_u8_buffer_clone(buffer);
        if (owner == nullptr) {
            return v8::ArrayBuffer::New(isolate, 0);
        }
    }

    auto buffer_ptr = (void *) canvas_native_u8_buffer_get_bytes(owner);
    auto buffer_len = canvas_native_u8_buffer_get_length(owner);

    auto backing = v8::ArrayBuffer::NewBackingStore(buffer_ptr, buffer_len,
                                                     [](void *data, size_t length, void *deleter_data) {
                                                         if (deleter_data != nullptr) {
                                                             canvas_native_u8_buffer_release((U8Buffer *) deleter_data);
                                                         }
                                                     }, (void *) owner);

    return v8::ArrayBuffer::New(isolate, std::move(backing));
}

} // namespace v8_helpers
