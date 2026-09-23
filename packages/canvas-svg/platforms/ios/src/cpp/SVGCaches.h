// Per-isolate FunctionTemplate cache for SVGDocument/SVGNode.
#pragma once

#include "SVGCommon.h"
#include <memory>
#include <mutex>
#include <unordered_map>

class SVGCaches {
public:
    std::unique_ptr<v8::Persistent<v8::FunctionTemplate>> SVGDocumentTmpl;
    std::unique_ptr<v8::Persistent<v8::FunctionTemplate>> SVGNodeTmpl;

    static SVGCaches *Get(v8::Isolate *isolate) {
        static std::mutex mutex;
        static std::unordered_map<v8::Isolate *, std::unique_ptr<SVGCaches>> caches;

        std::lock_guard<std::mutex> lock(mutex);
        auto it = caches.find(isolate);
        if (it != caches.end()) {
            return it->second.get();
        }
        auto *cache = new SVGCaches();
        caches.emplace(isolate, std::unique_ptr<SVGCaches>(cache));
        return cache;
    }
};
