#import "NSOperationQueueWrapper.h"


#import <Foundation/Foundation.h>
#import "NSOperationQueueWrapper.h"

@interface NSOperationQueueWrapperObjC : NSObject
- (void)addOperation:(void (^)())task;
@end

@implementation NSOperationQueueWrapperObjC {
    NSOperationQueue* operationQueue;
}

- (instancetype)init {
    self = [super init];
    if (self) {
        operationQueue = [[NSOperationQueue alloc] init];
    }
    return self;
}

-(instancetype)initWithCurrentQueue {
    self = [super init];
    if (self) {
        operationQueue = [NSOperationQueue currentQueue];
    }
    return self;
}

- (void)dealloc {
    // In ARC, there's no need to manually release
}

- (void)addOperation:(void (^)())task {
    NSBlockOperation* operation = [NSBlockOperation blockOperationWithBlock:^{
        // Call the C++ task
        task();
    }];
    
    [operationQueue addOperation:operation];
}

- (NSOperationQueue*)getOperationQueue {
    return operationQueue;
}


@end

NSOperationQueueWrapper::NSOperationQueueWrapper(bool currentQueue) {
    if (currentQueue) {
        NSOperationQueueWrapperObjC* objcWrapper = [[NSOperationQueueWrapperObjC alloc] initWithCurrentQueue];
        operationQueue = [objcWrapper getOperationQueue];
    }else {
        NSOperationQueueWrapperObjC* objcWrapper = [NSOperationQueueWrapperObjC init];
        operationQueue = [objcWrapper getOperationQueue];
    }
   
}

NSOperationQueueWrapper::~NSOperationQueueWrapper() {}

void NSOperationQueueWrapper::addOperation(void (*task)()) {
    NSOperationQueueWrapperObjC* queue = static_cast<NSOperationQueueWrapperObjC*>(operationQueue);
    [queue addOperation:^{
        task();
    }];
}
