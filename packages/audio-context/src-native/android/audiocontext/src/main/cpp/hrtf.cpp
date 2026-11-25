#include "hrtf.h"
#include "hrtf_kemar.h"

#include <algorithm>
#include <cmath>
#include <cstring>
#include <map>
#include <memory>
#include <mutex>

namespace {
// Band-limited resampling of a 44.1 kHz response to `ratio` times the rate, keeping its gain.
void resample(const float *src, int length, double ratio, float *dst, int taps) {
    if (ratio == 1.0) {
        std::memcpy(dst, src, std::min(length, taps) * sizeof(float));
        for (int n = length; n < taps; n++) dst[n] = 0.0f;
        return;
    }
    const double cutoff = std::min(1.0, ratio);
    const double half = 16.0 / cutoff;
    for (int n = 0; n < taps; n++) {
        const double t = n / ratio;
        double acc = 0.0;
        const int lo = std::max(0, static_cast<int>(std::ceil(t - half)));
        const int hi = std::min(length - 1, static_cast<int>(std::floor(t + half)));
        for (int k = lo; k <= hi; k++) {
            const double x = t - k;
            const double sinc = x == 0.0 ? 1.0 : std::sin(M_PI * cutoff * x) / (M_PI * cutoff * x);
            const double window = 0.5 * (1.0 + std::cos(M_PI * x / half));
            acc += src[k] * cutoff * sinc * window;
        }
        dst[n] = static_cast<float>(acc / ratio);
    }
}
}

const HrtfDatabase &HrtfDatabase::forRate(int sampleRate) {
    static std::mutex mutex;
    static std::map<int, std::unique_ptr<HrtfDatabase>> databases;
    std::lock_guard<std::mutex> lock(mutex);
    auto &db = databases[sampleRate];
    if (!db) db.reset(new HrtfDatabase(sampleRate));
    return *db;
}

HrtfDatabase::HrtfDatabase(int sampleRate) {
    const int L = kemar::kLength;
    const int count = kemar::kResponses;
    const double ratio = (sampleRate > 0 ? sampleRate : kemar::kRate) / static_cast<double>(kemar::kRate);
    taps_ = static_cast<int>(std::ceil(L * ratio));
    rowStart_.resize(kemar::kRows);
    for (int r = 0, start = 0; r < kemar::kRows; r++) {
        rowStart_[r] = start;
        start += kemar::kAzimuths[r];
    }
    // Onsets: the first sample within 20 dB of the ear's peak, less one for the rise.
    std::vector<int> onset(static_cast<size_t>(count) * 2);
    int earliest = L;
    for (int i = 0; i < count * 2; i++) {
        const int16_t *h = kemar::kData + static_cast<size_t>(i) * L;
        int peak = 0;
        for (int k = 0; k < L; k++) peak = std::max(peak, std::abs(static_cast<int>(h[k])));
        int at = 0;
        while (at < L - 1 && std::abs(static_cast<int>(h[at])) < peak / 10) at++;
        onset[i] = std::max(0, at - 1);
        earliest = std::min(earliest, onset[i]);
    }
    // Front and centre has unit power across both ears, as equal-power panning does.
    const int front = rowStart_[4];
    double power = 0.0;
    for (int k = 0; k < 2 * L; k++) {
        const double v = kemar::kData[static_cast<size_t>(front) * 2 * L + k] / 32768.0;
        power += v * v;
    }
    const float scale = static_cast<float>(1.0 / (32768.0 * std::sqrt(power)));
    kernels_.assign(static_cast<size_t>(count) * 2 * taps_, 0.0f);
    delays_.resize(static_cast<size_t>(count) * 2);
    std::vector<float> shifted(L);
    for (int i = 0; i < count * 2; i++) {
        const int16_t *h = kemar::kData + static_cast<size_t>(i) * L;
        std::fill(shifted.begin(), shifted.end(), 0.0f);
        for (int k = onset[i]; k < L; k++) shifted[k - onset[i]] = h[k] * scale;
        resample(shifted.data(), L, ratio, kernels_.data() + static_cast<size_t>(i) * taps_, taps_);
        delays_[i] = static_cast<float>((onset[i] - earliest) * ratio);
    }
}

