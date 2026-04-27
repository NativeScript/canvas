package org.nativescript.audiocontext;

import static org.junit.Assert.assertNotNull;

import android.content.Context;

import androidx.test.core.app.ApplicationProvider;
import androidx.test.ext.junit.runners.AndroidJUnit4;

import org.junit.Test;
import org.junit.runner.RunWith;

import java.io.File;
import java.io.FileOutputStream;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;

@RunWith(AndroidJUnit4.class)
public class AudioContextInstrumentedTest {

	private static byte[] generateWav(int sampleRate, double freq, float duration) {
		try {
			int numSamples = (int) (sampleRate * duration);
			int bytesPerSample = 2;
			int dataSize = numSamples * bytesPerSample;
			ByteBuffer bb = ByteBuffer.allocate(44 + dataSize).order(ByteOrder.LITTLE_ENDIAN);
			bb.put("RIFF".getBytes("US-ASCII"));
			bb.putInt(36 + dataSize);
			bb.put("WAVE".getBytes("US-ASCII"));
			bb.put("fmt ".getBytes("US-ASCII"));
			bb.putInt(16);
			bb.putShort((short) 1);
			bb.putShort((short) 1);
			bb.putInt(sampleRate);
			bb.putInt(sampleRate * bytesPerSample);
			bb.putShort((short) (bytesPerSample));
			bb.putShort((short) 16);
			bb.put("data".getBytes("US-ASCII"));
			bb.putInt(dataSize);
			for (int i = 0; i < numSamples; i++) {
				double t = i / (double) sampleRate;
				short val = (short) (Math.sin(2.0 * Math.PI * freq * t) * Short.MAX_VALUE);
				bb.putShort(val);
			}
			return bb.array();
		} catch (Exception e) {
			throw new RuntimeException(e);
		}
	}

	@Test
	public void decodeFromByteArray_returnsId() {
		byte[] wav = generateWav(8000, 440.0, 0.2f);
		AudioContext ctx = AudioContext.getInstance();
		AudioBuffer id = ctx.decodeAudioDataFromByteArray(wav);
		assertNotNull("decodeAudioDataFromByteArray returned null id", id);
		ctx = null;
	}

	@Test
	public void decodeFromByteBuffer_returnsId() {
		byte[] wav = generateWav(8000, 440.0, 0.2f);
		ByteBuffer buf = ByteBuffer.allocateDirect(wav.length).order(ByteOrder.LITTLE_ENDIAN);
		buf.put(wav);
		buf.position(0);
		AudioContext ctx = AudioContext.getInstance();
		AudioBuffer id = ctx.decodeAudioDataFromBuffer(buf);
		assertNotNull("decodeAudioDataFromBuffer returned null id", id);
		id = null;
	}

	@Test
	public void decodeFromFile_and_createStartStop() throws Exception {
		byte[] wav = generateWav(8000, 440.0, 0.2f);
		Context appContext = ApplicationProvider.getApplicationContext();
		File cache = appContext.getCacheDir();
		File f = new File(cache, "test_audio_context.wav");
		try (FileOutputStream fos = new FileOutputStream(f)) {
			fos.write(wav);
			fos.flush();
		}
		AudioContext instance = AudioContext.getInstance();
		AudioContextInstance ctx = instance.createContextInstance();
		AudioBuffer id = instance.decodeAudioDataFromFile(f.getAbsolutePath());
		assertNotNull("decodeAudioDataFromFile returned null id", id);

		AudioBufferSourceNode trackId = instance.createBufferSource(ctx, id);
		assertNotNull("createBufferSource returned null trackId", trackId);
		instance.startBufferSource(trackId.getId(), false);
		Thread.sleep(200);
		instance.stopTrack(trackId.getId());
		id = null;
	}

}
