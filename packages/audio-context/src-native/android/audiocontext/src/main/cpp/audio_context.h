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
#include <condition_variable>
#include <thread>
#include <algorithm>

#include "kiss_fft.h"

struct HRTFConvolver;
class Convolver;

#ifdef HAS_OBOE

#include <oboe/Oboe.h>

#endif

class NativeEngine {
public:
    // The engine this thread's calls reach: an offline context's inside its Scope, else realtime().
    static NativeEngine &getInstance();

    static NativeEngine &realtime();

    // An OfflineAudioContext's own engine, with no stream; returns its context id.
    static std::string createOffline(int sampleRate, int channels);

    static void releaseOffline(const std::string &contextId);

    // The offline engine whose prefix `id` carries, if any.
    static std::shared_ptr<NativeEngine> offlineFor(const char *id);

    // Routes getInstance() on this thread to `engine` (null: realtime) for the scope's lifetime.
    struct Scope {
        explicit Scope(std::shared_ptr<NativeEngine> engine);
        ~Scope();
        Scope(const Scope &) = delete;
        Scope &operator=(const Scope &) = delete;
    private:
        std::shared_ptr<NativeEngine> engine_;
        NativeEngine *previous_;
    };

    const std::string &idPrefix() const { return idPrefix_; }

    bool isOffline() const { return offline_; }

    // Offline: renders `frames` context frames from `startFrame`, channel c into `out[c]`.
    // Commands queued since the last call apply from `startFrame`.
    void renderOfflineFrames(int64_t startFrame, int frames, const std::vector<float *> &out);

    void resume();

    void suspend();

    std::string decodeAudioData(const std::vector<uint8_t> &data);

    std::string createOscillator(const std::string &type, double frequency);

    void startOscillator(const std::string &id, const std::string &type, double frequency);

    void stopTrack(const std::string &id);

    // `when` is in the context's currentTime seconds; at or before now starts/stops immediately.
    void startOscillatorAt(const std::string &id, const std::string &type, double frequency,
                           const std::string &contextId, double when);

    void stopTrackAt(const std::string &id, const std::string &contextId, double when);

    void setOscillatorType(const std::string &id, const std::string &type);

    std::string createBufferSource(std::vector<int16_t> &&pcm, int sampleRate, int channels);

    std::string createBufferSourceFromExisting(const std::string &bufferId);

    bool hasBuffer(const std::string &id);

    void startBufferSource(const std::string &id, bool loop);

    // `offset` is in seconds into the buffer.
    void startBufferSourceAt(const std::string &id, bool loop, const std::string &contextId,
                             double when, double offset);

    void setJavaVM(JavaVM *vm);

    void configureStream(int sampleRate, double latencyHintSec);

    int getStreamSampleRate();

    int getStreamBufferSizeFrames();

    int getFramesPerBurst();

    void setOutputDeviceId(int deviceId);

    int getOutputDeviceId() const { return desiredDeviceId_; }

    std::string
    decodeAudioDataFromDirect(const uint8_t *data, size_t len, JNIEnv *env, jobject byteBuffer);

    std::string
    createBufferSourceFromDirect(const void *ptr, size_t byteLen, int sampleRate, int channels,
                                 int bytesPerSample, JNIEnv *env, jobject byteBuffer);

    std::string
    createBufferSourceFromPlanar(const std::vector<const float *> &channelPtrs, int frameCount,
                                 int sampleRate, int channels);

    void freeBuffer(const std::string &id);

    // Frees the buffer once no playing voice reads it (a source the app no longer
    // references may still be playing: a one-shot, or a loop).
    void freeBufferWhenIdle(const std::string &id);

    std::string createExternalPcmSource(int sampleRate, int channels);
    void configureExternalPcmSource(const std::string &id, int sampleRate, int channels);
    void pushPcmFrames(const std::string &id, const float *interleaved, size_t sampleCount);
    void endExternalPcmSource(const std::string &id);

