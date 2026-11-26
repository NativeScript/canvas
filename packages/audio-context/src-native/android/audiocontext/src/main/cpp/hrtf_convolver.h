#ifndef HRTF_CONVOLVER_H
#define HRTF_CONVOLVER_H

#include <cstdint>
#include <vector>
#include "kiss_fft.h"

class HRTFConvolver {
public:
    HRTFConvolver();
    ~HRTFConvolver();

    bool init(const float *hrirL, const float *hrirR, size_t hrirLen, int sampleRate);

    bool setPartitionSize(int partitionSize);

    void process(const float *in, int frames, float *outL, float *outR);

    void reset();

private:
    size_t hrirLen_;
    int sampleRate_;
    std::vector<float> hrirL_;
    std::vector<float> hrirR_;
    std::vector<float> ring_;
    size_t ringPos_;

    int partitionSize_;
    int fftSize_;
    int numPartitions_;

    void* fwdCfg_ = nullptr;
    void* invCfg_ = nullptr;

    std::vector<std::vector<kiss_fft_cpx>> H_L_;
    std::vector<std::vector<kiss_fft_cpx>> H_R_;

    std::vector<std::vector<kiss_fft_cpx>> Xring_;
    int xringPos_;
    int xringFilled_;

    std::vector<float> inputBlock_;
    int inputPos_;

    std::vector<float> overlapL_;
    std::vector<float> overlapR_;
    std::vector<float> outputBlockL_;
    std::vector<float> outputBlockR_;
    int outputPos_;
    int outputAvail_;

    std::vector<kiss_fft_cpx> tmpIn_;
    std::vector<kiss_fft_cpx> tmpOut_;
    std::vector<kiss_fft_cpx> accFreqL_;
    std::vector<kiss_fft_cpx> accFreqR_;
};

#endif // HRTF_CONVOLVER_H
