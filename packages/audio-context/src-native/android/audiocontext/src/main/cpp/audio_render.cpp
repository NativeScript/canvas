#include "audio_context.h"
#include "audio_internal.h"
#include "convolver.h"

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
    if (offline_) return;
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
    // A time converted from seconds can land a few ns past its own frame.
    constexpr int64_t kFrameTimeSlackNs = 4;
    const double rate = streamSampleRate_ > 0 ? static_cast<double>(streamSampleRate_) : 48000.0;

    if (offline_) {
        // Offline renders one quantum per call, at nowNs_.
        quantumStartNs_ = nowNs_.load(std::memory_order_relaxed) + kFrameTimeSlackNs;
        drainCommands();
        renderQuantum(out, quantumStartNs_);
        return oboe::DataCallbackResult::Continue;
    }

    // A frame clock, so automation and source positions advance together; device timestamps
    // jitter between callbacks. It re-anchors to the monotonic clock after a stall or restart.
    constexpr int64_t kClockBehindNs = 50000000;
    constexpr int64_t kClockAheadNs = 500000000;
    const int64_t now = monotonicNowNs();
    int64_t clock = renderClockNs_;
    if (clock == 0 || now - clock > kClockBehindNs || clock - now > kClockAheadNs) clock = now;
    renderClockNs_ = clock + std::llround(static_cast<double>(numFrames) * 1e9 / rate);

    const size_t quantumSamples = static_cast<size_t>(kQuantum * channels);
    if (quantum_.size() != quantumSamples) {
        quantum_.assign(quantumSamples, 0.0f);
        quantumPos_ = kQuantum;
    }
    // The graph renders whole quanta; a callback plays what's left of the last one first.
    int written = 0;
    while (written < numFrames) {
        if (quantumPos_ >= kQuantum) {
            const int64_t startNs = clock + std::llround(static_cast<double>(written) * 1e9 / rate);
            quantumStartNs_ = startNs + kFrameTimeSlackNs;
            drainCommands();
            // 'Now' is the next quantum: what's queued from here on applies there.
            const int64_t next = startNs + std::llround(static_cast<double>(kQuantum) * 1e9 / rate);
            g_audioTimeNanos.store(next, std::memory_order_relaxed);
            nowNs_.store(next, std::memory_order_relaxed);
            renderQuantum(quantum_.data(), quantumStartNs_);
            quantumPos_ = 0;
        }
        const int n = std::min(numFrames - written, kQuantum - quantumPos_);
        std::memcpy(out + static_cast<size_t>(written * channels),
                    quantum_.data() + static_cast<size_t>(quantumPos_ * channels),
                    static_cast<size_t>(n * channels) * sizeof(float));
        quantumPos_ += n;
        written += n;
    }
    return oboe::DataCallbackResult::Continue;
}

void NativeEngine::drainCommands() {
    std::vector<NativeEngine::Command> cmds;
    {
        std::lock_guard<std::mutex> cl(commandMutex_);
        cmds.swap(pendingCommands_);
    }
    for (auto &c: cmds) applyCommand(c);
    runDeferred();

    // Voices that stopped since the last quantum let their buffers go.
    for (size_t i = 0; i < idleFrees_.size();) {
        if (bufferInUse(idleFrees_[i])) {
            ++i;
            continue;
        }
        std::string id = std::move(idleFrees_[i]);
        idleFrees_[i] = std::move(idleFrees_.back());
        idleFrees_.pop_back();
        releaseBufferData(id);
    }
}

