package org.nativescript.audiocontext;

public class ChannelMergerNode implements NativeObject, AudioNode {
	private final String id;
	private final int numberOfInputs;

	ChannelMergerNode(String id, int numberOfInputs) {
		this.id = id;
		this.numberOfInputs = Math.max(1, numberOfInputs);
		NativeFinalizer.register(this, NativeFinalizer.ResourceKind.GAIN, id);
	}

	@Override
	public String getId() {
		return id;
	}

	public int getNumberOfInputs() {
		return numberOfInputs;
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
		AudioContext.getInstance().releaseGain(id);
	}
}
