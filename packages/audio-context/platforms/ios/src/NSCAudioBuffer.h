#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;

@interface NSCAudioBuffer : NSObject

@property (nonatomic, readonly) float sampleRate;
@property (nonatomic, readonly) NSInteger length;
@property (nonatomic, readonly) double duration;
@property (nonatomic, readonly) NSInteger numberOfChannels;

- (instancetype)initWithContext:(nullable NSCAudioContext *)context
                             id:(NSString *)identifier
                         buffer:(AVAudioPCMBuffer *)buffer NS_DESIGNATED_INITIALIZER;
- (instancetype)initWithLength:(NSInteger)length
              numberOfChannels:(NSInteger)numberOfChannels
                    sampleRate:(double)sampleRate;
- (instancetype)init NS_UNAVAILABLE;

- (nullable AVAudioPCMBuffer *)getBuffer;
- (nullable NSMutableData *)getChannelData:(NSInteger)channel;
- (void)copyFromChannel:(NSMutableArray *)destination :(NSInteger)channel :(NSInteger)startInChannel;
- (void)copyToChannel:(id)source :(NSInteger)channel :(NSInteger)startInChannel;

@end

NS_ASSUME_NONNULL_END
