package org.nativescript.audiocontext;

import androidx.annotation.Nullable;

import java.nio.ByteBuffer;
import java.util.UUID;

public class AudioBuffer implements NativeObject {
	final String id;
	private ByteBuffer directBuffer = null;
	private int sampleRate = 48000;
	private int channels = 1;

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

	public java.nio.FloatBuffer getChannelData(int channel) {
		if (channel < 0 || channel >= channels) return null;
		if (directBuffer != null) {
			java.nio.ByteBuffer dup = directBuffer.duplicate().order(java.nio.ByteOrder.LITTLE_ENDIAN);
			return dup.asFloatBuffer();
		}
		return null;
	}


	public void copyFromChannel(float[] dest, int channel, int startInChannel) {
		if (dest == null) return;
		java.nio.FloatBuffer fb = java.nio.FloatBuffer.wrap(dest);
		copyFromChannel(fb, channel, startInChannel);
	}

	public void copyFromChannel(java.nio.FloatBuffer dest, int channel, int startInChannel) {
		if (dest == null) return;
		int start = Math.max(0, startInChannel);
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
			if (bytesPerSample == 4) {
				for (int i = 0; i < n; i++) {
					float val = source.get(i);
					int byteIndex = ((start + i) * channels + channel) * 4;
					dup.putFloat(byteIndex, val);
				}
			} else {
				for (int i = 0; i < n; i++) {
					float val = source.get(i);
					int byteIndex = ((start + i) * channels + channel) * 2;
					int v = Math.round(val * 32767.0f);
					if (v > Short.MAX_VALUE) v = Short.MAX_VALUE;
					if (v < Short.MIN_VALUE) v = Short.MIN_VALUE;
					dup.putShort(byteIndex, (short) v);
				}
			}
		}
	}

	public void copyToChannel(java.nio.ByteBuffer source, int channel, int startInChannel) {
		if (source == null) return;
		java.nio.FloatBuffer fb = source.order(java.nio.ByteOrder.LITTLE_ENDIAN).asFloatBuffer();
		copyToChannel(fb, channel, startInChannel);
	}
}
