// The render graph: nodes with inputs and outputs, rendered a quantum at a time in topological order.
#include "audio_context.h"
#include "audio_internal.h"
#include "hrtf_convolver.h"

#include <android/log.h>
#include <algorithm>
#include <cmath>
#include <cstring>
#include <functional>

#ifdef HAS_OBOE

namespace {
constexpr int Q = 128;
constexpr int kMaxChannels = 32;
// Quanta a filter keeps ringing after its input stops.
constexpr int kFilterTail = 40;

double eventValueAt(const std::vector<NativeEngine::ParamEvent> &evts, int64_t tNs, double fallback) {
    auto it = std::upper_bound(evts.begin(), evts.end(), tNs,
                               [](int64_t time, const NativeEngine::ParamEvent &e) { return time < e.timeNs; });
    size_t idx = static_cast<size_t>(std::distance(evts.begin(), it));
    if (idx == 0) return fallback;
    if (idx >= evts.size()) return evts.back().value;
    const auto &prev = evts[idx - 1];
    const auto &next = evts[idx];
    if (next.type == 1) {
        if (next.timeNs <= prev.timeNs) return next.value;
        double ratio = static_cast<double>(tNs - prev.timeNs) / static_cast<double>(next.timeNs - prev.timeNs);
        ratio = std::max(0.0, std::min(1.0, ratio));
        return prev.value + (next.value - prev.value) * ratio;
    }
    return prev.value;
}

// Fills a quantum of automation; true when it holds one value.
bool fillAutomation(const std::vector<NativeEngine::ParamEvent> *ev, double fallback, int64_t startNs,
                    double frameNs, double *out) {
    if (!ev || ev->empty()) {
        std::fill(out, out + Q, fallback);
        return true;
    }
    if ((*ev)[0].rate == NativeEngine::RATE_K) {
        std::fill(out, out + Q, eventValueAt(*ev, startNs, fallback));
        return true;
    }
    bool constant = true;
    for (int f = 0; f < Q; f++) {
        out[f] = eventValueAt(*ev, startNs + static_cast<int64_t>(f * frameNs), fallback);
        if (out[f] != out[0]) constant = false;
    }
    return constant;
}

// Mono down-mix of frame f.
inline float monoAt(const float *const *ch, int channels, int f) {
    if (channels == 1) return ch[0][f];
    return speakerMix([&](int c) { return ch[c][f]; }, channels, 1, 0);
}

double valueOr(const std::unordered_map<std::string, double> &values, const std::string &id, double fallback) {
    auto it = values.find(id);
    return it != values.end() ? it->second : fallback;
}
}

NativeEngine::GraphNode *NativeEngine::ensureNode(const std::string &id, int kind, int ports) {
    auto it = graph_.find(id);
    if (it != graph_.end()) return it->second.get();
    auto node = std::make_unique<GraphNode>();
    node->id = id;
    node->kind = kind;
    const int count = std::max(1, std::min(ports, kMaxChannels));
    switch (kind) {
        case NODE_SOURCE:
            node->inputs = 0;
            break;
        case NODE_SPLITTER:
            node->outputs = count;
            node->channelCount = count;
            node->countMode = COUNT_EXPLICIT;
            node->interpretation = INTERP_DISCRETE;
            break;
        case NODE_MERGER:
            node->inputs = count;
            node->channelCount = 1;
            node->countMode = COUNT_EXPLICIT;
            break;
        case NODE_PANNER:
        case NODE_COMPRESSOR:
        case NODE_CONVOLVER:
            node->countMode = COUNT_CLAMPED_MAX;
            break;
        case NODE_DESTINATION:
            node->channelCount = std::max(1, streamChannels_);
            node->countMode = COUNT_EXPLICIT;
            break;
        case NODE_WORKLET: {
            auto w = worklets_.find(id);
            if (w != worklets_.end()) {
                node->worklet = w->second;
                node->inputs = w->second->inputs;
                node->outputs = w->second->outputs;
                node->countMode = COUNT_EXPLICIT;
            }
            break;
        }
        default:
            break;
    }
    node->in.resize(static_cast<size_t>(node->inputs));
    node->inBus.resize(static_cast<size_t>(node->inputs));
    node->outBus.resize(static_cast<size_t>(node->outputs));
    for (auto &b: node->inBus) b.setChannels(1);
    for (auto &b: node->outBus) {
        b.setChannels(1);
        b.silence();
    }
    GraphNode *raw = node.get();
    graph_[id] = std::move(node);
    orderDirty_ = true;
    return raw;
}

void NativeEngine::removeNode(GraphNode *node) {
    if (!node) return;
    for (auto &kv: graph_) {
        for (auto &conns: kv.second->in) {
            conns.erase(std::remove_if(conns.begin(), conns.end(),
                                       [&](const std::pair<GraphNode *, int> &e) { return e.first == node; }),
                        conns.end());
        }
    }
    paramInputs_.erase(std::remove_if(paramInputs_.begin(), paramInputs_.end(),
                                      [&](const ParamInput &p) { return p.src == node || p.owner == node; }),
                       paramInputs_.end());
    for (auto &c: node->deferred) deferredRun_.push_back(std::move(c));
    std::string id = node->id;
    graph_.erase(id);
    orderDirty_ = true;
}

void NativeEngine::connectNodes(const Command &c) {
    GraphNode *src = ensureNode(c.id, c.kind, c.ports);
    if (c.outputIndex < 0 || c.outputIndex >= src->outputs) return;
    if (c.targetType == kTargetNode) {
        GraphNode *dst = ensureNode(c.targetId, c.targetKind, c.targetPorts);
        if (c.inputIndex < 0 || c.inputIndex >= dst->inputs) return;
        auto &conns = dst->in[static_cast<size_t>(c.inputIndex)];
        const std::pair<GraphNode *, int> edge{src, c.outputIndex};
        if (std::find(conns.begin(), conns.end(), edge) == conns.end()) conns.push_back(edge);
    } else {
        for (const auto &p: paramInputs_) {
            if (p.src == src && p.output == c.outputIndex && p.type == c.targetType && p.id == c.targetId) return;
        }
        paramInputs_.push_back({c.targetId, c.targetType, src, c.outputIndex, nullptr});
    }
    orderDirty_ = true;
}

