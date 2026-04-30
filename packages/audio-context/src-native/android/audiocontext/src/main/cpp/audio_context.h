#ifndef AUDIO_CONTEXT_H
#define AUDIO_CONTEXT_H

#include <jni.h>
#include <string>
#include <vector>
#include <unordered_map>
#include <unordered_set>
#include <mutex>
#include <utility>
#include <future>
#include <memory>
#include <atomic>
#include <cstdint>

#include "kiss_fft.h"

#ifdef HAS_OBOE

#include <oboe/Oboe.h>

#endif

class NativeEngine {
public:
    static NativeEngine &getInstance();

    void resume();

    void suspend();

    std::string decodeAudioData(const std::vector<uint8_t> &data);

    std::string createOscillator(const std::string &type, double frequency);

    void startOscillator(const std::string &id, const std::string &type, double frequency);

    void stopTrack(const std::string &id);

    std::string createBufferSource(std::vector<int16_t> &&pcm, int sampleRate, int channels);

    std::string createBufferSourceFromExisting(const std::string &bufferId);

    bool hasBuffer(const std::string &id);

    void startBufferSource(const std::string &id, bool loop);

    std::string
    renderOfflineForTracks(const std::vector<std::string> &trackIds, int frames, int sampleRate,
                           int channels);

    void setJavaVM(JavaVM *vm);

    void configureStream(int sampleRate, double latencyHintSec);

    void setOutputDeviceId(int deviceId);

    int getOutputDeviceId() const { return desiredDeviceId_; }

    std::string
    decodeAudioDataFromDirect(const uint8_t *data, size_t len, JNIEnv *env, jobject byteBuffer);

    std::string
    createBufferSourceFromDirect(const void *ptr, size_t byteLen, int sampleRate, int channels,
                                 int bytesPerSample, JNIEnv *env, jobject byteBuffer);

    void freeBuffer(const std::string &id);

    std::string createExternalPcmSource(int sampleRate, int channels);
    void pushPcmFrames(const std::string &id, const float *interleaved, size_t sampleCount);
    void endExternalPcmSource(const std::string &id);

#ifdef HAS_OBOE

    oboe::DataCallbackResult
    onAudioReady(oboe::AudioStream *stream, void *audioData, int32_t numFrames);

#endif

    void registerContextStart(const std::string &contextId, int64_t startNanos);

    void unregisterContextStart(const std::string &contextId);

    int64_t getContextCurrentTimeNanos(const std::string &contextId);

    enum PannerDistanceModel {
        DISTANCE_INVERSE = 0, DISTANCE_LINEAR = 1, DISTANCE_EXPONENTIAL = 2
    };
private:
    NativeEngine();

    ~NativeEngine();

    void ensureStream();

    void stopStream();

    std::recursive_mutex mutex_;
    struct BufferData {
        std::vector<int16_t> pcm;
        bool isDirect = false;
        std::string javaBufferId;
        const void *directPtr = nullptr;
        size_t byteLength = 0;
        // bytes per sample (2 = int16, 4 = float32)
        int bytesPerSample = 4;
        int sampleRate = 0;
        int channels = 0;
        bool nativeOwned = false;
    };

public:
    struct BiquadCoeffs {
        double b0 = 0.0;
        double b1 = 0.0;
        double b2 = 0.0;
        double a1 = 0.0;
        double a2 = 0.0;
    };

    struct BiquadState {
        double s1 = 0.0;
        double s2 = 0.0;
    };
private:

    struct ExternalRing {
        std::vector<float> data;
        uint32_t capacity = 0;
        uint32_t mask = 0;
        int channels = 1;
        std::atomic<uint32_t> writeIdx{0};
        std::atomic<uint32_t> readIdx{0};
        std::atomic<bool> ended{false};
    };

    struct Voice {
        enum Type {
            Oscillator, BufferSource, ExternalPCM
        } type;
        std::string id;
        std::string waveform = "sine";
        double frequency = 440.0;
        double phase = 0.0;
        double phaseIncrement = 0.0;
        std::string bufferId;
        double position = 0.0;
        double increment = 1.0;
        bool loop = false;
        double gain = 1.0;
        std::string gainId;
        std::string playbackRateId;
        std::string filterId;
        std::vector<BiquadState> filterState;
        std::vector<std::vector<double>> iirState;
        std::string periodicWaveId;
        std::string panId;
        double pan = 0.0;
        bool playing = false;
        int bufferChannels = 0;
        int bufferSampleRate = 0;

        std::shared_ptr<ExternalRing> externalRing;
        std::vector<float> externalPrev;
        std::vector<float> externalCurr;
        double externalSubPos = 0.0;
        bool externalPrimed = false;
    };

