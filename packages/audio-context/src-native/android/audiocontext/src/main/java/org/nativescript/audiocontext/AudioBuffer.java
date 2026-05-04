package org.nativescript.audiocontext;

import androidx.annotation.Nullable;

import java.nio.ByteBuffer;
import java.util.UUID;

public class AudioBuffer implements NativeObject {
	final String id;
	private ByteBuffer directBuffer = null;
	private int sampleRate = 48000;
	private int channels = 1;
	private final java.util.concurrent.ConcurrentHashMap<Integer, ByteBuffer> channelBufferCache = new java.util.concurrent.ConcurrentHashMap<>();
	private final java.util.concurrent.ConcurrentHashMap<Integer, Boolean> dirtyChannels = new java.util.concurrent.ConcurrentHashMap<>();

	AudioBuffer(String id, ByteBuffer buffer, @Nullable Integer sampleRate, @Nullable Integer channels) {
		this.id = id;
		this.directBuffer = buffer;
		if (sampleRate != null) {
			this.sampleRate = sampleRate;
		}
		if (channels != null) {
			this.channels = channels;
		}

		NativeFinalizer.register(this, NativeFinalizer.ResourceKind.BUFFER, id);
	}

	@Override
	public String getId() {
		return id;
	}

	public AudioBuffer(int length, int numberOfChannels, int sampleRate) {
		String nid = UUID.randomUUID().toString();
		this.id = nid;
		this.channels = Math.max(1, numberOfChannels);
		this.sampleRate = sampleRate > 0 ? sampleRate : 48000;
		int frames = Math.max(0, length);
		this.directBuffer = ByteBuffer.allocateDirect(frames * this.channels * 4).order(java.nio.ByteOrder.LITTLE_ENDIAN);
		for (int i = 0; i < frames * this.channels; i++) this.directBuffer.putFloat(0.0f);
		this.directBuffer.position(0);
		NativeFinalizer.register(this, NativeFinalizer.ResourceKind.BUFFER, nid);
		AudioContext.getInstance().registerBuffer(nid, this.directBuffer, this.sampleRate, this.channels);
	}

	@Override
	public void release() {
		AudioContext.getInstance().releaseBuffer(id);
	}

	public int getSampleRate() {
		return sampleRate;
	}

	public int getLength() {
		if (directBuffer != null) {
			int byteLen = directBuffer.capacity();
			int bytesPerSample = (byteLen % 4 == 0) ? 4 : 2;
			return (byteLen / bytesPerSample) / Math.max(1, channels);
		}
		return 0;
	}

	public double getDuration() {
		int sr = getSampleRate();
		if (sr <= 0) return 0.0;
		return ((double) getLength()) / ((double) sr);
	}

	public int getNumberOfChannels() {
		return channels;
	}

	public java.nio.ByteBuffer getChannelDataRaw(int channel) {
		if (channel < 0 || channel >= channels) return null;
		if (directBuffer == null) return null;

		ByteBuffer channelBuffer = channelBufferCache.get(channel);
		if (channelBuffer == null) {
			channelBuffer = buildChannelBuffer(channel);
			channelBufferCache.put(channel, channelBuffer);
		}
		java.nio.ByteBuffer view = channelBuffer.duplicate().order(java.nio.ByteOrder.LITTLE_ENDIAN);
		view.position(0);
		view.limit(channelBuffer.capacity());
		dirtyChannels.put(channel, Boolean.TRUE);
		return view;
	}

	public java.nio.FloatBuffer getChannelData(int channel) {
		java.nio.ByteBuffer raw = getChannelDataRaw(channel);
		if (raw == null) return null;
		return raw.order(java.nio.ByteOrder.LITTLE_ENDIAN).asFloatBuffer();
	}

	@Nullable
	java.nio.ByteBuffer[] getAllChannelDataRaw() {
		if (channels <= 0) return null;
		if (directBuffer == null && channelBufferCache.isEmpty()) return null;

		java.nio.ByteBuffer[] out = new java.nio.ByteBuffer[channels];
		for (int channel = 0; channel < channels; channel++) {
			ByteBuffer channelBuffer = channelBufferCache.get(channel);
			if (channelBuffer == null) {
				if (directBuffer == null) return null;
				channelBuffer = buildChannelBuffer(channel);
				channelBufferCache.put(channel, channelBuffer);
			}

			java.nio.ByteBuffer view = channelBuffer.duplicate().order(java.nio.ByteOrder.LITTLE_ENDIAN);
			view.position(0);
			view.limit(channelBuffer.capacity());
			out[channel] = view;
		}

		return out;
	}