void NativeEngine::disconnectNodes(const Command &c) {
    auto sit = graph_.find(c.id);
    if (sit == graph_.end()) return;
    GraphNode *src = sit->second.get();
    auto outputMatches = [&](int output) { return c.outputIndex < 0 || output == c.outputIndex; };
    if (c.targetType == kTargetNode) {
        GraphNode *dst = nullptr;
        if (!c.targetId.empty()) {
            auto dit = graph_.find(c.targetId);
            if (dit == graph_.end()) return;
            dst = dit->second.get();
        }
        for (auto &kv: graph_) {
            GraphNode *n = kv.second.get();
            if (dst && n != dst) continue;
            for (int i = 0; i < n->inputs; i++) {
                if (c.inputIndex >= 0 && i != c.inputIndex) continue;
                auto &conns = n->in[static_cast<size_t>(i)];
                conns.erase(std::remove_if(conns.begin(), conns.end(),
                                           [&](const std::pair<GraphNode *, int> &e) {
                                               return e.first == src && outputMatches(e.second);
                                           }),
                            conns.end());
            }
        }
    }
    // disconnect() and disconnect(output) cut param connections too.
    if (c.targetType != kTargetNode || c.targetId.empty()) {
        paramInputs_.erase(std::remove_if(paramInputs_.begin(), paramInputs_.end(),
                                          [&](const ParamInput &p) {
                                              if (p.src != src || !outputMatches(p.output)) return false;
                                              return c.targetType == kTargetNode ||
                                                     (p.type == c.targetType && p.id == c.targetId);
                                          }),
                           paramInputs_.end());
    }
    orderDirty_ = true;
}

void NativeEngine::configureNode(const Command &c) {
    GraphNode *n = ensureNode(c.id, c.kind, c.ports);
    if (c.channelCount > 0) n->channelCount = std::min(c.channelCount, kMaxChannels);
    if (c.countMode >= 0) n->countMode = c.countMode;
    if (c.interpretation >= 0) n->interpretation = c.interpretation;
}

bool NativeEngine::nodeBusy(const GraphNode &n) const {
    switch (n.kind) {
        case NODE_SOURCE:
            return audioVoices_.count(n.id) > 0;
        case NODE_WORKLET:
            return worklets_.count(n.id) > 0;
        case NODE_DESTINATION:
            return false;
        default:
            break;
    }
    if (n.tail > 0) return true;
    for (const auto &conns: n.in) {
        if (!conns.empty()) return true;
    }
    for (const auto &p: paramInputs_) {
        if (p.owner == &n) return true;
    }
    return false;
}

bool NativeEngine::deferFree(const Command &c) {
    auto it = graph_.find(c.id);
    if (it == graph_.end()) return false;
    GraphNode *n = it->second.get();
    if (nodeBusy(*n)) {
        // Still playing: it goes, and its state with it, once nothing plays into it.
        n->released = true;
        n->deferred.push_back(c);
        return true;
    }
    removeNode(n);
    return false;
}

void NativeEngine::runDeferred() {
    while (!deferredRun_.empty()) {
        std::vector<Command> cmds;
        cmds.swap(deferredRun_);
        for (auto &c: cmds) applyCommand(c);
    }
}

void NativeEngine::rebuildOrder() {
    order_.clear();
    destinations_.clear();
    std::vector<GraphNode *> nodes;
    nodes.reserve(graph_.size());
    for (auto &kv: graph_) {
        GraphNode *n = kv.second.get();
        n->index = static_cast<int>(nodes.size());
        n->onStack = false;
        n->delayInCycle = false;
        n->muted = false;
        nodes.push_back(n);
        if (n->kind == NODE_DESTINATION) destinations_.push_back(n);
    }
    std::vector<int> visit(nodes.size(), -1);

    // Each param input's owner: the node that reads that param.
    auto ownerOf = [&](const ParamInput &p) -> GraphNode * {
        auto it = graph_.find(p.id);
        if (it != graph_.end()) {
            const int k = it->second->kind;
            if (p.type == kTargetGainParam && (k == NODE_GAIN || k == NODE_DELAY || k == NODE_SOURCE)) return it->second.get();
            if (p.type != kTargetGainParam && k == NODE_PANNER) return it->second.get();
        }
        if (p.type != kTargetGainParam) return nullptr;
        for (const auto &kv: voicePlaybackRate_) {
            if (kv.second == p.id) {
                auto v = graph_.find(kv.first);
                if (v != graph_.end()) return v->second.get();
            }
        }
        for (const auto &kv: compressors_) {
            const DynamicsCompressor &d = kv.second;
            if (d.thresholdId == p.id || d.kneeId == p.id || d.ratioId == p.id || d.attackId == p.id ||
                d.releaseId == p.id) {
                auto v = graph_.find(kv.first);
                if (v != graph_.end()) return v->second.get();
            }
        }
        for (GraphNode *n: nodes) {
            if (n->worklet &&
                std::find(n->worklet->paramIds.begin(), n->worklet->paramIds.end(), p.id) != n->worklet->paramIds.end()) {
                return n;
            }
        }
        return nullptr;
    };
    for (auto &p: paramInputs_) p.owner = ownerOf(p);

    // deps[n]: the nodes n reads this quantum, through an input or a param.
    std::vector<std::vector<GraphNode *>> deps(nodes.size());
    for (GraphNode *n: nodes) {
        for (const auto &conns: n->in) {
            for (const auto &e: conns) deps[static_cast<size_t>(n->index)].push_back(e.first);
        }
    }
    for (const auto &p: paramInputs_) {
        if (p.owner) deps[static_cast<size_t>(p.owner->index)].push_back(p.src);
    }

    // Tarjan's strongly connected components: a cycle plays only through a DelayNode.
    int counter = 0;
    std::vector<GraphNode *> stack;
    std::function<void(GraphNode *)> strong = [&](GraphNode *v) {
        const size_t vi = static_cast<size_t>(v->index);
        visit[vi] = counter;
        v->low = counter++;
        stack.push_back(v);
        v->onStack = true;
        bool selfLoop = false;
        for (GraphNode *w: deps[vi]) {
            if (w == v) selfLoop = true;
            const size_t wi = static_cast<size_t>(w->index);
            if (visit[wi] < 0) {
                strong(w);
                v->low = std::min(v->low, w->low);
            } else if (w->onStack) {
                v->low = std::min(v->low, visit[wi]);
            }
        }
        if (v->low != visit[vi]) return;
        std::vector<GraphNode *> component;
        GraphNode *w;
        do {
            w = stack.back();
            stack.pop_back();
            w->onStack = false;
            component.push_back(w);
        } while (w != v);
        if (component.size() == 1 && !selfLoop) return;
        bool hasDelay = false;
        for (GraphNode *n: component) {
            if (n->kind == NODE_DELAY) {
                n->delayInCycle = true;
                hasDelay = true;
            }
        }
        if (!hasDelay) {
            for (GraphNode *n: component) n->muted = true;
        }
    };
    for (GraphNode *n: nodes) {
        if (visit[static_cast<size_t>(n->index)] < 0) strong(n);
    }

    // Kahn's order; a cycle's DelayNode reads before its input is known, and muted nodes are silent.
    std::vector<std::vector<GraphNode *>> readers(nodes.size());
    for (GraphNode *n: nodes) {
        n->pending = 0;
        if (n->muted) continue;
        const size_t ni = static_cast<size_t>(n->index);
        std::vector<GraphNode *> reads = deps[ni];
        if (n->delayInCycle) {
            reads.clear();
            for (const auto &p: paramInputs_) {
                if (p.owner == n) reads.push_back(p.src);
            }
        }
        for (GraphNode *d: reads) {
            if (d->muted) continue;
            n->pending++;
            readers[static_cast<size_t>(d->index)].push_back(n);
        }
    }
    std::vector<GraphNode *> ready;
    for (GraphNode *n: nodes) {
        if (n->pending == 0) ready.push_back(n);
    }
    while (!ready.empty()) {
        GraphNode *n = ready.back();
        ready.pop_back();
        order_.push_back(n);
        for (GraphNode *r: readers[static_cast<size_t>(n->index)]) {
            if (--r->pending == 0) ready.push_back(r);
        }
    }
    if (order_.size() < nodes.size()) {
        for (GraphNode *n: nodes) {
            if (n->pending > 0) {
                n->muted = true;
                order_.push_back(n);
            }
        }
    }
}

