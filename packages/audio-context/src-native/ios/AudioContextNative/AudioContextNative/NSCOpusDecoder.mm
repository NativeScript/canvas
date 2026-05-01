//
//  NSCOpusDecoder.mm
//  AudioContextNative
//

#import "NSCOpusDecoder.h"
#include "opusfile.h"
#include <vector>

@implementation NSCOpusDecoder

+ (nullable AVAudioPCMBuffer *)decodeOpusData:(NSData *)data {
    if (!data || data.length == 0) return nil;

    int err = 0;
    OggOpusFile *of = op_open_memory((const unsigned char *)data.bytes, (opus_int32)data.length, &err);
    if (!of || err != 0) {
        // If op_open_memory reports OP_ENOTFORMAT (commonly -132), the Ogg container
        // is not an Opus stream (it may be Vorbis). Try a lightweight detection
        // for the Opus identification header before emitting a hard failure.
        NSData *d = data;
        NSData *needle = [NSData dataWithBytes:"OpusHead" length:8];
        NSRange searchRange = NSMakeRange(0, (NSUInteger)MIN((size_t)1024, d.length));
        NSRange found = [d rangeOfData:needle options:0 range:searchRange];
        if (found.location == NSNotFound) {
            NSLog(@"NSCOpusDecoder: op_open_memory failed err=%d — OpusHead not found, likely a non-Opus Ogg (e.g. Vorbis). Falling back.", err);
            return nil;
        }
        NSLog(@"NSCOpusDecoder: op_open_memory failed err=%d (OpusHead present) — unexpected; falling back.", err);
        return nil;
    }

    const OpusHead *head = op_head(of, 0);
    int channels = head ? head->channel_count : 2;
    opus_int64 pcm_total = op_pcm_total(of, -1);
    if (pcm_total <= 0) {
        NSLog(@"NSCOpusDecoder: op_pcm_total returned %lld", (long long)pcm_total);
        op_free(of);
        return nil;
    }

    std::vector<float> pcm;
    try {
        pcm.resize((size_t)pcm_total * (size_t)channels);
    } catch (const std::bad_alloc &e) {
        NSLog(@"NSCOpusDecoder: failed to allocate pcm buffer: %s", e.what());
        op_free(of);
        return nil;
    }

    opus_int64 samples_read = 0;
    int samples = 0;
    while ((samples = op_read_float(of,
                                    pcm.data() + (size_t)samples_read * (size_t)channels,
                                    (int)(pcm_total - samples_read),
                                    NULL)) > 0) {
        samples_read += samples;
    }
    if (samples < 0) {
        NSLog(@"NSCOpusDecoder: op_read_float failed with code %d", samples);
        op_free(of);
        return nil;
    }
    op_free(of);
    if (samples_read <= 0) {
        NSLog(@"NSCOpusDecoder: no samples decoded (samples_read=%lld)", (long long)samples_read);
        return nil;
    }

    AVAudioFormat *format = [[AVAudioFormat alloc] initWithCommonFormat:AVAudioPCMFormatFloat32
                                                            sampleRate:48000
                                                              channels:(AVAudioChannelCount)channels
                                                           interleaved:NO];
    AVAudioPCMBuffer *buffer = [[AVAudioPCMBuffer alloc] initWithPCMFormat:format
                                                             frameCapacity:(AVAudioFrameCount)samples_read];
    if (!buffer) return nil;
    buffer.frameLength = (AVAudioFrameCount)samples_read;

    for (int ch = 0; ch < channels; ++ch) {
        float *dst = buffer.floatChannelData[ch];
        for (opus_int64 i = 0; i < samples_read; ++i) {
            dst[i] = pcm[(size_t)(i * channels + ch)];
        }
    }

    return buffer;
}

+ (nullable AVAudioPCMBuffer *)decodeOpusFile:(NSString *)path {
    if (!path) return nil;

    FILE *file = fopen([path fileSystemRepresentation], "rb");
    if (!file) return nil;

    fseek(file, 0, SEEK_END);
    long size = ftell(file);
    fseek(file, 0, SEEK_SET);
    if (size <= 0) { fclose(file); return nil; }

    std::vector<uint8_t> buf((size_t)size);
    fread(buf.data(), 1, (size_t)size, file);
    fclose(file);

    NSData *data = [NSData dataWithBytes:buf.data() length:(NSUInteger)size];
    return [self decodeOpusData:data];
}

@end
