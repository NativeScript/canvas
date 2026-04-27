package org.nativescript.audiocontext;

import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;

import androidx.test.ext.junit.runners.AndroidJUnit4;

import org.junit.After;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;

import java.nio.FloatBuffer;

/**
 * Smoke coverage for node lifecycle, connect/disconnect, and graph wiring.
 * Validates that the public Java API can build the typical Web Audio graphs
 * without throwing and that release/disconnect calls are idempotent.
 */
@RunWith(AndroidJUnit4.class)
public class NodeWiringInstrumentedTest {

	private AudioContext instance;
	private AudioContextInstance ctx;

	@Before
	public void setUp() {
		instance = AudioContext.getInstance();
		ctx = instance.createContextInstance();
		assertNotNull(ctx);
		assertNotNull(instance.getDestination(ctx));
	}

	@After
	public void tearDown() {
		ctx = null;
	}

	@Test
	public void gainNode_lifecycle() {
		GainNode g = instance.createGain(ctx);
		assertNotNull(g);
		g.setValue(0.5);
		g.disconnect();
		g.release();
	}

	@Test
	public void biquadNode_paramsRoundTrip() {
		AudioBiquadNode bq = instance.createBiquad(ctx, "highpass", 880.0, 1.4, -3.0);
		assertNotNull(bq);
		bq.setParams("lowpass", 440.0, 0.707, 0.0);
		assertNotNull(bq.getFrequency());
		assertNotNull(bq.getQ());
		assertNotNull(bq.getGain());
		assertNotNull(bq.getDetune());
		bq.disconnect();
		bq.release();
	}

	@Test
	public void analyser_connectsToGainAndDestination_withoutCrash() {
		GainNode g = instance.createGain(ctx);
		AnalyserNode a = instance.createAnalyser(ctx);

		// source(g) -> analyser -> destination
		g.connect(a);
		a.connect(instance.getDestination(ctx));

		// Disconnect both ends — must be safe.
		a.disconnect();
		g.disconnect();

		a.release();
		g.release();
	}

	@Test
	public void delayNode_paramAccess() {
		DelayNode d = instance.createDelay(ctx);
		assertNotNull(d);
		assertNotNull(d.getDelayTime());
		d.getDelayTime().setValue(0.25);
		d.disconnect();
		d.release();
	}

	@Test
	public void constantSource_offsetParamAccess() {
		ConstantSourceNode c = instance.createConstantSource(ctx);
		assertNotNull(c);
		c.getOffsetParam().setValue(0.5);
		c.start();
		c.stop();
		c.disconnect();
		c.release();
	}

	@Test
	public void oscillator_lifecycle() {
		AudioOscillatorNode osc = instance.createOscillator(ctx, "sine", 440.0);
		assertNotNull(osc);
		assertNotNull(osc.getFrequency());
		osc.start();
		osc.stop();
		osc.disconnect();
		osc.release();
	}

	@Test
	public void waveShaper_curveAndOversample() {
		WaveShaperNode ws = instance.createWaveShaper(ctx);
		assertNotNull(ws);
		float[] curve = new float[1024];
		for (int i = 0; i < curve.length; i++) {
			float x = (i / 511.5f) - 1.0f;
			curve[i] = (float) Math.tanh(2.0 * x);
		}
		ws.setCurveFromData(FloatBuffer.wrap(curve));
		ws.setOversample("4x");
		ws.disconnect();
		ws.release();
	}

	@Test
	public void iirFilter_lifecycle() {
		double[] ff = new double[]{0.5, 0.5};
		double[] fb = new double[]{1.0, -0.5};
		AudioIIRNode iir = instance.createIIR(ctx, ff, fb);
		assertNotNull(iir);

		float[] freqs = new float[]{100, 440, 1000, 5000};
		float[] mag = new float[freqs.length];
		float[] phase = new float[freqs.length];
		iir.getFrequencyResponse(freqs, mag, phase);
		// Magnitudes must be finite.
		for (float v : mag) {
			assertTrue("non-finite IIR magnitude", !Float.isNaN(v) && !Float.isInfinite(v));
		}
		iir.disconnect();
		iir.release();
	}

	@Test
	public void periodicWave_constructFromArrays() {
		double[] real = new double[]{0.0, 1.0};
		double[] imag = new double[]{0.0, 0.0};
		PeriodicWave pw = instance.createPeriodicWave(ctx, real, imag, false);
		assertNotNull(pw);
		pw.release();
	}

	@Test
	public void convolver_setBuffer() {
		ConvolverNode conv = instance.createConvolver(ctx);
		assertNotNull(conv);

		AudioBuffer ir = new AudioBuffer(256, 1, 48000);
		// Simple impulse response: single tap at frame 0.
		float[] data = new float[256];
		data[0] = 1.0f;
		ir.copyToChannel(data, 0, 0);
		conv.setBuffer(ir);
		conv.setNormalize(true);
		assertTrue(conv.getNormalize());

		conv.disconnect();
		conv.release();
	}

	@Test
	public void disconnect_isIdempotent() {
		GainNode g = instance.createGain(ctx);
		// Multiple disconnect calls must be a no-op, not throw.
		g.disconnect();
		g.disconnect();
		g.disconnect();
		g.release();
		// Release after release should also be safe.
		g.release();
	}

	@Test
	public void chain_oscillator_through_filter_and_gain_to_destination() {
		AudioOscillatorNode osc = instance.createOscillator(ctx, "sawtooth", 220.0);
		AudioBiquadNode bq = instance.createBiquad(ctx, "lowpass", 1200.0, 0.707, 0.0);
		GainNode g = instance.createGain(ctx);
		GainNode dest = instance.getDestination(ctx);

		osc.connect(bq);
		bq.connect(g);
		g.connect(dest);

		osc.start();
		try { Thread.sleep(50); } catch (InterruptedException ignored) {}
		osc.stop();

		g.disconnect();
		bq.disconnect();
		osc.disconnect();

		g.release();
		bq.release();
		osc.release();
	}
}