bool NativeEngine::mixInput(GraphNode &n, int input, int channels, int mode, int interpretation) {
    Bus &dst = n.inBus[static_cast<size_t>(input)];
    int maxChannels = 0;
    for (const auto &e: n.in[static_cast<size_t>(input)]) {
        const Bus &b = e.first->outBus[static_cast<size_t>(e.second)];
        if (b.active) maxChannels = std::max(maxChannels, b.channels);
    }
    if (maxChannels == 0) {
        dst.setChannels(mode == COUNT_EXPLICIT ? channels : 1);
        dst.active = false;
        dst.silence();
        return false;
    }
    int count = maxChannels;
    if (mode == COUNT_CLAMPED_MAX) count = std::min(maxChannels, channels);
    else if (mode == COUNT_EXPLICIT) count = channels;
    dst.setChannels(std::max(1, std::min(count, kMaxChannels)));
    dst.active = true;
    dst.silence();
    for (const auto &e: n.in[static_cast<size_t>(input)]) {
        const Bus &b = e.first->outBus[static_cast<size_t>(e.second)];
        if (!b.active) continue;
        if (b.channels == dst.channels || interpretation == INTERP_DISCRETE) {
            const int shared = std::min(b.channels, dst.channels);
            for (int c = 0; c < shared; c++) {
                const float *s = b.channel(c);
                float *d = dst.channel(c);
                for (int f = 0; f < Q; f++) d[f] += s[f];
            }
            continue;
        }
        for (int c = 0; c < dst.channels; c++) {
            float *d = dst.channel(c);
            for (int f = 0; f < Q; f++) {
                d[f] += speakerMix([&](int k) { return b.channel(k)[f]; }, b.channels, dst.channels, c);
            }
        }
    }
    return true;
}

bool NativeEngine::evalParam(const std::string &id, int type, double fallback, int64_t startNs, double frameNs,
                             double *out) {
    bool constant;
    {
        std::lock_guard<std::mutex> lock(scheduledEventsMutex_);
        const std::vector<ParamEvent> *ev = nullptr;
        if (type == kTargetGainParam) {
            auto it = scheduledGainEvents_.find(id);
            if (it != scheduledGainEvents_.end()) ev = &it->second;
        } else {
            auto it = scheduledPannerEvents_.find(id);
            if (it != scheduledPannerEvents_.end()) {
                auto jt = it->second.find(type);
                if (jt != it->second.end()) ev = &jt->second;
            }
        }
        constant = fillAutomation(ev, fallback, startNs, frameNs, out);
    }
    // Audio connected to the param adds to it, mixed down to mono.
    for (const auto &p: paramInputs_) {
        if (p.type != type || p.id != id) continue;
        const Bus &b = p.src->outBus[static_cast<size_t>(p.output)];
        if (!b.active) continue;
        const float *ch[kMaxChannels];
        for (int c = 0; c < b.channels; c++) ch[c] = b.channel(c);
        for (int f = 0; f < Q; f++) out[f] += monoAt(ch, b.channels, f);
        constant = false;
    }
    return constant;
}

double *NativeEngine::paramScratch(GraphNode &n, int index) {
    const size_t need = static_cast<size_t>(index + 1) * Q;
    if (n.params.size() < need) n.params.resize(need);
    return n.params.data() + static_cast<size_t>(index) * Q;
}

