//
//  NSCMediaElementAudioSourceNode.m
//

#import "NSCMediaElementAudioSourceNode.h"
#import "NSCMediaElementSourceTap.h"
#import "NSCAudioContext.h"
#import "NSCAudioLog.h"

@implementation NSCMediaElementAudioSourceNode

- (instancetype)initWithContext:(NSCAudioContext *)context node:(AVAudioNode *)node tap:(NSCMediaElementSourceTap *)tap {
    if (self = [super initWithContext:context node:node]) {
        _tap = tap;
    }
    return self;
}

- (void)detach {
    if (_tap) {
        @try { [_tap detach]; } @catch (NSException *ex) { NSCLogDebug(@"NSCMediaElementAudioSourceNode: tap detach threw: %@", ex); }
        _tap = nil;
    }

    AVAudioNode *av = self.avNode;
    if (av && av.engine) {
        @try { [self.context detachNode:av fromEngine:av.engine]; } @catch (NSException *ex) { NSCLogDebug(@"NSCMediaElementAudioSourceNode: detachNode threw: %@", ex); }
    }
}

- (void)dealloc {
    [self detach];
}

- (nullable NSCAudioParam *)getPlaybackRateParam {
    if (_tap) return [_tap getPlaybackRateParam];
    return nil;
}

@end
