package org.nativescript.audiocontext;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;

import androidx.test.ext.junit.runners.AndroidJUnit4;

import org.junit.After;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;

import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.FloatBuffer;

/**
 * Regression coverage for the AnalyserNode data getters.
 *
 * Historical bug: getByteFrequencyData / getFloatFrequencyData /
 * getByteTimeDomainData / getFloatTimeDomainData appeared to leave the
 * destination buffer untouched. Root causes:
 *   1) JNI used Float-element capacity where ART returns the byte size.
 *   2) Java AnalyserNode mutated the caller's position/limit instead of
 *      operating on a duplicate view.
 *   3) fftSize / smoothingTimeConstant were stored only in JS/Java fields
 *      and never propagated to the native engine.
 *
 * These tests use a sentinel-fill strategy: pre-fill the destination with a
 * value the analyser would never produce, call the getter, and assert that
 * the buffer was actually written into. They also verify the caller's
 * position/limit remain untouched and that boundary cases (tiny / oversized
 * destinations, both direct and heap buffers) all behave.
 */
@RunWith(AndroidJUnit4.class)
public class AnalyserNodeInstrumentedTest {

	private AudioContext instance;
	private AudioContextInstance ctx;
	private AnalyserNode analyser;

	@Before
	public void setUp() {
		instance = AudioContext.getInstance();
		ctx = instance.createContextInstance();
		analyser = instance.createAnalyser(ctx);
		assertNotNull("createAnalyser returned null", analyser);
		assertNotNull("analyser id null", analyser.getId());
	}

	@After
	public void tearDown() {
		if (analyser != null) analyser.release();
		analyser = null;
		ctx = null;
	}

	// --- Helpers -------------------------------------------------------------

	private static FloatBuffer directFloatBuffer(int floats) {
		ByteBuffer bb = ByteBuffer.allocateDirect(floats * 4).order(ByteOrder.nativeOrder());
		return bb.asFloatBuffer();
	}

	private static ByteBuffer directByteBuffer(int bytes) {
		return ByteBuffer.allocateDirect(bytes).order(ByteOrder.nativeOrder());
	}

	/** Fill every slot in [0..limit) with sentinel. */
	private static void fillSentinel(FloatBuffer fb, float sentinel) {
		int basePos = fb.position();
		for (int i = basePos; i < fb.limit(); i++) fb.put(i, sentinel);
	}

	private static void fillSentinel(ByteBuffer bb, byte sentinel) {
		int basePos = bb.position();
		for (int i = basePos; i < bb.limit(); i++) bb.put(i, sentinel);
	}

	/** Returns how many slots in [position..limit) still match the sentinel. */
	private static int countSentinel(FloatBuffer fb, float sentinel) {
		int n = 0;
		for (int i = fb.position(); i < fb.limit(); i++) {
			float v = fb.get(i);
			if (Float.floatToRawIntBits(v) == Float.floatToRawIntBits(sentinel)) n++;
		}
		return n;
	}

	private static int countSentinel(ByteBuffer bb, byte sentinel) {
		int n = 0;
		for (int i = bb.position(); i < bb.limit(); i++) {
			if (bb.get(i) == sentinel) n++;
		}
		return n;
	}

	// --- Bug-regression: dest must be written, not silently skipped ----------

	@Test
	public void getFloatTimeDomainData_writesIntoDirectBuffer() {
		FloatBuffer dest = directFloatBuffer(analyser.getFftSize());
		fillSentinel(dest, Float.NaN);
		int basePos = dest.position();
		int baseLim = dest.limit();

		analyser.getFloatTimeDomainData(dest);

		assertEquals("position changed", basePos, dest.position());
		assertEquals("limit changed", baseLim, dest.limit());
		// Silent input -> all zeros, but every slot must have been overwritten.
		assertEquals("dest not fully written", 0, countSentinel(dest, Float.NaN));
	}

	@Test
	public void getFloatFrequencyData_writesIntoDirectBuffer() {
		FloatBuffer dest = directFloatBuffer(analyser.getFftSize() / 2);
		fillSentinel(dest, Float.NaN);
		int basePos = dest.position();
		int baseLim = dest.limit();

		analyser.getFloatFrequencyData(dest);

		assertEquals("position changed", basePos, dest.position());
		assertEquals("limit changed", baseLim, dest.limit());
		assertEquals("dest not fully written", 0, countSentinel(dest, Float.NaN));
	}

	@Test
	public void getByteTimeDomainData_writesIntoDirectBuffer() {
		ByteBuffer dest = directByteBuffer(analyser.getFftSize());
		fillSentinel(dest, (byte) 0xAA);
		int basePos = dest.position();
		int baseLim = dest.limit();

		analyser.getByteTimeDomainData(dest);

		assertEquals("position changed", basePos, dest.position());
		assertEquals("limit changed", baseLim, dest.limit());
		// Silence -> 0.0 -> quantized to 0x80 (mid-scale). Sentinel 0xAA must
		// not survive in any slot.
		assertEquals("dest not fully written", 0, countSentinel(dest, (byte) 0xAA));
	}

