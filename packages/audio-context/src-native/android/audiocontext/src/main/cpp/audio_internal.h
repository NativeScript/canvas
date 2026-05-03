#ifndef AUDIO_INTERNAL_H
#define AUDIO_INTERNAL_H


#include "audio_context.h"

#include <atomic>
#include <cmath>
#include <cstdarg>
#include <cstdint>
#include <random>
#include <sstream>
#include <string>
#include <vector>
#include <algorithm>

#include <android/log.h>

#ifndef M_PI
#define M_PI 3.14159265358979323846
#endif

inline constexpr const char *TAG = "native-audio";

inline std::atomic<int64_t> g_audioTimeNanos{0};
inline std::atomic<bool> g_audioThreadLoggingEnabled{false};

inline constexpr int kPannerParamPositionX = 5;
inline constexpr int kPannerParamPositionY = 6;
inline constexpr int kPannerParamPositionZ = 7;
inline constexpr int kPannerParamOrientationX = 8;
inline constexpr int kPannerParamOrientationY = 9;
inline constexpr int kPannerParamOrientationZ = 10;
inline constexpr int kPannerParamPan = 11;

inline constexpr int kListenerParamPositionX = 12;
inline constexpr int kListenerParamPositionY = 13;
inline constexpr int kListenerParamPositionZ = 14;
inline constexpr int kListenerParamForwardX = 15;
inline constexpr int kListenerParamForwardY = 16;
inline constexpr int kListenerParamForwardZ = 17;
inline constexpr int kListenerParamUpX = 18;
inline constexpr int kListenerParamUpY = 19;
inline constexpr int kListenerParamUpZ = 20;

using ScheduledParamEnvelopeMap = std::unordered_map<int, std::vector<double>>;
using ScheduledNodeEnvelopeMap = std::unordered_map<std::string, ScheduledParamEnvelopeMap>;

inline void audioThreadLog(int prio, const char *fmt, ...) {
    if (!g_audioThreadLoggingEnabled.load(std::memory_order_relaxed)) return;
    va_list args;
    va_start(args, fmt);
    __android_log_vprint(prio, TAG, fmt, args);
    va_end(args);
}

inline std::string genId() {
    static std::mt19937_64 rng((std::random_device()) ());
    std::uniform_int_distribution<uint64_t> dist;
    std::ostringstream ss;
    ss << std::hex << dist(rng);
    return ss.str();
}