void NativeEngine::renderSource(GraphNode &n, int64_t startNs, double frameNs) {
    Bus &o = n.outBus[0];
    auto vit = audioVoices_.find(n.id);
    if (vit == audioVoices_.end() || !vit->second.playing) {
        o.setChannels(1);
        o.active = false;
        o.silence();
        return;
    }
    Voice &v = vit->second;
    const double sr = streamSampleRate_ > 0 ? static_cast<double>(streamSampleRate_) : 48000.0;
    const BufferData *bd = nullptr;
    int channels = 1;
    int bufFrames = 0;
    if (v.voiceType == Voice::BufferSource) {
        auto bit = audioBuffers_.find(v.bufferId);
        if (bit != audioBuffers_.end()) {
            bd = &bit->second;
            channels = std::max(1, bd->channels);
            if (bd->isPlanar && bd->planarFrames > 0) bufFrames = bd->planarFrames;
            else if (bd->isDirect) bufFrames = static_cast<int>((bd->byteLength / std::max(1, bd->bytesPerSample)) / channels);
            else bufFrames = static_cast<int>(bd->pcm.size() / static_cast<size_t>(channels));
        }
    } else if (v.voiceType == Voice::ExternalPCM) {
        channels = 0;
        const auto &ring = v.externalRing;
        if (ring && ring->capacity > 0 && !ring->data.empty()) {
            int resolved = ring->channels > 0 ? ring->channels : v.bufferChannels;
            if (v.bufferChannels > 0) resolved = std::min(resolved, v.bufferChannels);
            resolved = std::min(resolved, static_cast<int>(std::min(v.externalPrev.size(), v.externalCurr.size())));
            resolved = std::min(resolved, static_cast<int>(ring->data.size() / ring->capacity));
            channels = std::max(0, resolved);
        }
    }
    o.setChannels(std::max(1, channels));
    o.silence();
    o.active = false;

    if ((v.voiceType == Voice::BufferSource && (!bd || bufFrames <= 0)) ||
        (v.voiceType == Voice::ExternalPCM && channels <= 0)) {
        if (v.voiceType == Voice::ExternalPCM) {
            v.externalPrimed = false;
            const auto &ring = v.externalRing;
            if (!ring || (ring->ended.load(std::memory_order_acquire) &&
                          ring->writeIdx.load(std::memory_order_acquire) == ring->readIdx.load(std::memory_order_relaxed))) {
                v.playing = false;
                endedScratch_.push_back(v.id);
            }
        }
        return;
    }

    // Params: a constant source's offset, an oscillator's frequency, a buffer's playbackRate.
    const bool constant = v.voiceType == Voice::Oscillator && v.waveform == "constant";
    double *param = paramScratch(n, 0);
    if (v.voiceType == Voice::Oscillator) {
        evalParam(v.id, kTargetGainParam, valueOr(gains_, v.id, constant ? 1.0 : v.frequency), startNs, frameNs, param);
    } else if (v.voiceType == Voice::BufferSource && !v.playbackRateId.empty()) {
        evalParam(v.playbackRateId, kTargetGainParam, valueOr(gains_, v.playbackRateId, 1.0), startNs, frameNs, param);
    } else {
        std::fill(param, param + Q, 1.0);
    }
    const PeriodicWaveData *wave = nullptr;
    if (v.voiceType == Voice::Oscillator && !v.periodicWaveId.empty()) {
        auto pw = periodicWaves_.find(v.periodicWaveId);
        if (pw != periodicWaves_.end()) wave = &pw->second;
    }
    enum { SINE, SQUARE, SAW, TRIANGLE } shape = SINE;
    if (v.waveform == "square") shape = SQUARE;
    else if (v.waveform == "sawtooth") shape = SAW;
    else if (v.waveform == "triangle") shape = TRIANGLE;

    for (int f = 0; f < Q; f++) {
        const int64_t tNs = startNs + static_cast<int64_t>(f * frameNs);
        if (v.stopAtNs > 0 && tNs >= v.stopAtNs) {
            v.playing = false;
            break;
        }
        if (v.startAtNs > 0) {
            if (tNs < v.startAtNs) continue;
            v.startAtNs = 0;
        }
        o.active = true;
        if (v.voiceType == Voice::Oscillator) {
            double s;
            if (constant) {
                s = param[f];
            } else {
                const double freq = param[f];
                if (wave) {
                    s = computePeriodicWaveSample(*wave, v.phase, freq, sr);
                } else if (shape == SQUARE) {
                    s = v.phase < 0.5 ? 1.0 : -1.0;
                } else if (shape == SAW) {
                    s = 2.0 * v.phase - 1.0;
                } else if (shape == TRIANGLE) {
                    const double p = v.phase;
                    s = p < 0.25 ? 4.0 * p : (p < 0.75 ? 2.0 - 4.0 * p : 4.0 * p - 4.0);
                } else {
                    s = std::sin(v.phase * 2.0 * M_PI);
                }
                v.phase += freq / sr;
                v.phase -= std::floor(v.phase);
            }
            o.channel(0)[f] = static_cast<float>(s);
        } else if (v.voiceType == Voice::BufferSource) {
            const int i0 = static_cast<int>(std::floor(v.position));
            const float frac = static_cast<float>(v.position - i0);
            const int s0 = ((i0 % bufFrames) + bufFrames) % bufFrames;
            const int s1 = (s0 + 1) % bufFrames;
            for (int c = 0; c < channels; c++) {
                float a = 0.0f, b = 0.0f;
                if (bd->isPlanar && !bd->planarPtrs.empty()) {
                    const float *p = bd->planarPtrs[static_cast<size_t>(std::min(c, static_cast<int>(bd->planarPtrs.size()) - 1))];
                    if (p) {
                        a = p[s0];
                        b = p[s1];
                    }
                } else if (bd->isDirect && bd->directPtr) {
                    if (bd->bytesPerSample == 4) {
                        const auto *p = reinterpret_cast<const float *>(bd->directPtr);
                        a = p[s0 * channels + c];
                        b = p[s1 * channels + c];
                    } else {
                        const auto *p = reinterpret_cast<const int16_t *>(bd->directPtr);
                        a = static_cast<float>(p[s0 * channels + c]) / 32768.0f;
                        b = static_cast<float>(p[s1 * channels + c]) / 32768.0f;
                    }
                } else if (!bd->pcm.empty()) {
                    a = static_cast<float>(bd->pcm[static_cast<size_t>(s0 * channels + c)]) / 32768.0f;
                    b = static_cast<float>(bd->pcm[static_cast<size_t>(s1 * channels + c)]) / 32768.0f;
                }
                o.channel(c)[f] = a * (1.0f - frac) + b * frac;
            }
            v.position += v.increment * std::max(0.0, param[f]);
            if (v.position >= bufFrames) {
                if (v.loop) {
                    v.position = std::fmod(v.position, static_cast<double>(bufFrames));
                } else {
                    v.playing = false;
                    break;
                }
            }
        } else {
            auto &ring = *v.externalRing;
            uint32_t w = ring.writeIdx.load(std::memory_order_acquire);
            uint32_t r = ring.readIdx.load(std::memory_order_relaxed);
            if (!v.externalPrimed && w - r >= 1) {
                const uint32_t p = r & ring.mask;
                for (int c = 0; c < channels; c++) v.externalCurr[static_cast<size_t>(c)] = ring.data[static_cast<size_t>(p) * channels + c];
                ring.readIdx.store(r + 1, std::memory_order_release);
                v.externalPrimed = true;
            }
            if (!v.externalPrimed) {
                if (ring.ended.load(std::memory_order_acquire) && w == r) {
                    v.playing = false;
                    break;
                }
                continue;
            }
            const auto t = static_cast<float>(v.externalSubPos);
            for (int c = 0; c < channels; c++) {
                o.channel(c)[f] = v.externalPrev[static_cast<size_t>(c)] * (1.0f - t) + v.externalCurr[static_cast<size_t>(c)] * t;
            }
            v.externalSubPos += v.increment;
            while (v.externalSubPos >= 1.0) {
                for (int c = 0; c < channels; c++) v.externalPrev[static_cast<size_t>(c)] = v.externalCurr[static_cast<size_t>(c)];
                w = ring.writeIdx.load(std::memory_order_acquire);
                r = ring.readIdx.load(std::memory_order_relaxed);
                if (w - r >= 1) {
                    const uint32_t p = r & ring.mask;
                    for (int c = 0; c < channels; c++) v.externalCurr[static_cast<size_t>(c)] = ring.data[static_cast<size_t>(p) * channels + c];
                    ring.readIdx.store(r + 1, std::memory_order_release);
                    v.externalSubPos -= 1.0;
                } else {
                    v.externalSubPos = 0.999;
                    if (ring.ended.load(std::memory_order_acquire)) v.playing = false;
                    break;
                }
            }
            if (!v.playing) break;
        }
    }
    if (!v.playing) endedScratch_.push_back(v.id);
}

