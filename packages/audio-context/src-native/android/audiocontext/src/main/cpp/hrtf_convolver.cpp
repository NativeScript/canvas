#include "hrtf_convolver.h"

#include <cstring>
#include <cstdlib>
#include <cmath>
#include <algorithm>

HRTFConvolver::HRTFConvolver()
    : hrirLen_(0), sampleRate_(0), ringPos_(0), partitionSize_(256), fftSize_(0), numPartitions_(0),
      fwdCfg_(nullptr), invCfg_(nullptr), xringPos_(0), xringFilled_(0), inputPos_(0),
      outputPos_(0), outputAvail_(0) {
}

HRTFConvolver::~HRTFConvolver() {
    if (fwdCfg_) kiss_fft_free(fwdCfg_);
    if (invCfg_) kiss_fft_free(invCfg_);
}

static int next_power_of_two(int v) {
    int n = 1;
    while (n < v) n <<= 1;
    return n;
}

bool HRTFConvolver::init(const float *hrirL, const float *hrirR, size_t hrirLen, int sampleRate) {
    if (!hrirL || !hrirR || hrirLen == 0) return false;
    hrirLen_ = hrirLen;
    sampleRate_ = sampleRate;
    hrirL_.assign(hrirL, hrirL + hrirLen_);
    hrirR_.assign(hrirR, hrirR + hrirLen_);

    ring_.assign(hrirLen_, 0.0f);
    ringPos_ = 0;

    if (partitionSize_ <= 0) partitionSize_ = 256;
    if (partitionSize_ > static_cast<int>(hrirLen_)) partitionSize_ = static_cast<int>(hrirLen_);

    fftSize_ = next_power_of_two(partitionSize_ * 2);
    numPartitions_ = static_cast<int>((hrirLen_ + partitionSize_ - 1) / partitionSize_);

    if (fwdCfg_) kiss_fft_free(fwdCfg_);
    if (invCfg_) kiss_fft_free(invCfg_);
    fwdCfg_ = kiss_fft_alloc(fftSize_, 0, nullptr, nullptr);
    invCfg_ = kiss_fft_alloc(fftSize_, 1, nullptr, nullptr);
    if (!fwdCfg_ || !invCfg_) {
        if (fwdCfg_) kiss_fft_free(fwdCfg_);
        if (invCfg_) kiss_fft_free(invCfg_);
        fwdCfg_ = invCfg_ = nullptr;
        return false;
    }

    H_L_.assign(numPartitions_, std::vector<kiss_fft_cpx>(fftSize_));
    H_R_.assign(numPartitions_, std::vector<kiss_fft_cpx>(fftSize_));

    tmpIn_.assign(fftSize_, kiss_fft_cpx{0.0f, 0.0f});
    tmpOut_.assign(fftSize_, kiss_fft_cpx{0.0f, 0.0f});

    for (int p = 0; p < numPartitions_; ++p) {
        std::fill(tmpIn_.begin(), tmpIn_.end(), kiss_fft_cpx{0.0f, 0.0f});
        int base = p * partitionSize_;
        int copyLen = std::min(static_cast<int>(partitionSize_), static_cast<int>(hrirLen_ - base));
        for (int i = 0; i < copyLen; ++i) tmpIn_[i].r = hrirL_[base + i];
        kiss_fft(fwdCfg_, tmpIn_.data(), tmpOut_.data());
        for (int n = 0; n < fftSize_; ++n) H_L_[p][n] = tmpOut_[n];

        std::fill(tmpIn_.begin(), tmpIn_.end(), kiss_fft_cpx{0.0f, 0.0f});
        for (int i = 0; i < copyLen; ++i) tmpIn_[i].r = hrirR_[base + i];
        kiss_fft(fwdCfg_, tmpIn_.data(), tmpOut_.data());
        for (int n = 0; n < fftSize_; ++n) H_R_[p][n] = tmpOut_[n];
    }

    Xring_.assign(numPartitions_, std::vector<kiss_fft_cpx>(fftSize_));
    for (int p = 0; p < numPartitions_; ++p) {
        for (int n = 0; n < fftSize_; ++n) { Xring_[p][n].r = Xring_[p][n].i = 0.0f; }
    }
    xringPos_ = 0;
    xringFilled_ = 0;

    inputBlock_.assign(partitionSize_, 0.0f);
    inputPos_ = 0;

    overlapL_.assign(partitionSize_, 0.0f);
    overlapR_.assign(partitionSize_, 0.0f);
    outputBlockL_.assign(partitionSize_, 0.0f);
    outputBlockR_.assign(partitionSize_, 0.0f);
    outputPos_ = 0;
    outputAvail_ = 0;

    accFreqL_.assign(fftSize_, kiss_fft_cpx{0.0f, 0.0f});
    accFreqR_.assign(fftSize_, kiss_fft_cpx{0.0f, 0.0f});

    return true;
}

void HRTFConvolver::reset() {
    std::fill(ring_.begin(), ring_.end(), 0.0f);
    ringPos_ = 0;
    std::fill(inputBlock_.begin(), inputBlock_.end(), 0.0f);
    inputPos_ = 0;
    std::fill(overlapL_.begin(), overlapL_.end(), 0.0f);
    std::fill(overlapR_.begin(), overlapR_.end(), 0.0f);
    outputPos_ = 0;
    outputAvail_ = 0;
    xringPos_ = 0;
    xringFilled_ = 0;
    for (int p = 0; p < numPartitions_; ++p) {
        for (int n = 0; n < fftSize_; ++n) Xring_[p][n].r = Xring_[p][n].i = 0.0f;
    }
    for (int n = 0; n < fftSize_; ++n) {
        accFreqL_[n].r = accFreqL_[n].i = 0.0f;
        accFreqR_[n].r = accFreqR_[n].i = 0.0f;
    }
}

