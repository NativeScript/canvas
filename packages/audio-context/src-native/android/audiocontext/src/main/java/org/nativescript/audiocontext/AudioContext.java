package org.nativescript.audiocontext;

import android.media.AudioFormat;
import android.media.AudioManager;
import android.media.AudioTrack;
import android.media.MediaCodec;
import android.media.MediaDataSource;
import android.media.MediaExtractor;
import android.media.MediaFormat;
import android.os.Build;
import android.os.Handler;
import android.os.Looper;
import android.os.Process;
import android.util.Base64;
import android.util.Log;

import androidx.annotation.NonNull;
import androidx.annotation.Nullable;
import androidx.annotation.RequiresApi;

import java.io.ByteArrayOutputStream;
import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.FloatBuffer;
import java.nio.charset.StandardCharsets;
import java.util.Arrays;
import java.util.Collections;
import java.util.HashSet;
import java.util.Iterator;
import java.util.Set;
import java.util.UUID;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.CopyOnWriteArrayList;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.atomic.AtomicInteger;

public class AudioContext {
	private static AudioContext sInstance;
	private final ConcurrentHashMap<String, AudioTrack> tracks = new ConcurrentHashMap<>();
	private final ConcurrentHashMap<String, Thread> trackThreads = new ConcurrentHashMap<>();
	private final ConcurrentHashMap<String, java.util.concurrent.CopyOnWriteArrayList<EndedListener>> trackEndedListeners = new ConcurrentHashMap<>();

	public interface EndedListener {
		void onEnded(String trackId);
	}


