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
public class WaveShaperInstrumentedTest {

    @Before
    public void setUp() {
        AudioContext.getInstance();
    }

    @Test
    public void setWaveShaperCurve_directStoresAndRetrieves() {
        AudioContext ctx = AudioContext.getInstance();
        String id = "ws-instr-test";
        final float[] vals = new float[] {0.1f, 0.5f, -0.4f};
        ByteBuffer bb = ByteBuffer.allocateDirect(vals.length * 4).order(ByteOrder.LITTLE_ENDIAN);
        FloatBuffer fb = bb.asFloatBuffer();
        for (int i = 0; i < vals.length; ++i) fb.put(i, vals[i]);

        ctx.setWaveShaperCurveFromData(id, fb);

        float[] out = ctx.getWaveShaperCurve(id);
        Assert.assertNotNull(out);
        Assert.assertEquals(vals.length, out.length);
        for (int i = 0; i < vals.length; ++i) {
            Assert.assertEquals(vals[i], out[i], 1e-6f);
        }
    }
}
