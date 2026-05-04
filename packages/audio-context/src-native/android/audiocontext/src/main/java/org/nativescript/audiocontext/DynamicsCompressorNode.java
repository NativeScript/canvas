package org.nativescript.audiocontext;

public class DynamicsCompressorNode implements NativeObject, AudioNode {
	private final String id;
	private final String thresholdId;
	private final String kneeId;
	private final String ratioId;
	private final String attackId;
	private final String releaseId;
	private final String reductionId;

	private final AudioParam threshold;
	private final AudioParam knee;
	private final AudioParam ratio;
	private final AudioParam attack;
	private final AudioParam release;
	private final AudioParam reduction;

	DynamicsCompressorNode(
		String id,
		String thresholdId,
		String kneeId,
		String ratioId,
		String attackId,
		String releaseId,
		String reductionId
	) {
		this.id = id;
		this.thresholdId = thresholdId;
		this.kneeId = kneeId;
		this.ratioId = ratioId;
		this.attackId = attackId;
		this.releaseId = releaseId;
		this.reductionId = reductionId;

		this.threshold = new AudioParam(thresholdId);
		this.knee = new AudioParam(kneeId);
		this.ratio = new AudioParam(ratioId);
		this.attack = new AudioParam(attackId);
		this.release = new AudioParam(releaseId);
		this.reduction = new AudioParam(reductionId);

		NativeFinalizer.register(this, NativeFinalizer.ResourceKind.GAIN, id);
	}

	@Override
	public String getId() {
		return id;
	}

	public AudioParam getThreshold() {
		return threshold;
	}

	public AudioParam getKnee() {
		return knee;
	}

	public AudioParam getRatio() {
		return ratio;
	}

	public AudioParam getAttack() {
		return attack;
	}

	public AudioParam getRelease() {
		return release;
	}

	public AudioParam getReduction() {
		return reduction;
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
		if (node instanceof NativeObject) {
			AudioContext context = AudioContext.getInstance();
			NativeObject object = (NativeObject) node;
			String destId = object.getId();
			if (destId != null) {
				java.util.Set<String> sources = context.getNodeSourcesSnapshot(id);
				for (String sourceId : sources) {
					if (node instanceof AudioPannerNode) {
						context.attachPannerToVoice(sourceId, "", 0, 0);
					} else if (node instanceof AudioBiquadNode) {
						context.attachBiquadToVoice(sourceId, "", 0, 0);
					} else if (!context.isDestinationNode(destId)) {
						context.attachGainToVoice(sourceId, "", 0, 0);
					}
					context.unregisterNodeSource(destId, sourceId);
				}
				if (sources.isEmpty() && !context.isDestinationNode(destId)) {
					context.attachGainToVoice(destId, "", 0, 0);
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
		context.detachGain(id);
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
			NativeObject object = (NativeObject) node;
			String destId = object.getId();
			if (destId != null) {
				java.util.Set<String> sources = context.getNodeSourcesSnapshot(id);
				for (String sourceId : sources) {
					if (node instanceof AudioPannerNode) {
						context.attachPannerToVoice(sourceId, "", output, 0);
					} else if (node instanceof AudioBiquadNode) {
						context.attachBiquadToVoice(sourceId, "", output, 0);
					} else if (!context.isDestinationNode(destId)) {
						context.attachGainToVoice(sourceId, "", output, 0);
					}
					context.unregisterNodeSource(destId, sourceId);
				}
				if (sources.isEmpty() && !context.isDestinationNode(destId)) {
					context.attachGainToVoice(destId, "", output, 0);
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
			NativeObject object = (NativeObject) node;
			String destId = object.getId();
			if (destId != null) {
				java.util.Set<String> sources = context.getNodeSourcesSnapshot(id);
				for (String sourceId : sources) {
					if (node instanceof AudioPannerNode) {
						context.attachPannerToVoice(sourceId, "", output, input);
					} else if (node instanceof AudioBiquadNode) {
						context.attachBiquadToVoice(sourceId, "", output, input);
					} else if (!context.isDestinationNode(destId)) {
						context.attachGainToVoice(sourceId, "", output, input);
					}
					context.unregisterNodeSource(destId, sourceId);
				}
				if (sources.isEmpty() && !context.isDestinationNode(destId)) {
					context.attachGainToVoice(destId, "", output, input);
					context.unregisterNodeSource(destId, id);
				}
				return;
			}
		}
		disconnect();
	}

	@Override
	public void release() {
		AudioContext context = AudioContext.getInstance();
		context.releaseGain(id);
		context.releaseGain(thresholdId);
		context.releaseGain(kneeId);
		context.releaseGain(ratioId);
		context.releaseGain(attackId);
		context.releaseGain(releaseId);
		context.releaseGain(reductionId);
	}
}
