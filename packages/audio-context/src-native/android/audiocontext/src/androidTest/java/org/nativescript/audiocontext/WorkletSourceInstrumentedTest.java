package org.nativescript.audiocontext;

import androidx.test.ext.junit.runners.AndroidJUnit4;
import androidx.test.filters.SdkSuppress;

import org.junit.Assert;
import org.junit.Test;
import org.junit.runner.RunWith;

import java.lang.invoke.VarHandle;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.FloatBuffer;

/** A worklet voice over a ring served by a Java stand-in for the Worker. */
@RunWith(AndroidJUnit4.class)
@SdkSuppress(minSdkVersion = 33)
public class WorkletSourceInstrumentedTest {
	// workletRingLayout(0, [1], 1) in audio-worklet-ring.ts.
	private static final int SLOTS = 64;
	private static final int OUTPUT_COUNTS = 4;
	private static final int PARAM_LENGTHS = 5;
	private static final int OUTPUT_DATA = 8;
	private static final int PARAM_DATA = OUTPUT_DATA + 128;
	private static final int SLOT_WORDS = PARAM_DATA + 128;
	private static final int HEADER = SLOT_WORDS * SLOTS;

	private volatile boolean serving = true;

	private void serve(ByteBuffer ring) {
		while (serving) {
			int requested = ring.getInt(HEADER * 4);
			int done = ring.getInt((HEADER + 1) * 4);
			VarHandle.acquireFence();
			for (int k = done; k != requested; k++) {
				int slot = Integer.remainderUnsigned(k, SLOTS) * SLOT_WORDS * 4;
				float level = ring.getFloat(slot + PARAM_DATA * 4);
				for (int i = 0; i < 128; i++) ring.putFloat(slot + (OUTPUT_DATA + i) * 4, level);
				ring.putInt(slot + 2 * 4, 1);
			}
			VarHandle.releaseFence();
			ring.putInt((HEADER + 1) * 4, requested);
			try {
				Thread.sleep(1);
			} catch (InterruptedException e) {
				return;
			}
		}
	}

	private static float peak(AnalyserNode analyser) {
		FloatBuffer data = ByteBuffer.allocateDirect(analyser.getFftSize() * 4).order(ByteOrder.nativeOrder()).asFloatBuffer();
		analyser.getFloatTimeDomainData(data);
		float peak = 0;
		for (int i = 0; i < data.capacity(); i++) peak = Math.max(peak, Math.abs(data.get(i)));
		return peak;
	}

	@Test
	public void playsQuantaFromTheRingWithParamValues() throws Exception {
		AudioContext audio = AudioContext.getInstance();
		AudioContextInstance context = audio.createContextInstance(48000, 0.01);
		AnalyserNode analyser = audio.createAnalyser(context);
		ByteBuffer ring = ByteBuffer.allocateDirect((HEADER + 8) * 4).order(ByteOrder.LITTLE_ENDIAN);
		WorkletNode node = audio.createWorklet(context, ring, new int[0], new int[]{1}, new int[]{SLOT_WORDS, HEADER, OUTPUT_COUNTS, OUTPUT_DATA, OUTPUT_COUNTS, PARAM_LENGTHS, PARAM_DATA}, new int[]{OUTPUT_DATA}, new double[]{0.25});
		Assert.assertNotNull(node);
		node.connect(analyser);
		Assert.assertEquals("native marks the ring it reads", 0x574B4C54, ring.getInt((HEADER + 5) * 4));
		Assert.assertEquals(1, node.getParamCount());

		Thread worker = new Thread(() -> serve(ring));
		worker.start();
		try {
			Thread.sleep(600);
			Assert.assertEquals(0.25f, peak(analyser), 0.01f);
			Assert.assertTrue("requests keep coming", ring.getInt(HEADER * 4) > 100);
			node.getParam(0).setValue(0.5);
			Thread.sleep(400);
			Assert.assertEquals(0.5f, peak(analyser), 0.01f);
			node.stop();
			Thread.sleep(200);
			Assert.assertEquals("closed once the voice is gone", 1, ring.getInt((HEADER + 3) * 4));
		} finally {
			serving = false;
			worker.join();
		}
	}
}