    std::unordered_map<std::string, BufferData> audioBuffers_;
    std::unordered_map<std::string, Voice> audioVoices_;
    std::unordered_set<std::string> debugLoggedVoices_;
public:
    enum CommandType {
        CMD_CREATE_OSC,
        CMD_START_OSC,
        CMD_STOP_TRACK,
        CMD_CREATE_BUFFER_DIRECT,
        CMD_CREATE_BUFFER_COPY,
        CMD_START_BUFFER,
        CMD_FREE_BUFFER,
        CMD_CREATE_GAIN,
        CMD_SET_GAIN,
        CMD_ATTACH_GAIN,
        CMD_DETACH_GAIN,
        CMD_FREE_GAIN,
        CMD_CREATE_BIQUAD,
        CMD_SET_BIQUAD_PARAMS,
        CMD_ATTACH_BIQUAD,
        CMD_DETACH_BIQUAD,
        CMD_FREE_BIQUAD,
        CMD_CREATE_PANNER,
        CMD_SET_PANNER_PARAMS,
        CMD_SET_LISTENER_PARAMS,
        CMD_ATTACH_PANNER,
        CMD_DETACH_PANNER,
        CMD_FREE_PANNER,
        CMD_CREATE_IIR,
        CMD_FREE_IIR,
        CMD_CREATE_PERIODICWAVE,
        CMD_SET_WAVESHAPER_CURVE,
        CMD_ATTACH_PERIODICWAVE,
        CMD_FREE_PERIODICWAVE,
        CMD_RESUME,
        CMD_SUSPEND,
        CMD_CREATE_EXTERNAL_PCM,
        CMD_PUSH_EXTERNAL_PCM,
        CMD_END_EXTERNAL_PCM
        ,
        CMD_ATTACH_PLAYBACK_RATE,
        CMD_DETACH_PLAYBACK_RATE,
        CMD_FREE_PLAYBACK_RATE
    };

    struct Command {
        CommandType type;
        std::string id;
        std::string contextId;
        std::string waveform;
        double frequency = 0.0;
        std::string gainId;
        std::string playbackRateId;
        double gainValue = 1.0;
        std::string biquadId;
        std::string biquadType;
        double biquadFrequency = 440.0;
        double biquadQ = 1.0;
        double biquadGain = 0.0;
        std::string pannerId;
        double listenerPositionX = 0.0;
        double listenerPositionY = 0.0;
        double listenerPositionZ = 0.0;
        double listenerForwardX = 0.0;
        double listenerForwardY = 0.0;
        double listenerForwardZ = -1.0;
        double listenerUpX = 0.0;
        double listenerUpY = 1.0;
        double listenerUpZ = 0.0;
        double pannerPositionX = 0.0;
        double pannerPositionY = 0.0;
        double pannerPositionZ = 0.0;
        double pannerOrientationX = 1.0;
        double pannerOrientationY = 0.0;
        double pannerOrientationZ = 0.0;
        double pannerPan = 0.0;
        int pannerDistanceModel = 0;
        int pannerPanningModel = 0;
        double pannerRefDistance = 1.0;
        double pannerMaxDistance = 10000.0;
        double pannerRolloffFactor = 1.0;
        double pannerConeInnerAngle = 360.0;
        double pannerConeOuterAngle = 360.0;
        double pannerConeOuterGain = 0.0;
        const void *directPtr = nullptr;
        size_t byteLen = 0;
        int bytesPerSample = 4;
        int sampleRate = 0;
        int channels = 0;
        std::shared_ptr<std::vector<int16_t>> pcm;
        std::shared_ptr<std::vector<float>> pcmFloat;
        std::shared_ptr<ExternalRing> externalRing;
        bool loop = false;
        std::shared_ptr<std::promise<void>> completion;
        int outputIndex = 0;
        int inputIndex = 0;
        std::vector<double> iirFeedforward;
        std::vector<double> iirFeedback;
        std::vector<double> periodicReal;
        std::vector<double> periodicImag;
        bool periodicDisableNormalization = false;
        std::string periodicWaveId;
        std::vector<float> waveShaperCurve;
        std::string waveShaperOversample;
    };

    void enqueueCommand(Command &&cmd);

private:
    std::vector<Command> pendingCommands_;
    std::mutex commandMutex_;

    std::unordered_map<std::string, double> gains_;

public:
    enum AutomationRate {
        RATE_K = 0, RATE_A = 1
    };

