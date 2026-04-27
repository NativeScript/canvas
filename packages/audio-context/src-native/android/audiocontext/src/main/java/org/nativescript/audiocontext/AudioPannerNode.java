package org.nativescript.audiocontext;

public class AudioPannerNode implements NativeObject, AudioNode {
	private final String id;

	private double _positionX = 0.0;
	private double _positionY = 0.0;
	private double _positionZ = 0.0;
	private double _orientationX = 1.0;
	private double _orientationY = 0.0;
	private double _orientationZ = 0.0;
	private double _pan = 0.0;


	private int _distanceModel = 0;
	private int _panningModel = 0;
	private double _refDistance = 1.0;
	private double _maxDistance = 10000.0;
	private double _rolloffFactor = 1.0;
	private double _coneInnerAngle = 360.0;
	private double _coneOuterAngle = 360.0;
	private double _coneOuterGain = 0.0;

	public AudioPannerNode(String id) {
		this.id = id;
		NativeFinalizer.register(this, NativeFinalizer.ResourceKind.PANNER, id);
	}

	public String getId() {
		return id;
	}

	public void setPosition(double x, double y, double z) {
		this._positionX = x; this._positionY = y; this._positionZ = z;
		AudioContext.getInstance().setPannerParams(id, this._positionX, this._positionY, this._positionZ, this._orientationX, this._orientationY, this._orientationZ, this._pan, this._distanceModel, this._panningModel, this._refDistance, this._maxDistance, this._rolloffFactor, this._coneInnerAngle, this._coneOuterAngle, this._coneOuterGain);
	}

	public void setOrientation(double x, double y, double z) {
		this._orientationX = x; this._orientationY = y; this._orientationZ = z;
		AudioContext.getInstance().setPannerParams(id, this._positionX, this._positionY, this._positionZ, this._orientationX, this._orientationY, this._orientationZ, this._pan, this._distanceModel, this._panningModel, this._refDistance, this._maxDistance, this._rolloffFactor, this._coneInnerAngle, this._coneOuterAngle, this._coneOuterGain);
	}

	public void setPan(double p) {
		this._pan = p;
		AudioContext.getInstance().setPannerParams(id, this._positionX, this._positionY, this._positionZ, this._orientationX, this._orientationY, this._orientationZ, this._pan, this._distanceModel, this._panningModel, this._refDistance, this._maxDistance, this._rolloffFactor, this._coneInnerAngle, this._coneOuterAngle, this._coneOuterGain);
	}

	public int getDistanceModel() { return this._distanceModel; }
	public void setDistanceModel(int v) { this._distanceModel = v; AudioContext.getInstance().setPannerParams(id, this._positionX, this._positionY, this._positionZ, this._orientationX, this._orientationY, this._orientationZ, this._pan, this._distanceModel, this._panningModel, this._refDistance, this._maxDistance, this._rolloffFactor, this._coneInnerAngle, this._coneOuterAngle, this._coneOuterGain); }

	public int getPanningModel() { return this._panningModel; }
	public void setPanningModel(int v) { this._panningModel = v; AudioContext.getInstance().setPannerParams(id, this._positionX, this._positionY, this._positionZ, this._orientationX, this._orientationY, this._orientationZ, this._pan, this._distanceModel, this._panningModel, this._refDistance, this._maxDistance, this._rolloffFactor, this._coneInnerAngle, this._coneOuterAngle, this._coneOuterGain); }

	public double getRefDistance() { return this._refDistance; }
	public void setRefDistance(double v) { this._refDistance = v; AudioContext.getInstance().setPannerParams(id, this._positionX, this._positionY, this._positionZ, this._orientationX, this._orientationY, this._orientationZ, this._pan, this._distanceModel, this._panningModel, this._refDistance, this._maxDistance, this._rolloffFactor, this._coneInnerAngle, this._coneOuterAngle, this._coneOuterGain); }

	public double getMaxDistance() { return this._maxDistance; }
	public void setMaxDistance(double v) { this._maxDistance = v; AudioContext.getInstance().setPannerParams(id, this._positionX, this._positionY, this._positionZ, this._orientationX, this._orientationY, this._orientationZ, this._pan, this._distanceModel, this._panningModel, this._refDistance, this._maxDistance, this._rolloffFactor, this._coneInnerAngle, this._coneOuterAngle, this._coneOuterGain); }

