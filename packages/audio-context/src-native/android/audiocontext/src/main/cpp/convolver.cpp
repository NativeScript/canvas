#include "convolver.h"

#include <algorithm>
#include <cmath>
#include <cstring>

namespace {
constexpr int N = 256;

struct Tables {
    float cosTw[N / 2];
    float sinTw[N / 2];
    int reverse[N];

    Tables() {
        for (int k = 0; k < N / 2; k++) {
            const double angle = -2.0 * M_PI * k / N;
            cosTw[k] = static_cast<float>(std::cos(angle));
            sinTw[k] = static_cast<float>(std::sin(angle));
        }
        int bits = 0;
        while ((1 << bits) < N) bits++;
        for (int i = 0; i < N; i++) {
            int r = 0;
            for (int b = 0; b < bits; b++) r |= ((i >> b) & 1) << (bits - 1 - b);
            reverse[i] = r;
        }
    }
};

const Tables &tables() {
    static const Tables t;
    return t;
}

// In-place forward radix-2 FFT of N points.
void fft(float *re, float *im) {
    const Tables &t = tables();
    for (int i = 0; i < N; i++) {
        const int j = t.reverse[i];
        if (i < j) {
            std::swap(re[i], re[j]);
            std::swap(im[i], im[j]);
        }
    }
    for (int len = 2; len <= N; len <<= 1) {
        const int half = len >> 1;
        const int step = N / len;
        for (int i = 0; i < N; i += len) {
            for (int k = 0; k < half; k++) {
                const float wr = t.cosTw[k * step];
                const float wi = t.sinTw[k * step];
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

// Spectrum bins 0..N/2 of N real samples.
void forward(const float *samples, float *binsRe, float *binsIm) {
    float re[N], im[N];
    std::memcpy(re, samples, sizeof re);
    std::fill(im, im + N, 0.0f);
    fft(re, im);
    std::memcpy(binsRe, re, (N / 2 + 1) * sizeof(float));
    std::memcpy(binsIm, im, (N / 2 + 1) * sizeof(float));
}

// N real samples from spectrum bins 0..N/2.
void inverse(const float *binsRe, const float *binsIm, float *samples) {
    float re[N], im[N];
    // The conjugate of the full, Hermitian spectrum: an inverse FFT through the forward one.
    for (int k = 0; k <= N / 2; k++) {
        re[k] = binsRe[k];
        im[k] = -binsIm[k];
    }
    for (int k = N / 2 + 1; k < N; k++) {
        re[k] = binsRe[N - k];
        im[k] = binsIm[N - k];
    }
    fft(re, im);
    for (int i = 0; i < N; i++) samples[i] = re[i] / N;
}
}

Convolver::Convolver(const float *const *channels, int count, int frames, float sampleRate, bool normalize)
        : count_(std::max(1, count)), partitions_(std::max(1, (frames + kBlock - 1) / kBlock)) {
    // The spec's equal-power normalization.
    float scale = 1.0f;
    if (normalize) {
        double power = 0.0;
        for (int c = 0; c < count_; c++) {
            for (int i = 0; i < frames; i++) power += static_cast<double>(channels[c][i]) * channels[c][i];
        }
        power = std::sqrt(power / (static_cast<double>(count_) * std::max(1, frames)));
        if (!std::isfinite(power) || power < 0.000125) power = 0.000125;
        double s = 0.00125 / power;
        if (sampleRate > 0) s *= 44100.0 / sampleRate;
        if (count_ == 4) s *= 0.5;
        scale = static_cast<float>(s);
    }
    responseRe_.resize(static_cast<size_t>(count_));
    responseIm_.resize(static_cast<size_t>(count_));
    float block[kSize];
    for (int c = 0; c < count_; c++) {
        responseRe_[c].assign(static_cast<size_t>(partitions_) * kBins, 0.0f);
        responseIm_[c].assign(static_cast<size_t>(partitions_) * kBins, 0.0f);
        for (int p = 0; p < partitions_; p++) {
            std::fill(block, block + kSize, 0.0f);
            const int start = p * kBlock;
            const int n = std::min(kBlock, frames - start);
            for (int i = 0; i < n; i++) block[i] = channels[c][start + i] * scale;
            forward(block, responseRe_[c].data() + p * kBins, responseIm_[c].data() + p * kBins);
        }
    }
    for (int c = 0; c < 2; c++) {
        previous_[c].assign(kBlock, 0.0f);
        historyRe_[c].assign(static_cast<size_t>(partitions_) * kBins, 0.0f);
        historyIm_[c].assign(static_cast<size_t>(partitions_) * kBins, 0.0f);
    }
    scratchRe_.resize(kBins);
    scratchIm_.resize(kBins);
    accRe_.resize(kBins);
    accIm_.resize(kBins);
}

void Convolver::convolve(int input, int response, float *out, bool add) {
    std::fill(accRe_.begin(), accRe_.end(), 0.0f);
    std::fill(accIm_.begin(), accIm_.end(), 0.0f);
    const float *hRe = responseRe_[static_cast<size_t>(response)].data();
    const float *hIm = responseIm_[static_cast<size_t>(response)].data();
    const float *xRe = historyRe_[input].data();
    const float *xIm = historyIm_[input].data();
    float *aRe = accRe_.data();
    float *aIm = accIm_.data();
    // Partition p meets the input from p quanta ago.
    for (int p = 0; p < partitions_; p++) {
        const int slot = (head_ - p + partitions_) % partitions_;
        const float *br = hRe + p * kBins, *bi = hIm + p * kBins;
        const float *xr = xRe + slot * kBins, *xi = xIm + slot * kBins;
        for (int k = 0; k < kBins; k++) {
            aRe[k] += xr[k] * br[k] - xi[k] * bi[k];
            aIm[k] += xr[k] * bi[k] + xi[k] * br[k];
        }
    }
    float samples[kSize];
    inverse(aRe, aIm, samples);
    // Overlap-save: the second half is the linear convolution.
    if (add) {
        for (int i = 0; i < kBlock; i++) out[i] += samples[kBlock + i];
    } else {
        std::memcpy(out, samples + kBlock, kBlock * sizeof(float));
    }
}

void Convolver::process(const float *const *in, int inputs, float *const *out) {
    inputs = std::max(1, std::min(inputs, 2));
    head_ = (head_ + 1) % partitions_;
    float block[kSize];
    for (int c = 0; c < inputs; c++) {
        std::memcpy(block, previous_[c].data(), kBlock * sizeof(float));
        if (in[c]) std::memcpy(block + kBlock, in[c], kBlock * sizeof(float));
        else std::fill(block + kBlock, block + kSize, 0.0f);
        std::memcpy(previous_[c].data(), block + kBlock, kBlock * sizeof(float));
        forward(block, historyRe_[c].data() + head_ * kBins, historyIm_[c].data() + head_ * kBins);
    }
    const int r1 = std::min(1, count_ - 1);
    if (outputChannels(inputs) == 1) {
        convolve(0, 0, out[0], false);
    } else if (count_ == 4) {
        // True stereo: L = inL*IR0 + inR*IR2, R = inL*IR1 + inR*IR3; a mono input feeds both.
        const int right = inputs == 2 ? 1 : 0;
        convolve(0, 0, out[0], false);
        convolve(right, 2, out[0], true);
        convolve(0, 1, out[1], false);
        convolve(right, 3, out[1], true);
    } else {
        convolve(0, 0, out[0], false);
        convolve(inputs == 2 ? 1 : 0, r1, out[1], false);
    }
}