    struct ParamEvent {
        int type; // 0 = set, 1 = linearRamp
        int rate; // AutomationRate
        int64_t timeNs;
        double value;
    };
private:
    std::unordered_map<std::string, std::vector<ParamEvent>> scheduledGainEvents_;
    std::unordered_map<std::string, std::unordered_map<int, std::vector<ParamEvent>>> scheduledPannerEvents_;
    std::unordered_map<std::string, std::unordered_map<int, std::vector<ParamEvent>>> scheduledListenerEvents_;
    std::mutex scheduledEventsMutex_;

public:
    void
    scheduleGainEvent(const std::string &gainId, int type, int rate, double value, int64_t timeNs);

    void schedulePannerEvent(const std::string &pannerId, int paramType, int type, int rate,
                             double value, int64_t timeNs);

    void cancelGainEvents(const std::string &gainId, int64_t timeNs);

    void cancelPannerEvents(const std::string &pannerId, int paramType, int64_t timeNs);

    void cancelAndHoldGainEvents(const std::string &gainId, int rate, double value, int64_t timeNs);

    void
    cancelAndHoldPannerEvents(const std::string &pannerId, int paramType, int rate, double value,
                              int64_t timeNs);

    std::vector<double>
    getPannerParamValues(const std::string &pannerId, int paramType, int64_t startNs,
                         double sampleRate, int frameCount);

    std::vector<double>
    getGainParamValues(const std::string &gainId, int64_t startNs, double sampleRate,
                       int frameCount);

    std::pair<std::vector<double>, std::vector<double>>
    getIIRCoefficients(const std::string &iirId);

#ifdef HAS_OBOE
    std::shared_ptr<oboe::AudioStream> stream_;
    oboe::AudioStreamCallback *callback_ = nullptr;
    int streamSampleRate_ = 48000;
    int streamChannels_ = 2;
#endif
    int desiredSampleRate_ = 0;
    double desiredLatencyHintSec_ = 0.0;
    int desiredDeviceId_ = 0; // oboe::kUnspecified

    std::unordered_map<std::string, BiquadCoeffs> biquads_;
    std::unordered_map<std::string, std::unordered_map<int, std::string>> voiceGainByOutput_;
    std::unordered_map<std::string, std::unordered_map<int, std::string>> voicePlaybackRateByOutput_;
    std::unordered_map<std::string, std::unordered_map<int, std::string>> voiceFilterByOutput_;
    struct IIRData {
        std::vector<double> feedforward;
        std::vector<double> feedback;
    };
    std::unordered_map<std::string, IIRData> iirs_;
    struct PeriodicWaveData {
        std::vector<double> real;
        std::vector<double> imag;
        bool disableNormalization = false;
    };
    std::unordered_map<std::string, PeriodicWaveData> periodicWaves_;
    struct WaveShaperData {
        std::vector<float> curve;
        std::string oversample = "none";
    };
    std::unordered_map<std::string, WaveShaperData> waveShapers_;
    std::unordered_map<std::string, std::unordered_map<int, std::string>> voicePannerByOutput_;
    enum PannerPanningModel {
        PANNING_EQUALPOWER = 0, PANNING_HRTF = 1
    };
    struct Panner {
        double positionX = 0.0;
        double positionY = 0.0;
        double positionZ = 0.0;
        double orientationX = 1.0;
        double orientationY = 0.0;
        double orientationZ = 0.0;
        double pan = 0.0;
        PannerDistanceModel distanceModel = DISTANCE_INVERSE;
        PannerPanningModel panningModel = PANNING_EQUALPOWER;
        double refDistance = 1.0;
        double maxDistance = 10000.0;
        double rolloffFactor = 1.0;
        double coneInnerAngle = 360.0;
        double coneOuterAngle = 360.0;
        double coneOuterGain = 0.0;
        std::string contextId;
    };
    std::unordered_map<std::string, Panner> panners_;

    struct Listener {
        double positionX = 0.0;
        double positionY = 0.0;
        double positionZ = 0.0;
        double forwardX = 0.0;
        double forwardY = 0.0;
        double forwardZ = -1.0;
        double upX = 0.0;
        double upY = 1.0;
        double upZ = 0.0;
    };
    Listener listener_;
    std::unordered_map<std::string, Listener> listeners_;

public:
    void scheduleListenerEvent(const std::string &contextId, int paramType, int type, int rate, double value, int64_t timeNs);

    void cancelListenerEvents(const std::string &contextId, int paramType, int64_t timeNs);

    void cancelAndHoldListenerEvents(const std::string &contextId, int paramType, int rate, double value, int64_t timeNs);

    std::vector<double> getListenerParamValues(const std::string &contextId, int paramType, int64_t startNs, double sampleRate, int frameCount);

    double getListenerParamValue(const std::string &contextId, int paramType);

