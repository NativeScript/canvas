package org.nativescript.audiocontext;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;
import static org.junit.Assert.assertFalse;

import androidx.test.ext.junit.runners.AndroidJUnit4;

import org.junit.After;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;

@RunWith(AndroidJUnit4.class)
public class AudioParamInstrumentedTest {

	private AudioContext instance;
	private AudioContextInstance ctx;

	@Before
	public void setUp() {
		instance = AudioContext.getInstance();
		ctx = instance.createContextInstance();
	}

	@After
	public void tearDown() {
		ctx = null;
	}

	@Test
	public void gainParam_setValue_isReadBack() {
		GainNode g = instance.createGain(ctx);
		AudioParam gain = g.getGain();
		assertNotNull(gain);

		gain.setValue(0.25);
		assertEquals(0.25, gain.getValue(), 1e-6);

		gain.setValue(1.0);
		assertEquals(1.0, gain.getValue(), 1e-6);
		g.release();
	}

	@Test
	public void gainParam_automationRate_roundTripsAsString() {
		GainNode g = instance.createGain(ctx);
		AudioParam gain = g.getGain();
		gain.setAutomationRate("k-rate");
		assertEquals("k-rate", gain.getAutomationRate());
		assertEquals(0, gain.getAutomationRateCode());

		gain.setAutomationRate("a-rate");
		assertEquals("a-rate", gain.getAutomationRate());
		assertEquals(1, gain.getAutomationRateCode());

		// Unknown rate falls back to a-rate.
		gain.setAutomationRate("garbage");
		assertEquals("a-rate", gain.getAutomationRate());
		g.release();
	}

	@Test
	public void gainParam_setValueAtTimeImmediate_appliesNow() {
		GainNode g = instance.createGain(ctx);
		AudioParam gain = g.getGain();
		// t <= 0 should apply immediately (per the implementation).
		gain.setValueAtTime(0.5, 0.0);
		assertEquals(0.5, gain.getValue(), 1e-6);
		g.release();
	}

	@Test
	public void gainParam_getValuesForRange_returnsRequestedFrameCount() {
		GainNode g = instance.createGain(ctx);
		AudioParam gain = g.getGain();
		gain.setValue(0.75);

		double[] values = gain.getValuesForRange(0.0, 48000.0, 64);
		assertNotNull(values);
		assertEquals(64, values.length);
		// All values should be finite.
		for (double v : values) {
			assertFalse("NaN in param values", Double.isNaN(v));
			assertFalse("Inf in param values", Double.isInfinite(v));
		}
		g.release();
	}

	@Test
	public void biquadFrequency_setValue_isReadBack() {
		AudioBiquadNode bq = instance.createBiquad(ctx, "lowpass", 1000.0, 1.0, 0.0);
		AudioParam freq = bq.getFrequency();
		freq.setValue(880.0);
		assertEquals(880.0, freq.getValue(), 1e-3);
	}

	@Test
	public void biquadQ_setValue_isReadBack() {
		AudioBiquadNode bq = instance.createBiquad(ctx, "lowpass", 1000.0, 1.0, 0.0);
		AudioParam q = bq.getQ();
		q.setValue(2.5);
		assertEquals(2.5, q.getValue(), 1e-6);
	}

	@Test
	public void pannerPan_setValue_isReadBack() {
		AudioPannerNode pan = instance.createPanner(ctx);
		AudioParam pParam = new AudioParam(pan, AudioParam.Type.PANNER_PAN);
		pParam.setValue(0.4);
		assertEquals(0.4, pParam.getValue(), 1e-6);
	}
}
