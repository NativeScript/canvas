package org.nativescript.audiocontext;

public class WaveShaperNode implements NativeObject, AudioNode {
	private final String id;

	WaveShaperNode(String id) {
		this.id = id;
		NativeFinalizer.register(this, NativeFinalizer.ResourceKind.GAIN, id);
	}

	WaveShaperNode(AudioContextInstance context) {
		this(AudioContext.getInstance().createWaveShaper(context.getId()));
	}

	@Override
	public String getId() {
		return id;
	}

	public void setValue(double v) {
		AudioContext.getInstance().setGain(id, v);
	}

	@Override
	public void release() {
		AudioContext.getInstance().releaseGain(id);
	}

	public void setCurve(float[] curve) {
		if (curve == null) {
			AudioContext.getInstance().setWaveShaperCurveFromData(id, (java.nio.ByteBuffer) null);
			return;
		}
		java.nio.ByteBuffer bb = java.nio.ByteBuffer.allocateDirect(curve.length * 4).order(java.nio.ByteOrder.LITTLE_ENDIAN);
		for (float v : curve) bb.putFloat(v);
		bb.position(0);
		AudioContext.getInstance().setWaveShaperCurveFromData(id, bb);
	}

	public void setCurveFromData(java.nio.FloatBuffer data) {
		AudioContext.getInstance().setWaveShaperCurveFromData(id, data);
	}

	public void setCurveFromData(java.nio.ByteBuffer data) {
		AudioContext.getInstance().setWaveShaperCurveFromData(id, data);
	}

	public float[] getCurve() {
		return AudioContext.getInstance().getWaveShaperCurve(id);
	}

	public String getOversample() {
		return AudioContext.getInstance().getWaveShaperOversample(id);
	}

	public void setOversample(String v) {
		AudioContext.getInstance().setWaveShaperOversample(id, v);
	}
}
