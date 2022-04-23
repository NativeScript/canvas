//
// Created by Osei Fortune on 20/04/2022.
//

#ifndef CANVAS_NATIVE_OBJECTCACHEENTRY_H
#define CANVAS_NATIVE_OBJECTCACHEENTRY_H

#include "Common.h"

class ObjectCacheEntry {
public:
    ObjectCacheEntry(void *data_, v8::Persistent<v8::Object> *object_);

    void *data;
    v8::Persistent<v8::Object> *object;
};


#endif //CANVAS_NATIVE_OBJECTCACHEENTRY_H
