#include "audio_context.h"
#include "audio_internal.h"


NativeEngine &NativeEngine::getInstance() {
    static NativeEngine inst;
    return inst;
}

NativeEngine::NativeEngine() = default;

void NativeEngine::registerContextStart(const std::string &contextId, int64_t startNanos) {
    std::lock_guard<std::mutex> lock(contextMutex_);
    contextStartNanos_[contextId] = startNanos;
}

void NativeEngine::unregisterContextStart(const std::string &contextId) {
    std::lock_guard<std::mutex> lock(contextMutex_);
    contextStartNanos_.erase(contextId);
}


void
NativeEngine::scheduleGainEvent(const std::string &gainId, int type, int rate, double value,
                                int64_t timeNs) {
    std::lock_guard<std::mutex> lock(scheduledEventsMutex_);
    auto &vec = scheduledGainEvents_[gainId];
    ParamEvent ev{type, rate, timeNs, value};
    insertSortedByTime(vec, ev);
}

void
NativeEngine::schedulePannerEvent(const std::string &pannerId, int paramType, int type, int rate,
                                  double value, int64_t timeNs) {
    std::lock_guard<std::mutex> lock(scheduledEventsMutex_);
    auto &map = scheduledPannerEvents_[pannerId];
    auto &vec = map[paramType];
    ParamEvent ev{type, rate, timeNs, value};
    insertSortedByTime(vec, ev);
}

void
NativeEngine::scheduleListenerEvent(const std::string &contextId, int paramType, int type, int rate,
                                    double value, int64_t timeNs) {
    std::lock_guard<std::mutex> lock(scheduledEventsMutex_);
    auto &map = scheduledListenerEvents_[contextId];
    auto &vec = map[paramType];
    ParamEvent ev{type, rate, timeNs, value};
    insertSortedByTime(vec, ev);
}


void NativeEngine::cancelGainEvents(const std::string &gainId, int64_t timeNs) {
    std::lock_guard<std::mutex> lock(scheduledEventsMutex_);
    auto it = scheduledGainEvents_.find(gainId);
    if (it == scheduledGainEvents_.end()) return;
    auto &vec = it->second;
    trimEventsAtOrAfter(vec, timeNs);
    if (vec.empty()) scheduledGainEvents_.erase(it);
}

void NativeEngine::cancelPannerEvents(const std::string &pannerId, int paramType, int64_t timeNs) {
    std::lock_guard<std::mutex> lock(scheduledEventsMutex_);
    auto pit = scheduledPannerEvents_.find(pannerId);
    if (pit == scheduledPannerEvents_.end()) return;
    auto it = pit->second.find(paramType);
    if (it == pit->second.end()) return;
    auto &vec = it->second;
    trimEventsAtOrAfter(vec, timeNs);
    if (vec.empty()) pit->second.erase(it);
    if (pit->second.empty()) scheduledPannerEvents_.erase(pit);
}

void NativeEngine::cancelListenerEvents(const std::string &contextId, int paramType, int64_t timeNs) {
    std::lock_guard<std::mutex> lock(scheduledEventsMutex_);
    auto cit = scheduledListenerEvents_.find(contextId);
    if (cit == scheduledListenerEvents_.end()) return;
    auto it = cit->second.find(paramType);
    if (it == cit->second.end()) return;
    auto &vec = it->second;
    trimEventsAtOrAfter(vec, timeNs);
    if (vec.empty()) cit->second.erase(it);
    if (cit->second.empty()) scheduledListenerEvents_.erase(cit);
}

void NativeEngine::cancelAndHoldGainEvents(const std::string &gainId, int rate, double value,
                                           int64_t timeNs) {
    std::lock_guard<std::mutex> lock(scheduledEventsMutex_);
    auto &vec = scheduledGainEvents_[gainId];
    trimEventsAtOrAfter(vec, timeNs);
    vec.push_back(ParamEvent{0 /* set */, rate, timeNs, value});
}

void NativeEngine::cancelAndHoldPannerEvents(const std::string &pannerId, int paramType, int rate,
                                             double value, int64_t timeNs) {
    std::lock_guard<std::mutex> lock(scheduledEventsMutex_);
    auto &map = scheduledPannerEvents_[pannerId];
    auto &vec = map[paramType];
    trimEventsAtOrAfter(vec, timeNs);
    vec.push_back(ParamEvent{0 /* set */, rate, timeNs, value});
}