	boolean hasChannelDataCache() {
		return !channelBufferCache.isEmpty();
	}

	private int resolveBytesPerSample() {
		if (directBuffer == null) return 4;
		int byteLen = directBuffer.capacity();
		return (byteLen % 4 == 0) ? 4 : 2;
	}

	private int resolveFrames(int bytesPerSample) {
		if (directBuffer == null) return 0;
		if (channels <= 0 || bytesPerSample <= 0) return 0;
		int byteLen = directBuffer.capacity();
		return (byteLen / bytesPerSample) / channels;
	}

	private ByteBuffer buildChannelBuffer(int channel) {
		int bytesPerSample = resolveBytesPerSample();
		int frames = resolveFrames(bytesPerSample);
		ByteBuffer out = ByteBuffer.allocateDirect(Math.max(0, frames) * 4).order(java.nio.ByteOrder.LITTLE_ENDIAN);
		if (directBuffer == null || frames <= 0) {
			out.position(0);
			return out;
		}

		java.nio.ByteBuffer src = directBuffer.duplicate().order(java.nio.ByteOrder.LITTLE_ENDIAN);
		if (bytesPerSample == 4) {
			for (int i = 0; i < frames; i++) {
				float f = src.getFloat((i * channels + channel) * 4);
				out.putFloat(f);
			}
		} else {
			for (int i = 0; i < frames; i++) {
				short s = src.getShort((i * channels + channel) * 2);
				out.putFloat((float) s / 32768.0f);
			}
		}
		out.position(0);
		return out;
	}

	void syncChannelDataToInterleaved() {
		if (directBuffer == null || directBuffer.isReadOnly()) return;
		if (channelBufferCache.isEmpty()) return;
		if (dirtyChannels.isEmpty()) return;

		int bytesPerSample = resolveBytesPerSample();
		int frames = resolveFrames(bytesPerSample);
		if (frames <= 0) return;

		java.nio.ByteBuffer dst = directBuffer.duplicate().order(java.nio.ByteOrder.LITTLE_ENDIAN);
		for (Integer channelKey : dirtyChannels.keySet()) {
			int channel = channelKey != null ? channelKey : -1;
			if (channel < 0 || channel >= channels) continue;
			ByteBuffer channelBuffer = channelBufferCache.get(channel);
			if (channelBuffer == null) continue;
			java.nio.ByteBuffer src = channelBuffer.duplicate().order(java.nio.ByteOrder.LITTLE_ENDIAN);
			src.position(0);
			int channelFrames = Math.min(frames, src.capacity() / 4);
			if (bytesPerSample == 4) {
				for (int i = 0; i < channelFrames; i++) {
					float f = src.getFloat(i * 4);
					dst.putFloat((i * channels + channel) * 4, f);
				}
			} else {
				for (int i = 0; i < channelFrames; i++) {
					float f = src.getFloat(i * 4);
					int v = Math.round(f * 32767.0f);
					if (v > Short.MAX_VALUE) v = Short.MAX_VALUE;
					if (v < Short.MIN_VALUE) v = Short.MIN_VALUE;
					dst.putShort((i * channels + channel) * 2, (short) v);
				}
			}
			dirtyChannels.remove(channel);
		}
	}


	public void copyFromChannel(float[] dest, int channel, int startInChannel) {
		if (dest == null) return;
		java.nio.FloatBuffer fb = java.nio.FloatBuffer.wrap(dest);
		copyFromChannel(fb, channel, startInChannel);
	}

	public void copyFromChannel(java.nio.FloatBuffer dest, int channel, int startInChannel) {
		if (dest == null) return;
		int start = Math.max(0, startInChannel);
		ByteBuffer cached = channelBufferCache.get(channel);
		if (cached != null) {
			java.nio.ByteBuffer src = cached.duplicate().order(java.nio.ByteOrder.LITTLE_ENDIAN);
			int frames = src.capacity() / 4;
			int n = Math.min(dest.capacity(), Math.max(0, frames - start));
			for (int i = 0; i < n; i++) {
				dest.put(i, src.getFloat((start + i) * 4));
			}
			return;
		}
		if (directBuffer != null) {
			int byteLen = directBuffer.capacity();
			int bytesPerSample = (byteLen % 4 == 0) ? 4 : 2;
			int frames = (byteLen / bytesPerSample) / channels;
			int n = Math.min(dest.capacity(), Math.max(0, frames - start));
			java.nio.ByteBuffer dup = directBuffer.duplicate().order(java.nio.ByteOrder.LITTLE_ENDIAN);
			if (bytesPerSample == 4) {
				for (int i = 0; i < n; i++) {
					float f = dup.getFloat(((start + i) * channels + channel) * 4);
					dest.put(i, f);
				}
			} else {
				for (int i = 0; i < n; i++) {
					short s = dup.getShort(((start + i) * channels + channel) * 2);
					dest.put(i, (float) s / 32768.0f);
				}
			}
		}
	}

