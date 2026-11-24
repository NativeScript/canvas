// AudioWorkletNode: the C++ side of crates/audio-context-napi/src/worklet.rs's ring.
#include "audio_context.h"
#include "audio_internal.h"

#include <algorithm>
#include <chrono>
#include <cmath>
#include <cstring>
#include <unistd.h>

namespace {
constexpr int kQuantum = 128;
constexpr uint32_t kSlots = 64;
constexpr uint32_t kMaxBacklog = 48;
constexpr uint32_t kShrinkWindow = 128;
// Offline: a Worker answering nothing for this long renders silence.
constexpr auto kOfflineStall = std::chrono::seconds(5);
// Header words after the slots (audio-worklet-ring.ts).
enum { REQUESTED = 0, DONE = 1, READ = 2, CLOSED = 3, NATIVE_MARK = 5 };
enum { ALIVE = 2 };

std::string outputVoiceId(const std::string &id, int output) {
    return output == 0 ? id : id + "#" + std::to_string(output);
}
}

std::string NativeEngine::createWorklet(float *ring, const std::vector<int> &inputChannels,
                                        const std::vector<int> &outputChannels, int slotWords,
                                        int header, int inputCounts, int inputData,
                                        int outputCounts, const std::vector<int> &outputData,
                                        int paramLengths, int paramData,
                                        std::vector<std::string> paramIds) {
    const int inputs = static_cast<int>(inputChannels.size());
    const int outputs = static_cast<int>(outputChannels.size());
    if (!ring || inputs > kWorkletMaxPorts || outputs > kWorkletMaxPorts || inputs + outputs == 0 ||
        outputData.size() != outputChannels.size()) {
        return "";
    }
    auto w = std::make_shared<WorkletNode>();
    w->id = genId();
    w->words = ring;
    w->header = reinterpret_cast<std::atomic<uint32_t> *>(reinterpret_cast<uint32_t *>(ring) + header);
    w->slotWords = slotWords;
    w->inputCounts = inputCounts;
    w->inputData = inputData;
    w->outputCounts = outputCounts;
    w->paramLengths = paramLengths;
    w->paramData = paramData;
    w->inputs = inputs;
    w->outputs = outputs;
    for (int i = 0; i < inputs; i++) w->inChannels[i] = std::max(1, std::min(inputChannels[i], kWorkletMaxChannels));
    for (int o = 0; o < outputs; o++) {
        w->outChannels[o] = std::max(1, std::min(outputChannels[o], kWorkletMaxChannels));
        w->outputData[o] = outputData[o];
    }
    w->paramIds = std::move(paramIds);
#ifdef HAS_OBOE
    w->mixChannels = std::max(1, std::min(streamChannels_, kWorkletMaxChannels));
#endif
    w->inMix.assign(static_cast<size_t>(std::max(inputs, 1) * w->mixChannels * kQuantum), 0.0f);
    w->outQ.assign(static_cast<size_t>(std::max(outputs, 1) * kWorkletMaxChannels * kQuantum), 0.0f);
    // Lets JS check the runtime handed over the SharedArrayBuffer itself, not a copy.
    w->header[NATIVE_MARK].store(0x574B4C54u, std::memory_order_release);

    std::string id = w->id;
    Command cmd;
    cmd.type = CMD_CREATE_WORKLET;
    cmd.id = id;
    cmd.worklet = std::move(w);
    enqueueCommand(std::move(cmd));
    return id;
}

void NativeEngine::setWorkletOutputConnected(const std::string &id, int output, bool connected) {
    Command cmd;
    cmd.type = CMD_WORKLET_OUTPUT;
    cmd.id = id;
    cmd.outputIndex = output;
    cmd.loop = connected;
    enqueueCommand(std::move(cmd));
}

void NativeEngine::removeWorklet(const std::string &id) {
    auto it = worklets_.find(id);
    if (it == worklets_.end()) return;
    WorkletNode &w = *it->second;
    w.header[CLOSED].store(1, std::memory_order_release);
    // Voices routed into it stay routed, and silent.
    for (int o = 0; o < w.outputs; o++) audioVoices_.erase(outputVoiceId(id, o));
    worklets_.erase(it);
}

static float paramValueAt(const std::vector<NativeEngine::ParamEvent> *events, int64_t ns, double fallback) {
    return events ? getScheduledGainValueAt(*events, ns, fallback) : static_cast<float>(fallback);
}