void
NativeEngine::cancelAndHoldListenerEvents(const std::string &contextId, int paramType, int rate,
                                          double value, int64_t timeNs) {
    std::lock_guard<std::mutex> lock(scheduledEventsMutex_);
    auto &map = scheduledListenerEvents_[contextId];
    auto &vec = map[paramType];
    trimEventsAtOrAfter(vec, timeNs);
    vec.push_back(ParamEvent{0 /* set */, rate, timeNs, value});
}


std::vector<double>
NativeEngine::getPannerParamValues(const std::string &pannerId, int paramType, int64_t startNs,
                                   double sampleRate, int frameCount) {
    std::vector<double> out(frameCount, 0.0);
    std::lock_guard<std::mutex> lock(scheduledEventsMutex_);
    std::vector<ParamEvent> evts;
    auto pit = scheduledPannerEvents_.find(pannerId);
    if (pit != scheduledPannerEvents_.end()) {
        auto it = pit->second.find(paramType);
        if (it != pit->second.end()) evts = it->second;
    }
    double fallback = 0.0;
    auto pit2 = panners_.find(pannerId);
    if (pit2 != panners_.end()) {
        const Panner &p = pit2->second;
        switch (paramType) {
            case kPannerParamPositionX:
                fallback = p.positionX;
                break;
            case kPannerParamPositionY:
                fallback = p.positionY;
                break;
            case kPannerParamPositionZ:
                fallback = p.positionZ;
                break;
            case kPannerParamOrientationX:
                fallback = p.orientationX;
                break;
            case kPannerParamOrientationY:
                fallback = p.orientationY;
                break;
            case kPannerParamOrientationZ:
                fallback = p.orientationZ;
                break;
            case kPannerParamPan:
                fallback = p.pan;
                break;
            default:
                fallback = 0.0;
                break;
        }
    }

    if (evts.empty()) {
        for (int i = 0; i < frameCount; ++i) out[i] = fallback;
        return out;
    }

    int rate = evts.front().rate;
    if (rate == RATE_K) {
        float v = getScheduledGainValueAt(evts, startNs, fallback);
        for (int i = 0; i < frameCount; ++i) out[i] = v;
        return out;
    }

    double nsPerSample = 1e9 / (sampleRate > 0.0 ? sampleRate : 48000.0);
    for (int i = 0; i < frameCount; ++i) {
        int64_t tNs = startNs + static_cast<int64_t>(std::llround(i * nsPerSample));
        out[i] = getScheduledGainValueAt(evts, tNs, fallback);
    }
    return out;
}

std::vector<double>
NativeEngine::getListenerParamValues(const std::string &contextId, int paramType, int64_t startNs,
                                     double sampleRate, int frameCount) {
    std::vector<double> out(frameCount, 0.0);
    std::lock_guard<std::mutex> lock(scheduledEventsMutex_);
    std::vector<ParamEvent> evts;
    auto cit = scheduledListenerEvents_.find(contextId);
    if (cit != scheduledListenerEvents_.end()) {
        auto it = cit->second.find(paramType);
        if (it != cit->second.end()) evts = it->second;
    }
    double fallback = 0.0;
    auto lit = listeners_.find(contextId);
    if (lit != listeners_.end()) {
        const Listener &L = lit->second;
        switch (paramType) {
            case kListenerParamPositionX: fallback = L.positionX; break;
            case kListenerParamPositionY: fallback = L.positionY; break;
            case kListenerParamPositionZ: fallback = L.positionZ; break;
            case kListenerParamForwardX: fallback = L.forwardX; break;
            case kListenerParamForwardY: fallback = L.forwardY; break;
            case kListenerParamForwardZ: fallback = L.forwardZ; break;
            case kListenerParamUpX: fallback = L.upX; break;
            case kListenerParamUpY: fallback = L.upY; break;
            case kListenerParamUpZ: fallback = L.upZ; break;
            default: fallback = 0.0; break;
        }
    }

    if (evts.empty()) {
        for (int i = 0; i < frameCount; ++i) out[i] = fallback;
        return out;
    }

    int rate = evts.front().rate;
    if (rate == RATE_K) {
        float v = getScheduledGainValueAt(evts, startNs, fallback);
        for (int i = 0; i < frameCount; ++i) out[i] = v;
        return out;
    }

    double nsPerSample = 1e9 / (sampleRate > 0.0 ? sampleRate : 48000.0);
    for (int i = 0; i < frameCount; ++i) {
        int64_t tNs = startNs + static_cast<int64_t>(std::llround(i * nsPerSample));
        out[i] = getScheduledGainValueAt(evts, tNs, fallback);
    }
    return out;
}