inline NativeEngine::BiquadCoeffs
computeBiquadCoeffs(const std::string &type, double f0, double Q, double gainDB,
                    double sampleRate) {
    NativeEngine::BiquadCoeffs c;
    if (sampleRate <= 0) sampleRate = 48000.0;
    double omega = 2.0 * M_PI * f0 / sampleRate;
    double sn = std::sin(omega);
    double cs = std::cos(omega);
    double A = std::pow(10.0, gainDB / 40.0);
    double alpha = sn / (2.0 * Q);
    double a0 = 1.0;

    if (type == "lowpass") {
        double b0 = (1.0 - cs) / 2.0;
        double b1 = 1.0 - cs;
        double b2 = (1.0 - cs) / 2.0;
        double a1 = -2.0 * cs;
        double a2 = 1.0 - alpha;
        a0 = 1.0 + alpha;
        c.b0 = b0 / a0; c.b1 = b1 / a0; c.b2 = b2 / a0; c.a1 = a1 / a0; c.a2 = a2 / a0;
    } else if (type == "highpass") {
        double b0 = (1.0 + cs) / 2.0;
        double b1 = -(1.0 + cs);
        double b2 = (1.0 + cs) / 2.0;
        double a1 = -2.0 * cs;
        double a2 = 1.0 - alpha;
        a0 = 1.0 + alpha;
        c.b0 = b0 / a0; c.b1 = b1 / a0; c.b2 = b2 / a0; c.a1 = a1 / a0; c.a2 = a2 / a0;
    } else if (type == "bandpass") {
        double b0 = alpha;
        double b1 = 0.0;
        double b2 = -alpha;
        double a1 = -2.0 * cs;
        double a2 = 1.0 - alpha;
        a0 = 1.0 + alpha;
        c.b0 = b0 / a0; c.b1 = b1 / a0; c.b2 = b2 / a0; c.a1 = a1 / a0; c.a2 = a2 / a0;
    } else if (type == "notch") {
        double b0 = 1.0;
        double b1 = -2.0 * cs;
        double b2 = 1.0;
        double a1 = -2.0 * cs;
        double a2 = 1.0 - alpha;
        a0 = 1.0 + alpha;
        c.b0 = b0 / a0; c.b1 = b1 / a0; c.b2 = b2 / a0; c.a1 = a1 / a0; c.a2 = a2 / a0;
    } else if (type == "peaking") {
        double b0 = 1.0 + alpha * A;
        double b1 = -2.0 * cs;
        double b2 = 1.0 - alpha * A;
        double a1 = -2.0 * cs;
        double a2 = 1.0 - alpha / A;
        a0 = 1.0 + alpha / A;
        c.b0 = b0 / a0; c.b1 = b1 / a0; c.b2 = b2 / a0; c.a1 = a1 / a0; c.a2 = a2 / a0;
    } else if (type == "lowshelf") {
        double sqrtA = std::sqrt(A);
        double twoSqrtAalpha = 2.0 * sqrtA * alpha;
        double b0 = A * ((A + 1.0) - (A - 1.0) * cs + twoSqrtAalpha);
        double b1 = 2.0 * A * ((A - 1.0) - (A + 1.0) * cs);
        double b2 = A * ((A + 1.0) - (A - 1.0) * cs - twoSqrtAalpha);
        double a1 = 2.0 * ((A - 1.0) + (A + 1.0) * cs);
        double a2 = ((A + 1.0) + (A - 1.0) * cs - twoSqrtAalpha);
        a0 = ((A + 1.0) + (A - 1.0) * cs + twoSqrtAalpha);
        c.b0 = b0 / a0; c.b1 = b1 / a0; c.b2 = b2 / a0; c.a1 = a1 / a0; c.a2 = a2 / a0;
    } else if (type == "highshelf") {
        double sqrtA = std::sqrt(A);
        double twoSqrtAalpha = 2.0 * sqrtA * alpha;
        double b0 = A * ((A + 1.0) + (A - 1.0) * cs + twoSqrtAalpha);
        double b1 = -2.0 * A * ((A - 1.0) + (A + 1.0) * cs);
        double b2 = A * ((A + 1.0) + (A - 1.0) * cs - twoSqrtAalpha);
        double a1 = -2.0 * ((A - 1.0) - (A + 1.0) * cs);
        double a2 = ((A + 1.0) - (A - 1.0) * cs - twoSqrtAalpha);
        a0 = ((A + 1.0) - (A - 1.0) * cs + twoSqrtAalpha);
        c.b0 = b0 / a0; c.b1 = b1 / a0; c.b2 = b2 / a0; c.a1 = a1 / a0; c.a2 = a2 / a0;
    } else if (type == "allpass") {
        double b0 = 1.0 - alpha;
        double b1 = -2.0 * cs;
        double b2 = 1.0 + alpha;
        double a1 = -2.0 * cs;
        double a2 = 1.0 + alpha;
        a0 = 1.0 + alpha;
        c.b0 = b0 / a0; c.b1 = b1 / a0; c.b2 = b2 / a0; c.a1 = a1 / a0; c.a2 = a2 / a0;
    } else {
        c.b0 = 1.0; c.b1 = 0.0; c.b2 = 0.0; c.a1 = 0.0; c.a2 = 0.0;
    }
    return c;
}