    // An AudioWorkletNode over its ring (audio-worklet-ring.ts); offsets are in 4-byte words.
    std::string createWorklet(float *ring, const std::vector<int> &inputChannels,
                              const std::vector<int> &outputChannels, int slotWords, int header,
                              int inputCounts, int inputData, int outputCounts,
                              const std::vector<int> &outputData, int paramLengths, int paramData,
                              std::vector<std::string> paramIds);

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

    bool offline_ = false;
    std::string idPrefix_;
    int64_t offlineBaseNs_ = 0;
    std::mutex offlineRenderMutex_;
    std::vector<float> offlineScratch_;
    // Offline: the quantum being rendered and the end of the current render call, in frames.
    int64_t offlineFrame_ = 0;
    int64_t offlineLimitFrame_ = 0;
    // The render clock: CLOCK_MONOTONIC ns for realtime, virtual for offline.
    std::atomic<int64_t> nowNs_{0};
    // Realtime: the next block's first frame on the render thread's frame clock.
    int64_t renderClockNs_ = 0;

    void ensureStream();

    void stopStream();

    std::recursive_mutex mutex_;
    struct BufferData {
        std::vector<int16_t> pcm;
        bool isDirect = false;
        bool isPlanar = false;
        std::string javaBufferId;
        const void *directPtr = nullptr;
        std::vector<const float *> planarPtrs;
        int planarFrames = 0;
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

    static constexpr int kWorkletMaxPorts = 16;
    static constexpr int kWorkletMaxChannels = 8;

    struct WorkletNode {
        std::string id;
        float *words = nullptr;
        std::atomic<uint32_t> *header = nullptr;
        int slotWords = 0;
        int inputCounts = 0;
        int inputData = 0;
        int outputCounts = 0;
        int paramLengths = 0;
        int paramData = 0;
        int inputs = 0;
        int outputs = 0;
        int inChannels[kWorkletMaxPorts] = {};
        int outChannels[kWorkletMaxPorts] = {};
        int outputData[kWorkletMaxPorts] = {};
        std::vector<std::string> paramIds;
        // Each input at its channelCount, a quantum per channel: inputs x mixChannels x 128.
        int mixChannels = 1;
        std::vector<float> inMix;
        bool inActive[kWorkletMaxPorts] = {};
        // The quantum playing: outputs x kWorkletMaxChannels x 128.
        std::vector<float> outQ;
        uint64_t nextFrame = 0;
        uint32_t lowWater = UINT32_MAX;
        uint32_t window = 0;
        bool finished = false;
        bool drained = false;
        bool ended = false;
        // Offline sources: requests run ahead of the render to here.
        bool requesting = false;
        uint64_t requestFrame = 0;
    };

    struct Voice {
        enum Type {
            Oscillator, BufferSource, ExternalPCM
        } voiceType;
        std::string id;
        std::string waveform = "sine";
        double frequency = 440.0;
        double phase = 0.0;
        double phaseIncrement = 0.0;
        std::string bufferId;
        double position = 0.0;
        double increment = 1.0;
        bool loop = false;
        std::string playbackRateId;
        std::string periodicWaveId;
        bool playing = false;
        // CLOCK_MONOTONIC ns, the same clock as g_audioTimeNanos; 0 means unscheduled.
        int64_t startAtNs = 0;
        int64_t stopAtNs = 0;
        int bufferChannels = 0;
        int bufferSampleRate = 0;

        std::shared_ptr<ExternalRing> externalRing;
        std::vector<float> externalPrev;
        std::vector<float> externalCurr;
        double externalSubPos = 0.0;
        bool externalPrimed = false;
    };

    // A DelayNode's line; its delayTime is a gain-style param under the node's id.
    struct DelayLine {
        int channels = 0;
        int size = 0;
        int writePos = 0;
        // `channels` runs of `size` frames.
        std::vector<float> data;
    };
    std::unordered_map<std::string, DelayLine> delays_;

    std::unordered_map<std::string, std::shared_ptr<WorkletNode>> worklets_;

