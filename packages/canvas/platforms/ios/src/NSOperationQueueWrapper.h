#pragma once

#include <functional>
class NSOperationQueueWrapper {
public:
    NSOperationQueueWrapper(bool currentQueue, bool serial = false);
    ~NSOperationQueueWrapper();

    void addOperation(std::function<void()> task);

private:
void *operationQueue;
};
