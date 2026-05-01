//
//  NSCVorbisDecoder.mm
//  AudioContextNative
//

#import "NSCVorbisDecoder.h"
#import <Foundation/Foundation.h>

#if __has_include(<vorbis/vorbisfile.h>)
#include <vorbis/vorbisfile.h>
#define NSC_HAVE_VORBIS 1
#else
#define NSC_HAVE_VORBIS 0
#endif

#include <vector>
#include <cstdint>
#include <cstring>

@implementation NSCVorbisDecoder

+ (nullable AVAudioPCMBuffer *)decodeVorbisData:(NSData *)data {
    if (!data || data.length == 0) return nil;
#if NSC_HAVE_VORBIS
    struct MemoryBuf { const unsigned char *data; size_t size; size_t pos; } mem;
    mem.data = (const unsigned char *)data.bytes;
    mem.size = (size_t)data.length;
    mem.pos = 0;

    static ov_callbacks callbacks = {
        /* read */ [](void *ptr, size_t size, size_t nmemb, void *datasource) -> size_t {
            struct MemoryBuf *m = (struct MemoryBuf *)datasource;
            size_t bytesRequested = size * nmemb;
            size_t bytesAvailable = (m->pos < m->size) ? (m->size - m->pos) : 0;
            size_t toCopy = bytesRequested < bytesAvailable ? bytesRequested : bytesAvailable;
            if (toCopy > 0) memcpy(ptr, m->data + m->pos, toCopy);
            m->pos += toCopy;
            return (toCopy / (size ? size : 1));
        },
        /* seek */ [](void *datasource, ogg_int64_t offset, int whence) -> int {
            struct MemoryBuf *m = (struct MemoryBuf *)datasource;
            size_t newpos = 0;
            if (whence == SEEK_SET) {
                if (offset < 0) return -1;
                newpos = (size_t)offset;
            } else if (whence == SEEK_CUR) {
                ogg_int64_t np = (ogg_int64_t)m->pos + offset;
                if (np < 0) return -1;
                newpos = (size_t)np;
            } else if (whence == SEEK_END) {
                ogg_int64_t np = (ogg_int64_t)m->size + offset;
                if (np < 0) return -1;
                newpos = (size_t)np;
            } else return -1;
            if (newpos > m->size) return -1;
            m->pos = newpos;
            return 0;
        },
        /* close */ [](void *datasource) -> int {
            (void)datasource; return 0;
        },
        /* tell */ [](void *datasource) -> long {
            struct MemoryBuf *m = (struct MemoryBuf *)datasource;
            return (long)m->pos;
        }
    };

    OggVorbis_File vf;
    int r = ov_open_callbacks(&mem, &vf, NULL, 0, callbacks);
    if (r != 0) {
        NSLog(@"NSCVorbisDecoder: ov_open_callbacks failed err=%d", r);
        return nil;
    }

    vorbis_info *vi = ov_info(&vf, -1);
    if (!vi) {
        NSLog(@"NSCVorbisDecoder: ov_info failed");
        ov_clear(&vf);
        return nil;
    }
    int channels = vi->channels;
    long rate = vi->rate;

    ogg_int64_t pcm_total = ov_pcm_total(&vf, -1);
    std::vector<float> pcm;
    if (pcm_total > 0) {
        try {
            pcm.reserve((size_t)pcm_total * (size_t)channels);
        } catch (...) {
        }
    }

    int bitstream = 0;
    float **pcm_channels = NULL;
    long samples = 0;
    ogg_int64_t totalSamples = 0;
    while ((samples = ov_read_float(&vf, &pcm_channels, 4096, &bitstream)) > 0) {
        for (int i = 0; i < samples; ++i) {
            for (int c = 0; c < channels; ++c) {
                float v = pcm_channels[c][i];
                pcm.push_back(v);
            }
        }
        totalSamples += samples;
    }
    if (samples < 0) {
			NSLog(@"NSCVorbisDecoder: ov_read_float failed code=%ld", samples);
        ov_clear(&vf);
        return nil;
    }

    ov_clear(&vf);

    if (totalSamples <= 0) return nil;

    AVAudioFormat *format = [[AVAudioFormat alloc] initWithCommonFormat:AVAudioPCMFormatFloat32 sampleRate:(double)rate channels:(AVAudioChannelCount)channels interleaved:NO];
    if (!format) return nil;

    AVAudioPCMBuffer *buffer = [[AVAudioPCMBuffer alloc] initWithPCMFormat:format frameCapacity:(AVAudioFrameCount)totalSamples];
    if (!buffer) return nil;
    buffer.frameLength = (AVAudioFrameCount)totalSamples;

    for (int c = 0; c < channels; ++c) {
        float *dst = buffer.floatChannelData[c];
        for (ogg_int64_t i = 0; i < totalSamples; ++i) {
            dst[i] = pcm[(size_t)(i * channels + c)];
        }
    }

    return buffer;
#else
    NSLog(@"NSCVorbisDecoder: libvorbis not available at compile time");
    return nil;
#endif
}

+ (nullable AVAudioPCMBuffer *)decodeVorbisFile:(NSString *)path {
    if (!path) return nil;
    FILE *f = fopen([path fileSystemRepresentation], "rb");
    if (!f) return nil;
    fseek(f, 0, SEEK_END);
    long size = ftell(f);
    fseek(f, 0, SEEK_SET);
    if (size <= 0) { fclose(f); return nil; }
    std::vector<uint8_t> buf((size_t)size);
    fread(buf.data(), 1, (size_t)size, f);
    fclose(f);
    NSData *data = [NSData dataWithBytes:buf.data() length:(NSUInteger)size];
    return [self decodeVorbisData:data];
}

@end