inline float
processBiquadSample(const NativeEngine::BiquadCoeffs &c, NativeEngine::BiquadState &s, float in) {
    double out = c.b0 * in + s.s1;
    double s1 = c.b1 * in - c.a1 * out + s.s2;
    double s2 = c.b2 * in - c.a2 * out;
    s.s1 = s1;
    s.s2 = s2;
    return static_cast<float>(out);
}

inline float
processIIRSample(const NativeEngine::IIRData &coeffs, std::vector<double> &state, float in) {
    const std::vector<double> &b = coeffs.feedforward;
    const std::vector<double> &a = coeffs.feedback;
    size_t nb = b.size();
    size_t na = a.size();
    if (nb == 0) return 0.0f;

    size_t K = 0;
    if (std::max(nb, na) >= 1) K = std::max(nb, na) - 1;
    if (K == 0) {
        double b0 = (nb > 0) ? b[0] : 0.0;
        return static_cast<float>(b0 * in);
    }

    if (state.size() < K) state.assign(K, 0.0);

    double out = (nb > 0 ? b[0] : 0.0) * in + state[0];

    for (size_t i = 0; i < K; ++i) {
        double bi1 = (i + 1 < nb) ? b[i + 1] : 0.0;
        double ai1 = (i + 1 < na) ? a[i + 1] : 0.0;
        double nextState = (i + 1 < K) ? state[i + 1] : 0.0;
        double newState = bi1 * in - ai1 * out + nextState;
        state[i] = newState;
    }

    return static_cast<float>(out);
}

inline float
computePeriodicWaveSample(const NativeEngine::PeriodicWaveData &pw, double phase, double freq,
                          double sampleRate) {
    const std::vector<double> &real = pw.real;
    const std::vector<double> &imag = pw.imag;
    size_t N = std::min(real.size(), imag.size());
    double val = 0.0;
    if (N > 0) val += real[0] * 0.5;
    if (N <= 1) return static_cast<float>(val);
    if (freq <= 0.0) freq = 440.0;
    int maxHarm = 0;
    if (sampleRate > 0.0 && freq > 0.0) {
        maxHarm = static_cast<int>(std::floor((sampleRate / 2.0) / freq));
    }
    if (maxHarm < 0) maxHarm = 0;
    int maxN = static_cast<int>(N) - 1;
    int limit = std::min(maxN, maxHarm);
    for (int n = 1; n <= limit; ++n) {
        double angle = 2.0 * M_PI * static_cast<double>(n) * phase;
        val += real[n] * std::cos(angle) + imag[n] * std::sin(angle);
    }
    return static_cast<float>(val);
}

inline float
applyWaveShaperSample(const NativeEngine::WaveShaperData &ws, float in) {
    if (ws.curve.empty()) return in;
    size_t N = ws.curve.size();
    if (N == 1) return ws.curve[0];
    double x = in;
    if (x < -1.0) x = -1.0;
    if (x > 1.0) x = 1.0;
    double idx = (x + 1.0) * 0.5 * static_cast<double>(N - 1);
    int i0 = static_cast<int>(std::floor(idx));
    if (i0 < 0) i0 = 0;
    int i1 = i0 + 1;
    if (i1 >= static_cast<int>(N)) i1 = static_cast<int>(N - 1);
    double frac = idx - i0;
    float out = ws.curve[(size_t) i0] * (1.0 - frac) + ws.curve[(size_t) i1] * frac;
    return out;
}

inline double getEnvelopeFrameValue(const ScheduledParamEnvelopeMap *paramMap, int paramType,
                                    int frame, double fallback) {
    if (!paramMap || frame < 0) return fallback;
    auto it = paramMap->find(paramType);
    if (it == paramMap->end()) return fallback;
    auto index = static_cast<size_t>(frame);
    if (index >= it->second.size()) return fallback;
    return it->second[index];
}

