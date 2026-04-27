package org.nativescript.audiocontext;

import androidx.test.ext.junit.runners.AndroidJUnit4;
import org.junit.Assert;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;

import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.FloatBuffer;

@RunWith(AndroidJUnit4.class)
public class AnalyserInstrumentedTest {

    @Before
    public void setUp() {
        // Ensure singleton and native libs are initialized on device
        AudioContext.getInstance();
    }

    @Test
    public void analyserFloatDirect_respectsPositionAndLimit() {
        AudioContextInstance ctx = new AudioContextInstance();
        AnalyserNode node = AudioContext.getInstance().createAnalyser(ctx);

        final int capacity = 16;
        ByteBuffer bb = ByteBuffer.allocateDirect(capacity * 4).order(ByteOrder.LITTLE_ENDIAN);
        FloatBuffer fb = bb.asFloatBuffer();
        float sentinel = -12345.0f;
        for (int i = 0; i < capacity; ++i) fb.put(i, sentinel);

        // Create a duplicate for validation that retains full capacity/limit
        FloatBuffer checkFb = fb.duplicate();
        checkFb.position(0);
        checkFb.limit(checkFb.capacity());

        int pos = 3;
        int lim = 10;
        fb.position(pos);
        fb.limit(lim);

        node.getFloatTimeDomainData(fb);

        for (int i = 0; i < pos; ++i) {
            Assert.assertEquals("pre-window unchanged at " + i, sentinel, checkFb.get(i), 1e-6f);
        }
        for (int i = lim; i < capacity; ++i) {
            Assert.assertEquals("post-window unchanged at " + i, sentinel, checkFb.get(i), 1e-6f);
        }
    }

    @Test
    public void analyserByteDirect_respectsPositionAndLimit() {
        AudioContextInstance ctx = new AudioContextInstance();
        AnalyserNode node = AudioContext.getInstance().createAnalyser(ctx);

        final int capacity = 20;
        ByteBuffer bb = ByteBuffer.allocateDirect(capacity).order(ByteOrder.LITTLE_ENDIAN);
        byte sentinel = (byte) 0x7F;
        for (int i = 0; i < capacity; ++i) bb.put(i, sentinel);

        // Duplicate for validation so we can read absolute indices regardless of test window
        ByteBuffer checkBb = bb.duplicate().order(ByteOrder.LITTLE_ENDIAN);
        checkBb.position(0);
        checkBb.limit(checkBb.capacity());

        int pos = 2;
        int lim = 8;
        bb.position(pos);
        bb.limit(lim);

        node.getByteFrequencyData(bb);

        for (int i = 0; i < pos; ++i) {
            Assert.assertEquals("pre-window unchanged at " + i, sentinel, checkBb.get(i));
        }
        for (int i = lim; i < capacity; ++i) {
            Assert.assertEquals("post-window unchanged at " + i, sentinel, checkBb.get(i));
        }
    }
}