void NativeEngine::renderDelay(GraphNode &n, int64_t startNs, double frameNs, bool write, bool read) {
    Bus &o = n.outBus[0];
    auto dit = delays_.find(n.id);
    if (dit == delays_.end()) {
        if (read) {
            o.setChannels(1);
            o.active = false;
            o.silence();
        }
        return;
    }
    DelayLine &d = dit->second;
    if (write) {
        const bool active = mixInput(n, 0, n.channelCount, n.countMode, n.interpretation);
        const Bus &in = n.inBus[0];
        if (active) {
            n.delayChannels = std::min(in.channels, d.channels);
            n.tail = d.size / Q + 2;
        } else if (n.tail > 0) {
            n.tail--;
        }
        for (int c = 0; c < d.channels; c++) {
            float *line = d.data.data() + static_cast<size_t>(c) * d.size;
            const float *src = active && c < in.channels ? in.channel(c) : nullptr;
            for (int f = 0; f < Q; f++) line[(d.writePos + f) % d.size] = src ? src[f] : 0.0f;
        }
    }
    if (read) {
        o.setChannels(n.delayChannels);
        o.active = n.tail > 0;
        if (!o.active) {
            o.silence();
        } else {
            const double sr = streamSampleRate_ > 0 ? static_cast<double>(streamSampleRate_) : 48000.0;
            double *time = paramScratch(n, 0);
            evalParam(n.id, kTargetGainParam, valueOr(gains_, n.id, 0.0), startNs, frameNs, time);
            // In a cycle the delay is at least a quantum, as in the spec.
            const double minFrames = n.delayInCycle ? Q : 0.0;
            const double maxFrames = std::max(minFrames, static_cast<double>(d.size - Q - 2));
            for (int f = 0; f < Q; f++) {
                double frames = std::max(minFrames, std::min(time[f] * sr, maxFrames));
                // A delayTime of whole frames, give or take rounding, reads those frames exactly.
                if (std::fabs(frames - std::round(frames)) < 1e-6) frames = std::round(frames);
                double pos = static_cast<double>(d.writePos + f) - frames;
                while (pos < 0.0) pos += d.size;
                const int i0 = static_cast<int>(pos) % d.size;
                const double frac = pos - std::floor(pos);
                const int i1 = (i0 + 1) % d.size;
                for (int c = 0; c < o.channels; c++) {
                    const float *line = d.data.data() + static_cast<size_t>(c) * d.size;
                    o.channel(c)[f] = frac == 0.0 ? line[i0] : static_cast<float>(line[i0] * (1.0 - frac) + line[i1] * frac);
                }
            }
        }
    }
    if (write) d.writePos = (d.writePos + Q) % d.size;
}