double NativeEngine::getListenerParamValue(const std::string &contextId, int paramType) {
    std::lock_guard<std::mutex> lock(scheduledEventsMutex_);
    auto lit = listeners_.find(contextId);
    if (lit == listeners_.end()) return 0.0;
    const Listener &L = lit->second;
    switch (paramType) {
        case kListenerParamPositionX: return L.positionX;
        case kListenerParamPositionY: return L.positionY;
        case kListenerParamPositionZ: return L.positionZ;
        case kListenerParamForwardX: return L.forwardX;
        case kListenerParamForwardY: return L.forwardY;
        case kListenerParamForwardZ: return L.forwardZ;
        case kListenerParamUpX: return L.upX;
        case kListenerParamUpY: return L.upY;
        case kListenerParamUpZ: return L.upZ;
        default: return 0.0;
    }
}

std::vector<double>
NativeEngine::getGainParamValues(const std::string &gainId, int64_t startNs, double sampleRate,
                                 int frameCount) {
    std::vector<double> out(frameCount, 0.0);
    std::lock_guard<std::mutex> lock(scheduledEventsMutex_);
    std::vector<ParamEvent> evts;
    auto git = scheduledGainEvents_.find(gainId);
    if (git != scheduledGainEvents_.end()) evts = git->second;
    double fallback = 1.0;
    auto git2 = gains_.find(gainId);
    if (git2 != gains_.end()) fallback = git2->second;
    if (evts.empty()) {
        for (int i = 0; i < frameCount; ++i) out[i] = fallback;
        return out;
    }

    int rate = evts.front().rate;
    if (rate == RATE_K) {
        float v = getScheduledGainValueAt(evts, startNs, fallback);
        for (int i = 0; i < frameCount; ++i) out[i] = v;
        return out;
    }

    double nsPerSample = 1e9 / (sampleRate > 0.0 ? sampleRate : 48000.0);
    for (int i = 0; i < frameCount; ++i) {
        int64_t tNs = startNs + static_cast<int64_t>(std::llround(i * nsPerSample));
        out[i] = getScheduledGainValueAt(evts, tNs, fallback);
    }
    return out;
}

std::pair<std::vector<double>, std::vector<double>>
NativeEngine::getIIRCoefficients(const std::string &iirId) {
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    std::pair<std::vector<double>, std::vector<double>> empty;
    auto it = iirs_.find(iirId);
    if (it == iirs_.end()) return empty;
    return std::make_pair(it->second.feedforward, it->second.feedback);
}

int64_t NativeEngine::getContextCurrentTimeNanos(const std::string &contextId) {
    int64_t audioNs = g_audioTimeNanos.load(std::memory_order_relaxed);
    if (audioNs == 0) {
        audioNs = std::chrono::duration_cast<std::chrono::nanoseconds>(
                std::chrono::steady_clock::now().time_since_epoch()).count();
    }
    std::lock_guard<std::mutex> lock(contextMutex_);
    auto it = contextStartNanos_.find(contextId);
    if (it == contextStartNanos_.end()) return 0;
    int64_t startNs = it->second;
    int64_t diff = audioNs - startNs;
    return diff < 0 ? 0 : diff;
}

void NativeEngine::enqueueCommand(NativeEngine::Command &&cmd) {
    std::lock_guard<std::mutex> lock(commandMutex_);
    pendingCommands_.push_back(std::move(cmd));
}

void NativeEngine::setJavaVM(JavaVM *vm) {
    jvm_ = vm;
    if (!jvm_) return;
    JNIEnv *env = nullptr;
    bool attached = false;
    if (jvm_->GetEnv(reinterpret_cast<void **>(&env), JNI_VERSION_1_6) != JNI_OK) {
        if (jvm_->AttachCurrentThread(&env, nullptr) == JNI_OK) attached = true;
        else env = nullptr;
    }
    if (!env) return;
    jclass cls = env->FindClass("org/nativescript/audiocontext/AudioContext");
    if (cls) {
        audioContextClassGlobalRef = (jclass) (env->NewGlobalRef(cls));
        env->DeleteLocalRef(cls);
        onNativeBufferHeldMethod = env->GetStaticMethodID(audioContextClassGlobalRef,
                                                          "onNativeBufferHeld",
                                                          "(Ljava/lang/String;)V");
        onNativeBufferReleasedMethod = env->GetStaticMethodID(audioContextClassGlobalRef,
                                                              "onNativeBufferReleased",
                                                              "(Ljava/lang/String;)V");
    }
    if (attached) jvm_->DetachCurrentThread();
}