inline NativeEngine::Panner resolvePannerForFrame(const NativeEngine::Panner &base,
                                                  const ScheduledNodeEnvelopeMap &envelopes,
                                                  const std::string &pannerId,
                                                  int frame) {
    NativeEngine::Panner resolved = base;
    auto envIt = envelopes.find(pannerId);
    const ScheduledParamEnvelopeMap *paramMap = envIt == envelopes.end() ? nullptr : &envIt->second;
    resolved.positionX = getEnvelopeFrameValue(paramMap, kPannerParamPositionX, frame,
                                               resolved.positionX);
    resolved.positionY = getEnvelopeFrameValue(paramMap, kPannerParamPositionY, frame,
                                               resolved.positionY);
    resolved.positionZ = getEnvelopeFrameValue(paramMap, kPannerParamPositionZ, frame,
                                               resolved.positionZ);
    resolved.orientationX = getEnvelopeFrameValue(paramMap, kPannerParamOrientationX, frame,
                                                  resolved.orientationX);
    resolved.orientationY = getEnvelopeFrameValue(paramMap, kPannerParamOrientationY, frame,
                                                  resolved.orientationY);
    resolved.orientationZ = getEnvelopeFrameValue(paramMap, kPannerParamOrientationZ, frame,
                                                  resolved.orientationZ);
    resolved.pan = getEnvelopeFrameValue(paramMap, kPannerParamPan, frame, resolved.pan);
    return resolved;
}

inline NativeEngine::Listener resolveListenerForFrame(
        const std::string &contextId,
        const NativeEngine::Listener &fallback,
        const std::unordered_map<std::string, NativeEngine::Listener> &listeners,
        const ScheduledNodeEnvelopeMap &envelopes,
        int frame) {
    NativeEngine::Listener resolved = fallback;
    if (!contextId.empty()) {
        auto listenerIt = listeners.find(contextId);
        if (listenerIt != listeners.end()) {
            resolved = listenerIt->second;
        }
    }
    auto envIt = envelopes.find(contextId);
    const ScheduledParamEnvelopeMap *paramMap = envIt == envelopes.end() ? nullptr : &envIt->second;
    resolved.positionX = getEnvelopeFrameValue(paramMap, kListenerParamPositionX, frame,
                                               resolved.positionX);
    resolved.positionY = getEnvelopeFrameValue(paramMap, kListenerParamPositionY, frame,
                                               resolved.positionY);
    resolved.positionZ = getEnvelopeFrameValue(paramMap, kListenerParamPositionZ, frame,
                                               resolved.positionZ);
    resolved.forwardX = getEnvelopeFrameValue(paramMap, kListenerParamForwardX, frame,
                                              resolved.forwardX);
    resolved.forwardY = getEnvelopeFrameValue(paramMap, kListenerParamForwardY, frame,
                                              resolved.forwardY);
    resolved.forwardZ = getEnvelopeFrameValue(paramMap, kListenerParamForwardZ, frame,
                                              resolved.forwardZ);
    resolved.upX = getEnvelopeFrameValue(paramMap, kListenerParamUpX, frame, resolved.upX);
    resolved.upY = getEnvelopeFrameValue(paramMap, kListenerParamUpY, frame, resolved.upY);
    resolved.upZ = getEnvelopeFrameValue(paramMap, kListenerParamUpZ, frame, resolved.upZ);
    return resolved;
}