    // Pushes a request for the quantum at `frame`, with params from `paramValues` (params x 128)
    // or else from automation at `paramNs`; false if the ring is full.
    bool workletPush(WorkletNode &w, uint64_t frame, int64_t paramNs, bool withInputs,
                     const float *paramValues = nullptr);
    void workletReadSlot(WorkletNode &w, uint32_t read);
    void workletSilence(WorkletNode &w);
    // Realtime: pushes the quantum just filled and plays the newest response.
    void workletEndQuantum(WorkletNode &w, int64_t paramNs, const float *paramValues);
    // Offline: renders the quantum at offlineFrame_ into outQ, waiting for the Worker.
    void workletRenderOffline(WorkletNode &w, const float *paramValues);
    void removeWorklet(const std::string &id);

    std::unordered_map<std::string, BufferData> audioBuffers_;
    std::unordered_map<std::string, Voice> audioVoices_;
    // Buffers to free once no playing voice reads them; render thread only.
    std::vector<std::string> idleFrees_;

    void releaseBufferData(const std::string &id,
                           const std::shared_ptr<std::promise<void>> &completion = nullptr);

    bool bufferInUse(const std::string &id) const;
public:
    enum CommandType {
        CMD_CREATE_OSC,
        CMD_START_OSC,
        CMD_STOP_TRACK,
        CMD_STOP_TRACK_AT,
        CMD_SET_OSC_TYPE,
        CMD_CREATE_BUFFER_DIRECT,
        CMD_CREATE_BUFFER_PLANAR,
        CMD_CREATE_BUFFER_COPY,
        CMD_START_BUFFER,
        CMD_FREE_BUFFER,
        CMD_FREE_BUFFER_WHEN_IDLE,
        CMD_CREATE_GAIN,
        CMD_CREATE_DYNAMICS_COMPRESSOR,
        CMD_SET_GAIN,
        CMD_FREE_GAIN,
        CMD_CREATE_BIQUAD,
        CMD_SET_BIQUAD_PARAMS,
        CMD_FREE_BIQUAD,
        CMD_CREATE_PANNER,
        CMD_SET_PANNER_PARAMS,
        CMD_SET_LISTENER_PARAMS,
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
        CMD_CONFIGURE_EXTERNAL_PCM,
        CMD_PUSH_EXTERNAL_PCM,
        CMD_END_EXTERNAL_PCM
        ,
        CMD_ATTACH_PLAYBACK_RATE,
        CMD_DETACH_PLAYBACK_RATE,
        CMD_FREE_PLAYBACK_RATE,
        CMD_SET_PANNER_HRIR,
        CMD_SET_PANNER_PARTITION_SIZE,
        CMD_CREATE_WORKLET,
        CMD_CREATE_DELAY,
        CMD_CONNECT,
        CMD_DISCONNECT,
        CMD_NODE_CONFIG,
        // Stops source `id`'s voice; its graph node lives on as `targetId` when given.
        CMD_REPLACE_SOURCE,
        // A biquad's frequency, Q, gain and detune params, in paramIds.
        CMD_SET_BIQUAD_PARAM_IDS,
        // A convolver's response; null clears it.
        CMD_SET_CONVOLVER
    };

    // Graph node kinds; AudioContext.java's NODE_* match.
    enum NodeKind {
        NODE_SOURCE = 0, NODE_GAIN, NODE_DELAY, NODE_BIQUAD, NODE_IIR, NODE_PANNER, NODE_COMPRESSOR,
        NODE_WAVESHAPER, NODE_CONVOLVER, NODE_ANALYSER, NODE_SPLITTER, NODE_MERGER, NODE_WORKLET,
        NODE_DESTINATION
    };
    enum CountMode { COUNT_MAX = 0, COUNT_CLAMPED_MAX = 1, COUNT_EXPLICIT = 2 };
    enum Interpretation { INTERP_SPEAKERS = 0, INTERP_DISCRETE = 1 };
    // A connection's target: a node input, or a param (-1 gain-style, else a panner param type).
    static constexpr int kTargetNode = -2;
    static constexpr int kTargetGainParam = -1;

