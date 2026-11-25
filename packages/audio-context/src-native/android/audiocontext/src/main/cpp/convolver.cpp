#include "convolver.h"

#include <algorithm>
#include <chrono>
#include <cmath>
#include <cstring>

RealFft::RealFft(int size) : n_(size) {
    // A complex FFT of n/2 points over the even/odd samples, then an n-point split.
    const int m = n_ / 2;
    cos_.resize(static_cast<size_t>(m) + 1);
    sin_.resize(static_cast<size_t>(m) + 1);
    for (int k = 0; k <= m; k++) {
        const double angle = -2.0 * M_PI * k / n_;
        cos_[k] = static_cast<float>(std::cos(angle));
        sin_[k] = static_cast<float>(std::sin(angle));
    }
    int bits = 0;
    while ((1 << bits) < m) bits++;
    reverse_.resize(static_cast<size_t>(m));
    for (int i = 0; i < m; i++) {
        int r = 0;
        for (int b = 0; b < bits; b++) r |= ((i >> b) & 1) << (bits - 1 - b);
        reverse_[i] = r;
    }
    re_.resize(static_cast<size_t>(m));
    im_.resize(static_cast<size_t>(m));
}

// In-place forward complex FFT of n/2 points; its twiddles are every other n-point one.
void RealFft::transform(float *re, float *im) const {
    const int m = n_ / 2;
    for (int i = 0; i < m; i++) {
        const int j = reverse_[i];
        if (i < j) {
            std::swap(re[i], re[j]);
            std::swap(im[i], im[j]);
        }
    }
    for (int len = 2; len <= m; len <<= 1) {
        const int half = len >> 1;
        const int step = 2 * (m / len);
        for (int i = 0; i < m; i += len) {
            for (int k = 0; k < half; k++) {
                const float wr = cos_[k * step];
                const float wi = sin_[k * step];
                const int a = i + k;
                const int b = a + half;
                const float vr = re[b] * wr - im[b] * wi;
                const float vi = re[b] * wi + im[b] * wr;
                re[b] = re[a] - vr;
                im[b] = im[a] - vi;
                re[a] += vr;
                im[a] += vi;
            }
        }
    }
}

void RealFft::forward(const float *samples, float *re, float *im) {
    const int m = n_ / 2;
    float *zr = re_.data(), *zi = im_.data();
    for (int i = 0; i < m; i++) {
        zr[i] = samples[2 * i];
        zi[i] = samples[2 * i + 1];
    }
    transform(zr, zi);
    for (int k = 0; k <= m; k++) {
        const int a = k % m, b = (m - k) % m;
        const float er = 0.5f * (zr[a] + zr[b]), ei = 0.5f * (zi[a] - zi[b]);
        const float or_ = 0.5f * (zi[a] + zi[b]), oi = -0.5f * (zr[a] - zr[b]);
        re[k] = er + cos_[k] * or_ - sin_[k] * oi;
        im[k] = ei + cos_[k] * oi + sin_[k] * or_;
    }
}

void RealFft::inverse(const float *re, const float *im, float *samples) {
    const int m = n_ / 2;
    float *zr = re_.data(), *zi = im_.data();
    for (int k = 0; k < m; k++) {
        const float er = 0.5f * (re[k] + re[m - k]), ei = 0.5f * (im[k] - im[m - k]);
        const float dr = 0.5f * (re[k] - re[m - k]), di = 0.5f * (im[k] + im[m - k]);
        // Odd part: the difference times the conjugate twiddle.
        const float or_ = dr * cos_[k] + di * sin_[k], oi = di * cos_[k] - dr * sin_[k];
        // Conjugated so the forward transform inverts.
        zr[k] = er - oi;
        zi[k] = -(ei + or_);
    }
    transform(zr, zi);
    const float scale = 1.0f / static_cast<float>(m);
    for (int i = 0; i < m; i++) {
        samples[2 * i] = zr[i] * scale;
        samples[2 * i + 1] = -zi[i] * scale;
    }
}

struct Convolver::Stage {
    int block, offset, partitions, bins;
    bool async;
    RealFft fft;
    // Per response channel: partitions spectra of bins.
    std::vector<float> hRe[4], hIm[4];
    // Per input channel: a ring of partitions input spectra.
    std::vector<float> xRe[2], xIm[2];
    int head = 0;
    int64_t next = 0;
    std::vector<float> time, accRe, accIm;

