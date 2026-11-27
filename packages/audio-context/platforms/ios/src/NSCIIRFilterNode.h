#import <Foundation/Foundation.h>
#import "NSCProcessorNode.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;

@interface NSCIIRFilterNode : NSCProcessorNode

- (instancetype)initWithContext:(NSCAudioContext *)context
                    feedforward:(NSArray<NSNumber *> *)feedforward
                       feedback:(NSArray<NSNumber *> *)feedback;

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
