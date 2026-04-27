package org.nativescript.audiocontext;

public class AudioScheduledSourceNode implements NativeObject, AudioNode {
	public final String id;

	public AudioScheduledSourceNode(String id) {
		this.id = id;
	}

	public void start() {
	}

	public void stop() {
		AudioContext.getInstance().stopTrack(id);
	}

	@Override
	public String getId() {
		return id;
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
			String destId = ((NativeObject) node).getId();
			if (destId != null) {
				AudioContext context = AudioContext.getInstance();
				if (context.isDestinationNode(destId)) {
					return;
				}
				if (node instanceof AudioPannerNode) {
					context.attachPannerToVoice(id, destId, output, input);
				} else if (node instanceof AudioBiquadNode) {
					context.attachBiquadToVoice(id, destId, output, input);
				} else {
					context.attachGainToVoice(id, destId, output, input);
				}
				context.registerNodeSource(destId, id);
			}
		}
	}

	@Override
	public void disconnect(AudioNode node) {
		if (node instanceof NativeObject) {
			String destId = ((NativeObject) node).getId();
			if (destId != null) {
				AudioContext context = AudioContext.getInstance();
				if (node instanceof AudioPannerNode) {
					context.attachPannerToVoice(id, "", 0, 0);
				} else if (node instanceof AudioBiquadNode) {
					context.attachBiquadToVoice(id, "", 0, 0);
				} else if (!context.isDestinationNode(destId)) {
					context.attachGainToVoice(id, "", 0, 0);
				}
				context.unregisterNodeSource(destId, id);
				return;
			}
		}
		disconnect();
	}

	@Override
	public void disconnect() {
		AudioContext.getInstance().attachPannerToVoice(id, "", 0, 0);
		AudioContext.getInstance().attachBiquadToVoice(id, "", 0, 0);
		AudioContext.getInstance().attachGainToVoice(id, "", 0, 0);
	}

	@Override
	public void disconnect(int output) {
		AudioContext.getInstance().attachPannerToVoice(id, "", output, 0);
		AudioContext.getInstance().attachBiquadToVoice(id, "", output, 0);
		AudioContext.getInstance().attachGainToVoice(id, "", output, 0);
	}

	@Override
	public void disconnect(AudioNode node, int output) {
		if (node instanceof NativeObject) {
			String destId = ((NativeObject) node).getId();
			if (destId != null) {
				AudioContext context = AudioContext.getInstance();
				if (node instanceof AudioPannerNode) {
					context.attachPannerToVoice(id, "", output, 0);
				} else if (node instanceof AudioBiquadNode) {
					context.attachBiquadToVoice(id, "", output, 0);
				} else if (!context.isDestinationNode(destId)) {
					context.attachGainToVoice(id, "", output, 0);
				}
				context.unregisterNodeSource(destId, id);
				return;
			}
		}
		disconnect(output);
	}

	@Override
	public void disconnect(AudioNode node, int output, int input) {
		if (node instanceof NativeObject) {
			String destId = ((NativeObject) node).getId();
			if (destId != null) {
				AudioContext context = AudioContext.getInstance();
				if (node instanceof AudioPannerNode) {
					context.attachPannerToVoice(id, "", output, input);
				} else if (node instanceof AudioBiquadNode) {
					context.attachBiquadToVoice(id, "", output, input);
				} else if (!context.isDestinationNode(destId)) {
					context.attachGainToVoice(id, "", output, input);
				}
				context.unregisterNodeSource(destId, id);
				return;
			}
		}
		disconnect(output);
	}

	@Override
	public void release() {
		stop();
	}
}
