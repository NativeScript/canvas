#include "audio_context.h"
#include "audio_internal.h"
#include "hrtf_convolver.h"

#include <android/log.h>
#include <jni.h>
#include <string>
#include <vector>
#include <cstring>
#include <cmath>
#include <algorithm>
#include <atomic>
#include <chrono>

#ifdef HAS_OBOE

#include <oboe/Oboe.h>

#endif

extern "C" JNIEXPORT jstring JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeDecodeAudioData(JNIEnv *env, jobject thiz,
                                                                      jbyteArray bytes) {
    jsize len = env->GetArrayLength(bytes);
    std::vector<uint8_t> data(len);
    if (len > 0) {
        auto *arr = (jbyte *) env->GetPrimitiveArrayCritical(bytes, nullptr);
        if (arr) {
            memcpy(data.data(), arr, static_cast<size_t>(len));
            env->ReleasePrimitiveArrayCritical(bytes, arr, JNI_ABORT);
        } else {
            env->GetByteArrayRegion(bytes, 0, len, reinterpret_cast<jbyte *>(data.data()));
        }
    }
    std::string id = NativeEngine::getInstance().decodeAudioData(data);
    if (id.empty()) return nullptr;
    return env->NewStringUTF(id.c_str());
}

extern "C" JNIEXPORT jstring JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeCreateOscillator(JNIEnv *env, jobject thiz,
                                                                       jstring jtype,
                                                                       jdouble frequency) {
    const char *type = env->GetStringUTFChars(jtype, nullptr);
    std::string id = NativeEngine::getInstance().createOscillator(type ? type : "sine", frequency);
    if (type) env->ReleaseStringUTFChars(jtype, type);
    if (id.empty()) return nullptr;
    return env->NewStringUTF(id.c_str());
}