NativeEngine::~NativeEngine() {
#ifdef HAS_OBOE
    stopStream();
#endif
    {
        std::lock_guard<std::recursive_mutex> lock(mutex_);
        for (auto &kv: audioBuffers_) {
            BufferData &bd = kv.second;
            if (bd.nativeOwned) bd.pcm.clear();
            bd.directPtr = nullptr;
            bd.byteLength = 0;
            bd.javaBufferId.clear();
        }
        audioBuffers_.clear();
    }

    if (jvm_ && audioContextClassGlobalRef) {
        JNIEnv *env = nullptr;
        bool attached = false;
        if (jvm_->GetEnv(reinterpret_cast<void **>(&env), JNI_VERSION_1_6) != JNI_OK) {
            if (jvm_->AttachCurrentThread(&env, nullptr) == JNI_OK) attached = true;
            else env = nullptr;
        }
        if (env) {
            env->DeleteGlobalRef(audioContextClassGlobalRef);
            audioContextClassGlobalRef = nullptr;
            onNativeBufferHeldMethod = nullptr;
            onNativeBufferReleasedMethod = nullptr;
        }
        if (attached) jvm_->DetachCurrentThread();
    }
}

void NativeEngine::configureStream(int sampleRate, double latencyHintSec) {
#ifdef HAS_OBOE
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    if (sampleRate > 0) desiredSampleRate_ = sampleRate;
    if (latencyHintSec > 0.0) desiredLatencyHintSec_ = latencyHintSec;
    ensureStream();
#else
    (void)sampleRate; (void)latencyHintSec;
#endif
}

void NativeEngine::resume() {
#ifdef HAS_OBOE
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    ensureStream();
#endif
}

void NativeEngine::setOutputDeviceId(int deviceId) {
#ifdef HAS_OBOE
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    if (deviceId == desiredDeviceId_) return;
    desiredDeviceId_ = deviceId;
    bool wasRunning = (stream_ != nullptr);
    if (wasRunning) {
        stopStream();
        ensureStream();
    }
#else
    (void)deviceId;
#endif
}

void NativeEngine::suspend() {
#ifdef HAS_OBOE
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    stopStream();
#endif
}


std::string NativeEngine::decodeAudioDataFromDirect(const uint8_t *data, size_t len, JNIEnv *env,
                                                    jobject byteBuffer) {
    if (!data || len < 44) return {};
    if (std::memcmp(data, "RIFF", 4) != 0) return {};
    int bitsPerSample = data[34] | (data[35] << 8);
    int audioFormat = data[20] | (data[21] << 8);
    if (audioFormat != 1) return {};
    if (bitsPerSample != 16) return {};

    size_t dataOffset = 12;
    size_t dataLen = 0;
    while (dataOffset + 8 < len) {
        if (std::memcmp(data + dataOffset, "data", 4) == 0) {
            dataLen = *reinterpret_cast<const uint32_t *>(data + dataOffset + 4);
            dataOffset += 8;
            break;
        }
        uint32_t chunkSize = *reinterpret_cast<const uint32_t *>(data + dataOffset + 4);
        dataOffset += 8 + chunkSize;
    }
    if (dataLen == 0 || dataOffset + dataLen > len) return {};

    int numChannels = data[22] | (data[23] << 8);
    int sampleRate = data[24] | (data[25] << 8) | (data[26] << 16) | (data[27] << 24);

    std::string id = genId();
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_CREATE_BUFFER_DIRECT;
    cmd.id = id;
    cmd.directPtr = reinterpret_cast<const void *>(data + dataOffset);
    cmd.byteLen = dataLen;
    cmd.bytesPerSample = (bitsPerSample == 16) ? 2 : 4;
    cmd.sampleRate = sampleRate > 0 ? sampleRate : 48000;
    cmd.channels = numChannels > 0 ? numChannels : 1;
    enqueueCommand(std::move(cmd));
    __android_log_print(ANDROID_LOG_INFO, TAG,
                        "decodeAudioDataFromDirect: enqueued direct buffer %s (sr=%d ch=%d)",
                        id.c_str(), sampleRate, numChannels);
    return id;
}