void NativeEngine::applyCommand(Command &c) {
    const int channels = streamChannels_ > 0 ? streamChannels_ : 2;
    const double sr = streamSampleRate_ > 0 ? static_cast<double>(streamSampleRate_) : 48000.0;
    switch (c.type) {
        case CMD_CREATE_BUFFER_DIRECT: {
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
            if (jvm_ && audioContextClassGlobalRef && onNativeBufferHeldMethod) {
                JNIEnv *env = nullptr;
                bool attached = false;
                if (jvm_->GetEnv(reinterpret_cast<void **>(&env), JNI_VERSION_1_6) != JNI_OK) {
                    if (jvm_->AttachCurrentThread(&env, nullptr) == JNI_OK) attached = true;
                    else env = nullptr;
                }
                if (env) {
                    jstring js = env->NewStringUTF(c.id.c_str());
                    env->CallStaticVoidMethod(audioContextClassGlobalRef, onNativeBufferHeldMethod, js);
                    env->DeleteLocalRef(js);
                    if (attached) jvm_->DetachCurrentThread();
                }
            }
            break;
        }
        case CMD_CREATE_BUFFER_PLANAR: {
            BufferData bd;
            bd.isDirect = true;
            bd.isPlanar = true;
            if (c.planarPtrs) bd.planarPtrs = *c.planarPtrs;
            bd.planarFrames = c.planarFrames > 0 ? c.planarFrames : 0;
            bd.bytesPerSample = 4;
            bd.sampleRate = c.sampleRate > 0 ? c.sampleRate : 48000;
            bd.channels = c.channels > 0 ? c.channels : static_cast<int>(bd.planarPtrs.size());
            if (bd.channels <= 0) bd.channels = 1;
            if (bd.planarFrames > 0) {
                bd.byteLength = static_cast<size_t>(bd.planarFrames) * static_cast<size_t>(bd.channels) * 4;
            }
            bd.nativeOwned = false;
            audioBuffers_[c.id] = std::move(bd);
            break;
        }
        case CMD_CREATE_BUFFER_COPY: {
            BufferData bd;
            bd.pcm = std::move(*c.pcm);
            bd.sampleRate = c.sampleRate > 0 ? c.sampleRate : 48000;
            bd.channels = c.channels > 0 ? c.channels : 1;
            bd.nativeOwned = true;
            audioBuffers_[c.id] = std::move(bd);
            break;
        }
        case CMD_CREATE_OSC: {
            Voice v;
            v.voiceType = Voice::Oscillator;
            v.id = c.id;
            v.waveform = c.waveform;
            v.frequency = c.frequency;
            audioVoices_[c.id] = v;
            ensureNode(c.id, NODE_SOURCE, 1);
            break;
        }
        case CMD_CREATE_DELAY: {
            DelayLine d;
            d.channels = std::max(1, streamChannels_);
            d.size = static_cast<int>(std::ceil(std::max(c.maxDelayTime, 0.0) * sr)) + kQuantum + 2;
            d.data.assign(static_cast<size_t>(d.size) * d.channels, 0.0f);
            delays_[c.id] = std::move(d);
            gains_[c.id] = 0.0;
            break;
        }
        case CMD_CREATE_GAIN:
            gains_[c.id] = c.gainValue > 0.0 ? c.gainValue : 1.0;
            break;
        case CMD_CREATE_DYNAMICS_COMPRESSOR: {
            DynamicsCompressor compressor;
            compressor.thresholdId = c.compressorThresholdId;
            compressor.kneeId = c.compressorKneeId;
            compressor.ratioId = c.compressorRatioId;
            compressor.attackId = c.compressorAttackId;
            compressor.releaseId = c.compressorReleaseId;
            compressor.reductionId = c.compressorReductionId;
            compressor.meter = c.meter;
            compressors_[c.id] = std::move(compressor);
            auto ensureGainDefault = [&](const std::string &gid, double value) {
                if (!gid.empty() && gains_.find(gid) == gains_.end()) gains_[gid] = value;
            };
            ensureGainDefault(c.compressorThresholdId, -24.0);
            ensureGainDefault(c.compressorKneeId, 30.0);
            ensureGainDefault(c.compressorRatioId, 12.0);
            ensureGainDefault(c.compressorAttackId, 0.003);
            ensureGainDefault(c.compressorReleaseId, 0.25);
            ensureGainDefault(c.compressorReductionId, 0.0);
            orderDirty_ = true;
            break;
        }
        case CMD_SET_GAIN:
            gains_[c.id] = c.gainValue;
            break;
        case CMD_ATTACH_DETUNE: {
            sourceExtras_[c.id].detuneId = c.targetId;
            auto vit = audioVoices_.find(c.id);
            if (vit != audioVoices_.end()) vit->second.detuneId = c.targetId;
            break;
        }
        case CMD_SET_SOURCE_LOOP: {
            SourceExtras &ex = sourceExtras_[c.id];
            ex.loopStart = c.loopStart;
            ex.loopEnd = c.loopEnd;
            auto vit = audioVoices_.find(c.id);
            if (vit != audioVoices_.end()) {
                vit->second.loop = c.loop;
                vit->second.loopStart = c.loopStart;
                vit->second.loopEnd = c.loopEnd;
            }
            break;
        }
        case CMD_ATTACH_PLAYBACK_RATE: {
            auto vit = audioVoices_.find(c.id);
            if (vit != audioVoices_.end()) vit->second.playbackRateId = c.playbackRateId;
            if (c.playbackRateId.empty()) voicePlaybackRate_.erase(c.id);
            else voicePlaybackRate_[c.id] = c.playbackRateId;
            orderDirty_ = true;
            break;
        }
        case CMD_DETACH_PLAYBACK_RATE:
        case CMD_FREE_PLAYBACK_RATE: {
            for (auto &kv: audioVoices_) {
                if (kv.second.playbackRateId == c.id) kv.second.playbackRateId.clear();
            }
            for (auto it = voicePlaybackRate_.begin(); it != voicePlaybackRate_.end();) {
                if (it->second == c.id) it = voicePlaybackRate_.erase(it);
                else ++it;
            }
            orderDirty_ = true;
            break;
        }
        case CMD_FREE_GAIN: {
            if (deferFree(c)) break;
            delays_.erase(c.id);
            gains_.erase(c.id);
            waveShapers_.erase(c.id);
            convolvers_.erase(c.id);
            auto compressorIt = compressors_.find(c.id);
            if (compressorIt != compressors_.end()) {
                for (const std::string *paramId: {&compressorIt->second.thresholdId, &compressorIt->second.kneeId,
                                                  &compressorIt->second.ratioId, &compressorIt->second.attackId,
                                                  &compressorIt->second.releaseId, &compressorIt->second.reductionId}) {
                    if (!paramId->empty()) gains_.erase(*paramId);
                }
                compressors_.erase(compressorIt);
            }
            break;
        }
        case CMD_CREATE_BIQUAD: {
            const std::string &type = !c.waveform.empty() ? c.waveform : c.biquadType;
            biquads_[c.id] = computeBiquadCoeffs(type, c.biquadFrequency > 0.0 ? c.biquadFrequency : 440.0,
                                                 c.biquadQ > 0.0 ? c.biquadQ : 1.0, c.biquadGain,
                                                 static_cast<int>(sr));
            biquadFilters_[c.id].type = type;
            break;
        }
        case CMD_SET_BIQUAD_PARAMS: {
            BiquadFilter &f = biquadFilters_[c.id];
            if (!c.biquadType.empty()) f.type = c.biquadType;
            f.last[0] = -1.0;
            break;
        }
        case CMD_SET_BIQUAD_PARAM_IDS: {
            BiquadFilter &f = biquadFilters_[c.id];
            for (size_t i = 0; i < 4 && i < c.paramIds.size(); i++) f.paramIds[i] = c.paramIds[i];
            f.last[0] = -1.0;
            break;
        }
        case CMD_FREE_BIQUAD: {
            if (deferFree(c)) break;
            biquads_.erase(c.id);
            auto f = biquadFilters_.find(c.id);
            if (f != biquadFilters_.end()) {
                for (const std::string &param: f->second.paramIds) {
                    if (!param.empty()) gains_.erase(param);
                }
                biquadFilters_.erase(f);
            }
            break;
        }
        case CMD_SET_CONVOLVER:
            if (c.convolver) convolvers_[c.id] = c.convolver;
            else convolvers_.erase(c.id);
            break;
        case CMD_CREATE_IIR: {
            IIRData d;
            d.feedforward = c.iirFeedforward;
            d.feedback = c.iirFeedback;
            double a0 = 1.0;
            if (!d.feedback.empty() && d.feedback[0] != 0.0) a0 = d.feedback[0];
            if (a0 != 1.0) {
                for (double &x: d.feedforward) x /= a0;
                for (double &x: d.feedback) x /= a0;
            }
            iirs_[c.id] = d;
            break;
        }
        case CMD_FREE_IIR:
            if (deferFree(c)) break;
            iirs_.erase(c.id);
            break;
        case CMD_CREATE_PERIODICWAVE: {
            PeriodicWaveData d;
            d.real = c.periodicReal;
            d.imag = c.periodicImag;
            d.disableNormalization = c.periodicDisableNormalization;
            periodicWaves_[c.id] = d;
            break;
        }
        case CMD_ATTACH_PERIODICWAVE: {
            auto vit = audioVoices_.find(c.id);
            if (vit != audioVoices_.end()) vit->second.periodicWaveId = c.periodicWaveId;
            break;
        }
        case CMD_FREE_PERIODICWAVE:
            periodicWaves_.erase(c.id);
            for (auto &kv: audioVoices_) {
                if (kv.second.periodicWaveId == c.id) kv.second.periodicWaveId.clear();
            }
            break;
        case CMD_SET_WAVESHAPER_CURVE: {
            auto it = waveShapers_.find(c.id);
            if (it == waveShapers_.end()) {
                WaveShaperData d;
                if (!c.waveShaperCurve.empty()) d.curve = c.waveShaperCurve;
                if (!c.waveShaperOversample.empty()) d.oversample = c.waveShaperOversample;
                waveShapers_[c.id] = std::move(d);
            } else {
                if (!c.waveShaperCurve.empty()) it->second.curve = c.waveShaperCurve;
                if (!c.waveShaperOversample.empty()) it->second.oversample = c.waveShaperOversample;
            }
            break;
        }
        case CMD_CREATE_PANNER: {
            Panner p;
            p.positionX = c.pannerPositionX;
            p.positionY = c.pannerPositionY;
            p.positionZ = c.pannerPositionZ;
            p.orientationX = c.pannerOrientationX;
            p.orientationY = c.pannerOrientationY;
            p.orientationZ = c.pannerOrientationZ;
            p.pan = c.pannerPan;
            p.distanceModel = static_cast<PannerDistanceModel>(c.pannerDistanceModel);
            p.panningModel = static_cast<PannerPanningModel>(c.pannerPanningModel);
            p.refDistance = c.pannerRefDistance;
            p.maxDistance = c.pannerMaxDistance;
            p.rolloffFactor = c.pannerRolloffFactor;
            p.coneInnerAngle = c.pannerConeInnerAngle;
            p.coneOuterAngle = c.pannerConeOuterAngle;
            p.coneOuterGain = c.pannerConeOuterGain;
            p.contextId = c.contextId;
            panners_[c.id] = p;
            break;
        }
        case CMD_SET_PANNER_PARAMS: {
            auto it = panners_.find(c.id);
            if (it == panners_.end()) break;
            auto &p = it->second;
            p.positionX = c.pannerPositionX;
            p.positionY = c.pannerPositionY;
            p.positionZ = c.pannerPositionZ;
            p.orientationX = c.pannerOrientationX;
            p.orientationY = c.pannerOrientationY;
            p.orientationZ = c.pannerOrientationZ;
            p.pan = c.pannerPan;
            p.distanceModel = static_cast<PannerDistanceModel>(c.pannerDistanceModel);
            p.panningModel = static_cast<PannerPanningModel>(c.pannerPanningModel);
            p.refDistance = c.pannerRefDistance;
            p.maxDistance = c.pannerMaxDistance;
            p.rolloffFactor = c.pannerRolloffFactor;
            p.coneInnerAngle = c.pannerConeInnerAngle;
            p.coneOuterAngle = c.pannerConeOuterAngle;
            p.coneOuterGain = c.pannerConeOuterGain;
            break;
        }
        case CMD_SET_LISTENER_PARAMS: {
            Listener &L = c.contextId.empty() ? listener_ : listeners_[c.contextId];
            L.positionX = c.listenerPositionX;
            L.positionY = c.listenerPositionY;
            L.positionZ = c.listenerPositionZ;
            L.forwardX = c.listenerForwardX;
            L.forwardY = c.listenerForwardY;
            L.forwardZ = c.listenerForwardZ;
            L.upX = c.listenerUpX;
            L.upY = c.listenerUpY;
            L.upZ = c.listenerUpZ;
            break;
        }
        case CMD_FREE_PANNER: {
            if (deferFree(c)) break;
            auto it = panners_.find(c.id);
            if (it == panners_.end()) break;
            panners_.erase(it);
            break;
        }
        case CMD_START_OSC: {
            Voice v;
            auto it = audioVoices_.find(c.id);
            if (it == audioVoices_.end()) {
                v.voiceType = Voice::Oscillator;
                v.id = c.id;
            } else v = it->second;
            v.waveform = c.waveform;
            v.frequency = c.frequency;
            v.phaseIncrement = v.frequency / sr;
            v.playing = true;
            v.startAtNs = c.timeNs;
            v.stopAtNs = 0;
            audioVoices_[c.id] = v;
            ensureNode(c.id, NODE_SOURCE, 1);
            break;
        }
        case CMD_STOP_TRACK: {
            if (worklets_.count(c.id)) {
                removeWorklet(c.id);
                break;
            }
            audioVoices_.erase(c.id);
            // A stopped source never plays again.
            auto node = graph_.find(c.id);
            if (node != graph_.end() && node->second->kind == NODE_SOURCE) removeNode(node->second.get());
            break;
        }
        case CMD_STOP_TRACK_AT: {
            auto vit = audioVoices_.find(c.id);
            if (vit != audioVoices_.end()) vit->second.stopAtNs = c.timeNs;
            break;
        }
        case CMD_SET_OSC_TYPE: {
            auto vit = audioVoices_.find(c.id);
            if (vit != audioVoices_.end() && vit->second.voiceType == Voice::Oscillator) {
                vit->second.waveform = c.waveform;
                // A basic type replaces a periodic wave, as in the spec.
                vit->second.periodicWaveId.clear();
            }
            break;
        }
        case CMD_START_BUFFER: {
            auto bit = audioBuffers_.find(c.id);
            if (bit == audioBuffers_.end()) break;
            Voice v;
            v.voiceType = Voice::BufferSource;
            v.id = c.id;
            v.bufferId = c.id;
            v.loop = c.loop;
            v.playing = true;
            v.startAtNs = c.timeNs;
            v.bufferChannels = bit->second.channels;
            v.bufferSampleRate = bit->second.sampleRate;
            v.position = c.startOffset * static_cast<double>(v.bufferSampleRate);
            v.increment = static_cast<double>(v.bufferSampleRate) / sr;
            auto pr = voicePlaybackRate_.find(c.id);
            if (pr != voicePlaybackRate_.end()) v.playbackRateId = pr->second;
            auto ex = sourceExtras_.find(c.id);
            if (ex != sourceExtras_.end()) {
                v.detuneId = ex->second.detuneId;
                v.loopStart = ex->second.loopStart;
                v.loopEnd = ex->second.loopEnd;
            }
            audioVoices_[c.id] = v;
            ensureNode(c.id, NODE_SOURCE, 1);
            break;
        }
        case CMD_FREE_BUFFER:
            releaseBufferData(c.id, c.completion);
            break;
        case CMD_FREE_BUFFER_WHEN_IDLE:
            if (bufferInUse(c.id)) idleFrees_.push_back(c.id);
            else releaseBufferData(c.id);
            break;
        case CMD_CONTEXT_START:
            if (!c.clock) break;
            c.clock->offsetNs.store(quantumStartNs_ - c.clock->baseNs, std::memory_order_relaxed);
            c.clock->pausedAtNs.store(0, std::memory_order_relaxed);
            break;
        case CMD_CONTEXT_SUSPEND:
            if (c.clock && c.clock->pausedAtNs.load(std::memory_order_relaxed) == 0) {
                c.clock->pausedAtNs.store(quantumStartNs_, std::memory_order_relaxed);
            }
            break;
        case CMD_CONTEXT_RESUME: {
            if (!c.clock) break;
            const int64_t paused = c.clock->pausedAtNs.load(std::memory_order_relaxed);
            if (paused == 0) break;
            c.clock->offsetNs.fetch_add(quantumStartNs_ - paused, std::memory_order_relaxed);
            c.clock->pausedAtNs.store(0, std::memory_order_relaxed);
            break;
        }
        case CMD_RESUME:
            ensureStream();
            break;
        case CMD_SUSPEND:
            stopStream();
            break;
        case CMD_CREATE_WORKLET:
            if (!c.worklet) break;
            worklets_[c.worklet->id] = c.worklet;
            ensureNode(c.worklet->id, NODE_WORKLET, 0);
            break;
        case CMD_CREATE_EXTERNAL_PCM: {
            if (!c.externalRing) break;
            Voice v;
            v.voiceType = Voice::ExternalPCM;
            v.id = c.id;
            v.bufferChannels = c.channels > 0 ? c.channels : 2;
            v.bufferSampleRate = c.sampleRate > 0 ? c.sampleRate : 48000;
            v.increment = static_cast<double>(v.bufferSampleRate) / sr;
            v.externalRing = c.externalRing;
            v.externalPrev.assign(static_cast<size_t>(v.bufferChannels), 0.0f);
            v.externalCurr.assign(static_cast<size_t>(v.bufferChannels), 0.0f);
            audioVoices_[c.id] = v;
            ensureNode(c.id, NODE_SOURCE, 1);
            break;
        }
        case CMD_CONFIGURE_EXTERNAL_PCM: {
            auto vit = audioVoices_.find(c.id);
            if (vit == audioVoices_.end() || vit->second.voiceType != Voice::ExternalPCM) break;
            Voice &v = vit->second;
            const int ch = c.channels > 0 ? c.channels : 2;
            v.bufferChannels = ch;
            v.bufferSampleRate = c.sampleRate > 0 ? c.sampleRate : 48000;
            v.increment = static_cast<double>(v.bufferSampleRate) / sr;
            v.externalPrev.assign(static_cast<size_t>(ch), 0.0f);
            v.externalCurr.assign(static_cast<size_t>(ch), 0.0f);
            v.externalSubPos = 0.0;
            v.externalPrimed = false;
            auto &ring = v.externalRing;
            if (ring) {
                ring->channels = ch;
                ring->data.assign(static_cast<size_t>(ring->capacity) * static_cast<size_t>(ch), 0.0f);
                ring->writeIdx.store(0, std::memory_order_release);
                ring->readIdx.store(0, std::memory_order_release);
                ring->ended.store(false, std::memory_order_release);
                v.playing = false;
            }
            break;
        }
        case CMD_PUSH_EXTERNAL_PCM: {
            auto vit = audioVoices_.find(c.id);
            if (vit == audioVoices_.end() || vit->second.voiceType != Voice::ExternalPCM) break;
            Voice &v = vit->second;
            auto &ring = v.externalRing;
            if (!ring || !c.pcmFloat || c.pcmFloat->empty() || ring->capacity == 0) break;
            const int chs = std::max(1, std::min(ring->channels > 0 ? ring->channels : v.bufferChannels,
                                                 static_cast<int>(v.externalCurr.size())));
            if (ring->data.size() < static_cast<size_t>(ring->capacity) * chs) break;
            const std::vector<float> &src = *c.pcmFloat;
            const size_t framesIn = src.size() / static_cast<size_t>(chs);
            uint32_t w = ring->writeIdx.load(std::memory_order_relaxed);
            uint32_t r = ring->readIdx.load(std::memory_order_acquire);
            uint32_t avail = ring->capacity - (w - r);
            uint32_t writeFrames = framesIn > avail ? avail : static_cast<uint32_t>(framesIn);
            if (writeFrames > 0) v.playing = true;
            for (uint32_t i = 0; i < writeFrames; i++) {
                uint32_t pos = (w + i) & ring->mask;
                for (int ch = 0; ch < chs; ch++) {
                    ring->data[static_cast<size_t>(pos) * chs + ch] = src[static_cast<size_t>(i) * chs + ch];
                }
            }
            ring->writeIdx.store(w + writeFrames, std::memory_order_release);
            break;
        }
        case CMD_END_EXTERNAL_PCM: {
            auto vit = audioVoices_.find(c.id);
            if (vit != audioVoices_.end() && vit->second.externalRing) {
                vit->second.externalRing->ended.store(true, std::memory_order_release);
            }
            break;
        }
        case CMD_CONNECT:
            connectNodes(c);
            break;
        case CMD_DISCONNECT:
            disconnectNodes(c);
            break;
        case CMD_NODE_CONFIG:
            configureNode(c);
            break;
        case CMD_REPLACE_SOURCE: {
            audioVoices_.erase(c.id);
            if (c.targetId.empty() || c.targetId == c.id || graph_.count(c.targetId)) break;
            auto it = graph_.find(c.id);
            if (it == graph_.end()) break;
            std::unique_ptr<GraphNode> node = std::move(it->second);
            graph_.erase(it);
            node->id = c.targetId;
            graph_[c.targetId] = std::move(node);
            orderDirty_ = true;
            break;
        }
        default:
            break;
    }
    (void) channels;
}