    struct Command {
        CommandType type;
        std::string id;
        std::string contextId;
        std::string waveform;
        double frequency = 0.0;
        int64_t timeNs = 0;
        double startOffset = 0.0;
        std::string gainId;
        std::string compressorThresholdId;
        std::string compressorKneeId;
        std::string compressorRatioId;
        std::string compressorAttackId;
        std::string compressorReleaseId;
        std::string compressorReductionId;
        std::string playbackRateId;
        double gainValue = 1.0;
        double maxDelayTime = 1.0;
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
        std::shared_ptr<std::vector<const float *>> planarPtrs;
        int planarFrames = 0;
        size_t byteLen = 0;
        int bytesPerSample = 4;
        int sampleRate = 0;
        int channels = 0;
        std::shared_ptr<std::vector<int16_t>> pcm;
        std::shared_ptr<std::vector<float>> pcmFloat;
        std::shared_ptr<ExternalRing> externalRing;
        std::shared_ptr<WorkletNode> worklet;
        std::shared_ptr<std::vector<float>> hrirLeft;
        std::shared_ptr<std::vector<float>> hrirRight;
        bool clearHrir = false;
        int pannerPartitionSize = 0;
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
        std::string pannerBiquadId;
        std::vector<std::string> paramIds;
        std::shared_ptr<Convolver> convolver;
        // CMD_CONNECT / CMD_DISCONNECT: id output -> targetId input (or param targetType).
        // Disconnect wildcards: empty targetId, negative indices.
        std::string targetId;
        int kind = 0;
        int ports = 1;
        int targetKind = 0;
        int targetPorts = 1;
        int targetType = kTargetNode;
        int channelCount = 0;
        int countMode = -1;
        int interpretation = -1;
    };

    void enqueueCommand(Command &&cmd);

private:
    static constexpr int kQuantum = 128;

    struct Bus {
        int channels = 1;
        // Whether anything is playing into it this quantum.
        bool active = false;
        std::vector<float> data;

        void setChannels(int n) {
            channels = std::max(1, n);
            const size_t need = static_cast<size_t>(channels) * kQuantum;
            if (data.size() < need) data.resize(need);
        }

        float *channel(int c) { return data.data() + static_cast<size_t>(c) * kQuantum; }

        const float *channel(int c) const { return data.data() + static_cast<size_t>(c) * kQuantum; }

        void silence() { std::fill(data.begin(), data.begin() + channels * kQuantum, 0.0f); }
    };

    struct GraphNode {
        int kind = NODE_GAIN;
        std::string id;
        int inputs = 1;
        int outputs = 1;
        int channelCount = 2;
        int countMode = COUNT_MAX;
        int interpretation = INTERP_SPEAKERS;
        // Per input: (source, source output).
        std::vector<std::vector<std::pair<GraphNode *, int>>> in;
        std::vector<Bus> inBus;
        std::vector<Bus> outBus;
        std::shared_ptr<WorkletNode> worklet;
        std::vector<BiquadState> biquad;
        std::vector<std::vector<double>> iir;
        std::vector<BiquadState> lowpass;
        BiquadCoeffs lowpassCoeffs;
        double lowpassCutoff = -1.0;
        std::vector<float> reductionDb;
        // Param values for this quantum, kQuantum per param.
        std::vector<double> params;
        std::vector<float> paramValues;
        int delayChannels = 1;
        // Quanta it keeps ringing with nothing playing into it.
        int tail = 0;
        // A DelayNode in a cycle: its output is read before its input is written.
        bool delayInCycle = false;
        // In a cycle without a DelayNode: silent.
        bool muted = false;
        // Freed by the app; goes once nothing plays into it, then runs `deferred`.
        bool released = false;
        std::vector<Command> deferred;
        int index = -1;
        int low = 0;
        bool onStack = false;
        int pending = 0;
    };

    struct ParamInput {
        std::string id;
        int type;
        GraphNode *src;
        int output;
        // The node reading the param, found when the order is rebuilt.
        GraphNode *owner;
    };