    struct AnalyserData {
        int fftSize = 2048;
        int capacity = 2048;
        int capacityMask = 2047;
        std::vector<float> ring;
        std::atomic<int64_t> writeIdx{0};
        std::vector<float> smoothedMag;
        std::vector<float> dbCache;
        std::vector<float> byteScratch;
        double smoothingTimeConstant = 0.8;
        double minDecibels = -100.0;
        double maxDecibels = -30.0;

        void *fftCfg = nullptr;
        std::vector<float> window;
        std::vector<kiss_fft_cpx> fin;
        std::vector<kiss_fft_cpx> fout;
        std::vector<float> sampleScratch;

        int64_t spectrumFrameStamp = -1;
        int64_t dbCacheFrameStamp = -1;

        AnalyserData() = default;

        AnalyserData(AnalyserData &&other) noexcept
                : fftSize(other.fftSize),
                  capacity(other.capacity),
                  capacityMask(other.capacityMask),
                  ring(std::move(other.ring)),
                  writeIdx(other.writeIdx.load()),
                  smoothedMag(std::move(other.smoothedMag)),
                  dbCache(std::move(other.dbCache)),
                  byteScratch(std::move(other.byteScratch)),
                  smoothingTimeConstant(other.smoothingTimeConstant),
                  minDecibels(other.minDecibels),
                  maxDecibels(other.maxDecibels),
                  fftCfg(other.fftCfg),
                  window(std::move(other.window)),
                  fin(std::move(other.fin)),
                  fout(std::move(other.fout)),
                  sampleScratch(std::move(other.sampleScratch)),
                  spectrumFrameStamp(other.spectrumFrameStamp),
                  dbCacheFrameStamp(other.dbCacheFrameStamp) {
            other.fftCfg = nullptr;
        }

        AnalyserData &operator=(AnalyserData &&other) noexcept {
            if (this != &other) {
                if (fftCfg) { kiss_fft_free(fftCfg); fftCfg = nullptr; }
                fftSize = other.fftSize;
                capacity = other.capacity;
                capacityMask = other.capacityMask;
                ring = std::move(other.ring);
                writeIdx.store(other.writeIdx.load());
                smoothedMag = std::move(other.smoothedMag);
                dbCache = std::move(other.dbCache);
                byteScratch = std::move(other.byteScratch);
                smoothingTimeConstant = other.smoothingTimeConstant;
                minDecibels = other.minDecibels;
                maxDecibels = other.maxDecibels;
                fftCfg = other.fftCfg;
                other.fftCfg = nullptr;
                window = std::move(other.window);
                fin = std::move(other.fin);
                fout = std::move(other.fout);
                sampleScratch = std::move(other.sampleScratch);
                spectrumFrameStamp = other.spectrumFrameStamp;
                dbCacheFrameStamp = other.dbCacheFrameStamp;
            }
            return *this;
        }

        ~AnalyserData() {
            if (fftCfg) { kiss_fft_free(fftCfg); fftCfg = nullptr; }
        }

        AnalyserData(const AnalyserData &other) = delete;

        AnalyserData &operator=(const AnalyserData &other) = delete;
    };

    std::unordered_map<std::string, AnalyserData> analysers_;

    static void rebuildAnalyserCaches(AnalyserData &ad);

public:
    std::string createAnalyser(int fftSize = 2048, double smoothingTimeConstant = 0.8,
                               double minDecibels = -100.0, double maxDecibels = -30.0);

    void freeAnalyser(const std::string &id);

    std::vector<float> getAnalyserTimeDomainData(const std::string &id, int count);

    std::vector<float> getAnalyserFrequencyData(const std::string &id);

    int getAnalyserTimeDomainDataInto(const std::string &id, float *dst, int dstCount);

    int getAnalyserFrequencyDataInto(const std::string &id, float *dst, int dstCount);

    int getAnalyserByteTimeDomainDataInto(const std::string &id, uint8_t *dst, int dstCount);

    int getAnalyserByteFrequencyDataInto(const std::string &id, uint8_t *dst, int dstCount,
                                         float minDb, float maxDb);

    void setAnalyserDecibels(const std::string &id, double minDecibels, double maxDecibels);

    void setAnalyserFftSize(const std::string &id, int fftSize);

    void setAnalyserSmoothingTimeConstant(const std::string &id, double value);

    JavaVM *jvm_ = nullptr;
    jclass audioContextClassGlobalRef = nullptr;
    jmethodID onNativeBufferHeldMethod = nullptr;
    jmethodID onNativeBufferReleasedMethod = nullptr;

    std::unordered_map<std::string, int64_t> contextStartNanos_;
    std::mutex contextMutex_;
};

#endif // AUDIO_CONTEXT_H