std::string
NativeEngine::createBufferSourceFromDirect(const void *ptr, size_t byteLen, int sampleRate,
                                           int channels, int bytesPerSample, JNIEnv *env,
                                           jobject byteBuffer) {
    if (!ptr || byteLen == 0) return {};
    std::string id = genId();
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_CREATE_BUFFER_DIRECT;
    cmd.id = id;
    cmd.directPtr = reinterpret_cast<const void *>(ptr);
    cmd.byteLen = byteLen;
    cmd.bytesPerSample = bytesPerSample > 0 ? bytesPerSample : 4;
    cmd.sampleRate = sampleRate > 0 ? sampleRate : 48000;
    cmd.channels = channels > 0 ? channels : 1;
    enqueueCommand(std::move(cmd));
    __android_log_print(ANDROID_LOG_INFO, TAG,
                        "createBufferSourceFromDirect: enqueued direct %s bytes=%zu sr=%d ch=%d",
                        id.c_str(), byteLen, cmd.sampleRate, cmd.channels);
    return id;
}

bool NativeEngine::hasBuffer(const std::string &id) {
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    return audioBuffers_.find(id) != audioBuffers_.end();
}

std::string NativeEngine::createBufferSourceFromExisting(const std::string &bufferId) {
    if (bufferId.empty()) return {};
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    auto it = audioBuffers_.find(bufferId);
    if (it == audioBuffers_.end()) return {};
    __android_log_print(ANDROID_LOG_INFO, TAG, "createBufferSourceFromExisting: reused buffer %s",
                        bufferId.c_str());
    return bufferId;
}


std::string NativeEngine::decodeAudioData(const std::vector<uint8_t> &data) {
    if (data.size() < 44) return {};
    if (std::memcmp(data.data(), "RIFF", 4) != 0) return {};
    int bitsPerSample = data[34] | (data[35] << 8);
    int audioFormat = data[20] | (data[21] << 8);
    if (audioFormat != 1) return {};
    if (bitsPerSample != 16) return {};

    size_t dataOffset = 12;
    size_t dataLen = 0;
    while (dataOffset + 8 < data.size()) {
        if (std::memcmp(data.data() + dataOffset, "data", 4) == 0) {
            dataLen = *reinterpret_cast<const uint32_t *>(data.data() + dataOffset + 4);
            dataOffset += 8;
            break;
        }
        uint32_t chunkSize = *reinterpret_cast<const uint32_t *>(data.data() + dataOffset + 4);
        dataOffset += 8 + chunkSize;
    }
    if (dataLen == 0 || dataOffset + dataLen > data.size()) return {};

    int numChannels = data[22] | (data[23] << 8);
    int sampleRate = data[24] | (data[25] << 8) | (data[26] << 16) | (data[27] << 24);

    size_t samples = dataLen / 2;
    std::vector<int16_t> pcm(samples);
    const uint8_t *ptr = data.data() + dataOffset;
    for (size_t i = 0; i < samples; ++i) {
        auto s = (int16_t) (ptr[2 * i] | (ptr[2 * i + 1] << 8));
        pcm[i] = s;
    }

    std::string id = genId();
    auto sp = std::make_shared<std::vector<int16_t>>(std::move(pcm));
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_CREATE_BUFFER_COPY;
    cmd.id = id;
    cmd.pcm = sp;
    cmd.sampleRate = sampleRate > 0 ? sampleRate : 48000;
    cmd.channels = numChannels > 0 ? numChannels : 1;
    enqueueCommand(std::move(cmd));
    __android_log_print(ANDROID_LOG_INFO, TAG,
                        "decodeAudioData: enqueued copy buffer %s (sr=%d ch=%d)", id.c_str(),
                        cmd.sampleRate, cmd.channels);
    return id;
}

std::string NativeEngine::createOscillator(const std::string &type, double frequency) {
    std::string id = genId();
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_CREATE_OSC;
    cmd.id = id;
    cmd.waveform = type;
    cmd.frequency = frequency;
    enqueueCommand(std::move(cmd));
    __android_log_print(ANDROID_LOG_INFO, TAG, "createOscillator enqueued %s type=%s freq=%f",
                        id.c_str(), type.c_str(), frequency);
    return id;
}

static inline int nextPowerOfTwo(int v) {
    int p = 1;
    while (p < v) p <<= 1;
    return p;
}

