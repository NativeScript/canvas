#import <Foundation/Foundation.h>
#import "NSCAudioNode.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;

@interface NSCIIRFilterNode : NSCAudioNode

- (instancetype)initWithContext:(NSCAudioContext *)context
                    feedforward:(NSArray<NSNumber *> *)feedforward
                       feedback:(NSArray<NSNumber *> *)feedback NS_DESIGNATED_INITIALIZER;
- (instancetype)initWithContext:(NSCAudioContext *)context node:(nullable AVAudioNode *)node NS_UNAVAILABLE;

- (void)getFrequencyResponse:(NSData *)frequencyHzData
                    magResponse:(NSMutableData *)magResponse
                  phaseResponse:(NSMutableData *)phaseResponse;
- (void)getFrequencyResponseMagResponsePhaseResponseWithByteOffsets:(NSData *)frequencyHzData
                                            :(NSInteger)frequencyHzByteOffset
                                            :(NSMutableData *)magResponse
                                            :(NSInteger)magResponseByteOffset
                                            :(NSMutableData *)phaseResponse
                                            :(NSInteger)phaseResponseByteOffset;

@end

NS_ASSUME_NONNULL_END