    Stage(int block, int offset, int partitions, bool async)
            : block(block), offset(offset), partitions(partitions), bins(block + 1), async(async), fft(2 * block),
              time(static_cast<size_t>(2 * block)), accRe(static_cast<size_t>(block) + 1),
              accIm(static_cast<size_t>(block) + 1) {
        for (auto &x: xRe) x.assign(static_cast<size_t>(partitions) * bins, 0.0f);
        for (auto &x: xIm) x.assign(static_cast<size_t>(partitions) * bins, 0.0f);
    }
};

struct Convolver::Core {
    int count = 1;
    std::vector<std::unique_ptr<Stage>> stages;
    int64_t inMask = 0, outMask = 0;
    std::vector<float> in[2], out[2];
    // Render thread: frames taken so far.
    int64_t frame = 0;
    std::atomic<int64_t> written{0};
    std::atomic<int64_t> read{0};
    std::mutex mutex;
    std::condition_variable wake;
    std::atomic<bool> stop{false};

    // Block `b` of stage `s`: its spectrum against every partition, added to the output ring.
    void run(Stage &s, int64_t b) {
        const int B = s.block;
        const int64_t start = (b - 1) * B;
        s.head = (s.head + 1) % s.partitions;
        for (int c = 0; c < 2; c++) {
            for (int i = 0; i < 2 * B; i++) s.time[i] = in[c][(start + i) & inMask];
            s.fft.forward(s.time.data(), s.xRe[c].data() + s.head * s.bins, s.xIm[c].data() + s.head * s.bins);
        }
        // (input, response) pairs per output: Chromium's mapping for 1, 2 and 4 channels.
        static const int kPairs[3][2][4] = {
                {{0, 0, -1, -1}, {1, 0, -1, -1}},
                {{0, 0, -1, -1}, {1, 1, -1, -1}},
                {{0, 0, 1, 2},   {0, 1, 1, 3}},
        };
        const int layout = count == 4 ? 2 : (count == 2 ? 1 : 0);
        const int64_t at = b * B + s.offset;
        // Realtime: a block finished too late to play skips the part already played.
        const int64_t from = s.async ? read.load(std::memory_order_acquire) + 2 * kBlock : 0;
        for (int o = 0; o < 2; o++) {
            std::fill(s.accRe.begin(), s.accRe.end(), 0.0f);
            std::fill(s.accIm.begin(), s.accIm.end(), 0.0f);
            float *aRe = s.accRe.data(), *aIm = s.accIm.data();
            for (int t = 0; t < 4 && kPairs[layout][o][t] >= 0; t += 2) {
                const int input = kPairs[layout][o][t], response = kPairs[layout][o][t + 1];
                const float *hRe = s.hRe[response].data(), *hIm = s.hIm[response].data();
                const float *xRe = s.xRe[input].data(), *xIm = s.xIm[input].data();
                // Partition p meets the input from p blocks ago.
                for (int p = 0; p < s.partitions; p++) {
                    const int slot = (s.head - p + s.partitions) % s.partitions;
                    const float *br = hRe + p * s.bins, *bi = hIm + p * s.bins;
                    const float *xr = xRe + slot * s.bins, *xi = xIm + slot * s.bins;
                    for (int k = 0; k < s.bins; k++) {
                        aRe[k] += xr[k] * br[k] - xi[k] * bi[k];
                        aIm[k] += xr[k] * bi[k] + xi[k] * br[k];
                    }
                }
            }
            s.fft.inverse(aRe, aIm, s.time.data());
            // Overlap-save: the second half is the linear convolution.
            float *ring = out[o].data();
            for (int i = 0; i < B; i++) {
                if (at + i >= from) ring[(at + i) & outMask] += s.time[B + i];
            }
        }
    }

    void loop() {
        while (!stop.load(std::memory_order_acquire)) {
            const int64_t available = written.load(std::memory_order_acquire);
            bool ran = false;
            for (auto &s: stages) {
                if (!s->async) continue;
                while (!stop.load(std::memory_order_relaxed) && (s->next + 1) * s->block <= available) {
                    run(*s, s->next++);
                    ran = true;
                }
            }
            if (ran) continue;
            std::unique_lock<std::mutex> lock(mutex);
            if (written.load(std::memory_order_acquire) == available && !stop.load(std::memory_order_acquire)) {
                wake.wait_for(lock, std::chrono::milliseconds(20));
            }
        }
    }
};

namespace {
// (partition size, end of the response it covers); a stage starts at least two of its blocks in.
constexpr int kLayout[][2] = {{Convolver::kBlock, 4096}, {2048, 32768}, {16384, 1 << 30}};
}

