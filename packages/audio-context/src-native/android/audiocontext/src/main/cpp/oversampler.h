#ifndef AUDIO_OVERSAMPLER_H
#define AUDIO_OVERSAMPLER_H

#include <cmath>
#include <cstring>
#include <vector>

// WaveShaperNode's '2x' and '4x' oversampling: half-band FIR up- and down-sampling by two per
// stage, like Chromium's UpSampler and DownSampler.
struct Oversampler {
    static constexpr int kTaps = 63;
    static constexpr int kHistory = kTaps - 1;

    // Per stage: the zero-stuffed input's history going up, the signal's going down.
    std::vector<float> up[2], down[2];
    std::vector<float> scratch;

    Oversampler() {
        for (int s = 0; s < 2; s++) {
            up[s].assign(kHistory, 0.0f);
            down[s].assign(kHistory, 0.0f);
        }
    }

    // Windowed sinc at a quarter of the high rate.
    static const float *taps() {
        static const std::vector<float> h = [] {
            std::vector<float> t(kTaps);
            const int mid = kTaps / 2;
            for (int k = 0; k < kTaps; k++) {
                const double x = (k - mid) * 0.5;
                const double sinc = x == 0.0 ? 1.0 : std::sin(M_PI * x) / (M_PI * x);
                const double a = 2.0 * M_PI * k / (kTaps - 1);
                const double blackman = 0.42 - 0.5 * std::cos(a) + 0.08 * std::cos(2.0 * a);
                t[k] = static_cast<float>(0.5 * sinc * blackman);
            }
            return t;
        }();
        return h.data();
    }

    // `n` frames to 2n.
    void upsample(int stage, const float *in, int n, float *out) {
        const float *h = taps();
        scratch.resize(static_cast<size_t>(kHistory + 2 * n));
        float *z = scratch.data();
        std::memcpy(z, up[stage].data(), kHistory * sizeof(float));
        for (int i = 0; i < n; i++) {
            z[kHistory + 2 * i] = in[i];
            z[kHistory + 2 * i + 1] = 0.0f;
        }
        for (int j = 0; j < 2 * n; j++) {
            const float *x = z + kHistory + j;
            float acc = 0.0f;
            for (int k = 0; k < kTaps; k++) acc += h[k] * x[-k];
            out[j] = 2.0f * acc;
        }
        std::memcpy(up[stage].data(), z + 2 * n, kHistory * sizeof(float));
    }

    // `n` frames to n / 2.
    void downsample(int stage, const float *in, int n, float *out) {
        const float *h = taps();
        scratch.resize(static_cast<size_t>(kHistory + n));
        float *z = scratch.data();
        std::memcpy(z, down[stage].data(), kHistory * sizeof(float));
        std::memcpy(z + kHistory, in, n * sizeof(float));
        for (int m = 0; m < n / 2; m++) {
            const float *x = z + kHistory + 2 * m;
            float acc = 0.0f;
            for (int k = 0; k < kTaps; k++) acc += h[k] * x[-k];
            out[m] = acc;
        }
        std::memcpy(down[stage].data(), z + n, kHistory * sizeof(float));
    }
};

#endif // AUDIO_OVERSAMPLER_H
