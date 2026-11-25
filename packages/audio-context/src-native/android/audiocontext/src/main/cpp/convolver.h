#ifndef AUDIO_CONVOLVER_H
#define AUDIO_CONVOLVER_H

#include <atomic>
#include <condition_variable>
#include <memory>
#include <mutex>
#include <thread>
#include <vector>

// Radix-2 FFT of real signals, size a power of two.
class RealFft {
public:
    explicit RealFft(int size);

    int size() const { return n_; }

    // `n` samples to bins 0..n/2.
    void forward(const float *samples, float *re, float *im);

    // Bins 0..n/2 to `n` samples.
    void inverse(const float *re, const float *im, float *samples);

private:
    void transform(float *re, float *im) const;

    int n_;
    std::vector<float> cos_, sin_;
    std::vector<int> reverse_;
    std::vector<float> re_, im_;
};

// ConvolverNode: non-uniformly partitioned FFT convolution, like Chromium's ReverbConvolver.
// The head runs in 128-frame partitions on the render thread with no latency; later, longer
// partitions run on a background thread for realtime contexts, inline for offline ones.
class Convolver {
public:
    static constexpr int kBlock = 128;

    // `channels` holds `count` (1, 2 or 4) channels of `frames` samples; normalize as the spec says.
    Convolver(const float *const *channels, int count, int frames, float sampleRate, bool normalize,
              bool background);

    ~Convolver();

    Convolver(const Convolver &) = delete;
    Convolver &operator=(const Convolver &) = delete;

    // Output channels for `inputs` input channels.
    int outputChannels(int inputs) const { return count_ == 1 && inputs == 1 ? 1 : 2; }

    // Quanta it rings after its input stops.
    int tail() const { return tail_; }

    // Stage layout, for specs and logs.
    int stages() const;

    // One quantum: `in` has `inputs` (1 or 2) channels, null for silence; `out` outputChannels(inputs).
    void process(const float *const *in, int inputs, float *const *out);

private:
    struct Stage;
    struct Core;

    int count_;
    int tail_;
    std::shared_ptr<Core> core_;
    std::thread worker_;
};

#endif // AUDIO_CONVOLVER_H