	public double getRolloffFactor() { return this._rolloffFactor; }
	public void setRolloffFactor(double v) { this._rolloffFactor = v; AudioContext.getInstance().setPannerParams(id, this._positionX, this._positionY, this._positionZ, this._orientationX, this._orientationY, this._orientationZ, this._pan, this._distanceModel, this._panningModel, this._refDistance, this._maxDistance, this._rolloffFactor, this._coneInnerAngle, this._coneOuterAngle, this._coneOuterGain); }

	public double getConeInnerAngle() { return this._coneInnerAngle; }
	public void setConeInnerAngle(double v) { this._coneInnerAngle = v; AudioContext.getInstance().setPannerParams(id, this._positionX, this._positionY, this._positionZ, this._orientationX, this._orientationY, this._orientationZ, this._pan, this._distanceModel, this._panningModel, this._refDistance, this._maxDistance, this._rolloffFactor, this._coneInnerAngle, this._coneOuterAngle, this._coneOuterGain); }

	public double getConeOuterAngle() { return this._coneOuterAngle; }
	public void setConeOuterAngle(double v) { this._coneOuterAngle = v; AudioContext.getInstance().setPannerParams(id, this._positionX, this._positionY, this._positionZ, this._orientationX, this._orientationY, this._orientationZ, this._pan, this._distanceModel, this._panningModel, this._refDistance, this._maxDistance, this._rolloffFactor, this._coneInnerAngle, this._coneOuterAngle, this._coneOuterGain); }

	public double getConeOuterGain() { return this._coneOuterGain; }
	public void setConeOuterGain(double v) { this._coneOuterGain = v; AudioContext.getInstance().setPannerParams(id, this._positionX, this._positionY, this._positionZ, this._orientationX, this._orientationY, this._orientationZ, this._pan, this._distanceModel, this._panningModel, this._refDistance, this._maxDistance, this._rolloffFactor, this._coneInnerAngle, this._coneOuterAngle, this._coneOuterGain); }


	public AudioParam getPositionXParam() { return new AudioParam(this, AudioParam.Type.PANNER_POSITION_X); }
	public AudioParam getPositionYParam() { return new AudioParam(this, AudioParam.Type.PANNER_POSITION_Y); }
	public AudioParam getPositionZParam() { return new AudioParam(this, AudioParam.Type.PANNER_POSITION_Z); }
	public AudioParam getOrientationXParam() { return new AudioParam(this, AudioParam.Type.PANNER_ORIENTATION_X); }
	public AudioParam getOrientationYParam() { return new AudioParam(this, AudioParam.Type.PANNER_ORIENTATION_Y); }
	public AudioParam getOrientationZParam() { return new AudioParam(this, AudioParam.Type.PANNER_ORIENTATION_Z); }

	public AudioParam getPan() { return new AudioParam(this, AudioParam.Type.PANNER_PAN); }


	double __getPositionX() { return this._positionX; }
	double __getPositionY() { return this._positionY; }
	double __getPositionZ() { return this._positionZ; }
	double __getOrientationX() { return this._orientationX; }
	double __getOrientationY() { return this._orientationY; }
	double __getOrientationZ() { return this._orientationZ; }
	double __getPan() { return this._pan; }

	void __setPositionComponent(AudioParam.Type t, double v) {
		switch (t) {
			case PANNER_POSITION_X: this._positionX = v; break;
			case PANNER_POSITION_Y: this._positionY = v; break;
			case PANNER_POSITION_Z: this._positionZ = v; break;
			default: break;
		}
		AudioContext.getInstance().setPannerParams(id, this._positionX, this._positionY, this._positionZ, this._orientationX, this._orientationY, this._orientationZ, this._pan, 0, 0, 1.0, 10000.0, 1.0, 360.0, 360.0, 0.0);
	}

	void __setOrientationComponent(AudioParam.Type t, double v) {
		switch (t) {
			case PANNER_ORIENTATION_X: this._orientationX = v; break;
			case PANNER_ORIENTATION_Y: this._orientationY = v; break;
			case PANNER_ORIENTATION_Z: this._orientationZ = v; break;
			default: break;
		}
		AudioContext.getInstance().setPannerParams(id, this._positionX, this._positionY, this._positionZ, this._orientationX, this._orientationY, this._orientationZ, this._pan, 0, 0, 1.0, 10000.0, 1.0, 360.0, 360.0, 0.0);
	}

