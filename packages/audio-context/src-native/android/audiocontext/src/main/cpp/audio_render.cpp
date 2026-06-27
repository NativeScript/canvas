#include "audio_context.h"
#include "audio_internal.h"
#include "hrtf_convolver.h"

#include <android/log.h>
#include <jni.h>
#include <string>
#include <array>
#include <vector>
#include <cstring>
#include <cmath>
#include <algorithm>
#include <functional>
#include <unordered_set>
#include <atomic>
#include <chrono>
#include <time.h>

#include "kiss_fft.h"

#ifdef HAS_OBOE

#include <oboe/Oboe.h>

#endif

#ifdef HAS_OBOE

class EngineCallback : public oboe::AudioStreamCallback {
public:
    explicit EngineCallback(NativeEngine *e) : engine(e) {}

    oboe::DataCallbackResult
    onAudioReady(oboe::AudioStream *stream, void *audioData, int32_t numFrames) override {
        return engine->onAudioReady(stream, audioData, numFrames);
    }

private:
    NativeEngine *engine;
};

static inline int64_t monotonicNowNs() {
    timespec ts{};
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return static_cast<int64_t>(ts.tv_sec) * 1000000000LL + static_cast<int64_t>(ts.tv_nsec);
}

void NativeEngine::ensureStream() {
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    if (stream_) return;

    oboe::AudioStreamBuilder builder;
    oboe::PerformanceMode perf = oboe::PerformanceMode::LowLatency;
    if (desiredLatencyHintSec_ > 0.020) perf = oboe::PerformanceMode::PowerSaving;
    else if (desiredLatencyHintSec_ > 0.010) perf = oboe::PerformanceMode::None;
    builder.setPerformanceMode(perf);
    // Request exclusive mode for low-latency to bypass the audio mixer
    if (perf == oboe::PerformanceMode::LowLatency) {
        builder.setSharingMode(oboe::SharingMode::Exclusive);
    }
    builder.setFormat(oboe::AudioFormat::Float);
    builder.setChannelCount(2);
    if (desiredSampleRate_ > 0) {
        builder.setSampleRate(desiredSampleRate_);
        builder.setSampleRateConversionQuality(oboe::SampleRateConversionQuality::Medium);
    }
    if (desiredLatencyHintSec_ > 0.0) {
        int rate = desiredSampleRate_ > 0 ? desiredSampleRate_ : 48000;
        int desiredFrames = static_cast<int>(desiredLatencyHintSec_ * rate);
        if (desiredFrames > 0) builder.setBufferCapacityInFrames(desiredFrames * 4);
    }
    if (desiredDeviceId_ != static_cast<int>(oboe::kUnspecified)) {
        builder.setDeviceId(desiredDeviceId_);
    }

    callback_ = new EngineCallback(this);
    builder.setCallback(callback_);

    oboe::Result r = builder.openStream(stream_);
    if (r != oboe::Result::OK || !stream_) {
        __android_log_print(ANDROID_LOG_WARN, TAG, "Oboe openStream failed: %d",
                            static_cast<int>(r));
        if (stream_) {
            stream_->close();
            stream_.reset();
        }
        delete callback_;
        callback_ = nullptr;
        return;
    }

    streamSampleRate_ = stream_->getSampleRate();
    streamChannels_ = stream_->getChannelCount();

    int32_t framesPerBurst = stream_->getFramesPerBurst();
    int32_t capacityFrames = stream_->getBufferCapacityInFrames();
    if (framesPerBurst > 0 && capacityFrames > 0) {
        int32_t requestedFrames;
        if (desiredLatencyHintSec_ > 0.0 && streamSampleRate_ > 0) {
            // Use hint as a TARGET: round up to the nearest burst boundary.
            // Previously used std::max(framesPerBurst*4, hintedFrames) which treated the
            // hint as a floor — for 'interactive' (240 frames) this always lost to 768,
            // giving 16ms instead of ~8ms.
            int32_t hintedFrames = static_cast<int32_t>(
                    desiredLatencyHintSec_ * static_cast<double>(streamSampleRate_));
            int32_t bursts = hintedFrames > 0
                             ? std::max(1, (hintedFrames + framesPerBurst - 1) / framesPerBurst)
                             : 1;
            requestedFrames = bursts * framesPerBurst;
        } else {
            // No hint: 2 bursts (~8ms at 192fpb/48kHz) matches the web default
            requestedFrames = framesPerBurst * 2;
        }
        requestedFrames = std::min(requestedFrames, capacityFrames);

        auto bufferResult = stream_->setBufferSizeInFrames(requestedFrames);
        if (bufferResult) {
            __android_log_print(ANDROID_LOG_INFO, TAG,
                                "Oboe buffer size set to %d frames (burst=%d cap=%d hint=%.3fs)",
                                bufferResult.value(), framesPerBurst, capacityFrames,
                                desiredLatencyHintSec_);
        } else {
            __android_log_print(ANDROID_LOG_WARN, TAG,
                                "Oboe setBufferSizeInFrames(%d) failed: %d",
                                requestedFrames, static_cast<int>(bufferResult.error()));
        }
    }

    __android_log_print(ANDROID_LOG_INFO, TAG, "Oboe stream opened (rate=%d channels=%d)",
                        streamSampleRate_, streamChannels_);
    stream_->requestStart();
}

void NativeEngine::stopStream() {
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    if (!stream_) return;

    stream_->requestStop();
    stream_->close();
    stream_.reset();

    if (callback_) {
        delete callback_;
        callback_ = nullptr;
    }
}