void NativeEngine::rebuildAnalyserCaches(NativeEngine::AnalyserData &ad) {
    int n = ad.fftSize;
    if (n <= 0) n = 2048;
    int half = n / 2;

    if (ad.fftCfg) {
        kiss_fft_free(ad.fftCfg);
        ad.fftCfg = nullptr;
    }
    ad.fftCfg = kiss_fft_alloc(n, 0, nullptr, nullptr);

    ad.window.resize((size_t) n);
    const float twoPiOverN = 2.0f * (float) M_PI / (float) n;
    for (int i = 0; i < n; ++i) {
        ad.window[(size_t) i] = 0.5f * (1.0f - std::cos(twoPiOverN * (float) i));
    }

    ad.fin.assign((size_t) n, kiss_fft_cpx{0.0f, 0.0f});
    ad.fout.assign((size_t) n, kiss_fft_cpx{0.0f, 0.0f});
    ad.sampleScratch.assign((size_t) n, 0.0f);
    ad.smoothedMag.assign((size_t) half, 0.0f);
    ad.dbCache.assign((size_t) half, 0.0f);
    ad.byteScratch.assign((size_t) std::max(n, half), 0.0f);
    ad.spectrumFrameStamp = -1;
    ad.dbCacheFrameStamp = -1;
}

std::string
NativeEngine::createAnalyser(int fftSize, double smoothingTimeConstant, double minDecibels,
                             double maxDecibels) {
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    std::string id = genId();
    AnalyserData d;
    int p = nextPowerOfTwo(fftSize > 0 ? fftSize : 2048);
    d.fftSize = p;
    d.capacity = p;
    d.capacityMask = p - 1;
    d.ring.assign((size_t) d.capacity, 0.0f);
    atomic_init(&d.writeIdx, 0);
    d.smoothingTimeConstant = smoothingTimeConstant;
    d.minDecibels = minDecibels;
    d.maxDecibels = maxDecibels;
    rebuildAnalyserCaches(d);
    analysers_.emplace(id, std::move(d));
    __android_log_print(ANDROID_LOG_INFO, TAG, "createAnalyser: %s fft=%d", id.c_str(), p);
    return id;
}

void NativeEngine::freeAnalyser(const std::string &id) {
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    analysers_.erase(id);
}


static inline void copyLatestSamples(NativeEngine::AnalyserData &ad, float *dst, int n) {
    int64_t widx = ad.writeIdx.load(std::memory_order_acquire);
    int cap = ad.capacity;
    if (cap <= 0 || n <= 0) return;
    int64_t start = widx - n;
    int leadZeros = (start < 0) ? (int) std::min<int64_t>(-start, n) : 0;
    if (leadZeros > 0) std::memset(dst, 0, (size_t) leadZeros * sizeof(float));
    int firstReal = leadZeros;
    int64_t firstSrc = (start < 0) ? 0 : start;
    const int mask = ad.capacityMask;
    const bool isPow2 = (mask > 0) && ((cap & mask) == 0);
    int idx = isPow2 ? (int) (firstSrc & (int64_t) mask)
                     : (int) (((firstSrc % (int64_t) cap) + cap) % cap);
    int remaining = n - firstReal;
    const float *ring = ad.ring.data();
    while (remaining > 0) {
        int chunk = std::min(remaining, cap - idx);
        std::memcpy(dst + firstReal, ring + idx, (size_t) chunk * sizeof(float));
        firstReal += chunk;
        remaining -= chunk;
        idx = isPow2 ? ((idx + chunk) & mask) : ((idx + chunk) % cap);
    }
}


static int ensureSmoothedSpectrum(NativeEngine::AnalyserData &ad) {
    int n = ad.fftSize;
    if (n <= 0) return 0;
    int half = n / 2;
    int64_t widx = ad.writeIdx.load(std::memory_order_acquire);
    if (ad.spectrumFrameStamp == widx && (int) ad.smoothedMag.size() == half) {
        return half;
    }
    if ((int) ad.window.size() != n || !ad.fftCfg) {
        NativeEngine::rebuildAnalyserCaches(ad);
    }

    float *samples = ad.sampleScratch.data();
    copyLatestSamples(ad, samples, n);

    kiss_fft_cpx *fin = ad.fin.data();
    kiss_fft_cpx *fout = ad.fout.data();
    const float *win = ad.window.data();
    for (int i = 0; i < n; ++i) {
        fin[i].r = samples[i] * win[i];
        fin[i].i = 0.0f;
    }
    if (ad.fftCfg) {
        kiss_fft(ad.fftCfg, fin, fout);
    } else {
        std::memset(fout, 0, (size_t) n * sizeof(kiss_fft_cpx));
    }

    const float invN2 = 1.0f / ((float) n * (float) n + 1e-30f);
    float tau = (float) std::max(0.0, std::min(1.0, ad.smoothingTimeConstant));
    float oneMinus = 1.0f - tau;
    float *mag = ad.smoothedMag.data();
    for (int i = 0; i < half; ++i) {
        float re = fout[i].r;
        float im = fout[i].i;
        float power = (re * re + im * im) * invN2;
        mag[i] = tau * mag[i] + oneMinus * power;
    }
    ad.spectrumFrameStamp = widx;
    ad.dbCacheFrameStamp = -1;
    return half;
}