    std::unordered_map<std::string, std::unique_ptr<GraphNode>> graph_;
    std::vector<ParamInput> paramInputs_;
    std::vector<GraphNode *> order_;
    std::vector<GraphNode *> destinations_;
    std::vector<GraphNode *> scratchNodes_;
    bool orderDirty_ = true;
    // Realtime: the quantum being played out, interleaved.
    std::vector<float> quantum_;
    int quantumPos_ = kQuantum;
    std::vector<std::string> endedScratch_;
    // Frees deferred until a node left the graph.
    std::vector<Command> deferredRun_;
    // Buffer source id -> its playbackRate param.
    std::unordered_map<std::string, std::string> voicePlaybackRate_;

    GraphNode *ensureNode(const std::string &id, int kind, int ports);
    void removeNode(GraphNode *node);
    void connectNodes(const Command &c);
    void disconnectNodes(const Command &c);
    void configureNode(const Command &c);
    bool deferFree(const Command &c);
    bool nodeBusy(const GraphNode &n) const;
    void runDeferred();
    double *paramScratch(GraphNode &n, int index);
    void rebuildOrder();
    void applyCommand(Command &c);
    void drainCommands();
    // Renders one quantum starting at `startNs` into `out`, interleaved at the stream's channels.
    void renderQuantum(float *out, int64_t startNs);
    void processNode(GraphNode &n, int64_t startNs, double frameNs);
    bool mixInput(GraphNode &n, int input, int channels, int mode, int interpretation);
    // A quantum of a param: automation plus audio connected to it; true when constant.
    bool evalParam(const std::string &id, int type, double fallback, int64_t startNs,
                   double frameNs, double *out);
    void renderSource(GraphNode &n, int64_t startNs, double frameNs);
    void renderDelay(GraphNode &n, int64_t startNs, double frameNs, bool write, bool read);
    void renderPanner(GraphNode &n, int64_t startNs, double frameNs);
    void renderCompressor(GraphNode &n, int64_t startNs, double frameNs);
    void renderWorklet(GraphNode &n, int64_t startNs, double frameNs);
    void renderBiquad(GraphNode &n, int64_t startNs, double frameNs, int channels, bool active);
    void renderConvolver(GraphNode &n);

    std::vector<Command> pendingCommands_;
    std::mutex commandMutex_;

    std::unordered_map<std::string, double> gains_;

    struct DynamicsCompressor {
        std::string thresholdId;
        std::string kneeId;
        std::string ratioId;
        std::string attackId;
        std::string releaseId;
        std::string reductionId;
    };
    std::unordered_map<std::string, DynamicsCompressor> compressors_;

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
    // A BiquadFilterNode: its type and frequency, Q, gain and detune params.
    struct BiquadFilter {
        std::string type = "lowpass";
        std::string paramIds[4];
        double last[4] = {-1.0, -1.0, -1.0, -1.0};
        BiquadCoeffs coeffs;
    };
    std::unordered_map<std::string, BiquadFilter> biquadFilters_;
    std::unordered_map<std::string, std::shared_ptr<Convolver>> convolvers_;
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
        HRTFConvolver *hrtf = nullptr;
        int hrtfPartitionSize = 0;
        std::string biquadId;
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
    std::mutex analyserMutex_;
    std::atomic<size_t> analyserCount_{0};

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
    jmethodID onNativeTrackEndedMethod = nullptr;

    int64_t contextTimeToNanos(const std::string &contextId, double when);

    // Voices that stop on the render thread are reported to Java from this thread, so the
    // audio callback never attaches to the JVM.
    void queueEnded(const std::vector<std::string> &ids);
    void endedNotifierLoop();
    std::mutex endedMutex_;
    std::condition_variable endedCv_;
    std::vector<std::string> endedQueue_;
    std::thread endedThread_;
    bool endedStop_ = false;

    std::unordered_map<std::string, int64_t> contextStartNanos_;
    std::mutex contextMutex_;
};

#endif // AUDIO_CONTEXT_H
