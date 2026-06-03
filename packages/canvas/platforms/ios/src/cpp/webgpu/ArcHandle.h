//
// ArcHandle.h — RAII owner for the Rust Arc handles exposed across the canvas C ABI.
//

#ifndef CANVAS_WEBGPU_ARCHANDLE_H
#define CANVAS_WEBGPU_ARCHANDLE_H

#include <memory>

// Every WebGPU wrapper owns one raw pointer to a refcounted Rust object. The
// matching `..._release` C ABI function decrements that Arc exactly once. We hold
// the pointer in a unique_ptr with a stateless custom deleter so the lifecycle is
// a property of the type instead of a hand-written destructor plus an idempotency
// flag:
//
//   - reset() runs the deleter once and nulls the pointer. That is the
//     deterministic dispose, called at the WebGPU consumption point the spec
//     already defines (pass.end(), encoder.finish(), queue.submit(), present()).
//   - ~unique_ptr() invokes the deleter only when the pointer is non-null, so the
//     GC finalizer that runs later is a guaranteed no-op. No double-decrement, no
//     use-after-free, no flag to keep in sync.
//
// Decrementing our Arc share never frees a resource the GPU still needs: wgpu
// holds its own strong refs to anything in flight (submitted command buffers, the
// surface's current texture), so releasing at the consumption point is safe by the
// Arc semantics.
template <typename T, void (*Release)(const T *)>
struct ArcDeleter {
    void operator()(const T *ptr) const noexcept {
        if (ptr != nullptr) {
            Release(ptr);
        }
    }
};

template <typename T, void (*Release)(const T *)>
using ArcHandle = std::unique_ptr<const T, ArcDeleter<T, Release>>;

// Variant for the handful of C ABI release functions that take a non-const
// pointer (e.g. the compilation-info/message and limits wrappers). Same
// semantics; the only difference is the pointee constness.
template <typename T, void (*Release)(T *)>
struct MutArcDeleter {
    void operator()(T *ptr) const noexcept {
        if (ptr != nullptr) {
            Release(ptr);
        }
    }
};

template <typename T, void (*Release)(T *)>
using MutArcHandle = std::unique_ptr<T, MutArcDeleter<T, Release>>;

#endif // CANVAS_WEBGPU_ARCHANDLE_H
