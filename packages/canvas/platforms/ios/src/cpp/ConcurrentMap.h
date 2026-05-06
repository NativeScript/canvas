//
// Created by Osei Fortune on 10/06/2022.
//

#pragma once

#include <shared_mutex>
#include "include/robin_hood.h"

template<class TKey, class TValue>
class ConcurrentMap {
public:
    void Insert(const TKey &key, TValue value) {
        std::unique_lock<std::shared_mutex> lock(mutex_);
        container_[key] = std::move(value);
    }

    TValue Get(const TKey &key) {
        bool found;
        return Get(key, found);
    }


    TValue Get(const TKey &key, bool &found) {
        std::shared_lock<std::shared_mutex> lock(mutex_);
        auto it = container_.find(key);
        found = it != container_.end();
        if (found) return it->second;
        return nullptr;
    }

    template<class Factory>
    TValue GetOrCreate(const TKey &key, Factory factory) {
        return GetOrCreate(key, factory, nullptr);
    }

    template<class Factory>
    TValue GetOrCreate(const TKey &key, Factory factory, bool *created) {
        {
            std::shared_lock<std::shared_mutex> readLock(mutex_);
            auto it = container_.find(key);
            if (it != container_.end()) {
                if (created != nullptr) *created = false;
                return it->second;
            }
        }

        std::unique_lock<std::shared_mutex> writeLock(mutex_);
        auto it = container_.find(key);
        if (it != container_.end()) {
            if (created != nullptr) *created = false;
            return it->second;
        }
        TValue value = factory();
        if (created != nullptr) *created = true;
        container_[key] = value;
        return value;
    }

    bool ContainsKey(const TKey &key) {
        std::shared_lock<std::shared_mutex> lock(mutex_);
        return container_.find(key) != container_.end();
    }

    void Remove(const TKey &key) {
        std::unique_lock<std::shared_mutex> lock(mutex_);
        container_.erase(key);
    }

    ConcurrentMap() = default;

    ConcurrentMap(const ConcurrentMap &) = delete;

    ConcurrentMap &operator=(const ConcurrentMap &) = delete;

private:
    std::shared_mutex mutex_;
    robin_hood::unordered_map<TKey, TValue> container_;
};
