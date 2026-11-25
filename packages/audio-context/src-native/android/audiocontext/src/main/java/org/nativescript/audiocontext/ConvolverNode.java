package org.nativescript.audiocontext;

import androidx.annotation.Nullable;

public class ConvolverNode implements NativeObject, AudioNode {
    private final String id;
    private final String contextId;
    @Nullable
    private AudioBuffer buffer;
    private boolean normalize = true;

    ConvolverNode(String id) {
        this(id, null);
    }

    ConvolverNode(String id, @Nullable String contextId) {
        this.id = id;
        this.contextId = contextId;
        NativeFinalizer.register(this, NativeFinalizer.ResourceKind.GAIN, id);
    }

    ConvolverNode(AudioContextInstance context) {
        this(AudioContext.getInstance().createConvolver(context.getId()), context.getId());
    }

    @Override
    public String getId() {
        return id;
    }

    @Nullable
    public AudioBuffer getBuffer() {
        return buffer;
    }

    /** The response is taken, normalized or not, when it's set, as in the spec. */
    public void setBuffer(@Nullable AudioBuffer buffer) {
        this.buffer = buffer;
        if (buffer != null && contextId != null) {
            AudioContext.getInstance().assignBufferToContext(buffer, contextId);
        }
        AudioContext.getInstance().setConvolverBuffer(id, buffer, normalize);
    }

    public boolean getNormalize() { return normalize; }
    public void setNormalize(boolean v) { this.normalize = v; }

    public void setValue(double v) {
        AudioContext.getInstance().setGain(id, v);
    }

    @Override
    public void release() {
        AudioContext.getInstance().releaseGain(id);
    }
}