	@Test
	public void getByteFrequencyData_writesIntoDirectBuffer() {
		// The named bug: this getter "does not update the dest passed to it".
		ByteBuffer dest = directByteBuffer(analyser.getFftSize() / 2);
		fillSentinel(dest, (byte) 0xAA);
		int basePos = dest.position();
		int baseLim = dest.limit();

		analyser.getByteFrequencyData(dest);

		assertEquals("position changed", basePos, dest.position());
		assertEquals("limit changed", baseLim, dest.limit());
		assertEquals("dest not fully written", 0, countSentinel(dest, (byte) 0xAA));
	}

	// --- Position / limit on a partial-window slice --------------------------

	@Test
	public void getFloatTimeDomainData_respectsPositionAndLimit() {
		// User-set position=8 and limit=position+64. The getter must only
		// mutate slots [8..72) and leave [0..8) and [72..end) at their sentinel.
		FloatBuffer dest = directFloatBuffer(analyser.getFftSize());
		fillSentinel(dest, Float.NaN);
		dest.position(8);
		dest.limit(8 + 64);

		analyser.getFloatTimeDomainData(dest);

		assertEquals("position moved", 8, dest.position());
		assertEquals("limit moved", 8 + 64, dest.limit());

		// Slots inside the window: must have been written.
		dest.position(8);
		dest.limit(8 + 64);
		assertEquals("window not written", 0, countSentinel(dest, Float.NaN));

		// Slots outside the window: must remain sentinel.
		FloatBuffer outside = dest.duplicate();
		outside.position(0);
		outside.limit(8);
		assertEquals("leading region clobbered", 8, countSentinel(outside, Float.NaN));

		FloatBuffer trailing = dest.duplicate();
		trailing.position(8 + 64);
		trailing.limit(analyser.getFftSize());
		assertEquals("trailing region clobbered",
				analyser.getFftSize() - (8 + 64),
				countSentinel(trailing, Float.NaN));
	}

	@Test
	public void getByteFrequencyData_respectsPositionAndLimit() {
		ByteBuffer dest = directByteBuffer(analyser.getFftSize() / 2);
		fillSentinel(dest, (byte) 0xAA);
		dest.position(4);
		dest.limit(4 + 32);

		analyser.getByteFrequencyData(dest);

		assertEquals("position moved", 4, dest.position());
		assertEquals("limit moved", 4 + 32, dest.limit());

		// Outside-window must still be 0xAA.
		ByteBuffer leading = dest.duplicate();
		leading.position(0);
		leading.limit(4);
		assertEquals("leading region clobbered", 4, countSentinel(leading, (byte) 0xAA));

		ByteBuffer trailing = dest.duplicate();
		trailing.position(4 + 32);
		trailing.limit(analyser.getFftSize() / 2);
		assertEquals("trailing region clobbered",
				(analyser.getFftSize() / 2) - (4 + 32),
				countSentinel(trailing, (byte) 0xAA));
	}

	// --- Heap (non-direct) buffers must work via the fallback ----------------

	@Test
	public void getFloatTimeDomainData_writesIntoHeapBuffer() {
		FloatBuffer dest = FloatBuffer.allocate(256);
		fillSentinel(dest, Float.NaN);

		analyser.getFloatTimeDomainData(dest);

		assertEquals("position moved", 0, dest.position());
		assertEquals("limit moved", 256, dest.limit());
		assertEquals("dest not fully written", 0, countSentinel(dest, Float.NaN));
	}

	@Test
	public void getByteFrequencyData_writesIntoHeapBuffer() {
		ByteBuffer dest = ByteBuffer.allocate(256);
		fillSentinel(dest, (byte) 0xAA);

		analyser.getByteFrequencyData(dest);

		assertEquals("position moved", 0, dest.position());
		assertEquals("limit moved", 256, dest.limit());
		assertEquals("dest not fully written", 0, countSentinel(dest, (byte) 0xAA));
	}

	// --- Boundary cases ------------------------------------------------------

	@Test
	public void getFloatTimeDomainData_zeroLengthIsNoOp() {
		FloatBuffer empty = directFloatBuffer(0);
		// Must not throw.
		analyser.getFloatTimeDomainData(empty);
		analyser.getByteFrequencyData(directByteBuffer(0));
	}

	@Test
	public void getFloatTimeDomainData_nullIsNoOp() {
		// Must not throw.
		analyser.getFloatTimeDomainData(null);
		analyser.getFloatFrequencyData(null);
		analyser.getByteTimeDomainData(null);
		analyser.getByteFrequencyData(null);
	}

