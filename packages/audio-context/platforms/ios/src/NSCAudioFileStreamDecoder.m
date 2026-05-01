//
// NSCAudioFileStreamDecoder.m
//

#import "NSCAudioFileStreamDecoder.h"
#import <AudioToolbox/AudioToolbox.h>

#ifndef kAudioConverterErr_NoMoreInputData
#define kAudioConverterErr_NoMoreInputData ((OSStatus)-1)
#endif

@interface NSCAudioFileStreamDecoderContext : NSObject
@property (nonatomic, strong) NSMutableData *compData;
@property (nonatomic, strong) NSMutableData *packetDescData;
@property (nonatomic, assign) AudioStreamBasicDescription asbd;
@property (nonatomic, assign) BOOL hasASBD;
@property (nonatomic, strong) NSData *magicCookie;
@property (nonatomic, assign) UInt32 packetCount;
@property (nonatomic, assign) UInt32 packetIndex;
@end

@implementation NSCAudioFileStreamDecoderContext
@end

static void propertyListenerProc(void *inClientData,
                                 AudioFileStreamID inAudioFileStream,
                                 AudioFileStreamPropertyID inPropertyID,
                                 UInt32 *ioFlags) {
    if (!inClientData) return;
    NSCAudioFileStreamDecoderContext *ctx = (__bridge NSCAudioFileStreamDecoderContext *)inClientData;

    if (inPropertyID == kAudioFileStreamProperty_DataFormat) {
        AudioStreamBasicDescription asbdLocal;
        UInt32 asbdSize = sizeof(asbdLocal);
        OSStatus s = AudioFileStreamGetProperty(inAudioFileStream, inPropertyID, &asbdSize, &asbdLocal);
        if (s == noErr) {
            ctx.asbd = asbdLocal;
            ctx.hasASBD = YES;
        }
    } else if (inPropertyID == kAudioFileStreamProperty_MagicCookieData) {
        UInt32 cookieSize = 0;
        OSStatus s = AudioFileStreamGetPropertyInfo(inAudioFileStream, inPropertyID, &cookieSize, NULL);
        if (s == noErr && cookieSize > 0) {
            void *buf = malloc(cookieSize);
            if (buf) {
                UInt32 got = cookieSize;
                s = AudioFileStreamGetProperty(inAudioFileStream, inPropertyID, &got, buf);
                if (s == noErr) {
                    ctx.magicCookie = [NSData dataWithBytes:buf length:got];
                }
                free(buf);
            }
        }
    }
}

static void packetsProc(void *inClientData,
                        UInt32 inNumberBytes,
                        UInt32 inNumberPackets,
                        const void *inInputData,
                        AudioStreamPacketDescription *inPacketDescriptions) {
    NSCAudioFileStreamDecoderContext *ctx = (__bridge NSCAudioFileStreamDecoderContext *)inClientData;
    if (!ctx.compData) ctx.compData = [NSMutableData data];
    NSUInteger base = ctx.compData.length;
    if (inNumberBytes > 0 && inInputData) {
        [ctx.compData appendBytes:inInputData length:inNumberBytes];
    }

    if (!ctx.packetDescData) ctx.packetDescData = [NSMutableData data];

    if (inPacketDescriptions && inNumberPackets > 0) {
        for (UInt32 i = 0; i < inNumberPackets; ++i) {
            AudioStreamPacketDescription pd = inPacketDescriptions[i];
            pd.mStartOffset += (SInt64)base;
            [ctx.packetDescData appendBytes:&pd length:sizeof(pd)];
        }
    } else if (inNumberBytes > 0) {
        AudioStreamPacketDescription pd;
        pd.mStartOffset = (SInt64)base;
        pd.mDataByteSize = inNumberBytes;
        pd.mVariableFramesInPacket = 0;
        [ctx.packetDescData appendBytes:&pd length:sizeof(pd)];
    }

    ctx.packetCount = (UInt32)(ctx.packetDescData.length / sizeof(AudioStreamPacketDescription));
}

