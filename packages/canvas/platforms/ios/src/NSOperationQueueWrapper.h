#import <Foundation/Foundation.h>


class NSOperationQueueWrapper {
public:
    NSOperationQueueWrapper(bool currentQueue);
    ~NSOperationQueueWrapper();

    void addOperation(void (*task)());

private:
    NSOperationQueue *operationQueue;
};
