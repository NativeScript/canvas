package org.nativescript.audiocontext;

import android.media.MediaPlayer;
import android.media.AudioTrack;
import android.media.AudioManager;
import android.media.audiofx.Visualizer;
import android.util.Log;
import java.lang.reflect.Method;
import java.util.WeakHashMap;
import java.util.Map;

public class MediaPlayerAudioTapAdapter {

    private final MediaPlayer player;
    private Visualizer visualizer;
    private Object node;
    private Method pushMethod;
    private Method endMethod;
    private Object attachedContext;

    private MediaPlayerAudioTapAdapter(MediaPlayer player) {
        this.player = player;
    }

    private Object attachAudioContextTap(Object contextNative) {
        if (contextNative == null) return null;
        try {
            Method create = contextNative.getClass().getMethod("createExternalPcmSource", int.class, int.class);
            int sampleRate;
            try { sampleRate = AudioTrack.getNativeOutputSampleRate(AudioManager.STREAM_MUSIC); } catch (Throwable t) { sampleRate = 44100; }
            int channels = 1;
            Object replacement = create.invoke(contextNative, sampleRate, channels);
            if (replacement == null) return null;

            Method push = replacement.getClass().getMethod("pushFrames", float[].class);
            Method end = replacement.getClass().getMethod("endStream");

            this.attachedContext = contextNative;
            this.node = replacement;
            this.pushMethod = push;
            this.endMethod = end;

            int sessionId;
            try { sessionId = player.getAudioSessionId(); } catch (Throwable t) { sessionId = -1; }
            if (sessionId <= 0) {
                return replacement;
            }

            Visualizer vis = new Visualizer(sessionId);
            try {
                int maxRate;
                try { maxRate = Visualizer.getMaxCaptureRate(); } catch (Throwable t) { maxRate = sampleRate; }
                int[] capRange;
                try { capRange = Visualizer.getCaptureSizeRange(); } catch (Throwable t) { capRange = new int[]{128, 1024}; }
                int cap = (capRange != null && capRange.length >= 2) ? capRange[1] : 1024;
                vis.setCaptureSize(cap);
                final Method pm = this.pushMethod;
                final Object n = this.node;
                vis.setDataCaptureListener(new Visualizer.OnDataCaptureListener() {
                    @Override
                    public void onWaveFormDataCapture(Visualizer visualizer, byte[] waveform, int samplingRate) {
                        if (waveform == null) return;
                        float[] floats = new float[waveform.length];
                        for (int i = 0; i < waveform.length; i++) floats[i] = waveform[i] / 128.0f;
                        try { if (pm != null) pm.invoke(n, new Object[] { floats }); } catch (Throwable t) { Log.w("MediaPlayerTap", "pushFrames failed", t); }
                    }

                    @Override
                    public void onFftDataCapture(Visualizer visualizer, byte[] fft, int samplingRate) { }
                }, maxRate, true, false);
                vis.setEnabled(true);
                this.visualizer = vis;
            } catch (Throwable t) {
                Log.w("MediaPlayerTap", "visualizer setup failed", t);
            }

            return replacement;
        } catch (NoSuchMethodException e) {
            Log.w("MediaPlayerTap", "createExternalPcmSource missing", e);
            return null;
        } catch (Throwable t) {
            Log.w("MediaPlayerTap", "attach failed", t);
            return null;
        }
    }

    private void detachAudioContextTap() {
        Object nodeLocal = this.node;
        this.node = null;
        this.attachedContext = null;
        try { if (this.endMethod != null) this.endMethod.invoke(nodeLocal); } catch (Throwable ignored) { }
        this.pushMethod = null;
        this.endMethod = null;
        try {
            if (this.visualizer != null) {
                this.visualizer.setEnabled(false);
                this.visualizer.release();
            }
        } catch (Throwable ignored) { }
        this.visualizer = null;
    }

