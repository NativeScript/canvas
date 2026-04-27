package org.nativescript.audiocontext;

public class ConstantSourceNode extends AudioScheduledSourceNode {
    private double offset = 1.0;
    private final AudioParam offsetParam;

    ConstantSourceNode(String id) {
        super(id);
        this.offsetParam = new AudioParam(id);
        this.offsetParam.setValue(offset);
    }

    public ConstantSourceNode(AudioContextInstance context) {
        super(AudioContext.getInstance().createOscillator(context.getId(), "sine", 0.0));
        this.offsetParam = new AudioParam(id);
        this.offsetParam.setValue(offset);
    }

    public double getOffset() { return offsetParam.getValue(); }
    public void setOffset(double v) { offset = v; offsetParam.setValue(v); }

    public AudioParam getOffsetParam() { return offsetParam; }

    @Override
    public void start() {
        AudioContext.getInstance().startOscillator(id, "sine", 0.0);
    }

    @Override
    public void stop() {
        super.stop();
    }
}