static void ensureDbCacheLocked(NativeEngine::AnalyserData &ad, int half) {
    if (ad.dbCacheFrameStamp == ad.spectrumFrameStamp) return;
    if ((int) ad.dbCache.size() < half) ad.dbCache.assign((size_t) half, 0.0f);
    const float *mag = ad.smoothedMag.data();
    float *db = ad.dbCache.data();
    for (int i = 0; i < half; ++i) {
        float m = mag[i];
        if (m < 1e-20f) m = 1e-20f;
        db[i] = 10.0f * std::log10(m);
    }
    ad.dbCacheFrameStamp = ad.spectrumFrameStamp;
}

std::vector<float> NativeEngine::getAnalyserTimeDomainData(const std::string &id, int count) {
    std::vector<float> out;
    if (id.empty() || count <= 0) return out;
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    auto it = analysers_.find(id);
    if (it == analysers_.end()) return out;
    AnalyserData &ad = it->second;
    int n = std::min(count, ad.capacity);
    out.assign((size_t) n, 0.0f);
    copyLatestSamples(ad, out.data(), n);
    return out;
}

std::vector<float> NativeEngine::getAnalyserFrequencyData(const std::string &id) {
    std::vector<float> out;
    if (id.empty()) return out;
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    auto it = analysers_.find(id);
    if (it == analysers_.end()) return out;
    AnalyserData &ad = it->second;
    int half = ensureSmoothedSpectrum(ad);
    if (half <= 0) return out;
    ensureDbCacheLocked(ad, half);
    out.assign(ad.dbCache.begin(), ad.dbCache.begin() + half);
    return out;
}

int NativeEngine::getAnalyserTimeDomainDataInto(const std::string &id, float *dst, int dstCount) {
    if (id.empty() || !dst || dstCount <= 0) return 0;
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    auto it = analysers_.find(id);
    if (it == analysers_.end()) return 0;
    AnalyserData &ad = it->second;
    int n = std::min(dstCount, ad.capacity);
    copyLatestSamples(ad, dst, n);
    return n;
}

int NativeEngine::getAnalyserFrequencyDataInto(const std::string &id, float *dst, int dstCount) {
    if (id.empty() || !dst || dstCount <= 0) return 0;
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    auto it = analysers_.find(id);
    if (it == analysers_.end()) return 0;
    AnalyserData &ad = it->second;
    int half = ensureSmoothedSpectrum(ad);
    int n = std::min(dstCount, half);
    if (n <= 0) return 0;
    ensureDbCacheLocked(ad, half);
    std::memcpy(dst, ad.dbCache.data(), (size_t) n * sizeof(float));
    return n;
}

int NativeEngine::getAnalyserByteFrequencyDataInto(const std::string &id, uint8_t *dst, int dstCount,
                                                   float minDb, float maxDb) {
    if (id.empty() || !dst || dstCount <= 0) return 0;
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    auto it = analysers_.find(id);
    if (it == analysers_.end()) return 0;
    AnalyserData &ad = it->second;
    int half = ensureSmoothedSpectrum(ad);
    int n = std::min(dstCount, half);
    if (n <= 0) return 0;
    ensureDbCacheLocked(ad, half);
    float range = maxDb - minDb;
    if (range <= 0.0f) range = 1.0f;
    const float invRange = 1.0f / range;
    const float *db = ad.dbCache.data();
    for (int i = 0; i < n; ++i) {
        float norm = (db[i] - minDb) * invRange;
        if (norm < 0.0f) norm = 0.0f;
        else if (norm > 1.0f) norm = 1.0f;
        dst[i] = (uint8_t) (norm * 255.0f);
    }
    return n;
}

