#include "audio_context.h"
#include "audio_internal.h"

#include <android/log.h>
#include <jni.h>
#include <string>
#include <vector>
#include <cstring>
#include <cmath>
#include <algorithm>
#include <atomic>

#ifdef HAS_OBOE
#include <oboe/Oboe.h>
#endif

std::string
NativeEngine::renderOfflineForTracks(const std::vector<std::string> &trackIds, int frames,
                                     int sampleRate, int channels) {
    int ch = channels > 0 ? channels : 1;
    double sr = sampleRate > 0 ? sampleRate : 48000.0;
    std::vector<float> out((size_t) frames * ch, 0.0f);

#ifdef HAS_OBOE
    {
        const int previousSampleRate = streamSampleRate_;
        const int previousChannels = streamChannels_;
        streamSampleRate_ = static_cast<int>(sr);
        streamChannels_ = ch;
        float scratch = 0.0f;
        onAudioReady(nullptr, &scratch, 0);
        streamSampleRate_ = previousSampleRate;
        streamChannels_ = previousChannels;
    }
#endif

    std::unordered_map<std::string, BufferData> localBuffers;
    std::unordered_map<std::string, Voice> localVoices;
    std::unordered_map<std::string, double> localGains;
    std::unordered_map<std::string, BiquadCoeffs> localBiquads;
    std::unordered_map<std::string, NativeEngine::IIRData> localIirs;
    std::unordered_map<std::string, Panner> localPanners;
    std::unordered_map<std::string, NativeEngine::PeriodicWaveData> localPeriodicWaves;
    std::unordered_map<std::string, NativeEngine::WaveShaperData> localWaveShapers;
    std::unordered_map<std::string, std::unordered_map<int, std::string>> localVoiceGainByOutput;
    std::unordered_map<std::string, std::unordered_map<int, std::string>> localVoiceFilterByOutput;
    std::unordered_map<std::string, std::unordered_map<int, std::string>> localVoicePannerByOutput;

    {
        std::lock_guard<std::recursive_mutex> lock(mutex_);
        localGains = gains_;
        localBiquads = biquads_;
        localIirs = iirs_;
        localPeriodicWaves = periodicWaves_;
        localWaveShapers = waveShapers_;
        localPanners = panners_;
        localVoiceGainByOutput = voiceGainByOutput_;
        localVoiceFilterByOutput = voiceFilterByOutput_;
        localVoicePannerByOutput = voicePannerByOutput_;

        for (const auto &tid: trackIds) {
            auto it = audioVoices_.find(tid);
            if (it == audioVoices_.end()) continue;
            localVoices[tid] = it->second;
            if (it->second.type == Voice::BufferSource) {
                auto bit = audioBuffers_.find(it->second.bufferId);
                if (bit != audioBuffers_.end()) {
                    localBuffers[it->second.bufferId] = bit->second;
                }
            }
        }
    }

    {
        std::lock_guard<std::mutex> lock(scheduledEventsMutex_);
        for (const auto &kv: scheduledGainEvents_) {
            auto git = localGains.find(kv.first);
            if (git != localGains.end() && !kv.second.empty()) {
                git->second = kv.second.front().value;
            }
        }

        for (const auto &kv: scheduledPannerEvents_) {
            auto pit = localPanners.find(kv.first);
            if (pit == localPanners.end()) continue;
            auto &p = pit->second;
            for (const auto &paramEntry: kv.second) {
                if (paramEntry.second.empty()) continue;
                const double value = paramEntry.second.front().value;
                switch (paramEntry.first) {
                    case kPannerParamPositionX:
                        p.positionX = value;
                        break;
                    case kPannerParamPositionY:
                        p.positionY = value;
                        break;
                    case kPannerParamPositionZ:
                        p.positionZ = value;
                        break;
                    case kPannerParamOrientationX:
                        p.orientationX = value;
                        break;
                    case kPannerParamOrientationY:
                        p.orientationY = value;
                        break;
                    case kPannerParamOrientationZ:
                        p.orientationZ = value;
                        break;
                    case kPannerParamPan:
                        p.pan = value;
                        break;
                    default:
                        break;
                }
            }
        }
    }

    for (int frame = 0; frame < frames; ++frame) {
        for (auto &kv: localVoices) {
            Voice &v = kv.second;
            if (!v.playing) continue;

            float oscillatorSample = 0.0f;
            const BufferData *bufferData = nullptr;
            int bufChannels = 0;
            int bufFrames = 0;
            int i0 = 0;
            double frac = 0.0;

            if (v.type == Voice::Oscillator) {
                double s = 0.0;
                if (!v.periodicWaveId.empty()) {
                    auto pwIt = localPeriodicWaves.find(v.periodicWaveId);
                    if (pwIt != localPeriodicWaves.end()) {
                        double sr = sampleRate > 0 ? static_cast<double>(sampleRate) : 48000.0;
                        s = computePeriodicWaveSample(pwIt->second, v.phase, v.frequency, sr);
                    } else {
                        if (v.waveform == "sine") {
                            s = std::sin(v.phase * 2.0 * M_PI);
                        } else if (v.waveform == "square") {
                            s = (v.phase < 0.5) ? 1.0 : -1.0;
                        } else if (v.waveform == "sawtooth") {
                            s = 2.0 * v.phase - 1.0;
                        } else if (v.waveform == "triangle") {
                            double p = v.phase;
                            if (p < 0.25) s = 4.0 * p;
                            else if (p < 0.75) s = 2.0 - 4.0 * p;
                            else s = 4.0 * p - 4.0;
                        } else {
                            s = std::sin(v.phase * 2.0 * M_PI);
                        }
                    }
                } else {
                    if (v.waveform == "sine") {
                        s = std::sin(v.phase * 2.0 * M_PI);
                    } else if (v.waveform == "square") {
                        s = (v.phase < 0.5) ? 1.0 : -1.0;
                    } else if (v.waveform == "sawtooth") {
                        s = 2.0 * v.phase - 1.0;
                    } else if (v.waveform == "triangle") {
                        double p = v.phase;
                        if (p < 0.25) s = 4.0 * p;
                        else if (p < 0.75) s = 2.0 - 4.0 * p;
                        else s = 4.0 * p - 4.0;
                    } else {
                        s = std::sin(v.phase * 2.0 * M_PI);
                    }
                }
                oscillatorSample = static_cast<float>(s);
            } else if (v.type == Voice::BufferSource) {
                auto bit = localBuffers.find(v.bufferId);
                if (bit == localBuffers.end()) continue;
                bufferData = &bit->second;
                bufChannels = bufferData->channels > 0 ? bufferData->channels : 1;
                if (bufferData->isDirect) {
                    bufFrames = static_cast<int>(
                            (bufferData->byteLength / bufferData->bytesPerSample) /
                            std::max(1, bufChannels));
                } else {
                    bufFrames = static_cast<int>(bufferData->pcm.size() / std::max(1, bufChannels));
                }
                if (bufFrames <= 0) continue;
                i0 = static_cast<int>(std::floor(v.position));
                frac = v.position - i0;
            }

            for (int c = 0; c < ch; ++c) {
                float sample = oscillatorSample;
                int sampleChannelIndex = c;

                if (v.type == Voice::BufferSource) {
                    if (bufChannels == 1) sampleChannelIndex = 0;
                    else if (bufChannels >= ch) sampleChannelIndex = c;
                    else sampleChannelIndex = c % bufChannels;

                    const int idx0 = (i0 % bufFrames) * bufChannels + sampleChannelIndex;
                    const int idx1 = ((i0 + 1) % bufFrames) * bufChannels + sampleChannelIndex;

                    float fs0 = 0.0f;
                    float fs1 = 0.0f;
                    if (bufferData->isDirect && bufferData->directPtr) {
                        if (bufferData->bytesPerSample == 4) {
                            const auto *fptr = reinterpret_cast<const float *>(bufferData->directPtr);
                            const int maxIndex = static_cast<int>((bufferData->byteLength / 4) - 1);
                            const int safe0 = std::max(0, std::min(maxIndex, idx0));
                            const int safe1 = std::max(0, std::min(maxIndex, idx1));
                            fs0 = fptr[safe0];
                            fs1 = fptr[safe1];
                        } else {
                            const auto *iptr = reinterpret_cast<const int16_t *>(bufferData->directPtr);
                            const int maxIndex = static_cast<int>((bufferData->byteLength / 2) - 1);
                            const int safe0 = std::max(0, std::min(maxIndex, idx0));
                            const int safe1 = std::max(0, std::min(maxIndex, idx1));
                            fs0 = static_cast<float>(iptr[safe0]) / 32768.0f;
                            fs1 = static_cast<float>(iptr[safe1]) / 32768.0f;
                        }
                    } else {
                        const int maxIndex = static_cast<int>(bufferData->pcm.size()) - 1;
                        const int safe0 = std::max(0, std::min(maxIndex, idx0));
                        const int safe1 = std::max(0, std::min(maxIndex, idx1));
                        fs0 = static_cast<float>(bufferData->pcm[safe0]) / 32768.0f;
                        fs1 = static_cast<float>(bufferData->pcm[safe1]) / 32768.0f;
                    }

                    sample = fs0 * (1.0f - static_cast<float>(frac)) +
                             fs1 * static_cast<float>(frac);
                }

                std::string effectFilterId = v.filterId;
                auto vfIt = localVoiceFilterByOutput.find(v.id);
                if (vfIt != localVoiceFilterByOutput.end()) {
                    auto filterIt = vfIt->second.find(c);
                    if (filterIt != vfIt->second.end()) effectFilterId = filterIt->second;
                }
                if (!effectFilterId.empty()) {
                    auto bcit = localBiquads.find(effectFilterId);
                    if (bcit != localBiquads.end()) {
                        const int filterStateIndex =
                                v.type == Voice::BufferSource ? sampleChannelIndex : c;
                        if (v.filterState.size() <= static_cast<size_t>(filterStateIndex)) {
                            v.filterState.assign(std::max(ch, bufChannels > 0 ? bufChannels : ch),
                                                 NativeEngine::BiquadState());
                        }
                        sample = processBiquadSample(bcit->second, v.filterState[filterStateIndex],
                                                     sample);
                    } else {
                        auto iit = localIirs.find(effectFilterId);
                        if (iit != localIirs.end()) {
                            const int filterStateIndex =
                                    v.type == Voice::BufferSource ? sampleChannelIndex : c;
                            auto chCount = static_cast<size_t>(std::max(ch, bufChannels > 0
                                                                            ? bufChannels : ch));
                            if (v.iirState.size() <= chCount)
                                v.iirState.assign(chCount, std::vector<double>());
                            if (v.iirState.size() <= static_cast<size_t>(filterStateIndex))
                                v.iirState.assign(chCount, std::vector<double>());
                            sample = processIIRSample(iit->second,
                                                      v.iirState[static_cast<size_t>(filterStateIndex)],
                                                      sample);
                        }
                    }
                }

                double leftGain = 1.0;
                double rightGain = 1.0;
                double distanceAtt = 1.0;
                std::string effectPanId = v.panId;
                auto vpIt = localVoicePannerByOutput.find(v.id);
                if (vpIt != localVoicePannerByOutput.end()) {
                    auto panIt = vpIt->second.find(c);
                    if (panIt != vpIt->second.end()) effectPanId = panIt->second;
                }
                if (!effectPanId.empty()) {
                    auto pit = localPanners.find(effectPanId);
                    if (pit != localPanners.end()) {
                        computePannerGains(pit->second, leftGain, rightGain, distanceAtt);
                    }
                }

                std::string effectGainId;
                auto vgIt = localVoiceGainByOutput.find(v.id);
                if (vgIt != localVoiceGainByOutput.end()) {
                    auto gainIt = vgIt->second.find(c);
                    if (gainIt != vgIt->second.end()) effectGainId = gainIt->second;
                }
                if (effectGainId.empty()) effectGainId = v.gainId;

                double gainValue = v.gain;
                if (!effectGainId.empty()) {
                    auto git = localGains.find(effectGainId);
                    if (git != localGains.end()) gainValue = git->second;
                }

                if (!effectGainId.empty()) {
                    auto wsIt = localWaveShapers.find(effectGainId);
                    if (wsIt != localWaveShapers.end() && !wsIt->second.curve.empty()) {
                        sample = static_cast<double>(applyWaveShaperSample(wsIt->second,
                                                                           static_cast<float>(sample)));
                    }
                }

                if (ch <= 1) {
                    const double monoGain = 0.5 * (leftGain + rightGain) * distanceAtt * gainValue;
                    out[(size_t) frame * ch + c] += static_cast<float>(sample * monoGain);
                } else if (c == 0) {
                    out[(size_t) frame * ch + c] += static_cast<float>(sample * gainValue *
                                                                       distanceAtt * leftGain);
                } else if (c == 1) {
                    out[(size_t) frame * ch + c] += static_cast<float>(sample * gainValue *
                                                                       distanceAtt * rightGain);
                } else {
                    const double avgGain = 0.5 * (leftGain + rightGain);
                    out[(size_t) frame * ch + c] += static_cast<float>(sample * gainValue *
                                                                       distanceAtt * avgGain);
                }
            }

            if (v.type == Voice::Oscillator) {
                v.phase += v.frequency / sr;
                if (v.phase >= 1.0) v.phase -= 1.0;
            } else if (v.type == Voice::BufferSource) {
                v.position += v.increment;
                if (v.position >= bufFrames) {
                    if (v.loop) v.position = std::fmod(v.position, bufFrames);
                    else v.playing = false;
                }
            }
        }
    }

    std::vector<int16_t> pcm((size_t) frames * ch);
    for (size_t i = 0; i < pcm.size(); ++i) {
        float v = out[i];
        if (v > 1.0f) v = 1.0f;
        if (v < -1.0f) v = -1.0f;
        pcm[i] = static_cast<int16_t>(v * 32767.0f);
    }

    std::string id = genId();
    BufferData bd;
    bd.nativeOwned = true;
    bd.pcm = std::move(pcm);
    bd.sampleRate = sampleRate > 0 ? sampleRate : 48000;
    bd.channels = ch;
    {
        std::lock_guard<std::recursive_mutex> lock(mutex_);
        audioBuffers_[id] = std::move(bd);
    }
    __android_log_print(ANDROID_LOG_INFO, TAG,
                        "renderOfflineForTracks: created buffer %s frames=%d sr=%d ch=%d",
                        id.c_str(), frames, sampleRate, ch);
    return id;
}