bool NativeEngine::workletPush(WorkletNode &w, uint64_t frame, int64_t paramNs, bool withInputs) {
    uint32_t requested = w.header[REQUESTED].load(std::memory_order_relaxed);
    uint32_t read = w.header[READ].load(std::memory_order_acquire);
    if (requested - read >= kSlots) return false;
    float *slot = w.words + (requested % kSlots) * static_cast<uint32_t>(w.slotWords);
    auto *words = reinterpret_cast<uint32_t *>(slot);
    double frameValue = static_cast<double>(frame);
    std::memcpy(slot, &frameValue, sizeof frameValue);
    words[ALIVE] = 1;

    // Web Audio "speakers" mixing from the context's channels to the input's channelCount.
    const int have = w.mixChannels;
    for (int i = 0; i < w.inputs; i++) {
        const int want = w.inChannels[i];
        const uint32_t count = withInputs && w.inActive[i] ? static_cast<uint32_t>(want) : 0u;
        words[w.inputCounts + i] = count;
        const float *mix = w.inMix.data() + static_cast<size_t>(i * have * kQuantum);
        for (int c = 0; c < static_cast<int>(count); c++) {
            float *dst = slot + w.inputData + (i * kWorkletMaxChannels + c) * kQuantum;
            if (want == 1 && have > 1) {
                const float scale = 1.0f / static_cast<float>(have);
                for (int k = 0; k < kQuantum; k++) {
                    float sum = 0.0f;
                    for (int h = 0; h < have; h++) sum += mix[h * kQuantum + k];
                    dst[k] = sum * scale;
                }
            } else if (c < have) {
                std::memcpy(dst, mix + c * kQuantum, kQuantum * sizeof(float));
            } else if (have == 1) {
                std::memcpy(dst, mix, kQuantum * sizeof(float));
            } else {
                std::memset(dst, 0, kQuantum * sizeof(float));
            }
        }
    }
    for (int o = 0; o < w.outputs; o++) words[w.outputCounts + o] = static_cast<uint32_t>(w.outChannels[o]);

    if (!w.paramIds.empty()) {
#ifdef HAS_OBOE
        const double frameNs = 1e9 / static_cast<double>(streamSampleRate_ > 0 ? streamSampleRate_ : 48000);
#else
        const double frameNs = 1e9 / 48000.0;
#endif
        std::lock_guard<std::mutex> lock(scheduledEventsMutex_);
        for (size_t p = 0; p < w.paramIds.size(); p++) {
            const std::string &id = w.paramIds[p];
            auto git = gains_.find(id);
            double fallback = git != gains_.end() ? git->second : 0.0;
            auto eit = scheduledGainEvents_.find(id);
            const std::vector<ParamEvent> *events =
                    eit != scheduledGainEvents_.end() && !eit->second.empty() ? &eit->second : nullptr;
            float *values = slot + w.paramData + p * kQuantum;
            uint32_t length = 1;
            values[0] = paramValueAt(events, paramNs, fallback);
            if (events && (*events)[0].rate == RATE_A) {
                for (int k = 1; k < kQuantum; k++) {
                    values[k] = paramValueAt(events, paramNs + static_cast<int64_t>(k * frameNs), fallback);
                    if (values[k] != values[0]) length = kQuantum;
                }
            }
            words[w.paramLengths + p] = length;
        }
    }
    w.header[REQUESTED].store(requested + 1, std::memory_order_release);
    return true;
}

void NativeEngine::workletReadSlot(WorkletNode &w, uint32_t read) {
    float *slot = w.words + (read % kSlots) * static_cast<uint32_t>(w.slotWords);
    auto *words = reinterpret_cast<uint32_t *>(slot);
    for (int o = 0; o < w.outputs; o++) {
        uint32_t count = std::min(words[w.outputCounts + o], static_cast<uint32_t>(w.outChannels[o]));
        for (int c = 0; c < w.outChannels[o]; c++) {
            float *dst = w.outQ.data() + (o * kWorkletMaxChannels + c) * kQuantum;
            if (static_cast<uint32_t>(c) < count) std::memcpy(dst, slot + w.outputData[o] + c * kQuantum, kQuantum * sizeof(float));
            else std::memset(dst, 0, kQuantum * sizeof(float));
        }
    }
    if (words[ALIVE] == 0) w.finished = true;
}