Convolver::Convolver(const float *const *channels, int count, int frames, float sampleRate, bool normalize,
                     bool background)
        : count_(std::max(1, count)), tail_(std::max(1, (frames + kBlock - 1) / kBlock) + 1),
          core_(std::make_shared<Core>()) {
    frames = std::max(1, frames);
    // The spec's equal-power normalization.
    float scale = 1.0f;
    if (normalize) {
        double power = 0.0;
        for (int c = 0; c < count_; c++) {
            for (int i = 0; i < frames; i++) power += static_cast<double>(channels[c][i]) * channels[c][i];
        }
        power = std::sqrt(power / (static_cast<double>(count_) * frames));
        if (!std::isfinite(power) || power < 0.000125) power = 0.000125;
        double s = 0.00125 / power;
        if (sampleRate > 0) s *= 44100.0 / sampleRate;
        if (count_ == 4) s *= 0.5;
        scale = static_cast<float>(s);
    }
    Core &core = *core_;
    core.count = count_;
    int start = 0, maxBlock = kBlock, reach = kBlock;
    for (const auto &layout: kLayout) {
        if (start >= frames) break;
        const int block = layout[0];
        const int end = std::min(frames, layout[1]);
        const int partitions = (end - start + block - 1) / block;
        auto stage = std::make_unique<Stage>(block, start, partitions, background && block > kBlock);
        std::vector<float> time(static_cast<size_t>(2 * block));
        for (int c = 0; c < count_; c++) {
            stage->hRe[c].assign(static_cast<size_t>(partitions) * stage->bins, 0.0f);
            stage->hIm[c].assign(static_cast<size_t>(partitions) * stage->bins, 0.0f);
            for (int p = 0; p < partitions; p++) {
                std::fill(time.begin(), time.end(), 0.0f);
                const int from = start + p * block;
                const int n = std::min(block, frames - from);
                for (int i = 0; i < n; i++) time[i] = channels[c][from + i] * scale;
                stage->fft.forward(time.data(), stage->hRe[c].data() + p * stage->bins,
                                   stage->hIm[c].data() + p * stage->bins);
            }
        }
        maxBlock = std::max(maxBlock, block);
        reach = std::max(reach, start + block);
        start += partitions * block;
        core.stages.push_back(std::move(stage));
    }
    // Rings: two blocks of input with a block's slack; output up to the furthest write ahead.
    int64_t inSize = 1, outSize = 1;
    while (inSize < 4 * maxBlock) inSize <<= 1;
    while (outSize < 2 * reach + 2 * maxBlock) outSize <<= 1;
    core.inMask = inSize - 1;
    core.outMask = outSize - 1;
    for (int c = 0; c < 2; c++) {
        core.in[c].assign(static_cast<size_t>(inSize), 0.0f);
        core.out[c].assign(static_cast<size_t>(outSize), 0.0f);
    }
    bool async = false;
    for (auto &s: core.stages) async |= s->async;
    if (async) {
        std::shared_ptr<Core> shared = core_;
        worker_ = std::thread([shared]() { shared->loop(); });
    }
}

Convolver::~Convolver() {
    if (worker_.joinable()) {
        // The worker keeps the core alive until it notices; never wait on the render thread.
        core_->stop.store(true, std::memory_order_release);
        core_->wake.notify_all();
        worker_.detach();
    }
}

int Convolver::stages() const { return static_cast<int>(core_->stages.size()); }

void Convolver::process(const float *const *in, int inputs, float *const *out) {
    Core &core = *core_;
    inputs = std::max(1, std::min(inputs, 2));
    const int64_t frame = core.frame;
    // A mono input feeds both input channels.
    for (int c = 0; c < 2; c++) {
        const float *src = in[std::min(c, inputs - 1)];
        float *ring = core.in[c].data();
        for (int i = 0; i < kBlock; i++) ring[(frame + i) & core.inMask] = src ? src[i] : 0.0f;
    }
    const int64_t end = frame + kBlock;
    for (auto &s: core.stages) {
        if (s->async || end % s->block != 0) continue;
        core.run(*s, end / s->block - 1);
        s->next = end / s->block;
    }
    core.written.store(end, std::memory_order_release);
    if (worker_.joinable() && core.mutex.try_lock()) {
        core.wake.notify_one();
        core.mutex.unlock();
    }
    const int outputs = outputChannels(inputs);
    for (int o = 0; o < 2; o++) {
        float *ring = core.out[o].data();
        for (int i = 0; i < kBlock; i++) {
            float &v = ring[(frame + i) & core.outMask];
            if (o < outputs) out[o][i] = v;
            v = 0.0f;
        }
    }
    core.read.store(end, std::memory_order_release);
    core.frame = end;
}