static OSStatus audioConverterInputProc(AudioConverterRef inAudioConverter,
                                        UInt32 *ioNumberDataPackets,
                                        AudioBufferList *ioData,
                                        AudioStreamPacketDescription **outDataPacketDescription,
                                        void *inUserData) {
    NSCAudioFileStreamDecoderContext *ctx = (__bridge NSCAudioFileStreamDecoderContext *)inUserData;
    if (!ctx.packetDescData || ctx.packetIndex >= ctx.packetCount) {
        *ioNumberDataPackets = 0;
        return kAudioConverterErr_NoMoreInputData;
    }

    UInt32 packetsRequested = *ioNumberDataPackets;
    UInt32 packetsRemaining = ctx.packetCount - ctx.packetIndex;
    UInt32 packetsToProvide = packetsRequested < packetsRemaining ? packetsRequested : packetsRemaining;

    AudioStreamPacketDescription *pd = (AudioStreamPacketDescription *)ctx.packetDescData.bytes;
    AudioStreamPacketDescription *startDesc = &pd[ctx.packetIndex];

    UInt32 bytes = 0;
    for (UInt32 i = 0; i < packetsToProvide; ++i) bytes += pd[ctx.packetIndex + i].mDataByteSize;

    ioData->mNumberBuffers = 1;
    ioData->mBuffers[0].mNumberChannels = ctx.asbd.mChannelsPerFrame;
    ioData->mBuffers[0].mData = (void *)((uint8_t *)ctx.compData.bytes + startDesc->mStartOffset);
    ioData->mBuffers[0].mDataByteSize = bytes;

    if (outDataPacketDescription) {
        *outDataPacketDescription = startDesc;
    }

    *ioNumberDataPackets = packetsToProvide;
    ctx.packetIndex += packetsToProvide;
    return noErr;
}

@implementation NSCAudioFileStreamDecoder {
    NSCAudioFileStreamDecoderContext *_ctx;
    AudioFileStreamID _afs;
}

- (instancetype)init {
    self = [super init];
    if (self) {
        _ctx = [[NSCAudioFileStreamDecoderContext alloc] init];
        _ctx.compData = [NSMutableData data];
        _ctx.packetDescData = [NSMutableData data];
        _ctx.hasASBD = NO;
        _ctx.packetCount = 0;
        _ctx.packetIndex = 0;
        _afs = 0;
        OSStatus s = AudioFileStreamOpen((__bridge void *)_ctx, propertyListenerProc, packetsProc, 0, &_afs);
        (void)s;
    }
    return self;
}

- (void)reset {
    if (_afs) {
        AudioFileStreamClose(_afs);
        _afs = 0;
    }
    _ctx = [[NSCAudioFileStreamDecoderContext alloc] init];
    _ctx.compData = [NSMutableData data];
    _ctx.packetDescData = [NSMutableData data];
    _ctx.hasASBD = NO;
    _ctx.packetCount = 0;
    _ctx.packetIndex = 0;
    OSStatus s = AudioFileStreamOpen((__bridge void *)_ctx, propertyListenerProc, packetsProc, 0, &_afs);
    (void)s;
}

- (BOOL)appendData:(NSData *)data error:(NSError * _Nullable * _Nullable)error {
    if (!data || data.length == 0) return YES;
    if (!_afs) {
        if (error) *error = [NSError errorWithDomain:@"NSCAudioFileStreamDecoder" code:-1 userInfo:@{NSLocalizedDescriptionKey: @"AudioFileStream not open"}];
        return NO;
    }
    OSStatus s = AudioFileStreamParseBytes(_afs, (UInt32)data.length, data.bytes, 0);
    if (s != noErr) {
        if (error) *error = [NSError errorWithDomain:@"NSCAudioFileStreamDecoder" code:s userInfo:@{NSLocalizedDescriptionKey: @"AudioFileStreamParseBytes failed"}];
        return NO;
    }
    return YES;
}

