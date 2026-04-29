#include "audio_context.h"
#include "audio_internal.h"

#include <android/log.h>
#include <jni.h>
#include <string>
#include <vector>
#include <cstring>
#include <cmath>
#include <algorithm>
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

void NativeEngine::ensureStream() {
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    if (stream_) return;

    oboe::AudioStreamBuilder builder;
    oboe::PerformanceMode perf = oboe::PerformanceMode::LowLatency;
    if (desiredLatencyHintSec_ > 0.020) perf = oboe::PerformanceMode::PowerSaving;
    else if (desiredLatencyHintSec_ > 0.010) perf = oboe::PerformanceMode::None;
    builder.setPerformanceMode(perf);
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

    {

        if (stream) {
            auto tsRes = stream->getTimestamp(CLOCK_MONOTONIC);
            if (tsRes) {
                auto frameTs = tsRes.value();
                g_audioTimeNanos.store(frameTs.timestamp, std::memory_order_relaxed);
            } else {
                int64_t nowNs = std::chrono::duration_cast<std::chrono::nanoseconds>(
                        std::chrono::steady_clock::now().time_since_epoch()).count();
                g_audioTimeNanos.store(nowNs, std::memory_order_relaxed);
            }
        } else {
            int64_t nowNs = std::chrono::duration_cast<std::chrono::nanoseconds>(
                    std::chrono::steady_clock::now().time_since_epoch()).count();
            g_audioTimeNanos.store(nowNs, std::memory_order_relaxed);
        }
    }


    std::vector<NativeEngine::Command> cmds;
    {
        std::lock_guard<std::mutex> cl(commandMutex_);
        cmds.swap(pendingCommands_);
    }
    for (auto &c: cmds) {
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
                v.type = Voice::Oscillator;
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
            case NativeEngine::CMD_SET_GAIN: {
                double v = c.gainValue;
                gains_[c.id] = v;
                for (auto &kv: audioVoices_) {
                    if (kv.second.gainId == c.id) {
                        kv.second.gain = v;
                    }
                }

                for (const auto &vgKv : voiceGainByOutput_) {
                    const std::string &voiceId = vgKv.first;
                    for (const auto &entry : vgKv.second) {
                        if (entry.second == c.id) {
                            auto vit2 = audioVoices_.find(voiceId);
                            if (vit2 != audioVoices_.end()) vit2->second.gain = v;
                        }
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
                                                    voiceId.c_str(), entry.first, entry.second.c_str(), voice.gain);
                            }
                        }
                    }
                }
                break;
            }
            case NativeEngine::CMD_ATTACH_GAIN: {
                auto vit = audioVoices_.find(c.id);
                if (vit != audioVoices_.end()) {
                    vit->second.gainId = c.gainId;
                    auto git = gains_.find(c.gainId);
                    if (git != gains_.end()) vit->second.gain = git->second;
                    else vit->second.gain = 1.0;
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

                audioThreadLog(ANDROID_LOG_INFO,
                                    "CMD: attach gain %s -> voice %s (out=%d in=%d)",
                                    c.gainId.c_str(), c.id.c_str(), c.outputIndex, c.inputIndex);
                break;
            }
            case NativeEngine::CMD_DETACH_GAIN: {
                for (auto &kv: audioVoices_) {
                    if (kv.second.gainId == c.id) {
                        kv.second.gainId.clear();
                        kv.second.gain = 1.0;
                    }
                }
                audioThreadLog(ANDROID_LOG_INFO, "CMD: detach gain %s", c.id.c_str());
                break;
            }
            case NativeEngine::CMD_FREE_GAIN: {
                gains_.erase(c.id);
                for (auto &kv: audioVoices_) {
                    if (kv.second.gainId == c.id) {
                        kv.second.gainId.clear();
                        kv.second.gain = 1.0;
                    }
                }

                waveShapers_.erase(c.id);
                audioThreadLog(ANDROID_LOG_INFO, "CMD: free gain %s", c.id.c_str());
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
                if (vit != audioVoices_.end()) {
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
                break;
            }
            case NativeEngine::CMD_DETACH_BIQUAD: {
                for (auto &kv: audioVoices_) {
                    if (kv.second.filterId == c.id) {
                        kv.second.filterId.clear();
                        kv.second.filterState.clear();
                        kv.second.iirState.clear();
                    }
                }
                audioThreadLog(ANDROID_LOG_INFO, "CMD: detach biquad %s", c.id.c_str());
                break;
            }
            case NativeEngine::CMD_FREE_BIQUAD: {
                biquads_.erase(c.id);
                for (auto &kv: audioVoices_) {
                    if (kv.second.filterId == c.id) {
                        kv.second.filterId.clear();
                        kv.second.filterState.clear();
                        kv.second.iirState.clear();
                    }
                }
                audioThreadLog(ANDROID_LOG_INFO, "CMD: free biquad %s", c.id.c_str());
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
                NativeEngine::getInstance().iirs_.erase(c.id);
                for (auto &kv: audioVoices_) {
                    if (kv.second.filterId == c.id) {
                        kv.second.filterId.clear();
                        kv.second.iirState.clear();
                        kv.second.filterState.clear();
                    }
                }
                audioThreadLog(ANDROID_LOG_INFO, "CMD: free iir %s", c.id.c_str());
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
                panners_[c.id] = p;
                audioThreadLog(ANDROID_LOG_INFO, "CMD: create panner %s", c.id.c_str());
                break;
            }
            case NativeEngine::CMD_SET_PANNER_PARAMS: {
                auto it = panners_.find(c.id);
                if (it != panners_.end()) {
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
                }
                audioThreadLog(ANDROID_LOG_INFO, "CMD: set panner %s", c.id.c_str());
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
                    audioThreadLog(ANDROID_LOG_INFO, "CMD: set listener[%s] pos=(%f,%f,%f) fwd=(%f,%f,%f)", c.contextId.c_str(), c.listenerPositionX, c.listenerPositionY, c.listenerPositionZ, c.listenerForwardX, c.listenerForwardY, c.listenerForwardZ);
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
                    audioThreadLog(ANDROID_LOG_INFO, "CMD: set listener pos=(%f,%f,%f) fwd=(%f,%f,%f)", c.listenerPositionX, c.listenerPositionY, c.listenerPositionZ, c.listenerForwardX, c.listenerForwardY, c.listenerForwardZ);
                }
                break;
            }
            case NativeEngine::CMD_ATTACH_PANNER: {
                auto vit = audioVoices_.find(c.id);
                if (vit != audioVoices_.end()) {
                    vit->second.panId = c.pannerId;
                    auto pit = panners_.find(c.pannerId);
                    if (pit != panners_.end())
                        vit->second.pan = static_cast<float>(pit->second.pan);
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
                break;
            }
            case NativeEngine::CMD_DETACH_PANNER: {
                for (auto &kv: audioVoices_) {
                    if (kv.second.panId == c.id) {
                        kv.second.panId.clear();
                        kv.second.pan = 0.0f;
                    }
                }
                audioThreadLog(ANDROID_LOG_INFO, "CMD: detach panner %s", c.id.c_str());
                break;
            }
            case NativeEngine::CMD_FREE_PANNER: {
                panners_.erase(c.id);
                for (auto &kv: audioVoices_) {
                    if (kv.second.panId == c.id) {
                        kv.second.panId.clear();
                        kv.second.pan = 0.0f;
                    }
                }
                audioThreadLog(ANDROID_LOG_INFO, "CMD: free panner %s", c.id.c_str());
                break;
            }
            case NativeEngine::CMD_START_OSC: {
                Voice v;
                auto it = audioVoices_.find(c.id);
                if (it == audioVoices_.end()) {
                    v.type = Voice::Oscillator;
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
                audioVoices_[c.id] = v;
                break;
            }
            case NativeEngine::CMD_STOP_TRACK: {
                audioVoices_.erase(c.id);
                break;
            }
            case NativeEngine::CMD_START_BUFFER: {
                auto bit = audioBuffers_.find(c.id);
                if (bit == audioBuffers_.end()) break;
                Voice v;
                v.type = Voice::BufferSource;
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
                if (!c.externalRing) break;
                Voice v;
                v.type = Voice::ExternalPCM;
                v.id = c.id;
                v.position = 0.0;
                v.loop = false;
                v.gain = 1.0;
                v.playing = true;
                v.bufferChannels = c.channels > 0 ? c.channels : 2;
                v.bufferSampleRate = c.sampleRate > 0 ? c.sampleRate : 48000;
                double sr = streamSampleRate_ > 0 ? static_cast<double>(streamSampleRate_) : 48000.0;
                v.increment = static_cast<double>(v.bufferSampleRate) / sr;
                v.externalRing = c.externalRing;
                v.externalPrev.assign((size_t)v.bufferChannels, 0.0f);
                v.externalCurr.assign((size_t)v.bufferChannels, 0.0f);
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
                break;
            }
            case NativeEngine::CMD_PUSH_EXTERNAL_PCM: {
                auto vit = audioVoices_.find(c.id);
                if (vit == audioVoices_.end() || vit->second.type != Voice::ExternalPCM) break;
                auto &ring = vit->second.externalRing;
                if (!ring || !c.pcmFloat || c.pcmFloat->empty()) break;

                const std::vector<float> &src = *c.pcmFloat;
                int chs = ring->channels;
                if (chs <= 0) break;
                size_t framesIn = src.size() / (size_t)chs;
                if (framesIn == 0) break;

                uint32_t w = ring->writeIdx.load(std::memory_order_relaxed);
                uint32_t r = ring->readIdx.load(std::memory_order_acquire);
                uint32_t avail = ring->capacity - (w - r);
                uint32_t writeFrames = framesIn > avail ? avail : (uint32_t)framesIn;
                for (uint32_t i = 0; i < writeFrames; i++) {
                    uint32_t pos = (w + i) & ring->mask;
                    for (int ch = 0; ch < chs; ch++) {
                        ring->data[(size_t)pos * chs + ch] = src[(size_t)i * chs + ch];
                    }
                }
                ring->writeIdx.store(w + writeFrames, std::memory_order_release);
                break;
            }
            case NativeEngine::CMD_END_EXTERNAL_PCM: {
                auto vit = audioVoices_.find(c.id);
                if (vit != audioVoices_.end() && vit->second.externalRing) {
                    vit->second.externalRing->ended.store(true, std::memory_order_release);
                }
                break;
            }
            default:
                break;
        }
    }

    std::vector<Voice> localVoices;
    std::unordered_map<std::string, BufferData> localBuffers;
    for (auto &kv: audioVoices_) {
        if (kv.second.playing) {
            localVoices.push_back(kv.second);
            if (kv.second.type == Voice::BufferSource) {
                auto it = audioBuffers_.find(kv.second.bufferId);
                if (it != audioBuffers_.end()) localBuffers[kv.second.bufferId] = it->second;
            }
        }
    }

    std::fill(out, out + (numFrames * channels), 0.0f);

    std::unordered_map<std::string, std::vector<NativeEngine::ParamEvent>> scheduledSnapshot;
    std::unordered_map<std::string, std::unordered_map<int, std::vector<NativeEngine::ParamEvent>>> scheduledPannerSnapshot;
    std::unordered_map<std::string, std::unordered_map<int, std::vector<NativeEngine::ParamEvent>>> scheduledListenerSnapshot;
    {
        std::lock_guard<std::mutex> lk(scheduledEventsMutex_);
        for (const auto &kv: scheduledGainEvents_) scheduledSnapshot.emplace(kv.first, kv.second);
        for (const auto &kv: scheduledPannerEvents_)
            scheduledPannerSnapshot.emplace(kv.first, kv.second);
        for (const auto &kv: scheduledListenerEvents_)
            scheduledListenerSnapshot.emplace(kv.first, kv.second);
    }

    int64_t audioStartNs = g_audioTimeNanos.load(std::memory_order_relaxed);
    double sampleDurationNs =
            streamSampleRate_ > 0 ? (1000000000.0 / static_cast<double>(streamSampleRate_)) : (
                    1000000000.0 / 48000.0);


    std::unordered_map<std::string, std::vector<double>> scheduledEnvelopes;
    std::unordered_map<std::string, std::unordered_map<int, std::vector<double>>> scheduledPannerEnvelopes;
    std::unordered_map<std::string, std::unordered_map<int, std::vector<double>>> scheduledListenerEnvelopes;
    if ((!scheduledSnapshot.empty() || !scheduledPannerSnapshot.empty()) && !localVoices.empty()) {
        std::unordered_set<std::string> usedGains;
        for (const auto &v: localVoices) {
            if (!v.gainId.empty()) usedGains.insert(v.gainId);
            auto vgIt = voiceGainByOutput_.find(v.id);
            if (vgIt != voiceGainByOutput_.end()) {
                for (const auto &kv2: vgIt->second) {
                    if (!kv2.second.empty()) usedGains.insert(kv2.second);
                }
            }
        }

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

        std::unordered_set<std::string> usedPanners;
        for (const auto &v: localVoices) {
            if (!v.panId.empty()) usedPanners.insert(v.panId);
            auto vpIt = voicePannerByOutput_.find(v.id);
            if (vpIt != voicePannerByOutput_.end()) {
                for (const auto &kv2: vpIt->second) {
                    if (!kv2.second.empty()) usedPanners.insert(kv2.second);
                }
            }
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

        std::unordered_set<std::string> usedContexts;
        for (const auto &pid : usedPanners) {
            auto pit = panners_.find(pid);
            if (pit != panners_.end()) usedContexts.insert(pit->second.contextId);
        }

        for (const auto &ctxId : usedContexts) {
            auto it = scheduledListenerSnapshot.find(ctxId);
            if (it == scheduledListenerSnapshot.end()) continue;
            const auto &inner = it->second;
            std::unordered_map<int, std::vector<double>> paramMap;
            for (const auto &pkv : inner) {
                int paramType = pkv.first;
                const auto &evts = pkv.second;
                std::vector<double> env((size_t) numFrames);

                double fallback = 0.0;
                auto lit = listeners_.find(ctxId);
                const NativeEngine::Listener *LptrFallback = &listener_;
                if (lit != listeners_.end()) LptrFallback = &lit->second;
                const auto &Lfb = *LptrFallback;
                switch (paramType) {
                    case kListenerParamPositionX: fallback = Lfb.positionX; break;
                    case kListenerParamPositionY: fallback = Lfb.positionY; break;
                    case kListenerParamPositionZ: fallback = Lfb.positionZ; break;
                    case kListenerParamForwardX: fallback = Lfb.forwardX; break;
                    case kListenerParamForwardY: fallback = Lfb.forwardY; break;
                    case kListenerParamForwardZ: fallback = Lfb.forwardZ; break;
                    case kListenerParamUpX: fallback = Lfb.upX; break;
                    case kListenerParamUpY: fallback = Lfb.upY; break;
                    case kListenerParamUpZ: fallback = Lfb.upZ; break;
                    default: fallback = 0.0; break;
                }

                int rate = evts.empty() ? NativeEngine::RATE_A : evts[0].rate;
                if (rate == NativeEngine::RATE_K) {
                    float v = getScheduledGainValueAt(evts, static_cast<int64_t>(audioStartNs), fallback);
                    for (int f = 0; f < numFrames; ++f) env[(size_t) f] = v;
                } else {
                    size_t nextIdx = 0;
                    int prevIdx = -1;
                    for (int f = 0; f < numFrames; ++f) {
                        auto tNs = static_cast<int64_t>(audioStartNs + static_cast<double>(f) * sampleDurationNs);
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
                                    double ratio = double(tNs - prev.timeNs) / double(next.timeNs - prev.timeNs);
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

    for (int frame = 0; frame < numFrames; ++frame) {
        for (int ch = 0; ch < channels; ++ch) {
            float mixed = 0.0f;
            int64_t sampleTimeNs = static_cast<int64_t>(audioStartNs + static_cast<double>(frame) *
                                                                       sampleDurationNs);
            for (auto &v: localVoices) {
                if (!v.playing) continue;
                if (v.type == Voice::Oscillator) {
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

                    std::string effFilterId = v.filterId;
                    auto vfIt = voiceFilterByOutput_.find(v.id);
                    if (vfIt != voiceFilterByOutput_.end()) {
                        auto fIt2 = vfIt->second.find(ch);
                        if (fIt2 != vfIt->second.end()) effFilterId = fIt2->second;
                    }
                    if (!effFilterId.empty()) {
                        auto bcit = biquads_.find(effFilterId);
                        if (bcit != biquads_.end()) {
                            int fsIndex = ch;
                            if (v.filterState.size() <= static_cast<size_t>(fsIndex))
                                v.filterState.assign(channels, NativeEngine::BiquadState());
                            auto in = static_cast<float>(s);
                            float outS = processBiquadSample(bcit->second, v.filterState[fsIndex],
                                                             in);
                            s = outS;
                        } else {
                            auto iit = iirs_.find(effFilterId);
                            if (iit != iirs_.end()) {
                                int fsIndex = ch;
                                size_t chCount = static_cast<size_t>(channels);
                                if (v.iirState.size() <= chCount)
                                    v.iirState.assign(chCount, std::vector<double>());
                                if (v.iirState.size() <= static_cast<size_t>(fsIndex))
                                    v.iirState.assign(chCount, std::vector<double>());
                                float outS = processIIRSample(iit->second,
                                                              v.iirState[static_cast<size_t>(fsIndex)],
                                                              static_cast<float>(s));
                                s = outS;
                            }
                        }
                    }

                    double leftGain = 1.0, rightGain = 1.0, distanceAtt = 1.0;

                    std::string effPanId = v.panId;
                    auto vpIt = voicePannerByOutput_.find(v.id);
                    if (vpIt != voicePannerByOutput_.end()) {
                        auto pIt2 = vpIt->second.find(ch);
                        if (pIt2 != vpIt->second.end()) effPanId = pIt2->second;
                    }
                    if (!effPanId.empty()) {
                        auto pit = panners_.find(effPanId);
                        if (pit != panners_.end()) {
                            NativeEngine::Panner pcopy = pit->second;
                            auto spIt = scheduledPannerEnvelopes.find(effPanId);
                            if (spIt != scheduledPannerEnvelopes.end()) {
                                auto &paramMap = spIt->second;
                                auto itPosX = paramMap.find(kPannerParamPositionX);
                                if (itPosX != paramMap.end())
                                    pcopy.positionX = itPosX->second[(size_t) frame];
                                auto itPosY = paramMap.find(kPannerParamPositionY);
                                if (itPosY != paramMap.end())
                                    pcopy.positionY = itPosY->second[(size_t) frame];
                                auto itPosZ = paramMap.find(kPannerParamPositionZ);
                                if (itPosZ != paramMap.end())
                                    pcopy.positionZ = itPosZ->second[(size_t) frame];
                                auto itOriX = paramMap.find(kPannerParamOrientationX);
                                if (itOriX != paramMap.end())
                                    pcopy.orientationX = itOriX->second[(size_t) frame];
                                auto itOriY = paramMap.find(kPannerParamOrientationY);
                                if (itOriY != paramMap.end())
                                    pcopy.orientationY = itOriY->second[(size_t) frame];
                                auto itOriZ = paramMap.find(kPannerParamOrientationZ);
                                if (itOriZ != paramMap.end())
                                    pcopy.orientationZ = itOriZ->second[(size_t) frame];
                                auto itPan = paramMap.find(kPannerParamPan);
                                if (itPan != paramMap.end())
                                    pcopy.pan = itPan->second[(size_t) frame];
                            }
                            {
                                NativeEngine::Panner ptrans = pcopy;
                                const NativeEngine::Listener *Lptr = &listener_;
                                if (!pit->second.contextId.empty()) {
                                    auto lit = listeners_.find(pit->second.contextId);
                                    if (lit != listeners_.end()) Lptr = &lit->second;
                                }
                                double listenerPosX = Lptr->positionX;
                                double listenerPosY = Lptr->positionY;
                                double listenerPosZ = Lptr->positionZ;
                                double fx = Lptr->forwardX;
                                double fy = Lptr->forwardY;
                                double fz = Lptr->forwardZ;
                                double ux = Lptr->upX;
                                double uy = Lptr->upY;
                                double uz = Lptr->upZ;

                                auto lEnvIt = scheduledListenerEnvelopes.find(pit->second.contextId);
                                if (lEnvIt != scheduledListenerEnvelopes.end()) {
                                    auto &lparamMap = lEnvIt->second;
                                    auto itLPx = lparamMap.find(kListenerParamPositionX);
                                    if (itLPx != lparamMap.end()) listenerPosX = itLPx->second[(size_t) frame];
                                    auto itLPy = lparamMap.find(kListenerParamPositionY);
                                    if (itLPy != lparamMap.end()) listenerPosY = itLPy->second[(size_t) frame];
                                    auto itLPz = lparamMap.find(kListenerParamPositionZ);
                                    if (itLPz != lparamMap.end()) listenerPosZ = itLPz->second[(size_t) frame];
                                    auto itLFX = lparamMap.find(kListenerParamForwardX);
                                    if (itLFX != lparamMap.end()) fx = itLFX->second[(size_t) frame];
                                    auto itLFY = lparamMap.find(kListenerParamForwardY);
                                    if (itLFY != lparamMap.end()) fy = itLFY->second[(size_t) frame];
                                    auto itLFZ = lparamMap.find(kListenerParamForwardZ);
                                    if (itLFZ != lparamMap.end()) fz = itLFZ->second[(size_t) frame];
                                    auto itLUX = lparamMap.find(kListenerParamUpX);
                                    if (itLUX != lparamMap.end()) ux = itLUX->second[(size_t) frame];
                                    auto itLUY = lparamMap.find(kListenerParamUpY);
                                    if (itLUY != lparamMap.end()) uy = itLUY->second[(size_t) frame];
                                    auto itLUZ = lparamMap.find(kListenerParamUpZ);
                                    if (itLUZ != lparamMap.end()) uz = itLUZ->second[(size_t) frame];
                                }
                                double rx = pcopy.positionX - listenerPosX;
                                double ry = pcopy.positionY - listenerPosY;
                                double rz = pcopy.positionZ - listenerPosZ;
                                double fl = std::sqrt(fx*fx + fy*fy + fz*fz);
                                if (fl <= 1e-12) { fx = 0; fy = 0; fz = -1; fl = 1; } else { fx /= fl; fy /= fl; fz /= fl; }
                                double ul = std::sqrt(ux*ux + uy*uy + uz*uz);
                                if (ul <= 1e-12) { ux = 0; uy = 1; uz = 0; ul = 1; } else { ux /= ul; uy /= ul; uz /= ul; }

                                double rxv = fy*uz - fz*uy;
                                double ryv = fz*ux - fx*uz;
                                double rzv = fx*uy - fy*ux;
                                double rl = std::sqrt(rxv*rxv + ryv*ryv + rzv*rzv);
                                if (rl <= 1e-12) { rxv = 1; ryv = 0; rzv = 0; rl = 1; } else { rxv /= rl; ryv /= rl; rzv /= rl; }

                                double ux2 = ryv * fz - rzv * fy;
                                double uy2 = rzv * fx - rxv * fz;
                                double uz2 = rxv * fy - ryv * fx;
                                double localX = rx * rxv + ry * ryv + rz * rzv;
                                double localY = rx * ux2 + ry * uy2 + rz * uz2;
                                double localZ = rx * fx + ry * fy + rz * fz;
                                ptrans.positionX = localX;
                                ptrans.positionY = localY;
                                ptrans.positionZ = localZ;
                                computePannerGains(ptrans, leftGain, rightGain, distanceAtt);
                            }
                        }
                    }


                    if (channels <= 1) {
                        std::string effGainId;
                        auto vgIt = voiceGainByOutput_.find(v.id);
                        if (vgIt != voiceGainByOutput_.end()) {
                            auto gIt2 = vgIt->second.find(ch);
                            if (gIt2 != vgIt->second.end()) effGainId = gIt2->second;
                        }
                        if (effGainId.empty()) effGainId = v.gainId;

                        double scheduled = v.gain;
                        if (!effGainId.empty()) {
                            auto eit = scheduledEnvelopes.find(effGainId);
                            if (eit != scheduledEnvelopes.end())
                                scheduled = eit->second[(size_t) frame];
                            else {
                                auto git = gains_.find(effGainId);
                                if (git != gains_.end()) scheduled = git->second;
                            }
                        }

                        if (!effGainId.empty()) {
                            auto wsIt = waveShapers_.find(effGainId);
                            if (wsIt != waveShapers_.end() && !wsIt->second.curve.empty()) {
                                s = static_cast<double>(applyWaveShaperSample(wsIt->second,
                                                                              static_cast<float>(s)));
                            }
                        }
                        double monoGain = 0.5 * (leftGain + rightGain) * distanceAtt *
                                          static_cast<double>(scheduled);
                        mixed += static_cast<float>(s * monoGain);
                    } else {
                        std::string effGainId2;
                        auto vgIt2 = voiceGainByOutput_.find(v.id);
                        if (vgIt2 != voiceGainByOutput_.end()) {
                            auto gIt3 = vgIt2->second.find(ch);
                            if (gIt3 != vgIt2->second.end()) effGainId2 = gIt3->second;
                        }
                        if (effGainId2.empty()) effGainId2 = v.gainId;

                        double scheduled2 = v.gain;
                        if (!effGainId2.empty()) {
                            auto eit = scheduledEnvelopes.find(effGainId2);
                            if (eit != scheduledEnvelopes.end())
                                scheduled2 = eit->second[(size_t) frame];
                            else {
                                auto git = gains_.find(effGainId2);
                                if (git != gains_.end()) scheduled2 = git->second;
                            }
                        }
                        double g = static_cast<double>(scheduled2) * distanceAtt;
                        if (ch == 0) mixed += static_cast<float>(s * g * leftGain);
                        else if (ch == 1) mixed += static_cast<float>(s * g * rightGain);
                        else {
                            double avg = 0.5 * (leftGain + rightGain);
                            mixed += static_cast<float>(s * g * avg);
                        }
                    }
                    if (ch == channels - 1) {
                        v.phase += v.phaseIncrement;
                        if (v.phase >= 1.0) v.phase -= 1.0;
                    }
                } else if (v.type == Voice::BufferSource || v.type == Voice::ExternalPCM) {
                    int bufChannels = 1;
                    int bufFrames = 0;
                    int chIdx = ch;
                    float sample = 0.0f;
                    int i0_dbg = 0;
                    int idx0_dbg = 0;
                    int idx1_dbg = 0;
                    const BufferData *bd_dbg = nullptr;

                    if (v.type == Voice::BufferSource) {
                        auto bit = localBuffers.find(v.bufferId);
                        if (bit == localBuffers.end()) continue;
                        const BufferData &bd = bit->second;
                        bd_dbg = &bd;
                        bufChannels = bd.channels > 0 ? bd.channels : 1;
                        if (bd.isDirect) {
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
                        if (bd.isDirect && bd.directPtr) {
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
                        bufChannels = ring->channels > 0 ? ring->channels : 1;
                        bufFrames = INT32_MAX;
                        if (bufChannels == 1) chIdx = 0;
                        else if (bufChannels >= channels) chIdx = ch;
                        else chIdx = ch % bufChannels;

                        uint32_t w = ring->writeIdx.load(std::memory_order_acquire);
                        uint32_t r = ring->readIdx.load(std::memory_order_relaxed);
                        if (!v.externalPrimed && (w - r) >= 1) {
                            uint32_t p = r & ring->mask;
                            for (int c = 0; c < bufChannels; c++) {
                                v.externalCurr[(size_t)c] = ring->data[(size_t)p * bufChannels + c];
                            }
                            ring->readIdx.store(r + 1, std::memory_order_release);
                            v.externalPrimed = true;
                        }

                        if (v.externalPrimed) {
                            float prev = v.externalPrev[(size_t)chIdx];
                            float curr = v.externalCurr[(size_t)chIdx];
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

                    std::string effFilterId2 = v.filterId;
                    auto vfIt2 = voiceFilterByOutput_.find(v.id);
                    if (vfIt2 != voiceFilterByOutput_.end()) {
                        auto fIt3 = vfIt2->second.find(ch);
                        if (fIt3 != vfIt2->second.end()) effFilterId2 = fIt3->second;
                    }
                    if (!effFilterId2.empty()) {
                        auto bcit = biquads_.find(effFilterId2);
                        if (bcit != biquads_.end()) {
                            int fsIndex = chIdx;
                            if (v.filterState.size() <= static_cast<size_t>(fsIndex))
                                v.filterState.assign(channels, NativeEngine::BiquadState());
                            sample = processBiquadSample(bcit->second, v.filterState[fsIndex],
                                                         sample);
                        } else {
                            auto iit = iirs_.find(effFilterId2);
                            if (iit != iirs_.end()) {
                                int fsIndex = chIdx;
                                auto chCount = static_cast<size_t>(channels);
                                if (v.iirState.size() <= chCount)
                                    v.iirState.assign(chCount, std::vector<double>());
                                if (v.iirState.size() <= static_cast<size_t>(fsIndex))
                                    v.iirState.assign(chCount, std::vector<double>());
                                sample = processIIRSample(iit->second,
                                                          v.iirState[static_cast<size_t>(fsIndex)],
                                                          sample);
                            }
                        }
                    }

                    double leftGain = 1.0, rightGain = 1.0, distanceAtt = 1.0;
                    std::string effPanId2 = v.panId;
                    auto vpIt2 = voicePannerByOutput_.find(v.id);
                    if (vpIt2 != voicePannerByOutput_.end()) {
                        auto pIt3 = vpIt2->second.find(ch);
                        if (pIt3 != vpIt2->second.end()) effPanId2 = pIt3->second;
                    }
                    if (!effPanId2.empty()) {
                        auto pit = panners_.find(effPanId2);
                        if (pit != panners_.end()) {
                            NativeEngine::Panner pcopy = pit->second;
                            auto spIt = scheduledPannerEnvelopes.find(effPanId2);
                            if (spIt != scheduledPannerEnvelopes.end()) {
                                auto &paramMap = spIt->second;
                                auto itPosX = paramMap.find(kPannerParamPositionX);
                                if (itPosX != paramMap.end())
                                    pcopy.positionX = itPosX->second[(size_t) frame];
                                auto itPosY = paramMap.find(kPannerParamPositionY);
                                if (itPosY != paramMap.end())
                                    pcopy.positionY = itPosY->second[(size_t) frame];
                                auto itPosZ = paramMap.find(kPannerParamPositionZ);
                                if (itPosZ != paramMap.end())
                                    pcopy.positionZ = itPosZ->second[(size_t) frame];
                                auto itOriX = paramMap.find(kPannerParamOrientationX);
                                if (itOriX != paramMap.end())
                                    pcopy.orientationX = itOriX->second[(size_t) frame];
                                auto itOriY = paramMap.find(kPannerParamOrientationY);
                                if (itOriY != paramMap.end())
                                    pcopy.orientationY = itOriY->second[(size_t) frame];
                                auto itOriZ = paramMap.find(kPannerParamOrientationZ);
                                if (itOriZ != paramMap.end())
                                    pcopy.orientationZ = itOriZ->second[(size_t) frame];
                                auto itPan = paramMap.find(kPannerParamPan);
                                if (itPan != paramMap.end())
                                    pcopy.pan = itPan->second[(size_t) frame];
                            }
                            {
                                NativeEngine::Panner ptrans = pcopy;
                                const NativeEngine::Listener *Lptr = &listener_;
                                if (!pit->second.contextId.empty()) {
                                    auto lit = listeners_.find(pit->second.contextId);
                                    if (lit != listeners_.end()) Lptr = &lit->second;
                                }
                                double listenerPosX = Lptr->positionX;
                                double listenerPosY = Lptr->positionY;
                                double listenerPosZ = Lptr->positionZ;
                                double fx = Lptr->forwardX;
                                double fy = Lptr->forwardY;
                                double fz = Lptr->forwardZ;
                                double ux = Lptr->upX;
                                double uy = Lptr->upY;
                                double uz = Lptr->upZ;
                                auto lEnvIt = scheduledListenerEnvelopes.find(pit->second.contextId);
                                if (lEnvIt != scheduledListenerEnvelopes.end()) {
                                    auto &lparamMap = lEnvIt->second;
                                    auto itLPx = lparamMap.find(kListenerParamPositionX);
                                    if (itLPx != lparamMap.end()) listenerPosX = itLPx->second[(size_t) frame];
                                    auto itLPy = lparamMap.find(kListenerParamPositionY);
                                    if (itLPy != lparamMap.end()) listenerPosY = itLPy->second[(size_t) frame];
                                    auto itLPz = lparamMap.find(kListenerParamPositionZ);
                                    if (itLPz != lparamMap.end()) listenerPosZ = itLPz->second[(size_t) frame];
                                    auto itLFX = lparamMap.find(kListenerParamForwardX);
                                    if (itLFX != lparamMap.end()) fx = itLFX->second[(size_t) frame];
                                    auto itLFY = lparamMap.find(kListenerParamForwardY);
                                    if (itLFY != lparamMap.end()) fy = itLFY->second[(size_t) frame];
                                    auto itLFZ = lparamMap.find(kListenerParamForwardZ);
                                    if (itLFZ != lparamMap.end()) fz = itLFZ->second[(size_t) frame];
                                    auto itLUX = lparamMap.find(kListenerParamUpX);
                                    if (itLUX != lparamMap.end()) ux = itLUX->second[(size_t) frame];
                                    auto itLUY = lparamMap.find(kListenerParamUpY);
                                    if (itLUY != lparamMap.end()) uy = itLUY->second[(size_t) frame];
                                    auto itLUZ = lparamMap.find(kListenerParamUpZ);
                                    if (itLUZ != lparamMap.end()) uz = itLUZ->second[(size_t) frame];
                                }
                                double rx = pcopy.positionX - listenerPosX;
                                double ry = pcopy.positionY - listenerPosY;
                                double rz = pcopy.positionZ - listenerPosZ;
                                double fl = std::sqrt(fx*fx + fy*fy + fz*fz);
                                if (fl <= 1e-12) { fx = 0; fy = 0; fz = -1; fl = 1; } else { fx /= fl; fy /= fl; fz /= fl; }
                                double ul = std::sqrt(ux*ux + uy*uy + uz*uz);
                                if (ul <= 1e-12) { ux = 0; uy = 1; uz = 0; ul = 1; } else { ux /= ul; uy /= ul; uz /= ul; }
                                double rxv = fy*uz - fz*uy;
                                double ryv = fz*ux - fx*uz;
                                double rzv = fx*uy - fy*ux;
                                double rl = std::sqrt(rxv*rxv + ryv*ryv + rzv*rzv);
                                if (rl <= 1e-12) { rxv = 1; ryv = 0; rzv = 0; rl = 1; } else { rxv /= rl; ryv /= rl; rzv /= rl; }
                                double ux2 = ryv * fz - rzv * fy;
                                double uy2 = rzv * fx - rxv * fz;
                                double uz2 = rxv * fy - ryv * fx;
                                double localX = rx * rxv + ry * ryv + rz * rzv;
                                double localY = rx * ux2 + ry * uy2 + rz * uz2;
                                double localZ = rx * fx + ry * fy + rz * fz;
                                ptrans.positionX = localX;
                                ptrans.positionY = localY;
                                ptrans.positionZ = localZ;
                                computePannerGains(ptrans, leftGain, rightGain, distanceAtt);
                            }
                        }
                    }

                    if (v.type == Voice::BufferSource && bd_dbg &&
                        g_audioThreadLoggingEnabled.load(std::memory_order_relaxed)) {
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
                            if (bd.isDirect && bd.directPtr) {
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
                            sampleOther = fs0Other * (1.0f - static_cast<float>(frac)) + fs1Other * static_cast<float>(frac);
                            (void)sampleOther;
                            (void)idx0; (void)idx1;
                            audioThreadLog(ANDROID_LOG_INFO,
                                                "VOICE_DEBUG id=%s buffer=%s bufCh=%d streamCh=%d ch0_chIdx=%d ch0_idx0=%d ch0_idx1=%d ch0_sample=%f ch1_chIdx=%d ch1_idx0=%d ch1_idx1=%d ch1_sample=%f leftGain=%f rightGain=%f panId=%s",
                                                v.id.c_str(), v.bufferId.c_str(), bufChannels, channels, chIdx, idx0, idx1, sample, chIdxOther, idx0Other, idx1Other, sampleOther, leftGain, rightGain, v.panId.c_str());
                        }
                    }

                    std::string effGainId3;
                    auto vgIt3 = voiceGainByOutput_.find(v.id);
                    if (vgIt3 != voiceGainByOutput_.end()) {
                        auto gIt4 = vgIt3->second.find(ch);
                        if (gIt4 != vgIt3->second.end()) effGainId3 = gIt4->second;
                    }
                    if (effGainId3.empty()) effGainId3 = v.gainId;

                    if (channels <= 1) {
                        double scheduled = v.gain;
                        if (!effGainId3.empty()) {
                            auto eit = scheduledEnvelopes.find(effGainId3);
                            if (eit != scheduledEnvelopes.end())
                                scheduled = eit->second[(size_t) frame];
                            else {
                                auto git = gains_.find(effGainId3);
                                if (git != gains_.end()) scheduled = git->second;
                            }
                        }

                        if (!effGainId3.empty()) {
                            auto wsIt3 = waveShapers_.find(effGainId3);
                            if (wsIt3 != waveShapers_.end() && !wsIt3->second.curve.empty()) {
                                sample = static_cast<double>(applyWaveShaperSample(wsIt3->second,
                                                                                   static_cast<float>(sample)));
                            }
                        }
                        double monoGain = 0.5 * (leftGain + rightGain) * distanceAtt *
                                          static_cast<double>(scheduled);
                        mixed += static_cast<float>(sample * monoGain);
                    } else {
                        double scheduled = v.gain;
                        if (!effGainId3.empty()) {
                            auto eit = scheduledEnvelopes.find(effGainId3);
                            if (eit != scheduledEnvelopes.end())
                                scheduled = eit->second[(size_t) frame];
                            else {
                                auto git = gains_.find(effGainId3);
                                if (git != gains_.end()) scheduled = git->second;
                            }
                        }
                        double g = static_cast<double>(scheduled) * distanceAtt;
                        if (ch == 0) mixed += static_cast<float>(sample * g * leftGain);
                        else if (ch == 1) mixed += static_cast<float>(sample * g * rightGain);
                        else {
                            double avg = 0.5 * (leftGain + rightGain);
                            mixed += static_cast<float>(sample * g * avg);
                        }
                    }

                    if (ch == channels - 1) {
                        if (v.type == Voice::BufferSource) {
                            v.position += v.increment;
                            if (v.position >= bufFrames) {
                                if (v.loop) v.position = std::fmod(v.position, bufFrames);
                                else v.playing = false;
                            }
                        } else /* ExternalPCM */ {
                            auto &ring = v.externalRing;
                            if (ring && v.externalPrimed) {
                                v.externalSubPos += v.increment;
                                while (v.externalSubPos >= 1.0) {
                                    for (int c = 0; c < bufChannels; c++) {
                                        v.externalPrev[(size_t)c] = v.externalCurr[(size_t)c];
                                    }
                                    uint32_t w = ring->writeIdx.load(std::memory_order_acquire);
                                    uint32_t r = ring->readIdx.load(std::memory_order_relaxed);
                                    if ((w - r) >= 1) {
                                        uint32_t p = r & ring->mask;
                                        for (int c = 0; c < bufChannels; c++) {
                                            v.externalCurr[(size_t)c] =
                                                ring->data[(size_t)p * bufChannels + c];
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


    for (const auto &lv: localVoices) {
        auto it = audioVoices_.find(lv.id);
        if (it == audioVoices_.end()) continue;
        if (!lv.playing) {
            audioVoices_.erase(it);
        } else {
            it->second.phase = lv.phase;
            it->second.position = lv.position;
            it->second.playing = lv.playing;
            it->second.filterState = lv.filterState;
            it->second.iirState = lv.iirState;
            it->second.filterId = lv.filterId;
            it->second.panId = lv.panId;
            it->second.pan = lv.pan;
            it->second.externalPrev = lv.externalPrev;
            it->second.externalCurr = lv.externalCurr;
            it->second.externalSubPos = lv.externalSubPos;
            it->second.externalPrimed = lv.externalPrimed;
        }
    }

    return oboe::DataCallbackResult::Continue;
}

#endif
