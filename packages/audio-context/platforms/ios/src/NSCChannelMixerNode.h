#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>
#import "NSCAudioNode.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;

typedef NS_ENUM(NSInteger, NSCMixMode) {
    /// Sums every connection into `channels` channels on one output.
    NSCMixModeSum = 0,
    /// Plays channel `pick` of bus 0 (one ChannelSplitterNode output).
    NSCMixModePick = 1,
    /// Down-mixes each connection to mono into the channel its input index names (ChannelMergerNode).
    NSCMixModeMerge = 2,
    /// Plays bus 0 as is, after rendering the other busses (a source's param inputs).
    NSCMixModePass = 3,
};

/// Each connection on its own bus at its source's channel count, mixed with Web Audio's
/// speakers (1, 2, 4 and 6 channels) or discrete rules; the output maps to its channels the same way.
@interface NSCChannelMixerNode : NSCAudioNode

- (instancetype)initWithContext:(NSCAudioContext *)context
                           mode:(NSCMixMode)mode
                       channels:(NSInteger)channels
                       discrete:(BOOL)discrete NS_DESIGNATED_INITIALIZER;
- (instancetype)initWithContext:(NSCAudioContext *)context node:(nullable AVAudioNode *)node NS_UNAVAILABLE;

@property (nonatomic, readonly) NSInteger channels;
/// Pick mode: the channel of bus 0 this node plays.
- (void)setPick:(NSInteger)pick;
/// Sum mode: mixes to no more channels than the widest connection has.
- (void)setClamp:(BOOL)clamp;
- (void)setDiscrete:(BOOL)discrete;
/// The channels the last render mixed to; read on the render thread after pulling this node.
- (const UInt32 *)mixedChannels;

/// Connects `node`'s bus 0 to a bus rendered before the others; its audio is not mixed.
- (void)connectEarly:(AVAudioNode *)node format:(AVAudioFormat *)format;
/// Pass mode: connects `node`'s `bus` as the audio this node plays.
- (void)connectMain:(AVAudioNode *)node bus:(AVAudioNodeBus)bus format:(AVAudioFormat *)format;

@end

NS_ASSUME_NONNULL_END