extern "C" JNIEXPORT jstring JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeCreateGain(JNIEnv *env, jobject thiz) {
    std::string id = genId();
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_CREATE_GAIN;
    cmd.id = id;
    cmd.gainValue = 1.0;
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    return env->NewStringUTF(id.c_str());
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeSetGain(JNIEnv *env, jobject thiz,
                                                              jstring jgainId, jdouble value) {
    const char *gid = env->GetStringUTFChars(jgainId, nullptr);
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_SET_GAIN;
    cmd.id = gid ? gid : std::string();
    cmd.gainValue = value;
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    if (gid) env->ReleaseStringUTFChars(jgainId, gid);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeScheduleGainSet(JNIEnv *env, jobject thiz,
                                                                      jstring jgainId, jint jrate,
                                                                      jlong timeNs,
                                                                      jdouble value) {
    const char *gid = env->GetStringUTFChars(jgainId, nullptr);
    if (gid) {
        NativeEngine::getInstance().scheduleGainEvent(std::string(gid), 0, static_cast<int>(jrate),
                                                      value,
                                                      static_cast<int64_t>(timeNs));
        env->ReleaseStringUTFChars(jgainId, gid);
    }
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeScheduleGainRamp(JNIEnv *env, jobject thiz,
                                                                       jstring jgainId, jint jrate,
                                                                       jlong timeNs,
                                                                       jdouble value) {
    const char *gid = env->GetStringUTFChars(jgainId, nullptr);
    if (gid) {
        NativeEngine::getInstance().scheduleGainEvent(std::string(gid), 1, static_cast<int>(jrate),
                                                      value,
                                                      static_cast<int64_t>(timeNs));
        env->ReleaseStringUTFChars(jgainId, gid);
    }
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeSchedulePannerSet(JNIEnv *env, jobject thiz,
                                                                        jstring jpannerId,
                                                                        jint jparamType, jint jrate,
                                                                        jlong timeNs,
                                                                        jdouble value) {
    const char *pid = env->GetStringUTFChars(jpannerId, nullptr);
    if (pid) {
        NativeEngine::getInstance().schedulePannerEvent(std::string(pid),
                                                        static_cast<int>(jparamType), 0,
                                                        static_cast<int>(jrate), value,
                                                        static_cast<int64_t>(timeNs));
        env->ReleaseStringUTFChars(jpannerId, pid);
    }
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeSchedulePannerRamp(JNIEnv *env, jobject thiz,
                                                                         jstring jpannerId,
                                                                         jint jparamType,
                                                                         jint jrate,
                                                                         jlong timeNs,
                                                                         jdouble value) {
    const char *pid = env->GetStringUTFChars(jpannerId, nullptr);
    if (pid) {
        NativeEngine::getInstance().schedulePannerEvent(std::string(pid),
                                                        static_cast<int>(jparamType), 1,
                                                        static_cast<int>(jrate), value,
                                                        static_cast<int64_t>(timeNs));
        env->ReleaseStringUTFChars(jpannerId, pid);
    }
}

extern "C" JNIEXPORT jdoubleArray JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeGetPannerParamValues(JNIEnv *env,
                                                                           jobject thiz,
                                                                           jstring jpannerId,
                                                                           jint jparamType,
                                                                           jlong jstartNs,
                                                                           jdouble sampleRate,
                                                                           jint frameCount) {
    const char *pid = env->GetStringUTFChars(jpannerId, nullptr);
    if (!pid) return nullptr;
    std::vector<double> vals = NativeEngine::getInstance().getPannerParamValues(std::string(pid),
                                                                                static_cast<int>(jparamType),
                                                                                static_cast<int64_t>(jstartNs),
                                                                                static_cast<double>(sampleRate),
                                                                                static_cast<int>(frameCount));
    env->ReleaseStringUTFChars(jpannerId, pid);
    jdoubleArray arr = env->NewDoubleArray(static_cast<jsize>(vals.size()));
    if (!arr) return nullptr;
    env->SetDoubleArrayRegion(arr, 0, static_cast<jsize>(vals.size()), vals.data());
    return arr;
}

extern "C" JNIEXPORT jdoubleArray JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeGetGainParamValues(JNIEnv *env, jobject thiz,
                                                                         jstring jgainId,
                                                                         jlong jstartNs,
                                                                         jdouble sampleRate,
                                                                         jint frameCount) {
    const char *gid = env->GetStringUTFChars(jgainId, nullptr);
    if (!gid) return nullptr;
    std::vector<double> vals = NativeEngine::getInstance().getGainParamValues(std::string(gid),
                                                                              static_cast<int64_t>(jstartNs),
                                                                              static_cast<double>(sampleRate),
                                                                              static_cast<int>(frameCount));
    env->ReleaseStringUTFChars(jgainId, gid);
    jdoubleArray arr = env->NewDoubleArray(static_cast<jsize>(vals.size()));
    if (!arr) return nullptr;
    env->SetDoubleArrayRegion(arr, 0, static_cast<jsize>(vals.size()), vals.data());
    return arr;
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeCancelGainEvents(JNIEnv *env, jobject thiz,
                                                                       jstring jgainId,
                                                                       jlong timeNs) {
    const char *gid = env->GetStringUTFChars(jgainId, nullptr);
    if (gid) {
        NativeEngine::getInstance().cancelGainEvents(std::string(gid),
                                                     static_cast<int64_t>(timeNs));
        env->ReleaseStringUTFChars(jgainId, gid);
    }
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeCancelPannerEvents(JNIEnv *env, jobject thiz,
                                                                         jstring jpannerId,
                                                                         jint jparamType,
                                                                         jlong timeNs) {
    const char *pid = env->GetStringUTFChars(jpannerId, nullptr);
    if (pid) {
        NativeEngine::getInstance().cancelPannerEvents(std::string(pid),
                                                       static_cast<int>(jparamType),
                                                       static_cast<int64_t>(timeNs));
        env->ReleaseStringUTFChars(jpannerId, pid);
    }
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeCancelAndHoldGainEvents(JNIEnv *env,
                                                                              jobject thiz,
                                                                              jstring jgainId,
                                                                              jint jrate,
                                                                              jlong timeNs,
                                                                              jdouble value) {
    const char *gid = env->GetStringUTFChars(jgainId, nullptr);
    if (gid) {
        NativeEngine::getInstance().cancelAndHoldGainEvents(std::string(gid),
                                                            static_cast<int>(jrate),
                                                            static_cast<double>(value),
                                                            static_cast<int64_t>(timeNs));
        env->ReleaseStringUTFChars(jgainId, gid);
    }
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeCancelAndHoldPannerEvents(JNIEnv *env,
                                                                                jobject thiz,
                                                                                jstring jpannerId,
                                                                                jint jparamType,
                                                                                jint jrate,
                                                                                jlong timeNs,
                                                                                jdouble value) {
    const char *pid = env->GetStringUTFChars(jpannerId, nullptr);
    if (pid) {
        NativeEngine::getInstance().cancelAndHoldPannerEvents(std::string(pid),
                                                              static_cast<int>(jparamType),
                                                              static_cast<int>(jrate),
                                                              static_cast<double>(value),
                                                              static_cast<int64_t>(timeNs));
        env->ReleaseStringUTFChars(jpannerId, pid);
    }
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeAttachGain(JNIEnv *env, jobject thiz,
                                                                 jstring jvoiceId,
                                                                 jstring jgainId) {
    const char *vid = env->GetStringUTFChars(jvoiceId, nullptr);
    const char *gid = env->GetStringUTFChars(jgainId, nullptr);
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_ATTACH_GAIN;
    cmd.id = vid ? vid : std::string();
    cmd.gainId = gid ? gid : std::string();
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    if (vid) env->ReleaseStringUTFChars(jvoiceId, vid);
    if (gid) env->ReleaseStringUTFChars(jgainId, gid);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeAttachPlaybackRate(JNIEnv *env, jobject thiz,
                                                                         jstring jvoiceId,
                                                                         jstring jplaybackRateId) {
    const char *vid = env->GetStringUTFChars(jvoiceId, nullptr);
    const char *pid = env->GetStringUTFChars(jplaybackRateId, nullptr);
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_ATTACH_PLAYBACK_RATE;
    cmd.id = vid ? vid : std::string();
    cmd.playbackRateId = pid ? pid : std::string();
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    if (vid) env->ReleaseStringUTFChars(jvoiceId, vid);
    if (pid) env->ReleaseStringUTFChars(jplaybackRateId, pid);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeAttachPlaybackRate__Ljava_lang_String_2Ljava_lang_String_2II(
        JNIEnv *env, jobject thiz,
        jstring jvoiceId,
        jstring jplaybackRateId,
        jint joutput,
        jint jinput) {
    const char *vid = env->GetStringUTFChars(jvoiceId, nullptr);
    const char *pid = env->GetStringUTFChars(jplaybackRateId, nullptr);
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_ATTACH_PLAYBACK_RATE;
    cmd.id = vid ? vid : std::string();
    cmd.playbackRateId = pid ? pid : std::string();
    cmd.outputIndex = static_cast<int>(joutput);
    cmd.inputIndex = static_cast<int>(jinput);
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    if (vid) env->ReleaseStringUTFChars(jvoiceId, vid);
    if (pid) env->ReleaseStringUTFChars(jplaybackRateId, pid);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeDetachPlaybackRate(JNIEnv *env, jobject thiz,
                                                                         jstring jplaybackRateId) {
    const char *pid = env->GetStringUTFChars(jplaybackRateId, nullptr);
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_DETACH_PLAYBACK_RATE;
    cmd.id = pid ? pid : std::string();
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    if (pid) env->ReleaseStringUTFChars(jplaybackRateId, pid);
}


extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeReleasePlaybackRate(JNIEnv *env, jobject thiz,
                                                                          jstring jplaybackRateId) {
    const char *pid = env->GetStringUTFChars(jplaybackRateId, nullptr);
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_FREE_PLAYBACK_RATE;
    cmd.id = pid ? pid : std::string();
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    if (pid) env->ReleaseStringUTFChars(jplaybackRateId, pid);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeAttachGain__Ljava_lang_String_2Ljava_lang_String_2II(
        JNIEnv *env, jobject thiz,
        jstring jvoiceId,
        jstring jgainId,
        jint joutput,
        jint jinput) {
    const char *vid = env->GetStringUTFChars(jvoiceId, nullptr);
    const char *gid = env->GetStringUTFChars(jgainId, nullptr);
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_ATTACH_GAIN;
    cmd.id = vid ? vid : std::string();
    cmd.gainId = gid ? gid : std::string();
    cmd.outputIndex = static_cast<int>(joutput);
    cmd.inputIndex = static_cast<int>(jinput);
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    if (vid) env->ReleaseStringUTFChars(jvoiceId, vid);
    if (gid) env->ReleaseStringUTFChars(jgainId, gid);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeDetachGain(JNIEnv *env, jobject thiz,
                                                                 jstring jgainId) {
    const char *gid = env->GetStringUTFChars(jgainId, nullptr);
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_DETACH_GAIN;
    cmd.id = gid ? gid : std::string();
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    if (gid) env->ReleaseStringUTFChars(jgainId, gid);
}


extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeReleaseGain(JNIEnv *env, jobject thiz,
                                                                  jstring jgainId) {
    const char *gid = env->GetStringUTFChars(jgainId, nullptr);
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_FREE_GAIN;
    cmd.id = gid ? gid : std::string();
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    if (gid) env->ReleaseStringUTFChars(jgainId, gid);
}

extern "C" JNIEXPORT jstring JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeCreateBiquad(JNIEnv *env, jobject thiz,
                                                                   jstring jtype,
                                                                   jdouble frequency,
                                                                   jdouble Q,
                                                                   jdouble gain) {
    const char *type = env->GetStringUTFChars(jtype, nullptr);
    std::string id = genId();
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_CREATE_BIQUAD;
    cmd.id = id;
    cmd.biquadType = type ? type : std::string("peaking");
    cmd.biquadFrequency = frequency;
    cmd.biquadQ = Q;
    cmd.biquadGain = gain;
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    if (type) env->ReleaseStringUTFChars(jtype, type);
    return env->NewStringUTF(id.c_str());
}

extern "C" JNIEXPORT jstring JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeCreateIIR(JNIEnv *env, jobject thiz,
                                                                jdoubleArray jfeedforward,
                                                                jdoubleArray jfeedback) {
    std::string id = genId();
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_CREATE_IIR;
    cmd.id = id;
    if (jfeedforward != nullptr) {
        jsize len = env->GetArrayLength(jfeedforward);
        if (len > 0) {
            jdouble *arr = (jdouble *) env->GetPrimitiveArrayCritical(jfeedforward, nullptr);
            if (arr) {
                cmd.iirFeedforward.assign(arr, arr + len);
                env->ReleasePrimitiveArrayCritical(jfeedforward, arr, 0);
            }
        }
    }
    if (jfeedback != nullptr) {
        jsize len = env->GetArrayLength(jfeedback);
        if (len > 0) {
            auto *arr = (jdouble *) env->GetPrimitiveArrayCritical(jfeedback, nullptr);
            if (arr) {
                cmd.iirFeedback.assign(arr, arr + len);
                env->ReleasePrimitiveArrayCritical(jfeedback, arr, 0);
            }
        }
    }
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    return env->NewStringUTF(id.c_str());
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeReleaseIIR(JNIEnv *env, jobject thiz,
                                                                 jstring jid) {
    const char *id = env->GetStringUTFChars(jid, nullptr);
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_FREE_IIR;
    cmd.id = id ? id : std::string();
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    if (id) env->ReleaseStringUTFChars(jid, id);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeSetAudioThreadLoggingEnabled(JNIEnv *env,
                                                                                   jobject thiz,
                                                                                   jboolean jenabled) {
    g_audioThreadLoggingEnabled.store(jenabled != JNI_FALSE, std::memory_order_relaxed);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeSetGlobalAudioThreadLoggingEnabled(JNIEnv *env,
                                                                                         jclass clazz,
                                                                                         jboolean jenabled) {
    g_audioThreadLoggingEnabled.store(jenabled != JNI_FALSE, std::memory_order_relaxed);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeGetIIRFrequencyResponse(JNIEnv *env,
                                                                              jobject thiz,
                                                                              jstring jid,
                                                                              jdoubleArray jfrequencyHz,
                                                                              jdoubleArray jmagResponse,
                                                                              jdoubleArray jphaseResponse,
                                                                              jdouble jsampleRate) {
    if (jid == nullptr || jfrequencyHz == nullptr || jmagResponse == nullptr ||
        jphaseResponse == nullptr)
        return;
    const char *cid = env->GetStringUTFChars(jid, nullptr);
    std::string id = cid ? std::string(cid) : std::string();
    if (cid) env->ReleaseStringUTFChars(jid, cid);

    auto coeffs = NativeEngine::getInstance().getIIRCoefficients(id);
    std::vector<double> ff = std::move(coeffs.first);
    std::vector<double> fb = std::move(coeffs.second);

    jsize len = env->GetArrayLength(jfrequencyHz);
    jsize magLen = env->GetArrayLength(jmagResponse);
    jsize phaseLen = env->GetArrayLength(jphaseResponse);
    int n = (int) std::min((jsize) std::min(len, magLen), phaseLen);
    if (n <= 0) return;

    if (ff.empty() || fb.empty()) {
        auto *pm = (jdouble *) env->GetPrimitiveArrayCritical(jmagResponse, nullptr);
        auto *pp = (jdouble *) env->GetPrimitiveArrayCritical(jphaseResponse, nullptr);
        if (pm && pp) {
            for (int i = 0; i < n; ++i) {
                pm[i] = 0.0;
                pp[i] = 0.0;
            }
        }
        if (pm) env->ReleasePrimitiveArrayCritical(jmagResponse, pm, 0);
        if (pp) env->ReleasePrimitiveArrayCritical(jphaseResponse, pp, 0);
        return;
    }

    auto *freqs = (jdouble *) env->GetPrimitiveArrayCritical(jfrequencyHz, nullptr);
    auto *mag = (jdouble *) env->GetPrimitiveArrayCritical(jmagResponse, nullptr);
    auto *phase = (jdouble *) env->GetPrimitiveArrayCritical(jphaseResponse, nullptr);

    double sr = jsampleRate > 0.0 ? jsampleRate : 48000.0;
    double fb0 = !fb.empty() ? fb[0] : 1.0;

    if (freqs && mag && phase) {
        for (int i = 0; i < n; ++i) {
            double w = (2.0 * M_PI * freqs[i]) / sr;
            double nR = 0.0, nI = 0.0, dR = 0.0, dI = 0.0;
            for (size_t k = 0; k < ff.size(); ++k) {
                double c = ff[k] / fb0;
                nR += c * std::cos(w * (double) k);
                nI -= c * std::sin(w * (double) k);
            }
            for (size_t k = 0; k < fb.size(); ++k) {
                double c = fb[k] / fb0;
                dR += c * std::cos(w * (double) k);
                dI -= c * std::sin(w * (double) k);
            }
            double numMag = std::hypot(nR, nI);
            double denMag = std::hypot(dR, dI);
            mag[i] = denMag == 0.0 ? 0.0 : numMag / denMag;
            phase[i] = std::atan2(nI, nR) - std::atan2(dI, dR);
        }
        env->ReleasePrimitiveArrayCritical(jfrequencyHz, freqs, JNI_ABORT);
        env->ReleasePrimitiveArrayCritical(jmagResponse, mag, 0);
        env->ReleasePrimitiveArrayCritical(jphaseResponse, phase, 0);
        return;
    }

    jdouble *freqs2 = env->GetDoubleArrayElements(jfrequencyHz, nullptr);
    jdouble *mag2 = env->GetDoubleArrayElements(jmagResponse, nullptr);
    jdouble *phase2 = env->GetDoubleArrayElements(jphaseResponse, nullptr);
    if (freqs2 && mag2 && phase2) {
        for (int i = 0; i < n; ++i) {
            double w = (2.0 * M_PI * freqs2[i]) / sr;
            double nR = 0.0, nI = 0.0, dR = 0.0, dI = 0.0;
            for (size_t k = 0; k < ff.size(); ++k) {
                double c = ff[k] / fb0;
                nR += c * std::cos(w * (double) k);
                nI -= c * std::sin(w * (double) k);
            }
            for (size_t k = 0; k < fb.size(); ++k) {
                double c = fb[k] / fb0;
                dR += c * std::cos(w * (double) k);
                dI -= c * std::sin(w * (double) k);
            }
            double numMag = std::hypot(nR, nI);
            double denMag = std::hypot(dR, dI);
            mag2[i] = denMag == 0.0 ? 0.0 : numMag / denMag;
            phase2[i] = std::atan2(nI, nR) - std::atan2(dI, dR);
        }
    }
    if (freqs2) env->ReleaseDoubleArrayElements(jfrequencyHz, freqs2, JNI_ABORT);
    if (mag2) env->ReleaseDoubleArrayElements(jmagResponse, mag2, 0);
    if (phase2) env->ReleaseDoubleArrayElements(jphaseResponse, phase2, 0);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeGetIIRFrequencyResponseDirect(JNIEnv *env,
                                                                                    jobject thiz,
                                                                                    jstring jid,
                                                                                    jobject jfrequencyHz,
                                                                                    jobject jmagResponse,
                                                                                    jobject jphaseResponse,
                                                                                    jdouble jsampleRate) {
    if (jid == nullptr || jfrequencyHz == nullptr || jmagResponse == nullptr ||
        jphaseResponse == nullptr)
        return;
    const char *cid = env->GetStringUTFChars(jid, nullptr);
    std::string id = cid ? std::string(cid) : std::string();
    if (cid) env->ReleaseStringUTFChars(jid, cid);

    auto coeffs = NativeEngine::getInstance().getIIRCoefficients(id);
    std::vector<double> ff = std::move(coeffs.first);
    std::vector<double> fb = std::move(coeffs.second);


    void *freqAddr = env->GetDirectBufferAddress(jfrequencyHz);
    void *magAddr = env->GetDirectBufferAddress(jmagResponse);
    void *phaseAddr = env->GetDirectBufferAddress(jphaseResponse);
    jlong freqCap = env->GetDirectBufferCapacity(jfrequencyHz);
    jlong magCap = env->GetDirectBufferCapacity(jmagResponse);
    jlong phaseCap = env->GetDirectBufferCapacity(jphaseResponse);
    if (!freqAddr || !magAddr || !phaseAddr || freqCap <= 0 || magCap <= 0 || phaseCap <= 0) return;

    int n = (int) std::min((jlong) std::min(freqCap, magCap), phaseCap);
    if (n <= 0) return;

    auto *freqs = reinterpret_cast<float *>(freqAddr);
    auto *mag = reinterpret_cast<float *>(magAddr);
    auto *phase = reinterpret_cast<float *>(phaseAddr);

    if (ff.empty() || fb.empty()) {
        for (int i = 0; i < n; ++i) {
            mag[i] = 0.0f;
            phase[i] = 0.0f;
        }
        return;
    }

    double sr = jsampleRate > 0.0 ? jsampleRate : 48000.0;
    double fb0 = !fb.empty() ? fb[0] : 1.0;

    for (int i = 0; i < n; ++i) {
        double w = (2.0 * M_PI * (double) freqs[i]) / sr;
        double nR = 0.0, nI = 0.0, dR = 0.0, dI = 0.0;
        for (size_t k = 0; k < ff.size(); ++k) {
            double c = ff[k] / fb0;
            nR += c * std::cos(w * (double) k);
            nI -= c * std::sin(w * (double) k);
        }
        for (size_t k = 0; k < fb.size(); ++k) {
            double c = fb[k] / fb0;
            dR += c * std::cos(w * (double) k);
            dI -= c * std::sin(w * (double) k);
        }
        double numMag = std::hypot(nR, nI);
        double denMag = std::hypot(dR, dI);
        mag[i] = denMag == 0.0 ? 0.0f : (float) (numMag / denMag);
        phase[i] = (float) (std::atan2(nI, nR) - std::atan2(dI, dR));
    }
}

extern "C" JNIEXPORT jstring JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeCreatePeriodicWave(JNIEnv *env, jobject thiz,
                                                                         jdoubleArray jreal,
                                                                         jdoubleArray jimag,
                                                                         jboolean jdisable) {
    std::string id = genId();
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_CREATE_PERIODICWAVE;
    cmd.id = id;
    if (jreal != nullptr) {
        jsize len = env->GetArrayLength(jreal);
        if (len > 0) {
            auto *arr = (jdouble *) env->GetPrimitiveArrayCritical(jreal, nullptr);
            if (arr) {
                cmd.periodicReal.assign(arr, arr + len);
                env->ReleasePrimitiveArrayCritical(jreal, arr, 0);
            }
        }
    }
    if (jimag != nullptr) {
        jsize len = env->GetArrayLength(jimag);
        if (len > 0) {
            auto *arr = (jdouble *) env->GetPrimitiveArrayCritical(jimag, nullptr);
            if (arr) {
                cmd.periodicImag.assign(arr, arr + len);
                env->ReleasePrimitiveArrayCritical(jimag, arr, 0);
            }
        }
    }
    cmd.periodicDisableNormalization = (jdisable != JNI_FALSE);
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    return env->NewStringUTF(id.c_str());
}

extern "C" JNIEXPORT jstring JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeCreatePeriodicWaveDirect(JNIEnv *env,
                                                                               jobject thiz,
                                                                               jobject jreal,
                                                                               jobject jimag,
                                                                               jboolean jdisable) {
    std::string id = genId();
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_CREATE_PERIODICWAVE;
    cmd.id = id;
    if (jreal != nullptr) {
        jlong cap = env->GetDirectBufferCapacity(jreal);
        if (cap > 0) {
            auto *farr = reinterpret_cast<float *>(env->GetDirectBufferAddress(jreal));
            if (farr) {
                if (cap > 0) {
                    cmd.periodicReal.resize((size_t) cap);
                    for (jsize i = 0; i < cap; ++i) cmd.periodicReal[(size_t) i] = (double) farr[i];
                }
            }
        }
    }
    if (jimag != nullptr) {
        jlong cap = env->GetDirectBufferCapacity(jimag);
        if (cap > 0) {
            auto *farr = reinterpret_cast<float *>(env->GetDirectBufferAddress(jimag));
            if (farr) {
                if (cap > 0) {
                    cmd.periodicImag.resize((size_t) cap);
                    for (jsize i = 0; i < cap; ++i) cmd.periodicImag[(size_t) i] = (double) farr[i];
                }
            }
        }
    }
    cmd.periodicDisableNormalization = (jdisable != JNI_FALSE);
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    return env->NewStringUTF(id.c_str());
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeReleasePeriodicWave(JNIEnv *env, jobject thiz,
                                                                          jstring jid) {
    const char *id = env->GetStringUTFChars(jid, nullptr);
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_FREE_PERIODICWAVE;
    cmd.id = id ? id : std::string();
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    if (id) env->ReleaseStringUTFChars(jid, id);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeAttachPeriodicWave(JNIEnv *env, jobject thiz,
                                                                         jstring jvoiceId,
                                                                         jstring jwaveId) {
    const char *vid = env->GetStringUTFChars(jvoiceId, nullptr);
    const char *pid = env->GetStringUTFChars(jwaveId, nullptr);
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_ATTACH_PERIODICWAVE;
    cmd.id = vid ? vid : std::string();
    cmd.periodicWaveId = pid ? pid : std::string();
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    if (vid) env->ReleaseStringUTFChars(jvoiceId, vid);
    if (pid) env->ReleaseStringUTFChars(jwaveId, pid);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeSetBiquadParams(JNIEnv *env, jobject thiz,
                                                                      jstring jbiquadId,
                                                                      jdouble frequency,
                                                                      jdouble Q,
                                                                      jdouble gain,
                                                                      jstring jtype) {
    const char *bid = env->GetStringUTFChars(jbiquadId, nullptr);
    const char *type = env->GetStringUTFChars(jtype, nullptr);
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_SET_BIQUAD_PARAMS;
    cmd.id = bid ? bid : std::string();
    cmd.biquadFrequency = frequency;
    cmd.biquadQ = Q;
    cmd.biquadGain = gain;
    cmd.biquadType = type ? type : std::string();
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    if (bid) env->ReleaseStringUTFChars(jbiquadId, bid);
    if (type) env->ReleaseStringUTFChars(jtype, type);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeSetWaveShaperCurveArray(JNIEnv *env,
                                                                              jobject thiz,
                                                                              jstring jid,
                                                                              jfloatArray jcurve,
                                                                              jstring joversample) {
    const char *id = jid ? env->GetStringUTFChars(jid, nullptr) : nullptr;
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_SET_WAVESHAPER_CURVE;
    cmd.id = id ? id : std::string();
    if (jcurve != nullptr) {
        jsize len = env->GetArrayLength(jcurve);
        if (len > 0) {
            jfloat *arr = (jfloat *) env->GetPrimitiveArrayCritical(jcurve, nullptr);
            if (arr) {
                cmd.waveShaperCurve.resize((size_t) len);
                for (jsize i = 0; i < len; ++i) cmd.waveShaperCurve[(size_t) i] = arr[i];
                env->ReleasePrimitiveArrayCritical(jcurve, arr, 0);
            }
        }
    }
    if (joversample != nullptr) {
        const char *ov = env->GetStringUTFChars(joversample, nullptr);
        if (ov) cmd.waveShaperOversample = ov;
        if (ov) env->ReleaseStringUTFChars(joversample, ov);
    }
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    if (id) env->ReleaseStringUTFChars(jid, id);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeSetWaveShaperCurveDirect(JNIEnv *env,
                                                                               jobject thiz,
                                                                               jstring jid,
                                                                               jobject jdata,
                                                                               jstring joversample) {
    const char *id = jid ? env->GetStringUTFChars(jid, nullptr) : nullptr;
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_SET_WAVESHAPER_CURVE;
    cmd.id = id ? id : std::string();
    if (jdata != nullptr) {
        jlong cap = env->GetDirectBufferCapacity(jdata);
        if (cap > 0) {
            float *farr = reinterpret_cast<float *>(env->GetDirectBufferAddress(jdata));
            if (farr) {
                if (cap > 0) {
                    cmd.waveShaperCurve.resize((size_t) cap);
                    for (jsize i = 0; i < cap; ++i) cmd.waveShaperCurve[(size_t) i] = farr[i];
                }
            }
        }
    }
    if (joversample != nullptr) {
        const char *ov = env->GetStringUTFChars(joversample, nullptr);
        if (ov) cmd.waveShaperOversample = ov;
        if (ov) env->ReleaseStringUTFChars(joversample, ov);
    }
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    if (id) env->ReleaseStringUTFChars(jid, id);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeSetWaveShaperOversample(JNIEnv *env,
                                                                              jobject thiz,
                                                                              jstring jid,
                                                                              jstring joversample) {
    const char *id = jid ? env->GetStringUTFChars(jid, nullptr) : nullptr;
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_SET_WAVESHAPER_CURVE;
    cmd.id = id ? id : std::string();
    if (joversample != nullptr) {
        const char *ov = env->GetStringUTFChars(joversample, nullptr);
        if (ov) cmd.waveShaperOversample = ov;
        if (ov) env->ReleaseStringUTFChars(joversample, ov);
    }
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    if (id) env->ReleaseStringUTFChars(jid, id);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeAttachBiquad(JNIEnv *env, jobject thiz,
                                                                   jstring jvoiceId,
                                                                   jstring jbiquadId) {
    const char *vid = env->GetStringUTFChars(jvoiceId, nullptr);
    const char *bid = env->GetStringUTFChars(jbiquadId, nullptr);
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_ATTACH_BIQUAD;
    cmd.id = vid ? vid : std::string();
    cmd.biquadId = bid ? bid : std::string();
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    if (vid) env->ReleaseStringUTFChars(jvoiceId, vid);
    if (bid) env->ReleaseStringUTFChars(jbiquadId, bid);
}


extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeAttachBiquad__Ljava_lang_String_2Ljava_lang_String_2II(
        JNIEnv *env, jobject thiz,
        jstring jvoiceId,
        jstring jbiquadId,
        jint joutput,
        jint jinput) {
    const char *vid = env->GetStringUTFChars(jvoiceId, nullptr);
    const char *bid = env->GetStringUTFChars(jbiquadId, nullptr);
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_ATTACH_BIQUAD;
    cmd.id = vid ? vid : std::string();
    cmd.biquadId = bid ? bid : std::string();
    cmd.outputIndex = static_cast<int>(joutput);
    cmd.inputIndex = static_cast<int>(jinput);
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    if (vid) env->ReleaseStringUTFChars(jvoiceId, vid);
    if (bid) env->ReleaseStringUTFChars(jbiquadId, bid);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeDetachBiquad(JNIEnv *env, jobject thiz,
                                                                   jstring jbiquadId) {
    const char *bid = env->GetStringUTFChars(jbiquadId, nullptr);
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_DETACH_BIQUAD;
    cmd.id = bid ? bid : std::string();
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    if (bid) env->ReleaseStringUTFChars(jbiquadId, bid);
}


extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeAttachPanner__Ljava_lang_String_2Ljava_lang_String_2II(
        JNIEnv *env, jobject thiz,
        jstring jvoiceId,
        jstring jpannerId,
        jint joutput,
        jint jinput) {
    const char *vid = env->GetStringUTFChars(jvoiceId, nullptr);
    const char *pid = env->GetStringUTFChars(jpannerId, nullptr);
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_ATTACH_PANNER;
    cmd.id = vid ? vid : std::string();
    cmd.pannerId = pid ? pid : std::string();
    cmd.outputIndex = static_cast<int>(joutput);
    cmd.inputIndex = static_cast<int>(jinput);
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    if (vid) env->ReleaseStringUTFChars(jvoiceId, vid);
    if (pid) env->ReleaseStringUTFChars(jpannerId, pid);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeReleaseBiquad(JNIEnv *env, jobject thiz,
                                                                    jstring jbiquadId) {
    const char *bid = env->GetStringUTFChars(jbiquadId, nullptr);
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_FREE_BIQUAD;
    cmd.id = bid ? bid : std::string();
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    if (bid) env->ReleaseStringUTFChars(jbiquadId, bid);
}

extern "C" JNIEXPORT jstring JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeCreatePanner(JNIEnv *env, jobject thiz,
                                                                   jstring jcontextId,
                                                                   jdouble positionX,
                                                                   jdouble positionY,
                                                                   jdouble positionZ,
                                                                   jdouble orientationX,
                                                                   jdouble orientationY,
                                                                   jdouble orientationZ,
                                                                   jdouble pan,
                                                                   jint distanceModel,
                                                                   jint panningModel,
                                                                   jdouble refDistance,
                                                                   jdouble maxDistance,
                                                                   jdouble rolloffFactor,
                                                                   jdouble coneInnerAngle,
                                                                   jdouble coneOuterAngle,
                                                                   jdouble coneOuterGain) {
    std::string id = genId();
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_CREATE_PANNER;
    cmd.id = id;
    const char *cid = env->GetStringUTFChars(jcontextId, nullptr);
    if (cid) cmd.contextId = cid;
    if (cid) env->ReleaseStringUTFChars(jcontextId, cid);
    cmd.pannerPositionX = positionX;
    cmd.pannerPositionY = positionY;
    cmd.pannerPositionZ = positionZ;
    cmd.pannerOrientationX = orientationX;
    cmd.pannerOrientationY = orientationY;
    cmd.pannerOrientationZ = orientationZ;
    cmd.pannerPan = pan;
    cmd.pannerDistanceModel = distanceModel;
    cmd.pannerPanningModel = panningModel;
    cmd.pannerRefDistance = refDistance;
    cmd.pannerMaxDistance = maxDistance;
    cmd.pannerRolloffFactor = rolloffFactor;
    cmd.pannerConeInnerAngle = coneInnerAngle;
    cmd.pannerConeOuterAngle = coneOuterAngle;
    cmd.pannerConeOuterGain = coneOuterGain;
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    return env->NewStringUTF(id.c_str());
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeSetPannerParams(JNIEnv *env, jobject thiz,
                                                                      jstring jpannerId,
                                                                      jdouble positionX,
                                                                      jdouble positionY,
                                                                      jdouble positionZ,
                                                                      jdouble orientationX,
                                                                      jdouble orientationY,
                                                                      jdouble orientationZ,
                                                                      jdouble pan,
                                                                      jint distanceModel,
                                                                      jint panningModel,
                                                                      jdouble refDistance,
                                                                      jdouble maxDistance,
                                                                      jdouble rolloffFactor,
                                                                      jdouble coneInnerAngle,
                                                                      jdouble coneOuterAngle,
                                                                      jdouble coneOuterGain) {
    const char *pid = env->GetStringUTFChars(jpannerId, nullptr);
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_SET_PANNER_PARAMS;
    cmd.id = pid ? pid : std::string();
    cmd.pannerPositionX = positionX;
    cmd.pannerPositionY = positionY;
    cmd.pannerPositionZ = positionZ;
    cmd.pannerOrientationX = orientationX;
    cmd.pannerOrientationY = orientationY;
    cmd.pannerOrientationZ = orientationZ;
    cmd.pannerPan = pan;
    cmd.pannerDistanceModel = distanceModel;
    cmd.pannerPanningModel = panningModel;
    cmd.pannerRefDistance = refDistance;
    cmd.pannerMaxDistance = maxDistance;
    cmd.pannerRolloffFactor = rolloffFactor;
    cmd.pannerConeInnerAngle = coneInnerAngle;
    cmd.pannerConeOuterAngle = coneOuterAngle;
    cmd.pannerConeOuterGain = coneOuterGain;
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    if (pid) env->ReleaseStringUTFChars(jpannerId, pid);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeSetListenerParams(JNIEnv *env, jobject thiz,
                                                                        jstring jcontextId,
                                                                        jdouble positionX,
                                                                        jdouble positionY,
                                                                        jdouble positionZ,
                                                                        jdouble forwardX,
                                                                        jdouble forwardY,
                                                                        jdouble forwardZ,
                                                                        jdouble upX,
                                                                        jdouble upY,
                                                                        jdouble upZ) {
    const char *cid = env->GetStringUTFChars(jcontextId, nullptr);
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_SET_LISTENER_PARAMS;
    cmd.contextId = cid ? cid : std::string();
    if (cid) env->ReleaseStringUTFChars(jcontextId, cid);
    cmd.listenerPositionX = positionX;
    cmd.listenerPositionY = positionY;
    cmd.listenerPositionZ = positionZ;
    cmd.listenerForwardX = forwardX;
    cmd.listenerForwardY = forwardY;
    cmd.listenerForwardZ = forwardZ;
    cmd.listenerUpX = upX;
    cmd.listenerUpY = upY;
    cmd.listenerUpZ = upZ;
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeScheduleListenerSet(JNIEnv *env, jobject thiz,
                                                                          jstring jcontextId,
                                                                          jint jparamType,
                                                                          jint jrate,
                                                                          jlong timeNs,
                                                                          jdouble value) {
    const char *cid = env->GetStringUTFChars(jcontextId, nullptr);
    if (cid) {
        NativeEngine::getInstance().scheduleListenerEvent(std::string(cid),
                                                          static_cast<int>(jparamType), 0,
                                                          static_cast<int>(jrate), value,
                                                          static_cast<int64_t>(timeNs));
        env->ReleaseStringUTFChars(jcontextId, cid);
    }
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeScheduleListenerRamp(JNIEnv *env,
                                                                           jobject thiz,
                                                                           jstring jcontextId,
                                                                           jint jparamType,
                                                                           jint jrate,
                                                                           jlong timeNs,
                                                                           jdouble value) {
    const char *cid = env->GetStringUTFChars(jcontextId, nullptr);
    if (cid) {
        NativeEngine::getInstance().scheduleListenerEvent(std::string(cid),
                                                          static_cast<int>(jparamType), 1,
                                                          static_cast<int>(jrate), value,
                                                          static_cast<int64_t>(timeNs));
        env->ReleaseStringUTFChars(jcontextId, cid);
    }
}

extern "C" JNIEXPORT jdoubleArray JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeGetListenerParamValues(JNIEnv *env,
                                                                             jobject thiz,
                                                                             jstring jcontextId,
                                                                             jint jparamType,
                                                                             jlong jstartNs,
                                                                             jdouble sampleRate,
                                                                             jint frameCount) {
    const char *cid = env->GetStringUTFChars(jcontextId, nullptr);
    if (!cid) return nullptr;
    std::vector<double> vals = NativeEngine::getInstance().getListenerParamValues(std::string(cid),
                                                                                  static_cast<int>(jparamType),
                                                                                  static_cast<int64_t>(jstartNs),
                                                                                  static_cast<double>(sampleRate),
                                                                                  static_cast<int>(frameCount));
    env->ReleaseStringUTFChars(jcontextId, cid);
    jdoubleArray arr = env->NewDoubleArray(static_cast<jsize>(vals.size()));
    if (!arr) return nullptr;
    env->SetDoubleArrayRegion(arr, 0, static_cast<jsize>(vals.size()), vals.data());
    return arr;
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeCancelListenerEvents(JNIEnv *env,
                                                                           jobject thiz,
                                                                           jstring jcontextId,
                                                                           jint jparamType,
                                                                           jlong timeNs) {
    const char *cid = env->GetStringUTFChars(jcontextId, nullptr);
    if (cid) {
        NativeEngine::getInstance().cancelListenerEvents(std::string(cid),
                                                         static_cast<int>(jparamType),
                                                         static_cast<int64_t>(timeNs));
        env->ReleaseStringUTFChars(jcontextId, cid);
    }
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeCancelAndHoldListenerEvents(JNIEnv *env,
                                                                                  jobject thiz,
                                                                                  jstring jcontextId,
                                                                                  jint jparamType,
                                                                                  jint jrate,
                                                                                  jlong timeNs,
                                                                                  jdouble value) {
    const char *cid = env->GetStringUTFChars(jcontextId, nullptr);
    if (cid) {
        NativeEngine::getInstance().cancelAndHoldListenerEvents(std::string(cid),
                                                                static_cast<int>(jparamType),
                                                                static_cast<int>(jrate),
                                                                static_cast<double>(value),
                                                                static_cast<int64_t>(timeNs));
        env->ReleaseStringUTFChars(jcontextId, cid);
    }
}

extern "C" JNIEXPORT jdouble JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeGetListenerParamValue(JNIEnv *env,
                                                                            jobject thiz,
                                                                            jstring jcontextId,
                                                                            jint jparamType) {
    const char *cid = env->GetStringUTFChars(jcontextId, nullptr);
    if (!cid) return 0.0;
    double v = NativeEngine::getInstance().getListenerParamValue(std::string(cid),
                                                                 static_cast<int>(jparamType));
    env->ReleaseStringUTFChars(jcontextId, cid);
    return static_cast<jdouble>(v);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeAttachPanner(JNIEnv *env, jobject thiz,
                                                                   jstring jvoiceId,
                                                                   jstring jpannerId) {
    const char *vid = env->GetStringUTFChars(jvoiceId, nullptr);
    const char *pid = env->GetStringUTFChars(jpannerId, nullptr);
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_ATTACH_PANNER;
    cmd.id = vid ? vid : std::string();
    cmd.pannerId = pid ? pid : std::string();
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    if (vid) env->ReleaseStringUTFChars(jvoiceId, vid);
    if (pid) env->ReleaseStringUTFChars(jpannerId, pid);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeAttachBiquadToPanner(JNIEnv *env, jobject thiz,
                                                                          jstring jpannerId,
                                                                          jstring jbiquadId) {
    const char *pid = env->GetStringUTFChars(jpannerId, nullptr);
    const char *bid = env->GetStringUTFChars(jbiquadId, nullptr);
    if (pid && bid) {
        NativeEngine::Command cmd;
        cmd.type = NativeEngine::CMD_SET_PANNER_PARAMS;
        cmd.id = std::string(pid);
        cmd.pannerBiquadId = std::string(bid);
        NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    }
    if (pid) env->ReleaseStringUTFChars(jpannerId, pid);
    if (bid) env->ReleaseStringUTFChars(jbiquadId, bid);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeDetachPanner(JNIEnv *env, jobject thiz,
                                                                   jstring jpannerId) {
    const char *pid = env->GetStringUTFChars(jpannerId, nullptr);
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_DETACH_PANNER;
    cmd.id = pid ? pid : std::string();
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    if (pid) env->ReleaseStringUTFChars(jpannerId, pid);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeReleasePanner(JNIEnv *env, jobject thiz,
                                                                    jstring jpannerId) {
    const char *pid = env->GetStringUTFChars(jpannerId, nullptr);
    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_FREE_PANNER;
    cmd.id = pid ? pid : std::string();
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));
    if (pid) env->ReleaseStringUTFChars(jpannerId, pid);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeStartOscillator(JNIEnv *env, jobject thiz,
                                                                      jstring jid, jstring jtype,
                                                                      jdouble frequency) {
    const char *id = env->GetStringUTFChars(jid, nullptr);
    const char *type = env->GetStringUTFChars(jtype, nullptr);
    NativeEngine::getInstance().startOscillator(id ? id : std::string(),
                                                type ? type : std::string(), frequency);
    if (id) env->ReleaseStringUTFChars(jid, id);
    if (type) env->ReleaseStringUTFChars(jtype, type);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeStopTrack(JNIEnv *env, jobject thiz,
                                                                jstring jid) {
    const char *id = env->GetStringUTFChars(jid, nullptr);
    NativeEngine::getInstance().stopTrack(id ? id : std::string());
    if (id) env->ReleaseStringUTFChars(jid, id);
}

extern "C" JNIEXPORT jstring JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeCreateBufferSource(JNIEnv *env, jobject thiz,
                                                                         jstring jbufferId,
                                                                         jbyteArray pcmBytes,
                                                                         jint sampleRate,
                                                                         jint channels) {
    jsize len = env->GetArrayLength(pcmBytes);
    std::vector<int16_t> pcm(len / 2);
    if (len > 0) {
        auto *arr = (jbyte *) env->GetPrimitiveArrayCritical(pcmBytes, nullptr);
        if (arr) {
            memcpy(reinterpret_cast<uint8_t *>(pcm.data()), arr, static_cast<size_t>(len));
            env->ReleasePrimitiveArrayCritical(pcmBytes, arr, JNI_ABORT);
        } else {
            env->GetByteArrayRegion(pcmBytes, 0, len, reinterpret_cast<jbyte *>(pcm.data()));
        }
    }
    std::string id = NativeEngine::getInstance().createBufferSource(std::move(pcm), sampleRate,
                                                                    channels);
    if (id.empty()) return nullptr;
    return env->NewStringUTF(id.c_str());
}

extern "C" JNIEXPORT jstring JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeRenderOffline(JNIEnv *env, jobject thiz,
                                                                    jobjectArray jtrackIds,
                                                                    jint jframes,
                                                                    jint jsampleRate,
                                                                    jint jchannels) {
    std::vector<std::string> trackIds;
    if (jtrackIds != nullptr) {
        jsize len = env->GetArrayLength(jtrackIds);
        for (jsize i = 0; i < len; ++i) {
            auto s = (jstring) env->GetObjectArrayElement(jtrackIds, i);
            const char *c = env->GetStringUTFChars(s, nullptr);
            if (c) trackIds.emplace_back(c);
            if (c) env->ReleaseStringUTFChars(s, c);
            env->DeleteLocalRef(s);
        }
    }
    std::string id = NativeEngine::getInstance().renderOfflineForTracks(trackIds, (int) jframes,
                                                                        (int) jsampleRate,
                                                                        (int) jchannels);
    if (id.empty()) return nullptr;
    return env->NewStringUTF(id.c_str());
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeStartBufferSource(JNIEnv *env, jobject thiz,
                                                                        jstring jid,
                                                                        jboolean jloop) {
    const char *id = env->GetStringUTFChars(jid, nullptr);
    NativeEngine::getInstance().startBufferSource(id ? id : std::string(), jloop == JNI_TRUE);
    if (id) env->ReleaseStringUTFChars(jid, id);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeInit(JNIEnv *env, jclass clazz) {
    JavaVM *jvm = nullptr;
    env->GetJavaVM(&jvm);
    NativeEngine::getInstance().setJavaVM(jvm);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeConfigureStream(JNIEnv *env, jobject thiz,
                                                                      jint sampleRate,
                                                                      jdouble latencyHintSec) {
    (void) env;
    (void) thiz;
    NativeEngine::getInstance().configureStream(static_cast<int>(sampleRate),
                                                static_cast<double>(latencyHintSec));
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeResume(JNIEnv *env, jobject thiz) {
    NativeEngine::getInstance().resume();
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeSetOutputDeviceId(JNIEnv *env, jobject thiz,
                                                                        jint deviceId) {
    (void) env;
    (void) thiz;
    NativeEngine::getInstance().setOutputDeviceId(static_cast<int>(deviceId));
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeSetPannerHRIR(JNIEnv *env, jobject thiz,
                                                                    jstring jpannerId,
                                                                    jfloatArray jleft,
                                                                    jfloatArray jright) {
    if (!jpannerId) return;
    const char *pid = env->GetStringUTFChars(jpannerId, nullptr);
    if (!pid) return;
    std::string pannerId(pid);

    jsize leftLen = jleft ? env->GetArrayLength(jleft) : 0;
    jsize rightLen = jright ? env->GetArrayLength(jright) : 0;
    if (leftLen <= 0 || rightLen <= 0 || leftLen != rightLen) {
        env->ReleaseStringUTFChars(jpannerId, pid);
        return;
    }

    jfloat *leftArr = (jfloat *) env->GetPrimitiveArrayCritical(jleft, nullptr);
    jfloat *rightArr = (jfloat *) env->GetPrimitiveArrayCritical(jright, nullptr);
    if (!leftArr || !rightArr) {
        if (leftArr) env->ReleasePrimitiveArrayCritical(jleft, leftArr, JNI_ABORT);
        if (rightArr) env->ReleasePrimitiveArrayCritical(jright, rightArr, JNI_ABORT);
        env->ReleaseStringUTFChars(jpannerId, pid);
        return;
    }

    std::vector<float> left(static_cast<size_t>(leftLen));
    std::vector<float> right(static_cast<size_t>(rightLen));
    std::memcpy(left.data(), leftArr, static_cast<size_t>(leftLen) * sizeof(float));
    std::memcpy(right.data(), rightArr, static_cast<size_t>(rightLen) * sizeof(float));

    env->ReleasePrimitiveArrayCritical(jleft, leftArr, JNI_ABORT);
    env->ReleasePrimitiveArrayCritical(jright, rightArr, JNI_ABORT);

    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_SET_PANNER_HRIR;
    cmd.id = pannerId;
    cmd.hrirLeft = std::make_shared<std::vector<float>>(std::move(left));
    cmd.hrirRight = std::make_shared<std::vector<float>>(std::move(right));
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));

    env->ReleaseStringUTFChars(jpannerId, pid);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeSetPannerHRIRDirect(JNIEnv *env, jobject thiz,
                                                                          jstring jpannerId,
                                                                          jobject jleft,
                                                                          jobject jright) {
    if (!jpannerId) return;
    const char *pid = env->GetStringUTFChars(jpannerId, nullptr);
    if (!pid) return;
    std::string pannerId(pid);

    if (jleft == nullptr && jright == nullptr) {
        NativeEngine::Command cmd;
        cmd.type = NativeEngine::CMD_SET_PANNER_HRIR;
        cmd.id = pannerId;
        cmd.clearHrir = true;
        NativeEngine::getInstance().enqueueCommand(std::move(cmd));
        env->ReleaseStringUTFChars(jpannerId, pid);
        return;
    }

    jlong leftCap = jleft ? env->GetDirectBufferCapacity(jleft) : 0;
    jlong rightCap = jright ? env->GetDirectBufferCapacity(jright) : 0;
    if (leftCap <= 0 || rightCap <= 0 || leftCap != rightCap) {
        env->ReleaseStringUTFChars(jpannerId, pid);
        return;
    }

    float *leftArr = reinterpret_cast<float *>(env->GetDirectBufferAddress(jleft));
    float *rightArr = reinterpret_cast<float *>(env->GetDirectBufferAddress(jright));
    if (!leftArr || !rightArr) {
        env->ReleaseStringUTFChars(jpannerId, pid);
        return;
    }

    size_t count = static_cast<size_t>(std::min(leftCap, rightCap));
    if (count == 0) {
        env->ReleaseStringUTFChars(jpannerId, pid);
        return;
    }

    std::vector<float> left(count);
    std::vector<float> right(count);
    std::memcpy(left.data(), leftArr, count * sizeof(float));
    std::memcpy(right.data(), rightArr, count * sizeof(float));

    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_SET_PANNER_HRIR;
    cmd.id = pannerId;
    cmd.hrirLeft = std::make_shared<std::vector<float>>(std::move(left));
    cmd.hrirRight = std::make_shared<std::vector<float>>(std::move(right));
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));

    env->ReleaseStringUTFChars(jpannerId, pid);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeSetPannerPartitionSize(JNIEnv *env,
                                                                             jobject thiz,
                                                                             jstring jpannerId,
                                                                             jint jpartitionSize) {
    if (!jpannerId) return;
    const char *pid = env->GetStringUTFChars(jpannerId, nullptr);
    if (!pid) return;

    int part = static_cast<int>(jpartitionSize);
    if (part <= 0) {
        env->ReleaseStringUTFChars(jpannerId, pid);
        return;
    }

    NativeEngine::Command cmd;
    cmd.type = NativeEngine::CMD_SET_PANNER_PARTITION_SIZE;
    cmd.id = pid;
    cmd.pannerPartitionSize = part;
    NativeEngine::getInstance().enqueueCommand(std::move(cmd));

    env->ReleaseStringUTFChars(jpannerId, pid);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeSuspend(JNIEnv *env, jobject thiz) {
    NativeEngine::getInstance().suspend();
}


extern "C" JNIEXPORT jstring JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeDecodeAudioDataDirect(JNIEnv *env,
                                                                            jobject thiz,
                                                                            jobject byteBuffer) {
    if (byteBuffer == nullptr) return nullptr;
    void *addr = env->GetDirectBufferAddress(byteBuffer);
    jlong cap = env->GetDirectBufferCapacity(byteBuffer);
    if (!addr || cap <= 0) return nullptr;
    std::string id = NativeEngine::getInstance().decodeAudioDataFromDirect(
            reinterpret_cast<const uint8_t *>(addr), static_cast<size_t>(cap), env, byteBuffer);
    if (id.empty()) return nullptr;
    return env->NewStringUTF(id.c_str());
}

extern "C" JNIEXPORT jstring JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeCreateBufferSourceDirect(JNIEnv *env,
                                                                               jobject thiz,
                                                                               jobject byteBuffer,
                                                                               jint sampleRate,
                                                                               jint channels,
                                                                               jint bytesPerSample) {
    if (byteBuffer == nullptr) return nullptr;
    void *addr = env->GetDirectBufferAddress(byteBuffer);
    jlong cap = env->GetDirectBufferCapacity(byteBuffer);
    if (!addr || cap <= 0) return nullptr;
    std::string id = NativeEngine::getInstance().createBufferSourceFromDirect(addr,
                                                                              static_cast<size_t>(cap),
                                                                              sampleRate, channels,
                                                                              bytesPerSample,
                                                                              env, byteBuffer);
    if (id.empty()) return nullptr;
    return env->NewStringUTF(id.c_str());
}

extern "C" JNIEXPORT jboolean JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeHasBuffer(JNIEnv *env, jobject thiz,
                                                                jstring jid) {
    if (jid == nullptr) return JNI_FALSE;
    const char *id = env->GetStringUTFChars(jid, nullptr);
    if (!id) return JNI_FALSE;
    bool ok = NativeEngine::getInstance().hasBuffer(id);
    env->ReleaseStringUTFChars(jid, id);
    return ok ? JNI_TRUE : JNI_FALSE;
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeReleaseBuffer(JNIEnv *env, jobject thiz,
                                                                    jstring jid) {
    const char *id = env->GetStringUTFChars(jid, nullptr);
    if (id) {
        NativeEngine::getInstance().freeBuffer(id);
        env->ReleaseStringUTFChars(jid, id);
    }
}

extern "C" JNIEXPORT jlong JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeGetAudioTimeNanos(JNIEnv *env, jobject thiz) {
    (void) env;
    (void) thiz;
    return static_cast<jlong>(g_audioTimeNanos.load(std::memory_order_relaxed));
}

extern "C" JNIEXPORT jstring JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeCreateAnalyser(JNIEnv *env, jobject thiz,
                                                                     jint fftSize,
                                                                     jdouble smoothingTimeConstant,
                                                                     jdouble minDecibels,
                                                                     jdouble maxDecibels) {
    (void) thiz;
    std::string id = NativeEngine::getInstance().createAnalyser((int) fftSize,
                                                                (double) smoothingTimeConstant,
                                                                (double) minDecibels,
                                                                (double) maxDecibels);
    if (id.empty()) return nullptr;
    return env->NewStringUTF(id.c_str());
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeReleaseAnalyser(JNIEnv *env, jobject thiz,
                                                                      jstring jid) {
    (void) thiz;
    if (jid == nullptr) return;
    const char *id = env->GetStringUTFChars(jid, nullptr);
    if (id) {
        NativeEngine::getInstance().freeAnalyser(id);
        env->ReleaseStringUTFChars(jid, id);
    }
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeSetAnalyserDecibels(JNIEnv *env, jobject thiz,
                                                                          jstring jid,
                                                                          jfloat jminDb,
                                                                          jfloat jmaxDb) {
    (void) thiz;
    if (jid == nullptr) return;
    const char *id = env->GetStringUTFChars(jid, nullptr);
    if (!id) return;
    NativeEngine::getInstance().setAnalyserDecibels(std::string(id), (double) jminDb,
                                                    (double) jmaxDb);
    env->ReleaseStringUTFChars(jid, id);
}

extern "C" JNIEXPORT jfloatArray JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeGetAnalyserFloatTimeDomainData(JNIEnv *env,
                                                                                     jobject thiz,
                                                                                     jstring jid,
                                                                                     jint count) {
    (void) thiz;
    if (jid == nullptr || count <= 0) return nullptr;
    const char *id = env->GetStringUTFChars(jid, nullptr);
    if (!id) return nullptr;
    std::vector<float> data = NativeEngine::getInstance().getAnalyserTimeDomainData(id,
                                                                                    (int) count);
    env->ReleaseStringUTFChars(jid, id);
    if (data.empty()) return nullptr;
    jfloatArray out = env->NewFloatArray((jsize) data.size());
    if (!out) return nullptr;
    env->SetFloatArrayRegion(out, 0, (jsize) data.size(), data.data());
    return out;
}

extern "C" JNIEXPORT jfloatArray JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeGetAnalyserFloatFrequencyData(JNIEnv *env,
                                                                                    jobject thiz,
                                                                                    jstring jid) {
    (void) thiz;
    if (jid == nullptr) return nullptr;
    const char *id = env->GetStringUTFChars(jid, nullptr);
    if (!id) return nullptr;
    std::vector<float> data = NativeEngine::getInstance().getAnalyserFrequencyData(id);
    env->ReleaseStringUTFChars(jid, id);
    if (data.empty()) return nullptr;
    jfloatArray out = env->NewFloatArray((jsize) data.size());
    if (!out) return nullptr;
    env->SetFloatArrayRegion(out, 0, (jsize) data.size(), data.data());
    return out;
}

extern "C" JNIEXPORT jboolean JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeGetAnalyserFloatTimeDomainDataDirect(
        JNIEnv *env, jobject thiz, jstring jid, jobject jdest) {
    (void) thiz;
    if (jid == nullptr || jdest == nullptr) return JNI_FALSE;
    void *addr = env->GetDirectBufferAddress(jdest);
    jlong cap = env->GetDirectBufferCapacity(jdest);
    if (!addr || cap <= 0) return JNI_FALSE;
    float *dest = reinterpret_cast<float *>(addr);
    const char *id = env->GetStringUTFChars(jid, nullptr);
    if (!id) return JNI_FALSE;
    int n = NativeEngine::getInstance().getAnalyserTimeDomainDataInto(id, dest, (int) cap);
    env->ReleaseStringUTFChars(jid, id);
    return n > 0 ? JNI_TRUE : JNI_FALSE;
}

extern "C" JNIEXPORT jboolean JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeGetAnalyserFloatFrequencyDataDirect(
        JNIEnv *env, jobject thiz, jstring jid, jobject jdest) {
    (void) thiz;
    if (jid == nullptr || jdest == nullptr) return JNI_FALSE;
    void *addr = env->GetDirectBufferAddress(jdest);
    jlong cap = env->GetDirectBufferCapacity(jdest);
    if (!addr || cap <= 0) return JNI_FALSE;
    float *dest = reinterpret_cast<float *>(addr);
    const char *id = env->GetStringUTFChars(jid, nullptr);
    if (!id) return JNI_FALSE;
    int n = NativeEngine::getInstance().getAnalyserFrequencyDataInto(id, dest, (int) cap);
    env->ReleaseStringUTFChars(jid, id);
    return n > 0 ? JNI_TRUE : JNI_FALSE;
}

extern "C" JNIEXPORT jboolean JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeGetAnalyserByteTimeDomainDataDirect(
        JNIEnv *env, jobject thiz, jstring jid, jobject jdest) {
    (void) thiz;
    if (jid == nullptr || jdest == nullptr) return JNI_FALSE;
    void *addr = env->GetDirectBufferAddress(jdest);
    jlong cap = env->GetDirectBufferCapacity(jdest);
    if (!addr || cap <= 0) return JNI_FALSE;
    uint8_t *dest = reinterpret_cast<uint8_t *>(addr);
    const char *id = env->GetStringUTFChars(jid, nullptr);
    if (!id) return JNI_FALSE;
    int n = NativeEngine::getInstance().getAnalyserByteTimeDomainDataInto(id, dest, (int) cap);
    env->ReleaseStringUTFChars(jid, id);
    return n > 0 ? JNI_TRUE : JNI_FALSE;
}

extern "C" JNIEXPORT jboolean JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeGetAnalyserByteFrequencyDataDirect(
        JNIEnv *env, jobject thiz, jstring jid, jobject jdest, jfloat jminDb, jfloat jmaxDb) {
    (void) thiz;
    if (jid == nullptr || jdest == nullptr) return JNI_FALSE;
    void *addr = env->GetDirectBufferAddress(jdest);
    jlong cap = env->GetDirectBufferCapacity(jdest);
    if (!addr || cap <= 0) return JNI_FALSE;
    uint8_t *dest = reinterpret_cast<uint8_t *>(addr);
    const char *id = env->GetStringUTFChars(jid, nullptr);
    if (!id) return JNI_FALSE;
    int n = NativeEngine::getInstance()
            .getAnalyserByteFrequencyDataInto(id, dest, (int) cap,
                                              (float) jminDb, (float) jmaxDb);
    env->ReleaseStringUTFChars(jid, id);
    return n > 0 ? JNI_TRUE : JNI_FALSE;
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeSetAnalyserFftSize(
        JNIEnv *env, jobject thiz, jstring jid, jint jfft) {
    (void) thiz;
    if (jid == nullptr) return;
    const char *id = env->GetStringUTFChars(jid, nullptr);
    if (!id) return;
    NativeEngine::getInstance().setAnalyserFftSize(std::string(id), (int) jfft);
    env->ReleaseStringUTFChars(jid, id);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeSetAnalyserSmoothing(
        JNIEnv *env, jobject thiz, jstring jid, jdouble jvalue) {
    (void) thiz;
    if (jid == nullptr) return;
    const char *id = env->GetStringUTFChars(jid, nullptr);
    if (!id) return;
    NativeEngine::getInstance().setAnalyserSmoothingTimeConstant(std::string(id), (double) jvalue);
    env->ReleaseStringUTFChars(jid, id);
}

extern "C" JNIEXPORT jlong JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeGetMonotonicTimeNanos(JNIEnv *env,
                                                                            jobject thiz) {
    (void) env;
    (void) thiz;
    int64_t audioNs = g_audioTimeNanos.load(std::memory_order_relaxed);
    if (audioNs != 0) return static_cast<jlong>(audioNs);
    int64_t nowNs = std::chrono::duration_cast<std::chrono::nanoseconds>(
            std::chrono::steady_clock::now().time_since_epoch()).count();
    return static_cast<jlong>(nowNs);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeRegisterContextStart(JNIEnv *env,
                                                                           jobject thiz,
                                                                           jstring jcontextId,
                                                                           jlong startNanos) {
    if (jcontextId == nullptr) return;
    const char *c = env->GetStringUTFChars(jcontextId, nullptr);
    if (!c) return;
    NativeEngine::getInstance().registerContextStart(std::string(c),
                                                     static_cast<int64_t>(startNanos));
    env->ReleaseStringUTFChars(jcontextId, c);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeUnregisterContextStart(JNIEnv *env,
                                                                             jobject thiz,
                                                                             jstring jcontextId) {
    if (jcontextId == nullptr) return;
    const char *c = env->GetStringUTFChars(jcontextId, nullptr);
    if (!c) return;
    NativeEngine::getInstance().unregisterContextStart(std::string(c));
    env->ReleaseStringUTFChars(jcontextId, c);
}

extern "C" JNIEXPORT jlong JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeGetContextCurrentTimeNanos(JNIEnv *env,
                                                                                 jobject thiz,
                                                                                 jstring jcontextId) {
    if (jcontextId == nullptr) return 0;
    const char *c = env->GetStringUTFChars(jcontextId, nullptr);
    if (!c) return 0;
    int64_t res = NativeEngine::getInstance().getContextCurrentTimeNanos(std::string(c));
    env->ReleaseStringUTFChars(jcontextId, c);
    return static_cast<jlong>(res);
}

extern "C" JNIEXPORT jstring JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeCreateExternalPcmSource(JNIEnv *env,
                                                                              jobject thiz,
                                                                              jint sampleRate,
                                                                              jint channels) {
    std::string id = NativeEngine::getInstance().createExternalPcmSource(
            static_cast<int>(sampleRate), static_cast<int>(channels));
    return env->NewStringUTF(id.c_str());
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeConfigureExternalPcmSource(JNIEnv *env,
                                                                                 jobject thiz,
                                                                                 jstring jid,
                                                                                 jint sampleRate,
                                                                                 jint channels) {
    if (!jid) return;
    const char *idChars = env->GetStringUTFChars(jid, nullptr);
    if (!idChars) return;
    NativeEngine::getInstance().configureExternalPcmSource(std::string(idChars),
                                                           static_cast<int>(sampleRate),
                                                           static_cast<int>(channels));
    env->ReleaseStringUTFChars(jid, idChars);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativePushPcmFramesFloatArray(JNIEnv *env,
                                                                              jobject thiz,
                                                                              jstring jid,
                                                                              jfloatArray jdata,
                                                                              jint frames) {
    if (!jid || !jdata || frames <= 0) return;
    const char *idChars = env->GetStringUTFChars(jid, nullptr);
    if (!idChars) return;
    std::string id(idChars);
    env->ReleaseStringUTFChars(jid, idChars);

    jsize len = env->GetArrayLength(jdata);
    if (len <= 0) return;
    jfloat *raw = env->GetFloatArrayElements(jdata, nullptr);
    if (!raw) return;
    NativeEngine::getInstance().pushPcmFrames(id, raw, static_cast<size_t>(len));
    env->ReleaseFloatArrayElements(jdata, raw, JNI_ABORT);
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativePushPcmFramesDirect(JNIEnv *env,
                                                                          jobject thiz,
                                                                          jstring jid,
                                                                          jobject jbuffer,
                                                                          jint sampleCount) {
    if (!jid || !jbuffer || sampleCount <= 0) return;
    const char *idChars = env->GetStringUTFChars(jid, nullptr);
    if (!idChars) return;
    std::string id(idChars);
    env->ReleaseStringUTFChars(jid, idChars);

    void *addr = env->GetDirectBufferAddress(jbuffer);
    if (!addr) return;
    NativeEngine::getInstance().pushPcmFrames(id, reinterpret_cast<const float *>(addr),
                                              static_cast<size_t>(sampleCount));
}

extern "C" JNIEXPORT void JNICALL
Java_org_nativescript_audiocontext_AudioContext_nativeEndExternalPcmSource(JNIEnv *env,
                                                                           jobject thiz,
                                                                           jstring jid) {
    if (!jid) return;
    const char *idChars = env->GetStringUTFChars(jid, nullptr);
    if (!idChars) return;
    NativeEngine::getInstance().endExternalPcmSource(std::string(idChars));
    env->ReleaseStringUTFChars(jid, idChars);
}
