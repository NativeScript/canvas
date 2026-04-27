package org.nativescript.audiocontext;

import org.junit.Assert;
import org.junit.Before;
import org.junit.Test;

import java.lang.reflect.Field;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.FloatBuffer;

public class AnalyserNodeTest {

    static class TestAudioContext extends AudioContext {
        @Override
        public boolean getAnalyserFloatTimeDomainDataDirect(String id, FloatBuffer dest) {
            // dest is expected to be a sliced view (position 0, capacity == window length)
            for (int i = 0; i < dest.capacity(); ++i) dest.put(i, (float) (i + 1));
            return true;
        }

        @Override
        public boolean getAnalyserByteFrequencyDataDirect(String id, ByteBuffer dest, float minDecibels, float maxDecibels) {
            for (int i = 0; i < dest.capacity(); ++i) dest.put(i, (byte) (i + 1));
            return true;
        }
    }

    @Before
    public void setUp() throws Exception {
        // Replace the singleton AudioContext instance with our test subclass
        TestAudioContext ctx = new TestAudioContext();
        Field sInstance = AudioContext.class.getDeclaredField("sInstance");
        sInstance.setAccessible(true);
        sInstance.set(null, ctx);
    }

    @Test
    public void testGetFloatTimeDomainData_directSliceWritesWindow() {
        final int capacity = 10;
        ByteBuffer bb = ByteBuffer.allocateDirect(capacity * 4).order(ByteOrder.LITTLE_ENDIAN);
        FloatBuffer fb = bb.asFloatBuffer();
        for (int i = 0; i < capacity; ++i) fb.put(i, -1.0f);

        // Set a window in the caller buffer
        fb.position(2);
        fb.limit(6);

        AnalyserNode node = new AnalyserNode("test");
        node.getFloatTimeDomainData(fb);

        // The TestAudioContext writes 1,2,3,4 into the slice (length 4)
        for (int i = 2; i < 6; ++i) {
            float expected = (float) (i - 2 + 1);
            Assert.assertEquals("value at " + i, expected, fb.get(i), 1e-6f);
        }
    }

    @Test
    public void testGetByteFrequencyData_directSliceWritesWindow() {
        final int capacity = 12;
        ByteBuffer bb = ByteBuffer.allocateDirect(capacity).order(ByteOrder.LITTLE_ENDIAN);
        for (int i = 0; i < capacity; ++i) bb.put(i, (byte) 0x7F);

        // Use a subwindow
        bb.position(1);
        bb.limit(5);

        AnalyserNode node = new AnalyserNode("test");
        node.getByteFrequencyData(bb);

        for (int i = 1; i < 5; ++i) {
            byte expected = (byte) (i - 1 + 1);
            Assert.assertEquals("byte at " + i, expected, bb.get(i));
        }
    }
}
