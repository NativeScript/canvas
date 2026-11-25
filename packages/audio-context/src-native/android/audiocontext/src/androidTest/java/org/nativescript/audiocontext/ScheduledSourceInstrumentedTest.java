package org.nativescript.audiocontext;

import static org.junit.Assert.assertEquals;
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
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;

// The analyser sees the whole output mix, so its RMS tells whether the scheduled voice sounds.
@RunWith(AndroidJUnit4.class)
public class ScheduledSourceInstrumentedTest {

	private AudioContext instance;
	private AudioContextInstance ctx;
	private AnalyserNode analyser;

	@Before
	public void setUp() {
		instance = AudioContext.getInstance();
		ctx = instance.createContextInstance();
		ctx.resumeAsync(null);
		analyser = instance.createAnalyser(ctx);
		assertNotNull(analyser);
		analyser.setFftSize(512);
	}

	@After
	public void tearDown() {
		if (analyser != null) analyser.release();
		if (ctx != null) ctx.release();
	}

	private float rms() {
		FloatBuffer fb = ByteBuffer.allocateDirect(512 * 4).order(ByteOrder.LITTLE_ENDIAN).asFloatBuffer();
		analyser.getFloatTimeDomainData(fb);
		double sum = 0;
		for (int i = 0; i < 512; i++) {
			float v = fb.get(i);
			sum += v * v;
		}
		return (float) Math.sqrt(sum / 512);
	}

	private void waitUntil(double contextTime) throws InterruptedException {
		long deadline = System.nanoTime() + TimeUnit.SECONDS.toNanos(5);
		while (ctx.getCurrentTime() < contextTime) {
			if (System.nanoTime() > deadline) throw new AssertionError("context clock stalled at " + ctx.getCurrentTime());
			Thread.sleep(5);
		}
	}

	private void waitForClock() throws InterruptedException {
		long deadline = System.nanoTime() + TimeUnit.SECONDS.toNanos(5);
		while (ctx.getCurrentTime() <= 0.0) {
			if (System.nanoTime() > deadline) throw new AssertionError("context clock never started");
			Thread.sleep(5);
		}
	}

	@Test
	public void oscillatorStartAtStopAt_soundsOnlyInsideTheWindow() throws Exception {
		AudioOscillatorNode osc = ctx.createOscillatorNodeFrequency("square", 440.0);
		assertNotNull(osc);
		osc.connect(instance.getDestination(ctx));
		CountDownLatch ended = new CountDownLatch(1);
		osc.addEndedListener(id -> ended.countDown());

		osc.start();
		osc.stop();
		waitForClock();
		assertTrue("immediate stop fires ended", ended.await(1, TimeUnit.SECONDS));

		AudioOscillatorNode scheduled = ctx.createOscillatorNodeFrequency("square", 440.0);
		scheduled.connect(instance.getDestination(ctx));
		CountDownLatch scheduledEnded = new CountDownLatch(1);
		scheduled.addEndedListener(id -> scheduledEnded.countDown());

		double t0 = ctx.getCurrentTime();
		scheduled.startAt(t0 + 0.5);
		scheduled.stopAt(t0 + 1.0);

		waitUntil(t0 + 0.35);
		float before = rms();
		waitUntil(t0 + 0.75);
		float during = rms();
		assertEquals("ended must not fire before stop time", 1, scheduledEnded.getCount());
		waitUntil(t0 + 1.3);
		float after = rms();

		assertTrue("silent before startAt, rms=" + before, before < 0.01f);
		assertTrue("audible between startAt and stopAt, rms=" + during, during > 0.5f);
		assertTrue("silent after stopAt, rms=" + after, after < 0.01f);
		assertTrue("ended fires after a scheduled stop", scheduledEnded.await(1, TimeUnit.SECONDS));
	}

	@Test
	public void setTypeChangesARunningOscillator() throws Exception {
		AudioOscillatorNode osc = ctx.createOscillatorNodeFrequency("sine", 440.0);
		osc.connect(instance.getDestination(ctx));
		osc.start();
		waitForClock();
		double t0 = ctx.getCurrentTime();
		waitUntil(t0 + 0.2);
		float sine = rms();
		osc.setType("square");
		assertEquals("square", osc.getType());
		waitUntil(t0 + 0.45);
		float square = rms();
		osc.stop();

		// A full-scale sine has rms 1/sqrt(2); a square has rms 1.
		assertEquals("sine rms", 0.707f, sine, 0.08f);
		assertEquals("square rms", 1.0f, square, 0.08f);
	}

	@Test
	public void bufferSourceStartAt_waitsForTheScheduledTime() throws Exception {
		int sampleRate = 48000;
		int frames = sampleRate / 2;
		float[] pcm = new float[frames];
		for (int i = 0; i < frames; i++) pcm[i] = (i / 50) % 2 == 0 ? 1.0f : -1.0f;
		AudioBuffer buffer = new AudioBuffer(frames, 1, sampleRate);
		buffer.copyToChannel(pcm, 0, 0);

		AudioBufferSourceNode src = instance.createBufferSource(ctx, buffer);
		src.connect(instance.getDestination(ctx));
		CountDownLatch ended = new CountDownLatch(1);
		src.addEndedListener(id -> ended.countDown());

		waitForClock();
		double t0 = ctx.getCurrentTime();
		src.startAt(t0 + 0.4);

		waitUntil(t0 + 0.3);
		float before = rms();
		waitUntil(t0 + 0.6);
		float during = rms();
		// The buffer is 0.5 s long, so it ends on its own at about t0 + 0.9.
		assertTrue("ended fires when the buffer runs out", ended.await(2, TimeUnit.SECONDS));
		// Let the analyser window move past the buffer's last frames.
		waitUntil(ctx.getCurrentTime() + 0.05);
		float after = rms();

		assertTrue("silent before startAt, rms=" + before, before < 0.01f);
		assertTrue("audible after startAt, rms=" + during, during > 0.5f);
		assertTrue("silent after the buffer ends, rms=" + after, after < 0.01f);
	}

	@Test
	public void bufferSourceOffset_skipsTheStartOfTheBuffer() throws Exception {
		int sampleRate = 48000;
		int frames = sampleRate;
		float[] pcm = new float[frames];
		// Silent first half, full-scale square second half.
		for (int i = frames / 2; i < frames; i++) pcm[i] = (i / 50) % 2 == 0 ? 1.0f : -1.0f;
		AudioBuffer buffer = new AudioBuffer(frames, 1, sampleRate);
		buffer.copyToChannel(pcm, 0, 0);

		AudioBufferSourceNode src = instance.createBufferSource(ctx, buffer);
		src.connect(instance.getDestination(ctx));
		waitForClock();
		double t0 = ctx.getCurrentTime();
		src.startAt(0, 0.5);
		waitUntil(t0 + 0.2);
		float rms = rms();
		src.stop();

		assertTrue("offset 0.5 s starts in the audible half, rms=" + rms, rms > 0.5f);
	}

	@Test
	public void constantSource_outputsItsOffset() throws Exception {
		ConstantSourceNode src = instance.createConstantSource(ctx);
		src.getOffsetParam().setValue(0.5);
		src.connect(instance.getDestination(ctx));
		waitForClock();
		double t0 = ctx.getCurrentTime();
		src.start();
		waitUntil(t0 + 0.2);
		float rms = rms();
		src.stop();

		assertEquals("constant 0.5 has rms 0.5", 0.5f, rms, 0.05f);
	}
}