void NativeEngine::workletSilence(WorkletNode &w) {
    std::fill(w.outQ.begin(), w.outQ.end(), 0.0f);
}

template<typename Worklet>
static void clearInputs(Worklet &w) {
    std::fill(w.inMix.begin(), w.inMix.end(), 0.0f);
    std::fill(std::begin(w.inActive), std::end(w.inActive), false);
}

void NativeEngine::workletEndQuantum(WorkletNode &w, int64_t paramNs) {
    // process() returned false: a source ends once its last quantum has played.
    if (w.inputs == 0 && w.drained) w.ended = true;
    if (!w.finished && w.header[CLOSED].load(std::memory_order_acquire) == 0) {
        workletPush(w, w.nextFrame, paramNs, true);
    }
    w.nextFrame += kQuantum;

    // The newest responses, shrinking a backlog a stall left behind.
    uint32_t read = w.header[READ].load(std::memory_order_relaxed);
    uint32_t done = w.header[DONE].load(std::memory_order_acquire);
    uint32_t backlog = done - read;
    if (backlog > kMaxBacklog) {
        read = done - kMaxBacklog;
    } else {
        uint32_t low = std::min(w.lowWater, backlog);
        if (++w.window >= kShrinkWindow) {
            if (low >= 2) read += std::max((low - 1) / 2, 1u);
            w.window = 0;
            w.lowWater = UINT32_MAX;
        } else {
            w.lowWater = low;
        }
    }
    if (done != read) {
        workletReadSlot(w, read);
        read++;
    } else {
        workletSilence(w);
    }
    w.header[READ].store(read, std::memory_order_release);
    w.drained = w.finished && done == read;
    clearInputs(w);
    w.position = 0;
}

void NativeEngine::workletRenderOffline(WorkletNode &w) {
    const auto frame = static_cast<uint64_t>(offlineFrame_);
#ifdef HAS_OBOE
    const double frameNs = 1e9 / static_cast<double>(streamSampleRate_ > 0 ? streamSampleRate_ : 48000);
#else
    const double frameNs = 1e9 / 48000.0;
#endif
    auto paramNs = [&](uint64_t f) {
        return offlineBaseNs_ + std::llround(static_cast<double>(f) * frameNs) + 4;
    };
    if (w.inputs == 0 && w.drained) w.ended = true;
    auto closed = [&] { return w.header[CLOSED].load(std::memory_order_acquire) != 0; };
    if (!w.finished && !closed()) {
        if (w.inputs > 0) {
            // This quantum's inputs exist only now.
            workletPush(w, frame, paramNs(frame), true);
        } else {
            // A source's requests run ahead to the end of this render call; params can't change before it.
            if (!w.requesting || w.requestFrame < frame) {
                w.requesting = true;
                w.requestFrame = frame;
            }
            const auto limit = static_cast<uint64_t>(std::max(offlineLimitFrame_, offlineFrame_ + kQuantum));
            while (w.requestFrame < limit && workletPush(w, w.requestFrame, paramNs(w.requestFrame), false)) {
                w.requestFrame += kQuantum;
            }
        }
        const auto deadline = std::chrono::steady_clock::now() + kOfflineStall;
        while (w.header[DONE].load(std::memory_order_acquire) == w.header[READ].load(std::memory_order_relaxed) &&
               w.header[REQUESTED].load(std::memory_order_relaxed) != w.header[READ].load(std::memory_order_relaxed) &&
               !closed()) {
            if (std::chrono::steady_clock::now() > deadline) {
                __android_log_print(ANDROID_LOG_ERROR, TAG, "AudioWorkletNode: the Worker stopped answering; rendering silence");
                w.finished = true;
                w.header[CLOSED].store(1, std::memory_order_release);
                break;
            }
            usleep(100);
        }
    }
    // The oldest response, never skipped.
    uint32_t read = w.header[READ].load(std::memory_order_relaxed);
    uint32_t done = w.header[DONE].load(std::memory_order_acquire);
    if (done != read) {
        workletReadSlot(w, read);
        read++;
        w.header[READ].store(read, std::memory_order_release);
    } else {
        workletSilence(w);
    }
    w.drained = w.finished && done == read;
    w.nextFrame = frame + kQuantum;
    clearInputs(w);
}
