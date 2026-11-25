#ifndef AUDIO_HRTF_H
#define AUDIO_HRTF_H

#include <vector>

// The MIT KEMAR responses at one sample rate: per direction and ear, a kernel with its onset
// removed and that onset as a delay, like Chromium's HRTFKernel.
class HrtfDatabase {
public:
    // Built once per rate; the first call for a rate resamples the set.
    static const HrtfDatabase &forRate(int sampleRate);

    int taps() const { return taps_; }

    // Interpolated kernels and delays (frames) for an azimuth (degrees, right positive) and elevation.
    void lookup(double azimuth, double elevation, float *left, float *right, double &delayLeft,
                double &delayRight) const;

private:
    explicit HrtfDatabase(int sampleRate);

    int taps_ = 0;
    // Per measured response: taps_ left, taps_ right.
    std::vector<float> kernels_;
    std::vector<float> delays_;
    std::vector<int> rowStart_;
};

// PannerNode's 'HRTF' panning: each ear's input delayed by its interaural delay and convolved with
// its kernel, crossfading over a quantum when the direction changes.
class HrtfPanner {
public:
    static constexpr int kQuantum = 128;

    explicit HrtfPanner(int sampleRate);

    // Frames it rings after its input stops.
    int tailFrames() const { return taps_ + kLine / 2; }

    // One quantum; a mono input passes inR == inL.
    void process(double azimuth, double elevation, const float *inL, const float *inR, float *outL, float *outR);

    void reset();

private:
    static constexpr int kLine = 256;

    const HrtfDatabase &db_;
    int taps_;
    bool primed_ = false;
    double azimuth_ = 0.0, elevation_ = 0.0;
    std::vector<float> kernel_[2], previous_[2];
    double delay_[2] = {0.0, 0.0}, previousDelay_[2] = {0.0, 0.0};
    // Per ear: the input's delay line, and its delayed signal with taps_ - 1 frames of history.
    std::vector<float> line_[2];
    int linePos_ = 0;
    std::vector<float> delayed_[2];
};

#endif // AUDIO_HRTF_H