#endif

bool NativeEngine::bufferInUse(const std::string &id) const {
    for (const auto &kv: audioVoices_) {
        const Voice &v = kv.second;
        if (v.playing && v.voiceType == Voice::BufferSource && v.bufferId == id) return true;
    }
    return false;
}

void NativeEngine::releaseBufferData(const std::string &id,
                                     const std::shared_ptr<std::promise<void>> &completion) {
    auto it = audioBuffers_.find(id);
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

    if (completion) {
        try {
            completion->set_value();
        } catch (...) {}
    }

    // Java frees the memory it lent once nothing here reads it.
    if (jvm_ && audioContextClassGlobalRef && onNativeBufferReleasedMethod) {
        JNIEnv *env = nullptr;
        bool attached = false;
        if (jvm_->GetEnv(reinterpret_cast<void **>(&env), JNI_VERSION_1_6) != JNI_OK) {
            if (jvm_->AttachCurrentThread(&env, nullptr) == JNI_OK) attached = true;
            else env = nullptr;
        }
        if (env) {
            jstring js = env->NewStringUTF(id.c_str());
            env->CallStaticVoidMethod(audioContextClassGlobalRef,
                                      onNativeBufferReleasedMethod, js);
            env->DeleteLocalRef(js);
            if (attached) jvm_->DetachCurrentThread();
        }
    }
}
