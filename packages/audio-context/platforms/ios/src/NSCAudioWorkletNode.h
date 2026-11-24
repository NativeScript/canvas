#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>
#import "NSCAudioNode.h"
#import "NSCAudioParam.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;

/// An AudioWorkletNode with no inputs and one output: renders the quanta a Worker writes into
/// `ring` (layout in audio-worklet-ring.ts). In an offline context it plays every response in
/// order, so the output is sample-exact.
@interface NSCAudioWorkletNode : NSCAudioNode

@property (nonatomic, readonly) NSArray<NSCAudioParam *> *params;

- (instancetype)initWithContext:(NSCAudioContext *)context
                           ring:(NSData *)ring
                       channels:(NSInteger)channels
                      slotWords:(NSInteger)slotWords
                         header:(NSInteger)header
                   outputCounts:(NSInteger)outputCounts
                     outputData:(NSInteger)outputData
                   paramLengths:(NSInteger)paramLengths
                      paramData:(NSInteger)paramData
                  paramDefaults:(NSArray<NSNumber *> *)paramDefaults NS_DESIGNATED_INITIALIZER;
- (instancetype)initWithContext:(NSCAudioContext *)context node:(nullable AVAudioNode *)node NS_UNAVAILABLE;

/// The Worker stops serving the ring once it has drained.
- (void)close;

/// Offline: queues requests ahead, not past `limit`, and reports whether the next quantum's
/// response is in.
- (BOOL)prepareOfflineQuantumBefore:(uint64_t)limit;
/// Offline: renders silence from now on.
- (void)abandonOffline;

@end

NS_ASSUME_NONNULL_END