inline NativeEngine::Panner transformPannerToListenerSpace(const NativeEngine::Panner &p,
                                                           const NativeEngine::Listener &listener) {
    NativeEngine::Panner transformed = p;
    double rx = p.positionX - listener.positionX;
    double ry = p.positionY - listener.positionY;
    double rz = p.positionZ - listener.positionZ;

    double fx = listener.forwardX;
    double fy = listener.forwardY;
    double fz = listener.forwardZ;
    double ux = listener.upX;
    double uy = listener.upY;
    double uz = listener.upZ;

    double fl = std::sqrt(fx * fx + fy * fy + fz * fz);
    if (fl <= 1e-12) {
        fx = 0.0;
        fy = 0.0;
        fz = -1.0;
    } else {
        fx /= fl;
        fy /= fl;
        fz /= fl;
    }

    double ul = std::sqrt(ux * ux + uy * uy + uz * uz);
    if (ul <= 1e-12) {
        ux = 0.0;
        uy = 1.0;
        uz = 0.0;
    } else {
        ux /= ul;
        uy /= ul;
        uz /= ul;
    }

    double rxv = fy * uz - fz * uy;
    double ryv = fz * ux - fx * uz;
    double rzv = fx * uy - fy * ux;
    double rl = std::sqrt(rxv * rxv + ryv * ryv + rzv * rzv);
    if (rl <= 1e-12) {
        rxv = 1.0;
        ryv = 0.0;
        rzv = 0.0;
    } else {
        rxv /= rl;
        ryv /= rl;
        rzv /= rl;
    }

    double ux2 = ryv * fz - rzv * fy;
    double uy2 = rzv * fx - rxv * fz;
    double uz2 = rxv * fy - ryv * fx;

    transformed.positionX = rx * rxv + ry * ryv + rz * rzv;
    transformed.positionY = rx * ux2 + ry * uy2 + rz * uz2;
    transformed.positionZ = rx * fx + ry * fy + rz * fz;
    return transformed;
}

inline double wrapPannerAzimuthDegrees(double azimuth) {
    azimuth = std::max(-180.0, std::min(180.0, azimuth));
    if (azimuth < -90.0) azimuth = -180.0 - azimuth;
    else if (azimuth > 90.0) azimuth = 180.0 - azimuth;
    return azimuth;
}

inline double computeDistanceAttenuation(const NativeEngine::Panner &p,
                                         const NativeEngine::Listener &listener) {
    double dx = p.positionX - listener.positionX;
    double dy = p.positionY - listener.positionY;
    double dz = p.positionZ - listener.positionZ;
    double dist = std::sqrt(dx * dx + dy * dy + dz * dz);
    double ref = std::max(1e-6, p.refDistance);
    double maxd = std::max(ref, p.maxDistance);
    double rf = std::max(0.0, p.rolloffFactor);
    double d = std::min(std::max(dist, ref), maxd);

    if (p.distanceModel == NativeEngine::DISTANCE_LINEAR) {
        if (maxd <= ref) return dist > maxd ? 0.0 : 1.0;
        double gain = 1.0 - rf * (d - ref) / (maxd - ref);
        return std::max(0.0, std::min(1.0, gain));
    }

    double gain = 1.0;
    if (p.distanceModel == NativeEngine::DISTANCE_INVERSE) {
        gain = ref / (ref + rf * (d - ref));
    } else {
        gain = std::pow(d / ref, -rf);
    }
    return std::max(0.0, std::min(1.0, gain));
}

inline double computeConeAttenuation(const NativeEngine::Panner &p,
                                     const NativeEngine::Listener &listener) {
    double ox = p.orientationX;
    double oy = p.orientationY;
    double oz = p.orientationZ;
    double olen = std::sqrt(ox * ox + oy * oy + oz * oz);
    if (olen <= 1e-12) {
        ox = 1.0;
        oy = 0.0;
        oz = 0.0;
        olen = 1.0;
    }

    double sx = listener.positionX - p.positionX;
    double sy = listener.positionY - p.positionY;
    double sz = listener.positionZ - p.positionZ;
    double slen = std::sqrt(sx * sx + sy * sy + sz * sz);
    if (slen <= 1e-12) return 1.0;

    if (p.coneInnerAngle == 360.0 && p.coneOuterAngle == 360.0) return 1.0;

    double dot = (ox * sx + oy * sy + oz * sz) / (olen * slen);
    if (dot > 1.0) dot = 1.0;
    if (dot < -1.0) dot = -1.0;

    double angle = std::acos(dot) * 180.0 / M_PI;
    double inner = std::max(0.0, std::min(360.0, p.coneInnerAngle)) * 0.5;
    double outer = std::max(0.0, std::min(360.0, p.coneOuterAngle)) * 0.5;
    if (outer < inner) inner = outer;
    double outerGain = std::max(0.0, std::min(1.0, p.coneOuterGain));

    if (angle <= inner) return 1.0;
    if (angle >= outer) return outerGain;

    double denom = outer - inner;
    if (denom <= 0.0) return outerGain;
    double t = (angle - inner) / denom;
    return (1.0 - t) + t * outerGain;
}

