//
// Created by Osei Fortune on 20/04/2022.
//

#include "ObjectCacheEntry.h"

ObjectCacheEntry::ObjectCacheEntry(void *data_, v8::Persistent<v8::Object>* object_) : data(data_),
                                                                                                      object(object_) {}
