#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>
#import <stdatomic.h>

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;
@class NSCAudioParam;
@class NSCChannelMixerNode;

@interface NSCAudioNode : NSObject

@property (nonatomic, weak, nullable) NSCAudioContext *context;
@property (nonatomic, readonly) AVAudioNode *avNode;

- (instancetype)initWithContext:(NSCAudioContext *)context node:(nullable AVAudioNode *)node NS_DESIGNATED_INITIALIZER;
- (instancetype)init NS_UNAVAILABLE;

 -(void)handleConnectFrom:(nullable NSCAudioNode *)source
                   output:(nullable NSNumber *)output
                    input:(nullable NSNumber *)input;
 -(void)handleDisconnectFrom:(nullable NSCAudioNode *)source
                      output:(nullable NSNumber *)output
                       input:(nullable NSNumber *)input;

 -(void)handleConnectedTo:(nullable NSCAudioNode *)destination
                   output:(nullable NSNumber *)output
                    input:(nullable NSNumber *)input;
 -(void)handleDisconnectedFrom:(nullable NSCAudioNode *)destination
                        output:(nullable NSNumber *)output
                         input:(nullable NSNumber *)input;

- (void)connectTo:(NSCAudioNode *)destination;
- (void)connectTo:(NSCAudioNode *)destination output:(nullable NSNumber *)output;
- (void)connectTo:(NSCAudioNode *)destination output:(nullable NSNumber *)output input:(nullable NSNumber *)input;

/// Adds this node's output, downmixed to mono, to `param` per frame; a no-op for params not read per frame.
- (void)connectToParam:(NSCAudioParam *)param output:(nullable NSNumber *)output;
- (void)disconnectFromParam:(NSCAudioParam *)param output:(nullable NSNumber *)output;

- (void)disconnect;
- (void)disconnectOutput:(NSNumber *)output;
- (void)disconnectTo:(NSCAudioNode *)destination;
- (void)disconnectTo:(NSCAudioNode *)destination output:(nullable NSNumber *)output;
- (void)disconnectTo:(NSCAudioNode *)destination output:(nullable NSNumber *)output input:(nullable NSNumber *)input;

/// Web Audio channelCount, channelCountMode (0 max, 1 clamped-max, 2 explicit) and
/// channelInterpretation (0 speakers, 1 discrete); -1 leaves a value as is.
- (void)setChannelConfig:(NSInteger)count mode:(NSInteger)mode interpretation:(NSInteger)interpretation;
@property (nonatomic, readonly) NSInteger channelCount;
@property (nonatomic, readonly) NSInteger channelCountMode;
@property (nonatomic, readonly) NSInteger channelInterpretation;
/// Called after the channel config changes; applies it to `channelPort`.
- (void)channelConfigChanged;
/// Where this node's inputs mix, if it has one mixer for them.
- (nullable NSCChannelMixerNode *)channelPort;

/// The channel count `output` plays at, read on the render thread; NULL when it is its bus's.
- (nullable const _Atomic UInt32 *)logicalChannelCellForOutput:(NSInteger)output;
/// Sets this node's own output channel count (sources).
- (void)setLogicalChannels:(NSInteger)channels;
/// Removes nodes this one added to the engine for itself; called when the context closes or it is collected.
- (void)detachHelpers;

/// Applies param automation at the context's current time. Offline rendering calls it per quantum.
- (void)applyAutomation;

/// The format this node plays at before it is connected; sources set theirs, else the context's.
@property (nonatomic, strong, nullable) AVAudioFormat *nativeFormat;
/// The format a new connection from `bus` should use: the bus's own once it is connected.
- (AVAudioFormat *)formatForOutput:(AVAudioNodeBus)bus;
/// The node this one plays from: an output stage once it has one, else `avNode`.
- (nullable AVAudioNode *)outputAVNode;
/// The node and bus that play `output` (nil: output 0).
- (nullable AVAudioNode *)outputNodeFor:(nullable NSNumber *)output bus:(AVAudioNodeBus *)bus;
/// Calls `block` with each node playing `output` (nil: every output) and its bus (nil: every bus).
- (void)eachOutputNode:(nullable NSNumber *)output block:(void (NS_NOESCAPE ^)(AVAudioNode *node, NSNumber *_Nullable bus))block;
/// Swaps the node this wrapper plays from; only before it is connected.
- (void)replaceAVNode:(AVAudioNode *)node;
/// Where audio connected to `param` goes; made on first use.
- (nullable NSCAudioNode *)inputForParam:(NSCAudioParam *)param;
/// Connects a param's sink where it renders before this node reads the param.
- (void)attachParamSink:(AVAudioNode *)sink;
/// YES for nodes that play through an output stage when a param of theirs takes audio.
@property (nonatomic, readonly) BOOL usesOutputStage;
/// Plays this node through a stage mixing `channels` to each destination; made once.
- (void)ensureOutputStageWithChannels:(NSInteger)channels;

@end

/// Allocates a unit's render resources if the engine has not.
void NSCEnsureAllocated(AVAudioNode *node);
/// Whether audio flows from `from` to `to` in the engine graph.
BOOL NSCHasPath(AVAudioEngine *engine, AVAudioNode *from, AVAudioNode *to);
/// Adds a connection from `src`'s bus to `dst`'s bus, keeping `src`'s other connections.
BOOL NSCConnectAdding(NSCAudioContext *_Nullable context, AVAudioNode *src, AVAudioNodeBus fromBus, AVAudioNode *dst, AVAudioNodeBus toBus, AVAudioFormat *_Nullable format);
/// Removes `src`'s connections from `fromBus` (nil: every bus) to `dst` (nil: any) on `toBus` (nil: any).
void NSCDisconnectRemoving(NSCAudioContext *_Nullable context, AVAudioNode *src, NSNumber *_Nullable fromBus, AVAudioNode *_Nullable dst, NSNumber *_Nullable toBus);

NS_ASSUME_NONNULL_END
