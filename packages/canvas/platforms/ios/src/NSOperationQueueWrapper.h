#include <functional>
class NSOperationQueueWrapper {
public:
    NSOperationQueueWrapper(bool currentQueue);
    ~NSOperationQueueWrapper();

    void addOperation(std::function<void()> task);

private:
void *operationQueue;
};