- (nullable AVAudioPCMBuffer *)finalizeAndDecodeWithError:(NSError * _Nullable * _Nullable)error {
    if (_afs) {
        AudioFileStreamClose(_afs);
        _afs = 0;
    }

    if (!_ctx.hasASBD || _ctx.packetCount == 0 || _ctx.compData.length == 0) {
        if (error) *error = [NSError errorWithDomain:@"NSCAudioFileStreamDecoder" code:-1 userInfo:@{NSLocalizedDescriptionKey: @"no parsable audio packets"}];
        return nil;
    }

    AudioStreamBasicDescription srcASBD = _ctx.asbd;
    AVAudioFormat *dstFormat = [[AVAudioFormat alloc] initWithCommonFormat:AVAudioPCMFormatFloat32 sampleRate:srcASBD.mSampleRate channels:srcASBD.mChannelsPerFrame interleaved:NO];
    if (!dstFormat) {
        if (error) *error = [NSError errorWithDomain:@"NSCAudioFileStreamDecoder" code:-1 userInfo:@{NSLocalizedDescriptionKey: @"failed to create destination format"}];
        return nil;
    }

    AudioConverterRef converter = NULL;
    OSStatus s = AudioConverterNew(&srcASBD, dstFormat.streamDescription, &converter);
    if (s != noErr || !converter) {
        if (error) *error = [NSError errorWithDomain:@"NSCAudioFileStreamDecoder" code:s userInfo:@{NSLocalizedDescriptionKey: @"AudioConverterNew failed"}];
        return nil;
    }

    if (_ctx.magicCookie && _ctx.magicCookie.length > 0) {
        OSStatus ss = AudioConverterSetProperty(converter, kAudioConverterDecompressionMagicCookie, (UInt32)_ctx.magicCookie.length, (void *)_ctx.magicCookie.bytes);
        (void)ss;
    }

    UInt32 numChannels = (UInt32)dstFormat.channelCount;
    UInt32 bufferFrames = 4096;

    NSMutableArray<NSMutableData *> *channelAcc = [NSMutableArray arrayWithCapacity:numChannels];
    for (UInt32 c = 0; c < numChannels; ++c) [channelAcc addObject:[NSMutableData data]];

    size_t ablSize = sizeof(AudioBufferList) + (numChannels - 1) * sizeof(AudioBuffer);
    AudioBufferList *outABL = (AudioBufferList *)malloc(ablSize);
    memset(outABL, 0, ablSize);
    outABL->mNumberBuffers = numChannels;

    UInt32 totalFrames = 0;

    UInt32 primeFrames = 256;
    for (UInt32 c = 0; c < numChannels; ++c) {
        outABL->mBuffers[c].mNumberChannels = 1;
        outABL->mBuffers[c].mDataByteSize = primeFrames * sizeof(float);
        outABL->mBuffers[c].mData = malloc(outABL->mBuffers[c].mDataByteSize);
    }
    UInt32 ioPrimePackets = primeFrames;
    OSStatus primeStatus = AudioConverterFillComplexBuffer(converter, audioConverterInputProc, (__bridge void *)_ctx, &ioPrimePackets, outABL, NULL);
    if (primeStatus == noErr && ioPrimePackets > 0) {
        UInt32 bytesProducedPerChannel = ioPrimePackets * sizeof(float);
        for (UInt32 c = 0; c < numChannels; ++c) {
            [channelAcc[c] appendBytes:outABL->mBuffers[c].mData length:bytesProducedPerChannel];
            free(outABL->mBuffers[c].mData);
            outABL->mBuffers[c].mData = NULL;
        }
        totalFrames += ioPrimePackets;
    } else {
        for (UInt32 c = 0; c < numChannels; ++c) {
            if (outABL->mBuffers[c].mData) {
                free(outABL->mBuffers[c].mData);
                outABL->mBuffers[c].mData = NULL;
            }
        }
    }

    while (1) {
        for (UInt32 c = 0; c < numChannels; ++c) {
            outABL->mBuffers[c].mNumberChannels = 1;
            outABL->mBuffers[c].mDataByteSize = bufferFrames * sizeof(float);
            outABL->mBuffers[c].mData = malloc(outABL->mBuffers[c].mDataByteSize);
        }

        UInt32 ioOutputPackets = bufferFrames; 
        OSStatus cs = AudioConverterFillComplexBuffer(converter, audioConverterInputProc, (__bridge void *)_ctx, &ioOutputPackets, outABL, NULL);

        if (cs == noErr) {
            if (ioOutputPackets == 0) {
                for (UInt32 c = 0; c < numChannels; ++c) free(outABL->mBuffers[c].mData);
                break;
            }

            UInt32 bytesProducedPerChannel = ioOutputPackets * sizeof(float);
            for (UInt32 c = 0; c < numChannels; ++c) {
                [channelAcc[c] appendBytes:outABL->mBuffers[c].mData length:bytesProducedPerChannel];
                free(outABL->mBuffers[c].mData);
            }
            totalFrames += ioOutputPackets;
            continue;
        } else if (cs == kAudioConverterErr_NoMoreInputData) {
            for (UInt32 c = 0; c < numChannels; ++c) free(outABL->mBuffers[c].mData);
            break;
        } else {
            for (UInt32 c = 0; c < numChannels; ++c) {
                if (outABL->mBuffers[c].mData) free(outABL->mBuffers[c].mData);
            }
            AudioConverterDispose(converter);
            free(outABL);
            channelAcc = nil;
            if (error) *error = [NSError errorWithDomain:@"NSCAudioFileStreamDecoder" code:cs userInfo:@{NSLocalizedDescriptionKey: @"AudioConverterFillComplexBuffer failed"}];
            return nil;
        }
    }

    AudioConverterDispose(converter);
    free(outABL);

    if (totalFrames == 0) {
        channelAcc = nil;
        if (error) *error = [NSError errorWithDomain:@"NSCAudioFileStreamDecoder" code:-1 userInfo:@{NSLocalizedDescriptionKey: @"no decoded frames"}];
        return nil;
    }

    AVAudioPCMBuffer *pcm = [[AVAudioPCMBuffer alloc] initWithPCMFormat:dstFormat frameCapacity:totalFrames];
    pcm.frameLength = totalFrames;

    for (UInt32 c = 0; c < numChannels; ++c) {
        void *src = channelAcc[c].bytes;
        memcpy(pcm.floatChannelData[c], src, channelAcc[c].length);
    }

    (void)channelAcc;

    return pcm;
}