	@Test
	public void getFloatTimeDomainData_smallerThanFftSizeFillsAllOfDest() {
		// dest smaller than fftSize: every slot must still be written.
		FloatBuffer dest = directFloatBuffer(64);
		fillSentinel(dest, Float.NaN);

		analyser.getFloatTimeDomainData(dest);

		assertEquals("dest not fully written when smaller than fftSize",
				0, countSentinel(dest, Float.NaN));
	}

	@Test
	public void getFloatTimeDomainData_largerThanFftSizeStillFillsBufferBound() {
		// dest larger than fftSize: at least fftSize slots must be written;
		// the rest are allowed to remain sentinel because the analyser ring
		// only holds fftSize samples.
		int fft = analyser.getFftSize();
		FloatBuffer dest = directFloatBuffer(fft + 64);
		fillSentinel(dest, Float.NaN);

		analyser.getFloatTimeDomainData(dest);

		int unwritten = countSentinel(dest, Float.NaN);
		assertTrue("expected at most " + 64 + " untouched trailing slots, got " + unwritten,
				unwritten <= 64);
	}

	// --- Settings propagate to native ---------------------------------------

	@Test
	public void setFftSize_isReflectedInBinCount() {
		analyser.setFftSize(512);
		assertEquals(512, analyser.getFftSize());
		assertEquals(256, analyser.getFrequencyBinCount());

		analyser.setFftSize(2048);
		assertEquals(2048, analyser.getFftSize());
		assertEquals(1024, analyser.getFrequencyBinCount());
	}

	@Test
	public void setFftSize_clampsAndPowerOfTwoSnaps() {
		// Below min clamps to 32.
		analyser.setFftSize(4);
		assertEquals(32, analyser.getFftSize());
		// Non-power-of-two snaps up.
		analyser.setFftSize(257);
		assertEquals(512, analyser.getFftSize());
		// Above max clamps to 32768.
		analyser.setFftSize(1 << 20);
		assertEquals(32768, analyser.getFftSize());
	}

	@Test
	public void setFftSize_reroutesSubsequentGetters() {
		// After fftSize=512, asking for FloatFrequencyData with bins=256 must
		// fully populate the buffer.
		analyser.setFftSize(512);
		FloatBuffer dest = directFloatBuffer(256);
		fillSentinel(dest, Float.NaN);

		analyser.getFloatFrequencyData(dest);

		assertEquals("dest not fully written after fftSize change",
				0, countSentinel(dest, Float.NaN));
	}

	@Test
	public void setSmoothingTimeConstant_clampsAndStores() {
		analyser.setSmoothingTimeConstant(-0.5);
		assertEquals(0.0, analyser.getSmoothingTimeConstant(), 1e-9);
		analyser.setSmoothingTimeConstant(2.5);
		assertEquals(1.0, analyser.getSmoothingTimeConstant(), 1e-9);
		analyser.setSmoothingTimeConstant(0.7);
		assertEquals(0.7, analyser.getSmoothingTimeConstant(), 1e-9);
	}

	@Test
	public void setDecibels_storesAndDoesNotCrashGetters() {
		analyser.setMinDecibels(-90.0);
		analyser.setMaxDecibels(-20.0);
		assertEquals(-90.0, analyser.getMinDecibels(), 1e-9);
		assertEquals(-20.0, analyser.getMaxDecibels(), 1e-9);

		ByteBuffer dest = directByteBuffer(analyser.getFrequencyBinCount());
		fillSentinel(dest, (byte) 0xAA);
		analyser.getByteFrequencyData(dest);
		assertEquals("dest not written after decibels change",
				0, countSentinel(dest, (byte) 0xAA));
	}

	// --- Repeated calls don't crash and stay bounded ------------------------

	@Test
	public void repeatedGettersAreStable() {
		FloatBuffer fdest = directFloatBuffer(analyser.getFftSize());
		FloatBuffer freq = directFloatBuffer(analyser.getFrequencyBinCount());
		ByteBuffer btime = directByteBuffer(analyser.getFftSize());
		ByteBuffer bfreq = directByteBuffer(analyser.getFrequencyBinCount());

		for (int i = 0; i < 200; i++) {
			analyser.getFloatTimeDomainData(fdest);
			analyser.getFloatFrequencyData(freq);
			analyser.getByteTimeDomainData(btime);
			analyser.getByteFrequencyData(bfreq);
		}

		// Float frequency data is in dB and bounded above by ~0 dB (well, by
		// power 1.0). Below by minDecibels logically, but we just make sure
		// nothing went NaN/Inf.
		freq.position(0);
		while (freq.hasRemaining()) {
			float v = freq.get();
			assertFalse("NaN in frequency data", Float.isNaN(v));
			assertFalse("Inf in frequency data", Float.isInfinite(v));
		}

		// Byte data is range-limited by construction — assert it.
		bfreq.position(0);
		while (bfreq.hasRemaining()) {
			int v = bfreq.get() & 0xFF;
			assertTrue(v >= 0 && v <= 255);
		}
	}
}
