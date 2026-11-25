#import "NSCAudioContext.h"

@implementation NSCChannelSplitterNode {
    NSArray<NSCChannelMixerNode *> *_outputs;
}

- (instancetype)initWithContext:(NSCAudioContext *)context numberOfOutputs:(NSInteger)numberOfOutputs {
    if (!(self = [super initWithContext:context mode:NSCMixModeSum channels:MAX(1, numberOfOutputs) discrete:YES])) return nil;
    AVAudioFormat *format = [super formatForOutput:0];
    NSMutableArray<NSCChannelMixerNode *> *outputs = [NSMutableArray array];
    for (NSInteger i = 0; i < self.channels; i++) {
        NSCChannelMixerNode *output = [[NSCChannelMixerNode alloc] initWithContext:context mode:NSCMixModePick channels:1 discrete:YES];
        if (!output) return nil;
        [output setPick:i];
        [output connectMain:self.avNode bus:0 format:format];
        [outputs addObject:output];
    }
    _outputs = outputs;
    return self;
}

- (NSInteger)numberOfOutputs {
    return self.channels;
}

- (AVAudioNode *)outputNodeFor:(NSNumber *)output bus:(AVAudioNodeBus *)bus {
    *bus = 0;
    NSInteger i = output ? output.integerValue : 0;
    return i >= 0 && i < (NSInteger)_outputs.count ? _outputs[(NSUInteger)i].avNode : nil;
}

- (void)eachOutputNode:(NSNumber *)output block:(void (NS_NOESCAPE ^)(AVAudioNode *, NSNumber *))block {
    for (NSInteger i = 0; i < (NSInteger)_outputs.count; i++) {
        if (output && output.integerValue != i) continue;
        block(_outputs[(NSUInteger)i].avNode, @0);
    }
}

- (AVAudioFormat *)formatForOutput:(AVAudioNodeBus)bus {
    NSCChannelMixerNode *output = bus < _outputs.count ? _outputs[bus] : _outputs.firstObject;
    return output ? [output formatForOutput:0] : [super formatForOutput:bus];
}

@end