oboe::DataCallbackResult
NativeEngine::onAudioReady(oboe::AudioStream *stream, void *audioData, int32_t numFrames) {
    if (!audioData) return oboe::DataCallbackResult::Continue;
    auto *out = reinterpret_cast<float *>(audioData);
    const int channels = streamChannels_ > 0 ? streamChannels_ : 2;
    auto resolveExternalChannelCount = [&](const Voice &voice,
                                           const std::shared_ptr<ExternalRing> &ring) -> int {
        if (!ring || ring->capacity == 0 || ring->data.empty()) return 0;

        int resolved = ring->channels > 0 ? ring->channels : voice.bufferChannels;
        if (resolved <= 0) resolved = 1;

        if (voice.bufferChannels > 0) {
            resolved = std::min(resolved, voice.bufferChannels);
        }

        size_t scratchChannels = std::min(voice.externalPrev.size(), voice.externalCurr.size());
        if (scratchChannels == 0) return 0;
        resolved = std::min(resolved, static_cast<int>(scratchChannels));

        size_t dataChannels = ring->data.size() / static_cast<size_t>(ring->capacity);
        if (dataChannels == 0) return 0;
        resolved = std::min(resolved, static_cast<int>(dataChannels));

        return resolved > 0 ? resolved : 0;
    };

    auto hydrateVoicePannerDefaults = [&](Voice &voice) {
        auto vpIt = voicePannerByOutput_.find(voice.id);
        if (vpIt == voicePannerByOutput_.end()) return;
        auto mapIt = vpIt->second.find(0);
        if (mapIt == vpIt->second.end() || mapIt->second.empty()) return;

        const std::string &mappedPannerId = mapIt->second;
        voice.panId = mappedPannerId;

        auto pit = panners_.find(mappedPannerId);
        if (pit == panners_.end()) return;

        voice.pan = static_cast<float>(pit->second.pan);

        const std::string &pbid = pit->second.biquadId;
        if (!pbid.empty() && voice.filterId.empty()) {
            voice.filterId = pbid;
            int ch = streamChannels_ > 0 ? streamChannels_ : 2;
            auto iit = iirs_.find(pbid);
            if (iit != iirs_.end()) {
                size_t nb = iit->second.feedforward.size();
                size_t na = iit->second.feedback.size();
                size_t stateSize = 0;
                if (std::max(nb, na) >= 1) stateSize = std::max(nb, na) - 1;
                voice.iirState.assign(ch, std::vector<double>(stateSize, 0.0));
            } else {
                voice.filterState.assign(ch, NativeEngine::BiquadState());
            }
        }
    };

    const bool traceEnabled = g_audioThreadLoggingEnabled.load(std::memory_order_relaxed);

    auto countMappedRefs = [&](const std::unordered_map<std::string, std::unordered_map<int, std::string>> &mapByVoice,
                               const std::string &targetId) -> size_t {
        if (!traceEnabled || targetId.empty()) return 0;
        size_t count = 0;
        for (const auto &voiceEntry: mapByVoice) {
            for (const auto &outEntry: voiceEntry.second) {
                if (outEntry.second == targetId) ++count;
            }
        }
        return count;
    };

    auto voiceTypeName = [](Voice::Type type) -> const char * {
        switch (type) {
            case Voice::Oscillator:
                return "osc";
            case Voice::BufferSource:
                return "buffer";
            case Voice::ExternalPCM:
                return "external-pcm";
            default:
                return "unknown";
        }
    };

    auto countVoiceFieldRefs = [&](const std::string &targetId,
                                   const std::function<const std::string &(const Voice &)> &selector) -> size_t {
        if (!traceEnabled || targetId.empty()) return 0;
        size_t count = 0;
        for (const auto &voiceEntry: audioVoices_) {
            if (selector(voiceEntry.second) == targetId) ++count;
        }
        return count;
    };

    auto logRingState = [&](unsigned long long seq,
                            const char *label,
                            const std::string &voiceId,
                            const std::shared_ptr<ExternalRing> &ring,
                            int voiceChannels,
                            int voiceSampleRate,
                            bool playing) {
        if (!traceEnabled) return;
        if (!ring) {
            audioThreadLog(ANDROID_LOG_INFO,
                           "GRAPH_TRACE seq=%llu %s voice=%s ring=null voiceCh=%d sr=%d playing=%d",
                           seq, label, voiceId.c_str(), voiceChannels, voiceSampleRate,
                           playing ? 1 : 0);
            return;
        }
        uint32_t w = ring->writeIdx.load(std::memory_order_acquire);
        uint32_t r = ring->readIdx.load(std::memory_order_acquire);
        uint32_t queued = w >= r ? (w - r) : 0;
        audioThreadLog(ANDROID_LOG_INFO,
                       "GRAPH_TRACE seq=%llu %s voice=%s ring=%p cap=%u mask=%u ringCh=%d voiceCh=%d sr=%d data=%zu w=%u r=%u queued=%u ended=%d playing=%d",
                       seq, label, voiceId.c_str(), static_cast<void *>(ring.get()),
                       ring->capacity, ring->mask, ring->channels, voiceChannels,
                       voiceSampleRate, ring->data.size(), w, r, queued,
                       ring->ended.load(std::memory_order_acquire) ? 1 : 0,
                       playing ? 1 : 0);
    };

    static std::atomic<unsigned long long> sGraphTraceSeq{0};

    {

        if (stream) {
            auto tsRes = stream->getTimestamp(CLOCK_MONOTONIC);
            if (tsRes) {
                auto frameTs = tsRes.value();
                g_audioTimeNanos.store(frameTs.timestamp, std::memory_order_relaxed);
            } else {
                int64_t nowNs = monotonicNowNs();
                g_audioTimeNanos.store(nowNs, std::memory_order_relaxed);
            }
        } else {
            int64_t nowNs = monotonicNowNs();
            g_audioTimeNanos.store(nowNs, std::memory_order_relaxed);
        }
    }


    std::vector<NativeEngine::Command> cmds;
    {
        std::lock_guard<std::mutex> cl(commandMutex_);
        cmds.swap(pendingCommands_);
    }

    if (traceEnabled && !cmds.empty()) {
        size_t cmdCreateExternalPcm = 0;
        size_t cmdConfigureExternalPcm = 0;
        size_t cmdPushExternalPcm = 0;
        size_t cmdEndExternalPcm = 0;
        size_t cmdStopTrack = 0;
        size_t cmdFreeGain = 0;

        for (const auto &cmd: cmds) {
            switch (cmd.type) {
                case NativeEngine::CMD_CREATE_EXTERNAL_PCM:
                    ++cmdCreateExternalPcm;
                    break;
                case NativeEngine::CMD_CONFIGURE_EXTERNAL_PCM:
                    ++cmdConfigureExternalPcm;
                    break;
                case NativeEngine::CMD_PUSH_EXTERNAL_PCM:
                    ++cmdPushExternalPcm;
                    break;
                case NativeEngine::CMD_END_EXTERNAL_PCM:
                    ++cmdEndExternalPcm;
                    break;
                case NativeEngine::CMD_STOP_TRACK:
                    ++cmdStopTrack;
                    break;
                case NativeEngine::CMD_FREE_GAIN:
                    ++cmdFreeGain;
                    break;
                default:
                    break;
            }
        }

        audioThreadLog(ANDROID_LOG_INFO,
                       "GRAPH_TRACE batch commands=%zu activeVoices=%zu gains=%zu panners=%zu biquads=%zu iirs=%zu extCreate=%zu extCfg=%zu extPush=%zu extEnd=%zu stopTrack=%zu freeGain=%zu",
                       cmds.size(), audioVoices_.size(), gains_.size(), panners_.size(),
                       biquads_.size(), iirs_.size(), cmdCreateExternalPcm,
                       cmdConfigureExternalPcm, cmdPushExternalPcm,
                       cmdEndExternalPcm, cmdStopTrack, cmdFreeGain);
    }

    for (auto &c: cmds) {
        const unsigned long long traceSeq =
                sGraphTraceSeq.fetch_add(1, std::memory_order_relaxed) + 1;
        switch (c.type) {
            case NativeEngine::CMD_CREATE_BUFFER_DIRECT: {
                BufferData bd;
                bd.isDirect = true;
                bd.directPtr = c.directPtr;
                bd.byteLength = c.byteLen;
                bd.bytesPerSample = c.bytesPerSample;
                bd.javaBufferId = c.id;
                bd.sampleRate = c.sampleRate > 0 ? c.sampleRate : 48000;
                bd.channels = c.channels > 0 ? c.channels : 1;
                bd.nativeOwned = false;
                audioBuffers_[c.id] = bd;
                audioThreadLog(ANDROID_LOG_INFO,
                               "CMD: create direct buffer %s sr=%d ch=%d", c.id.c_str(),
                               bd.sampleRate, bd.channels);
                if (jvm_ && audioContextClassGlobalRef && onNativeBufferHeldMethod) {
                    JNIEnv *env = nullptr;
                    bool attached = false;
                    if (jvm_->GetEnv(reinterpret_cast<void **>(&env), JNI_VERSION_1_6) != JNI_OK) {
                        if (jvm_->AttachCurrentThread(&env, nullptr) == JNI_OK) attached = true;
                        else env = nullptr;
                    }
                    if (env) {
                        jstring js = env->NewStringUTF(c.id.c_str());
                        env->CallStaticVoidMethod(audioContextClassGlobalRef,
                                                  onNativeBufferHeldMethod, js);
                        env->DeleteLocalRef(js);
                        if (attached) jvm_->DetachCurrentThread();
                    }
                }
                break;
            }
            case NativeEngine::CMD_CREATE_BUFFER_PLANAR: {
                BufferData bd;
                bd.isDirect = true;
                bd.isPlanar = true;
                if (c.planarPtrs) {
                    bd.planarPtrs = *c.planarPtrs;
                }
                bd.planarFrames = c.planarFrames > 0 ? c.planarFrames : 0;
                bd.bytesPerSample = 4;
                bd.sampleRate = c.sampleRate > 0 ? c.sampleRate : 48000;
                bd.channels = c.channels > 0
                              ? c.channels
                              : static_cast<int>(bd.planarPtrs.size());
                if (bd.channels <= 0) bd.channels = 1;
                if (bd.planarFrames > 0) {
                    bd.byteLength = static_cast<size_t>(bd.planarFrames) *
                                    static_cast<size_t>(bd.channels) * 4;
                }
                bd.nativeOwned = false;
                audioBuffers_[c.id] = std::move(bd);
                audioThreadLog(ANDROID_LOG_INFO,
                               "CMD: create planar buffer %s sr=%d ch=%d frames=%d",
                               c.id.c_str(), c.sampleRate, c.channels, c.planarFrames);
                break;
            }
            case NativeEngine::CMD_CREATE_BUFFER_COPY: {
                BufferData bd;
                bd.pcm = std::move(*c.pcm);
                bd.sampleRate = c.sampleRate > 0 ? c.sampleRate : 48000;
                bd.channels = c.channels > 0 ? c.channels : 1;
                bd.nativeOwned = true;
                audioBuffers_[c.id] = std::move(bd);
                audioThreadLog(ANDROID_LOG_INFO, "CMD: create copy buffer %s sr=%d ch=%d",
                               c.id.c_str(), c.sampleRate, c.channels);
                break;
            }
            case NativeEngine::CMD_CREATE_OSC: {
                Voice v;
                v.voiceType = Voice::Oscillator;
                v.id = c.id;
                v.waveform = c.waveform;
                v.frequency = c.frequency;
                v.phase = 0.0;
                v.phaseIncrement = 0.0;
                v.playing = false;

                auto vgInitIt = voiceGainByOutput_.find(v.id);
                if (vgInitIt != voiceGainByOutput_.end() && !vgInitIt->second.empty()) {
                    auto mapIt = vgInitIt->second.find(0);
                    if (mapIt == vgInitIt->second.end()) mapIt = vgInitIt->second.begin();
                    v.gainId = mapIt->second;
                    auto git = gains_.find(v.gainId);
                    if (git != gains_.end()) v.gain = git->second;
                }
                audioVoices_[c.id] = v;
                break;
            }
            case NativeEngine::CMD_CREATE_GAIN: {
                gains_[c.id] = c.gainValue > 0.0 ? c.gainValue : 1.0;
                audioThreadLog(ANDROID_LOG_INFO, "CMD: create gain %s value=%f",
                               c.id.c_str(), gains_[c.id]);
                break;
            }
            case NativeEngine::CMD_CREATE_DYNAMICS_COMPRESSOR: {
                NativeEngine::DynamicsCompressor compressor;
                compressor.thresholdId = c.compressorThresholdId;
                compressor.kneeId = c.compressorKneeId;
                compressor.ratioId = c.compressorRatioId;
                compressor.attackId = c.compressorAttackId;
                compressor.releaseId = c.compressorReleaseId;
                compressor.reductionId = c.compressorReductionId;
                compressor.envelopeByChannel.assign(static_cast<size_t>(std::max(1, channels)),
                                                    0.0f);
                compressor.reductionDbByChannel.assign(static_cast<size_t>(std::max(1, channels)),
                                                       0.0f);
                compressors_[c.id] = std::move(compressor);

                auto ensureGainDefault = [&](const std::string &gid, double value) {
                    if (gid.empty()) return;
                    if (gains_.find(gid) == gains_.end()) gains_[gid] = value;
                };
                ensureGainDefault(c.compressorThresholdId, -24.0);
                ensureGainDefault(c.compressorKneeId, 30.0);
                ensureGainDefault(c.compressorRatioId, 12.0);
                ensureGainDefault(c.compressorAttackId, 0.003);
                ensureGainDefault(c.compressorReleaseId, 0.25);
                ensureGainDefault(c.compressorReductionId, 0.0);

                audioThreadLog(ANDROID_LOG_INFO,
                               "CMD: create dynamics-compressor %s threshold=%s knee=%s ratio=%s attack=%s release=%s reduction=%s",
                               c.id.c_str(), c.compressorThresholdId.c_str(),
                               c.compressorKneeId.c_str(), c.compressorRatioId.c_str(),
                               c.compressorAttackId.c_str(),
                               c.compressorReleaseId.c_str(),
                               c.compressorReductionId.c_str());
                break;
            }
            case NativeEngine::CMD_SET_GAIN: {
                double v = c.gainValue;
                gains_[c.id] = v;
                for (auto &kv: audioVoices_) {
                    if (kv.second.gainId == c.id) {
                        kv.second.gain = v;
                    }
                }
                audioThreadLog(ANDROID_LOG_INFO, "CMD: set gain %s value=%f",
                               c.id.c_str(), v);

                for (const auto &av: audioVoices_) {
                    const std::string &voiceId = av.first;
                    const Voice &voice = av.second;
                    if (voice.gainId == c.id) {
                        audioThreadLog(ANDROID_LOG_INFO,
                                       "VOICE_GAIN_FIELD voice=%s gainId=%s voiceGain=%f",
                                       voiceId.c_str(), voice.gainId.c_str(), voice.gain);
                    }
                    auto vgIt = voiceGainByOutput_.find(voiceId);
                    if (vgIt != voiceGainByOutput_.end()) {
                        for (const auto &entry: vgIt->second) {
                            if (entry.second == c.id) {
                                audioThreadLog(ANDROID_LOG_INFO,
                                               "VOICE_GAIN_MAP voice=%s out=%d mapsToGain=%s currentVoiceGain=%f",
                                               voiceId.c_str(), entry.first, entry.second.c_str(),
                                               voice.gain);
                            }
                        }
                    }
                }
                break;
            }
            case NativeEngine::CMD_ATTACH_GAIN: {
                auto vit = audioVoices_.find(c.id);
                std::string prevGainId;
                bool voiceExists = vit != audioVoices_.end();
                if (vit != audioVoices_.end()) {
                    prevGainId = vit->second.gainId;
                }

                if (!c.gainId.empty()) {
                    voiceGainByOutput_[c.id][c.outputIndex] = c.gainId;
                } else {
                    auto vgIt = voiceGainByOutput_.find(c.id);
                    if (vgIt != voiceGainByOutput_.end()) {
                        vgIt->second.erase(c.outputIndex);
                        if (vgIt->second.empty()) voiceGainByOutput_.erase(vgIt);
                    }
                }

                if (vit != audioVoices_.end()) {
                    std::string resolvedGainId;
                    auto vgResolved = voiceGainByOutput_.find(c.id);
                    if (vgResolved != voiceGainByOutput_.end() && !vgResolved->second.empty()) {
                        auto mapIt = vgResolved->second.find(0);
                        if (mapIt == vgResolved->second.end()) mapIt = vgResolved->second.begin();
                        if (mapIt != vgResolved->second.end()) {
                            resolvedGainId = mapIt->second;
                        }
                    }

                    vit->second.gainId = resolvedGainId;
                    auto git = gains_.find(resolvedGainId);
                    if (!resolvedGainId.empty() && git != gains_.end()) {
                        vit->second.gain = git->second;
                    } else {
                        vit->second.gain = 1.0;
                    }
                }

                audioThreadLog(ANDROID_LOG_INFO,
                               "CMD: attach gain %s -> voice %s (out=%d in=%d)",
                               c.gainId.c_str(), c.id.c_str(), c.outputIndex, c.inputIndex);
                if (traceEnabled) {
                    size_t mappedRefs = countMappedRefs(voiceGainByOutput_, c.gainId);
                    size_t fieldRefs = countVoiceFieldRefs(c.gainId,
                                                           [](const Voice &voice) -> const std::string & {
                                                               return voice.gainId;
                                                           });
                    size_t outputsForVoice = 0;
                    auto vgDbg = voiceGainByOutput_.find(c.id);
                    if (vgDbg != voiceGainByOutput_.end()) outputsForVoice = vgDbg->second.size();
                    audioThreadLog(ANDROID_LOG_INFO,
                                   "GRAPH_TRACE seq=%llu ATTACH_GAIN voice=%s exists=%d prev=%s new=%s out=%d in=%d mappedRefs=%zu fieldRefs=%zu outputsForVoice=%zu",
                                   traceSeq, c.id.c_str(), voiceExists ? 1 : 0,
                                   prevGainId.c_str(), c.gainId.c_str(), c.outputIndex,
                                   c.inputIndex, mappedRefs, fieldRefs, outputsForVoice);
                }
                break;
            }
            case NativeEngine::CMD_ATTACH_PLAYBACK_RATE: {
                auto vit = audioVoices_.find(c.id);
                if (vit != audioVoices_.end()) {
                    vit->second.playbackRateId = c.playbackRateId;
                }

                if (!c.playbackRateId.empty()) {
                    voicePlaybackRateByOutput_[c.id][c.outputIndex] = c.playbackRateId;
                } else {
                    auto vprIt = voicePlaybackRateByOutput_.find(c.id);
                    if (vprIt != voicePlaybackRateByOutput_.end()) {
                        vprIt->second.erase(c.outputIndex);
                        if (vprIt->second.empty()) voicePlaybackRateByOutput_.erase(vprIt);
                    }
                }

                audioThreadLog(ANDROID_LOG_INFO,
                               "CMD: attach playbackRate %s -> voice %s (out=%d in=%d)",
                               c.playbackRateId.c_str(), c.id.c_str(), c.outputIndex, c.inputIndex);
                break;
            }
            case NativeEngine::CMD_DETACH_PLAYBACK_RATE: {
                for (auto &kv: audioVoices_) {
                    if (kv.second.playbackRateId == c.id) {
                        kv.second.playbackRateId.clear();
                    }
                }
                audioThreadLog(ANDROID_LOG_INFO, "CMD: detach playbackRate %s", c.id.c_str());
                break;
            }
            case NativeEngine::CMD_FREE_PLAYBACK_RATE: {
                for (auto &kv: audioVoices_) {
                    if (kv.second.playbackRateId == c.id) {
                        kv.second.playbackRateId.clear();
                    }
                }
                audioThreadLog(ANDROID_LOG_INFO, "CMD: free playbackRate %s", c.id.c_str());
                break;
            }
            case NativeEngine::CMD_DETACH_GAIN: {
                size_t fieldRefsBefore = countVoiceFieldRefs(c.id,
                                                             [](const Voice &voice) -> const std::string & {
                                                                 return voice.gainId;
                                                             });
                size_t mappedRefsBefore = countMappedRefs(voiceGainByOutput_, c.id);
                for (auto &kv: audioVoices_) {
                    if (kv.second.gainId == c.id) {
                        kv.second.gainId.clear();
                        kv.second.gain = 1.0;
                    }
                }
                audioThreadLog(ANDROID_LOG_INFO, "CMD: detach gain %s", c.id.c_str());
                if (traceEnabled) {
                    size_t fieldRefsAfter = countVoiceFieldRefs(c.id,
                                                                [](const Voice &voice) -> const std::string & {
                                                                    return voice.gainId;
                                                                });
                    size_t mappedRefsAfter = countMappedRefs(voiceGainByOutput_, c.id);
                    audioThreadLog(ANDROID_LOG_INFO,
                                   "GRAPH_TRACE seq=%llu DETACH_GAIN gain=%s fieldRefsBefore=%zu fieldRefsAfter=%zu mappedRefsBefore=%zu mappedRefsAfter=%zu",
                                   traceSeq, c.id.c_str(), fieldRefsBefore, fieldRefsAfter,
                                   mappedRefsBefore, mappedRefsAfter);
                }
                break;
            }
            case NativeEngine::CMD_FREE_GAIN: {
                size_t fieldRefsBefore = countVoiceFieldRefs(c.id,
                                                             [](const Voice &voice) -> const std::string & {
                                                                 return voice.gainId;
                                                             });
                size_t mappedRefsBefore = countMappedRefs(voiceGainByOutput_, c.id);
                bool hadGain = gains_.find(c.id) != gains_.end();
                bool hadWaveShaper = waveShapers_.find(c.id) != waveShapers_.end();
                auto compressorIt = compressors_.find(c.id);
                bool hadCompressor = compressorIt != compressors_.end();
                if (traceEnabled && !hadGain) {
                    audioThreadLog(ANDROID_LOG_WARN,
                                   "GRAPH_TRACE seq=%llu FREE_GAIN_DUPLICATE gain=%s mappedRefsBefore=%zu fieldRefsBefore=%zu",
                                   traceSeq, c.id.c_str(), mappedRefsBefore,
                                   fieldRefsBefore);
                }
                gains_.erase(c.id);
                for (auto &kv: audioVoices_) {
                    if (kv.second.gainId == c.id) {
                        kv.second.gainId.clear();
                        kv.second.gain = 1.0;
                    }
                }

                waveShapers_.erase(c.id);
                if (hadCompressor) {
                    auto eraseCompressorParam = [&](const std::string &paramId) {
                        if (paramId.empty()) return;
                        gains_.erase(paramId);
                    };
                    eraseCompressorParam(compressorIt->second.thresholdId);
                    eraseCompressorParam(compressorIt->second.kneeId);
                    eraseCompressorParam(compressorIt->second.ratioId);
                    eraseCompressorParam(compressorIt->second.attackId);
                    eraseCompressorParam(compressorIt->second.releaseId);
                    eraseCompressorParam(compressorIt->second.reductionId);
                    compressors_.erase(compressorIt);
                }
                audioThreadLog(ANDROID_LOG_INFO, "CMD: free gain %s", c.id.c_str());
                if (traceEnabled) {
                    size_t fieldRefsAfter = countVoiceFieldRefs(c.id,
                                                                [](const Voice &voice) -> const std::string & {
                                                                    return voice.gainId;
                                                                });
                    size_t mappedRefsAfter = countMappedRefs(voiceGainByOutput_, c.id);
                    audioThreadLog(ANDROID_LOG_INFO,
                                   "GRAPH_TRACE seq=%llu FREE_GAIN gain=%s hadGain=%d hadWaveShaper=%d hadCompressor=%d fieldRefsBefore=%zu fieldRefsAfter=%zu mappedRefsBefore=%zu mappedRefsAfter=%zu",
                                   traceSeq, c.id.c_str(), hadGain ? 1 : 0,
                                   hadWaveShaper ? 1 : 0, hadCompressor ? 1 : 0,
                                   fieldRefsBefore, fieldRefsAfter, mappedRefsBefore,
                                   mappedRefsAfter);
                }
                break;
            }
            case NativeEngine::CMD_CREATE_BIQUAD: {
                NativeEngine::BiquadCoeffs bc = computeBiquadCoeffs(
                        !c.waveform.empty() ? c.waveform : c.biquadType,
                        c.biquadFrequency > 0.0 ? c.biquadFrequency : 440.0,
                        c.biquadQ > 0.0 ? c.biquadQ : 1.0, c.biquadGain,
                        streamSampleRate_ > 0 ? streamSampleRate_ : 48000);
                biquads_[c.id] = bc;
                audioThreadLog(ANDROID_LOG_INFO,
                               "CMD: create biquad %s type=%s f=%f Q=%f g=%f", c.id.c_str(),
                               c.biquadType.c_str(), c.biquadFrequency, c.biquadQ,
                               c.biquadGain);
                break;
            }
            case NativeEngine::CMD_SET_BIQUAD_PARAMS: {
                auto it = biquads_.find(c.id);
                NativeEngine::BiquadCoeffs bc = computeBiquadCoeffs(
                        !c.biquadType.empty() ? c.biquadType : c.waveform,
                        c.biquadFrequency > 0.0 ? c.biquadFrequency : 440.0,
                        c.biquadQ > 0.0 ? c.biquadQ : 1.0, c.biquadGain,
                        streamSampleRate_ > 0 ? streamSampleRate_ : 48000);
                biquads_[c.id] = bc;
                audioThreadLog(ANDROID_LOG_INFO, "CMD: set biquad %s f=%f Q=%f g=%f",
                               c.id.c_str(), c.biquadFrequency, c.biquadQ, c.biquadGain);
                break;
            }
            case NativeEngine::CMD_ATTACH_BIQUAD: {
                auto vit = audioVoices_.find(c.id);
                std::string prevFilterId;
                bool voiceExists = vit != audioVoices_.end();
                if (vit != audioVoices_.end()) {
                    prevFilterId = vit->second.filterId;
                    vit->second.filterId = c.biquadId;
                    int ch = streamChannels_ > 0 ? streamChannels_ : 2;
                    auto iit = iirs_.find(c.biquadId);
                    if (iit != iirs_.end()) {
                        size_t nb = iit->second.feedforward.size();
                        size_t na = iit->second.feedback.size();
                        size_t stateSize = 0;
                        if (std::max(nb, na) >= 1) stateSize = std::max(nb, na) - 1;
                        vit->second.iirState.assign(ch, std::vector<double>(stateSize, 0.0));
                    } else {
                        vit->second.filterState.assign(ch, NativeEngine::BiquadState());
                    }
                }

                if (!c.biquadId.empty()) {
                    voiceFilterByOutput_[c.id][c.outputIndex] = c.biquadId;
                } else {
                    auto vfIt = voiceFilterByOutput_.find(c.id);
                    if (vfIt != voiceFilterByOutput_.end()) {
                        vfIt->second.erase(c.outputIndex);
                        if (vfIt->second.empty()) voiceFilterByOutput_.erase(vfIt);
                    }
                }

                audioThreadLog(ANDROID_LOG_INFO,
                               "CMD: attach biquad/iir %s -> voice %s (out=%d in=%d)",
                               c.biquadId.c_str(), c.id.c_str(), c.outputIndex, c.inputIndex);
                if (traceEnabled) {
                    size_t mappedRefs = countMappedRefs(voiceFilterByOutput_, c.biquadId);
                    size_t fieldRefs = countVoiceFieldRefs(c.biquadId,
                                                           [](const Voice &voice) -> const std::string & {
                                                               return voice.filterId;
                                                           });
                    audioThreadLog(ANDROID_LOG_INFO,
                                   "GRAPH_TRACE seq=%llu ATTACH_FILTER voice=%s exists=%d prev=%s new=%s out=%d in=%d mappedRefs=%zu fieldRefs=%zu",
                                   traceSeq, c.id.c_str(), voiceExists ? 1 : 0,
                                   prevFilterId.c_str(), c.biquadId.c_str(), c.outputIndex,
                                   c.inputIndex, mappedRefs, fieldRefs);
                }
                break;
            }
            case NativeEngine::CMD_DETACH_BIQUAD: {
                size_t fieldRefsBefore = countVoiceFieldRefs(c.id,
                                                             [](const Voice &voice) -> const std::string & {
                                                                 return voice.filterId;
                                                             });
                size_t mappedRefsBefore = countMappedRefs(voiceFilterByOutput_, c.id);
                for (auto &kv: audioVoices_) {
                    if (kv.second.filterId == c.id) {
                        kv.second.filterId.clear();
                        kv.second.filterState.clear();
                        kv.second.iirState.clear();
                    }
                }
                audioThreadLog(ANDROID_LOG_INFO, "CMD: detach biquad %s", c.id.c_str());
                if (traceEnabled) {
                    size_t fieldRefsAfter = countVoiceFieldRefs(c.id,
                                                                [](const Voice &voice) -> const std::string & {
                                                                    return voice.filterId;
                                                                });
                    size_t mappedRefsAfter = countMappedRefs(voiceFilterByOutput_, c.id);
                    audioThreadLog(ANDROID_LOG_INFO,
                                   "GRAPH_TRACE seq=%llu DETACH_FILTER id=%s fieldRefsBefore=%zu fieldRefsAfter=%zu mappedRefsBefore=%zu mappedRefsAfter=%zu",
                                   traceSeq, c.id.c_str(), fieldRefsBefore, fieldRefsAfter,
                                   mappedRefsBefore, mappedRefsAfter);
                }
                break;
            }
            case NativeEngine::CMD_FREE_BIQUAD: {
                bool hadBiquad = biquads_.find(c.id) != biquads_.end();
                size_t fieldRefsBefore = countVoiceFieldRefs(c.id,
                                                             [](const Voice &voice) -> const std::string & {
                                                                 return voice.filterId;
                                                             });
                size_t mappedRefsBefore = countMappedRefs(voiceFilterByOutput_, c.id);
                biquads_.erase(c.id);
                for (auto &kv: audioVoices_) {
                    if (kv.second.filterId == c.id) {
                        kv.second.filterId.clear();
                        kv.second.filterState.clear();
                        kv.second.iirState.clear();
                    }
                }
                audioThreadLog(ANDROID_LOG_INFO, "CMD: free biquad %s", c.id.c_str());
                if (traceEnabled) {
                    size_t fieldRefsAfter = countVoiceFieldRefs(c.id,
                                                                [](const Voice &voice) -> const std::string & {
                                                                    return voice.filterId;
                                                                });
                    size_t mappedRefsAfter = countMappedRefs(voiceFilterByOutput_, c.id);
                    audioThreadLog(ANDROID_LOG_INFO,
                                   "GRAPH_TRACE seq=%llu FREE_FILTER id=%s hadBiquad=%d fieldRefsBefore=%zu fieldRefsAfter=%zu mappedRefsBefore=%zu mappedRefsAfter=%zu",
                                   traceSeq, c.id.c_str(), hadBiquad ? 1 : 0,
                                   fieldRefsBefore, fieldRefsAfter, mappedRefsBefore,
                                   mappedRefsAfter);
                }
                break;
            }
            case NativeEngine::CMD_CREATE_IIR: {
                NativeEngine::IIRData d;
                d.feedforward = c.iirFeedforward;
                d.feedback = c.iirFeedback;
                double a0 = 1.0;
                if (!d.feedback.empty() && d.feedback[0] != 0.0) a0 = d.feedback[0];
                if (a0 != 1.0) {
                    for (double &x: d.feedforward) x /= a0;
                    for (double &x: d.feedback) x /= a0;
                }
                NativeEngine::getInstance().iirs_[c.id] = d;
                audioThreadLog(ANDROID_LOG_INFO, "CMD: create iir %s ff=%zu fb=%zu",
                               c.id.c_str(), d.feedforward.size(), d.feedback.size());
                break;
            }
            case NativeEngine::CMD_FREE_IIR: {
                bool hadIir = NativeEngine::getInstance().iirs_.find(c.id) !=
                               NativeEngine::getInstance().iirs_.end();
                size_t fieldRefsBefore = countVoiceFieldRefs(c.id,
                                                             [](const Voice &voice) -> const std::string & {
                                                                 return voice.filterId;
                                                             });
                size_t mappedRefsBefore = countMappedRefs(voiceFilterByOutput_, c.id);
                NativeEngine::getInstance().iirs_.erase(c.id);
                for (auto &kv: audioVoices_) {
                    if (kv.second.filterId == c.id) {
                        kv.second.filterId.clear();
                        kv.second.iirState.clear();
                        kv.second.filterState.clear();
                    }
                }
                audioThreadLog(ANDROID_LOG_INFO, "CMD: free iir %s", c.id.c_str());
                if (traceEnabled) {
                    size_t fieldRefsAfter = countVoiceFieldRefs(c.id,
                                                                [](const Voice &voice) -> const std::string & {
                                                                    return voice.filterId;
                                                                });
                    size_t mappedRefsAfter = countMappedRefs(voiceFilterByOutput_, c.id);
                    audioThreadLog(ANDROID_LOG_INFO,
                                   "GRAPH_TRACE seq=%llu FREE_IIR id=%s hadIir=%d fieldRefsBefore=%zu fieldRefsAfter=%zu mappedRefsBefore=%zu mappedRefsAfter=%zu",
                                   traceSeq, c.id.c_str(), hadIir ? 1 : 0,
                                   fieldRefsBefore, fieldRefsAfter, mappedRefsBefore,
                                   mappedRefsAfter);
                }
                break;
            }
            case NativeEngine::CMD_CREATE_PERIODICWAVE: {
                NativeEngine::PeriodicWaveData d;
                d.real = c.periodicReal;
                d.imag = c.periodicImag;
                d.disableNormalization = c.periodicDisableNormalization;
                NativeEngine::getInstance().periodicWaves_[c.id] = d;
                audioThreadLog(ANDROID_LOG_INFO,
                               "CMD: create periodic %s real=%zu imag=%zu", c.id.c_str(),
                               d.real.size(), d.imag.size());
                break;
            }
            case NativeEngine::CMD_ATTACH_PERIODICWAVE: {
                auto vit = audioVoices_.find(c.id);
                if (vit != audioVoices_.end()) {
                    vit->second.periodicWaveId = c.periodicWaveId;
                }
                audioThreadLog(ANDROID_LOG_INFO, "CMD: attach periodic %s -> voice %s",
                               c.periodicWaveId.c_str(), c.id.c_str());
                break;
            }
            case NativeEngine::CMD_FREE_PERIODICWAVE: {
                NativeEngine::getInstance().periodicWaves_.erase(c.id);
                for (auto &kv: audioVoices_) {
                    if (kv.second.periodicWaveId == c.id) {
                        kv.second.periodicWaveId.clear();
                    }
                }
                audioThreadLog(ANDROID_LOG_INFO, "CMD: free periodic %s", c.id.c_str());
                break;
            }
            case NativeEngine::CMD_SET_WAVESHAPER_CURVE: {
                NativeEngine::WaveShaperData d;
                if (!c.waveShaperCurve.empty()) d.curve = c.waveShaperCurve;
                if (!c.waveShaperOversample.empty()) d.oversample = c.waveShaperOversample;
                auto it = waveShapers_.find(c.id);
                if (it == waveShapers_.end()) {
                    if (!d.curve.empty() || !d.oversample.empty()) {
                        waveShapers_[c.id] = d;
                    }
                } else {
                    if (!d.curve.empty()) it->second.curve = d.curve;
                    if (!c.waveShaperOversample.empty()) it->second.oversample = d.oversample;
                }
                audioThreadLog(ANDROID_LOG_INFO,
                               "CMD: set waveshaper %s curve=%zu oversample=%s",
                               c.id.c_str(), (size_t) d.curve.size(), d.oversample.c_str());
                break;
            }
            case NativeEngine::CMD_CREATE_PANNER: {
                bool replaced = panners_.find(c.id) != panners_.end();
                NativeEngine::Panner p;
                p.positionX = c.pannerPositionX;
                p.positionY = c.pannerPositionY;
                p.positionZ = c.pannerPositionZ;
                p.orientationX = c.pannerOrientationX;
                p.orientationY = c.pannerOrientationY;
                p.orientationZ = c.pannerOrientationZ;
                p.pan = c.pannerPan;
                p.distanceModel = static_cast<NativeEngine::PannerDistanceModel>(c.pannerDistanceModel);
                p.panningModel = static_cast<NativeEngine::PannerPanningModel>(c.pannerPanningModel);
                p.refDistance = c.pannerRefDistance;
                p.maxDistance = c.pannerMaxDistance;
                p.rolloffFactor = c.pannerRolloffFactor;
                p.coneInnerAngle = c.pannerConeInnerAngle;
                p.coneOuterAngle = c.pannerConeOuterAngle;
                p.coneOuterGain = c.pannerConeOuterGain;
                p.contextId = c.contextId;
                p.biquadId = "";
                panners_[c.id] = p;
                audioThreadLog(ANDROID_LOG_INFO, "CMD: create panner %s", c.id.c_str());
                if (traceEnabled) {
                    audioThreadLog(ANDROID_LOG_INFO,
                                   "GRAPH_TRACE seq=%llu CREATE_PANNER id=%s replaced=%d context=%s pos=(%.3f,%.3f,%.3f) pan=%.3f distModel=%d panModel=%d",
                                   traceSeq, c.id.c_str(), replaced ? 1 : 0,
                                   c.contextId.c_str(), c.pannerPositionX,
                                   c.pannerPositionY, c.pannerPositionZ, c.pannerPan,
                                   c.pannerDistanceModel, c.pannerPanningModel);
                }
                break;
            }
            case NativeEngine::CMD_SET_PANNER_PARAMS: {
                auto it = panners_.find(c.id);
                if (it != panners_.end()) {
                    if (!c.pannerBiquadId.empty()) {
                        std::string prevBiquadId = it->second.biquadId;
                        it->second.biquadId = c.pannerBiquadId;
                        audioThreadLog(ANDROID_LOG_INFO, "CMD: attach biquad %s -> panner %s", c.pannerBiquadId.c_str(), c.id.c_str());
                        if (traceEnabled) {
                            audioThreadLog(ANDROID_LOG_INFO,
                                           "GRAPH_TRACE seq=%llu SET_PANNER_BIQUAD panner=%s prevBiquad=%s newBiquad=%s",
                                           traceSeq, c.id.c_str(), prevBiquadId.c_str(),
                                           c.pannerBiquadId.c_str());
                        }
                        break;
                    }
                    auto &p = it->second;
                    p.positionX = c.pannerPositionX;
                    p.positionY = c.pannerPositionY;
                    p.positionZ = c.pannerPositionZ;
                    p.orientationX = c.pannerOrientationX;
                    p.orientationY = c.pannerOrientationY;
                    p.orientationZ = c.pannerOrientationZ;
                    p.pan = c.pannerPan;
                    p.distanceModel = static_cast<NativeEngine::PannerDistanceModel>(c.pannerDistanceModel);
                    p.panningModel = static_cast<NativeEngine::PannerPanningModel>(c.pannerPanningModel);
                    p.refDistance = c.pannerRefDistance;
                    p.maxDistance = c.pannerMaxDistance;
                    p.rolloffFactor = c.pannerRolloffFactor;
                    p.coneInnerAngle = c.pannerConeInnerAngle;
                    p.coneOuterAngle = c.pannerConeOuterAngle;
                    p.coneOuterGain = c.pannerConeOuterGain;
                } else if (traceEnabled) {
                    audioThreadLog(ANDROID_LOG_WARN,
                                   "GRAPH_TRACE seq=%llu SET_PANNER_MISSING panner=%s",
                                   traceSeq, c.id.c_str());
                }
                audioThreadLog(ANDROID_LOG_INFO, "CMD: set panner %s", c.id.c_str());
                if (traceEnabled && it != panners_.end()) {
                    audioThreadLog(ANDROID_LOG_INFO,
                                   "GRAPH_TRACE seq=%llu SET_PANNER_PARAMS panner=%s pos=(%.3f,%.3f,%.3f) orient=(%.3f,%.3f,%.3f) pan=%.3f ref=%.3f max=%.3f rolloff=%.3f",
                                   traceSeq, c.id.c_str(), c.pannerPositionX,
                                   c.pannerPositionY, c.pannerPositionZ,
                                   c.pannerOrientationX, c.pannerOrientationY,
                                   c.pannerOrientationZ, c.pannerPan,
                                   c.pannerRefDistance, c.pannerMaxDistance,
                                   c.pannerRolloffFactor);
                }
                break;
            }
            case NativeEngine::CMD_SET_PANNER_PARTITION_SIZE: {
                auto it = panners_.find(c.id);
                if (it == panners_.end()) {
                    if (traceEnabled) {
                        audioThreadLog(ANDROID_LOG_WARN,
                                       "GRAPH_TRACE seq=%llu SET_PANNER_PARTITION_MISSING panner=%s size=%d",
                                       traceSeq, c.id.c_str(), c.pannerPartitionSize);
                    }
                    break;
                }

                if (c.pannerPartitionSize > 0) {
                    it->second.hrtfPartitionSize = c.pannerPartitionSize;
                    if (it->second.hrtf) {
                        it->second.hrtf->setPartitionSize(c.pannerPartitionSize);
                    }
                }

                if (traceEnabled) {
                    audioThreadLog(ANDROID_LOG_INFO,
                                   "GRAPH_TRACE seq=%llu SET_PANNER_PARTITION panner=%s size=%d hasHrtf=%d",
                                   traceSeq, c.id.c_str(), it->second.hrtfPartitionSize,
                                   it->second.hrtf ? 1 : 0);
                }
                break;
            }
            case NativeEngine::CMD_SET_PANNER_HRIR: {
                auto it = panners_.find(c.id);
                if (it == panners_.end()) {
                    if (traceEnabled) {
                        audioThreadLog(ANDROID_LOG_WARN,
                                       "GRAPH_TRACE seq=%llu SET_PANNER_HRIR_MISSING panner=%s clear=%d",
                                       traceSeq, c.id.c_str(), c.clearHrir ? 1 : 0);
                    }
                    break;
                }

                if (c.clearHrir || !c.hrirLeft || !c.hrirRight ||
                    c.hrirLeft->empty() || c.hrirRight->empty()) {
                    bool hadHrtf = it->second.hrtf != nullptr;
                    if (it->second.hrtf) {
                        delete it->second.hrtf;
                        it->second.hrtf = nullptr;
                    }
                    if (traceEnabled) {
                        audioThreadLog(ANDROID_LOG_INFO,
                                       "GRAPH_TRACE seq=%llu SET_PANNER_HRIR_CLEAR panner=%s hadHrtf=%d",
                                       traceSeq, c.id.c_str(), hadHrtf ? 1 : 0);
                    }
                    break;
                }

                size_t count = std::min(c.hrirLeft->size(), c.hrirRight->size());
                if (count == 0) {
                    if (traceEnabled) {
                        audioThreadLog(ANDROID_LOG_WARN,
                                       "GRAPH_TRACE seq=%llu SET_PANNER_HRIR_EMPTY panner=%s left=%zu right=%zu",
                                       traceSeq, c.id.c_str(), c.hrirLeft->size(),
                                       c.hrirRight->size());
                    }
                    break;
                }

                HRTFConvolver *convolver = it->second.hrtf;
                if (!convolver) {
                    convolver = new HRTFConvolver();
                }
                if (it->second.hrtfPartitionSize > 0) {
                    convolver->setPartitionSize(it->second.hrtfPartitionSize);
                }
                convolver->init(c.hrirLeft->data(), c.hrirRight->data(), count, 48000);
                it->second.hrtf = convolver;

                if (traceEnabled) {
                    audioThreadLog(ANDROID_LOG_INFO,
                                   "GRAPH_TRACE seq=%llu SET_PANNER_HRIR panner=%s samples=%zu part=%d",
                                   traceSeq, c.id.c_str(), count,
                                   it->second.hrtfPartitionSize);
                }
                break;
            }
            case NativeEngine::CMD_SET_LISTENER_PARAMS: {
                if (!c.contextId.empty()) {
                    auto &L = listeners_[c.contextId];
                    L.positionX = c.listenerPositionX;
                    L.positionY = c.listenerPositionY;
                    L.positionZ = c.listenerPositionZ;
                    L.forwardX = c.listenerForwardX;
                    L.forwardY = c.listenerForwardY;
                    L.forwardZ = c.listenerForwardZ;
                    L.upX = c.listenerUpX;
                    L.upY = c.listenerUpY;
                    L.upZ = c.listenerUpZ;
                    audioThreadLog(ANDROID_LOG_INFO,
                                   "CMD: set listener[%s] pos=(%f,%f,%f) fwd=(%f,%f,%f)",
                                   c.contextId.c_str(), c.listenerPositionX, c.listenerPositionY,
                                   c.listenerPositionZ, c.listenerForwardX, c.listenerForwardY,
                                   c.listenerForwardZ);
                } else {
                    listener_.positionX = c.listenerPositionX;
                    listener_.positionY = c.listenerPositionY;
                    listener_.positionZ = c.listenerPositionZ;
                    listener_.forwardX = c.listenerForwardX;
                    listener_.forwardY = c.listenerForwardY;
                    listener_.forwardZ = c.listenerForwardZ;
                    listener_.upX = c.listenerUpX;
                    listener_.upY = c.listenerUpY;
                    listener_.upZ = c.listenerUpZ;
                    audioThreadLog(ANDROID_LOG_INFO,
                                   "CMD: set listener pos=(%f,%f,%f) fwd=(%f,%f,%f)",
                                   c.listenerPositionX, c.listenerPositionY, c.listenerPositionZ,
                                   c.listenerForwardX, c.listenerForwardY, c.listenerForwardZ);
                }
                break;
            }
            case NativeEngine::CMD_ATTACH_PANNER: {
                auto vit = audioVoices_.find(c.id);
                bool voiceExists = vit != audioVoices_.end();
                std::string prevPanId;
                std::string prevFilterId;
                if (vit != audioVoices_.end()) {
                    prevPanId = vit->second.panId;
                    prevFilterId = vit->second.filterId;
                    vit->second.panId = c.pannerId;
                    auto pit = panners_.find(c.pannerId);
                    if (pit != panners_.end())
                        vit->second.pan = static_cast<float>(pit->second.pan);
                }

                bool pannerExists = false;
                std::string pannerBiquadId;
                if (!c.pannerId.empty()) {
                    auto pit2 = panners_.find(c.pannerId);
                    if (pit2 != panners_.end()) {
                        pannerExists = true;
                        const std::string &pbid = pit2->second.biquadId;
                        pannerBiquadId = pbid;
                        if (!pbid.empty()) {
                            auto vit2 = audioVoices_.find(c.id);
                            if (vit2 != audioVoices_.end()) {
                                vit2->second.filterId = pbid;
                                int ch = streamChannels_ > 0 ? streamChannels_ : 2;
                                auto iit = iirs_.find(pbid);
                                if (iit != iirs_.end()) {
                                    size_t nb = iit->second.feedforward.size();
                                    size_t na = iit->second.feedback.size();
                                    size_t stateSize = 0;
                                    if (std::max(nb, na) >= 1) stateSize = std::max(nb, na) - 1;
                                    vit2->second.iirState.assign(ch, std::vector<double>(stateSize, 0.0));
                                } else {
                                    vit2->second.filterState.assign(ch, NativeEngine::BiquadState());
                                }
                                voiceFilterByOutput_[c.id][c.outputIndex] = pbid;
                            }
                        }
                    }
                }

                if (!c.pannerId.empty()) {
                    voicePannerByOutput_[c.id][c.outputIndex] = c.pannerId;
                } else {
                    auto vpIt = voicePannerByOutput_.find(c.id);
                    if (vpIt != voicePannerByOutput_.end()) {
                        vpIt->second.erase(c.outputIndex);
                        if (vpIt->second.empty()) voicePannerByOutput_.erase(vpIt);
                    }
                }

                audioThreadLog(ANDROID_LOG_INFO,
                               "CMD: attach panner %s -> voice %s (out=%d in=%d)",
                               c.pannerId.c_str(), c.id.c_str(), c.outputIndex, c.inputIndex);
                if (traceEnabled) {
                    size_t panMappedRefs = countMappedRefs(voicePannerByOutput_, c.pannerId);
                    size_t panFieldRefs = countVoiceFieldRefs(c.pannerId,
                                                              [](const Voice &voice) -> const std::string & {
                                                                  return voice.panId;
                                                              });
                    size_t outputsForVoice = 0;
                    auto vpDbg = voicePannerByOutput_.find(c.id);
                    if (vpDbg != voicePannerByOutput_.end()) outputsForVoice = vpDbg->second.size();

                    size_t filterMappedRefs = 0;
                    size_t filterFieldRefs = 0;
                    if (!pannerBiquadId.empty()) {
                        filterMappedRefs = countMappedRefs(voiceFilterByOutput_, pannerBiquadId);
                        filterFieldRefs = countVoiceFieldRefs(pannerBiquadId,
                                                              [](const Voice &voice) -> const std::string & {
                                                                  return voice.filterId;
                                                              });
                    }

                    audioThreadLog(ANDROID_LOG_INFO,
                                   "GRAPH_TRACE seq=%llu ATTACH_PANNER voice=%s exists=%d prevPan=%s newPan=%s prevFilter=%s pannerExists=%d pannerBiquad=%s out=%d in=%d panMappedRefs=%zu panFieldRefs=%zu filterMappedRefs=%zu filterFieldRefs=%zu outputsForVoice=%zu",
                                   traceSeq, c.id.c_str(), voiceExists ? 1 : 0,
                                   prevPanId.c_str(), c.pannerId.c_str(),
                                   prevFilterId.c_str(), pannerExists ? 1 : 0,
                                   pannerBiquadId.c_str(), c.outputIndex, c.inputIndex,
                                   panMappedRefs, panFieldRefs, filterMappedRefs,
                                   filterFieldRefs, outputsForVoice);
                }
                break;
            }
            case NativeEngine::CMD_DETACH_PANNER: {
                size_t panFieldRefsBefore = countVoiceFieldRefs(c.id,
                                                                [](const Voice &voice) -> const std::string & {
                                                                    return voice.panId;
                                                                });
                size_t panMappedRefsBefore = countMappedRefs(voicePannerByOutput_, c.id);
                for (auto &kv: audioVoices_) {
                    if (kv.second.panId == c.id) {
                        kv.second.panId.clear();
                        kv.second.pan = 0.0f;
                    }
                }
                audioThreadLog(ANDROID_LOG_INFO, "CMD: detach panner %s", c.id.c_str());
                if (traceEnabled) {
                    size_t panFieldRefsAfter = countVoiceFieldRefs(c.id,
                                                                   [](const Voice &voice) -> const std::string & {
                                                                       return voice.panId;
                                                                   });
                    size_t panMappedRefsAfter = countMappedRefs(voicePannerByOutput_, c.id);
                    audioThreadLog(ANDROID_LOG_INFO,
                                   "GRAPH_TRACE seq=%llu DETACH_PANNER id=%s panFieldRefsBefore=%zu panFieldRefsAfter=%zu panMappedRefsBefore=%zu panMappedRefsAfter=%zu",
                                   traceSeq, c.id.c_str(), panFieldRefsBefore,
                                   panFieldRefsAfter, panMappedRefsBefore,
                                   panMappedRefsAfter);
                }
                break;
            }
            case NativeEngine::CMD_FREE_PANNER: {
                bool hadPanner = panners_.find(c.id) != panners_.end();
                size_t panFieldRefsBefore = countVoiceFieldRefs(c.id,
                                                                [](const Voice &voice) -> const std::string & {
                                                                    return voice.panId;
                                                                });
                size_t panMappedRefsBefore = countMappedRefs(voicePannerByOutput_, c.id);

                bool hadHrtf = false;
                std::string bid;
                size_t filterFieldRefsBefore = 0;
                size_t filterMappedRefsBefore = 0;

                auto pitf = panners_.find(c.id);
                if (pitf != panners_.end()) {
                    hadHrtf = pitf->second.hrtf != nullptr;
                    if (pitf->second.hrtf) {
                        delete pitf->second.hrtf;
                        pitf->second.hrtf = nullptr;
                    }
                    bid = pitf->second.biquadId;
                    if (!bid.empty()) {
                        filterFieldRefsBefore = countVoiceFieldRefs(bid,
                                                                    [](const Voice &voice) -> const std::string & {
                                                                        return voice.filterId;
                                                                    });
                        filterMappedRefsBefore = countMappedRefs(voiceFilterByOutput_, bid);
                        biquads_.erase(bid);
                        for (auto &kv: audioVoices_) {
                            if (kv.second.filterId == bid) {
                                kv.second.filterId.clear();
                                kv.second.filterState.clear();
                                kv.second.iirState.clear();
                            }
                        }
                    }
                }
                panners_.erase(c.id);
                for (auto &kv: audioVoices_) {
                    if (kv.second.panId == c.id) {
                        kv.second.panId.clear();
                        kv.second.pan = 0.0f;
                    }
                }
                audioThreadLog(ANDROID_LOG_INFO, "CMD: free panner %s", c.id.c_str());
                if (traceEnabled) {
                    size_t panFieldRefsAfter = countVoiceFieldRefs(c.id,
                                                                   [](const Voice &voice) -> const std::string & {
                                                                       return voice.panId;
                                                                   });
                    size_t panMappedRefsAfter = countMappedRefs(voicePannerByOutput_, c.id);

                    size_t filterFieldRefsAfter = 0;
                    size_t filterMappedRefsAfter = 0;
                    if (!bid.empty()) {
                        filterFieldRefsAfter = countVoiceFieldRefs(bid,
                                                                   [](const Voice &voice) -> const std::string & {
                                                                       return voice.filterId;
                                                                   });
                        filterMappedRefsAfter = countMappedRefs(voiceFilterByOutput_, bid);
                    }

                    audioThreadLog(ANDROID_LOG_INFO,
                                   "GRAPH_TRACE seq=%llu FREE_PANNER id=%s hadPanner=%d hadHrtf=%d pannerBiquad=%s panFieldRefsBefore=%zu panFieldRefsAfter=%zu panMappedRefsBefore=%zu panMappedRefsAfter=%zu filterFieldRefsBefore=%zu filterFieldRefsAfter=%zu filterMappedRefsBefore=%zu filterMappedRefsAfter=%zu",
                                   traceSeq, c.id.c_str(), hadPanner ? 1 : 0,
                                   hadHrtf ? 1 : 0, bid.c_str(), panFieldRefsBefore,
                                   panFieldRefsAfter, panMappedRefsBefore,
                                   panMappedRefsAfter, filterFieldRefsBefore,
                                   filterFieldRefsAfter, filterMappedRefsBefore,
                                   filterMappedRefsAfter);
                }
                break;
            }
            case NativeEngine::CMD_START_OSC: {
                Voice v;
                auto it = audioVoices_.find(c.id);
                if (it == audioVoices_.end()) {
                    v.voiceType = Voice::Oscillator;
                    v.id = c.id;
                    v.waveform = c.waveform;
                    v.frequency = c.frequency;
                    v.phase = 0.0;
                } else v = it->second;
                v.waveform = c.waveform;
                v.frequency = c.frequency;
                double sr =
                        streamSampleRate_ > 0 ? static_cast<double>(streamSampleRate_) : 48000.0;
                v.phaseIncrement = v.frequency / sr;
                v.playing = true;

                auto vgInitIt2 = voiceGainByOutput_.find(v.id);
                if (vgInitIt2 != voiceGainByOutput_.end() && !vgInitIt2->second.empty()) {
                    auto mapIt2 = vgInitIt2->second.find(0);
                    if (mapIt2 == vgInitIt2->second.end()) mapIt2 = vgInitIt2->second.begin();
                    v.gainId = mapIt2->second;
                    auto git2 = gains_.find(v.gainId);
                    if (git2 != gains_.end()) v.gain = git2->second;
                }

                hydrateVoicePannerDefaults(v);
                audioVoices_[c.id] = v;
                break;
            }
            case NativeEngine::CMD_STOP_TRACK: {
                auto vit = audioVoices_.find(c.id);
                bool existed = vit != audioVoices_.end();
                size_t gainOutputsForVoice = 0;
                size_t pannerOutputsForVoice = 0;
                size_t filterOutputsForVoice = 0;
                if (traceEnabled && existed) {
                    auto vg = voiceGainByOutput_.find(c.id);
                    if (vg != voiceGainByOutput_.end()) gainOutputsForVoice = vg->second.size();
                    auto vp = voicePannerByOutput_.find(c.id);
                    if (vp != voicePannerByOutput_.end()) pannerOutputsForVoice = vp->second.size();
                    auto vf = voiceFilterByOutput_.find(c.id);
                    if (vf != voiceFilterByOutput_.end()) filterOutputsForVoice = vf->second.size();

                    if (vit->second.voiceType == Voice::ExternalPCM) {
                        logRingState(traceSeq, "STOP_TRACK_BEFORE", c.id,
                                     vit->second.externalRing,
                                     vit->second.bufferChannels,
                                     vit->second.bufferSampleRate,
                                     vit->second.playing);
                    }

                    audioThreadLog(ANDROID_LOG_INFO,
                                   "GRAPH_TRACE seq=%llu STOP_TRACK voice=%s existed=%d type=%s gainId=%s panId=%s filterId=%s gainOutputs=%zu panOutputs=%zu filterOutputs=%zu",
                                   traceSeq, c.id.c_str(), existed ? 1 : 0,
                                   voiceTypeName(vit->second.voiceType),
                                   vit->second.gainId.c_str(), vit->second.panId.c_str(),
                                   vit->second.filterId.c_str(), gainOutputsForVoice,
                                   pannerOutputsForVoice, filterOutputsForVoice);
                }
                audioVoices_.erase(c.id);
                break;
            }
            case NativeEngine::CMD_START_BUFFER: {
                auto bit = audioBuffers_.find(c.id);
                if (bit == audioBuffers_.end()) break;
                Voice v;
                v.voiceType = Voice::BufferSource;
                v.id = c.id;
                v.bufferId = c.id;
                v.position = 0.0;
                v.loop = c.loop;
                v.gain = 1.0;
                v.playing = true;
                v.bufferChannels = bit->second.channels;
                v.bufferSampleRate = bit->second.sampleRate;
                double sr =
                        streamSampleRate_ > 0 ? static_cast<double>(streamSampleRate_) : 48000.0;
                v.increment = static_cast<double>(v.bufferSampleRate) / sr;

                auto vgInitIt3 = voiceGainByOutput_.find(v.id);
                if (vgInitIt3 != voiceGainByOutput_.end() && !vgInitIt3->second.empty()) {
                    auto mapIt3 = vgInitIt3->second.find(0);
                    if (mapIt3 == vgInitIt3->second.end()) mapIt3 = vgInitIt3->second.begin();
                    v.gainId = mapIt3->second;
                    auto git3 = gains_.find(v.gainId);
                    if (git3 != gains_.end()) v.gain = git3->second;
                }
                auto vprInitIt = voicePlaybackRateByOutput_.find(v.id);
                if (vprInitIt != voicePlaybackRateByOutput_.end() && !vprInitIt->second.empty()) {
                    auto mapIt4 = vprInitIt->second.find(0);
                    if (mapIt4 == vprInitIt->second.end()) mapIt4 = vprInitIt->second.begin();
                    v.playbackRateId = mapIt4->second;
                    auto git4 = gains_.find(v.playbackRateId);
                    if (git4 != gains_.end()) {
                        
                    }
                }

                hydrateVoicePannerDefaults(v);
                audioVoices_[c.id] = v;
                break;
            }
            case NativeEngine::CMD_FREE_BUFFER: {
                auto it = audioBuffers_.find(c.id);
                if (it != audioBuffers_.end()) {
                    if (it->second.nativeOwned) {
                        it->second.pcm.clear();
                        it->second.pcm.shrink_to_fit();
                    }
                    it->second.planarPtrs.clear();
                    it->second.planarFrames = 0;
                    it->second.directPtr = nullptr;
                    it->second.byteLength = 0;
                    it->second.javaBufferId.clear();
                    audioBuffers_.erase(it);
                }

                if (c.completion) {
                    try {
                        c.completion->set_value();
                    } catch (...) {}
                }

                if (jvm_ && audioContextClassGlobalRef && onNativeBufferReleasedMethod) {
                    JNIEnv *env = nullptr;
                    bool attached = false;
                    if (jvm_->GetEnv(reinterpret_cast<void **>(&env), JNI_VERSION_1_6) != JNI_OK) {
                        if (jvm_->AttachCurrentThread(&env, nullptr) == JNI_OK) attached = true;
                        else env = nullptr;
                    }
                    if (env) {
                        jstring js = env->NewStringUTF(c.id.c_str());
                        env->CallStaticVoidMethod(audioContextClassGlobalRef,
                                                  onNativeBufferReleasedMethod, js);
                        env->DeleteLocalRef(js);
                        if (attached) jvm_->DetachCurrentThread();
                    }
                }
                break;
            }
            case NativeEngine::CMD_RESUME: {
                ensureStream();
                break;
            }
            case NativeEngine::CMD_SUSPEND: {
                stopStream();
                break;
            }
            case NativeEngine::CMD_CREATE_EXTERNAL_PCM: {
                if (!c.externalRing) {
                    if (traceEnabled) {
                        audioThreadLog(ANDROID_LOG_WARN,
                                       "GRAPH_TRACE seq=%llu CREATE_EXTERNAL_PCM voice=%s ring=null",
                                       traceSeq, c.id.c_str());
                    }
                    break;
                }
                bool replacedVoice = audioVoices_.find(c.id) != audioVoices_.end();
                Voice v;
                v.voiceType = Voice::ExternalPCM;
                v.id = c.id;
                v.position = 0.0;
                v.loop = false;
                v.gain = 1.0;
                v.playing = false;
                v.bufferChannels = c.channels > 0 ? c.channels : 2;
                v.bufferSampleRate = c.sampleRate > 0 ? c.sampleRate : 48000;
                double sr =
                        streamSampleRate_ > 0 ? static_cast<double>(streamSampleRate_) : 48000.0;
                v.increment = static_cast<double>(v.bufferSampleRate) / sr;
                v.externalRing = c.externalRing;
                v.externalPrev.assign((size_t) v.bufferChannels, 0.0f);
                v.externalCurr.assign((size_t) v.bufferChannels, 0.0f);
                v.externalSubPos = 0.0;
                v.externalPrimed = false;

                auto vgInit = voiceGainByOutput_.find(v.id);
                if (vgInit != voiceGainByOutput_.end() && !vgInit->second.empty()) {
                    auto mapIt = vgInit->second.find(0);
                    if (mapIt == vgInit->second.end()) mapIt = vgInit->second.begin();
                    v.gainId = mapIt->second;
                    auto git = gains_.find(v.gainId);
                    if (git != gains_.end()) v.gain = git->second;
                }
                audioVoices_[c.id] = v;
                if (traceEnabled) {
                    audioThreadLog(ANDROID_LOG_INFO,
                                   "GRAPH_TRACE seq=%llu CREATE_EXTERNAL_PCM voice=%s replaced=%d gainId=%s increment=%f",
                                   traceSeq, c.id.c_str(), replacedVoice ? 1 : 0,
                                   v.gainId.c_str(), v.increment);
                    logRingState(traceSeq, "CREATE_EXTERNAL_PCM_RING", c.id, v.externalRing,
                                 v.bufferChannels, v.bufferSampleRate, v.playing);
                }
                break;
            }
            case NativeEngine::CMD_CONFIGURE_EXTERNAL_PCM: {
                auto vit = audioVoices_.find(c.id);
                if (vit == audioVoices_.end() || vit->second.voiceType != Voice::ExternalPCM) {
                    if (traceEnabled) {
                        const char *foundType = vit == audioVoices_.end()
                                                ? "missing"
                                                : voiceTypeName(vit->second.voiceType);
                        audioThreadLog(ANDROID_LOG_WARN,
                                       "GRAPH_TRACE seq=%llu CONFIG_EXTERNAL_PCM_SKIPPED voice=%s found=%s",
                                       traceSeq, c.id.c_str(), foundType);
                    }
                    break;
                }

                Voice &v = vit->second;
                if (traceEnabled) {
                    logRingState(traceSeq, "CONFIG_EXTERNAL_PCM_BEFORE", c.id, v.externalRing,
                                 v.bufferChannels, v.bufferSampleRate, v.playing);
                }
                int channelsForVoice = c.channels > 0 ? c.channels : 2;
                int sampleRateForVoice = c.sampleRate > 0 ? c.sampleRate : 48000;
                double sr = streamSampleRate_ > 0 ? static_cast<double>(streamSampleRate_) : 48000.0;

                v.bufferChannels = channelsForVoice;
                v.bufferSampleRate = sampleRateForVoice;
                v.increment = static_cast<double>(sampleRateForVoice) / sr;
                v.externalPrev.assign(static_cast<size_t>(channelsForVoice), 0.0f);
                v.externalCurr.assign(static_cast<size_t>(channelsForVoice), 0.0f);
                v.externalSubPos = 0.0;
                v.externalPrimed = false;

                auto &ring = v.externalRing;
                if (ring) {
                    ring->channels = channelsForVoice;
                    ring->data.assign(static_cast<size_t>(ring->capacity) * static_cast<size_t>(channelsForVoice), 0.0f);
                    ring->writeIdx.store(0, std::memory_order_release);
                    ring->readIdx.store(0, std::memory_order_release);
                    ring->ended.store(false, std::memory_order_release);
                    v.playing = false;
                }

                audioThreadLog(ANDROID_LOG_INFO,
                               "CMD: configure external pcm %s sr=%d ch=%d",
                               c.id.c_str(), sampleRateForVoice, channelsForVoice);
                if (traceEnabled) {
                    logRingState(traceSeq, "CONFIG_EXTERNAL_PCM_AFTER", c.id, v.externalRing,
                                 v.bufferChannels, v.bufferSampleRate, v.playing);
                }
                break;
            }
            case NativeEngine::CMD_PUSH_EXTERNAL_PCM: {
                auto vit = audioVoices_.find(c.id);
                if (vit == audioVoices_.end() || vit->second.voiceType != Voice::ExternalPCM) {
                    if (traceEnabled) {
                        const char *foundType = vit == audioVoices_.end()
                                                ? "missing"
                                                : voiceTypeName(vit->second.voiceType);
                        audioThreadLog(ANDROID_LOG_WARN,
                                       "GRAPH_TRACE seq=%llu PUSH_EXTERNAL_PCM_SKIPPED voice=%s found=%s",
                                       traceSeq, c.id.c_str(), foundType);
                    }
                    break;
                }
                auto &ring = vit->second.externalRing;
                if (!ring || !c.pcmFloat || c.pcmFloat->empty()) {
                    if (traceEnabled) {
                        audioThreadLog(ANDROID_LOG_WARN,
                                       "GRAPH_TRACE seq=%llu PUSH_EXTERNAL_PCM_EMPTY voice=%s ring=%p hasPcm=%d size=%zu",
                                       traceSeq, c.id.c_str(), static_cast<void *>(ring.get()),
                                       c.pcmFloat ? 1 : 0,
                                       c.pcmFloat ? c.pcmFloat->size() : 0U);
                    }
                    break;
                }

                const std::vector<float> &src = *c.pcmFloat;
                int chs = resolveExternalChannelCount(vit->second, ring);
                if (chs <= 0) {
                    if (traceEnabled) {
                        audioThreadLog(ANDROID_LOG_ERROR,
                                       "GRAPH_TRACE seq=%llu PUSH_EXTERNAL_PCM_BAD_LAYOUT voice=%s ring=%p ringCh=%d voiceCh=%d prev=%zu curr=%zu capacity=%u data=%zu",
                                       traceSeq, c.id.c_str(), static_cast<void *>(ring.get()),
                                       ring->channels, vit->second.bufferChannels,
                                       vit->second.externalPrev.size(),
                                       vit->second.externalCurr.size(), ring->capacity,
                                       ring->data.size());
                    }
                    break;
                }
                if (traceEnabled && (src.size() % static_cast<size_t>(chs) != 0)) {
                    audioThreadLog(ANDROID_LOG_WARN,
                                   "GRAPH_TRACE seq=%llu PUSH_EXTERNAL_PCM_PARTIAL voice=%s samples=%zu channels=%d remainder=%zu",
                                   traceSeq, c.id.c_str(), src.size(), chs,
                                   src.size() % static_cast<size_t>(chs));
                }
                size_t framesIn = src.size() / (size_t) chs;
                if (framesIn == 0) break;

                uint32_t w = ring->writeIdx.load(std::memory_order_relaxed);
                uint32_t r = ring->readIdx.load(std::memory_order_acquire);
                uint32_t avail = ring->capacity - (w - r);
                uint32_t writeFrames = framesIn > avail ? avail : (uint32_t) framesIn;
                if (writeFrames > 0) {
                    vit->second.playing = true;
                }
                for (uint32_t i = 0; i < writeFrames; i++) {
                    uint32_t pos = (w + i) & ring->mask;
                    for (int ch = 0; ch < chs; ch++) {
                        ring->data[(size_t) pos * chs + ch] = src[(size_t) i * chs + ch];
                    }
                }
                uint32_t nextW = w + writeFrames;
                ring->writeIdx.store(nextW, std::memory_order_release);
                if (traceEnabled && writeFrames < framesIn) {
                    audioThreadLog(ANDROID_LOG_WARN,
                                   "GRAPH_TRACE seq=%llu PUSH_EXTERNAL_PCM_DROPPED voice=%s requested=%zu written=%u dropped=%zu avail=%u w=%u r=%u cap=%u",
                                   traceSeq, c.id.c_str(), framesIn, writeFrames,
                                   framesIn - writeFrames, avail, w, r, ring->capacity);
                }
                if (traceEnabled && writeFrames > 0 && ((traceSeq & 0x3fULL) == 0ULL)) {
                    audioThreadLog(ANDROID_LOG_INFO,
                                   "GRAPH_TRACE seq=%llu PUSH_EXTERNAL_PCM_OK voice=%s requested=%zu written=%u w=%u r=%u nextW=%u cap=%u",
                                   traceSeq, c.id.c_str(), framesIn, writeFrames,
                                   w, r, nextW, ring->capacity);
                }
                break;
            }
            case NativeEngine::CMD_END_EXTERNAL_PCM: {
                auto vit = audioVoices_.find(c.id);
                if (vit != audioVoices_.end() && vit->second.externalRing) {
                    vit->second.externalRing->ended.store(true, std::memory_order_release);
                    if (traceEnabled) {
                        logRingState(traceSeq, "END_EXTERNAL_PCM", c.id,
                                     vit->second.externalRing,
                                     vit->second.bufferChannels,
                                     vit->second.bufferSampleRate,
                                     vit->second.playing);
                    }
                } else if (traceEnabled) {
                    audioThreadLog(ANDROID_LOG_WARN,
                                   "GRAPH_TRACE seq=%llu END_EXTERNAL_PCM_SKIPPED voice=%s hasVoice=%d hasRing=%d",
                                   traceSeq, c.id.c_str(), vit != audioVoices_.end() ? 1 : 0,
                                   (vit != audioVoices_.end() && vit->second.externalRing) ? 1
                                                                                          : 0);
                }
                break;
            }
            default:
                break;
        }
    }

    static constexpr int kRouteCacheChannels = 8;

    struct ActiveVoiceView {
        Voice *voice;
        const BufferData *bufferData;
        const std::unordered_map<int, std::string> *gainRoute;
        const std::unordered_map<int, std::string> *filterRoute;
        const std::unordered_map<int, std::string> *pannerRoute;
        const std::unordered_map<int, std::string> *playbackRateRoute;
        std::array<const std::string *, kRouteCacheChannels> gainIdByOutput{};
        std::array<const std::string *, kRouteCacheChannels> filterIdByOutput{};
        std::array<const std::string *, kRouteCacheChannels> pannerIdByOutput{};
        std::array<const NativeEngine::Panner *, kRouteCacheChannels> pannerNodeByOutput{};
        const std::string *playbackRatePrimaryId = nullptr;
    };

    std::vector<ActiveVoiceView> localVoices;
    localVoices.reserve(audioVoices_.size());
    for (auto &kv: audioVoices_) {
        Voice &voice = kv.second;
        if (!voice.playing) continue;

        const BufferData *bufferData = nullptr;
        if (voice.voiceType == Voice::BufferSource) {
            auto it = audioBuffers_.find(voice.bufferId);
            if (it != audioBuffers_.end()) bufferData = &it->second;
        }

        const std::unordered_map<int, std::string> *gainRoute = nullptr;
        const std::unordered_map<int, std::string> *filterRoute = nullptr;
        const std::unordered_map<int, std::string> *pannerRoute = nullptr;
        const std::unordered_map<int, std::string> *playbackRateRoute = nullptr;

        auto gainRouteIt = voiceGainByOutput_.find(voice.id);
        if (gainRouteIt != voiceGainByOutput_.end()) gainRoute = &gainRouteIt->second;

        auto filterRouteIt = voiceFilterByOutput_.find(voice.id);
        if (filterRouteIt != voiceFilterByOutput_.end()) filterRoute = &filterRouteIt->second;

        auto pannerRouteIt = voicePannerByOutput_.find(voice.id);
        if (pannerRouteIt != voicePannerByOutput_.end()) pannerRoute = &pannerRouteIt->second;

        auto playbackRateRouteIt = voicePlaybackRateByOutput_.find(voice.id);
        if (playbackRateRouteIt != voicePlaybackRateByOutput_.end()) {
            playbackRateRoute = &playbackRateRouteIt->second;
        }

        ActiveVoiceView view{
                &voice,
                bufferData,
                gainRoute,
                filterRoute,
                pannerRoute,
                playbackRateRoute,
        };

        for (int out = 0; out < kRouteCacheChannels; ++out) {
            const std::string *gainId = &voice.gainId;
            if (gainRoute) {
                auto it = gainRoute->find(out);
                if (it != gainRoute->end()) gainId = &it->second;
            }
            view.gainIdByOutput[out] = gainId;

            const std::string *filterId = &voice.filterId;
            if (filterRoute) {
                auto it = filterRoute->find(out);
                if (it != filterRoute->end()) filterId = &it->second;
            }
            view.filterIdByOutput[out] = filterId;

            const std::string *pannerId = &voice.panId;
            if (pannerRoute) {
                auto it = pannerRoute->find(out);
                if (it != pannerRoute->end()) pannerId = &it->second;
            }
            view.pannerIdByOutput[out] = pannerId;

            const NativeEngine::Panner *pannerNode = nullptr;
            if (pannerId && !pannerId->empty()) {
                auto pit = panners_.find(*pannerId);
                if (pit != panners_.end()) pannerNode = &pit->second;
            }
            view.pannerNodeByOutput[out] = pannerNode;
        }

        const std::string *playbackRateId = &voice.playbackRateId;
        if (playbackRateRoute && !playbackRateRoute->empty()) {
            auto mapIt = playbackRateRoute->find(0);
            if (mapIt == playbackRateRoute->end()) mapIt = playbackRateRoute->begin();
            if (mapIt != playbackRateRoute->end() && !mapIt->second.empty()) {
                playbackRateId = &mapIt->second;
            }
        }
        view.playbackRatePrimaryId = playbackRateId;

        localVoices.push_back(std::move(view));
    }

    std::fill(out, out + (numFrames * channels), 0.0f);

    std::unordered_set<std::string> usedGains;
    std::unordered_set<std::string> usedPanners;
    std::unordered_set<std::string> usedContexts;
    if (!localVoices.empty()) {
        auto addCompressorParamGains = [&](const std::string &gainId) {
            if (gainId.empty()) return;
            auto cit = compressors_.find(gainId);
            if (cit == compressors_.end()) return;
            const auto &comp = cit->second;
            if (!comp.thresholdId.empty()) usedGains.insert(comp.thresholdId);
            if (!comp.kneeId.empty()) usedGains.insert(comp.kneeId);
            if (!comp.ratioId.empty()) usedGains.insert(comp.ratioId);
            if (!comp.attackId.empty()) usedGains.insert(comp.attackId);
            if (!comp.releaseId.empty()) usedGains.insert(comp.releaseId);
            if (!comp.reductionId.empty()) usedGains.insert(comp.reductionId);
        };

        for (const auto &voiceView: localVoices) {
            const Voice &v = *voiceView.voice;
            if (!v.gainId.empty()) {
                usedGains.insert(v.gainId);
                addCompressorParamGains(v.gainId);
            }
            if (voiceView.gainRoute) {
                for (const auto &kv2: *voiceView.gainRoute) {
                    if (!kv2.second.empty()) {
                        usedGains.insert(kv2.second);
                        addCompressorParamGains(kv2.second);
                    }
                }
            }
            if (!v.playbackRateId.empty()) usedGains.insert(v.playbackRateId);
            if (voiceView.playbackRateRoute) {
                for (const auto &kv2: *voiceView.playbackRateRoute) {
                    if (!kv2.second.empty()) usedGains.insert(kv2.second);
                }
            }

            if (!v.panId.empty()) usedPanners.insert(v.panId);
            if (voiceView.pannerRoute) {
                for (const auto &kv2: *voiceView.pannerRoute) {
                    if (!kv2.second.empty()) usedPanners.insert(kv2.second);
                }
            }
        }

        for (const auto &pid: usedPanners) {
            auto pit = panners_.find(pid);
            if (pit != panners_.end() && !pit->second.contextId.empty()) {
                usedContexts.insert(pit->second.contextId);
            }
        }
    }

    std::unordered_map<std::string, std::vector<NativeEngine::ParamEvent>> scheduledSnapshot;
    std::unordered_map<std::string, std::unordered_map<int, std::vector<NativeEngine::ParamEvent>>> scheduledPannerSnapshot;
    std::unordered_map<std::string, std::unordered_map<int, std::vector<NativeEngine::ParamEvent>>> scheduledListenerSnapshot;
    {
        std::lock_guard<std::mutex> lk(scheduledEventsMutex_);
        for (const auto &gainId: usedGains) {
            auto it = scheduledGainEvents_.find(gainId);
            if (it != scheduledGainEvents_.end()) {
                scheduledSnapshot.emplace(gainId, it->second);
            }
        }
        for (const auto &pannerId: usedPanners) {
            auto it = scheduledPannerEvents_.find(pannerId);
            if (it != scheduledPannerEvents_.end()) {
                scheduledPannerSnapshot.emplace(pannerId, it->second);
            }
        }
        for (const auto &contextId: usedContexts) {
            auto it = scheduledListenerEvents_.find(contextId);
            if (it != scheduledListenerEvents_.end()) {
                scheduledListenerSnapshot.emplace(contextId, it->second);
            }
        }
    }

    int64_t audioStartNs = g_audioTimeNanos.load(std::memory_order_relaxed);
    double sampleDurationNs =
            streamSampleRate_ > 0 ? (1000000000.0 / static_cast<double>(streamSampleRate_)) : (
                    1000000000.0 / 48000.0);


    std::unordered_map<std::string, std::vector<double>> scheduledEnvelopes;
    std::unordered_map<std::string, std::unordered_map<int, std::vector<double>>> scheduledPannerEnvelopes;
    std::unordered_map<std::string, std::unordered_map<int, std::vector<double>>> scheduledListenerEnvelopes;
    if ((!scheduledSnapshot.empty() || !scheduledPannerSnapshot.empty()) && !localVoices.empty()) {

        for (const auto &kv: scheduledSnapshot) {
            const std::string &gainId = kv.first;
            if (usedGains.find(gainId) == usedGains.end()) continue;
            const auto &evts = kv.second;
            std::vector<double> env((size_t) numFrames);

            double fallback = 1.0;
            auto git = gains_.find(gainId);
            if (git != gains_.end()) fallback = git->second;

            size_t nextIdx = 0;
            int rate = evts.empty() ? NativeEngine::RATE_A : evts[0].rate;
            if (rate == NativeEngine::RATE_K) {
                float v = getScheduledGainValueAt(evts, static_cast<int64_t>(audioStartNs),
                                                  fallback);
                for (int f = 0; f < numFrames; ++f) env[(size_t) f] = v;
            } else {
                size_t nextIdx = 0;
                int prevIdx = -1;
                for (int f = 0; f < numFrames; ++f) {
                    auto tNs = static_cast<int64_t>(audioStartNs +
                                                    static_cast<double>(f) * sampleDurationNs);
                    while (nextIdx < evts.size() && evts[nextIdx].timeNs <= tNs) {
                        prevIdx = static_cast<int>(nextIdx);
                        ++nextIdx;
                    }
                    double val;
                    if (prevIdx == -1) {
                        val = fallback;
                    } else if (nextIdx >= evts.size()) {
                        val = evts[prevIdx].value;
                    } else {
                        const auto &prev = evts[prevIdx];
                        const auto &next = evts[nextIdx];
                        if (next.type == 1) {
                            if (next.timeNs <= prev.timeNs) val = next.value;
                            else {
                                double ratio =
                                        double(tNs - prev.timeNs) /
                                        double(next.timeNs - prev.timeNs);
                                if (ratio < 0.0) ratio = 0.0;
                                if (ratio > 1.0) ratio = 1.0;
                                val = prev.value + (next.value - prev.value) * ratio;
                            }
                        } else {
                            val = prev.value;
                        }
                    }
                    env[(size_t) f] = val;
                }
            }

            scheduledEnvelopes.emplace(gainId, std::move(env));
        }

        for (const auto &kv: scheduledPannerSnapshot) {
            const std::string &pannerId = kv.first;
            if (usedPanners.find(pannerId) == usedPanners.end()) continue;
            const auto &inner = kv.second;
            std::unordered_map<int, std::vector<double>> paramMap;
            for (const auto &pkv: inner) {
                int paramType = pkv.first;
                const auto &evts = pkv.second;
                std::vector<double> env((size_t) numFrames);

                double fallback = 0.0;
                auto pit = panners_.find(pannerId);
                if (pit != panners_.end()) {
                    const auto &pp = pit->second;
                    switch (paramType) {
                        case kPannerParamPositionX:
                            fallback = pp.positionX;
                            break;
                        case kPannerParamPositionY:
                            fallback = pp.positionY;
                            break;
                        case kPannerParamPositionZ:
                            fallback = pp.positionZ;
                            break;
                        case kPannerParamOrientationX:
                            fallback = pp.orientationX;
                            break;
                        case kPannerParamOrientationY:
                            fallback = pp.orientationY;
                            break;
                        case kPannerParamOrientationZ:
                            fallback = pp.orientationZ;
                            break;
                        case kPannerParamPan:
                            fallback = pp.pan;
                            break;
                        default:
                            fallback = 0.0;
                            break;
                    }
                }

                int rate = evts.empty() ? NativeEngine::RATE_A : evts[0].rate;
                if (rate == NativeEngine::RATE_K) {
                    float v = getScheduledGainValueAt(evts, static_cast<int64_t>(audioStartNs),
                                                      fallback);
                    for (int f = 0; f < numFrames; ++f) env[(size_t) f] = v;
                } else {
                    size_t nextIdx = 0;
                    int prevIdx = -1;
                    for (int f = 0; f < numFrames; ++f) {
                        auto tNs = static_cast<int64_t>(audioStartNs +
                                                        static_cast<double>(f) * sampleDurationNs);
                        while (nextIdx < evts.size() && evts[nextIdx].timeNs <= tNs) {
                            prevIdx = static_cast<int>(nextIdx);
                            ++nextIdx;
                        }
                        double val;
                        if (prevIdx == -1) {
                            val = fallback;
                        } else if (nextIdx >= evts.size()) {
                            val = evts[prevIdx].value;
                        } else {
                            const auto &prev = evts[prevIdx];
                            const auto &next = evts[nextIdx];
                            if (next.type == 1) {
                                if (next.timeNs <= prev.timeNs) val = next.value;
                                else {
                                    double ratio = double(tNs - prev.timeNs) /
                                                   double(next.timeNs - prev.timeNs);
                                    if (ratio < 0.0) ratio = 0.0;
                                    if (ratio > 1.0) ratio = 1.0;
                                    val = prev.value + (next.value - prev.value) * ratio;
                                }
                            } else {
                                val = prev.value;
                            }
                        }
                        env[(size_t) f] = val;
                    }
                }

                paramMap.emplace(paramType, std::move(env));
            }
            scheduledPannerEnvelopes.emplace(pannerId, std::move(paramMap));
        }

        for (const auto &ctxId: usedContexts) {
            auto it = scheduledListenerSnapshot.find(ctxId);
            if (it == scheduledListenerSnapshot.end()) continue;
            const auto &inner = it->second;
            std::unordered_map<int, std::vector<double>> paramMap;
            for (const auto &pkv: inner) {
                int paramType = pkv.first;
                const auto &evts = pkv.second;
                std::vector<double> env((size_t) numFrames);

                double fallback = 0.0;
                auto lit = listeners_.find(ctxId);
                const NativeEngine::Listener *LptrFallback = &listener_;
                if (lit != listeners_.end()) LptrFallback = &lit->second;
                const auto &Lfb = *LptrFallback;
                switch (paramType) {
                    case kListenerParamPositionX:
                        fallback = Lfb.positionX;
                        break;
                    case kListenerParamPositionY:
                        fallback = Lfb.positionY;
                        break;
                    case kListenerParamPositionZ:
                        fallback = Lfb.positionZ;
                        break;
                    case kListenerParamForwardX:
                        fallback = Lfb.forwardX;
                        break;
                    case kListenerParamForwardY:
                        fallback = Lfb.forwardY;
                        break;
                    case kListenerParamForwardZ:
                        fallback = Lfb.forwardZ;
                        break;
                    case kListenerParamUpX:
                        fallback = Lfb.upX;
                        break;
                    case kListenerParamUpY:
                        fallback = Lfb.upY;
                        break;
                    case kListenerParamUpZ:
                        fallback = Lfb.upZ;
                        break;
                    default:
                        fallback = 0.0;
                        break;
                }

                int rate = evts.empty() ? NativeEngine::RATE_A : evts[0].rate;
                if (rate == NativeEngine::RATE_K) {
                    float v = getScheduledGainValueAt(evts, static_cast<int64_t>(audioStartNs),
                                                      fallback);
                    for (int f = 0; f < numFrames; ++f) env[(size_t) f] = v;
                } else {
                    size_t nextIdx = 0;
                    int prevIdx = -1;
                    for (int f = 0; f < numFrames; ++f) {
                        auto tNs = static_cast<int64_t>(audioStartNs +
                                                        static_cast<double>(f) * sampleDurationNs);
                        while (nextIdx < evts.size() && evts[nextIdx].timeNs <= tNs) {
                            prevIdx = static_cast<int>(nextIdx);
                            ++nextIdx;
                        }
                        double val;
                        if (prevIdx == -1) {
                            val = fallback;
                        } else if (nextIdx >= evts.size()) {
                            val = evts[prevIdx].value;
                        } else {
                            const auto &prev = evts[prevIdx];
                            const auto &next = evts[nextIdx];
                            if (next.type == 1) {
                                if (next.timeNs <= prev.timeNs) val = next.value;
                                else {
                                    double ratio = double(tNs - prev.timeNs) /
                                                   double(next.timeNs - prev.timeNs);
                                    if (ratio < 0.0) ratio = 0.0;
                                    if (ratio > 1.0) ratio = 1.0;
                                    val = prev.value + (next.value - prev.value) * ratio;
                                }
                            } else {
                                val = prev.value;
                            }
                        }
                        env[(size_t) f] = val;
                    }
                }

                paramMap.emplace(paramType, std::move(env));
            }
            scheduledListenerEnvelopes.emplace(ctxId, std::move(paramMap));
        }
    }

    auto resolveGainValueForFrame = [&](const std::string &gainId,
                                        double fallback,
                                        int frameIndex) -> double {
        if (gainId.empty()) return fallback;
        auto eit = scheduledEnvelopes.find(gainId);
        if (eit != scheduledEnvelopes.end()) {
            const auto &env = eit->second;
            if (!env.empty()) {
                size_t idx = static_cast<size_t>(std::max(0, frameIndex));
                if (idx >= env.size()) idx = env.size() - 1;
                return env[idx];
            }
        }
        auto git = gains_.find(gainId);
        if (git != gains_.end()) return git->second;
        return fallback;
    };

    auto applyDynamicsCompressorForGain = [&](const std::string &gainId,
                                               int channelIndex,
                                               int frameIndex,
                                               float inputSample) -> float {
        if (gainId.empty()) return inputSample;

        auto cit = compressors_.find(gainId);
        if (cit == compressors_.end()) return inputSample;

        auto &compressor = cit->second;
        size_t safeChannel = static_cast<size_t>(std::max(0, channelIndex));
        size_t requiredChannels = static_cast<size_t>(std::max(1, channels));
        if (compressor.envelopeByChannel.size() < requiredChannels) {
            compressor.envelopeByChannel.resize(requiredChannels, 0.0f);
        }
        if (compressor.reductionDbByChannel.size() < requiredChannels) {
            compressor.reductionDbByChannel.resize(requiredChannels, 0.0f);
        }
        if (safeChannel >= requiredChannels) safeChannel = requiredChannels - 1;

        double thresholdDb = resolveGainValueForFrame(compressor.thresholdId, -24.0,
                                                      frameIndex);
        double kneeDb = resolveGainValueForFrame(compressor.kneeId, 30.0, frameIndex);
        double ratio = resolveGainValueForFrame(compressor.ratioId, 12.0, frameIndex);
        double attackSec = resolveGainValueForFrame(compressor.attackId, 0.003,
                                                    frameIndex);
        double releaseSec = resolveGainValueForFrame(compressor.releaseId, 0.25,
                                                     frameIndex);

        if (ratio < 1.0) ratio = 1.0;
        if (kneeDb < 0.0) kneeDb = 0.0;

        double sr = streamSampleRate_ > 0 ? static_cast<double>(streamSampleRate_) : 48000.0;
        if (attackSec <= 0.0) attackSec = 1.0 / sr;
        if (releaseSec <= 0.0) releaseSec = 1.0 / sr;

        double absIn = std::max(1.0e-12, std::fabs(static_cast<double>(inputSample)));
        double levelDb = 20.0 * std::log10(absIn);
        double overThresholdDb = levelDb - thresholdDb;

        double compressedLevelDb = levelDb;
        if (kneeDb > 0.0) {
            double halfKnee = 0.5 * kneeDb;
            if (overThresholdDb <= -halfKnee) {
                compressedLevelDb = levelDb;
            } else if (overThresholdDb >= halfKnee) {
                compressedLevelDb = thresholdDb + overThresholdDb / ratio;
            } else {
                double t = overThresholdDb + halfKnee;
                compressedLevelDb = levelDb +
                                    (1.0 / ratio - 1.0) * (t * t) / (2.0 * kneeDb);
            }
        } else if (overThresholdDb > 0.0) {
            compressedLevelDb = thresholdDb + overThresholdDb / ratio;
        }

        double targetReductionDb = std::max(0.0, levelDb - compressedLevelDb);
        double currentReductionDb = static_cast<double>(
                compressor.reductionDbByChannel[safeChannel]);
        double attackCoeff = std::exp(-1.0 / (attackSec * sr));
        double releaseCoeff = std::exp(-1.0 / (releaseSec * sr));
        if (targetReductionDb > currentReductionDb) {
            currentReductionDb = attackCoeff * currentReductionDb +
                                 (1.0 - attackCoeff) * targetReductionDb;
        } else {
            currentReductionDb = releaseCoeff * currentReductionDb +
                                 (1.0 - releaseCoeff) * targetReductionDb;
        }

        compressor.envelopeByChannel[safeChannel] = static_cast<float>(absIn);
        compressor.reductionDbByChannel[safeChannel] =
                static_cast<float>(currentReductionDb);

        if (!compressor.reductionId.empty()) {
            double maxReductionDb = 0.0;
            for (float value: compressor.reductionDbByChannel) {
                if (value > maxReductionDb) maxReductionDb = value;
            }
            gains_[compressor.reductionId] = -maxReductionDb;
        }

        double gainLinear = std::pow(10.0, -currentReductionDb / 20.0);
        return static_cast<float>(static_cast<double>(inputSample) * gainLinear);
    };

    auto resolveOutputMappedId = [](const std::unordered_map<int, std::string> *route,
                                    int outputIndex,
                                    const std::string &fallback) -> const std::string & {
        if (!route) return fallback;
        auto it = route->find(outputIndex);
        return it != route->end() ? it->second : fallback;
    };

    struct FilterRef {
        const NativeEngine::BiquadCoeffs *biquad = nullptr;
        const NativeEngine::IIRData *iir = nullptr;
    };

    std::unordered_map<std::string, FilterRef> filterRefCache;
    filterRefCache.reserve(localVoices.size() * 2);
    auto resolveFilterRef = [&](const std::string &filterId) -> FilterRef {
        if (filterId.empty()) return {};
        auto fit = filterRefCache.find(filterId);
        if (fit != filterRefCache.end()) return fit->second;

        FilterRef ref;
        auto bcit = biquads_.find(filterId);
        if (bcit != biquads_.end()) {
            ref.biquad = &bcit->second;
        } else {
            auto iit = iirs_.find(filterId);
            if (iit != iirs_.end()) ref.iir = &iit->second;
        }
        filterRefCache.emplace(filterId, ref);
        return ref;
    };

    std::unordered_map<std::string, double> pannerCutoffByBiquad;
    pannerCutoffByBiquad.reserve(localVoices.size());

    struct PannerFrameData {
        bool hasPanner = false;
        bool hrtfModel = false;
        HRTFConvolver *hrtfConvolver = nullptr;
        double leftGain = 1.0;
        double rightGain = 1.0;
        double distanceAtt = 1.0;
        bool steerLeftOrCenter = true;
        double leftGainStereo = 1.0;
        double rightGainStereo = 1.0;
        double distanceAttStereo = 1.0;
        bool steerLeftOrCenterStereo = true;
        std::string biquadId;
        bool cutoffEvaluated = false;
    };

    std::unordered_map<const NativeEngine::Panner *, PannerFrameData> pannerFrameCache;
    pannerFrameCache.reserve(std::max(localVoices.size(), usedPanners.size()));
    auto resolvePannerFrameData = [&](const std::string &pannerId,
                                      const NativeEngine::Panner *pannerNode,
                                      int frameIndex,
                                      bool evaluateCutoff) -> const PannerFrameData * {
        if (pannerId.empty()) return nullptr;

        const NativeEngine::Panner *panner = pannerNode;
        if (!panner) {
            auto pit = panners_.find(pannerId);
            if (pit == panners_.end()) return nullptr;
            panner = &pit->second;
        }

        auto cacheIt = pannerFrameCache.find(panner);
        if (cacheIt == pannerFrameCache.end()) {
            PannerFrameData frameData;
            frameData.hasPanner = true;
            frameData.hrtfConvolver = panner->hrtf;
            frameData.biquadId = panner->biquadId;

            NativeEngine::Panner resolvedPanner = resolvePannerForFrame(
                    *panner, scheduledPannerEnvelopes, pannerId, frameIndex);
            NativeEngine::Listener resolvedListener = resolveListenerForFrame(
                    panner->contextId, listener_, listeners_,
                    scheduledListenerEnvelopes, frameIndex);
            frameData.hrtfModel = resolvedPanner.panningModel == NativeEngine::PANNING_HRTF;

            computePannerGains(resolvedPanner, resolvedListener, false,
                               frameData.leftGain, frameData.rightGain,
                               frameData.distanceAtt, &frameData.steerLeftOrCenter);
            computePannerGains(resolvedPanner, resolvedListener, true,
                               frameData.leftGainStereo, frameData.rightGainStereo,
                               frameData.distanceAttStereo,
                               &frameData.steerLeftOrCenterStereo);

            cacheIt = pannerFrameCache.emplace(panner, std::move(frameData)).first;
        }

        if (evaluateCutoff && !cacheIt->second.cutoffEvaluated &&
            !cacheIt->second.biquadId.empty()) {
            double lowFreq = 800.0;
            double highFreq = 22050.0;
            double g = cacheIt->second.distanceAtt;
            if (g < 0.0) g = 0.0;
            if (g > 1.0) g = 1.0;
            double cutoff = lowFreq + (highFreq - lowFreq) * std::pow(g, 0.5);

            auto cacheCutoffIt = pannerCutoffByBiquad.find(cacheIt->second.biquadId);
            const bool shouldUpdate =
                    cacheCutoffIt == pannerCutoffByBiquad.end() ||
                    std::fabs(cacheCutoffIt->second - cutoff) >= 1.0;
            if (shouldUpdate) {
                auto bit = biquads_.find(cacheIt->second.biquadId);
                if (bit != biquads_.end()) {
                    bit->second = computeBiquadCoeffs(
                            "lowpass",
                            cutoff,
                            0.707,
                            0.0,
                            streamSampleRate_ > 0 ? streamSampleRate_ : 48000);
                    pannerCutoffByBiquad[cacheIt->second.biquadId] = cutoff;
                    audioThreadLog(ANDROID_LOG_INFO,
                                   "PANNER_CUTOFF panner=%s gain=%f cutoff=%f",
                                   pannerId.c_str(), g, cutoff);
                }
            }
            cacheIt->second.cutoffEvaluated = true;
        }

        return &cacheIt->second;
    };

    std::unordered_map<std::string, std::array<float, 2>> stereoInputCache;
    std::unordered_map<std::string, std::array<float, 2>> hrtfOutputCache;
    stereoInputCache.reserve(localVoices.size());
    hrtfOutputCache.reserve(localVoices.size());

    for (int frame = 0; frame < numFrames; ++frame) {
        pannerCutoffByBiquad.clear();
        pannerFrameCache.clear();
        stereoInputCache.clear();
        hrtfOutputCache.clear();
        for (int ch = 0; ch < channels; ++ch) {
            float mixed = 0.0f;
            for (const auto &voiceView: localVoices) {
                Voice &v = *voiceView.voice;
                const BufferData *voiceBuffer = voiceView.bufferData;
                const auto *gainRoute = voiceView.gainRoute;
                const auto *filterRoute = voiceView.filterRoute;
                const auto *pannerRoute = voiceView.pannerRoute;
                const std::string *playbackRatePrimaryId = voiceView.playbackRatePrimaryId;
                if (!v.playing) continue;
                if (v.voiceType == Voice::Oscillator) {
                    double s = 0.0;
                    if (!v.periodicWaveId.empty()) {
                        auto pwIt = periodicWaves_.find(v.periodicWaveId);
                        if (pwIt != periodicWaves_.end()) {
                            double sr =
                                    streamSampleRate_ > 0 ? static_cast<double>(streamSampleRate_)
                                                          : 48000.0;
                            s = computePeriodicWaveSample(pwIt->second, v.phase, v.frequency, sr);
                        } else {
                            if (v.waveform == "sine") {
                                s = std::sin(v.phase * 2.0 * M_PI);
                            } else if (v.waveform == "square") {
                                s = (v.phase < 0.5) ? 1.0 : -1.0;
                            } else if (v.waveform == "sawtooth") {
                                s = 2.0 * (v.phase) - 1.0;
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
                            s = 2.0 * (v.phase) - 1.0;
                        } else if (v.waveform == "triangle") {
                            double p = v.phase;
                            if (p < 0.25) s = 4.0 * p;
                            else if (p < 0.75) s = 2.0 - 4.0 * p;
                            else s = 4.0 * p - 4.0;
                        } else {
                            s = std::sin(v.phase * 2.0 * M_PI);
                        }
                    }

                        const std::string &effFilterId =
                            (ch < kRouteCacheChannels && voiceView.filterIdByOutput[ch])
                            ? *voiceView.filterIdByOutput[ch]
                            : resolveOutputMappedId(filterRoute, ch, v.filterId);
                    if (!effFilterId.empty()) {
                        FilterRef filterRef = resolveFilterRef(effFilterId);
                        if (filterRef.biquad) {
                            int fsIndex = ch;
                            if (v.filterState.size() <= static_cast<size_t>(fsIndex))
                                v.filterState.assign(channels, NativeEngine::BiquadState());
                            auto in = static_cast<float>(s);
                            float outS = processBiquadSample(*filterRef.biquad,
                                                             v.filterState[fsIndex],
                                                             in);
                            s = outS;
                        } else if (filterRef.iir) {
                            int fsIndex = ch;
                            size_t chCount = static_cast<size_t>(channels);
                            if (v.iirState.size() < chCount)
                                v.iirState.assign(chCount, std::vector<double>());
                            if (v.iirState.size() <= static_cast<size_t>(fsIndex))
                                v.iirState.assign(chCount, std::vector<double>());
                            float outS = processIIRSample(*filterRef.iir,
                                                          v.iirState[static_cast<size_t>(fsIndex)],
                                                          static_cast<float>(s));
                            s = outS;
                        }
                    }

                    double leftGain = 1.0, rightGain = 1.0, distanceAtt = 1.0;

                        const std::string &effPanId =
                            (ch < kRouteCacheChannels && voiceView.pannerIdByOutput[ch])
                            ? *voiceView.pannerIdByOutput[ch]
                            : resolveOutputMappedId(pannerRoute, ch, v.panId);
                        const NativeEngine::Panner *effPanNode =
                            ch < kRouteCacheChannels ? voiceView.pannerNodeByOutput[ch] : nullptr;
                    const PannerFrameData *pannerFrame = resolvePannerFrameData(effPanId,
                                                    effPanNode,
                                                                                frame,
                                                                                true);
                    if (pannerFrame && pannerFrame->hasPanner) {
                        leftGain = pannerFrame->leftGain;
                        rightGain = pannerFrame->rightGain;
                        distanceAtt = pannerFrame->distanceAtt;
                    }

                        const std::string &effGainId =
                            (ch < kRouteCacheChannels && voiceView.gainIdByOutput[ch])
                            ? *voiceView.gainIdByOutput[ch]
                            : resolveOutputMappedId(gainRoute, ch, v.gainId);
                    double scheduled = resolveGainValueForFrame(effGainId, v.gain, frame);

                    if (channels <= 1) {
                        if (!effGainId.empty()) {
                            auto wsIt = waveShapers_.find(effGainId);
                            if (wsIt != waveShapers_.end() && !wsIt->second.curve.empty()) {
                                s = static_cast<double>(applyWaveShaperSample(wsIt->second,
                                                                              static_cast<float>(s)));
                            }
                        }
                        double monoGain = 0.5 * (leftGain + rightGain) * distanceAtt *
                                          static_cast<double>(scheduled);
                        float voiceOut = static_cast<float>(s * monoGain);
                        voiceOut = applyDynamicsCompressorForGain(effGainId, ch, frame,
                                                                   voiceOut);
                        mixed += voiceOut;
                    } else {
                        double g = static_cast<double>(scheduled) * distanceAtt;
                        if (ch == 0) {
                            float voiceOut = static_cast<float>(s * g * leftGain);
                            voiceOut = applyDynamicsCompressorForGain(effGainId, ch, frame,
                                                                       voiceOut);
                            mixed += voiceOut;
                        } else if (ch == 1) {
                            float voiceOut = static_cast<float>(s * g * rightGain);
                            voiceOut = applyDynamicsCompressorForGain(effGainId, ch, frame,
                                                                       voiceOut);
                            mixed += voiceOut;
                        } else {
                            double avg = 0.5 * (leftGain + rightGain);
                            float voiceOut = static_cast<float>(s * g * avg);
                            voiceOut = applyDynamicsCompressorForGain(effGainId, ch, frame,
                                                                       voiceOut);
                            mixed += voiceOut;
                        }
                    }
                    if (ch == channels - 1) {
                        v.phase += v.phaseIncrement;
                        if (v.phase >= 1.0) v.phase -= 1.0;
                    }
                } else if (v.voiceType == Voice::BufferSource ||
                           v.voiceType == Voice::ExternalPCM) {
                    int bufChannels = 1;
                    int bufFrames = 0;
                    int chIdx = ch;
                    float sample = 0.0f;
                    int i0_dbg = 0;
                    int idx0_dbg = 0;
                    int idx1_dbg = 0;
                    const BufferData *bd_dbg = nullptr;

                    if (v.voiceType == Voice::BufferSource) {
                        if (!voiceBuffer) continue;
                        const BufferData &bd = *voiceBuffer;
                        bd_dbg = &bd;
                        bufChannels = bd.channels > 0 ? bd.channels : 1;
                        if (bd.isPlanar && bd.planarFrames > 0) {
                            bufFrames = bd.planarFrames;
                        } else if (bd.isDirect) {
                            bufFrames = static_cast<int>((bd.byteLength / bd.bytesPerSample) /
                                                         (bufChannels ? bufChannels : 1));
                        } else {
                            bufFrames = static_cast<int>(bd.pcm.size() /
                                                         (bufChannels ? bufChannels : 1));
                        }
                        if (bufFrames <= 0) continue;

                        double pos = v.position;
                        int i0 = static_cast<int>(std::floor(pos));
                        i0_dbg = i0;
                        double frac = pos - i0;

                        if (bufChannels == 1) chIdx = 0;
                        else if (bufChannels >= channels) chIdx = ch;
                        else chIdx = ch % bufChannels;

                        int idx0 = (i0 % bufFrames) * bufChannels + chIdx;
                        int idx1 = ((i0 + 1) % bufFrames) * bufChannels + chIdx;
                        idx0_dbg = idx0;
                        idx1_dbg = idx1;
                        float fs0 = 0.0f;
                        float fs1 = 0.0f;
                        if (bd.isPlanar && !bd.planarPtrs.empty() && bd.planarFrames > 0) {
                            int planarChannel = std::max(0, std::min(
                                    static_cast<int>(bd.planarPtrs.size()) - 1,
                                    chIdx));
                            const float *fptr = bd.planarPtrs[static_cast<size_t>(planarChannel)];
                            if (fptr) {
                                int maxIndex = bd.planarFrames - 1;
                                int sample0 = i0 % bufFrames;
                                int sample1 = (i0 + 1) % bufFrames;
                                int safe0 = std::max(0, std::min(maxIndex, sample0));
                                int safe1 = std::max(0, std::min(maxIndex, sample1));
                                fs0 = fptr[safe0];
                                fs1 = fptr[safe1];
                            }
                        } else if (bd.isDirect && bd.directPtr) {
                            if (bd.bytesPerSample == 4) {
                                const auto *fptr = reinterpret_cast<const float *>(bd.directPtr);
                                int maxIndex = static_cast<int>((bd.byteLength / 4) - 1);
                                int safe0 = std::max(0, std::min(maxIndex, idx0));
                                int safe1 = std::max(0, std::min(maxIndex, idx1));
                                fs0 = fptr[safe0];
                                fs1 = fptr[safe1];
                            } else {
                                const auto *iptr = reinterpret_cast<const int16_t *>(bd.directPtr);
                                int maxIndex = static_cast<int>((bd.byteLength / 2) - 1);
                                int safe0 = std::max(0, std::min(maxIndex, idx0));
                                int safe1 = std::max(0, std::min(maxIndex, idx1));
                                fs0 = static_cast<float>(iptr[safe0]) / 32768.0f;
                                fs1 = static_cast<float>(iptr[safe1]) / 32768.0f;
                            }
                        } else {
                            int maxIndex = static_cast<int>(bd.pcm.size()) - 1;
                            int safe0 = std::max(0, std::min(maxIndex, idx0));
                            int safe1 = std::max(0, std::min(maxIndex, idx1));
                            fs0 = static_cast<float>(bd.pcm[safe0]) / 32768.0f;
                            fs1 = static_cast<float>(bd.pcm[safe1]) / 32768.0f;
                        }
                        sample = fs0 * (1.0f - static_cast<float>(frac)) +
                                 fs1 * static_cast<float>(frac);
                    } else /* ExternalPCM */ {
                        auto &ring = v.externalRing;
                        if (!ring) continue;
                        int externalChannels = resolveExternalChannelCount(v, ring);
                        if (externalChannels <= 0) {
                            bufChannels = 0;
                            sample = 0.0f;
                            v.externalPrimed = false;
                            if (ring->ended.load(std::memory_order_acquire) &&
                                ring->writeIdx.load(std::memory_order_acquire) ==
                                ring->readIdx.load(std::memory_order_relaxed)) {
                                v.playing = false;
                            }
                        } else {
                            bufChannels = externalChannels;
                            bufFrames = INT32_MAX;
                            if (bufChannels == 1) chIdx = 0;
                            else if (bufChannels >= channels) chIdx = ch;
                            else chIdx = ch % bufChannels;

                            uint32_t w = ring->writeIdx.load(std::memory_order_acquire);
                            uint32_t r = ring->readIdx.load(std::memory_order_relaxed);
                            if (!v.externalPrimed && (w - r) >= 1) {
                                uint32_t p = r & ring->mask;
                                for (int c = 0; c < bufChannels; c++) {
                                    v.externalCurr[(size_t) c] =
                                            ring->data[(size_t) p * bufChannels + c];
                                }
                                ring->readIdx.store(r + 1, std::memory_order_release);
                                v.externalPrimed = true;
                            }

                            if (v.externalPrimed) {
                                size_t safeChannel = static_cast<size_t>(chIdx);
                                float prev = v.externalPrev[safeChannel];
                                float curr = v.externalCurr[safeChannel];
                                auto t = static_cast<float>(v.externalSubPos);
                                sample = prev * (1.0f - t) + curr * t;
                            } else {
                                sample = 0.0f;
                                if (ring->ended.load(std::memory_order_acquire) &&
                                    ring->writeIdx.load(std::memory_order_acquire) ==
                                    ring->readIdx.load(std::memory_order_relaxed)) {
                                    v.playing = false;
                                }
                            }
                        }
                    }

                    auto applyVoiceFilter = [&](float inputSample,
                                                int outputChannelIndex) -> float {
                        const std::string &filterId =
                            (outputChannelIndex >= 0 &&
                             outputChannelIndex < kRouteCacheChannels &&
                             voiceView.filterIdByOutput[outputChannelIndex])
                            ? *voiceView.filterIdByOutput[outputChannelIndex]
                            : resolveOutputMappedId(filterRoute,
                                        outputChannelIndex,
                                        v.filterId);
                        if (filterId.empty()) return inputSample;

                        FilterRef filterRef = resolveFilterRef(filterId);
                        if (filterRef.biquad) {
                            int fsIndex = outputChannelIndex;
                            if (v.filterState.size() <= static_cast<size_t>(fsIndex)) {
                                v.filterState.assign(channels, NativeEngine::BiquadState());
                            }
                            return processBiquadSample(*filterRef.biquad,
                                                       v.filterState[static_cast<size_t>(fsIndex)],
                                                       inputSample);
                        }

                        if (filterRef.iir) {
                            int fsIndex = outputChannelIndex;
                            auto chCount = static_cast<size_t>(channels);
                            if (v.iirState.size() < chCount) {
                                v.iirState.assign(chCount, std::vector<double>());
                            }
                            if (v.iirState.size() <= static_cast<size_t>(fsIndex)) {
                                v.iirState.assign(chCount, std::vector<double>());
                            }
                            return processIIRSample(*filterRef.iir,
                                                    v.iirState[static_cast<size_t>(fsIndex)],
                                                    inputSample);
                        }

                        return inputSample;
                    };
                    auto sampleInputChannel = [&](int inputChannelIndex) -> float {
                        if (bufChannels <= 1) inputChannelIndex = 0;
                        else if (inputChannelIndex < 0) inputChannelIndex = 0;
                        else if (inputChannelIndex >= bufChannels)
                            inputChannelIndex = bufChannels - 1;

                        if (v.voiceType == Voice::BufferSource) {
                            if (!bd_dbg || bufFrames <= 0) return 0.0f;
                            const BufferData &bd = *bd_dbg;
                            double pos = v.position;
                            int i0 = static_cast<int>(std::floor(pos));
                            double frac = pos - i0;
                            int idx0 = (i0 % bufFrames) * bufChannels + inputChannelIndex;
                            int idx1 = ((i0 + 1) % bufFrames) * bufChannels + inputChannelIndex;
                            float fs0 = 0.0f;
                            float fs1 = 0.0f;
                            if (bd.isPlanar && !bd.planarPtrs.empty() && bd.planarFrames > 0) {
                                int planarChannel = std::max(0, std::min(
                                        static_cast<int>(bd.planarPtrs.size()) - 1,
                                        inputChannelIndex));
                                const float *fptr =
                                        bd.planarPtrs[static_cast<size_t>(planarChannel)];
                                if (fptr) {
                                    int maxIndex = bd.planarFrames - 1;
                                    int sample0 = i0 % bufFrames;
                                    int sample1 = (i0 + 1) % bufFrames;
                                    int safe0 = std::max(0, std::min(maxIndex, sample0));
                                    int safe1 = std::max(0, std::min(maxIndex, sample1));
                                    fs0 = fptr[safe0];
                                    fs1 = fptr[safe1];
                                }
                            } else if (bd.isDirect && bd.directPtr) {
                                if (bd.bytesPerSample == 4) {
                                    const auto *fptr = reinterpret_cast<const float *>(bd.directPtr);
                                    int maxIndex = static_cast<int>((bd.byteLength / 4) - 1);
                                    int safe0 = std::max(0, std::min(maxIndex, idx0));
                                    int safe1 = std::max(0, std::min(maxIndex, idx1));
                                    fs0 = fptr[safe0];
                                    fs1 = fptr[safe1];
                                } else {
                                    const auto *iptr = reinterpret_cast<const int16_t *>(bd.directPtr);
                                    int maxIndex = static_cast<int>((bd.byteLength / 2) - 1);
                                    int safe0 = std::max(0, std::min(maxIndex, idx0));
                                    int safe1 = std::max(0, std::min(maxIndex, idx1));
                                    fs0 = static_cast<float>(iptr[safe0]) / 32768.0f;
                                    fs1 = static_cast<float>(iptr[safe1]) / 32768.0f;
                                }
                            } else {
                                int maxIndex = static_cast<int>(bd.pcm.size()) - 1;
                                int safe0 = std::max(0, std::min(maxIndex, idx0));
                                int safe1 = std::max(0, std::min(maxIndex, idx1));
                                fs0 = static_cast<float>(bd.pcm[safe0]) / 32768.0f;
                                fs1 = static_cast<float>(bd.pcm[safe1]) / 32768.0f;
                            }
                            return fs0 * (1.0f - static_cast<float>(frac)) +
                                   fs1 * static_cast<float>(frac);
                        }

                        auto &ring = v.externalRing;
                        if (!ring || !v.externalPrimed || bufChannels <= 0) return 0.0f;
                        size_t safeChannel = static_cast<size_t>(inputChannelIndex);
                        float prev = v.externalPrev[safeChannel];
                        float curr = v.externalCurr[safeChannel];
                        auto t = static_cast<float>(v.externalSubPos);
                        return prev * (1.0f - t) + curr * t;
                    };

                    bool stereoInput = bufChannels > 1 && channels > 1;
                    float inputLeft = sample;
                    float inputRight = sample;
                    bool hrtfModel = false;
                    HRTFConvolver *hrtfConvolver = nullptr;
                    if (stereoInput && ch < 2) {
                        auto cacheIt = stereoInputCache.find(v.id);
                        if (cacheIt == stereoInputCache.end()) {
                            std::array<float, 2> cached{{
                                    applyVoiceFilter(sampleInputChannel(0), 0),
                                    applyVoiceFilter(sampleInputChannel(1), 1),
                            }};
                            cacheIt = stereoInputCache.emplace(v.id, cached).first;
                        }
                        inputLeft = cacheIt->second[0];
                        inputRight = cacheIt->second[1];
                        sample = ch == 0 ? inputLeft : inputRight;
                    } else {
                        sample = applyVoiceFilter(sample, chIdx);
                    }

                    double leftGain = 1.0, rightGain = 1.0, distanceAtt = 1.0;
                    bool steerLeftOrCenter = true;
                    bool hasResolvedPanner = false;
                        const std::string &effPanId2 =
                            (ch < kRouteCacheChannels && voiceView.pannerIdByOutput[ch])
                            ? *voiceView.pannerIdByOutput[ch]
                            : resolveOutputMappedId(pannerRoute, ch, v.panId);
                        const NativeEngine::Panner *effPanNode2 =
                            ch < kRouteCacheChannels ? voiceView.pannerNodeByOutput[ch] : nullptr;
                    const PannerFrameData *pannerFrame = resolvePannerFrameData(effPanId2,
                                                    effPanNode2,
                                                                                frame,
                                                                                false);
                    if (pannerFrame && pannerFrame->hasPanner) {
                        hasResolvedPanner = true;
                        hrtfModel = pannerFrame->hrtfModel;
                        hrtfConvolver = pannerFrame->hrtfConvolver;
                        bool useStereoPanningLaw =
                                stereoInput && (!hrtfModel || hrtfConvolver == nullptr);
                        if (useStereoPanningLaw) {
                            leftGain = pannerFrame->leftGainStereo;
                            rightGain = pannerFrame->rightGainStereo;
                            distanceAtt = pannerFrame->distanceAttStereo;
                            steerLeftOrCenter = pannerFrame->steerLeftOrCenterStereo;
                        } else {
                            leftGain = pannerFrame->leftGain;
                            rightGain = pannerFrame->rightGain;
                            distanceAtt = pannerFrame->distanceAtt;
                            steerLeftOrCenter = pannerFrame->steerLeftOrCenter;
                        }
                    }

                    bool useHrtfConvolution =
                            hrtfModel && hrtfConvolver != nullptr && channels > 1;
                    bool treatStereoAsMono = stereoInput && useHrtfConvolution;
                    if (useHrtfConvolution && ch < 2) {
                        auto hrtfIt = hrtfOutputCache.find(v.id);
                        if (hrtfIt == hrtfOutputCache.end()) {
                            float monoInput = stereoInput
                                              ? 0.5f * (inputLeft + inputRight)
                                              : sample;
                            float monoFrame[1]{monoInput};
                            std::array<float, 2> hrtfFrame{{monoInput, monoInput}};
                            hrtfConvolver->process(monoFrame, 1, &hrtfFrame[0],
                                                   &hrtfFrame[1]);
                            hrtfIt = hrtfOutputCache.emplace(v.id, hrtfFrame).first;
                        }
                        sample = ch == 0 ? hrtfIt->second[0] : hrtfIt->second[1];
                    } else if (treatStereoAsMono) {
                        sample = 0.5f * (inputLeft + inputRight);
                    }

                    if (v.voiceType == Voice::BufferSource && bd_dbg && traceEnabled) {
                        if (debugLoggedVoices_.find(v.id) == debugLoggedVoices_.end()) {
                            debugLoggedVoices_.insert(v.id);
                            const BufferData &bd = *bd_dbg;
                            int i0 = i0_dbg;
                            double frac = v.position - i0;
                            int idx0 = idx0_dbg;
                            int idx1 = idx1_dbg;
                            int chOther = (channels > 1) ? 1 : 0;
                            int chIdxOther = chOther;
                            if (bufChannels == 1) chIdxOther = 0;
                            else if (bufChannels >= channels) chIdxOther = chOther;
                            else chIdxOther = chOther % bufChannels;
                            int idx0Other = (i0 % bufFrames) * bufChannels + chIdxOther;
                            int idx1Other = ((i0 + 1) % bufFrames) * bufChannels + chIdxOther;
                            float fs0Other = 0.0f, fs1Other = 0.0f, sampleOther = 0.0f;
                            if (bd.isPlanar && !bd.planarPtrs.empty() && bd.planarFrames > 0) {
                                int planarChannel = std::max(0, std::min(
                                        static_cast<int>(bd.planarPtrs.size()) - 1,
                                        chIdxOther));
                                const float *fptr =
                                        bd.planarPtrs[static_cast<size_t>(planarChannel)];
                                if (fptr) {
                                    int maxIndex = bd.planarFrames - 1;
                                    int sample0 = i0 % bufFrames;
                                    int sample1 = (i0 + 1) % bufFrames;
                                    int safe0 = std::max(0, std::min(maxIndex, sample0));
                                    int safe1 = std::max(0, std::min(maxIndex, sample1));
                                    fs0Other = fptr[safe0];
                                    fs1Other = fptr[safe1];
                                }
                            } else if (bd.isDirect && bd.directPtr) {
                                if (bd.bytesPerSample == 4) {
                                    const auto *fptr = reinterpret_cast<const float *>(bd.directPtr);
                                    int maxIndex = static_cast<int>((bd.byteLength / 4) - 1);
                                    int safe0 = std::max(0, std::min(maxIndex, idx0Other));
                                    int safe1 = std::max(0, std::min(maxIndex, idx1Other));
                                    fs0Other = fptr[safe0];
                                    fs1Other = fptr[safe1];
                                } else {
                                    const auto *iptr = reinterpret_cast<const int16_t *>(bd.directPtr);
                                    int maxIndex = static_cast<int>((bd.byteLength / 2) - 1);
                                    int safe0 = std::max(0, std::min(maxIndex, idx0Other));
                                    int safe1 = std::max(0, std::min(maxIndex, idx1Other));
                                    fs0Other = static_cast<float>(iptr[safe0]) / 32768.0f;
                                    fs1Other = static_cast<float>(iptr[safe1]) / 32768.0f;
                                }
                            } else {
                                int maxIndex = static_cast<int>(bd.pcm.size()) - 1;
                                int safe0 = std::max(0, std::min(maxIndex, idx0Other));
                                int safe1 = std::max(0, std::min(maxIndex, idx1Other));
                                fs0Other = static_cast<float>(bd.pcm[safe0]) / 32768.0f;
                                fs1Other = static_cast<float>(bd.pcm[safe1]) / 32768.0f;
                            }
                            sampleOther = fs0Other * (1.0f - static_cast<float>(frac)) +
                                          fs1Other * static_cast<float>(frac);
                            (void) sampleOther;
                            (void) idx0;
                            (void) idx1;
                            audioThreadLog(ANDROID_LOG_INFO,
                                           "VOICE_DEBUG id=%s buffer=%s bufCh=%d streamCh=%d ch0_chIdx=%d ch0_idx0=%d ch0_idx1=%d ch0_sample=%f ch1_chIdx=%d ch1_idx0=%d ch1_idx1=%d ch1_sample=%f leftGain=%f rightGain=%f panId=%s",
                                           v.id.c_str(), v.bufferId.c_str(), bufChannels, channels,
                                           chIdx, idx0, idx1, sample, chIdxOther, idx0Other,
                                           idx1Other, sampleOther, leftGain, rightGain,
                                           v.panId.c_str());
                        }
                    }

                        const std::string &effGainId3 =
                            (ch < kRouteCacheChannels && voiceView.gainIdByOutput[ch])
                            ? *voiceView.gainIdByOutput[ch]
                            : resolveOutputMappedId(gainRoute, ch, v.gainId);
                    double scheduled = resolveGainValueForFrame(effGainId3, v.gain, frame);

                    if (channels <= 1) {
                        if (!effGainId3.empty()) {
                            auto wsIt3 = waveShapers_.find(effGainId3);
                            if (wsIt3 != waveShapers_.end() && !wsIt3->second.curve.empty()) {
                                sample = static_cast<double>(applyWaveShaperSample(wsIt3->second,
                                                                                   static_cast<float>(sample)));
                            }
                        }
                        double monoGain = 0.5 * (leftGain + rightGain) * distanceAtt *
                                          static_cast<double>(scheduled);
                        float voiceOut = static_cast<float>(sample * monoGain);
                        voiceOut = applyDynamicsCompressorForGain(effGainId3, ch, frame,
                                                                   voiceOut);
                        mixed += voiceOut;
                    } else {
                        double g = static_cast<double>(scheduled) * distanceAtt;
                        if (stereoInput && !treatStereoAsMono) {
                            if (ch == 0) {
                                double outputL = inputLeft;
                                if (hasResolvedPanner) {
                                    outputL = steerLeftOrCenter
                                              ? inputLeft + inputRight * leftGain
                                              : inputLeft * leftGain;
                                }
                                float voiceOut = static_cast<float>(outputL * g);
                                voiceOut = applyDynamicsCompressorForGain(effGainId3, ch,
                                                                           frame, voiceOut);
                                mixed += voiceOut;
                            } else if (ch == 1) {
                                double outputR = inputRight;
                                if (hasResolvedPanner) {
                                    outputR = steerLeftOrCenter
                                              ? inputRight * rightGain
                                              : inputRight + inputLeft * rightGain;
                                }
                                float voiceOut = static_cast<float>(outputR * g);
                                voiceOut = applyDynamicsCompressorForGain(effGainId3, ch,
                                                                           frame, voiceOut);
                                mixed += voiceOut;
                            } else {
                                double avg = 0.5 * (leftGain + rightGain);
                                float voiceOut = static_cast<float>(sample * g * avg);
                                voiceOut = applyDynamicsCompressorForGain(effGainId3, ch,
                                                                           frame, voiceOut);
                                mixed += voiceOut;
                            }
                        } else if (useHrtfConvolution) {
                            if (ch <= 1) {
                                float voiceOut = static_cast<float>(sample * g);
                                voiceOut = applyDynamicsCompressorForGain(effGainId3, ch,
                                                                           frame, voiceOut);
                                mixed += voiceOut;
                            } else {
                                auto hrtfIt = hrtfOutputCache.find(v.id);
                                float hrtfAvg = sample;
                                if (hrtfIt != hrtfOutputCache.end()) {
                                    hrtfAvg =
                                            0.5f * (hrtfIt->second[0] + hrtfIt->second[1]);
                                }
                                float voiceOut = static_cast<float>(hrtfAvg * g);
                                voiceOut = applyDynamicsCompressorForGain(effGainId3, ch,
                                                                           frame, voiceOut);
                                mixed += voiceOut;
                            }
                        } else if (ch == 0) {
                            float voiceOut = static_cast<float>(sample * g * leftGain);
                            voiceOut = applyDynamicsCompressorForGain(effGainId3, ch, frame,
                                                                       voiceOut);
                            mixed += voiceOut;
                        } else if (ch == 1) {
                            float voiceOut = static_cast<float>(sample * g * rightGain);
                            voiceOut = applyDynamicsCompressorForGain(effGainId3, ch, frame,
                                                                       voiceOut);
                            mixed += voiceOut;
                        } else {
                            double avg = 0.5 * (leftGain + rightGain);
                            float voiceOut = static_cast<float>(sample * g * avg);
                            voiceOut = applyDynamicsCompressorForGain(effGainId3, ch, frame,
                                                                       voiceOut);
                            mixed += voiceOut;
                        }
                    }

                    if (ch == channels - 1) {
                        if (v.voiceType == Voice::BufferSource) {
                            double prScale = 1.0;
                            const std::string &resolvedPRId =
                                    playbackRatePrimaryId ? *playbackRatePrimaryId
                                                          : v.playbackRateId;
                            if (!resolvedPRId.empty()) {
                                prScale = resolveGainValueForFrame(resolvedPRId, prScale, frame);
                            }
                            if (prScale < 0.0) prScale = 0.0;
                            double incr = v.increment * prScale;
                            v.position += incr;
                            if (v.position >= bufFrames) {
                                if (v.loop) v.position = std::fmod(v.position, bufFrames);
                                else v.playing = false;
                            }
                        } else /* ExternalPCM */ {
                            auto &ring = v.externalRing;
                            if (ring && v.externalPrimed && bufChannels > 0) {
                                v.externalSubPos += v.increment;
                                while (v.externalSubPos >= 1.0) {
                                    for (int c = 0; c < bufChannels; c++) {
                                        v.externalPrev[(size_t) c] = v.externalCurr[(size_t) c];
                                    }
                                    uint32_t w = ring->writeIdx.load(std::memory_order_acquire);
                                    uint32_t r = ring->readIdx.load(std::memory_order_relaxed);
                                    if ((w - r) >= 1) {
                                        uint32_t p = r & ring->mask;
                                        for (int c = 0; c < bufChannels; c++) {
                                            v.externalCurr[(size_t) c] =
                                                    ring->data[(size_t) p * bufChannels + c];
                                        }
                                        ring->readIdx.store(r + 1, std::memory_order_release);
                                        v.externalSubPos -= 1.0;
                                    } else {
                                        v.externalSubPos = 0.999;
                                        if (ring->ended.load(std::memory_order_acquire)) {
                                            v.playing = false;
                                        }
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }


            if (mixed > 1.0f) mixed = 1.0f;
            if (mixed < -1.0f) mixed = -1.0f;
            out[frame * channels + ch] = mixed;
        }

        float mono = 0.0f;
        for (int c = 0; c < channels; ++c) mono += out[frame * channels + c];
        mono = mono / static_cast<float>(channels > 0 ? channels : 1);
        for (auto &av: analysers_) {
            AnalyserData &ad = av.second;
            int64_t idx = ad.writeIdx.fetch_add(1, std::memory_order_relaxed);
            if (ad.capacity > 0) ad.ring[(size_t) (idx % ad.capacity)] = mono;
        }
    }
    std::vector<std::string> stoppedVoices;
    stoppedVoices.reserve(localVoices.size());
    for (const auto &voiceView: localVoices) {
        const Voice &voice = *voiceView.voice;
        if (voice.playing) continue;

        if (traceEnabled) {
            auto vg = voiceGainByOutput_.find(voice.id);
            auto vp = voicePannerByOutput_.find(voice.id);
            auto vf = voiceFilterByOutput_.find(voice.id);
            size_t gainOutputs = vg != voiceGainByOutput_.end() ? vg->second.size() : 0;
            size_t panOutputs = vp != voicePannerByOutput_.end() ? vp->second.size() : 0;
            size_t filterOutputs = vf != voiceFilterByOutput_.end()
                                   ? vf->second.size()
                                   : 0;

            if (voice.voiceType == Voice::ExternalPCM) {
                const unsigned long long seq =
                        sGraphTraceSeq.fetch_add(1, std::memory_order_relaxed) + 1;
                logRingState(seq, "VOICE_AUTO_STOP", voice.id, voice.externalRing,
                             voice.bufferChannels, voice.bufferSampleRate, voice.playing);
                audioThreadLog(ANDROID_LOG_INFO,
                               "GRAPH_TRACE seq=%llu VOICE_AUTO_STOP voice=%s type=%s gainId=%s panId=%s filterId=%s gainOutputs=%zu panOutputs=%zu filterOutputs=%zu",
                               seq, voice.id.c_str(), voiceTypeName(voice.voiceType),
                               voice.gainId.c_str(), voice.panId.c_str(), voice.filterId.c_str(),
                               gainOutputs, panOutputs, filterOutputs);
            } else {
                audioThreadLog(ANDROID_LOG_INFO,
                               "GRAPH_TRACE VOICE_AUTO_STOP voice=%s type=%s gainId=%s panId=%s filterId=%s gainOutputs=%zu panOutputs=%zu filterOutputs=%zu",
                               voice.id.c_str(), voiceTypeName(voice.voiceType),
                               voice.gainId.c_str(), voice.panId.c_str(), voice.filterId.c_str(),
                               gainOutputs, panOutputs, filterOutputs);
            }
        }

        stoppedVoices.push_back(voice.id);
    }

    for (const auto &voiceId: stoppedVoices) {
        audioVoices_.erase(voiceId);
    }

    return oboe::DataCallbackResult::Continue;
}

#endif
