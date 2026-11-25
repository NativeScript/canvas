#include "audio_context.h"
#include "audio_internal.h"

#include <algorithm>
#include <cmath>

namespace {
constexpr int kQuantum = 128;
}

void NativeEngine::renderOfflineFrames(int64_t startFrame, int frames, const std::vector<float *> &out) {
#ifdef HAS_OBOE
    if (!offline_ || frames <= 0) return;
    std::lock_guard<std::mutex> lock(offlineRenderMutex_);
    const int channels = std::max(1, streamChannels_);
    const double frameNs = 1e9 / static_cast<double>(streamSampleRate_ > 0 ? streamSampleRate_ : 48000);
    offlineScratch_.resize(static_cast<size_t>(kQuantum * channels));
    offlineLimitFrame_ = startFrame + frames;
    for (int done = 0; done < frames; done += kQuantum) {
        const int64_t frame = startFrame + done;
        offlineFrame_ = frame;
        nowNs_.store(offlineBaseNs_ + std::llround(static_cast<double>(frame) * frameNs), std::memory_order_relaxed);
        onAudioReady(nullptr, offlineScratch_.data(), kQuantum);
        const int n = std::min(kQuantum, frames - done);
        for (int c = 0; c < std::min(channels, static_cast<int>(out.size())); c++) {
            if (!out[c]) continue;
            float *dst = out[c] + done;
            for (int i = 0; i < n; i++) dst[i] = offlineScratch_[static_cast<size_t>(i * channels + c)];
        }
    }
    const int64_t end = startFrame + frames;
    nowNs_.store(offlineBaseNs_ + std::llround(static_cast<double>(end) * frameNs), std::memory_order_relaxed);
#endif
}
