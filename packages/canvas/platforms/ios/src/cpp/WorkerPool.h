#pragma once

#include <functional>
#include <vector>
#include <deque>
#include <thread>
#include <mutex>
#include <condition_variable>
#include <memory>

class WorkerPool {
public:
    static WorkerPool &Instance();

    void Enqueue(std::function<void()> task);

    ~WorkerPool();

    WorkerPool(const WorkerPool &) = delete;
    WorkerPool &operator=(const WorkerPool &) = delete;

private:
    explicit WorkerPool(size_t threads = 0);
    void WorkerLoop();

    std::vector<std::thread> workers_;
    std::deque<std::function<void()>> tasks_;
    std::mutex mtx_;
    std::condition_variable cv_;
    bool stop_ = false;
};