    private static final WeakHashMap<MediaPlayer, MediaPlayerAudioTapAdapter> map = new WeakHashMap<>();
    private static final WeakHashMap<Object, MediaPlayer> nodeToPlayer = new WeakHashMap<>();

    public static Object attach(MediaPlayer player, Object contextNative) {
        synchronized (map) {
            if (map.containsKey(player)) return null;
            MediaPlayerAudioTapAdapter adapter = new MediaPlayerAudioTapAdapter(player);
            Object node = adapter.attachAudioContextTap(contextNative);
            if (node == null) return null;
            map.put(player, adapter);
            synchronized (nodeToPlayer) { nodeToPlayer.put(node, player); }
            return node;
        }
    }

    public static void detach(MediaPlayer player) {
        synchronized (map) {
            MediaPlayerAudioTapAdapter adapter = map.remove(player);
            if (adapter != null) adapter.detachAudioContextTap();
            // remove any node->player mappings for this player
            synchronized (nodeToPlayer) {
                java.util.Iterator<Map.Entry<Object, MediaPlayer>> it = nodeToPlayer.entrySet().iterator();
                while (it.hasNext()) {
                    Map.Entry<Object, MediaPlayer> e = it.next();
                    MediaPlayer p = e.getValue();
                    if (p == player) it.remove();
                }
            }
        }
    }

    public static boolean attachToNode(MediaPlayer player, Object externalNode) {
        if (player == null || externalNode == null) return false;
        synchronized (map) {
            if (map.containsKey(player)) return false;
            MediaPlayerAudioTapAdapter adapter = new MediaPlayerAudioTapAdapter(player);
            try {
                Method push = externalNode.getClass().getMethod("pushFrames", float[].class);
                Method end = externalNode.getClass().getMethod("endStream");
                adapter.node = externalNode;
                adapter.pushMethod = push;
                adapter.endMethod = end;
            } catch (Throwable t) {
                return false;
            }

            int sessionId;
            try { sessionId = player.getAudioSessionId(); } catch (Throwable t) { sessionId = -1; }
            if (sessionId > 0) {
                try {
                    Visualizer vis = new Visualizer(sessionId);
                    int maxRate;
                    try { maxRate = Visualizer.getMaxCaptureRate(); } catch (Throwable t) { maxRate = AudioTrack.getNativeOutputSampleRate(AudioManager.STREAM_MUSIC); }
                    int[] capRange;
                    try { capRange = Visualizer.getCaptureSizeRange(); } catch (Throwable t) { capRange = new int[]{128,1024}; }
                    int cap = (capRange != null && capRange.length >= 2) ? capRange[1] : 1024;
                    vis.setCaptureSize(cap);
                    final Method pm = adapter.pushMethod;
                    final Object n = adapter.node;
                    vis.setDataCaptureListener(new Visualizer.OnDataCaptureListener() {
                        @Override
                        public void onWaveFormDataCapture(Visualizer visualizer, byte[] waveform, int samplingRate) {
                            if (waveform == null) return;
                            float[] floats = new float[waveform.length];
                            for (int i = 0; i < waveform.length; i++) floats[i] = waveform[i] / 128.0f;
                            try { if (pm != null) pm.invoke(n, new Object[] { floats }); } catch (Throwable t) { Log.w("MediaPlayerTap","pushFrames failed", t); }
                        }

                        @Override
                        public void onFftDataCapture(Visualizer visualizer, byte[] fft, int samplingRate) { }
                    }, maxRate, true, false);
                    vis.setEnabled(true);
                    adapter.visualizer = vis;
                } catch (Throwable t) {
                    Log.w("MediaPlayerTap", "visualizer setup failed", t);
                }
            }

            map.put(player, adapter);
            synchronized (nodeToPlayer) { nodeToPlayer.put(externalNode, player); }
            return true;
        }
    }

    public static void detachByNode(Object externalNode) {
        if (externalNode == null) return;
        MediaPlayer player = null;
        synchronized (nodeToPlayer) { player = nodeToPlayer.remove(externalNode); }
        if (player != null) detach(player);
    }

}