	public void attachToVoice(String voiceId) {
		AudioContext.getInstance().attachPannerToVoice(voiceId, id);
	}

	public void detach() {
		AudioContext.getInstance().detachPanner(id);
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
			String destId = ((NativeObject) node).getId();
			if (destId != null) {
				if (context.isDestinationNode(destId)) {
					return;
				}
				java.util.Set<String> sources = context.getNodeSourcesSnapshot(id);
				if (sources.isEmpty()) {
					context.attachPannerToVoice(destId, id, output, input);
					context.registerNodeSource(destId, id);
					return;
				}
				for (String sourceId : sources) {
					if (node instanceof GainNode) {
						context.attachGainToVoice(sourceId, destId, output, input);
					} else if (node instanceof AudioBiquadNode) {
						context.attachBiquadToVoice(sourceId, destId, output, input);
					} else {
						context.attachPannerToVoice(sourceId, destId, output, input);
					}
				}
				context.registerNodeSources(destId, sources);
			}
		}
	}

	@Override
	public void disconnect(AudioNode node) {
		if (node instanceof NativeObject) {
			AudioContext context = AudioContext.getInstance();
			String destId = ((NativeObject) node).getId();
			if (destId != null) {
				java.util.Set<String> sources = context.getNodeSourcesSnapshot(id);
				for (String sourceId : sources) {
					if (node instanceof GainNode) {
						context.attachGainToVoice(sourceId, "", 0, 0);
					} else if (node instanceof AudioBiquadNode) {
						context.attachBiquadToVoice(sourceId, "", 0, 0);
					} else if (!context.isDestinationNode(destId)) {
						context.attachPannerToVoice(sourceId, "", 0, 0);
					}
					context.unregisterNodeSource(destId, sourceId);
				}
				if (sources.isEmpty() && !context.isDestinationNode(destId)) {
					context.attachPannerToVoice(destId, "");
					context.unregisterNodeSource(destId, id);
				}
				return;
			}
		}
		disconnect();
	}

	@Override
	public void disconnect() {
		AudioContext context = AudioContext.getInstance();
		context.detachPanner(id);
		context.clearNodeSources(id);
	}

	@Override
	public void disconnect(int output) {
		disconnect();
	}

	@Override
	public void disconnect(AudioNode node, int output) {
		if (node instanceof NativeObject) {
			AudioContext context = AudioContext.getInstance();
			String destId = ((NativeObject) node).getId();
			if (destId != null) {
				java.util.Set<String> sources = context.getNodeSourcesSnapshot(id);
				for (String sourceId : sources) {
					if (node instanceof GainNode) {
						context.attachGainToVoice(sourceId, "", output, 0);
					} else if (node instanceof AudioBiquadNode) {
						context.attachBiquadToVoice(sourceId, "", output, 0);
					} else if (!context.isDestinationNode(destId)) {
						context.attachPannerToVoice(sourceId, "", output, 0);
					}
					context.unregisterNodeSource(destId, sourceId);
				}
				if (sources.isEmpty() && !context.isDestinationNode(destId)) {
					context.attachPannerToVoice(destId, "", output, 0);
					context.unregisterNodeSource(destId, id);
				}
				return;
			}
		}
		disconnect();
	}

	@Override
	public void disconnect(AudioNode node, int output, int input) {
		if (node instanceof NativeObject) {
			AudioContext context = AudioContext.getInstance();
			String destId = ((NativeObject) node).getId();
			if (destId != null) {
				java.util.Set<String> sources = context.getNodeSourcesSnapshot(id);
				for (String sourceId : sources) {
					if (node instanceof GainNode) {
						context.attachGainToVoice(sourceId, "", output, input);
					} else if (node instanceof AudioBiquadNode) {
						context.attachBiquadToVoice(sourceId, "", output, input);
					} else if (!context.isDestinationNode(destId)) {
						context.attachPannerToVoice(sourceId, "", output, input);
					}
					context.unregisterNodeSource(destId, sourceId);
				}
				if (sources.isEmpty() && !context.isDestinationNode(destId)) {
					context.attachPannerToVoice(destId, "", output, input);
					context.unregisterNodeSource(destId, id);
				}
				return;
			}
		}
		disconnect();
	}

	@Override
	public void release() {
		AudioContext.getInstance().releasePanner(id);
	}
}