void NativeEngine::renderPanner(GraphNode &n, int64_t startNs, double frameNs) {
    const bool active = mixInput(n, 0, n.channelCount, n.countMode, n.interpretation);
    Bus &in = n.inBus[0];
    Bus &o = n.outBus[0];
    o.setChannels(2);
    o.active = active;
    o.silence();
    if (!active) return;
    auto pit = panners_.find(n.id);
    if (pit == panners_.end()) {
        for (int c = 0; c < 2; c++) {
            for (int f = 0; f < Q; f++) {
                o.channel(c)[f] = speakerMix([&](int k) { return in.channel(k)[f]; }, in.channels, 2, c);
            }
        }
        return;
    }
    const Panner &base = pit->second;
    const double sr = streamSampleRate_ > 0 ? static_cast<double>(streamSampleRate_) : 48000.0;

    // Panner params, then listener params, a quantum each.
    static constexpr int kPannerParams[] = {kPannerParamPositionX, kPannerParamPositionY, kPannerParamPositionZ,
                                            kPannerParamOrientationX, kPannerParamOrientationY,
                                            kPannerParamOrientationZ, kPannerParamPan};
    const double pannerBase[] = {base.positionX, base.positionY, base.positionZ, base.orientationX,
                                 base.orientationY, base.orientationZ, base.pan};
    bool constant = true;
    double *values[16];
    for (int i = 0; i < 7; i++) {
        values[i] = paramScratch(n, i);
        constant &= evalParam(n.id, kPannerParams[i], pannerBase[i], startNs, frameNs, values[i]);
    }
    const Listener *listenerBase = &listener_;
    if (!base.contextId.empty()) {
        auto lit = listeners_.find(base.contextId);
        if (lit != listeners_.end()) listenerBase = &lit->second;
    }
    const double listenerValues[] = {listenerBase->positionX, listenerBase->positionY, listenerBase->positionZ,
                                     listenerBase->forwardX, listenerBase->forwardY, listenerBase->forwardZ,
                                     listenerBase->upX, listenerBase->upY, listenerBase->upZ};
    {
        std::lock_guard<std::mutex> lock(scheduledEventsMutex_);
        auto lit = scheduledListenerEvents_.find(base.contextId);
        for (int i = 0; i < 9; i++) {
            values[7 + i] = paramScratch(n, 7 + i);
            const std::vector<ParamEvent> *ev = nullptr;
            if (lit != scheduledListenerEvents_.end()) {
                auto jt = lit->second.find(kListenerParamPositionX + i);
                if (jt != lit->second.end()) ev = &jt->second;
            }
            constant &= fillAutomation(ev, listenerValues[i], startNs, frameNs, values[7 + i]);
        }
    }

    const bool stereo = in.channels > 1;
    const bool hrtf = base.panningModel == PANNING_HRTF && base.hrtf != nullptr;
    Panner p = base;
    Listener L = *listenerBase;
    double leftGain = 1.0, rightGain = 1.0, dist = 1.0;
    bool steerLeft = true;
    auto resolve = [&](int f) {
        p.positionX = values[0][f];
        p.positionY = values[1][f];
        p.positionZ = values[2][f];
        p.orientationX = values[3][f];
        p.orientationY = values[4][f];
        p.orientationZ = values[5][f];
        p.pan = values[6][f];
        L.positionX = values[7][f];
        L.positionY = values[8][f];
        L.positionZ = values[9][f];
        L.forwardX = values[10][f];
        L.forwardY = values[11][f];
        L.forwardZ = values[12][f];
        L.upX = values[13][f];
        L.upY = values[14][f];
        L.upZ = values[15][f];
        computePannerGains(p, L, stereo && !hrtf, leftGain, rightGain, dist, &steerLeft);
    };
    resolve(0);

    // Distance-dependent air absorption, when the panner has its low-pass.
    const bool lowpass = !base.biquadId.empty();
    if (lowpass) {
        const double g = std::max(0.0, std::min(1.0, dist));
        const double cutoff = 800.0 + (22050.0 - 800.0) * std::sqrt(g);
        if (std::fabs(cutoff - n.lowpassCutoff) >= 1.0) {
            n.lowpassCoeffs = computeBiquadCoeffs("lowpass", cutoff, 0.707, 0.0, static_cast<int>(sr));
            n.lowpassCutoff = cutoff;
        }
        if (n.lowpass.size() < static_cast<size_t>(in.channels)) n.lowpass.resize(static_cast<size_t>(in.channels));
        for (int c = 0; c < in.channels; c++) {
            float *x = in.channel(c);
            for (int f = 0; f < Q; f++) x[f] = processBiquadSample(n.lowpassCoeffs, n.lowpass[static_cast<size_t>(c)], x[f]);
        }
    }

    float *outL = o.channel(0);
    float *outR = o.channel(1);
    const float *inL = in.channel(0);
    const float *inR = stereo ? in.channel(1) : inL;
    for (int f = 0; f < Q; f++) {
        if (!constant && f > 0) resolve(f);
        if (hrtf) {
            float mono = stereo ? 0.5f * (inL[f] + inR[f]) : inL[f];
            float l = mono, r = mono;
            base.hrtf->process(&mono, 1, &l, &r);
            outL[f] = static_cast<float>(l * dist);
            outR[f] = static_cast<float>(r * dist);
        } else if (!stereo) {
            outL[f] = static_cast<float>(inL[f] * leftGain * dist);
            outR[f] = static_cast<float>(inL[f] * rightGain * dist);
        } else if (steerLeft) {
            outL[f] = static_cast<float>((inL[f] + inR[f] * leftGain) * dist);
            outR[f] = static_cast<float>(inR[f] * rightGain * dist);
        } else {
            outL[f] = static_cast<float>(inL[f] * leftGain * dist);
            outR[f] = static_cast<float>((inR[f] + inL[f] * rightGain) * dist);
        }
    }
}

void NativeEngine::renderCompressor(GraphNode &n, int64_t startNs, double frameNs) {
    const bool active = mixInput(n, 0, n.channelCount, n.countMode, n.interpretation);
    Bus &in = n.inBus[0];
    Bus &o = n.outBus[0];
    o.setChannels(in.channels);
    o.active = active;
    if (!active) {
        o.silence();
        return;
    }
    auto cit = compressors_.find(n.id);
    if (cit == compressors_.end()) {
        for (int c = 0; c < in.channels; c++) std::memcpy(o.channel(c), in.channel(c), Q * sizeof(float));
        return;
    }
    const DynamicsCompressor &comp = cit->second;
    const double sr = streamSampleRate_ > 0 ? static_cast<double>(streamSampleRate_) : 48000.0;
    double *threshold = paramScratch(n, 0);
    double *knee = paramScratch(n, 1);
    double *ratio = paramScratch(n, 2);
    double *attack = paramScratch(n, 3);
    double *release = paramScratch(n, 4);
    evalParam(comp.thresholdId, kTargetGainParam, valueOr(gains_, comp.thresholdId, -24.0), startNs, frameNs, threshold);
    evalParam(comp.kneeId, kTargetGainParam, valueOr(gains_, comp.kneeId, 30.0), startNs, frameNs, knee);
    evalParam(comp.ratioId, kTargetGainParam, valueOr(gains_, comp.ratioId, 12.0), startNs, frameNs, ratio);
    evalParam(comp.attackId, kTargetGainParam, valueOr(gains_, comp.attackId, 0.003), startNs, frameNs, attack);
    evalParam(comp.releaseId, kTargetGainParam, valueOr(gains_, comp.releaseId, 0.25), startNs, frameNs, release);
    if (n.reductionDb.size() < static_cast<size_t>(in.channels)) n.reductionDb.resize(static_cast<size_t>(in.channels), 0.0f);
    for (int f = 0; f < Q; f++) {
        const double r = std::max(1.0, ratio[f]);
        const double k = std::max(0.0, knee[f]);
        const double t = threshold[f];
        const double attackCoeff = std::exp(-1.0 / (std::max(attack[f], 1.0 / sr) * sr));
        const double releaseCoeff = std::exp(-1.0 / (std::max(release[f], 1.0 / sr) * sr));
        for (int c = 0; c < in.channels; c++) {
            const float x = in.channel(c)[f];
            const double level = 20.0 * std::log10(std::max(1.0e-12, std::fabs(static_cast<double>(x))));
            const double over = level - t;
            double compressed = level;
            if (k > 0.0) {
                const double half = 0.5 * k;
                if (over >= half) compressed = t + over / r;
                else if (over > -half) compressed = level + (1.0 / r - 1.0) * (over + half) * (over + half) / (2.0 * k);
            } else if (over > 0.0) {
                compressed = t + over / r;
            }
            const double target = std::max(0.0, level - compressed);
            double current = n.reductionDb[static_cast<size_t>(c)];
            const double coeff = target > current ? attackCoeff : releaseCoeff;
            current = coeff * current + (1.0 - coeff) * target;
            n.reductionDb[static_cast<size_t>(c)] = static_cast<float>(current);
            o.channel(c)[f] = static_cast<float>(x * std::pow(10.0, -current / 20.0));
        }
    }
    if (!comp.reductionId.empty()) {
        float most = 0.0f;
        for (int c = 0; c < in.channels; c++) most = std::max(most, n.reductionDb[static_cast<size_t>(c)]);
        gains_[comp.reductionId] = -most;
    }
}

