#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>
#import "NSCAudioNode.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;

@interface NSCAnalyserNode : NSCAudioNode

@property (nonatomic) NSInteger fftSize;
@property (nonatomic, readonly) NSInteger frequencyBinCount;
@property (nonatomic) double smoothingTimeConstant;
@property (nonatomic) double minDecibels;
@property (nonatomic) double maxDecibels;

- (instancetype)initWithContext:(NSCAudioContext *)context NS_DESIGNATED_INITIALIZER;
- (instancetype)initWithContext:(NSCAudioContext *)context node:(nullable AVAudioNode *)node NS_UNAVAILABLE;

- (void)getFloatTimeDomainData:(NSMutableData *)data;
- (void)getFloatTimeDomainDataWithByteOffset:(NSMutableData *)data :(NSInteger)byteOffset;
- (void)getByteTimeDomainData:(NSMutableData *)data;
- (void)getFloatFrequencyData:(NSMutableData *)data;
- (void)getFloatFrequencyDataWithByteOffset:(NSMutableData *)data :(NSInteger)byteOffset;
- (void)getByteFrequencyData:(NSMutableData *)data;
- (void)appendBufferToRing:(AVAudioPCMBuffer *)buffer;

- (void)setAcceptingFrames:(BOOL)accept;

- (AVAudioNode *)mixerNode;

@end

NS_ASSUME_NONNULL_END
