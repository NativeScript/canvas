package org.nativescript.audiocontext;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertNull;
import static org.junit.Assert.assertTrue;

import androidx.test.ext.junit.runners.AndroidJUnit4;

import org.junit.Test;
import org.junit.runner.RunWith;

import java.nio.ByteBuffer;
import java.nio.FloatBuffer;

@RunWith(AndroidJUnit4.class)
public class AudioBufferInstrumentedTest {

	@Test
	public void construction_storesProperties() {
		AudioBuffer buf = new AudioBuffer(1024, 2, 44100);
		assertEquals(1024, buf.getLength());
		assertEquals(2, buf.getNumberOfChannels());
		assertEquals(44100, buf.getSampleRate());
		assertEquals(1024.0 / 44100.0, buf.getDuration(), 1e-9);
		buf.release();
	}

	@Test
	public void construction_clampsZeroOrNegativeChannels() {
		AudioBuffer buf = new AudioBuffer(64, 0, 48000);
		// Implementation clamps channels to >= 1.
		assertTrue(buf.getNumberOfChannels() >= 1);
		buf.release();
	}

	@Test
	public void copyToChannel_roundTrip_fromFloatArray() {
		AudioBuffer buf = new AudioBuffer(256, 1, 48000);
		float[] src = new float[256];
		for (int i = 0; i < src.length; i++) {
			src[i] = (float) Math.sin(i * 0.1);
		}
		buf.copyToChannel(src, 0, 0);

		float[] dst = new float[256];
		buf.copyFromChannel(dst, 0, 0);
		for (int i = 0; i < 256; i++) {
			assertEquals("sample " + i + " round-trip mismatch", src[i], dst[i], 1e-6f);
		}
		buf.release();
	}

	@Test
	public void copyToChannel_roundTrip_fromFloatBuffer() {
		AudioBuffer buf = new AudioBuffer(128, 1, 48000);
		FloatBuffer src = FloatBuffer.allocate(128);
		for (int i = 0; i < 128; i++) src.put(i, i * 0.01f);
		buf.copyToChannel(src, 0, 0);

		FloatBuffer dst = FloatBuffer.allocate(128);
		buf.copyFromChannel(dst, 0, 0);
		for (int i = 0; i < 128; i++) {
			assertEquals(src.get(i), dst.get(i), 1e-6f);
		}
		buf.release();
	}

	@Test
	public void copyToChannel_offsetIsRespected() {
		AudioBuffer buf = new AudioBuffer(256, 1, 48000);
		float[] src = new float[64];
		for (int i = 0; i < 64; i++) src[i] = i + 1.0f;
		// Write at startInChannel = 100.
		buf.copyToChannel(src, 0, 100);

		float[] dst = new float[256];
		buf.copyFromChannel(dst, 0, 0);

		// Slots [0..100) should still be 0.
		for (int i = 0; i < 100; i++) assertEquals("pre-offset slot non-zero", 0.0f, dst[i], 1e-6f);
		// Slots [100..164) should match src.
		for (int i = 0; i < 64; i++) assertEquals("data slot mismatch", src[i], dst[100 + i], 1e-6f);
		// Slots [164..) should still be 0.
		for (int i = 164; i < 256; i++) assertEquals("post-data slot non-zero", 0.0f, dst[i], 1e-6f);
		buf.release();
	}

	@Test
	public void copyToChannel_independentChannels() {
		AudioBuffer buf = new AudioBuffer(64, 2, 48000);
		float[] left = new float[64];
		float[] right = new float[64];
		for (int i = 0; i < 64; i++) {
			left[i] = i;
			right[i] = -i;
		}
		buf.copyToChannel(left, 0, 0);
		buf.copyToChannel(right, 1, 0);

		float[] outL = new float[64];
		float[] outR = new float[64];
		buf.copyFromChannel(outL, 0, 0);
		buf.copyFromChannel(outR, 1, 0);

		for (int i = 0; i < 64; i++) {
			assertEquals(left[i], outL[i], 1e-6f);
			assertEquals(right[i], outR[i], 1e-6f);
		}
		buf.release();
	}

	@Test
	public void getChannelData_returnsBufferAtCorrectLength() {
		AudioBuffer buf = new AudioBuffer(128, 1, 48000);
		FloatBuffer view = buf.getChannelData(0);
		assertNotNull("getChannelData(0) returned null", view);
		// Capacity should be at least the requested length.
		assertTrue(view.capacity() >= 128);
		buf.release();
	}

	@Test
	public void getChannelData_outOfRangeReturnsNull() {
		AudioBuffer buf = new AudioBuffer(64, 1, 48000);
		assertNull(buf.getChannelData(-1));
		assertNull(buf.getChannelData(7));
		buf.release();
	}

	@Test
	public void copyFromChannel_nullDestIsNoOp() {
		AudioBuffer buf = new AudioBuffer(8, 1, 48000);
		buf.copyFromChannel((float[]) null, 0, 0);
		buf.copyFromChannel((FloatBuffer) null, 0, 0);
		buf.copyFromChannel((ByteBuffer) null, 0, 0);
		buf.release();
	}

	@Test
	public void copyToChannel_nullSourceIsNoOp() {
		AudioBuffer buf = new AudioBuffer(8, 1, 48000);
		buf.copyToChannel((float[]) null, 0, 0);
		buf.copyToChannel((FloatBuffer) null, 0, 0);
		buf.copyToChannel((ByteBuffer) null, 0, 0);
		buf.release();
	}
}