void NativeEngine::renderWorklet(GraphNode &n, int64_t startNs, double frameNs) {
    if (!n.worklet) {
        for (auto &b: n.outBus) {
            b.active = false;
            b.silence();
        }
        return;
    }
    WorkletNode &w = *n.worklet;
    for (int i = 0; i < w.inputs && i < n.inputs; i++) {
        const bool active = mixInput(n, i, w.inChannels[i], COUNT_EXPLICIT, n.interpretation);
        w.inActive[i] = active;
        if (!active) continue;
        const Bus &in = n.inBus[static_cast<size_t>(i)];
        for (int c = 0; c < std::min(in.channels, w.mixChannels); c++) {
            std::memcpy(w.inMix.data() + static_cast<size_t>((i * w.mixChannels + c) * Q), in.channel(c), Q * sizeof(float));
        }
    }
    // Param values, with any audio connected to them.
    const size_t params = w.paramIds.size();
    const float *paramValues = nullptr;
    if (params > 0) {
        if (n.paramValues.size() < params * Q) n.paramValues.resize(params * Q);
        double *scratch = paramScratch(n, 0);
        for (size_t p = 0; p < params; p++) {
            evalParam(w.paramIds[p], kTargetGainParam, valueOr(gains_, w.paramIds[p], 0.0), startNs, frameNs, scratch);
            for (int f = 0; f < Q; f++) n.paramValues[p * Q + f] = static_cast<float>(scratch[f]);
        }
        paramValues = n.paramValues.data();
    }
    auto copyOut = [&] {
        for (int o = 0; o < w.outputs && o < n.outputs; o++) {
            Bus &b = n.outBus[static_cast<size_t>(o)];
            b.setChannels(w.outChannels[o]);
            b.active = !w.ended;
            for (int c = 0; c < b.channels; c++) {
                std::memcpy(b.channel(c), w.outQ.data() + static_cast<size_t>((o * kWorkletMaxChannels + c) * Q), Q * sizeof(float));
            }
        }
    };
    if (offline_) {
        workletRenderOffline(w, w.inputs > 0 ? paramValues : nullptr);
        copyOut();
    } else {
        // Realtime plays the newest response and requests the next quantum with these inputs.
        copyOut();
        workletEndQuantum(w, startNs + static_cast<int64_t>(Q * frameNs), paramValues);
    }
}

