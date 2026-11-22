// AudioWorkletNode sources: the C++ side of crates/audio-context-napi/src/worklet.rs's ring.
#include "audio_context.h"
#include "audio_internal.h"

#include <algorithm>
#include <cstring>

namespace {
constexpr int kQuantum = 128;
constexpr uint32_t kSlots = 64;
constexpr uint32_t kMaxBacklog = 48;
constexpr uint32_t kShrinkWindow = 128;
constexpr int kMaxChannels = 8;
// Header words after the slots (audio-worklet-ring.ts).
enum { REQUESTED = 0, DONE = 1, READ = 2, CLOSED = 3, NATIVE_MARK = 5 };
enum { ALIVE = 2 };
}

std::string NativeEngine::createWorkletSource(float *ring, int channels, int slotWords, int header,
                                              int outputCounts, int outputData, int paramLengths,
                                              int paramData, std::vector<std::string> paramIds) {
    if (!ring) return "";
    auto worklet = std::make_shared<WorkletRing>();
    worklet->words = ring;
    worklet->header = reinterpret_cast<std::atomic<uint32_t> *>(reinterpret_cast<uint32_t *>(ring) + header);
    worklet->channels = std::max(1, std::min(channels, kMaxChannels));
    worklet->slotWords = slotWords;
    worklet->outputCounts = outputCounts;
    worklet->outputData = outputData;
    worklet->paramLengths = paramLengths;
    worklet->paramData = paramData;
    worklet->paramIds = std::move(paramIds);
    worklet->out.assign(static_cast<size_t>(worklet->channels * kQuantum), 0.0f);
    // Lets JS check the runtime handed over the SharedArrayBuffer itself, not a copy.
    worklet->header[NATIVE_MARK].store(0x574B4C54u, std::memory_order_release);

    auto external = std::make_shared<ExternalRing>();
    external->capacity = 1u << 13;
    external->mask = external->capacity - 1;
    external->channels = worklet->channels;
    external->data.assign(static_cast<size_t>(external->capacity) * worklet->channels, 0.0f);

    std::string id = genId();
    Command cmd;
    cmd.type = CMD_CREATE_WORKLET;
    cmd.id = id;
    cmd.channels = worklet->channels;
    cmd.externalRing = external;
    cmd.worklet = worklet;
    enqueueCommand(std::move(cmd));
    return id;
}

static float paramValueAt(const std::vector<NativeEngine::ParamEvent> *events, int64_t ns, double fallback) {
    return events ? getScheduledGainValueAt(*events, ns, fallback) : static_cast<float>(fallback);
}

void NativeEngine::produceWorkletQuanta(Voice &voice, int frames, int64_t startNs, double frameNs) {
    if (!voice.worklet || !voice.externalRing) return;
    WorkletRing &w = *voice.worklet;
    ExternalRing &ring = *voice.externalRing;
    if (ring.ended.load(std::memory_order_acquire)) return;
    uint32_t write = ring.writeIdx.load(std::memory_order_relaxed);
    uint32_t buffered = write - ring.readIdx.load(std::memory_order_acquire);
    if (buffered >= static_cast<uint32_t>(frames)) return;

    std::lock_guard<std::mutex> lock(scheduledEventsMutex_);
    while (buffered < static_cast<uint32_t>(frames) && ring.capacity - buffered >= kQuantum) {
        auto quantumNs = startNs + static_cast<int64_t>(buffered * frameNs);
        if (!w.finished) {
            uint32_t requested = w.header[REQUESTED].load(std::memory_order_relaxed);
            uint32_t read = w.header[READ].load(std::memory_order_acquire);
            if (requested - read < kSlots) {
                float *slot = w.words + (requested % kSlots) * static_cast<uint32_t>(w.slotWords);
                auto *words = reinterpret_cast<uint32_t *>(slot);
                double frame = static_cast<double>(w.nextFrame);
                std::memcpy(slot, &frame, sizeof frame);
                words[ALIVE] = 1;
                words[w.outputCounts] = static_cast<uint32_t>(w.channels);
                for (size_t p = 0; p < w.paramIds.size(); p++) {
                    const std::string &id = w.paramIds[p];
                    auto git = gains_.find(id);
                    double fallback = git != gains_.end() ? git->second : 0.0;
                    auto eit = scheduledGainEvents_.find(id);
                    const std::vector<ParamEvent> *events =
                            eit != scheduledGainEvents_.end() && !eit->second.empty() ? &eit->second : nullptr;
                    float *values = slot + w.paramData + p * kQuantum;
                    uint32_t length = 1;
                    values[0] = paramValueAt(events, quantumNs, fallback);
                    if (events && (*events)[0].rate == RATE_A) {
                        for (int i = 1; i < kQuantum; i++) {
                            values[i] = paramValueAt(events, quantumNs + static_cast<int64_t>(i * frameNs), fallback);
                            if (values[i] != values[0]) length = kQuantum;
                        }
                    }
                    words[w.paramLengths + p] = length;
                }
                w.header[REQUESTED].store(requested + 1, std::memory_order_release);
            }
        }

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
            float *slot = w.words + (read % kSlots) * static_cast<uint32_t>(w.slotWords);
            auto *words = reinterpret_cast<uint32_t *>(slot);
            uint32_t count = std::min(words[w.outputCounts], static_cast<uint32_t>(w.channels));
            for (int c = 0; c < w.channels; c++) {
                float *dst = w.out.data() + c * kQuantum;
                if (static_cast<uint32_t>(c) < count) std::memcpy(dst, slot + w.outputData + c * kQuantum, kQuantum * sizeof(float));
                else std::memset(dst, 0, kQuantum * sizeof(float));
            }
            if (words[ALIVE] == 0) w.finished = true;
            read++;
        } else {
            std::fill(w.out.begin(), w.out.end(), 0.0f);
        }
        w.header[READ].store(read, std::memory_order_release);

        for (int i = 0; i < kQuantum; i++) {
            uint32_t pos = (write + i) & ring.mask;
            for (int c = 0; c < w.channels; c++) {
                ring.data[static_cast<size_t>(pos) * w.channels + c] = w.out[static_cast<size_t>(c * kQuantum + i)];
            }
        }
        write += kQuantum;
        buffered += kQuantum;
        w.nextFrame += kQuantum;
    }
    ring.writeIdx.store(write, std::memory_order_release);
    // process() returned false: play out what's queued, then end like a finished stream.
    if (w.finished && w.header[DONE].load(std::memory_order_acquire) == w.header[READ].load(std::memory_order_relaxed)) {
        ring.ended.store(true, std::memory_order_release);
    }
}
