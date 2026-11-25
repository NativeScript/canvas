package org.nativescript.audiocontext;

public class AudioScheduledSourceNode implements NativeObject, AudioNode {
	public String id;
	@androidx.annotation.Nullable
	String contextId;

	public AudioScheduledSourceNode(String id) {
		this.id = id;
	}

	public void start() {
	}

	public void stop() {
		AudioContext.getInstance().stopTrack(id);
	}

	// `when` is in the context's currentTime seconds; the render thread starts the voice on
	// the first frame at or after it.
	public void startAt(double when) {
		start();
	}

	public void stopAt(double when) {
		AudioContext.getInstance().stopTrackAt(id, resolveContextId(), when);
	}

	@androidx.annotation.Nullable
	String resolveContextId() {
		if (contextId == null) contextId = AudioContext.getInstance().findContextForTrack(id);
		return contextId;
	}

	public void addEndedListener(@androidx.annotation.NonNull AudioContext.EndedListener listener) {
		AudioContext.getInstance().addEndedListener(id, listener);
	}

	public void removeEndedListener(@androidx.annotation.NonNull AudioContext.EndedListener listener) {
		AudioContext.getInstance().removeEndedListener(id, listener);
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
		if (node == null) disconnect();
		else AudioContext.getInstance().disconnectNode(id, true, node, -1, -1);
	}

	@Override
	public void disconnect() {
		AudioContext.getInstance().disconnectNode(id, true, null, -1, -1);
	}

	@Override
	public void disconnect(int output) {
		disconnect();
	}

	@Override
	public void disconnect(AudioNode node, int output) {
		if (node == null) disconnect(output);
		else AudioContext.getInstance().disconnectNode(id, true, node, -1, -1);
	}

	@Override
	public void disconnect(AudioNode node, int output, int input) {
		if (node == null) disconnect(output);
		else AudioContext.getInstance().disconnectNode(id, true, node, -1, input);
	}

	@Override
	public void release() {
		stop();
	}
}
