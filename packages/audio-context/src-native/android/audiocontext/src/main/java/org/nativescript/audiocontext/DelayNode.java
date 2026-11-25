package org.nativescript.audiocontext;

public class DelayNode implements NativeObject, AudioNode {
    private final String id;
    private final AudioParam delayTimeParam;
    private double maxDelayTime = 1.0;

    DelayNode(String id) {
        this.id = id;
        NativeFinalizer.register(this, NativeFinalizer.ResourceKind.GAIN, id);
        this.delayTimeParam = new AudioParam(id);
    }

    DelayNode(AudioContextInstance context) {
        this(AudioContext.getInstance().createDelay(context.getId(), 1.0));
    }

    @Override
    public String getId() { return id; }

    public double getMaxDelayTime() { return maxDelayTime; }
    public void setMaxDelayTime(double v) { maxDelayTime = v; }
    public AudioParam getDelayTime() { return delayTimeParam; }

    @Override
    public void release() {
        AudioContext.getInstance().releaseGain(id);
    }
}