void NativeEngine::processNode(GraphNode &n, int64_t startNs, double frameNs) {
    if (n.muted) {
        for (auto &b: n.outBus) {
            b.setChannels(1);
            b.active = false;
            b.silence();
        }
        return;
    }
    switch (n.kind) {
        case NODE_SOURCE:
            renderSource(n, startNs, frameNs);
            return;
        case NODE_DELAY:
            renderDelay(n, startNs, frameNs, !n.delayInCycle, true);
            return;
        case NODE_PANNER:
            renderPanner(n, startNs, frameNs);
            return;
        case NODE_COMPRESSOR:
            renderCompressor(n, startNs, frameNs);
            return;
        case NODE_WORKLET:
            renderWorklet(n, startNs, frameNs);
            return;
        case NODE_DESTINATION:
            mixInput(n, 0, n.channelCount, COUNT_EXPLICIT, n.interpretation);
            return;
        case NODE_SPLITTER: {
            const bool active = mixInput(n, 0, n.outputs, COUNT_EXPLICIT, INTERP_DISCRETE);
            const Bus &in = n.inBus[0];
            for (int o = 0; o < n.outputs; o++) {
                Bus &b = n.outBus[static_cast<size_t>(o)];
                b.setChannels(1);
                b.active = active;
                if (active && o < in.channels) std::memcpy(b.channel(0), in.channel(o), Q * sizeof(float));
                else b.silence();
            }
            return;
        }
        case NODE_MERGER: {
            Bus &b = n.outBus[0];
            b.setChannels(n.inputs);
            b.active = false;
            for (int i = 0; i < n.inputs; i++) {
                const bool active = mixInput(n, i, 1, COUNT_EXPLICIT, n.interpretation);
                b.active |= active;
                std::memcpy(b.channel(i), n.inBus[static_cast<size_t>(i)].channel(0), Q * sizeof(float));
            }
            return;
        }
        default:
            break;
    }

    // One input, one output, at the input's channels.
    const bool active = mixInput(n, 0, n.channelCount, n.countMode, n.interpretation);
    const Bus &in = n.inBus[0];
    Bus &o = n.outBus[0];
    const bool filter = n.kind == NODE_BIQUAD || n.kind == NODE_IIR;
    if (filter) {
        if (active) {
            n.tail = kFilterTail;
            n.delayChannels = in.channels;
        } else if (n.tail > 0) {
            n.tail--;
        }
    }
    const bool ringing = filter && !active && n.tail > 0;
    const int channels = active ? in.channels : (ringing ? n.delayChannels : 1);
    o.setChannels(channels);
    o.active = active || ringing;
    if (!o.active) {
        o.silence();
        if (n.kind == NODE_ANALYSER) {
            std::lock_guard<std::mutex> lock(analyserMutex_);
            auto it = analysers_.find(n.id);
            if (it != analysers_.end() && it->second.capacity > 0) {
                AnalyserData &ad = it->second;
                for (int f = 0; f < Q; f++) ad.ring[static_cast<size_t>(ad.writeIdx.fetch_add(1, std::memory_order_relaxed) % ad.capacity)] = 0.0f;
            }
        }
        return;
    }
    auto input = [&](int c) -> const float * {
        static const float kSilence[Q] = {};
        return active && c < in.channels ? in.channel(c) : kSilence;
    };
    switch (n.kind) {
        case NODE_GAIN: {
            double *g = paramScratch(n, 0);
            evalParam(n.id, kTargetGainParam, valueOr(gains_, n.id, 1.0), startNs, frameNs, g);
            for (int c = 0; c < channels; c++) {
                const float *x = input(c);
                float *y = o.channel(c);
                for (int f = 0; f < Q; f++) y[f] = static_cast<float>(x[f] * g[f]);
            }
            break;
        }
        case NODE_BIQUAD: {
            auto it = biquads_.find(n.id);
            if (n.biquad.size() < static_cast<size_t>(channels)) n.biquad.resize(static_cast<size_t>(channels));
            for (int c = 0; c < channels; c++) {
                const float *x = input(c);
                float *y = o.channel(c);
                if (it == biquads_.end()) std::memcpy(y, x, Q * sizeof(float));
                else for (int f = 0; f < Q; f++) y[f] = processBiquadSample(it->second, n.biquad[static_cast<size_t>(c)], x[f]);
            }
            break;
        }
        case NODE_IIR: {
            auto it = iirs_.find(n.id);
            if (n.iir.size() < static_cast<size_t>(channels)) n.iir.resize(static_cast<size_t>(channels));
            for (int c = 0; c < channels; c++) {
                const float *x = input(c);
                float *y = o.channel(c);
                if (it == iirs_.end()) std::memcpy(y, x, Q * sizeof(float));
                else for (int f = 0; f < Q; f++) y[f] = processIIRSample(it->second, n.iir[static_cast<size_t>(c)], x[f]);
            }
            break;
        }
        case NODE_WAVESHAPER: {
            auto it = waveShapers_.find(n.id);
            const bool shape = it != waveShapers_.end() && !it->second.curve.empty();
            for (int c = 0; c < channels; c++) {
                const float *x = input(c);
                float *y = o.channel(c);
                if (!shape) std::memcpy(y, x, Q * sizeof(float));
                else for (int f = 0; f < Q; f++) y[f] = applyWaveShaperSample(it->second, x[f]);
            }
            break;
        }
        case NODE_ANALYSER: {
            const float *ch[kMaxChannels];
            for (int c = 0; c < channels; c++) {
                ch[c] = input(c);
                std::memcpy(o.channel(c), ch[c], Q * sizeof(float));
            }
            std::lock_guard<std::mutex> lock(analyserMutex_);
            auto it = analysers_.find(n.id);
            if (it != analysers_.end() && it->second.capacity > 0) {
                AnalyserData &ad = it->second;
                for (int f = 0; f < Q; f++) {
                    ad.ring[static_cast<size_t>(ad.writeIdx.fetch_add(1, std::memory_order_relaxed) % ad.capacity)] = monoAt(ch, channels, f);
                }
            }
            break;
        }
        default:
            // A convolver passes its input through.
            for (int c = 0; c < channels; c++) std::memcpy(o.channel(c), input(c), Q * sizeof(float));
            break;
    }
}

void NativeEngine::renderQuantum(float *out, int64_t startNs) {
    const int channels = std::max(1, streamChannels_);
    const double frameNs = 1e9 / (streamSampleRate_ > 0 ? static_cast<double>(streamSampleRate_) : 48000.0);
    if (orderDirty_) {
        rebuildOrder();
        orderDirty_ = false;
    }
    endedScratch_.clear();
    for (GraphNode *n: order_) processNode(*n, startNs, frameNs);
    // A cycle's DelayNode takes its input once everything feeding it has rendered.
    for (GraphNode *n: order_) {
        if (n->delayInCycle && !n->muted) renderDelay(*n, startNs, frameNs, true, false);
    }

    std::fill(out, out + Q * channels, 0.0f);
    for (GraphNode *d: destinations_) {
        const Bus &b = d->inBus[0];
        if (!b.active) continue;
        for (int c = 0; c < std::min(b.channels, channels); c++) {
            const float *s = b.channel(c);
            for (int f = 0; f < Q; f++) out[f * channels + c] += s[f];
        }
    }
    // The device clips; an offline buffer keeps the full range, as in browsers.
    if (!offline_) {
        for (int i = 0; i < Q * channels; i++) out[i] = std::max(-1.0f, std::min(1.0f, out[i]));
    }

    // Ended sources, finished worklets and released nodes nothing plays into leave the graph.
    scratchNodes_.clear();
    for (GraphNode *n: order_) {
        if (n->kind == NODE_WORKLET && n->worklet && n->worklet->ended) {
            endedScratch_.push_back(n->id);
            scratchNodes_.push_back(n);
        }
    }
    for (GraphNode *n: scratchNodes_) removeWorklet(n->id);
    if (!endedScratch_.empty()) {
        for (const std::string &id: endedScratch_) {
            audioVoices_.erase(id);
            auto it = graph_.find(id);
            if (it != graph_.end() && it->second->kind == NODE_SOURCE) removeNode(it->second.get());
        }
        queueEnded(endedScratch_);
    }
    scratchNodes_.clear();
    for (auto &kv: graph_) {
        if (kv.second->released && !nodeBusy(*kv.second)) scratchNodes_.push_back(kv.second.get());
    }
    for (GraphNode *n: scratchNodes_) removeNode(n);
    runDeferred();
}

#endif