bool HRTFConvolver::setPartitionSize(int partitionSize) {
    if (partitionSize <= 0) return false;
    partitionSize_ = partitionSize;
    if (hrirLen_ > 0) {
        return init(hrirL_.data(), hrirR_.data(), hrirLen_, sampleRate_);
    }
    return true;
}

void HRTFConvolver::process(const float *in, int frames, float *outL, float *outR) {
    if (!in || frames <= 0 || !outL || !outR) return;
    if (hrirLen_ == 0) {
        for (int i = 0; i < frames; ++i) {
            float s = in[i];
            outL[i] = s;
            outR[i] = s;
        }
        return;
    }

    for (int fi = 0; fi < frames; ++fi) {
        float sample = in[fi];

        ring_[ringPos_] = sample;

        if (inputPos_ < partitionSize_) inputBlock_[inputPos_++] = sample;

        float out_l = 0.0f;
        float out_r = 0.0f;

        if (outputAvail_ > 0) {
            out_l = outputBlockL_[outputPos_];
            out_r = outputBlockR_[outputPos_];
            outputPos_ = (outputPos_ + 1) % partitionSize_;
            --outputAvail_;
        } else {
            if (inputPos_ >= partitionSize_) {
                tmpIn_.assign(fftSize_, kiss_fft_cpx{0.0f, 0.0f});
                for (int n = 0; n < partitionSize_; ++n) tmpIn_[n].r = inputBlock_[n];

                std::vector<kiss_fft_cpx> Xfft(fftSize_);
                kiss_fft(fwdCfg_, tmpIn_.data(), Xfft.data());

                for (int n = 0; n < fftSize_; ++n) Xring_[xringPos_][n] = Xfft[n];

                int availableBlocks = xringFilled_ + 1;
                std::fill(accFreqL_.begin(), accFreqL_.end(), kiss_fft_cpx{0.0f, 0.0f});
                std::fill(accFreqR_.begin(), accFreqR_.end(), kiss_fft_cpx{0.0f, 0.0f});

                for (int p = 0; p < numPartitions_; ++p) {
                    if (p >= availableBlocks) break;
                    int idx = xringPos_ - p;
                    if (idx < 0) idx += numPartitions_;
              
                    for (int n = 0; n < fftSize_; ++n) {
                        const kiss_fft_cpx &HcL = H_L_[p][n];
                        const kiss_fft_cpx &HcR = H_R_[p][n];
                        const kiss_fft_cpx &Xc = Xring_[idx][n];
                    
                        float ar = HcL.r, ai = HcL.i, br = Xc.r, bi = Xc.i;
                        accFreqL_[n].r += ar * br - ai * bi;
                        accFreqL_[n].i += ar * bi + ai * br;
                        
                        ar = HcR.r; ai = HcR.i;
                        accFreqR_[n].r += ar * br - ai * bi;
                        accFreqR_[n].i += ar * bi + ai * br;
                    }
                }

                
                std::vector<kiss_fft_cpx> yL(fftSize_), yR(fftSize_);
                kiss_fft(invCfg_, accFreqL_.data(), yL.data());
                kiss_fft(invCfg_, accFreqR_.data(), yR.data());
                const float scale = 1.0f / static_cast<float>(fftSize_);

       
                for (int n = 0; n < partitionSize_; ++n) {
                    float yl0 = yL[n].r * scale;
                    float yl1 = yL[n + partitionSize_].r * scale;
                    float yr0 = yR[n].r * scale;
                    float yr1 = yR[n + partitionSize_].r * scale;
                    outputBlockL_[n] = yl0 + overlapL_[n];
                    overlapL_[n] = yl1;
                    outputBlockR_[n] = yr0 + overlapR_[n];
                    overlapR_[n] = yr1;
                }

                
                outputPos_ = 0;
                outputAvail_ = partitionSize_;

                
                xringPos_ = (xringPos_ + 1) % numPartitions_;
                if (xringFilled_ < numPartitions_) ++xringFilled_;

               
                inputPos_ = 0;

                
                out_l = outputBlockL_[outputPos_];
                out_r = outputBlockR_[outputPos_];
                outputPos_ = (outputPos_ + 1) % partitionSize_;
                --outputAvail_;
            } else {
                
                double accL = 0.0;
                double accR = 0.0;
                size_t rp = ringPos_;
                for (size_t k = 0; k < hrirLen_; ++k) {
                    float x = ring_[rp];
                    accL += static_cast<double>(x) * static_cast<double>(hrirL_[k]);
                    accR += static_cast<double>(x) * static_cast<double>(hrirR_[k]);
                    if (rp == 0) rp = hrirLen_ - 1;
                    else --rp;
                }
                out_l = static_cast<float>(accL);
                out_r = static_cast<float>(accR);
            }
        }

        outL[fi] = out_l;
        outR[fi] = out_r;

     
        ringPos_ = (ringPos_ + 1) % ring_.size();
    }
}
