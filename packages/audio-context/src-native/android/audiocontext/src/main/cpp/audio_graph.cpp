// The render graph: nodes with inputs and outputs, rendered a quantum at a time in topological order.
#include "audio_context.h"
#include "audio_internal.h"
#include "hrtf.h"
#include "oversampler.h"
#include "convolver.h"

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

bool isRamp(const NativeEngine::ParamEvent &e) {
    return e.type == NativeEngine::EVENT_LINEAR || e.type == NativeEngine::EVENT_EXPONENTIAL;
}

// The spec's automation value at tNs from events [0, end); `depth` bounds setTarget chains.
double eventValueAt(const std::vector<NativeEngine::ParamEvent> &evts, size_t end, int64_t tNs, double fallback,
                    int depth = 0) {
    auto it = std::upper_bound(evts.begin(), evts.begin() + static_cast<std::ptrdiff_t>(end), tNs,
                               [](int64_t time, const NativeEngine::ParamEvent &e) { return time < e.timeNs; });
    const auto idx = static_cast<size_t>(std::distance(evts.begin(), it));
    // A ramp runs from the end of the event before it, or from when it was called.
    if (idx < end && isRamp(evts[idx])) {
        const auto &next = evts[idx];
        double v0 = fallback;
        int64_t t0 = next.callNs;
        if (idx > 0) {
            const auto &prev = evts[idx - 1];
            if (prev.type == NativeEngine::EVENT_TARGET) {
                t0 = std::max(prev.timeNs, next.callNs);
                v0 = depth < 16 ? eventValueAt(evts, idx, t0, fallback, depth + 1) : prev.value;
            } else if (prev.type == NativeEngine::EVENT_CURVE) {
                t0 = prev.timeNs + prev.durationNs;
                v0 = prev.curve && !prev.curve->empty() ? prev.curve->back() : prev.value;
            } else {
                t0 = prev.timeNs;
                v0 = prev.value;
            }
            if (tNs < t0) return eventValueAt(evts, idx, tNs, fallback, depth + 1);
        }
        if (tNs < t0 || next.timeNs <= t0) return v0;
        const double ratio = static_cast<double>(tNs - t0) / static_cast<double>(next.timeNs - t0);
        if (next.type == NativeEngine::EVENT_LINEAR) return v0 + (next.value - v0) * ratio;
        if (v0 == 0.0 || v0 * next.value < 0.0) return v0;
        return v0 * std::pow(next.value / v0, ratio);
    }
    if (idx == 0) return fallback;
    const auto &prev = evts[idx - 1];
    switch (prev.type) {
        case NativeEngine::EVENT_TARGET: {
            if (prev.timeConstant <= 0.0) return prev.value;
            const double start = depth < 16 ? eventValueAt(evts, idx - 1, prev.timeNs, fallback, depth + 1) : prev.value;
            const double elapsed = static_cast<double>(tNs - prev.timeNs) * 1e-9;
            return prev.value + (start - prev.value) * std::exp(-elapsed / prev.timeConstant);
        }
        case NativeEngine::EVENT_CURVE: {
            if (!prev.curve || prev.curve->empty()) return prev.value;
            const auto &c = *prev.curve;
            const int n = static_cast<int>(c.size());
            if (prev.durationNs <= 0 || tNs >= prev.timeNs + prev.durationNs) return c.back();
            const double pos = static_cast<double>(tNs - prev.timeNs) / static_cast<double>(prev.durationNs) * (n - 1);
            const int k = std::min(n - 1, static_cast<int>(pos));
            if (k >= n - 1) return c.back();
            return c[k] + (c[k + 1] - c[k]) * (pos - k);
        }
        default:
            return prev.value;
    }
}

