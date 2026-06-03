//
// ArcHandle.h — RAII owner for the Rust Arc handles exposed across the canvas C ABI.
//

#ifndef CANVAS_WEBGPU_ARCHANDLE_H
#define CANVAS_WEBGPU_ARCHANDLE_H

#include <memory>

// unique_ptr owner for a Rust Arc handle exposed across the C ABI. reset() runs the
// matching ..._release once and nulls the pointer; the GC finalizer's later drop is
// then a no-op. ArcHandle for const release fns, MutArcHandle for non-const.
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
