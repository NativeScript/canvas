package org.nativescript.audiocontext;

public class ChannelSplitterNode implements NativeObject, AudioNode {
	private final String id;
	private final int numberOfOutputs;

	ChannelSplitterNode(String id, int numberOfOutputs) {
		this.id = id;
		this.numberOfOutputs = Math.max(1, numberOfOutputs);
		NativeFinalizer.register(this, NativeFinalizer.ResourceKind.GAIN, id);
	}

	@Override
	public String getId() {
		return id;
	}

	public int getNumberOfOutputs() {
		return numberOfOutputs;
	}

	private int clampOutput(int output) {
		if (output < 0) return 0;
		if (output >= numberOfOutputs) return numberOfOutputs - 1;
		return output;
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

				int sourceOutput = clampOutput(output);
				java.util.Set<String> sources = context.getNodeSourcesSnapshot(id);
				if (sources.isEmpty()) {
					context.attachGainToVoice(destId, id, sourceOutput, input);
					context.registerNodeSource(destId, id);
					return;
				}

				for (String sourceId : sources) {
					if (node instanceof AudioPannerNode) {
						context.attachPannerToVoice(sourceId, destId, sourceOutput, input);
					} else if (node instanceof AudioBiquadNode) {
						context.attachBiquadToVoice(sourceId, destId, sourceOutput, input);
					} else {
						context.attachGainToVoice(sourceId, destId, sourceOutput, input);
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
				int sourceOutput = clampOutput(output);
				java.util.Set<String> sources = context.getNodeSourcesSnapshot(id);
				for (String sourceId : sources) {
					if (node instanceof AudioPannerNode) {
						context.attachPannerToVoice(sourceId, "", sourceOutput, 0);
					} else if (node instanceof AudioBiquadNode) {
						context.attachBiquadToVoice(sourceId, "", sourceOutput, 0);
					} else if (!context.isDestinationNode(destId)) {
						context.attachGainToVoice(sourceId, "", sourceOutput, 0);
					}
					context.unregisterNodeSource(destId, sourceId);
				}
				if (sources.isEmpty() && !context.isDestinationNode(destId)) {
					context.attachGainToVoice(destId, "", sourceOutput, 0);
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
				int sourceOutput = clampOutput(output);
				java.util.Set<String> sources = context.getNodeSourcesSnapshot(id);
				for (String sourceId : sources) {
					if (node instanceof AudioPannerNode) {
						context.attachPannerToVoice(sourceId, "", sourceOutput, input);
					} else if (node instanceof AudioBiquadNode) {
						context.attachBiquadToVoice(sourceId, "", sourceOutput, input);
					} else if (!context.isDestinationNode(destId)) {
						context.attachGainToVoice(sourceId, "", sourceOutput, input);
					}
					context.unregisterNodeSource(destId, sourceId);
				}
				if (sources.isEmpty() && !context.isDestinationNode(destId)) {
					context.attachGainToVoice(destId, "", sourceOutput, input);
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
