#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>
#import "NSCAudioNode.h"
#import "NSCAudioParam.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;

/// An AudioWorkletNode: an audio unit that feeds its inputs to a Worker and plays the quanta it
/// writes back into `ring` (layout in audio-worklet-ring.ts). Each input sums its connections in a
/// mixer. In an offline context the output is sample-exact.
@interface NSCAudioWorkletNode : NSCAudioNode

@property (nonatomic, readonly) NSArray<NSCAudioParam *> *params;
@property (nonatomic, readonly) NSInteger numberOfInputs;
@property (nonatomic, readonly) NSInteger numberOfOutputs;

/// `inputChannels` and `outputChannels` hold a channel count per input and output; `outputData` a
/// word offset per output.
- (nullable instancetype)initWithContext:(NSCAudioContext *)context
                                    ring:(NSData *)ring
                           inputChannels:(NSArray<NSNumber *> *)inputChannels
                          outputChannels:(NSArray<NSNumber *> *)outputChannels
                               slotWords:(NSInteger)slotWords
                                  header:(NSInteger)header
                             inputCounts:(NSInteger)inputCounts
                               inputData:(NSInteger)inputData
                            outputCounts:(NSInteger)outputCounts
                              outputData:(NSArray<NSNumber *> *)outputData
                            paramLengths:(NSInteger)paramLengths
                               paramData:(NSInteger)paramData
                           paramDefaults:(NSArray<NSNumber *> *)paramDefaults NS_DESIGNATED_INITIALIZER;
- (instancetype)initWithContext:(NSCAudioContext *)context node:(nullable AVAudioNode *)node NS_UNAVAILABLE;

/// The Worker stops serving the ring once it has drained.
- (void)close;

/// Offline: queues requests ahead, not past `limit`, and reports whether the next quantum's
/// response is in. A node with inputs waits for each response as it renders instead.
- (BOOL)prepareOfflineQuantumBefore:(uint64_t)limit;
/// Offline: renders silence from now on.
- (void)abandonOffline;

@end

NS_ASSUME_NONNULL_END