void HrtfDatabase::lookup(double azimuth, double elevation, float *left, float *right, double &delayLeft,
                          double &delayRight) const {
    std::fill(left, left + taps_, 0.0f);
    std::fill(right, right + taps_, 0.0f);
    delayLeft = delayRight = 0.0;
    if (!std::isfinite(azimuth)) azimuth = 0.0;
    if (!std::isfinite(elevation)) elevation = 0.0;
    const double el = std::max(-40.0, std::min(90.0, elevation));
    const double row = (el + 40.0) / 10.0;
    const int r0 = std::min(kemar::kRows - 1, static_cast<int>(std::floor(row)));
    const int r1 = std::min(kemar::kRows - 1, r0 + 1);
    const double rowFrac = row - r0;
    double az = std::fmod(azimuth, 360.0);
    if (az < 0.0) az += 360.0;
    auto add = [&](int r, int i, double weight) {
        if (weight <= 0.0) return;
        const int circle = kemar::kCircle[r];
        i %= circle;
        // Measured on the right; the left side mirrors it with the ears swapped.
        bool swap = false;
        if (i >= kemar::kAzimuths[r]) {
            i = circle - i;
            swap = true;
        }
        const size_t entry = static_cast<size_t>(rowStart_[r] + i) * 2;
        const float *kl = kernels_.data() + (entry + (swap ? 1 : 0)) * taps_;
        const float *kr = kernels_.data() + (entry + (swap ? 0 : 1)) * taps_;
        const auto w = static_cast<float>(weight);
        for (int k = 0; k < taps_; k++) {
            left[k] += w * kl[k];
            right[k] += w * kr[k];
        }
        delayLeft += weight * delays_[entry + (swap ? 1 : 0)];
        delayRight += weight * delays_[entry + (swap ? 0 : 1)];
    };
    for (int pass = 0; pass < 2; pass++) {
        const int r = pass == 0 ? r0 : r1;
        const double rw = pass == 0 ? 1.0 - rowFrac : rowFrac;
        if (rw <= 0.0) continue;
        const double pos = az / (360.0 / kemar::kCircle[r]);
        const int i0 = static_cast<int>(std::floor(pos));
        const double frac = pos - i0;
        add(r, i0, rw * (1.0 - frac));
        add(r, i0 + 1, rw * frac);
    }
}

HrtfPanner::HrtfPanner(int sampleRate) : db_(HrtfDatabase::forRate(sampleRate)), taps_(db_.taps()) {
    for (int e = 0; e < 2; e++) {
        kernel_[e].assign(static_cast<size_t>(taps_), 0.0f);
        previous_[e].assign(static_cast<size_t>(taps_), 0.0f);
        line_[e].assign(kLine, 0.0f);
        delayed_[e].assign(static_cast<size_t>(taps_ - 1 + kQuantum), 0.0f);
    }
}

void HrtfPanner::reset() {
    primed_ = false;
    linePos_ = 0;
    for (int e = 0; e < 2; e++) {
        std::fill(line_[e].begin(), line_[e].end(), 0.0f);
        std::fill(delayed_[e].begin(), delayed_[e].end(), 0.0f);
    }
}

void HrtfPanner::process(double azimuth, double elevation, const float *inL, const float *inR, float *outL,
                         float *outR) {
    bool fade = false;
    if (!primed_ || azimuth != azimuth_ || elevation != elevation_) {
        for (int e = 0; e < 2; e++) {
            kernel_[e].swap(previous_[e]);
            previousDelay_[e] = delay_[e];
        }
        db_.lookup(azimuth, elevation, kernel_[0].data(), kernel_[1].data(), delay_[0], delay_[1]);
        // Never further back than the line holds.
        for (double &d: delay_) d = std::min(d, static_cast<double>(kLine - kQuantum - 2));
        if (!primed_) {
            for (int e = 0; e < 2; e++) {
                previous_[e] = kernel_[e];
                previousDelay_[e] = delay_[e];
            }
        }
        fade = primed_;
        primed_ = true;
        azimuth_ = azimuth;
        elevation_ = elevation;
    }
    constexpr int mask = kLine - 1;
    const int history = taps_ - 1;
    for (int e = 0; e < 2; e++) {
        const float *in = e == 0 ? inL : inR;
        float *out = e == 0 ? outL : outR;
        float *line = line_[e].data();
        float *d = delayed_[e].data();
        for (int f = 0; f < kQuantum; f++) line[(linePos_ + f) & mask] = in ? in[f] : 0.0f;
        // The interaural delay, ramped across the quantum when it moves.
        for (int f = 0; f < kQuantum; f++) {
            const double delay = previousDelay_[e] + (delay_[e] - previousDelay_[e]) * (f + 1) / kQuantum;
            const double pos = linePos_ + f - delay;
            const double base = std::floor(pos);
            const auto frac = static_cast<float>(pos - base);
            const int i0 = static_cast<int>(base) & mask;
            const float a = line[i0];
            d[history + f] = frac == 0.0f ? a : a + (line[(i0 + 1) & mask] - a) * frac;
        }
        const float *k = kernel_[e].data();
        const float *p = previous_[e].data();
        for (int f = 0; f < kQuantum; f++) {
            const float *x = d + history + f;
            float acc = 0.0f;
            for (int t = 0; t < taps_; t++) acc += k[t] * x[-t];
            if (fade) {
                float old = 0.0f;
                for (int t = 0; t < taps_; t++) old += p[t] * x[-t];
                const float w = static_cast<float>(f + 1) / kQuantum;
                acc = old + (acc - old) * w;
            }
            out[f] = acc;
        }
        std::memmove(d, d + kQuantum, history * sizeof(float));
    }
    linePos_ = (linePos_ + kQuantum) & mask;
}
