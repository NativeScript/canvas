// NSCAudioStreamingSourceNode.m

#import "NSCAudioStreamingSourceNode.h"
#import "NSCAudioContext.h"
#import "NSCAudioLog.h"
#import "NSCAudioFileStreamDecoder.h"

@implementation NSCAudioStreamingSourceNode {
    AVAudioPlayerNode *_player;
    NSCAudioFileStreamDecoder *_decoder;
    dispatch_queue_t _decodeQueue;
    NSMutableArray<AVAudioPCMBuffer *> *_pendingBuffers;
    BOOL _isPlaying;
    BOOL _finalized;
}

- (instancetype)initWithContext:(NSCAudioContext *)context {
    AVAudioPlayerNode *p = [[AVAudioPlayerNode alloc] init];
    if (self = [super initWithContext:context node:p]) {
        _player = p;
        if ([NSThread isMainThread]) {
            [context.engine attachNode:_player];
        } else {
            dispatch_sync(dispatch_get_main_queue(), ^{ [context.engine attachNode:_player]; });
        }

        _decoder = [[NSCAudioFileStreamDecoder alloc] init];
        _decodeQueue = dispatch_queue_create("ns.c.streaming.decode", DISPATCH_QUEUE_SERIAL);
        _pendingBuffers = [NSMutableArray array];
        _isPlaying = NO;
        _finalized = NO;
    }
    return self;
}

- (BOOL)appendData:(NSData *)data error:(NSError * _Nullable * _Nullable)error {
    if (!data || data.length == 0) return YES;
    __weak typeof(self) weakSelf = self;
    dispatch_async(_decodeQueue, ^{
        __strong typeof(weakSelf) s = weakSelf; if (!s) return;
        NSError *e = nil;
        if (![s->_decoder appendData:data error:&e]) {
            NSCLogError(@"NSCAudioStreamingSourceNode: appendData parse failed: %@", e);
            return;
        }

        NSArray<AVAudioPCMBuffer *> *buffers = [s->_decoder decodeAvailableWithError:&e];
        if (!buffers || buffers.count == 0) return;

        for (AVAudioPCMBuffer *buf in buffers) {
            @synchronized (s->_pendingBuffers) {
                [s->_pendingBuffers addObject:buf];
            }
            dispatch_async(dispatch_get_main_queue(), ^{
                @try {
                    [s->_player scheduleBuffer:buf completionHandler:^{
                        dispatch_async(dispatch_get_main_queue(), ^{
                            @synchronized (s->_pendingBuffers) {
                                if (s->_pendingBuffers.count > 0) [s->_pendingBuffers removeObjectAtIndex:0];
                                BOOL nothingLeft = (s->_pendingBuffers.count == 0);
                                if (s->_finalized && nothingLeft) {
                                    if (s.onended) s.onended();
                                }
                            }
                        });
                    }];
                } @catch (NSException *ex) {
                    NSCLogError(@"NSCAudioStreamingSourceNode: scheduleBuffer exception: %@", ex);
                }
            });
        }
    });
    return YES;
}

- (void)endStream {
    __weak typeof(self) weakSelf = self;
    dispatch_async(_decodeQueue, ^{
        __strong typeof(weakSelf) s = weakSelf; if (!s) return;
        NSError *e = nil;
        NSArray<AVAudioPCMBuffer *> *finalBuffers = [s->_decoder finalizeAndDecodeWithError:&e];
        if (finalBuffers && finalBuffers.count > 0) {
            for (AVAudioPCMBuffer *buf in finalBuffers) {
                @synchronized (s->_pendingBuffers) { [s->_pendingBuffers addObject:buf]; }
                dispatch_async(dispatch_get_main_queue(), ^{
                    @try {
                        [s->_player scheduleBuffer:buf completionHandler:^{
                            dispatch_async(dispatch_get_main_queue(), ^{
                                @synchronized (s->_pendingBuffers) {
                                    if (s->_pendingBuffers.count > 0) [s->_pendingBuffers removeObjectAtIndex:0];
                                    BOOL nothingLeft = (s->_pendingBuffers.count == 0);
                                    if (s->_finalized && nothingLeft) {
                                        if (s.onended) s.onended();
                                    }
                                }
                            });
                        }];
                    } @catch (NSException *ex) {
                        NSCLogError(@"NSCAudioStreamingSourceNode: scheduleBuffer(final) exception: %@", ex);
                    }
                });
            }
        }
        s->_finalized = YES;
    });
}

- (void)start {
    NSCAudioContext *ctx = self.context;
    if (!ctx) return;
    if (!_isPlaying) {
        _isPlaying = YES;
        [ctx incrementActiveCount];
    }

    AVAudioPlayerNode *player = _player;
    if ([NSThread isMainThread]) {
        if (player.engine != ctx.engine) {
            @try { if (player.engine) [ctx detachNode:player fromEngine:player.engine]; } @catch (NSException *e) {}
            @try { [ctx.engine attachNode:player]; } @catch (NSException *e) {}
            @try { [ctx.engine connect:player to:ctx.engine.mainMixerNode format:ctx.format]; } @catch (NSException *e) {}
            @try { [ctx.engine prepare]; } @catch (NSException *e) {}
        }
        @try { [player play]; } @catch (NSException *e) {}
        return;
    }

    dispatch_sync(dispatch_get_main_queue(), ^{ [self start]; });
}

- (void)stop {
    @try { [_player stop]; } @catch (NSException *e) {}
    if (_isPlaying) {
        _isPlaying = NO;
        if (self.context) [self.context decrementActiveCount];
    }
    @synchronized (_pendingBuffers) { [_pendingBuffers removeAllObjects]; }
}

- (void)dealloc {
    [self stop];
}

@end
