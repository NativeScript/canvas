#include "WorkerPool.h"
#include <algorithm>
#include <iostream>

WorkerPool &WorkerPool::Instance() {
    static WorkerPool pool;
    return pool;
}

WorkerPool::WorkerPool(size_t threads) {
    if (threads == 0) {
        unsigned hc = std::thread::hardware_concurrency();
        threads = hc == 0 ? 4 : hc;
    }

    workers_.reserve(threads);
    for (size_t i = 0; i < threads; ++i) {
        workers_.emplace_back([this]() { this->WorkerLoop(); });
    }
}

WorkerPool::~WorkerPool() {
    {
        std::unique_lock<std::mutex> lock(mtx_);
        stop_ = true;
    }
    cv_.notify_all();
    for (auto &t: workers_) {
        if (t.joinable()) t.join();
    }
}

void WorkerPool::Enqueue(std::function<void()> task) {
    {
        std::unique_lock<std::mutex> lock(mtx_);
        tasks_.emplace_back(std::move(task));
    }
    cv_.notify_one();
}

void WorkerPool::WorkerLoop() {
    while (true) {
        std::function<void()> task;
        {
            std::unique_lock<std::mutex> lock(mtx_);
            cv_.wait(lock, [this]() { return stop_ || !tasks_.empty(); });
            if (stop_ && tasks_.empty()) return;
            task = std::move(tasks_.front());
            tasks_.pop_front();
        }
        try {
            task();
        } catch (...) {
            // swallow exceptions to avoid terminating the pool
        }
    }
}
