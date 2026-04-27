package org.nativescript.audiocontext;

import org.junit.Assert;
import org.junit.Before;
import org.junit.Test;

import java.lang.reflect.Field;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.FloatBuffer;

public class WaveShaperCurveTest {

    @Before
    public void setUp() throws Exception {
        // Ensure we have an AudioContext instance for tests
        Field sInstance = AudioContext.class.getDeclaredField("sInstance");
        sInstance.setAccessible(true);
        if (sInstance.get(null) == null) {
            sInstance.set(null, new AudioContext());
        }
    }

    @Test
    public void testSetWaveShaperCurveFromByteBufferFallbackStoresArray() {
        AudioContext ctx = AudioContext.getInstance();
        String id = "ws-test";

        float[] input = new float[] {0.1f, 0.5f, -0.3f};
        ByteBuffer bb = ByteBuffer.allocate(input.length * 4).order(ByteOrder.LITTLE_ENDIAN);
        FloatBuffer fb = bb.asFloatBuffer();
        for (int i = 0; i < input.length; ++i) fb.put(i, input[i]);

        // Call the ByteBuffer overload (non-direct) which should store an array fallback
        ctx.setWaveShaperCurveFromData(id, bb);

        float[] out = ctx.getWaveShaperCurve(id);
        Assert.assertNotNull(out);
        Assert.assertEquals(input.length, out.length);
        for (int i = 0; i < input.length; ++i) {
            Assert.assertEquals(input[i], out[i], 1e-6f);
        }
    }
}
