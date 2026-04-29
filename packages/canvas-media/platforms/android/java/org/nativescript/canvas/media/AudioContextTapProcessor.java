package org.nativescript.canvas.media;

import androidx.media3.common.audio.AudioProcessor;
import androidx.media3.common.audio.BaseAudioProcessor;
import androidx.media3.common.util.UnstableApi;

import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.FloatBuffer;


@UnstableApi
public class AudioContextTapProcessor extends BaseAudioProcessor {

    public interface Sink {
        void onPcmFrames(float[] interleaved, int sampleCount, int sampleRate, int channels);

        boolean isActive();
    }

    private volatile Sink _sink;
    private float[] _floatScratch = new float[2048];
    private int _silenceFlag;
    static final int ENCODING_PCM_16BIT_BIG_ENDIAN = 0x10000000;

    public void setSink(Sink sink) {
        this._sink = sink;
    }

    public Sink getSink() {
        return _sink;
    }

    @Override
    public AudioFormat onConfigure(AudioFormat inputFormat) throws UnhandledAudioFormatException {
        return inputFormat;
    }

    @Override
    public void queueInput(ByteBuffer inputBuffer) {
        if (!inputBuffer.hasRemaining()) return;

        Sink sink = _sink;
        AudioFormat fmt = inputAudioFormat;

        int remaining = inputBuffer.remaining();
        int channels = fmt.channelCount;
        int sampleRate = fmt.sampleRate;
        int bytesPerSample = bytesPerSample(fmt.encoding);

        ByteBuffer out = replaceOutputBuffer(remaining);
        int srcStart = inputBuffer.position();
        out.put(inputBuffer);
        out.flip();

        if (sink != null && sink.isActive() && bytesPerSample > 0 && channels > 0) {
            int sampleCount = remaining / bytesPerSample;
            if (sampleCount > 0) {
                if (_floatScratch.length < sampleCount) {
                    _floatScratch = new float[Math.max(sampleCount, _floatScratch.length * 2)];
                }
                ByteBuffer src = inputBuffer.duplicate();
                src.order(ByteOrder.nativeOrder());
                src.position(srcStart);
                src.limit(srcStart + remaining);
                copyToFloat(src, _floatScratch, sampleCount, fmt.encoding);
                try {
                    sink.onPcmFrames(_floatScratch, sampleCount, sampleRate, channels);
                } catch (Throwable t) {
                    android.util.Log.w("AudioContextTapProc", "sink threw", t);
                }
            }
        }
        
        inputBuffer.position(srcStart + remaining);
    }

    private static int bytesPerSample(int encoding) {
        switch (encoding) {
            case android.media.AudioFormat.ENCODING_PCM_8BIT:
                return 1;
            case android.media.AudioFormat.ENCODING_PCM_16BIT:
            case ENCODING_PCM_16BIT_BIG_ENDIAN:
                return 2;
            case android.media.AudioFormat.ENCODING_PCM_24BIT_PACKED:
                return 3;
            case android.media.AudioFormat.ENCODING_PCM_32BIT:
            case android.media.AudioFormat.ENCODING_PCM_FLOAT:
                return 4;
            default:
                return 0;
        }
    }

    private static void copyToFloat(ByteBuffer src, float[] dst, int sampleCount, int encoding) {
        switch (encoding) {
            case android.media.AudioFormat.ENCODING_PCM_FLOAT: {
                FloatBuffer fb = src.asFloatBuffer();
                fb.get(dst, 0, sampleCount);
                break;
            }
            case android.media.AudioFormat.ENCODING_PCM_16BIT: {
                for (int i = 0; i < sampleCount; i++) {
                    short s = src.getShort();
                    dst[i] = s / 32768.0f;
                }
                break;
            }
            case android.media.AudioFormat.ENCODING_PCM_8BIT: {
                for (int i = 0; i < sampleCount; i++) {
                    int u = src.get() & 0xff;
                    dst[i] = (u - 128) / 128.0f;
                }
                break;
            }
            case android.media.AudioFormat.ENCODING_PCM_32BIT: {
                for (int i = 0; i < sampleCount; i++) {
                    int v = src.getInt();
                    dst[i] = v / 2147483648.0f;
                }
                break;
            }
            case android.media.AudioFormat.ENCODING_PCM_24BIT_PACKED: {
                for (int i = 0; i < sampleCount; i++) {
                    int b0 = src.get() & 0xff;
                    int b1 = src.get() & 0xff;
                    int b2 = src.get();
                    int v = (b2 << 16) | (b1 << 8) | b0;
                    if ((v & 0x800000) != 0) v |= 0xff000000;
                    dst[i] = v / 8388608.0f;
                }
                break;
            }
            default: {
                java.util.Arrays.fill(dst, 0, sampleCount, 0f);
                break;
            }
        }
    }
}
