#import <Accelerate/Accelerate.h>
#import <objc/runtime.h>
#import "NSCAudioContext.h"

static void *bufferKey = &bufferKey;

@implementation NSCAudioBuffer {
    NSCAudioContext *_context;
    AVAudioPCMBuffer *_buffer;
    NSString *_identifier;
    double _storedSampleRate;
    NSInteger _storedChannels;
}

- (instancetype)initWithContext:(NSCAudioContext *)context id:(NSString *)identifier buffer:(AVAudioPCMBuffer *)buffer {
    if (self = [super init]) {
        _context = context;
        _identifier = [identifier copy];
        _buffer = buffer;
        if (buffer) {
            _storedSampleRate = buffer.format.sampleRate;
            _storedChannels = (NSInteger)buffer.format.channelCount;
        }
    }
    return self;
}

- (instancetype)initWithLength:(NSInteger)length numberOfChannels:(NSInteger)numberOfChannels sampleRate:(double)sampleRate {
    AVAudioChannelCount ch = (AVAudioChannelCount)MAX(1, numberOfChannels);
    double sr = sampleRate > 0 ? sampleRate : 48000.0;
    AVAudioFormat *fmt = [[AVAudioFormat alloc] initWithCommonFormat:AVAudioPCMFormatFloat32 sampleRate:sr channels:ch interleaved:NO];
    AVAudioFrameCount frames = (AVAudioFrameCount)MAX(0, length);
    AVAudioPCMBuffer *buf = [[AVAudioPCMBuffer alloc] initWithPCMFormat:fmt frameCapacity:frames];
    buf.frameLength = frames;
    return [self initWithContext:nil id:[[NSUUID UUID] UUIDString] buffer:buf];
}

- (AVAudioPCMBuffer *)getBuffer {
    return _buffer;
}

- (float)sampleRate {
    if (_buffer) return (float)_buffer.format.sampleRate;
    return (float)_storedSampleRate;
}

- (NSInteger)length {
    if (!_buffer) return 0;
    return (NSInteger)_buffer.frameLength;
}

- (double)duration {
    double sr = (double)self.sampleRate;
    if (sr == 0.0) return 0.0;
    return (double)self.length / sr;
}

- (NSInteger)numberOfChannels {
    if (_buffer) return (NSInteger)_buffer.format.channelCount;
    return _storedChannels;
}

- (NSMutableData *)getChannelData:(NSInteger)channel {
    if (!_buffer) return nil;
    float * const *fdata = _buffer.floatChannelData;
    if (!fdata) return nil;
    NSInteger channels = (NSInteger)_buffer.format.channelCount;
    if (channel < 0 || channel >= channels) return nil;
    AVAudioFrameCount frames = _buffer.frameLength;
    float *ptr = fdata[channel];
    NSUInteger byteCount = (NSUInteger)frames * sizeof(float);
    NSMutableData *data = [NSMutableData dataWithBytesNoCopy:ptr length:byteCount freeWhenDone:NO];
    objc_setAssociatedObject(data, bufferKey, _buffer, OBJC_ASSOCIATION_RETAIN_NONATOMIC);
    return data;
}

- (void)copyFromChannel:(id)destination :(NSInteger)channel :(NSInteger)startInChannel {
    if (!_buffer || !destination) return;
    float * const *fdata = _buffer.floatChannelData;
    if (!fdata) return;
    NSInteger channels = (NSInteger)_buffer.format.channelCount;
    if (channel < 0 || channel >= channels) return;
    AVAudioFrameCount frames = _buffer.frameLength;
    NSInteger start = MAX(0, startInChannel);
    if ((NSUInteger)start >= (NSUInteger)frames) return;
    const float *src = fdata[channel] + start;
    NSUInteger srcFloats = (NSUInteger)frames - (NSUInteger)start;

    if ([destination isKindOfClass:[NSMutableArray class]]) {
        NSMutableArray *destArr = (NSMutableArray *)destination;
        NSUInteger count = destArr.count == 0 ? srcFloats : MIN(destArr.count, srcFloats);
        BOOL append = destArr.count == 0;
        for (NSUInteger i = 0; i < count; ++i) {
            NSNumber *boxed = @(src[i]);
            if (append) [destArr addObject:boxed];
            else destArr[i] = boxed;
        }
        return;
    }

    if ([destination isKindOfClass:[NSData class]]) {
        NSMutableData *md = (NSMutableData *)destination;
        float *dst = (float *)md.mutableBytes;
        if (!dst) {
            const void *raw = ((NSData *)destination).bytes;
            dst = (float *)(void *)raw;
        }
        if (!dst) return;
        NSUInteger destFloats = md.length / sizeof(float);
        NSUInteger count = MIN(srcFloats, destFloats);
        if (count == 0) return;
        memcpy(dst, src, count * sizeof(float));
        return;
    }
}

- (void)copyToChannel:(id)source :(NSInteger)channel :(NSInteger)startInChannel {
    if (!_buffer) return;
    float * const *fdata = _buffer.floatChannelData;
    if (!fdata) return;
    NSInteger channels = (NSInteger)_buffer.format.channelCount;
    if (channel < 0 || channel >= channels) return;
    AVAudioFrameCount capacity = _buffer.frameCapacity;
    NSInteger start = MAX(0, startInChannel);
    if ((NSUInteger)start >= (NSUInteger)capacity) return;
    float *ptr = fdata[channel];
    
    if ([source isKindOfClass:[NSArray class]]) {
        NSArray *arr = (NSArray *)source;
        NSUInteger available = (NSUInteger)capacity - (NSUInteger)start;
        NSUInteger count = MIN(arr.count, available);
        for (NSUInteger i = 0; i < count; ++i) {
            id obj = arr[i];
            if ([obj isKindOfClass:[NSNumber class]]) {
                ptr[start + i] = [obj floatValue];
            }
        }
        NSUInteger newLen = MAX((NSUInteger)_buffer.frameLength, (NSUInteger)start + count);
        _buffer.frameLength = (AVAudioFrameCount)newLen;
        return;
    }
    
    if ([source isKindOfClass:[NSData class]] || [source isKindOfClass:[NSMutableData class]]) {
        NSData *raw = (NSData *)source;
        NSUInteger available = (NSUInteger)capacity - (NSUInteger)start;
        if (raw.length >= 4) {
            NSUInteger floats = raw.length / 4;
            NSUInteger n = MIN(floats, available);
            [raw getBytes:(ptr + start) length:n * sizeof(float)];
            NSUInteger newLen = MAX((NSUInteger)_buffer.frameLength, (NSUInteger)start + n);
            _buffer.frameLength = (AVAudioFrameCount)newLen;
            return;
        }
        if (raw.length >= 2) {
            NSUInteger shorts = raw.length / 2;
            NSUInteger n = MIN(shorts, available);
            const int16_t *iptr = (const int16_t *)raw.bytes;
            float *dst = ptr + start;
            vDSP_vflt16(iptr, 1, dst, 1, (vDSP_Length)n);
            float scale = 1.0f / 32768.0f;
            vDSP_vsmul(dst, 1, &scale, dst, 1, (vDSP_Length)n);
            NSUInteger newLen = MAX((NSUInteger)_buffer.frameLength, (NSUInteger)start + n);
            _buffer.frameLength = (AVAudioFrameCount)newLen;
            return;
        }
    }
}

@end