inline void computePannerGains(const NativeEngine::Panner &p,
                               const NativeEngine::Listener &listener,
                               bool stereoInput,
                               double &leftGain,
                               double &rightGain,
                               double &distanceAtt,
                               bool *steerLeftOrCenter = nullptr) {
    leftGain = 1.0;
    rightGain = 1.0;
    distanceAtt = computeDistanceAttenuation(p, listener) * computeConeAttenuation(p, listener);

    bool steerLeft = true;
    double x = 0.5;
    if (std::abs(p.pan) > 1e-12) {
        double pan = std::max(-1.0, std::min(1.0, p.pan));
        steerLeft = pan <= 0.0;
        x = stereoInput ? (steerLeft ? pan + 1.0 : pan) : (pan + 1.0) * 0.5;
    } else {
        NativeEngine::Panner local = transformPannerToListenerSpace(p, listener);
        double azimuth = wrapPannerAzimuthDegrees(
                std::atan2(local.positionX, local.positionZ) * 180.0 / M_PI);
        steerLeft = azimuth <= 0.0;
        x = stereoInput ? (steerLeft ? (azimuth + 90.0) / 90.0 : azimuth / 90.0)
                        : (azimuth + 90.0) / 180.0;
    }

    x = std::max(0.0, std::min(1.0, x));

    leftGain = std::cos(x * M_PI / 2.0);
    rightGain = std::sin(x * M_PI / 2.0);
    if (steerLeftOrCenter) *steerLeftOrCenter = steerLeft;
}

inline void insertSortedByTime(std::vector<NativeEngine::ParamEvent> &vec,
                               const NativeEngine::ParamEvent &ev) {
    if (vec.empty() || ev.timeNs >= vec.back().timeNs) {
        vec.push_back(ev);
        return;
    }
    auto it = std::upper_bound(vec.begin(), vec.end(), ev.timeNs,
                               [](int64_t t, const NativeEngine::ParamEvent &e) {
                                   return t < e.timeNs;
                               });
    vec.insert(it, ev);
}

inline void trimEventsAtOrAfter(std::vector<NativeEngine::ParamEvent> &vec, int64_t timeNs) {
    auto cut = std::lower_bound(vec.begin(), vec.end(), timeNs,
                                [](const NativeEngine::ParamEvent &e, int64_t t) {
                                    return e.timeNs < t;
                                });
    vec.erase(cut, vec.end());
}

inline float getScheduledGainValueAt(const std::vector<NativeEngine::ParamEvent> &evts, int64_t tNs,
                                     double fallback) {
    if (evts.empty()) return static_cast<float>(fallback);
    auto it = std::upper_bound(evts.begin(), evts.end(), tNs,
                               [](int64_t time, const NativeEngine::ParamEvent &e) {
                                   return time < e.timeNs;
                               });
    size_t idx = std::distance(evts.begin(), it);
    if (idx == 0) {
        return static_cast<float>(fallback);
    }
    if (idx >= evts.size()) {
        return static_cast<float>(evts.back().value);
    }
    const auto &prev = evts[idx - 1];
    const auto &next = evts[idx];
    if (next.type == 1) {
        if (next.timeNs <= prev.timeNs) return static_cast<float>(next.value);
        double ratio = double(tNs - prev.timeNs) / double(next.timeNs - prev.timeNs);
        if (ratio < 0.0) ratio = 0.0;
        if (ratio > 1.0) ratio = 1.0;
        return static_cast<float>(prev.value + (next.value - prev.value) * ratio);
    }
    return static_cast<float>(prev.value);
}

#endif // AUDIO_INTERNAL_H