double eventValueAt(const std::vector<NativeEngine::ParamEvent> &evts, int64_t tNs, double fallback) {
    return eventValueAt(evts, evts.size(), tNs, fallback);
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

bool isListenerParam(int type) {
    return type >= kListenerParamPositionX && type <= kListenerParamUpZ;
}

// A BiquadFilterNode's coefficients, as the spec gives them (lowpass and highpass Q in dB).
NativeEngine::BiquadCoeffs biquadCoeffs(const std::string &type, double frequency, double Q, double gain,
                                        double detune, double sampleRate) {
    NativeEngine::BiquadCoeffs c;
    auto set = [&](double b0, double b1, double b2, double a0, double a1, double a2) {
        c.b0 = b0 / a0;
        c.b1 = b1 / a0;
        c.b2 = b2 / a0;
        c.a1 = a1 / a0;
        c.a2 = a2 / a0;
    };
    auto scale = [&](double k) { set(k, 0.0, 0.0, 1.0, 0.0, 0.0); };
    const double nyquist = sampleRate / 2.0;
    const double f = std::max(0.0, std::min(frequency * std::pow(2.0, detune / 1200.0), nyquist)) / nyquist;
    const double w0 = M_PI * f;
    const double cs = std::cos(w0);
    const double sn = std::sin(w0);
    const double A = std::pow(10.0, gain / 40.0);
    const bool inside = f > 0.0 && f < 1.0;
    if (type == "lowpass" || type == "highpass") {
        const bool low = type == "lowpass";
        if (f >= 1.0) { scale(low ? 1.0 : 0.0); return c; }
        if (f <= 0.0) { scale(low ? 0.0 : 1.0); return c; }
        const double alpha = sn / (2.0 * std::pow(10.0, Q / 20.0));
        if (low) set((1.0 - cs) / 2.0, 1.0 - cs, (1.0 - cs) / 2.0, 1.0 + alpha, -2.0 * cs, 1.0 - alpha);
        else set((1.0 + cs) / 2.0, -(1.0 + cs), (1.0 + cs) / 2.0, 1.0 + alpha, -2.0 * cs, 1.0 - alpha);
    } else if (type == "bandpass") {
        if (!inside) { scale(0.0); return c; }
        if (Q <= 0.0) { scale(1.0); return c; }
        const double alpha = sn / (2.0 * Q);
        set(alpha, 0.0, -alpha, 1.0 + alpha, -2.0 * cs, 1.0 - alpha);
    } else if (type == "notch") {
        if (!inside) { scale(1.0); return c; }
        if (Q <= 0.0) { scale(0.0); return c; }
        const double alpha = sn / (2.0 * Q);
        set(1.0, -2.0 * cs, 1.0, 1.0 + alpha, -2.0 * cs, 1.0 - alpha);
    } else if (type == "peaking") {
        if (!inside) { scale(1.0); return c; }
        if (Q <= 0.0) { scale(A * A); return c; }
        const double alpha = sn / (2.0 * Q);
        set(1.0 + alpha * A, -2.0 * cs, 1.0 - alpha * A, 1.0 + alpha / A, -2.0 * cs, 1.0 - alpha / A);
    } else if (type == "allpass") {
        if (!inside) { scale(1.0); return c; }
        if (Q <= 0.0) { scale(-1.0); return c; }
        const double alpha = sn / (2.0 * Q);
        set(1.0 - alpha, -2.0 * cs, 1.0 + alpha, 1.0 + alpha, -2.0 * cs, 1.0 - alpha);
    } else if (type == "lowshelf" || type == "highshelf") {
        const bool low = type == "lowshelf";
        if (f >= 1.0) { scale(low ? A * A : 1.0); return c; }
        if (f <= 0.0) { scale(low ? 1.0 : A * A); return c; }
        // Shelf slope 1.
        const double k = 2.0 * std::sqrt(A) * sn / 2.0 * std::sqrt(2.0);
        if (low) {
            set(A * ((A + 1.0) - (A - 1.0) * cs + k), 2.0 * A * ((A - 1.0) - (A + 1.0) * cs),
                A * ((A + 1.0) - (A - 1.0) * cs - k), (A + 1.0) + (A - 1.0) * cs + k,
                -2.0 * ((A - 1.0) + (A + 1.0) * cs), (A + 1.0) + (A - 1.0) * cs - k);
        } else {
            set(A * ((A + 1.0) + (A - 1.0) * cs + k), -2.0 * A * ((A - 1.0) + (A + 1.0) * cs),
                A * ((A + 1.0) + (A - 1.0) * cs - k), (A + 1.0) - (A - 1.0) * cs + k,
                2.0 * ((A - 1.0) - (A + 1.0) * cs), (A + 1.0) - (A - 1.0) * cs - k);
        }
    } else {
        scale(1.0);
    }
    return c;
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
            if (p.type != kTargetGainParam && !isListenerParam(p.type) && k == NODE_PANNER) return it->second.get();
        }
        if (p.type != kTargetGainParam) return nullptr;
        for (const auto &kv: voicePlaybackRate_) {
            if (kv.second == p.id) {
                auto v = graph_.find(kv.first);
                if (v != graph_.end()) return v->second.get();
            }
        }
        for (const auto &kv: biquadFilters_) {
            const std::string *ids = kv.second.paramIds;
            if (std::find(ids, ids + 4, p.id) != ids + 4) {
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
        // A listener param is read by every panner of its context.
        if (!isListenerParam(p.type)) continue;
        for (GraphNode *n: nodes) {
            if (n->kind != NODE_PANNER) continue;
            auto panner = panners_.find(n->id);
            if (panner != panners_.end() && panner->second.contextId == p.id) deps[static_cast<size_t>(n->index)].push_back(p.src);
        }
    }

    // Each node's context clock; connections never cross contexts, so it spreads along them.
    {
        std::lock_guard<std::mutex> lock(contextMutex_);
        for (GraphNode *n: nodes) {
            auto it = nodeClocks_.find(n->id);
            n->clock = it != nodeClocks_.end() ? it->second : nullptr;
        }
    }
    std::vector<GraphNode *> spread;
    for (GraphNode *n: nodes) {
        if (n->clock) spread.push_back(n);
    }
    std::vector<std::vector<GraphNode *>> linked(nodes.size());
    for (GraphNode *n: nodes) {
        for (GraphNode *d: deps[static_cast<size_t>(n->index)]) {
            linked[static_cast<size_t>(n->index)].push_back(d);
            linked[static_cast<size_t>(d->index)].push_back(n);
        }
    }
    while (!spread.empty()) {
        GraphNode *n = spread.back();
        spread.pop_back();
        for (GraphNode *m: linked[static_cast<size_t>(n->index)]) {
            if (m->clock) continue;
            m->clock = n->clock;
            spread.push_back(m);
        }
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
    std::lock_guard<std::mutex> lock(scheduledEventsMutex_);
    {
        const std::vector<ParamEvent> *ev = nullptr;
        if (type == kTargetGainParam) {
            auto it = scheduledGainEvents_.find(id);
            if (it != scheduledGainEvents_.end()) ev = &it->second;
        } else if (isListenerParam(type)) {
            auto it = scheduledListenerEvents_.find(id);
            if (it != scheduledListenerEvents_.end()) {
                auto jt = it->second.find(type);
                if (jt != it->second.end()) ev = &jt->second;
            }
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
    if (!paramSlots_.empty()) {
        auto it = paramSlots_.find(id);
        if (it != paramSlots_.end()) {
            for (const auto &slot: it->second) {
                if (slot->type != type || quantumStartNs_ < slot->holdNs.load(std::memory_order_relaxed)) continue;
                slot->value.store(out[Q - 1], std::memory_order_relaxed);
            }
        }
    }
    return constant;
}

double *NativeEngine::paramScratch(GraphNode &n, int index) {
    // Sized for every param a node reads at once, so earlier pointers stay valid.
    const size_t need = static_cast<size_t>(std::max(index + 1, 16)) * Q;
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
    // detune: cents on the oscillator's frequency or the buffer's playbackRate.
    double *detune = paramScratch(n, 1);
    const bool detuned = !v.detuneId.empty();
    if (detuned) evalParam(v.detuneId, kTargetGainParam, valueOr(gains_, v.detuneId, 0.0), startNs, frameNs, detune);
    // A buffer source's playbackRate and detune are k-rate.
    const double rate = std::max(0.0, param[0]) * (detuned && detune[0] != 0.0 ? std::exp2(detune[0] / 1200.0) : 1.0);
    double loopFrom = 0.0, loopTo = bufFrames;
    if (v.voiceType == Voice::BufferSource && v.loopStart >= 0.0 && v.loopEnd > 0.0 && v.loopStart < v.loopEnd) {
        loopFrom = std::min(v.loopStart * v.bufferSampleRate, static_cast<double>(bufFrames));
        loopTo = std::min(v.loopEnd * v.bufferSampleRate, static_cast<double>(bufFrames));
        if (loopTo <= loopFrom) loopFrom = 0.0, loopTo = bufFrames;
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
                const double freq = detuned && detune[f] != 0.0 ? param[f] * std::exp2(detune[f] / 1200.0) : param[f];
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
            const int s1 = v.loop && s0 + 1 >= static_cast<int>(std::ceil(loopTo)) ? static_cast<int>(loopFrom) : (s0 + 1) % bufFrames;
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
            v.position += v.increment * rate;
            if (v.loop && v.position >= loopTo) {
                v.position = loopFrom + std::fmod(v.position - loopFrom, loopTo - loopFrom);
            } else if (v.position >= bufFrames) {
                v.playing = false;
                break;
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
    const double sr = streamSampleRate_ > 0 ? static_cast<double>(streamSampleRate_) : 48000.0;
    auto pit = panners_.find(n.id);
    const bool hrtf = pit != panners_.end() && pit->second.panningModel == PANNING_HRTF;
    if (hrtf && !n.hrtf) n.hrtf = std::make_shared<HrtfPanner>(static_cast<int>(sr));
    else if (!hrtf && n.hrtf) n.hrtf.reset();
    // HRTF rings for its kernels and interaural delay after its input stops.
    if (active) {
        n.delayChannels = in.channels;
        n.tail = hrtf ? (n.hrtf->tailFrames() + Q - 1) / Q : 0;
    } else if (n.tail > 0) {
        n.tail--;
    }
    o.active = active || n.tail > 0;
    o.silence();
    if (!o.active) return;
    if (pit == panners_.end()) {
        for (int c = 0; c < 2; c++) {
            for (int f = 0; f < Q; f++) {
                o.channel(c)[f] = speakerMix([&](int k) { return in.channel(k)[f]; }, in.channels, 2, c);
            }
        }
        return;
    }
    const Panner &base = pit->second;
    const bool stereoPanner = base.panningModel == PANNING_STEREO;

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
    for (int i = 0; i < 9; i++) {
        values[7 + i] = paramScratch(n, 7 + i);
        constant &= evalParam(base.contextId, kListenerParamPositionX + i, listenerValues[i], startNs, frameNs, values[7 + i]);
    }

    const bool stereo = (active ? in.channels : n.delayChannels) > 1;
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
        L.positionX = values[7][f];
        L.positionY = values[8][f];
        L.positionZ = values[9][f];
        L.forwardX = values[10][f];
        L.forwardY = values[11][f];
        L.forwardZ = values[12][f];
        L.upX = values[13][f];
        L.upY = values[14][f];
        L.upZ = values[15][f];
        if (stereoPanner) {
            // StereoPannerNode: pan alone, no distance or cone.
            const double pan = std::max(-1.0, std::min(1.0, values[6][f]));
            steerLeft = pan <= 0.0;
            const double x = stereo ? (steerLeft ? pan + 1.0 : pan) : (pan + 1.0) * 0.5;
            leftGain = std::cos(x * M_PI / 2.0);
            rightGain = std::sin(x * M_PI / 2.0);
            dist = 1.0;
        } else {
            p.pan = 0.0;
            computePannerGains(p, L, stereo && !hrtf, leftGain, rightGain, dist, &steerLeft);
        }
    };
    resolve(0);

    float *outL = o.channel(0);
    float *outR = o.channel(1);
    const float *inL = active ? in.channel(0) : nullptr;
    const float *inR = active && stereo ? in.channel(1) : inL;
    if (hrtf) {
        // Direction is k-rate, from the quantum's first frame, as in Chromium.
        const Panner local = transformPannerToListenerSpace(p, L);
        const double x = local.positionX, y = local.positionY, z = local.positionZ;
        const double horizontal = std::sqrt(x * x + z * z);
        const double azimuth = horizontal + std::fabs(y) < 1e-12 ? 0.0 : std::atan2(x, z) * 180.0 / M_PI;
        const double elevation = horizontal + std::fabs(y) < 1e-12 ? 0.0 : std::atan2(y, horizontal) * 180.0 / M_PI;
        n.hrtf->process(azimuth, elevation, inL, inR, outL, outR);
        for (int f = 0; f < Q; f++) {
            if (!constant && f > 0) resolve(f);
            outL[f] = static_cast<float>(outL[f] * dist);
            outR[f] = static_cast<float>(outR[f] * dist);
        }
        return;
    }
    if (!active) return;
    for (int f = 0; f < Q; f++) {
        if (!constant && f > 0) resolve(f);
        if (!stereo) {
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
    // Its look-ahead line plays out after the input stops.
    if (active) {
        n.tail = 4;
        n.delayChannels = in.channels;
    } else if (n.tail > 0) {
        n.tail--;
        in.setChannels(n.delayChannels);
        in.silence();
    }
    o.setChannels(in.channels);
    o.active = active || n.tail > 0;
    if (!o.active) {
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
    // k-rate params; one envelope for every channel; 6 ms of look-ahead; the spec's makeup gain.
    const double r = std::max(1.0, ratio[0]);
    const double k = std::max(0.0, knee[0]);
    const double t = threshold[0];
    const double attackCoeff = std::exp(-1.0 / (std::max(attack[0], 1.0 / sr) * sr));
    const double releaseCoeff = std::exp(-1.0 / (std::max(release[0], 1.0 / sr) * sr));
    auto reductionAt = [&](double level) {
        const double over = level - t;
        double compressed = level;
        if (k > 0.0) {
            const double half = 0.5 * k;
            if (over >= half) compressed = t + over / r;
            else if (over > -half) compressed = level + (1.0 / r - 1.0) * (over + half) * (over + half) / (2.0 * k);
        } else if (over > 0.0) {
            compressed = t + over / r;
        }
        return std::max(0.0, level - compressed);
    };
    const double makeup = std::pow(10.0, 0.6 * reductionAt(0.0) / 20.0);
    const int lookahead = std::max(1, static_cast<int>(std::lround(0.006 * sr)));
    const size_t lineSize = static_cast<size_t>(lookahead) * in.channels;
    if (n.preDelay.size() != lineSize) {
        n.preDelay.assign(lineSize, 0.0f);
        n.preDelayPos = 0;
    }
    if (n.reductionDb.empty()) n.reductionDb.assign(1, 0.0f);
    double current = n.reductionDb[0];
    for (int f = 0; f < Q; f++) {
        double peak = 0.0;
        for (int c = 0; c < in.channels; c++) peak = std::max(peak, std::fabs(static_cast<double>(in.channel(c)[f])));
        const double target = reductionAt(20.0 * std::log10(std::max(1.0e-12, peak)));
        const double coeff = target > current ? attackCoeff : releaseCoeff;
        current = coeff * current + (1.0 - coeff) * target;
        const double gain = makeup * std::pow(10.0, -current / 20.0);
        for (int c = 0; c < in.channels; c++) {
            float &slot = n.preDelay[static_cast<size_t>(c) * lookahead + n.preDelayPos];
            const float delayed = slot;
            slot = in.channel(c)[f];
            o.channel(c)[f] = static_cast<float>(delayed * gain);
        }
        n.preDelayPos = (n.preDelayPos + 1) % lookahead;
    }
    n.reductionDb[0] = static_cast<float>(current);
    if (!comp.reductionId.empty()) {
        gains_[comp.reductionId] = -n.reductionDb[0];
    }
    if (comp.meter) {
        comp.meter->store(-n.reductionDb[0], std::memory_order_relaxed);
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
        // computedNumberOfChannels from this quantum's connections and the channel config.
        const bool active = mixInput(n, i, std::min(n.channelCount, kWorkletMaxChannels), n.countMode, n.interpretation);
        w.inActive[i] = active;
        if (!active) continue;
        const Bus &in = n.inBus[static_cast<size_t>(i)];
        w.inCount[i] = std::min(in.channels, w.mixChannels);
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
        case NODE_CONVOLVER:
            renderConvolver(n);
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
        case NODE_BIQUAD:
            renderBiquad(n, startNs, frameNs, channels, active);
            break;
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
            const int factor = !shape ? 1 : (it->second.oversample == "4x" ? 4 : (it->second.oversample == "2x" ? 2 : 1));
            while (factor > 1 && n.oversamplers.size() < static_cast<size_t>(channels)) {
                n.oversamplers.push_back(std::make_shared<Oversampler>());
            }
            for (int c = 0; c < channels; c++) {
                const float *x = input(c);
                float *y = o.channel(c);
                if (!shape) {
                    std::memcpy(y, x, Q * sizeof(float));
                } else if (factor == 1) {
                    for (int f = 0; f < Q; f++) y[f] = applyWaveShaperSample(it->second, x[f]);
                } else {
                    // Shapes at 2x or 4x the rate, so the harmonics it adds alias less.
                    Oversampler &os = *n.oversamplers[static_cast<size_t>(c)];
                    float twice[2 * Q], fourfold[4 * Q];
                    os.upsample(0, x, Q, twice);
                    float *high = twice;
                    if (factor == 4) {
                        os.upsample(1, twice, 2 * Q, fourfold);
                        high = fourfold;
                    }
                    for (int f = 0; f < factor * Q; f++) high[f] = applyWaveShaperSample(it->second, high[f]);
                    if (factor == 4) os.downsample(1, fourfold, 4 * Q, twice);
                    os.downsample(0, twice, 2 * Q, y);
                }
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
            for (int c = 0; c < channels; c++) std::memcpy(o.channel(c), input(c), Q * sizeof(float));
            break;
    }
}

void NativeEngine::renderBiquad(GraphNode &n, int64_t startNs, double frameNs, int channels, bool active) {
    static const float kSilence[Q] = {};
    const Bus &in = n.inBus[0];
    Bus &o = n.outBus[0];
    const float *x[kMaxChannels];
    for (int c = 0; c < channels; c++) x[c] = active && c < in.channels ? in.channel(c) : kSilence;
    if (n.biquad.size() < static_cast<size_t>(channels)) n.biquad.resize(static_cast<size_t>(channels));
    auto fit = biquadFilters_.find(n.id);
    if (fit == biquadFilters_.end() || fit->second.paramIds[0].empty()) {
        auto it = biquads_.find(n.id);
        for (int c = 0; c < channels; c++) {
            float *y = o.channel(c);
            if (it == biquads_.end()) std::memcpy(y, x[c], Q * sizeof(float));
            else for (int f = 0; f < Q; f++) y[f] = processBiquadSample(it->second, n.biquad[static_cast<size_t>(c)], x[c][f]);
        }
        return;
    }
    BiquadFilter &bf = fit->second;
    const double sr = streamSampleRate_ > 0 ? static_cast<double>(streamSampleRate_) : 48000.0;
    // frequency, Q, gain, detune.
    static constexpr double kDefaults[4] = {350.0, 1.0, 0.0, 0.0};
    double *v[4];
    bool constant = true;
    for (int i = 0; i < 4; i++) {
        v[i] = paramScratch(n, i);
        constant &= evalParam(bf.paramIds[i], kTargetGainParam, valueOr(gains_, bf.paramIds[i], kDefaults[i]), startNs,
                              frameNs, v[i]);
    }
    if (constant) {
        if (v[0][0] != bf.last[0] || v[1][0] != bf.last[1] || v[2][0] != bf.last[2] || v[3][0] != bf.last[3]) {
            bf.coeffs = biquadCoeffs(bf.type, v[0][0], v[1][0], v[2][0], v[3][0], sr);
            for (int i = 0; i < 4; i++) bf.last[i] = v[i][0];
        }
        for (int c = 0; c < channels; c++) {
            float *y = o.channel(c);
            for (int f = 0; f < Q; f++) y[f] = processBiquadSample(bf.coeffs, n.biquad[static_cast<size_t>(c)], x[c][f]);
        }
        return;
    }
    // Automated or driven by audio: new coefficients every frame.
    for (int f = 0; f < Q; f++) {
        bf.coeffs = biquadCoeffs(bf.type, v[0][f], v[1][f], v[2][f], v[3][f], sr);
        for (int c = 0; c < channels; c++) {
            o.channel(c)[f] = processBiquadSample(bf.coeffs, n.biquad[static_cast<size_t>(c)], x[c][f]);
        }
    }
    bf.last[0] = -1.0;
}

void NativeEngine::renderConvolver(GraphNode &n) {
    const bool active = mixInput(n, 0, n.channelCount, n.countMode, n.interpretation);
    const Bus &in = n.inBus[0];
    Bus &o = n.outBus[0];
    auto it = convolvers_.find(n.id);
    if (it != convolvers_.end() && active) {
        n.tail = it->second->tail();
        // The input channels it convolves, kept for its tail.
        n.delayChannels = std::min(2, in.channels);
    } else if (n.tail > 0) {
        n.tail--;
    }
    // No response, or nothing left ringing: silent.
    if (it == convolvers_.end() || (!active && n.tail == 0)) {
        if (it == convolvers_.end()) n.tail = 0;
        o.setChannels(1);
        o.active = false;
        o.silence();
        return;
    }
    Convolver &cv = *it->second;
    const int inputs = std::max(1, n.delayChannels);
    const float *src[2] = {nullptr, nullptr};
    for (int c = 0; c < inputs && active; c++) src[c] = in.channel(std::min(c, in.channels - 1));
    o.setChannels(cv.outputChannels(inputs));
    o.active = true;
    float *dst[2] = {o.channel(0), o.channels > 1 ? o.channel(1) : nullptr};
    cv.process(src, inputs, dst);
}

void NativeEngine::renderQuantum(float *out, int64_t startNs) {
    const int channels = std::max(1, streamChannels_);
    const double frameNs = 1e9 / (streamSampleRate_ > 0 ? static_cast<double>(streamSampleRate_) : 48000.0);
    if (clocksDirty_.exchange(false, std::memory_order_acquire)) orderDirty_ = true;
    if (orderDirty_) {
        rebuildOrder();
        orderDirty_ = false;
    }
    endedScratch_.clear();
    // A suspended context's nodes hold still; the others render at their context's time.
    auto paused = [](const GraphNode &n) {
        return n.clock && n.clock->pausedAtNs.load(std::memory_order_relaxed) != 0;
    };
    auto nodeNs = [startNs](const GraphNode &n) {
        return n.clock ? startNs - n.clock->offsetNs.load(std::memory_order_relaxed) : startNs;
    };
    for (GraphNode *n: order_) {
        if (paused(*n)) {
            for (auto &b: n->inBus) b.active = false;
            for (auto &b: n->outBus) {
                b.active = false;
                b.silence();
            }
            continue;
        }
        processNode(*n, nodeNs(*n), frameNs);
    }
    // A cycle's DelayNode takes its input once everything feeding it has rendered.
    for (GraphNode *n: order_) {
        if (n->delayInCycle && !n->muted && !paused(*n)) renderDelay(*n, nodeNs(*n), frameNs, true, false);
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
            sourceExtras_.erase(id);
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
