#import "NSOperationQueueWrapper.h"
#include <Foundation/Foundation.h>

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

- (void)dealloc {}

- (void)addOperation:(void (^)())task {
    NSBlockOperation* operation = [NSBlockOperation blockOperationWithBlock:^{
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
        
        CFTypeRef ptr = (__bridge_retained CFTypeRef)objcWrapper;
    
        operationQueue = (void*)ptr;
    }else {
        NSOperationQueueWrapperObjC* objcWrapper = [[NSOperationQueueWrapperObjC alloc] init];
        CFTypeRef ptr = (__bridge_retained CFTypeRef)objcWrapper;
        operationQueue = (void*)ptr;
    }
   
}

NSOperationQueueWrapper::~NSOperationQueueWrapper() {
    if(operationQueue != nullptr){
        CFBridgingRelease(operationQueue);
        operationQueue = nullptr;
    }
}

void NSOperationQueueWrapper::addOperation(std::function<void()> task) {
    
    auto queue = (__bridge NSOperationQueueWrapperObjC*)(operationQueue);
    
    [queue addOperation:^{
        task();
    }];
}