	public void addEndedListener(String trackId, @NonNull EndedListener listener) {
		if (trackId == null) return;
		if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N) {
			trackEndedListeners
				.computeIfAbsent(trackId, k -> new CopyOnWriteArrayList<>())
				.addIfAbsent(listener);
		} else {
			synchronized (trackEndedListeners) {
				CopyOnWriteArrayList<EndedListener> list = trackEndedListeners.get(trackId);
				if (list == null) {
					CopyOnWriteArrayList<EndedListener> newList = new CopyOnWriteArrayList<>();
					CopyOnWriteArrayList<EndedListener> existing =
						trackEndedListeners.putIfAbsent(trackId, newList);
					list = (existing != null) ? existing : newList;
				}

				if (!list.contains(listener)) {
					list.add(listener);
				}
			}
		}
	}

	public void removeEndedListener(String trackId, @NonNull EndedListener listener) {
		if (trackId == null) return;
		java.util.concurrent.CopyOnWriteArrayList<EndedListener> list = trackEndedListeners.get(trackId);
		if (list != null) list.remove(listener);
	}


	void fireEnded(String trackId) {
		java.util.concurrent.CopyOnWriteArrayList<EndedListener> list = trackEndedListeners.remove(trackId);
		if (list == null) return;
		for (EndedListener l : list) {
			try {
				l.onEnded(trackId);
			} catch (Throwable t) {
				Log.w("AudioContext", "ended listener threw", t);
			}
		}
	}

	@SuppressWarnings("unused")
	void nativeOnTrackEnded(String trackId) {
		fireEnded(trackId);
	}

	private static boolean nativeAvailable;
	private final ConcurrentHashMap<String, Integer> sampleRates = new ConcurrentHashMap<>();

	private final ConcurrentHashMap<String, Integer> contextSampleRates = new ConcurrentHashMap<>();
	private final ConcurrentHashMap<String, Set<String>> contextBuffers = new ConcurrentHashMap<>();
	private final ConcurrentHashMap<String, Set<String>> contextTracksMap = new ConcurrentHashMap<>();

	private final ConcurrentHashMap<String, Long> contextStartNanos = new ConcurrentHashMap<>();
	private final ConcurrentHashMap<String, java.nio.ByteBuffer> directBuffers = new ConcurrentHashMap<>();
	private final ConcurrentHashMap<String, AtomicInteger> directRefCounts = new ConcurrentHashMap<>();
	private final ConcurrentHashMap<String, Integer> bufferChannels = new ConcurrentHashMap<>();

	private final ConcurrentHashMap<String, Double> gains = new ConcurrentHashMap<>();
	private final ConcurrentHashMap<String, java.util.List<ParamEvent>> gainEvents = new ConcurrentHashMap<>();
	private final ConcurrentHashMap<String, java.util.concurrent.ConcurrentHashMap<Integer, java.util.List<ParamEvent>>> pannerEvents = new ConcurrentHashMap<>();
	private final ConcurrentHashMap<String, java.util.concurrent.ConcurrentHashMap<Integer, java.util.List<ParamEvent>>> listenerEvents = new ConcurrentHashMap<>();
	private final ConcurrentHashMap<String, String> voicePlaybackRateMap = new ConcurrentHashMap<>();
	private final ConcurrentHashMap<String, ConcurrentHashMap<Integer, String>> voicePlaybackRateMapByOutput = new ConcurrentHashMap<>();

	private static class ParamEvent {
		public final int type; // 0 = set, 1 = linearRamp
		public final int rate; // 0 = k-rate, 1 = a-rate
		public final double time;
		public final double value;

		public ParamEvent(int type, int rate, double time, double value) {
			this.type = type;
			this.rate = rate;
			this.time = time;
			this.value = value;
		}
	}

	private final ConcurrentHashMap<String, Set<String>> contextGains = new ConcurrentHashMap<>();
	private final ConcurrentHashMap<String, String> gainContexts = new ConcurrentHashMap<>();
	private final ConcurrentHashMap<String, ConcurrentHashMap<Integer, String>> voiceGainMapByOutput = new ConcurrentHashMap<>();
	private final ConcurrentHashMap<String, Set<String>> contextBiquads = new ConcurrentHashMap<>();
	private final ConcurrentHashMap<String, Set<String>> contextIirs = new ConcurrentHashMap<>();
	private final ConcurrentHashMap<String, Set<String>> contextPeriodicWaves = new ConcurrentHashMap<>();
	private final ConcurrentHashMap<String, float[]> waveShaperCurves = new ConcurrentHashMap<>();
	private final ConcurrentHashMap<String, java.nio.FloatBuffer> waveShaperCurveBuffers = new ConcurrentHashMap<>();
	private final ConcurrentHashMap<String, String> waveShaperOversamples = new ConcurrentHashMap<>();
	private final ConcurrentHashMap<String, Set<String>> contextPanners = new ConcurrentHashMap<>();
	private final ConcurrentHashMap<String, String> pannerContexts = new ConcurrentHashMap<>();
	private final ConcurrentHashMap<String, String> contextDestination = new ConcurrentHashMap<>();
	private final Set<String> destinationIds = createConcurrentSet();
	private final ConcurrentHashMap<String, Set<String>> nodeSources = new ConcurrentHashMap<>();
	private final ExecutorService decodeExecutor = Executors.newFixedThreadPool(Math.max(2, Runtime.getRuntime().availableProcessors()));


	private final ConcurrentHashMap<Looper, Handler> _handlerCache = new ConcurrentHashMap<>();

	private Handler handlerForLooper(Looper looper) {
		Handler h = _handlerCache.get(looper);
		if (h != null) return h;
		Handler created = new Handler(looper);
		Handler prev = _handlerCache.putIfAbsent(looper, created);
		return prev != null ? prev : created;
	}


	private Looper captureCallerLooper() {
		return Looper.myLooper();
	}

	private void dispatch(Looper target, Runnable r) {
		if (target == null) {
			r.run();
			return;
		}

		if (Looper.myLooper() == target) {
			r.run();
			return;
		}
		handlerForLooper(target).post(r);
	}

	public static synchronized AudioContext getInstance() {
		if (sInstance == null) sInstance = new AudioContext();
		return sInstance;
	}

	static {
		try {
			System.loadLibrary("native-audio");
			nativeInit();
			nativeAvailable = true;
		} catch (UnsatisfiedLinkError ex) {
			nativeAvailable = false;
		}
	}

	private static native void nativeInit();

	private native void nativeCancelGainEvents(String gainId, long timeNs);

	private native void nativeCancelPannerEvents(String pannerId, int paramType, long timeNs);

	private native void nativeCancelAndHoldGainEvents(String gainId, int rate, long timeNs, double value);

	private native void nativeCancelAndHoldPannerEvents(String pannerId, int paramType, int rate, long timeNs, double value);

	String createContext() {
		return createContext(0, 0.0);
	}

	String createContext(int sampleRate, double latencyHintSec) {
		String id = UUID.randomUUID().toString();
		int sr = sampleRate > 0 ? sampleRate : 48000;
		if (sampleRate <= 0) {
			try {
				sr = AudioTrack.getNativeOutputSampleRate(AudioManager.STREAM_MUSIC);
			} catch (Throwable ignored) {
			}
		}
		if (nativeAvailable) {
			try {
				nativeConfigureStream(sampleRate, latencyHintSec);
			} catch (Throwable ignored) {
			}
		}
		contextSampleRates.put(id, sr);
		contextBuffers.put(id, createConcurrentSet());
		contextTracksMap.put(id, createConcurrentSet());
		long start = 0L;
		if (nativeAvailable) {
			start = nativeGetMonotonicTimeNanos();
		}
		if (start == 0L) start = System.nanoTime();
		contextStartNanos.put(id, start);
		try {
			nativeRegisterContextStart(id, start);
		} catch (Throwable ignored) {
		}
		return id;
	}

	public void renderOfflineAsync(final String[] trackIds, final int frames, final int sampleRate, final int channels, final DecodeCallback cb) {
		final Looper callerLooper = captureCallerLooper();
		decodeExecutor.submit(() -> {
			AudioBuffer id;
			if (nativeAvailable) {
				try {
					String nid = nativeRenderOffline(trackIds, frames, sampleRate, channels);
					id = makeAudioBufferFromId(nid);
				} catch (Throwable t) {
					id = null;
				}
			} else {
				try {
					if (trackIds == null || trackIds.length == 0) {
						id = null;
					} else {
						int outSampleRate = sampleRate > 0 ? sampleRate : 48000;
						int outChannels = channels > 0 ? channels : 1;
						float[] out = new float[frames * outChannels];
						for (String tid : trackIds) {
							if (tid == null) continue;
							java.nio.ByteBuffer src = directBuffers.get(tid);
							if (src == null) src = directBuffers.get(tid + "::buffer");
							if (src == null) continue;
							int srcChannels = 1;
							if (bufferChannels.containsKey(tid)) srcChannels = bufferChannels.get(tid);
							else if (bufferChannels.containsKey(tid + "::buffer"))
								srcChannels = bufferChannels.get(tid + "::buffer");
							int srcSampleRate = 48000;
							if (sampleRates.containsKey(tid)) srcSampleRate = sampleRates.get(tid);
							else if (sampleRates.containsKey(tid + "::buffer"))
								srcSampleRate = sampleRates.get(tid + "::buffer");
							int byteLen = src.capacity();
							int bytesPerSample = (byteLen % 4 == 0) ? 4 : 2;
							int srcFrames = (byteLen / bytesPerSample) / Math.max(1, srcChannels);
							java.nio.ByteBuffer dup = src.duplicate().order(ByteOrder.LITTLE_ENDIAN);
							double increment = ((double) srcSampleRate) / ((double) outSampleRate);
							double pos = 0.0;
							for (int f = 0; f < frames; f++) {
								int i0 = (int) Math.floor(pos);
								double frac = pos - i0;
								for (int c = 0; c < outChannels; c++) {
									int chIdx;
									if (srcChannels == 1) chIdx = 0;
									else if (srcChannels >= outChannels) chIdx = c;
									else chIdx = c % srcChannels;
									int idx0 = Math.max(0, Math.min(srcFrames - 1, i0)) * srcChannels + chIdx;
									int idx1 = Math.max(0, Math.min(srcFrames - 1, i0 + 1)) * srcChannels + chIdx;
									float s0 = 0.0f, s1 = 0.0f;
									if (bytesPerSample == 4) {
										s0 = dup.getFloat(idx0 * 4);
										s1 = dup.getFloat(idx1 * 4);
									} else {
										s0 = (float) dup.getShort(idx0 * 2) / 32768.0f;
										s1 = (float) dup.getShort(idx1 * 2) / 32768.0f;
									}
									float sample = s0 * (float) (1.0 - frac) + s1 * (float) frac;
									out[f * outChannels + c] += sample;
								}
								pos += increment;
								if (pos >= srcFrames) pos = srcFrames;
							}
						}
						java.nio.ByteBuffer outBb = java.nio.ByteBuffer.allocateDirect(frames * outChannels * 4).order(java.nio.ByteOrder.LITTLE_ENDIAN);
						for (float value : out) {
							float v = value;
							if (v > 1.0f) v = 1.0f;
							if (v < -1.0f) v = -1.0f;
							outBb.putFloat(v);
						}
						outBb.position(0);
						String nid = UUID.randomUUID().toString();
						directBuffers.put(nid, outBb);
						getOrCreate(nid).incrementAndGet();
						sampleRates.put(nid, outSampleRate);
						bufferChannels.put(nid, outChannels);
						id = makeAudioBufferFromId(nid);
					}
				} catch (Throwable t) {
					id = null;
				}
			}
			final AudioBuffer result = id;
			dispatch(callerLooper, () -> cb.onResult(result));
		});
	}

	public AudioContextInstance createContextInstance() {
		return new AudioContextInstance(createContext());
	}

	public AudioContextInstance createContextInstance(int sampleRate, double latencyHintSec) {
		return new AudioContextInstance(createContext(sampleRate, latencyHintSec));
	}

	void releaseContext(String contextId) {
		if (contextId == null) return;
		Set<String> tracksSet = contextTracksMap.remove(contextId);
		if (tracksSet != null) {
			for (String tid : tracksSet) {
				stopTrack(tid);
				tracks.remove(tid);
			}
		}
		Set<String> bufs = contextBuffers.remove(contextId);
		if (bufs != null) {
			for (String bid : bufs) {
				releaseBuffer(bid);
			}
		}
		Set<String> gset = contextGains.remove(contextId);
		if (gset != null) {
			for (String gid : gset) {
				releaseGain(gid);
			}
		}
		Set<String> bset = contextBiquads.remove(contextId);
		if (bset != null) {
			for (String bid : bset) {
				releaseBiquad(bid);
			}
		}
		Set<String> pset = contextPanners.remove(contextId);
		if (pset != null) {
			for (String pid : pset) {
				releasePanner(pid);
			}
		}
		Set<String> iset = contextIirs.remove(contextId);
		if (iset != null) {
			for (String iid : iset) {
				releaseIIR(iid);
			}
		}
		Set<String> pwset = contextPeriodicWaves.remove(contextId);
		if (pwset != null) {
			for (String pid : pwset) {
				releasePeriodicWave(pid);
			}
		}
		String did = contextDestination.remove(contextId);
		if (did != null) {
			destinationIds.remove(did);
			clearNodeSources(did);
			releaseGain(did);
		}
		contextSampleRates.remove(contextId);
		try {
			nativeUnregisterContextStart(contextId);
		} catch (Throwable ignored) {
		}
		contextStartNanos.remove(contextId);
	}

	public long getContextStartNanos(String contextId) {
		Long v = contextStartNanos.get(contextId);
		return v == null ? 0L : v;
	}


	public long getAudioTimeNanos() {
		if (!nativeAvailable) return 0L;
		try {
			return nativeGetAudioTimeNanos();
		} catch (Throwable t) {
			return 0L;
		}
	}

	void renderOfflineForContext(final String contextId, final int frames, final int sampleRate, final int channels, final DecodeCallback cb) {
		Set<String> set = contextTracksMap.get(contextId);
		String[] arr = null;
		if (set != null && !set.isEmpty()) {
			arr = set.toArray(new String[0]);
		}
		renderOfflineAsync(arr, frames, sampleRate, channels, cb);
	}

	public AudioIIRNode createIIR(AudioContextInstance context, double[] feedforward, double[] feedback) {
		String id = createIIR(context != null ? context.getId() : null, feedforward, feedback);
		return new AudioIIRNode(id, feedforward, feedback, context != null ? context.getId() : null);
	}

	String createIIR(String contextId, double[] feedforward, double[] feedback) {
		String id = null;
		if (nativeAvailable) {
			try {
				id = nativeCreateIIR(feedforward, feedback);
			} catch (Throwable ignored) {
			}
		}
		if (id == null) id = UUID.randomUUID().toString();
		if (contextId != null) {
			addToContextSet(contextIirs, contextId, id);
		}
		return id;
	}

	public PeriodicWave createPeriodicWave(AudioContextInstance context, double[] real, double[] imag, boolean disableNormalization) {
		return new PeriodicWave(createPeriodicWave(context.getId(), real, imag, disableNormalization));
	}

	public PeriodicWave createPeriodicWave(AudioContextInstance context, java.nio.FloatBuffer real, java.nio.FloatBuffer imag, boolean disableNormalization) {
		return new PeriodicWave(createPeriodicWave(context.getId(), real, imag, disableNormalization));
	}

	String createPeriodicWave(String contextId, double[] real, double[] imag, boolean disableNormalization) {
		String id = null;
		if (nativeAvailable) {
			try {
				id = nativeCreatePeriodicWave(real, imag, disableNormalization);
			} catch (Throwable ignored) {
			}
		}
		if (id == null) id = UUID.randomUUID().toString();
		if (contextId != null) {
			addToContextSet(contextPeriodicWaves, contextId, id);
		}
		return id;
	}

	String createPeriodicWave(String contextId, java.nio.FloatBuffer real, java.nio.FloatBuffer imag, boolean disableNormalization) {
		String id = null;
		if (nativeAvailable) {
			try {
				java.nio.FloatBuffer realView = real.duplicate();
				realView.position(real.position());
				realView.limit(real.limit());
				java.nio.FloatBuffer realSlice = realView.slice();

				java.nio.FloatBuffer imagView = imag.duplicate();
				imagView.position(imag.position());
				imagView.limit(imag.limit());
				java.nio.FloatBuffer imagSlice = imagView.slice();

				id = nativeCreatePeriodicWaveDirect(realSlice, imagSlice, disableNormalization);
			} catch (Throwable ignored) {
			}
		}
		if (id == null) id = UUID.randomUUID().toString();
		if (contextId != null) {
			addToContextSet(contextPeriodicWaves, contextId, id);
		}
		return id;
	}


	boolean getIIRFrequencyResponse(String id, double[] frequencyHz, double[] magResponse, double[] phaseResponse, double sampleRate) {
		if (id == null) return false;
		if (!nativeAvailable) return false;
		try {
			nativeGetIIRFrequencyResponse(id, frequencyHz, magResponse, phaseResponse, sampleRate);
			return true;
		} catch (Throwable ignored) {
			return false;
		}
	}

	boolean getIIRFrequencyResponse(String id, FloatBuffer frequencyHz, FloatBuffer magResponse, FloatBuffer phaseResponse, double sampleRate) {
		if (id == null) return false;
		if (!nativeAvailable) return false;
		try {
			if (frequencyHz.isDirect() && magResponse.isDirect() && phaseResponse.isDirect()) {
				java.nio.FloatBuffer fqView = frequencyHz.duplicate();
				fqView.position(frequencyHz.position());
				fqView.limit(frequencyHz.limit());
				java.nio.FloatBuffer fqSlice = fqView.slice();

				java.nio.FloatBuffer magView = magResponse.duplicate();
				magView.position(magResponse.position());
				magView.limit(magResponse.limit());
				java.nio.FloatBuffer magSlice = magView.slice();

				java.nio.FloatBuffer phView = phaseResponse.duplicate();
				phView.position(phaseResponse.position());
				phView.limit(phaseResponse.limit());
				java.nio.FloatBuffer phSlice = phView.slice();

				nativeGetIIRFrequencyResponseDirect(id, fqSlice, magSlice, phSlice, sampleRate);
				return true;
			}
			return false;
		} catch (Throwable ignored) {
			return false;
		}
	}


	public void releasePeriodicWave(String id) {
		if (id == null) return;
		try {
			if (nativeAvailable) nativeReleasePeriodicWave(id);
		} catch (Throwable ignored) {
		}
	}

	public void attachPeriodicWaveToVoice(String voiceId, String waveId) {
		if (voiceId == null) return;
		try {
			if (nativeAvailable) nativeAttachPeriodicWave(voiceId, waveId);
		} catch (Throwable ignored) {
		}
	}

	public void registerContextTrack(String contextId, String trackId) {
		if (contextId == null || trackId == null || trackId.isEmpty()) return;
		addToContextSet(contextTracksMap, contextId, trackId);
	}

	public void unregisterContextTrack(String contextId, String trackId) {
		if (contextId == null || trackId == null) return;
		Set<String> set = contextTracksMap.get(contextId);
		if (set == null) return;
		set.remove(trackId);
		if (set.isEmpty()) contextTracksMap.remove(contextId);
	}

	public String[] getContextTrackIds(String contextId) {
		Set<String> set = contextTracksMap.get(contextId);
		if (set == null || set.isEmpty()) return new String[0];
		return set.toArray(new String[0]);
	}

	private native void nativeReleaseBuffer(String id);

	public static void onNativeBufferHeld(String id) {
		AudioContext inst = getInstance();
		if (inst != null) inst.handleNativeBufferHeld(id);
	}

	public static void onNativeBufferReleased(String id) {
		AudioContext inst = getInstance();
		if (inst != null) inst.handleNativeBufferReleased(id);
	}

	private void handleNativeBufferHeld(String id) {
		if (id == null) return;
		getOrCreate(id).incrementAndGet();
	}

	private void handleNativeBufferReleased(String id) {
		if (id == null) return;
		AtomicInteger ai = directRefCounts.get(id);
		if (ai != null) {
			int v = ai.decrementAndGet();
			if (v <= 0) {
				directRefCounts.remove(id);
				directBuffers.remove(id);
				sampleRates.remove(id);
				bufferChannels.remove(id);
			}
		} else {
			directBuffers.remove(id);
			sampleRates.remove(id);
			bufferChannels.remove(id);
		}
	}

	private native void nativeConfigureStream(int sampleRate, double latencyHintSec);

	private native void nativeResume();

	private native void nativeSuspend();

	private native void nativeSetOutputDeviceId(int deviceId);

	public boolean setSinkId(String deviceId) {
		int id = 0; // oboe::kUnspecified
		if (deviceId != null && !deviceId.isEmpty() && !"default".equals(deviceId)) {
			try {
				id = Integer.parseInt(deviceId);
			} catch (NumberFormatException nfe) {
				return false;
			}
		}
		if (nativeAvailable) {
			try {
				nativeSetOutputDeviceId(id);
				return true;
			} catch (Throwable t) {
				return false;
			}
		}
		return false;
	}

	private native long nativeGetAudioTimeNanos();

	private native long nativeGetMonotonicTimeNanos();

	private native void nativeRegisterContextStart(String contextId, long startNanos);

	private native void nativeUnregisterContextStart(String contextId);

	private native long nativeGetContextCurrentTimeNanos(String contextId);

	private native String nativeDecodeAudioData(byte[] bytes);

	private native String nativeDecodeAudioDataDirect(java.nio.ByteBuffer buffer);

	private native String nativeCreateOscillator(String type, double frequency);

	private native void nativeStartOscillator(String id, String type, double frequency);

	private native void nativeStopTrack(String id);

	private native String nativeCreateBufferSource(String bufferId, byte[] pcmBytes, int sampleRate, int channels);

	private native String nativeCreateBufferSourceDirect(java.nio.ByteBuffer buffer, int sampleRate, int channels, int bytesPerSample);

	private native boolean nativeHasBuffer(String id);

	private native void nativeStartBufferSource(String trackId, boolean loop);

	private native String nativeRenderOffline(String[] trackIds, int frames, int sampleRate, int channels);

	private native String nativeCreateGain();

	private native void nativeSetGain(String gainId, double value);

	private native void nativeSetAudioThreadLoggingEnabled(boolean enabled);

	private native void nativeScheduleGainSet(String gainId, int rate, long timeNs, double value);

	private native void nativeScheduleGainRamp(String gainId, int rate, long timeNs, double value);

	private native void nativeSchedulePannerSet(String pannerId, int paramType, int rate, long timeNs, double value);

	private native void nativeSchedulePannerRamp(String pannerId, int paramType, int rate, long timeNs, double value);

	private native double[] nativeGetPannerParamValues(String pannerId, int paramType, long startNs, double sampleRate, int frameCount);

	private native double[] nativeGetGainParamValues(String gainId, long startNs, double sampleRate, int frameCount);

	private native void nativeAttachGain(String voiceId, String gainId);

	private native void nativeAttachGain(String voiceId, String gainId, int output, int input);

	private native void nativeAttachPlaybackRate(String voiceId, String playbackRateId);

	private native void nativeAttachPlaybackRate(String voiceId, String playbackRateId, int output, int input);

	private native void nativeDetachPlaybackRate(String playbackRateId);

	private native void nativeReleasePlaybackRate(String playbackRateId);

	private native void nativeDetachGain(String gainId);

	private native void nativeReleaseGain(String gainId);

	private native String nativeCreateBiquad(String type, double frequency, double Q, double gain);

	private native String nativeCreatePanner(String contextId, double positionX, double positionY, double positionZ,
																					 double orientationX, double orientationY, double orientationZ,
																					 double pan,
																					 int distanceModel, int panningModel,
																					 double refDistance, double maxDistance, double rolloffFactor,
																					 double coneInnerAngle, double coneOuterAngle, double coneOuterGain);

	private native void nativeSetPannerParams(String pannerId,
																						double positionX, double positionY, double positionZ,
																						double orientationX, double orientationY, double orientationZ,
																						double pan,
																						int distanceModel, int panningModel,
																						double refDistance, double maxDistance, double rolloffFactor,
																						double coneInnerAngle, double coneOuterAngle, double coneOuterGain);

	private native void nativeSetListenerParams(String contextId, double positionX, double positionY, double positionZ,
																							double forwardX, double forwardY, double forwardZ,
																							double upX, double upY, double upZ);

	private native void nativeScheduleListenerSet(String contextId, int paramType, int rate, long timeNs, double value);

	private native void nativeScheduleListenerRamp(String contextId, int paramType, int rate, long timeNs, double value);

	private native double[] nativeGetListenerParamValues(String contextId, int paramType, long startNs, double sampleRate, int frameCount);

	private native void nativeCancelListenerEvents(String contextId, int paramType, long timeNs);

	private native void nativeCancelAndHoldListenerEvents(String contextId, int paramType, int rate, long timeNs, double value);

	private native double nativeGetListenerParamValue(String contextId, int paramType);

	private native void nativeAttachPanner(String voiceId, String pannerId);

	private native void nativeAttachPanner(String voiceId, String pannerId, int output, int input);

	private native void nativeDetachPanner(String pannerId);

	private native void nativeReleasePanner(String pannerId);

	private native void nativeSetBiquadParams(String biquadId, double frequency, double Q, double gain, String type);

	private native String nativeCreateIIR(double[] feedforward, double[] feedback);

	private native void nativeReleaseIIR(String id);

	private native void nativeGetIIRFrequencyResponse(String iirId, double[] frequencyHz, double[] magResponse, double[] phaseResponse, double sampleRate);

	private native void nativeGetIIRFrequencyResponseDirect(String iirId, java.nio.FloatBuffer frequencyHz, java.nio.FloatBuffer magResponse, java.nio.FloatBuffer phaseResponse, double sampleRate);

	private native String nativeCreatePeriodicWave(double[] real, double[] imag, boolean disableNormalization);

	private native String nativeCreatePeriodicWaveDirect(java.nio.FloatBuffer real, java.nio.FloatBuffer imag, boolean disableNormalization);

	private native void nativeSetWaveShaperCurveArray(String id, float[] curve, String oversample);

	private native void nativeSetWaveShaperCurveDirect(String id, java.nio.FloatBuffer data, String oversample);

	private native String nativeCreateAnalyser(int fftSize, double smoothingTimeConstant, double minDecibels, double maxDecibels);

	private native void nativeReleaseAnalyser(String id);

	private native float[] nativeGetAnalyserFloatTimeDomainData(String id, int count);

	private native float[] nativeGetAnalyserFloatFrequencyData(String id);

	private native boolean nativeGetAnalyserFloatTimeDomainDataDirect(String id, java.nio.FloatBuffer dest);

	private native boolean nativeGetAnalyserFloatFrequencyDataDirect(String id, java.nio.FloatBuffer dest);

	private native boolean nativeGetAnalyserByteTimeDomainDataDirect(String id, java.nio.ByteBuffer dest);

	private native boolean nativeGetAnalyserByteFrequencyDataDirect(String id, java.nio.ByteBuffer dest, float minDecibels, float maxDecibels);

	private native void nativeSetAnalyserDecibels(String id, float minDecibels, float maxDecibels);

	private native void nativeSetAnalyserFftSize(String id, int fftSize);

	private native void nativeSetAnalyserSmoothing(String id, double value);

	private native void nativeSetWaveShaperOversample(String id, String oversample);

	private native void nativeReleasePeriodicWave(String id);

	private native void nativeAttachPeriodicWave(String voiceId, String waveId);

	private native void nativeAttachBiquad(String voiceId, String biquadId);

	private native void nativeAttachBiquad(String voiceId, String biquadId, int output, int input);

	private native void nativeDetachBiquad(String biquadId);

	private native void nativeReleaseBiquad(String biquadId);

	private final ConcurrentHashMap<String, String> voiceGainMap = new ConcurrentHashMap<>();

	private AtomicInteger getOrCreate(String id) {
		if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N) {
			return directRefCounts.computeIfAbsent(id, k -> new AtomicInteger(0));
		} else {
			AtomicInteger counter = directRefCounts.get(id);
			if (counter == null) {
				AtomicInteger newCounter = new AtomicInteger(0);
				AtomicInteger existing = directRefCounts.putIfAbsent(id, newCounter);
				counter = (existing != null) ? existing : newCounter;
			}
			return counter;
		}
	}

	private Set<String> createConcurrentSet() {
		return Collections.newSetFromMap(new ConcurrentHashMap<>());
	}

	private Set<String> getOrCreateContextSet(ConcurrentHashMap<String, Set<String>> map, String contextId) {
		if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N) {
			return map.computeIfAbsent(contextId, (key) -> createConcurrentSet());
		}

		Set<String> current = map.get(contextId);
		if (current == null) {
			Set<String> created = createConcurrentSet();
			Set<String> existing = map.putIfAbsent(contextId, created);
			current = existing != null ? existing : created;
		}
		return current;
	}

	private void addToContextSet(ConcurrentHashMap<String, Set<String>> map, String contextId, String value) {
		if (contextId == null || value == null || value.isEmpty()) return;
		getOrCreateContextSet(map, contextId).add(value);
	}

	private Set<String> getOrCreateNodeSources(String nodeId) {
		return getOrCreateContextSet(nodeSources, nodeId);
	}

	void registerNodeSource(String nodeId, String sourceId) {
		if (nodeId == null || nodeId.isEmpty() || sourceId == null || sourceId.isEmpty()) return;
		getOrCreateNodeSources(nodeId).add(sourceId);
	}

	void registerNodeSources(String nodeId, Set<String> sourceIds) {
		if (nodeId == null || nodeId.isEmpty() || sourceIds == null || sourceIds.isEmpty()) return;
		getOrCreateNodeSources(nodeId).addAll(sourceIds);
	}

	Set<String> getNodeSourcesSnapshot(String nodeId) {
		Set<String> sources = nodeSources.get(nodeId);
		if (sources == null || sources.isEmpty()) {
			return Collections.emptySet();
		}
		return new HashSet<>(sources);
	}

	void unregisterNodeSource(String nodeId, String sourceId) {
		if (nodeId == null || sourceId == null) return;
		Set<String> sources = nodeSources.get(nodeId);
		if (sources == null) return;
		sources.remove(sourceId);
		if (sources.isEmpty()) {
			nodeSources.remove(nodeId);
		}
	}

	void clearNodeSources(String nodeId) {
		if (nodeId == null) return;
		nodeSources.remove(nodeId);
	}

	boolean isDestinationNode(String nodeId) {
		return nodeId != null && destinationIds.contains(nodeId);
	}

	public AudioBuffer decodeAudioData(String base64) {
		try {
			byte[] data = Base64.decode(base64, Base64.DEFAULT);
			String id = decodeAudioDataBytes(data);
			if (id == null) return null;
			return makeAudioBufferFromId(id);
		} catch (Exception e) {
			return null;
		}
	}

	public AudioBuffer decodeAudioDataFromByteArray(byte[] data) {
		if (data == null) return null;
		AudioBuffer buffer = tryDecodeWithMediaExtractorFromByteArray(data);
		if (buffer != null) return buffer;
		String id = decodeAudioDataBytes(data);
		if (id == null) return null;
		return makeAudioBufferFromId(id);
	}

	public AudioBuffer decodeAudioDataFromFile(String path) {
		if (path == null) return null;
		AudioBuffer buffer = tryDecodeWithMediaExtractorFromFile(path);
		if (buffer != null) return buffer;
		try {
			File f = new File(path);
			if (!f.exists() || !f.isFile()) return null;
			try (FileInputStream fis = new FileInputStream(f)) {
				int len = (int) f.length();
				byte[] data = new byte[len];
				int read = 0;
				while (read < len) {
					int r = fis.read(data, read, len - read);
					if (r < 0) break;
					read += r;
				}
				String id = decodeAudioDataBytes(data);
				if (id == null) return null;
				return makeAudioBufferFromId(id);
			} catch (IOException ignored) {
			}
		} catch (Exception ignored) {
		}
		return null;
	}

	public AudioBuffer decodeAudioDataFromBuffer(java.nio.ByteBuffer buffer) {
		if (buffer == null) return null;
		if (nativeAvailable) {
			try {
				java.nio.ByteBuffer view = buffer.duplicate();
				view.position(buffer.position());
				view.limit(buffer.limit());
				java.nio.ByteBuffer slice = view.slice();
				String id = nativeDecodeAudioDataDirect(slice);
				if (id != null) {
					directBuffers.put(id, buffer);
					getOrCreate(id).incrementAndGet();
					if (!sampleRates.containsKey(id)) sampleRates.put(id, 48000);
					if (!bufferChannels.containsKey(id)) bufferChannels.put(id, 1);
					return makeAudioBufferFromId(id);
				}
			} catch (Throwable ignored) {
			}
		}
		AudioBuffer audioBuffer = tryDecodeWithMediaExtractorFromBuffer(buffer);
		if (audioBuffer != null) return audioBuffer;
		try {
			int cap = buffer.capacity();
			byte[] data = new byte[cap];
			buffer.position(0);
			buffer.get(data);
			String id = decodeAudioDataBytes(data);
			if (id == null) return null;
			return makeAudioBufferFromId(id);
		} catch (Exception e) {
			return null;
		}
	}


	public double getContextCurrentTime(String contextId) {
		if (contextId == null) return 0.0;
		if (nativeAvailable) {
			long ns = nativeGetContextCurrentTimeNanos(contextId);
			if (ns > 0L) return (double) ns / 1e9;
		}
		Long start = contextStartNanos.get(contextId);
		if (start == null) return 0.0;
		long now = System.nanoTime();
		return (double) (now - start) / 1e9;
	}

	private AudioBuffer tryDecodeWithMediaExtractorFromFile(String path) {
		if (path == null) return null;
		try {
			File f = new File(path);
			if (!f.exists() || !f.isFile()) return null;
			return decodeUsingExtractor((extractor) -> extractor.setDataSource(path));
		} catch (Throwable t) {
			return null;
		}
	}

	private AudioBuffer tryDecodeWithMediaExtractorFromByteArray(byte[] data) {
		if (data == null) return null;
		if (Build.VERSION.SDK_INT >= 23) {
			ByteArrayMediaDataSource ds = new ByteArrayMediaDataSource(data);
			try {
				return decodeUsingExtractor((extractor) -> extractor.setDataSource(ds));
			} catch (Throwable t) {
				return null;
			} finally {
				try {
					ds.close();
				} catch (Throwable ignored) {
				}
			}
		} else {
			File tmp = null;
			try {
				tmp = File.createTempFile("audioctx", ".tmp");
				java.io.FileOutputStream fos = new java.io.FileOutputStream(tmp);
				fos.write(data);
				fos.close();
				AudioBuffer buffer = tryDecodeWithMediaExtractorFromFile(tmp.getAbsolutePath());
				try {
					tmp.delete();
				} catch (Throwable ignored) {
				}
				return buffer;
			} catch (Throwable t) {
				if (tmp != null) try {
					tmp.delete();
				} catch (Throwable ignored) {
				}
				return null;
			}
		}
	}

	private AudioBuffer tryDecodeWithMediaExtractorFromBuffer(final java.nio.ByteBuffer buffer) {
		if (buffer == null) return null;
		if (Build.VERSION.SDK_INT >= 23) {
			ByteBufferMediaDataSource ds = new ByteBufferMediaDataSource(buffer);
			try {
				return decodeUsingExtractor((extractor) -> extractor.setDataSource(ds));
			} catch (Throwable t) {
				return null;
			} finally {
				try {
					ds.close();
				} catch (Throwable ignored) {
				}
			}
		} else {
			try {
				int cap = buffer.capacity();
				byte[] data = new byte[cap];
				buffer.position(0);
				buffer.get(data);
				return tryDecodeWithMediaExtractorFromByteArray(data);
			} catch (Throwable t) {
				return null;
			}
		}
	}

	private interface ExtractorSetter {
		void set(MediaExtractor e) throws Exception;
	}

	private AudioBuffer decodeUsingExtractor(ExtractorSetter setter) {
		MediaExtractor extractor = null;
		MediaCodec codec = null;
		ByteArrayOutputStream baos = null;
		try {
			extractor = new MediaExtractor();
			setter.set(extractor);
			int trackIndex = -1;
			MediaFormat trackFormat = null;
			String mime = null;
			for (int i = 0; i < extractor.getTrackCount(); i++) {
				MediaFormat mf = extractor.getTrackFormat(i);
				String m = mf.getString(MediaFormat.KEY_MIME);
				if (m != null && m.startsWith("audio/")) {
					trackIndex = i;
					trackFormat = mf;
					mime = m;
					break;
				}
			}
			if (trackIndex < 0) return null;
			extractor.selectTrack(trackIndex);
			codec = MediaCodec.createDecoderByType(mime);
			codec.configure(trackFormat, null, null, 0);
			codec.start();
			baos = new ByteArrayOutputStream();
			final MediaCodec.BufferInfo info = new MediaCodec.BufferInfo();
			boolean sawInputEOS = false;
			boolean sawOutputEOS = false;
			int inputBufSize = 65536;
			if (trackFormat.containsKey(MediaFormat.KEY_MAX_INPUT_SIZE))
				inputBufSize = trackFormat.getInteger(MediaFormat.KEY_MAX_INPUT_SIZE);
			ByteBuffer inputBuf = ByteBuffer.allocate(Math.max(4096, inputBufSize));
			while (!sawOutputEOS) {
				if (!sawInputEOS) {
					int inIndex = codec.dequeueInputBuffer(10000);
					if (inIndex >= 0) {
						inputBuf.clear();
						int sampleSize = extractor.readSampleData(inputBuf, 0);
						if (sampleSize < 0) {
							codec.queueInputBuffer(inIndex, 0, 0, 0, MediaCodec.BUFFER_FLAG_END_OF_STREAM);
							sawInputEOS = true;
						} else {
							inputBuf.position(0);
							inputBuf.limit(sampleSize);
							ByteBuffer dst = codec.getInputBuffer(inIndex);
							dst.clear();
							dst.put(inputBuf);
							long pts = extractor.getSampleTime();
							codec.queueInputBuffer(inIndex, 0, sampleSize, pts, 0);
							extractor.advance();
						}
					}
				}
				int outIndex = codec.dequeueOutputBuffer(info, 10000);
				if (outIndex >= 0) {
					ByteBuffer outBuf = codec.getOutputBuffer(outIndex);
					if (outBuf != null && info.size > 0) {
						byte[] chunk = new byte[info.size];
						outBuf.position(info.offset);
						outBuf.limit(info.offset + info.size);
						outBuf.get(chunk);
						baos.write(chunk);
					}
					codec.releaseOutputBuffer(outIndex, false);
					if ((info.flags & MediaCodec.BUFFER_FLAG_END_OF_STREAM) != 0) sawOutputEOS = true;
				} else if (outIndex == MediaCodec.INFO_OUTPUT_FORMAT_CHANGED) {
					// format changed
				} else {
					// try again
				}
			}
			byte[] pcmBytes = baos.toByteArray();
			if (pcmBytes.length == 0) return null;
			short[] tmp = new short[pcmBytes.length / 2];
			ByteBuffer.wrap(pcmBytes).order(ByteOrder.LITTLE_ENDIAN).asShortBuffer().get(tmp);
			float[] pcm = new float[tmp.length];
			for (int i = 0; i < tmp.length; i++) pcm[i] = (float) tmp[i] / 32768.0f;
			String id = UUID.randomUUID().toString();
			java.nio.ByteBuffer bb = java.nio.ByteBuffer.allocateDirect(pcm.length * 4).order(java.nio.ByteOrder.LITTLE_ENDIAN);
			for (float f : pcm) bb.putFloat(f);
			bb.position(0);
			directBuffers.put(id, bb);
			getOrCreate(id).incrementAndGet();
			int sampleRate = 48000;
			if (trackFormat != null && trackFormat.containsKey(MediaFormat.KEY_SAMPLE_RATE))
				sampleRate = trackFormat.getInteger(MediaFormat.KEY_SAMPLE_RATE);
			sampleRates.put(id, sampleRate);
			int numChannels = 1;
			if (trackFormat != null && trackFormat.containsKey(MediaFormat.KEY_CHANNEL_COUNT))
				numChannels = trackFormat.getInteger(MediaFormat.KEY_CHANNEL_COUNT);
			bufferChannels.put(id, numChannels);
			return makeAudioBufferFromId(id);
		} catch (Throwable t) {
			Log.w("AudioContext", "MediaExtractor decode failed: " + t);
			return null;
		} finally {
			try {
				if (codec != null) {
					codec.stop();
					codec.release();
				}
			} catch (Throwable ignored) {
			}
			try {
				if (extractor != null) extractor.release();
			} catch (Throwable ignored) {
			}
			try {
				if (baos != null) baos.close();
			} catch (Throwable ignored) {
			}
		}
	}

	@RequiresApi(Build.VERSION_CODES.M)
	private static class ByteArrayMediaDataSource extends MediaDataSource {
		private final byte[] data;

		ByteArrayMediaDataSource(byte[] d) {
			this.data = d;
		}

		@Override
		public int readAt(long position, byte[] buffer, int offset, int size) throws IOException {
			if (position >= data.length) return -1;
			int avail = Math.min(size, data.length - (int) position);
			System.arraycopy(data, (int) position, buffer, offset, avail);
			return avail;
		}

		@Override
		public long getSize() throws IOException {
			return data.length;
		}

		@Override
		public void close() throws IOException {
		}
	}

	@RequiresApi(Build.VERSION_CODES.M)
	private static class ByteBufferMediaDataSource extends MediaDataSource {
		private final java.nio.ByteBuffer buf;

		ByteBufferMediaDataSource(java.nio.ByteBuffer b) {
			this.buf = b;
		}

		@Override
		public int readAt(long position, byte[] buffer, int offset, int size) throws IOException {
			if (position >= buf.capacity()) return -1;
			int avail = Math.min(size, buf.capacity() - (int) position);
			java.nio.ByteBuffer dup = buf.duplicate();
			dup.position((int) position);
			dup.get(buffer, offset, avail);
			return avail;
		}

		@Override
		public long getSize() throws IOException {
			return buf.capacity();
		}

		@Override
		public void close() throws IOException {
		}
	}

	public void decodeAudioDataAsync(final String base64, final AudioContextInstance context, final DecodeCallback cb) {
		decodeAudioDataAsync(base64, context.getId(), cb);
	}

	void decodeAudioDataAsync(final String base64, final String contextId, final DecodeCallback cb) {
		final Looper callerLooper = captureCallerLooper();
		decodeExecutor.submit(() -> {
			final AudioBuffer buffer = decodeAudioData(base64);
			if (buffer != null && contextId != null) {
				assignBufferToContext(buffer, contextId);
			}
			dispatch(callerLooper, () -> cb.onResult(buffer));
		});
	}

	public void decodeAudioDataFromByteArrayAsync(final byte[] data, final AudioContextInstance context, final DecodeCallback cb) {
		decodeAudioDataFromByteArrayAsync(data, context.getId(), cb);
	}

	void decodeAudioDataFromByteArrayAsync(final byte[] data, final String contextId, final DecodeCallback cb) {
		final Looper callerLooper = captureCallerLooper();
		decodeExecutor.submit(() -> {
			final AudioBuffer buffer = decodeAudioDataFromByteArray(data);
			if (buffer != null && contextId != null) {
				assignBufferToContext(buffer, contextId);
			}
			dispatch(callerLooper, () -> cb.onResult(buffer));
		});
	}


	public void decodeAudioDataFromFileAsync(final String path, final AudioContextInstance context, final DecodeCallback cb) {
		decodeAudioDataFromFileAsync(path, context.getId(), cb);
	}

	void decodeAudioDataFromFileAsync(final String path, final String contextId, final DecodeCallback cb) {
		final Looper callerLooper = captureCallerLooper();
		decodeExecutor.submit(() -> {
			final AudioBuffer buffer = decodeAudioDataFromFile(path);
			if (buffer != null && contextId != null) {
				assignBufferToContext(buffer, contextId);
			}
			dispatch(callerLooper, () -> cb.onResult(buffer));
		});
	}

	public void decodeAudioDataFromBufferAsync(final java.nio.ByteBuffer buffer, final AudioContextInstance context, final DecodeCallback cb) {
		decodeAudioDataFromBufferAsync(buffer, context.getId(), cb);
	}

	void decodeAudioDataFromBufferAsync(final java.nio.ByteBuffer buffer, final String contextId, final DecodeCallback cb) {
		final Looper callerLooper = captureCallerLooper();
		decodeExecutor.submit(() -> {
			final AudioBuffer audioBuffer = decodeAudioDataFromBuffer(buffer);
			if (audioBuffer != null && contextId != null) {
				assignBufferToContext(audioBuffer, contextId);
			}
			dispatch(callerLooper, () -> cb.onResult(audioBuffer));
		});
	}

	public AudioBuffer getAudioBuffer(final String id) {
		if (id == null) return null;
		if (!directBuffers.containsKey(id)) return null;
		return makeAudioBufferFromId(id);
	}

	public void registerBuffer(String id, java.nio.ByteBuffer bb, int sampleRate, int channels) {
		if (id == null || bb == null) return;
		directBuffers.put(id, bb);
		sampleRates.put(id, sampleRate);
		bufferChannels.put(id, channels);
	}

	private AudioBuffer makeAudioBufferFromId(String id) {
		if (id == null) return null;
		java.nio.ByteBuffer bb = directBuffers.get(id);
		Integer sr = sampleRates.get(id);
		Integer ch = bufferChannels.get(id);
		return new AudioBuffer(id, bb, sr, ch);
	}

	public int getSampleRate(String id) {
		if (id == null) return 0;
		Integer sr = contextSampleRates.get(id);
		if (sr != null) return sr;
		sr = sampleRates.get(id);
		return sr != null ? sr : 48000;
	}

	public int getLength(String id) {
		if (id == null) return 0;
		Integer ch = bufferChannels.get(id);
		int channels = ch != null ? ch : 1;
		java.nio.ByteBuffer bb = directBuffers.get(id);
		if (bb != null) {
			int byteLen = bb.capacity();
			int bytesPerSample = (byteLen % 4 == 0) ? 4 : 2;
			return (byteLen / bytesPerSample) / channels;
		}
		return 0;
	}

	public double getDuration(String id) {
		int sr = getSampleRate(id);
		if (sr <= 0) return 0.0;
		int len = getLength(id);
		return ((double) len) / ((double) sr);
	}

	public int getNumberOfChannels(String id) {
		if (id == null) return 0;
		Integer ch = bufferChannels.get(id);
		return ch != null ? ch : 1;
	}

	public java.nio.ByteBuffer getChannelData(String id, int channel) {
		if (id == null) return null;
		int channels = getNumberOfChannels(id);
		if (channel < 0 || channel >= channels) return null;
		java.nio.ByteBuffer bb = directBuffers.get(id);
		if (bb != null) {
			return bb.duplicate().order(ByteOrder.LITTLE_ENDIAN);
		}
		return null;
	}

	public void copyFromChannel(String id, java.util.List<Float> dest, int channel, int startInChannel) {
		java.nio.ByteBuffer data = getChannelData(id, channel);
		java.nio.FloatBuffer fb = null;
		if (data != null) {
			fb = data.asFloatBuffer();
		}
		if (fb == null || dest == null) return;
		int ch = getNumberOfChannels(id);
		int frames = fb.capacity() / ch;
		int start = Math.max(0, startInChannel);
		if (start >= frames) return;
		if (dest.isEmpty()) {
			for (int i = start; i < frames; i++) dest.add(fb.get(i * ch + channel));
			return;
		}
		int n = Math.min(dest.size(), frames - start);
		for (int i = 0; i < n; i++) dest.set(i, fb.get((start + i) * ch + channel));
	}

	public void copyFromChannel(String id, float[] dest, int channel, int startInChannel) {
		java.nio.ByteBuffer data = getChannelData(id, channel);
		java.nio.FloatBuffer fb = null;
		if (data != null) {
			fb = data.asFloatBuffer();
		}

		if (fb == null || dest == null) return;
		int ch = getNumberOfChannels(id);
		int frames = fb.capacity() / ch;
		int start = Math.max(0, startInChannel);
		if (start >= frames) return;
		int n = Math.min(dest.length, frames - start);
		for (int i = 0; i < n; i++) dest[i] = fb.get((start + i) * ch + channel);
	}

	public void copyFromChannel(String id, java.nio.FloatBuffer dest, int channel, int startInChannel) {
		if (id == null || dest == null) return;
		AudioBuffer ab = makeAudioBufferFromId(id);
		if (ab == null) return;
		ab.copyFromChannel(dest, channel, startInChannel);
	}

	public void copyFromChannel(String id, java.nio.ByteBuffer dest, int channel, int startInChannel) {
		if (id == null || dest == null) return;
		AudioBuffer ab = makeAudioBufferFromId(id);
		if (ab == null) return;
		ab.copyFromChannel(dest, channel, startInChannel);
	}

	public void copyToChannel(String id, float[] source, int channel, int startInChannel) {
		if (id == null || source == null) return;
		int channels = getNumberOfChannels(id);
		if (channel < 0 || channel >= channels) return;
		int start = Math.max(0, startInChannel);
		java.nio.ByteBuffer bb = directBuffers.get(id);
		if (bb != null && !bb.isReadOnly()) {
			java.nio.ByteBuffer dup = bb.duplicate().order(ByteOrder.LITTLE_ENDIAN);
			int byteLen = bb.capacity();
			int bytesPerSample = (byteLen % 4 == 0) ? 4 : 2;
			int frames = (byteLen / bytesPerSample) / channels;
			int n = Math.min(source.length, Math.max(0, frames - start));
			if (bytesPerSample == 4) {
				for (int i = 0; i < n; i++) {
					int byteIndex = ((start + i) * channels + channel) * 4;
					dup.putFloat(byteIndex, source[i]);
				}
			} else {
				for (int i = 0; i < n; i++) {
					int byteIndex = ((start + i) * channels + channel) * 2;
					int v = Math.round(source[i] * 32767.0f);
					if (v > Short.MAX_VALUE) v = Short.MAX_VALUE;
					if (v < Short.MIN_VALUE) v = Short.MIN_VALUE;
					dup.putShort(byteIndex, (short) v);
				}
			}
		}
	}

	public void copyToChannel(String id, java.nio.FloatBuffer source, int channel, int startInChannel) {
		if (id == null || source == null) return;
		AudioBuffer ab = makeAudioBufferFromId(id);
		if (ab == null) return;
		ab.copyToChannel(source, channel, startInChannel);
	}

	public void copyToChannel(String id, java.nio.ByteBuffer source, int channel, int startInChannel) {
		if (id == null || source == null) return;
		AudioBuffer ab = makeAudioBufferFromId(id);
		if (ab == null) return;
		ab.copyToChannel(source, channel, startInChannel);
	}

	private String decodeAudioDataBytes(byte[] data) {
		if (data == null || data.length < 44) return null;
		if (nativeAvailable) {
			try {
				java.nio.ByteBuffer bb = java.nio.ByteBuffer.allocateDirect(data.length).order(java.nio.ByteOrder.LITTLE_ENDIAN);
				bb.put(data);
				bb.position(0);
				String id = nativeDecodeAudioDataDirect(bb);
				if (id != null) {
					directBuffers.put(id, bb);
					getOrCreate(id).incrementAndGet();
					return id;
				}
			} catch (Throwable ignored) {
			}
		}

		String riff;
		try {
			riff = new String(data, 0, 4, StandardCharsets.US_ASCII);
		} catch (Exception e) {
			return null;
		}
		if (!"RIFF".equals(riff)) return null;

		int audioFormat = ((data[20] & 0xff) | ((data[21] & 0xff) << 8));
		int numChannels = ((data[22] & 0xff) | ((data[23] & 0xff) << 8));
		int sampleRate = ((data[24] & 0xff) | ((data[25] & 0xff) << 8) | ((data[26] & 0xff) << 16) | ((data[27] & 0xff) << 24));
		int bitsPerSample = ((data[34] & 0xff) | ((data[35] & 0xff) << 8));

		if (audioFormat != 1) return null;
		if (bitsPerSample != 16) return null;

		int dataOffset = 12;
		while (dataOffset + 8 < data.length) {
			String chunkId;
			try {
				chunkId = new String(data, dataOffset, 4, StandardCharsets.US_ASCII);
			} catch (Exception e) {
				break;
			}
			int chunkSize = ((data[dataOffset + 4] & 0xff) | ((data[dataOffset + 5] & 0xff) << 8) | ((data[dataOffset + 6] & 0xff) << 16) | ((data[dataOffset + 7] & 0xff) << 24));
			if ("data".equals(chunkId)) {
				dataOffset += 8;
				int dataLen = chunkSize;
				short[] tmp = new short[dataLen / 2];
				ByteBuffer.wrap(data, dataOffset, dataLen).order(ByteOrder.LITTLE_ENDIAN).asShortBuffer().get(tmp);
				float[] pcm = new float[tmp.length];
				for (int i = 0; i < tmp.length; i++) pcm[i] = (float) tmp[i] / 32768.0f;
				String id = UUID.randomUUID().toString();
				java.nio.ByteBuffer bb = java.nio.ByteBuffer.allocateDirect(pcm.length * 4).order(java.nio.ByteOrder.LITTLE_ENDIAN);
				for (float f : pcm) bb.putFloat(f);
				bb.position(0);
				directBuffers.put(id, bb);
				getOrCreate(id).incrementAndGet();
				sampleRates.put(id, sampleRate);
				bufferChannels.put(id, numChannels);
				return id;
			}
			dataOffset += 8 + chunkSize;
		}

		return null;
	}

	void releaseBuffer(AudioBuffer buffer) {
		releaseBuffer(buffer.id);
	}

	void releaseBuffer(String id) {
		if (id == null) return;

		AtomicInteger ai = directRefCounts.get(id);
		if (ai != null) {
			if (ai.decrementAndGet() > 0) return;
			if (nativeAvailable) {
				nativeReleaseBuffer(id);
				return;
			}
		} else if (nativeAvailable) {
			nativeReleaseBuffer(id);
			return;
		}

		directRefCounts.remove(id);
		directBuffers.remove(id);
		sampleRates.remove(id);
		bufferChannels.remove(id);
	}

	public void resume() {
		if (nativeAvailable) {
			nativeResume();
		}
	}

	public void suspend() {
		if (nativeAvailable) {
			nativeSuspend();
		}
	}

	public interface AsyncCallback {
		void onComplete(boolean ok);
	}

	public void resumeAsync(@Nullable AsyncCallback cb) {
		final Looper callerLooper = captureCallerLooper();
		decodeExecutor.submit(() -> {
			boolean ok = true;
			try {
				if (nativeAvailable) nativeResume();
			} catch (Throwable t) {
				ok = false;
			}
			final boolean result = ok;
			if (cb != null) dispatch(callerLooper, () -> cb.onComplete(result));
		});
	}

	public void suspendAsync(@Nullable AsyncCallback cb) {
		final Looper callerLooper = captureCallerLooper();
		decodeExecutor.submit(() -> {
			boolean ok = true;
			try {
				if (nativeAvailable) nativeSuspend();
			} catch (Throwable t) {
				ok = false;
			}
			final boolean result = ok;
			if (cb != null) dispatch(callerLooper, () -> cb.onComplete(result));
		});
	}

	public void closeAsync(@Nullable AsyncCallback cb) {
		final Looper callerLooper = captureCallerLooper();
		decodeExecutor.submit(() -> {
			boolean ok = true;
			try {
				if (nativeAvailable) nativeSuspend();
			} catch (Throwable t) {
				ok = false;
			}
			final boolean result = ok;
			if (cb != null) dispatch(callerLooper, () -> cb.onComplete(result));
		});
	}

	public AudioOscillatorNode createOscillator(AudioContextInstance context) {
		return new AudioOscillatorNode(context);
	}

	public AudioOscillatorNode createOscillator(AudioContextInstance context, String type, double frequency) {
		return new AudioOscillatorNode(context, type, frequency);
	}

	String createOscillator(String contextId, String type, double frequency) {
		if (nativeAvailable) {
			String id = nativeCreateOscillator(type, frequency);
			if (id != null) {
				addToContextSet(contextTracksMap, contextId, id);
				return id;
			}
		}

		int sampleRate = 48000;
		if (contextId != null && contextSampleRates.containsKey(contextId))
			sampleRate = contextSampleRates.get(contextId);
		int bufferSize = AudioTrack.getMinBufferSize(sampleRate, AudioFormat.CHANNEL_OUT_MONO, AudioFormat.ENCODING_PCM_16BIT);

		AudioTrack track = new AudioTrack(AudioManager.STREAM_MUSIC, sampleRate, AudioFormat.CHANNEL_OUT_MONO, AudioFormat.ENCODING_PCM_16BIT, Math.max(bufferSize, 8192), AudioTrack.MODE_STREAM);
		String id = UUID.randomUUID().toString();
		tracks.put(id, track);
		addToContextSet(contextTracksMap, contextId, id);
		return id;
	}

	public void startOscillator(final String id, final String type, final double frequency) {
		if (nativeAvailable) {
			nativeStartOscillator(id, type, frequency);
			return;
		}

		final AudioTrack t = tracks.get(id);
		if (t == null) return;
		Thread th = new Thread(() -> {
			try {
				try {
					Process.setThreadPriority(Process.THREAD_PRIORITY_AUDIO);
				} catch (Throwable ignored) {
				}
			} catch (Throwable ignored) {
			}
			t.play();
			int sampleRate = t.getSampleRate();
			int bufSize = 1024;
			short[] buffer = new short[bufSize];
			double phase = 0.0;
			while (t.getPlayState() == AudioTrack.PLAYSTATE_PLAYING) {
				double g = 1.0;
				String gid = null;
				ConcurrentHashMap<Integer, String> om = voiceGainMapByOutput.get(id);
				if (om != null) gid = om.get(0);
				if (gid == null) gid = voiceGainMap.get(id);
				if (gid != null) {
					Double gf = gains.get(gid);
					if (gf != null) g = gf;
				}
				for (int i = 0; i < bufSize; i++) {
					double value = Math.sin(phase * 2.0 * Math.PI);
					int sample = (int) Math.round(value * Short.MAX_VALUE * g);
					if (sample > Short.MAX_VALUE) sample = Short.MAX_VALUE;
					if (sample < Short.MIN_VALUE) sample = Short.MIN_VALUE;
					buffer[i] = (short) sample;
					phase += frequency / sampleRate;
					if (phase >= 1.0) phase -= 1.0;
				}
				t.write(buffer, 0, bufSize);
			}
		});
		trackThreads.put(id, th);
		th.start();
	}

	public void stopTrack(String id) {
		if (nativeAvailable) {
			nativeStopTrack(id);
			fireEnded(id);
			return;
		}

		AudioTrack t = tracks.remove(id);
		Thread th = trackThreads.remove(id);

		if (th != null) {
			th.interrupt();
		}

		if (t != null) {
			try {
				t.pause();
				t.flush();
			} catch (Exception ignored) {
			}
			try {
				t.stop();
			} catch (Exception ignored) {
			}
			try {
				t.release();
			} catch (Exception ignored) {
			}
		}

		fireEnded(id);
	}

	void assignBufferToContext(AudioBuffer buffer, AudioContextInstance context) {
		assignBufferToContext(buffer, context.getId());
	}

	void assignBufferToContext(AudioBuffer buffer, String contextId) {
		if (buffer == null || contextId == null) return;
		addToContextSet(contextBuffers, contextId, buffer.id);
	}

	public GainNode createGain(AudioContextInstance context) {
		return createGain(context.getId());
	}

	GainNode createGain(String contextId) {
		String id = null;
		if (nativeAvailable) {
			id = nativeCreateGain();
		}

		if (id == null) {
			id = UUID.randomUUID().toString();
			gains.put(id, 1.0);
		}

		if (contextId != null) {
			addToContextSet(contextGains, contextId, id);
			gainContexts.put(id, contextId);
		}

		return new GainNode(id);
	}

	public GainNode getDestination(AudioContextInstance context) {
		return getDestination(context.getId());
	}

	GainNode getDestination(String contextId) {
		if (contextId == null) return null;
		String did = contextDestination.get(contextId);
		if (did != null) return new GainNode(did);
		GainNode g = createGain(contextId);
		if (g != null) {
			contextDestination.put(contextId, g.getId());
			destinationIds.add(g.getId());
		}
		return g;
	}

	public AudioBiquadNode createBiquad(AudioContextInstance context, String type, double frequency, double Q, double gain) {
		return new AudioBiquadNode(createBiquad(context.getId(), type, frequency, Q, gain));
	}

	public ConvolverNode createConvolver(AudioContextInstance context) {
		return new ConvolverNode(createConvolver(context.getId()));
	}

	String createConvolver(String contextId) {
		String id = null;
		if (nativeAvailable) {
			try {
				id = nativeCreateGain();
			} catch (Throwable ignored) {
			}
		}
		if (id == null) {
			id = UUID.randomUUID().toString();
			gains.put(id, 1.0);
		}
		if (contextId != null) {
			addToContextSet(contextGains, contextId, id);
			gainContexts.put(id, contextId);
		}
		return id;
	}

	public AudioBiquadNode createBiquad(AudioContextInstance context, String type, double frequency, double Q, double gain, double detune) {
		return new AudioBiquadNode(createBiquad(context.getId(), type, frequency, Q, gain));
	}

	String createBiquad(String contextId, String type, double frequency, double Q, double gain) {
		String id = null;
		if (nativeAvailable) {
			id = nativeCreateBiquad(type, frequency, Q, gain);
		}

		if (id == null) id = UUID.randomUUID().toString();

		if (contextId != null) {
			addToContextSet(contextBiquads, contextId, id);
		}

		return id;
	}

	public AudioPannerNode createPanner(AudioContextInstance context) {
		return createPanner(context.getId());
	}

	AudioPannerNode createPanner(String contextId) {
		String id = null;
		if (nativeAvailable) {
			id = nativeCreatePanner(contextId, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0, 0, 1.0, 10000.0, 1.0, 360.0, 360.0, 0.0);
		}

		if (id == null) id = UUID.randomUUID().toString();

		if (contextId != null) {
			addToContextSet(contextPanners, contextId, id);
			pannerContexts.put(id, contextId);
		}

		return new AudioPannerNode(id);
	}

	void scheduleGainSet(String id, int rate, double value, double time) {
		if (id == null) return;
		long absNs = -1L;
		String ctx = gainContexts.get(id);
		if (ctx != null) {
			Long startNs = contextStartNanos.get(ctx);
			if (startNs != null) absNs = startNs + (long) (time * 1000000000.0);
		}
		if (nativeAvailable && absNs >= 0) {
			nativeScheduleGainSet(id, rate, absNs, value);
		}
		java.util.List<ParamEvent> lst = gainEvents.get(id);
		if (lst == null) {
			lst = java.util.Collections.synchronizedList(new java.util.ArrayList<>());
			gainEvents.put(id, lst);
		}
		lst.add(new ParamEvent(0, rate, time, value));
	}

	void scheduleGainRamp(String id, int rate, double value, double time) {
		if (id == null) return;
		long absNs = -1L;
		String ctx = gainContexts.get(id);
		if (ctx != null) {
			Long startNs = contextStartNanos.get(ctx);
			if (startNs != null) absNs = startNs + (long) (time * 1000000000.0);
		}
		if (nativeAvailable && absNs >= 0) {
			nativeScheduleGainRamp(id, rate, absNs, value);
		}
		java.util.List<ParamEvent> lst = gainEvents.get(id);
		if (lst == null) {
			lst = java.util.Collections.synchronizedList(new java.util.ArrayList<>());
			gainEvents.put(id, lst);
		}
		lst.add(new ParamEvent(1, rate, time, value));
	}

	public void setListenerParams(String contextId, double positionX, double positionY, double positionZ,
																double forwardX, double forwardY, double forwardZ,
																double upX, double upY, double upZ) {
		if (nativeAvailable) {
			try {
				nativeSetListenerParams(contextId, positionX, positionY, positionZ, forwardX, forwardY, forwardZ, upX, upY, upZ);
			} catch (Throwable ignored) {
			}
		}
	}

	public void setListenerParams(double positionX, double positionY, double positionZ,
																double forwardX, double forwardY, double forwardZ,
																double upX, double upY, double upZ) {
		setListenerParams("", positionX, positionY, positionZ, forwardX, forwardY, forwardZ, upX, upY, upZ);
	}

	void scheduleListenerSet(String contextId, int paramType, int rate, double value, double time) {
		if (contextId == null) return;
		long absNs = -1L;
		String ctx = contextId;
		if (ctx != null) {
			Long startNs = contextStartNanos.get(ctx);
			if (startNs != null) absNs = startNs + (long) (time * 1000000000.0);
		}
		if (nativeAvailable && absNs >= 0) {
			nativeScheduleListenerSet(contextId, paramType, rate, absNs, value);
		}
		java.util.concurrent.ConcurrentHashMap<Integer, java.util.List<ParamEvent>> map = listenerEvents.get(contextId);
		if (map == null) {
			map = new java.util.concurrent.ConcurrentHashMap<>();
			listenerEvents.put(contextId, map);
		}
		java.util.List<ParamEvent> lst = map.get(paramType);
		if (lst == null) {
			lst = java.util.Collections.synchronizedList(new java.util.ArrayList<>());
			map.put(paramType, lst);
		}
		lst.add(new ParamEvent(0, rate, time, value));
	}

	void scheduleListenerRamp(String contextId, int paramType, int rate, double value, double time) {
		if (contextId == null) return;
		long absNs = -1L;
		String ctx = contextId;
		if (ctx != null) {
			Long startNs = contextStartNanos.get(ctx);
			if (startNs != null) absNs = startNs + (long) (time * 1000000000.0);
		}
		if (nativeAvailable && absNs >= 0) {
			nativeScheduleListenerRamp(contextId, paramType, rate, absNs, value);
		}
		java.util.concurrent.ConcurrentHashMap<Integer, java.util.List<ParamEvent>> map = listenerEvents.get(contextId);
		if (map == null) {
			map = new java.util.concurrent.ConcurrentHashMap<>();
			listenerEvents.put(contextId, map);
		}
		java.util.List<ParamEvent> lst = map.get(paramType);
		if (lst == null) {
			lst = java.util.Collections.synchronizedList(new java.util.ArrayList<>());
			map.put(paramType, lst);
		}
		lst.add(new ParamEvent(1, rate, time, value));
	}

	void cancelListenerScheduledValues(String contextId, int paramType, double time) {
		if (contextId == null) return;
		long absNs = -1L;
		String ctx = contextId;
		if (ctx != null) {
			Long startNs = contextStartNanos.get(ctx);
			if (startNs != null) absNs = startNs + (long) (time * 1000000000.0);
		}
		if (nativeAvailable && absNs >= 0) {
			nativeCancelListenerEvents(contextId, paramType, absNs);
		}
		java.util.concurrent.ConcurrentHashMap<Integer, java.util.List<ParamEvent>> map = listenerEvents.get(contextId);
		if (map == null) return;
		java.util.List<ParamEvent> lst = map.get(paramType);
		if (lst == null) return;
		synchronized (lst) {
			if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N) {
				lst.removeIf(ev -> ev.time >= time);
			} else {
				Iterator<ParamEvent> it = lst.iterator();
				while (it.hasNext()) if (it.next().time >= time) it.remove();
			}
		}
	}

	void cancelAndHoldListenerScheduledValues(String contextId, int paramType, int rate, double heldValue, double time) {
		if (contextId == null) return;
		long absNs = -1L;
		Long startNs = contextStartNanos.get(contextId);
		if (startNs != null) absNs = startNs + (long) (time * 1000000000.0);
		if (nativeAvailable && absNs >= 0) {
			nativeCancelAndHoldListenerEvents(contextId, paramType, rate, absNs, heldValue);
		}
		java.util.concurrent.ConcurrentHashMap<Integer, java.util.List<ParamEvent>> map = listenerEvents.get(contextId);
		if (map == null) {
			map = new java.util.concurrent.ConcurrentHashMap<>();
			listenerEvents.put(contextId, map);
		}
		java.util.List<ParamEvent> lst = map.get(paramType);
		if (lst == null) {
			lst = java.util.Collections.synchronizedList(new java.util.ArrayList<>());
			map.put(paramType, lst);
		}
		if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N) {
			lst.removeIf(ev -> ev.time >= time);
		} else {
			Iterator<ParamEvent> it = lst.iterator();
			while (it.hasNext()) if (it.next().time >= time) it.remove();
		}
		lst.add(new ParamEvent(0, rate, time, heldValue));
	}

	double[] getListenerParamValues(String contextId, int paramType, double startTime, double sampleRate, int frameCount) {
		if (contextId == null) return null;
		long absNs = -1L;
		String ctx = contextId;
		if (ctx != null) {
			Long startNs = contextStartNanos.get(ctx);
			if (startNs != null) absNs = startNs + (long) (startTime * 1000000000.0);
		}
		if (nativeAvailable && absNs >= 0) {
			try {
				return nativeGetListenerParamValues(contextId, paramType, absNs, sampleRate, frameCount);
			} catch (Throwable ignored) {
			}
		}
		return null;
	}

	double getListenerParamValue(String contextId, int paramType) {
		if (contextId == null) return 0.0;
		if (nativeAvailable) {
			try {
				return nativeGetListenerParamValue(contextId, paramType);
			} catch (Throwable ignored) {
			}
		}
		return 0.0;
	}

	void schedulePannerSet(String id, int paramType, int rate, double value, double time) {
		if (id == null) return;
		long absNs = -1L;
		String ctx = pannerContexts.get(id);
		if (ctx != null) {
			Long startNs = contextStartNanos.get(ctx);
			if (startNs != null) absNs = startNs + (long) (time * 1000000000.0);
		}
		if (nativeAvailable && absNs >= 0) {
			nativeSchedulePannerSet(id, paramType, rate, absNs, value);
		}
		java.util.concurrent.ConcurrentHashMap<Integer, java.util.List<ParamEvent>> map = pannerEvents.get(id);
		if (map == null) {
			map = new java.util.concurrent.ConcurrentHashMap<>();
			pannerEvents.put(id, map);
		}
		java.util.List<ParamEvent> lst = map.get(paramType);
		if (lst == null) {
			lst = java.util.Collections.synchronizedList(new java.util.ArrayList<>());
			map.put(paramType, lst);
		}
		lst.add(new ParamEvent(0, rate, time, value));
	}

	void schedulePannerRamp(String id, int paramType, int rate, double value, double time) {
		if (id == null) return;
		long absNs = -1L;
		String ctx = pannerContexts.get(id);
		if (ctx != null) {
			Long startNs = contextStartNanos.get(ctx);
			if (startNs != null) absNs = startNs + (long) (time * 1000000000.0);
		}
		if (nativeAvailable && absNs >= 0) {
			nativeSchedulePannerRamp(id, paramType, rate, absNs, value);
		}
		java.util.concurrent.ConcurrentHashMap<Integer, java.util.List<ParamEvent>> map = pannerEvents.get(id);
		if (map == null) {
			map = new java.util.concurrent.ConcurrentHashMap<>();
			pannerEvents.put(id, map);
		}
		java.util.List<ParamEvent> lst = map.get(paramType);
		if (lst == null) {
			lst = java.util.Collections.synchronizedList(new java.util.ArrayList<>());
			map.put(paramType, lst);
		}
		lst.add(new ParamEvent(1, rate, time, value));
	}

	void setGain(String id, double value) {
		if (id == null) return;
		if (nativeAvailable) {
			nativeSetGain(id, value);
		}
		gains.put(id, value);
		try {
			android.util.Log.d("AudioContext", "setGain: id=" + id + " value=" + value);
		} catch (Throwable ignored) {
		}
	}

	void setAudioThreadLoggingEnabled(boolean enabled) {
		if (nativeAvailable) nativeSetAudioThreadLoggingEnabled(enabled);
	}

	double getGain(String id) {
		if (id == null) return 1.0;
		Double v = gains.get(id);
		return v == null ? 1.0 : v;
	}

	void setBiquadParams(String id, double frequency, double Q, double gain, String type) {
		if (id == null) return;
		if (nativeAvailable) {
			nativeSetBiquadParams(id, frequency, Q, gain, type);
		}
	}

	public void setPannerParams(AudioPannerNode panner,
															double positionX, double positionY, double positionZ,
															double orientationX, double orientationY, double orientationZ,
															double pan,
															int distanceModel, int panningModel,
															double refDistance, double maxDistance, double rolloffFactor,
															double coneInnerAngle, double coneOuterAngle, double coneOuterGain) {
		if (nativeAvailable) {
			nativeSetPannerParams(panner.getId(), positionX, positionY, positionZ, orientationX, orientationY, orientationZ, pan, distanceModel, panningModel, refDistance, maxDistance, rolloffFactor, coneInnerAngle, coneOuterAngle, coneOuterGain);
		}
	}

	void setPannerParams(String id,
											 double positionX, double positionY, double positionZ,
											 double orientationX, double orientationY, double orientationZ,
											 double pan,
											 int distanceModel, int panningModel,
											 double refDistance, double maxDistance, double rolloffFactor,
											 double coneInnerAngle, double coneOuterAngle, double coneOuterGain) {
		if (id == null) return;
		if (nativeAvailable) {
			nativeSetPannerParams(id, positionX, positionY, positionZ, orientationX, orientationY, orientationZ, pan, distanceModel, panningModel, refDistance, maxDistance, rolloffFactor, coneInnerAngle, coneOuterAngle, coneOuterGain);
		}
	}

	void cancelGainScheduledValues(String id, double time) {
		if (id == null) return;
		long absNs = -1L;
		String ctx = gainContexts.get(id);
		if (ctx != null) {
			Long startNs = contextStartNanos.get(ctx);
			if (startNs != null) absNs = startNs + (long) (time * 1000000000.0);
		}
		if (nativeAvailable && absNs >= 0) {
			nativeCancelGainEvents(id, absNs);
		}
		java.util.List<ParamEvent> lst = gainEvents.get(id);
		if (lst != null) {
			if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N) {
				lst.removeIf(ev -> ev.time >= time);
			} else {
				Iterator<ParamEvent> it = lst.iterator();
				while (it.hasNext()) {
					ParamEvent ev = it.next();
					if (ev.time >= time) {
						it.remove();
					}
				}
			}
		}
	}


	void cancelAndHoldGainScheduledValues(String id, int rate, double heldValue, double time) {
		if (id == null) return;
		long absNs = -1L;
		String ctx = gainContexts.get(id);
		if (ctx != null) {
			Long startNs = contextStartNanos.get(ctx);
			if (startNs != null) absNs = startNs + (long) (time * 1000000000.0);
		}
		if (nativeAvailable && absNs >= 0) {
			nativeCancelAndHoldGainEvents(id, rate, absNs, heldValue);
		}
		java.util.List<ParamEvent> lst = gainEvents.get(id);
		if (lst != null) {
			if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N) {
				lst.removeIf(ev -> ev.time >= time);
			} else {
				Iterator<ParamEvent> it = lst.iterator();
				while (it.hasNext()) if (it.next().time >= time) it.remove();
			}
		} else {
			lst = java.util.Collections.synchronizedList(new java.util.ArrayList<>());
			gainEvents.put(id, lst);
		}
		lst.add(new ParamEvent(0, rate, time, heldValue));
	}

	void cancelAndHoldPannerScheduledValues(String id, int paramType, int rate, double heldValue, double time) {
		if (id == null) return;
		long absNs = -1L;
		String ctx = pannerContexts.get(id);
		if (ctx != null) {
			Long startNs = contextStartNanos.get(ctx);
			if (startNs != null) absNs = startNs + (long) (time * 1000000000.0);
		}
		if (nativeAvailable && absNs >= 0) {
			nativeCancelAndHoldPannerEvents(id, paramType, rate, absNs, heldValue);
		}
		java.util.concurrent.ConcurrentHashMap<Integer, java.util.List<ParamEvent>> map = pannerEvents.get(id);
		if (map == null) {
			map = new java.util.concurrent.ConcurrentHashMap<>();
			pannerEvents.put(id, map);
		}
		java.util.List<ParamEvent> lst = map.get(paramType);
		if (lst != null) {
			if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N) {
				lst.removeIf(ev -> ev.time >= time);
			} else {
				Iterator<ParamEvent> it = lst.iterator();
				while (it.hasNext()) if (it.next().time >= time) it.remove();
			}
		} else {
			lst = java.util.Collections.synchronizedList(new java.util.ArrayList<>());
			map.put(paramType, lst);
		}
		lst.add(new ParamEvent(0, rate, time, heldValue));
	}

	void cancelPannerScheduledValues(String id, int paramType, double time) {
		if (id == null) return;
		long absNs = -1L;
		String ctx = pannerContexts.get(id);
		if (ctx != null) {
			Long startNs = contextStartNanos.get(ctx);
			if (startNs != null) absNs = startNs + (long) (time * 1000000000.0);
		}
		if (nativeAvailable && absNs >= 0) {
			nativeCancelPannerEvents(id, paramType, absNs);
		}
		java.util.concurrent.ConcurrentHashMap<Integer, java.util.List<ParamEvent>> map = pannerEvents.get(id);
		if (map != null) {
			java.util.List<ParamEvent> lst = map.get(paramType);
			if (lst != null) if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N) {
				lst.removeIf(ev -> ev.time >= time);
			} else {
				Iterator<ParamEvent> it = lst.iterator();
				while (it.hasNext()) {
					ParamEvent ev = it.next();
					if (ev.time >= time) {
						it.remove();
					}
				}
			}
		}
	}

	double[] getPannerParamValues(String id, int paramType, double startTime, double sampleRate, int frameCount) {
		if (id == null) return new double[frameCount];
		long absNs = -1L;
		String ctx = pannerContexts.get(id);
		if (ctx != null) {
			Long startNs = contextStartNanos.get(ctx);
			if (startNs != null) absNs = startNs + (long) (startTime * 1000000000.0);
		}
		if (nativeAvailable && absNs >= 0) {
			try {
				double[] a = nativeGetPannerParamValues(id, paramType, absNs, sampleRate, frameCount);
				if (a != null) return a;
			} catch (Throwable ignored) {
			}
		}
		java.util.concurrent.ConcurrentHashMap<Integer, java.util.List<ParamEvent>> map = pannerEvents.get(id);
		double[] out = new double[frameCount];
		double defaultVal = 0.0;
		java.util.List<ParamEvent> lst = null;
		if (map != null) lst = map.get(paramType);
		if (lst == null || lst.isEmpty()) {
			Arrays.fill(out, defaultVal);
			return out;
		}
		for (int i = 0; i < frameCount; ++i) {
			double t = startTime + ((double) i) / (sampleRate > 0.0 ? sampleRate : 48000.0);
			double val;
			int idx = -1;
			for (int j = 0; j < lst.size(); ++j) if (lst.get(j).time <= t) idx = j;
			if (idx < 0) val = defaultVal;
			else if (idx >= lst.size() - 1) val = lst.get(lst.size() - 1).value;
			else {
				ParamEvent prev = lst.get(idx);
				ParamEvent next = lst.get(idx + 1);
				if (next.type == 1) {
					double ratio = (t - prev.time) / (next.time - prev.time);
					if (ratio < 0.0) ratio = 0.0;
					if (ratio > 1.0) ratio = 1.0;
					val = prev.value + (next.value - prev.value) * ratio;
				} else val = prev.value;
			}
			out[i] = val;
		}
		return out;
	}

	double[] getGainParamValues(String id, double startTime, double sampleRate, int frameCount) {
		if (id == null) return new double[frameCount];
		long absNs = -1L;
		String ctx = gainContexts.get(id);
		if (ctx != null) {
			Long startNs = contextStartNanos.get(ctx);
			if (startNs != null) absNs = startNs + (long) (startTime * 1000000000.0);
		}
		if (nativeAvailable && absNs >= 0) {
			try {
				double[] a = nativeGetGainParamValues(id, absNs, sampleRate, frameCount);
				if (a != null) return a;
			} catch (Throwable ignored) {
			}
		}
		java.util.List<ParamEvent> lst = gainEvents.get(id);
		double[] out = new double[frameCount];
		double defaultVal = 1.0;
		if (lst == null || lst.isEmpty()) {
			for (int i = 0; i < frameCount; ++i) out[i] = defaultVal;
			return out;
		}
		for (int i = 0; i < frameCount; ++i) {
			double t = startTime + ((double) i) / (sampleRate > 0.0 ? sampleRate : 48000.0);
			double val = defaultVal;
			int idx = -1;
			for (int j = 0; j < lst.size(); ++j) if (lst.get(j).time <= t) idx = j;
			if (idx < 0) val = defaultVal;
			else if (idx >= lst.size() - 1) val = lst.get(lst.size() - 1).value;
			else {
				ParamEvent prev = lst.get(idx);
				ParamEvent next = lst.get(idx + 1);
				if (next.type == 1) {
					double ratio = (t - prev.time) / (next.time - prev.time);
					if (ratio < 0.0) ratio = 0.0;
					if (ratio > 1.0) ratio = 1.0;
					val = prev.value + (next.value - prev.value) * ratio;
				} else val = prev.value;
			}
			out[i] = val;
		}
		return out;
	}

	void attachPannerToVoice(String voiceId, String pannerId) {
		if (voiceId == null || pannerId == null) return;
		if (nativeAvailable) {
			nativeAttachPanner(voiceId, pannerId);
		}
	}

	void attachPannerToVoice(String voiceId, String pannerId, int output, int input) {
		if (voiceId == null || pannerId == null) return;
		if (nativeAvailable) {
			nativeAttachPanner(voiceId, pannerId, output, input);
			return;
		}
		attachPannerToVoice(voiceId, pannerId);
	}

	void detachPanner(String pannerId) {
		if (pannerId == null) return;
		if (nativeAvailable) {
			nativeDetachPanner(pannerId);
		}
	}

	void releasePanner(String id) {
		if (id == null) return;
		if (nativeAvailable) {
			nativeReleasePanner(id);
		}
		pannerContexts.remove(id);
		clearNodeSources(id);
	}

	void attachBiquadToVoice(String voiceId, String biquadId) {
		if (voiceId == null || biquadId == null) return;
		if (nativeAvailable) {
			nativeAttachBiquad(voiceId, biquadId);
		}
	}

	void attachBiquadToVoice(String voiceId, String biquadId, int output, int input) {
		if (voiceId == null || biquadId == null) return;
		if (nativeAvailable) {
			nativeAttachBiquad(voiceId, biquadId, output, input);
			return;
		}
		attachBiquadToVoice(voiceId, biquadId);
	}

	void detachBiquad(String biquadId) {
		if (biquadId == null) return;
		if (nativeAvailable) {
			nativeDetachBiquad(biquadId);
		}
	}

	void releaseBiquad(String id) {
		if (id == null) return;
		if (nativeAvailable) {
			nativeReleaseBiquad(id);
		}
		clearNodeSources(id);
	}

	void releaseIIR(String id) {
		if (id == null) return;
		if (nativeAvailable) {
			nativeReleaseIIR(id);
		}
		clearNodeSources(id);
	}

	void releaseGain(String id) {
		if (id == null) return;
		if (nativeAvailable) {
			nativeReleaseGain(id);
		}
		gains.remove(id);
		gainContexts.remove(id);
		waveShaperCurves.remove(id);
		waveShaperCurveBuffers.remove(id);
		waveShaperOversamples.remove(id);
		destinationIds.remove(id);
		clearNodeSources(id);
	}

	void detachGain(String gainId) {
		if (gainId == null) return;
		if (nativeAvailable) {
			nativeDetachGain(gainId);
			return;
		}

		for (String vid : voiceGainMap.keySet()) {
			String gid = voiceGainMap.get(vid);
			if (gid != null && gid.equals(gainId)) {
				voiceGainMap.remove(vid);
			}
		}

		for (String vid : voiceGainMapByOutput.keySet()) {
			ConcurrentHashMap<Integer, String> map = voiceGainMapByOutput.get(vid);
			if (map != null) {
				for (Integer out : map.keySet()) {
					String gid = map.get(out);
					if (gid != null && gid.equals(gainId)) map.remove(out);
				}
				if (map.isEmpty()) voiceGainMapByOutput.remove(vid);
			}
		}
	}

	void detachPlaybackRate(String playbackRateId) {
		if (playbackRateId == null) return;
		if (nativeAvailable) {
			nativeDetachPlaybackRate(playbackRateId);
			return;
		}

		for (String vid : voicePlaybackRateMap.keySet()) {
			String pid = voicePlaybackRateMap.get(vid);
			if (pid != null && pid.equals(playbackRateId)) {
				voicePlaybackRateMap.remove(vid);
			}
		}

		for (String vid : voicePlaybackRateMapByOutput.keySet()) {
			ConcurrentHashMap<Integer, String> map = voicePlaybackRateMapByOutput.get(vid);
			if (map != null) {
				for (Integer out : map.keySet()) {
					String pid = map.get(out);
					if (pid != null && pid.equals(playbackRateId)) map.remove(out);
				}
				if (map.isEmpty()) voicePlaybackRateMapByOutput.remove(vid);
			}
		}
	}

	void releasePlaybackRate(String id) {
		if (id == null) return;
		if (nativeAvailable) {
			nativeReleasePlaybackRate(id);
		}
		for (String vid : voicePlaybackRateMap.keySet()) {
			String pid = voicePlaybackRateMap.get(vid);
			if (pid != null && pid.equals(id)) {
				voicePlaybackRateMap.remove(vid);
			}
		}
		for (String vid : voicePlaybackRateMapByOutput.keySet()) {
			ConcurrentHashMap<Integer, String> map = voicePlaybackRateMapByOutput.get(vid);
			if (map != null) {
				for (Integer out : map.keySet()) {
					String pid = map.get(out);
					if (pid != null && pid.equals(id)) map.remove(out);
				}
				if (map.isEmpty()) voicePlaybackRateMapByOutput.remove(vid);
			}
		}
	}

	void attachGainToVoice(String voiceId, String gainId) {
		if (voiceId == null || gainId == null) return;
		if (nativeAvailable) {
			nativeAttachGain(voiceId, gainId);
			return;
		}
		voiceGainMap.put(voiceId, gainId);
		try {
			android.util.Log.d("AudioContext", "attachGainToVoice: voice=" + voiceId + " gain=" + gainId);
		} catch (Throwable ignored) {
		}
	}

	void attachGainToVoice(String voiceId, String gainId, int output, int input) {
		if (voiceId == null || gainId == null) return;
		if (nativeAvailable) {
			nativeAttachGain(voiceId, gainId, output, input);
			return;
		}

		if (gainId.isEmpty()) {
			ConcurrentHashMap<Integer, String> map = voiceGainMapByOutput.get(voiceId);
			if (map != null) {
				map.remove(output);
				if (map.isEmpty()) voiceGainMapByOutput.remove(voiceId);
			}
			if (output == 0) voiceGainMap.remove(voiceId);
			return;
		}
		ConcurrentHashMap<Integer, String> map = voiceGainMapByOutput.get(voiceId);
		if (map == null) {
			map = new ConcurrentHashMap<>();
			voiceGainMapByOutput.put(voiceId, map);
		}
		map.put(output, gainId);

		voiceGainMap.put(voiceId, gainId);
	}

	void attachPlaybackRateToVoice(String voiceId, String playbackRateId) {
		if (voiceId == null || playbackRateId == null) return;
		if (nativeAvailable) {
			nativeAttachPlaybackRate(voiceId, playbackRateId);
			return;
		}
		voicePlaybackRateMap.put(voiceId, playbackRateId);
		try {
			android.util.Log.d("AudioContext", "attachPlaybackRateToVoice: voice=" + voiceId + " pr=" + playbackRateId);
		} catch (Throwable ignored) {
		}
	}

	void attachPlaybackRateToVoice(String voiceId, String playbackRateId, int output, int input) {
		if (voiceId == null || playbackRateId == null) return;
		if (nativeAvailable) {
			nativeAttachPlaybackRate(voiceId, playbackRateId, output, input);
			return;
		}

		if (playbackRateId.isEmpty()) {
			ConcurrentHashMap<Integer, String> map = voicePlaybackRateMapByOutput.get(voiceId);
			if (map != null) {
				map.remove(output);
				if (map.isEmpty()) voicePlaybackRateMapByOutput.remove(voiceId);
			}
			if (output == 0) voicePlaybackRateMap.remove(voiceId);
			return;
		}
		ConcurrentHashMap<Integer, String> map = voicePlaybackRateMapByOutput.get(voiceId);
		if (map == null) {
			map = new ConcurrentHashMap<>();
			voicePlaybackRateMapByOutput.put(voiceId, map);
		}
		map.put(output, playbackRateId);

		voicePlaybackRateMap.put(voiceId, playbackRateId);
	}

	public AudioBufferSourceNode createBufferSource(AudioContextInstance context, @Nullable AudioBuffer buffer) {
		String id = createBufferSource(context.getId(), buffer);
		GainNode pr = createGain(context);
		attachPlaybackRateToVoice(id, pr.getId());
		return new AudioBufferSourceNode(id, buffer, pr.getId());
	}

	public ExternalPcmSourceNode createExternalPcmSource(AudioContextInstance context, int sampleRate, int channels) {
		String id = createExternalPcmSource(sampleRate, channels);
		if (id == null) return null;
		GainNode pr = createGain(context);
		attachPlaybackRateToVoice(id, pr.getId());
		return new ExternalPcmSourceNode(id, sampleRate, channels, pr.getId());
	}

	public String createExternalPcmSource(int sampleRate, int channels) {
		if (nativeAvailable) {
			return nativeCreateExternalPcmSource(sampleRate, channels);
		}
		return null;
	}

	public void pushPcmFrames(String trackId, float[] data) {
		if (trackId == null || data == null || data.length == 0) return;
		if (nativeAvailable) {
			nativePushPcmFramesFloatArray(trackId, data, data.length);
		}
	}


	public void pushPcmFrames(String trackId, java.nio.FloatBuffer data) {
		if (trackId == null || data == null) return;
		int remaining = data.remaining();
		if (remaining <= 0) return;
		if (!data.isDirect()) {
			float[] copy = new float[remaining];
			int p = data.position();
			data.get(copy, 0, remaining);
			data.position(p);
			pushPcmFrames(trackId, copy);
			return;
		}
		if (nativeAvailable) {
			nativePushPcmFramesDirect(trackId, data, remaining);
		}
	}


	public void endExternalPcmSource(String trackId) {
		if (trackId == null) return;
		if (nativeAvailable) {
			nativeEndExternalPcmSource(trackId);
		}

		fireEnded(trackId);
	}

	private native String nativeCreateExternalPcmSource(int sampleRate, int channels);

	private native void nativePushPcmFramesFloatArray(String trackId, float[] data, int frames);

	private native void nativePushPcmFramesDirect(String trackId, java.nio.Buffer data, int sampleCount);

	private native void nativeEndExternalPcmSource(String trackId);

	@Nullable
	String switchBufferSource(String trackId, @Nullable AudioBuffer buffer) {
		if (trackId == null) return null;
		stopTrack(trackId);

		if (buffer == null) {
			directBuffers.remove(trackId + "::buffer");
			return null;
		}

		if (nativeAvailable) {
			java.nio.ByteBuffer src = directBuffers.get(buffer.id);
			if (src == null) return null;
			try {
				int sampleRate = sampleRates.containsKey(buffer.id) ? sampleRates.get(buffer.id) : 48000;
				int channels = bufferChannels.containsKey(buffer.id) ? bufferChannels.get(buffer.id) : 1;
				int byteLen = src.capacity();
				int bytesPerSample = (byteLen % 4 == 0) ? 4 : 2;
				java.nio.ByteBuffer srcView = src.duplicate();
				srcView.position(src.position());
				srcView.limit(src.limit());
				java.nio.ByteBuffer srcSlice = srcView.slice();
				String newId = nativeCreateBufferSourceDirect(srcSlice, sampleRate, channels, bytesPerSample);
				if (newId == null || newId.isEmpty()) return null;
				directBuffers.put(newId, src);
				getOrCreate(newId).incrementAndGet();
				sampleRates.put(newId, sampleRate);
				bufferChannels.put(newId, channels);
				return newId;
			} catch (Throwable ignored) {
				return null;
			}
		}

		java.nio.ByteBuffer src = directBuffers.get(buffer.id);
		if (src == null) return null;
		String bufferId = buffer.id;
		int sampleRate = sampleRates.containsKey(bufferId) ? sampleRates.get(bufferId) : 48000;
		int srcChannels = bufferChannels.containsKey(bufferId) ? bufferChannels.get(bufferId) : 1;
		int byteLen = src.capacity();
		int bytesPerSample = (byteLen % 4 == 0) ? 4 : 2;
		int frames = (byteLen / bytesPerSample) / srcChannels;
		java.nio.ByteBuffer monoBb = java.nio.ByteBuffer.allocateDirect(frames * 4).order(ByteOrder.LITTLE_ENDIAN);
		java.nio.ByteBuffer dup = src.duplicate().order(ByteOrder.LITTLE_ENDIAN);
		if (bytesPerSample == 4) {
			for (int i = 0; i < frames; i++) {
				monoBb.putFloat(dup.getFloat((i * srcChannels) * 4));
			}
		} else {
			for (int i = 0; i < frames; i++) {
				monoBb.putFloat(dup.getShort((i * srcChannels) * 2) / 32768.0f);
			}
		}
		monoBb.position(0);
		directBuffers.put(trackId + "::buffer", monoBb);
		sampleRates.put(trackId, sampleRate);
		return null;
	}

	String createBufferSource(String contextId, @Nullable AudioBuffer buffer) {
		String bufferId = "";
		if (buffer != null) {
			bufferId = buffer.id;
		}
		java.nio.ByteBuffer src = directBuffers.get(bufferId);
		if (nativeAvailable && nativeHasBuffer(bufferId)) {
			addToContextSet(contextBuffers, contextId, bufferId);
			addToContextSet(contextTracksMap, contextId, bufferId);
			return bufferId;
		}
		if (nativeAvailable && src != null) {
			try {
				int sampleRate = sampleRates.containsKey(bufferId) ? sampleRates.get(bufferId) : 48000;
				int channels = bufferChannels.containsKey(bufferId) ? bufferChannels.get(bufferId) : 1;
				int byteLen = src.capacity();
				int bytesPerSample = (byteLen % 4 == 0) ? 4 : 2;
				java.nio.ByteBuffer srcView = src.duplicate();
				srcView.position(src.position());
				srcView.limit(src.limit());
				java.nio.ByteBuffer srcSlice = srcView.slice();
				String id = nativeCreateBufferSourceDirect(srcSlice, sampleRate, channels, bytesPerSample);
				if (id != null) {
					directBuffers.put(id, src);
					getOrCreate(id).incrementAndGet();
					sampleRates.put(id, sampleRate);
					bufferChannels.put(id, channels);
					addToContextSet(contextBuffers, contextId, id);
					addToContextSet(contextTracksMap, contextId, id);
					return id;
				}
			} catch (Throwable ignored) {
			}
		}
		int sampleRate = 48000;
		if (sampleRates.containsKey(bufferId)) sampleRate = sampleRates.get(bufferId);
		else if (contextId != null && contextSampleRates.containsKey(contextId))
			sampleRate = contextSampleRates.get(contextId);
		int bufferSize = AudioTrack.getMinBufferSize(sampleRate, AudioFormat.CHANNEL_OUT_MONO, AudioFormat.ENCODING_PCM_16BIT);
		AudioTrack track = new AudioTrack(AudioManager.STREAM_MUSIC, sampleRate, AudioFormat.CHANNEL_OUT_MONO, AudioFormat.ENCODING_PCM_16BIT, Math.max(bufferSize, 2048), AudioTrack.MODE_STREAM);
		String id = UUID.randomUUID().toString();
		tracks.put(id, track);
		java.nio.ByteBuffer monoBb;
		if (src != null) {
			int byteLen = src.capacity();
			int bytesPerSample = (byteLen % 4 == 0) ? 4 : 2;
			int srcChannels = bufferChannels.containsKey(bufferId) ? bufferChannels.get(bufferId) : 1;
			int frames = (byteLen / bytesPerSample) / srcChannels;
			monoBb = java.nio.ByteBuffer.allocateDirect(frames * 4).order(ByteOrder.LITTLE_ENDIAN);
			java.nio.ByteBuffer dup = src.duplicate().order(ByteOrder.LITTLE_ENDIAN);
			if (bytesPerSample == 4) {
				for (int i = 0; i < frames; i++) {
					float f = dup.getFloat((i * srcChannels) * 4);
					monoBb.putFloat(f);
				}
			} else {
				for (int i = 0; i < frames; i++) {
					short s = dup.getShort((i * srcChannels) * 2);
					monoBb.putFloat((float) s / 32768.0f);
				}
			}
			monoBb.position(0);
			directBuffers.put(id + "::buffer", monoBb);
		}
		if (contextId != null) {
			addToContextSet(contextTracksMap, contextId, id);
			addToContextSet(contextBuffers, contextId, id + "::buffer");
		}
		return id;
	}

	public void startBufferSource(final String trackId, final boolean loop) {
		if (nativeAvailable) {
			nativeStartBufferSource(trackId, loop);
			return;
		}

		final AudioTrack t = tracks.get(trackId);
		final java.nio.ByteBuffer buf = directBuffers.get(trackId + "::buffer");
		if (t == null) return;

		Thread th = new Thread(() -> {
			t.play();
			try {
				if (buf == null) {
					short[] silence = new short[1024];
					while (t.getPlayState() == AudioTrack.PLAYSTATE_PLAYING
						&& !Thread.currentThread().isInterrupted()) {
						t.write(silence, 0, silence.length);
					}
				} else {
					final int CHUNK_FRAMES = 4096;
					final short[] chunk = new short[CHUNK_FRAMES];
					final java.nio.ByteBuffer dup = buf.duplicate().order(ByteOrder.LITTLE_ENDIAN);
					final int byteLen = dup.capacity();
					final int bytesPerSample = (byteLen % 4 == 0) ? 4 : 2;
					final int totalFrames = byteLen / bytesPerSample;

					do {
						double g = 1.0;
						String gid = null;
						ConcurrentHashMap<Integer, String> om = voiceGainMapByOutput.get(trackId);
						if (om != null) gid = om.get(0);
						if (gid == null) gid = voiceGainMap.get(trackId);
						if (gid != null) {
							Double gf = gains.get(gid);
							if (gf != null) g = gf;
						}

						int offset = 0;
						while (offset < totalFrames) {


							if (Thread.currentThread().isInterrupted()) return;
							if (t.getPlayState() != AudioTrack.PLAYSTATE_PLAYING) return;

							int count = Math.min(CHUNK_FRAMES, totalFrames - offset);
							if (bytesPerSample == 4) {
								for (int i = 0; i < count; i++) {
									float v = dup.getFloat((offset + i) * 4);
									int val = (int) Math.round(v * 32767.0 * g);
									if (val > Short.MAX_VALUE) val = Short.MAX_VALUE;
									if (val < Short.MIN_VALUE) val = Short.MIN_VALUE;
									chunk[i] = (short) val;
								}
							} else {
								for (int i = 0; i < count; i++) {
									short s = dup.getShort((offset + i) * 2);
									int val = (int) Math.round((s / 32768.0) * 32767.0 * g);
									if (val > Short.MAX_VALUE) val = Short.MAX_VALUE;
									if (val < Short.MIN_VALUE) val = Short.MIN_VALUE;
									chunk[i] = (short) val;
								}
							}
							t.write(chunk, 0, count);
							offset += count;
						}
					} while (loop && !Thread.currentThread().isInterrupted());
				}
			} finally {
				try {
					t.stop();
				} catch (Exception ignored) {
				}
			}
		});
		trackThreads.put(trackId, th);
		th.start();
	}

	public WaveShaperNode createWaveShaper(AudioContextInstance context) {
		return new WaveShaperNode(createWaveShaper(context.getId()));
	}

	String createWaveShaper(String contextId) {
		String id = null;
		if (nativeAvailable) {
			try {
				id = nativeCreateGain();
			} catch (Throwable ignored) {
			}
		}
		if (id == null) {
			id = UUID.randomUUID().toString();
			gains.put(id, 1.0);
		}

		waveShaperOversamples.put(id, "none");
		if (contextId != null) {
			addToContextSet(contextGains, contextId, id);
			gainContexts.put(id, contextId);
		}
		return id;
	}


	void setWaveShaperCurveFromData(String id, @Nullable java.nio.FloatBuffer data) {
		if (id == null) return;
		if (data == null) {
			waveShaperCurves.remove(id);
			waveShaperCurveBuffers.remove(id);
			if (nativeAvailable) {
				try {
					nativeSetWaveShaperCurveDirect(id, null, getWaveShaperOversample(id));
				} catch (Throwable ignored) {
				}
			}
			return;
		}

		if (data.isDirect()) {
			try {
				java.nio.FloatBuffer view = data.duplicate();
				view.position(data.position());
				view.limit(data.limit());
				java.nio.FloatBuffer slice = view.slice();

				nativeSetWaveShaperCurveDirect(id, slice, getWaveShaperOversample(id));
				java.nio.FloatBuffer dup = data.duplicate();
				dup.rewind();
				waveShaperCurveBuffers.put(id, dup);
				waveShaperCurves.remove(id);
				return;
			} catch (Throwable ignored) {
			}
		}
		data.rewind();
		float[] arr = new float[data.remaining()];
		data.get(arr);
		waveShaperCurves.put(id, arr);
		waveShaperCurveBuffers.remove(id);
		if (nativeAvailable) {
			try {
				nativeSetWaveShaperCurveArray(id, arr, getWaveShaperOversample(id));
			} catch (Throwable ignored) {
			}
		}
	}

	void setWaveShaperCurveFromData(String id, java.nio.ByteBuffer data) {
		if (id == null) return;
		if (data == null) {
			waveShaperCurves.remove(id);
			waveShaperCurveBuffers.remove(id);
			if (nativeAvailable) {
				try {
					nativeSetWaveShaperCurveDirect(id, null, getWaveShaperOversample(id));
				} catch (Throwable ignored) {
				}
			}
			return;
		}
		data.order(java.nio.ByteOrder.LITTLE_ENDIAN);
		java.nio.FloatBuffer fb = data.asFloatBuffer();
		if (data.isDirect()) {
			try {
				java.nio.FloatBuffer view = fb.duplicate();
				view.position(fb.position());
				view.limit(fb.limit());
				java.nio.FloatBuffer slice = view.slice();

				nativeSetWaveShaperCurveDirect(id, slice, getWaveShaperOversample(id));
				java.nio.FloatBuffer dup = fb.duplicate();
				dup.rewind();
				waveShaperCurveBuffers.put(id, dup);
				waveShaperCurves.remove(id);
				return;
			} catch (Throwable ignored) {
			}
		}
		float[] arr = new float[fb.remaining()];
		fb.get(arr);
		waveShaperCurves.put(id, arr);
		waveShaperCurveBuffers.remove(id);
		if (nativeAvailable) {
			try {
				nativeSetWaveShaperCurveArray(id, arr, getWaveShaperOversample(id));
			} catch (Throwable ignored) {
			}
		}
	}

	float[] getWaveShaperCurve(String id) {
		if (id == null) return null;
		float[] arr = waveShaperCurves.get(id);
		if (arr != null) return arr;
		java.nio.FloatBuffer fb = waveShaperCurveBuffers.get(id);
		if (fb == null) return null;
		java.nio.FloatBuffer dup = fb.duplicate();
		dup.rewind();
		float[] out = new float[dup.remaining()];
		dup.get(out);
		return out;
	}

	public float[] getAnalyserFloatTimeDomainData(String id, int count) {
		if (id == null) return null;
		if (nativeAvailable) {
			try {
				return nativeGetAnalyserFloatTimeDomainData(id, count);
			} catch (Throwable ignored) {
			}
		}
		return null;
	}

	public float[] getAnalyserFloatFrequencyData(String id) {
		if (id == null) return null;
		if (nativeAvailable) {
			try {
				return nativeGetAnalyserFloatFrequencyData(id);
			} catch (Throwable ignored) {
			}
		}
		return null;
	}

	public boolean getAnalyserFloatTimeDomainDataDirect(String id, java.nio.FloatBuffer dest) {
		if (id == null || dest == null || !dest.isDirect()) return false;
		if (nativeAvailable) {
			try {
				java.nio.FloatBuffer view = dest.duplicate();
				view.position(dest.position());
				view.limit(dest.limit());
				java.nio.FloatBuffer slice = view.slice();
				return nativeGetAnalyserFloatTimeDomainDataDirect(id, slice);
			} catch (Throwable ignored) {
			}
		}
		return false;
	}

	public boolean getAnalyserFloatFrequencyDataDirect(String id, java.nio.FloatBuffer dest) {
		if (id == null || dest == null || !dest.isDirect()) return false;
		if (nativeAvailable) {
			try {
				java.nio.FloatBuffer view = dest.duplicate();
				view.position(dest.position());
				view.limit(dest.limit());
				java.nio.FloatBuffer slice = view.slice();
				return nativeGetAnalyserFloatFrequencyDataDirect(id, slice);
			} catch (Throwable ignored) {
			}
		}
		return false;
	}

	public boolean getAnalyserByteTimeDomainDataDirect(String id, java.nio.ByteBuffer dest) {
		if (id == null || dest == null || !dest.isDirect()) return false;
		if (nativeAvailable) {
			try {
				java.nio.ByteBuffer view = dest.duplicate();
				view.position(dest.position());
				view.limit(dest.limit());
				java.nio.ByteBuffer slice = view.slice();
				return nativeGetAnalyserByteTimeDomainDataDirect(id, slice);
			} catch (Throwable ignored) {
			}
		}
		return false;
	}

	public boolean getAnalyserByteFrequencyDataDirect(String id, java.nio.ByteBuffer dest, float minDecibels, float maxDecibels) {
		if (id == null || dest == null || !dest.isDirect()) return false;
		if (nativeAvailable) {
			try {
				java.nio.ByteBuffer view = dest.duplicate();
				view.position(dest.position());
				view.limit(dest.limit());
				java.nio.ByteBuffer slice = view.slice();
				return nativeGetAnalyserByteFrequencyDataDirect(id, slice, minDecibels, maxDecibels);
			} catch (Throwable ignored) {
			}
		}
		return false;
	}

	public void setAnalyserDecibels(String id, float minDecibels, float maxDecibels) {
		if (id == null) return;
		if (nativeAvailable) {
			nativeSetAnalyserDecibels(id, minDecibels, maxDecibels);
		}
	}

	public void setAnalyserFftSize(String id, int fftSize) {
		if (id == null) return;
		if (nativeAvailable) {
			try {
				nativeSetAnalyserFftSize(id, fftSize);
			} catch (Throwable ignored) {
			}
		}
	}

	public void setAnalyserSmoothingTimeConstant(String id, double value) {
		if (id == null) return;
		if (nativeAvailable) {
			try {
				nativeSetAnalyserSmoothing(id, value);
			} catch (Throwable ignored) {
			}
		}
	}

	public void releaseAnalyser(String id) {
		if (id == null) return;
		if (nativeAvailable) {
			try {
				nativeReleaseAnalyser(id);
			} catch (Throwable ignored) {
			}
		}
	}

	void setWaveShaperOversample(String id, String oversample) {
		if (id == null) return;
		if (oversample == null) waveShaperOversamples.remove(id);
		else waveShaperOversamples.put(id, oversample);
		if (nativeAvailable) {
			try {
				nativeSetWaveShaperOversample(id, oversample);
			} catch (Throwable ignored) {
			}
		}
	}

	String getWaveShaperOversample(String id) {
		if (id == null) return "none";
		String v = waveShaperOversamples.get(id);
		return v != null ? v : "none";
	}

	public AnalyserNode createAnalyser(AudioContextInstance context) {
		return new AnalyserNode(createAnalyser(context.getId()));
	}

	String createAnalyser(String contextId) {
		String id = null;
		if (nativeAvailable) {
			try {
				id = nativeCreateAnalyser(2048, 0.8, -100.0, -30.0);
			} catch (Throwable ignored) {
			}
		}
		if (id == null) {
			id = UUID.randomUUID().toString();
			gains.put(id, 1.0);
		}
		if (contextId != null) {
			addToContextSet(contextGains, contextId, id);
			gainContexts.put(id, contextId);
		}
		return id;
	}

	public DelayNode createDelay(AudioContextInstance context) {
		return new DelayNode(createDelay(context.getId()));
	}

	String createDelay(String contextId) {
		String id = null;
		if (nativeAvailable) {
			try {
				id = nativeCreateGain();
			} catch (Throwable ignored) {
			}
		}
		if (id == null) {
			id = UUID.randomUUID().toString();
			gains.put(id, 1.0);
		}
		if (contextId != null) {
			addToContextSet(contextGains, contextId, id);
			gainContexts.put(id, contextId);
		}
		return id;
	}

	public ConstantSourceNode createConstantSource(AudioContextInstance context) {
		return new ConstantSourceNode(context);
	}

}

