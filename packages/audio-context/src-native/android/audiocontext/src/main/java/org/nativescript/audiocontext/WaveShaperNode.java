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
	public void connect(AudioNode node) {
		connect(node, 0, 0);
	}

	@Override
	public void connect(AudioNode node, int output) {
		connect(node, output, 0);
	}

	@Override
	public void connect(AudioNode node, int output, int input) {
		if (node instanceof NativeObject) {
			AudioContext context = AudioContext.getInstance();
			NativeObject object = (NativeObject) node;
			String destId = object.getId();
			if (destId != null) {
				if (context.isDestinationNode(destId)) {
					return;
				}

				java.util.Set<String> sources = context.getNodeSourcesSnapshot(id);
				if (sources.isEmpty()) {
					context.attachGainToVoice(destId, id, output, input);
					context.registerNodeSource(destId, id);
					return;
				}

				for (String sourceId : sources) {
					if (node instanceof AudioPannerNode) {
						context.attachPannerToVoice(sourceId, destId, output, input);
					} else if (node instanceof AudioBiquadNode) {
						context.attachBiquadToVoice(sourceId, destId, output, input);
					} else {
						context.attachGainToVoice(sourceId, destId, output, input);
					}
				}
				context.registerNodeSources(destId, sources);
			}
		}
	}

	@Override
	public void disconnect(AudioNode node) {
		if (node == null) disconnect();
		else AudioContext.getInstance().disconnectNode(id, false, node, -1, -1);
	}

	@Override
	public void disconnect() {
		AudioContext context = AudioContext.getInstance();
		context.detachGain(id);
		context.clearNodeSources(id);
	}

	@Override
	public void disconnect(int output) {
		disconnect();
	}

	@Override
	public void disconnect(AudioNode node, int output) {
		if (node == null) disconnect(output);
		else AudioContext.getInstance().disconnectNode(id, false, node, -1, -1);
	}

	@Override
	public void disconnect(AudioNode node, int output, int input) {
		if (node == null) disconnect(output);
		else AudioContext.getInstance().disconnectNode(id, false, node, -1, input);
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