int NativeEngine::getAnalyserByteTimeDomainDataInto(const std::string &id, uint8_t *dst, int dstCount) {
    if (id.empty() || !dst || dstCount <= 0) return 0;
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    auto it = analysers_.find(id);
    if (it == analysers_.end()) return 0;
    AnalyserData &ad = it->second;
    int n = std::min(dstCount, ad.capacity);
    if (n <= 0) return 0;
    if ((int) ad.byteScratch.size() < n) ad.byteScratch.assign((size_t) n, 0.0f);
    float *tmp = ad.byteScratch.data();
    copyLatestSamples(ad, tmp, n);
    for (int i = 0; i < n; ++i) {
        float v = tmp[i];
        if (v < -1.0f) v = -1.0f;
        else if (v > 1.0f) v = 1.0f;
        int b = (int) ((v * 0.5f + 0.5f) * 255.0f);
        if (b < 0) b = 0;
        else if (b > 255) b = 255;
        dst[i] = (uint8_t) b;
    }
    return n;
}

void NativeEngine::setAnalyserDecibels(const std::string &id, double minDecibels, double maxDecibels) {
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    auto it = analysers_.find(id);
    if (it == analysers_.end()) return;
    AnalyserData &ad = it->second;
    ad.minDecibels = minDecibels;
    ad.maxDecibels = maxDecibels;
}

void NativeEngine::setAnalyserFftSize(const std::string &id, int fftSize) {
    if (id.empty()) return;
    int p = nextPowerOfTwo(fftSize > 0 ? fftSize : 2048);
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    auto it = analysers_.find(id);
    if (it == analysers_.end()) return;
    AnalyserData &ad = it->second;
    if (ad.fftSize == p && ad.capacity == p && ad.fftCfg) return;
    ad.fftSize = p;
    ad.capacity = p;
    ad.capacityMask = p - 1;
    ad.ring.assign((size_t) p, 0.0f);
    ad.writeIdx.store(0, std::memory_order_release);
    rebuildAnalyserCaches(ad);
}

void NativeEngine::setAnalyserSmoothingTimeConstant(const std::string &id, double value) {
    if (id.empty()) return;
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    auto it = analysers_.find(id);
    if (it == analysers_.end()) return;
    AnalyserData &ad = it->second;
    ad.smoothingTimeConstant = value;
    ad.spectrumFrameStamp = -1;
    ad.dbCacheFrameStamp = -1;
}

void
NativeEngine::startOscillator(const std::string &id, const std::string &type, double frequency) {
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_START_OSC;
    cmd.id = id;
    cmd.waveform = type;
    cmd.frequency = frequency;
    enqueueCommand(std::move(cmd));
    __android_log_print(ANDROID_LOG_INFO, TAG, "startOscillator enqueued %s type=%s freq=%f",
                        id.c_str(), type.c_str(), frequency);
}

void NativeEngine::stopTrack(const std::string &id) {
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_STOP_TRACK;
    cmd.id = id;
    enqueueCommand(std::move(cmd));
    __android_log_print(ANDROID_LOG_INFO, TAG, "stopTrack enqueued %s", id.c_str());
}

std::string
NativeEngine::createBufferSource(std::vector<int16_t> &&pcm, int sampleRate, int channels) {
    std::string id = genId();
    auto sp = std::make_shared<std::vector<int16_t>>(std::move(pcm));
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_CREATE_BUFFER_COPY;
    cmd.id = id;
    cmd.pcm = sp;
    cmd.sampleRate = sampleRate > 0 ? sampleRate : 48000;
    cmd.channels = channels > 0 ? channels : 1;
    enqueueCommand(std::move(cmd));
    __android_log_print(ANDROID_LOG_INFO, TAG, "createBufferSource enqueued %s sr=%d ch=%d",
                        id.c_str(), cmd.sampleRate, cmd.channels);
    return id;
}

void NativeEngine::freeBuffer(const std::string &id) {
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_FREE_BUFFER;
    cmd.id = id;
    enqueueCommand(std::move(cmd));
    __android_log_print(ANDROID_LOG_INFO, TAG, "freeBuffer enqueued %s", id.c_str());
}

void NativeEngine::startBufferSource(const std::string &id, bool loop) {
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_START_BUFFER;
    cmd.id = id;
    cmd.loop = loop;
    enqueueCommand(std::move(cmd));
    __android_log_print(ANDROID_LOG_INFO, TAG, "startBufferSource enqueued %s loop=%d", id.c_str(),
                        loop ? 1 : 0);
}



