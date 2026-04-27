#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>

#import "NSCAudioNode.h"
#import "NSCAudioBuffer.h"
#import "NSCAudioParam.h"
#import "NSCAudioScheduledSourceNode.h"
#import "NSCAudioOscillatorNode.h"
#import "NSCAudioBufferSourceNode.h"
#import "NSCGainNode.h"
#import "NSCBiquadNode.h"
#import "NSCAudioPannerNode.h"
#import "NSCStereoPannerNode.h"
#import "NSCDelayNode.h"
#import "NSCConstantSourceNode.h"
#import "NSCAnalyserNode.h"
#import "NSCWaveShaperNode.h"
#import "NSCIIRFilterNode.h"
#import "NSCConvolverNode.h"
#import "NSCPeriodicWave.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCOfflineAudioContext;

#pragma mark - NSCAudioContext

@interface NSCAudioContext : NSObject

@property (nonatomic, readonly) AVAudioEngine *engine;
@property (nonatomic, readonly) AVAudioEnvironmentNode *environmentNode;
@property (nonatomic, strong, readonly) NSCAudioNode *destination;
@property (nonatomic, readonly) double sampleRate;
@property (nonatomic, readonly) double currentTime;
@property (nonatomic, readonly) AVAudioFormat *format;

@property (nonatomic, strong) NSMutableDictionary<NSValue *, NSMutableDictionary<NSNumber *, NSCAudioNode *> *> *voiceGainByOutput;
@property (nonatomic, strong) NSMutableDictionary<NSValue *, NSMutableDictionary<NSNumber *, NSCAudioNode *> *> *voiceFilterByOutput;
@property (nonatomic, strong) NSMutableDictionary<NSValue *, NSMutableDictionary<NSNumber *, NSCAudioNode *> *> *voicePannerByOutput;

- (instancetype)initWithSampleRate:(double)sampleRate;
- (instancetype)initWithSampleRate:(double)sampleRate latencyHint:(double)latencyHint NS_DESIGNATED_INITIALIZER;
- (instancetype)init NS_UNAVAILABLE;

- (void)resume;
- (void)suspend;
- (void)ensureEnvironmentNodeAttached;

- (void)registerPendingNode:(NSCAudioBufferSourceNode *)node;
- (void)unregisterPendingNode:(NSCAudioBufferSourceNode *)node;
- (void)resumeAllPendingNodes;
- (void)ensureMainMixerConnectedToOutput;

- (void)registerNodeWrapper:(NSCAudioNode *)node;
- (nullable NSCAudioNode *)nodeWrapperForAVNode:(AVAudioNode *)avNode;
- (NSArray<NSCAnalyserNode *> *)allAnalyserWrappers;


- (BOOL)setSinkId:(nullable NSString *)deviceId;
- (NSString *)currentSinkId;

- (void)incrementActiveCount;
- (void)decrementActiveCount;
- (BOOL)hasActiveAudio;

- (double)extraLatencySeconds;

- (nullable NSCAudioBuffer *)decodeAudioData:(NSString *)base64;
- (nullable NSCAudioBuffer *)decodeAudioDataFromFile:(NSString *)path;
- (nullable NSCAudioBuffer *)decodeAudioDataFromData:(NSData *)data;

- (void)decodeAudioDataAsync:(NSString *)base64 :(void (^)(NSCAudioBuffer * _Nullable))completion;
- (void)decodeAudioDataFromFileAsync:(NSString *)path :(void (^)(NSCAudioBuffer * _Nullable))completion;
- (void)decodeAudioDataFromDataAsync:(NSData *)data :(void (^)(NSCAudioBuffer * _Nullable))completion;

- (NSCGainNode *)createGainNode;
- (NSCBiquadNode *)createBiquadNode:(nullable NSString *)type frequency:(double)frequency Q:(double)Q gain:(double)gain;
- (NSCBiquadNode *)createBiquadNode;
- (NSCAudioPannerNode *)createPannerNode;
- (NSCAudioOscillatorNode *)createOscillatorNode:(nullable NSString *)type frequency:(double)frequency;
- (nullable NSCAudioBufferSourceNode *)createBufferSourceNode:(nullable NSCAudioBuffer *)buffer;
- (NSCStereoPannerNode *)createStereoPannerNode:(double)pan;
- (NSCDelayNode *)createDelayNode:(double)delayTime maxDelayTime:(double)maxDelayTime;
- (NSCConstantSourceNode *)createConstantSourceNode:(double)offset;
- (NSCAnalyserNode *)createAnalyserNode;
- (NSCWaveShaperNode *)createWaveShaperNode;
- (NSCIIRFilterNode *)createIIRFilterNode:(NSArray<NSNumber *> *)feedforward feedback:(NSArray<NSNumber *> *)feedback;
- (NSCConvolverNode *)createConvolverNode;


FOUNDATION_EXPORT void NSCAudioContext_scheduleResumeOnEngineStart(AVAudioEngine *engine, double delay);
FOUNDATION_EXPORT void NSCAudioContext_cancelScheduledResume(AVAudioEngine *engine);

FOUNDATION_EXPORT void *NSCProducerTokenKey;

+ (BOOL)startEngineWithRetry:(AVAudioEngine *)engine
                    attempts:(NSInteger)attempts
                       label:(NSString *)label
             asyncCompletion:(nullable void (^)(BOOL))completion;


+ (NSMutableData *)marshalMutableData:(NSMutableData *)data;

@end

#pragma mark - NSCOfflineAudioContext

@interface NSCOfflineAudioContext : NSCAudioContext

- (instancetype)init;
- (void)configure:(NSNumber *)channels :(NSNumber *)lengthInFrames :(NSNumber *)sampleRate;

- (void)decodeAudioDataFromFileAsyncOffline:(NSString *)path :(void (^)(NSCAudioBuffer * _Nullable))completion;
- (void)decodeAudioDataFromDataAsyncOffline:(NSData *)data :(void (^)(NSCAudioBuffer * _Nullable))completion;
- (nullable NSCAudioBufferSourceNode *)createBufferSourceNodeOffline:(nullable NSCAudioBuffer *)buffer;
- (void)startRenderingAsync:(void (^)(NSCAudioBuffer * _Nullable))completion;

@end

NS_ASSUME_NONNULL_END
