#ifndef AUDIO_CONVOLVER_H
#define AUDIO_CONVOLVER_H

#include <vector>

// ConvolverNode: uniformly partitioned FFT convolution in quanta, so it adds no latency.
class Convolver {
public:
    static constexpr int kBlock = 128;

    // `channels` holds `count` (1, 2 or 4) channels of `frames` samples; normalize as the spec says.
    Convolver(const float *const *channels, int count, int frames, float sampleRate, bool normalize);

    // Output channels for `inputs` input channels.
    int outputChannels(int inputs) const { return count_ == 1 && inputs == 1 ? 1 : 2; }

    // Quanta it rings after its input stops.
    int tail() const { return partitions_ + 1; }

    // One quantum: `in` has `inputs` (1 or 2) channels, null for silence; `out` outputChannels(inputs).
    void process(const float *const *in, int inputs, float *const *out);

private:
    static constexpr int kSize = 2 * kBlock;
    static constexpr int kBins = kBlock + 1;

    void convolve(int input, int response, float *out, bool add);

    int count_;
    int partitions_;
    // Per response channel: partitions_ spectra of kBins, real then imaginary.
    std::vector<std::vector<float>> responseRe_, responseIm_;
    // Per input channel: the last block and a ring of partitions_ input spectra.
    std::vector<float> previous_[2];
    std::vector<float> historyRe_[2], historyIm_[2];
    int head_ = 0;
    std::vector<float> scratchRe_, scratchIm_, accRe_, accIm_;
};

#endif // AUDIO_CONVOLVER_H