	public void copyFromChannel(java.nio.ByteBuffer dest, int channel, int startInChannel) {
		if (dest == null) return;
		java.nio.FloatBuffer fb = dest.order(java.nio.ByteOrder.LITTLE_ENDIAN).asFloatBuffer();
		copyFromChannel(fb, channel, startInChannel);
	}

	public void copyToChannel(float[] source, int channel, int startInChannel) {
		if (source == null) return;
		int start = Math.max(0, startInChannel);
		if (directBuffer != null && !directBuffer.isReadOnly()) {
			ByteBuffer dup = directBuffer.duplicate().order(java.nio.ByteOrder.LITTLE_ENDIAN);
			int byteLen = directBuffer.capacity();
			int bytesPerSample = (byteLen % 4 == 0) ? 4 : 2;
			int frames = (byteLen / bytesPerSample) / channels;
			int n = Math.min(source.length, Math.max(0, frames - start));
			ByteBuffer channelView = channelBufferCache.get(channel);
			ByteBuffer channelDup = channelView != null ? channelView.duplicate().order(java.nio.ByteOrder.LITTLE_ENDIAN) : null;
			if (bytesPerSample == 4) {
				for (int i = 0; i < n; i++) {
					float value = source[i];
					int byteIndex = ((start + i) * channels + channel) * 4;
					dup.putFloat(byteIndex, value);
					if (channelDup != null) channelDup.putFloat((start + i) * 4, value);
				}
			} else {
				for (int i = 0; i < n; i++) {
					float value = source[i];
					int byteIndex = ((start + i) * channels + channel) * 2;
					int v = Math.round(value * 32767.0f);
					if (v > Short.MAX_VALUE) v = Short.MAX_VALUE;
					if (v < Short.MIN_VALUE) v = Short.MIN_VALUE;
					dup.putShort(byteIndex, (short) v);
					if (channelDup != null) channelDup.putFloat((start + i) * 4, value);
				}
			}
			dirtyChannels.remove(channel);
		}
	}

	public void copyToChannel(java.nio.FloatBuffer source, int channel, int startInChannel) {
		if (source == null) return;
		int start = Math.max(0, startInChannel);
		int srcLen = source.capacity();
		if (directBuffer != null && !directBuffer.isReadOnly()) {
			java.nio.ByteBuffer dup = directBuffer.duplicate().order(java.nio.ByteOrder.LITTLE_ENDIAN);
			int byteLen = directBuffer.capacity();
			int bytesPerSample = (byteLen % 4 == 0) ? 4 : 2;
			int frames = (byteLen / bytesPerSample) / channels;
			int n = Math.min(srcLen, Math.max(0, frames - start));
			ByteBuffer channelView = channelBufferCache.get(channel);
			ByteBuffer channelDup = channelView != null ? channelView.duplicate().order(java.nio.ByteOrder.LITTLE_ENDIAN) : null;
			if (bytesPerSample == 4) {
				for (int i = 0; i < n; i++) {
					float val = source.get(i);
					int byteIndex = ((start + i) * channels + channel) * 4;
					dup.putFloat(byteIndex, val);
					if (channelDup != null) channelDup.putFloat((start + i) * 4, val);
				}
			} else {
				for (int i = 0; i < n; i++) {
					float val = source.get(i);
					int byteIndex = ((start + i) * channels + channel) * 2;
					int v = Math.round(val * 32767.0f);
					if (v > Short.MAX_VALUE) v = Short.MAX_VALUE;
					if (v < Short.MIN_VALUE) v = Short.MIN_VALUE;
					dup.putShort(byteIndex, (short) v);
					if (channelDup != null) channelDup.putFloat((start + i) * 4, val);
				}
			}
			dirtyChannels.remove(channel);
		}
	}

	public void copyToChannel(java.nio.ByteBuffer source, int channel, int startInChannel) {
		if (source == null) return;
		java.nio.FloatBuffer fb = source.order(java.nio.ByteOrder.LITTLE_ENDIAN).asFloatBuffer();
		copyToChannel(fb, channel, startInChannel);
	}
}
