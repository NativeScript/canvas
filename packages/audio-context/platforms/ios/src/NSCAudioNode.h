#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;
@class NSCAudioParam;

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

/// Applies param automation at the context's current time. Offline rendering calls it per quantum.
- (void)applyAutomation;

@end

NS_ASSUME_NONNULL_END