- (nullable NSArray<AVAudioPCMBuffer *> *)decodeAvailableWithError:(NSError * _Nullable * _Nullable)error {
    if (!_ctx.hasASBD || _ctx.packetCount == 0 || _ctx.compData.length == 0) {
        return @[];
    }

    AudioStreamBasicDescription srcASBD = _ctx.asbd;
    AVAudioFormat *dstFormat = [[AVAudioFormat alloc] initWithCommonFormat:AVAudioPCMFormatFloat32 sampleRate:srcASBD.mSampleRate channels:srcASBD.mChannelsPerFrame interleaved:NO];
    if (!dstFormat) {
        if (error) *error = [NSError errorWithDomain:@"NSCAudioFileStreamDecoder" code:-1 userInfo:@{NSLocalizedDescriptionKey: @"failed to create destination format"}];
        return nil;
    }

    AudioConverterRef converter = NULL;
    OSStatus s = AudioConverterNew(&srcASBD, dstFormat.streamDescription, &converter);
    if (s != noErr || !converter) {
        if (error) *error = [NSError errorWithDomain:@"NSCAudioFileStreamDecoder" code:s userInfo:@{NSLocalizedDescriptionKey: @"AudioConverterNew failed"}];
        return nil;
    }

    if (_ctx.magicCookie && _ctx.magicCookie.length > 0) {
        OSStatus ss = AudioConverterSetProperty(converter, kAudioConverterDecompressionMagicCookie, (UInt32)_ctx.magicCookie.length, (void *)_ctx.magicCookie.bytes);
        (void)ss;
    }

    UInt32 numChannels = (UInt32)dstFormat.channelCount;
    UInt32 bufferFrames = 4096;

    NSMutableArray<NSMutableData *> *channelAcc = [NSMutableArray arrayWithCapacity:numChannels];
    for (UInt32 c = 0; c < numChannels; ++c) [channelAcc addObject:[NSMutableData data]];

    size_t ablSize = sizeof(AudioBufferList) + (numChannels - 1) * sizeof(AudioBuffer);
    AudioBufferList *outABL = (AudioBufferList *)malloc(ablSize);
    memset(outABL, 0, ablSize);
    outABL->mNumberBuffers = numChannels;

    UInt32 totalFrames = 0;

    UInt32 primeFrames = 256;
    for (UInt32 c = 0; c < numChannels; ++c) {
        outABL->mBuffers[c].mNumberChannels = 1;
        outABL->mBuffers[c].mDataByteSize = primeFrames * sizeof(float);
        outABL->mBuffers[c].mData = malloc(outABL->mBuffers[c].mDataByteSize);
    }
    UInt32 ioPrimePackets = primeFrames;
    OSStatus primeStatus = AudioConverterFillComplexBuffer(converter, audioConverterInputProc, (__bridge void *)_ctx, &ioPrimePackets, outABL, NULL);
    if (primeStatus == noErr && ioPrimePackets > 0) {
        UInt32 bytesProducedPerChannel = ioPrimePackets * sizeof(float);
        for (UInt32 c = 0; c < numChannels; ++c) {
            [channelAcc[c] appendBytes:outABL->mBuffers[c].mData length:bytesProducedPerChannel];
            free(outABL->mBuffers[c].mData);
            outABL->mBuffers[c].mData = NULL;
        }
        totalFrames += ioPrimePackets;
    } else {
        for (UInt32 c = 0; c < numChannels; ++c) {
            if (outABL->mBuffers[c].mData) {
                free(outABL->mBuffers[c].mData);
                outABL->mBuffers[c].mData = NULL;
            }
        }
    }

    while (1) {
        for (UInt32 c = 0; c < numChannels; ++c) {
            outABL->mBuffers[c].mNumberChannels = 1;
            outABL->mBuffers[c].mDataByteSize = bufferFrames * sizeof(float);
            outABL->mBuffers[c].mData = malloc(outABL->mBuffers[c].mDataByteSize);
        }

        UInt32 ioOutputPackets = bufferFrames;
        OSStatus cs = AudioConverterFillComplexBuffer(converter, audioConverterInputProc, (__bridge void *)_ctx, &ioOutputPackets, outABL, NULL);

        if (cs == noErr) {
            if (ioOutputPackets == 0) {
                for (UInt32 c = 0; c < numChannels; ++c) free(outABL->mBuffers[c].mData);
                break;
            }
            UInt32 bytesProducedPerChannel = ioOutputPackets * sizeof(float);
            for (UInt32 c = 0; c < numChannels; ++c) {
                [channelAcc[c] appendBytes:outABL->mBuffers[c].mData length:bytesProducedPerChannel];
                free(outABL->mBuffers[c].mData);
            }
            totalFrames += ioOutputPackets;
            continue;
        } else if (cs == kAudioConverterErr_NoMoreInputData) {
            for (UInt32 c = 0; c < numChannels; ++c) free(outABL->mBuffers[c].mData);
            break;
        } else {
            for (UInt32 c = 0; c < numChannels; ++c) {
                if (outABL->mBuffers[c].mData) free(outABL->mBuffers[c].mData);
            }
            AudioConverterDispose(converter);
            free(outABL);
            channelAcc = nil;
            if (error) *error = [NSError errorWithDomain:@"NSCAudioFileStreamDecoder" code:cs userInfo:@{NSLocalizedDescriptionKey: @"AudioConverterFillComplexBuffer failed"}];
            return nil;
        }
    }

    AudioConverterDispose(converter);
    free(outABL);

    if (totalFrames == 0) {
        channelAcc = nil;
        return @[];
    }

    AVAudioPCMBuffer *pcm = [[AVAudioPCMBuffer alloc] initWithPCMFormat:dstFormat frameCapacity:totalFrames];
    pcm.frameLength = totalFrames;

    for (UInt32 c = 0; c < numChannels; ++c) {
        void *src = channelAcc[c].bytes;
        memcpy(pcm.floatChannelData[c], src, channelAcc[c].length);
    }

    return @[pcm];
}

+ (nullable AVAudioPCMBuffer *)decodeDataToPCMBuffer:(NSData *)data error:(NSError * _Nullable * _Nullable)error {
    NSCAudioFileStreamDecoder *dec = [[NSCAudioFileStreamDecoder alloc] init];
    if (![dec appendData:data error:error]) return nil;
    return [dec finalizeAndDecodeWithError:error];
}

@end
