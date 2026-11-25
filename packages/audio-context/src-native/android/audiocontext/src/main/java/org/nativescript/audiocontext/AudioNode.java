package org.nativescript.audiocontext;

import androidx.annotation.Nullable;

/** A node in the render graph; every connection is an edge the engine renders. */
public interface AudioNode {

	default void connect(AudioNode node) {
		connect(node, 0, 0);
	}

	default void connect(AudioNode node, int output) {
		connect(node, output, 0);
	}

	default void connect(AudioNode node, int output, int input) {
		AudioContext.getInstance().connectNodes(this, output, node, input);
	}

	/** Audio-rate modulation: the output, mixed down to mono, adds to the param every frame. */
	default void connect(AudioParam param) {
		connect(param, 0);
	}

	default void connect(AudioParam param, int output) {
		AudioContext.getInstance().connectParam(this, output, param);
	}

	default void disconnect() {
		AudioContext.getInstance().disconnectNodes(this, -1, null, -1);
	}

	default void disconnect(int output) {
		AudioContext.getInstance().disconnectNodes(this, output, null, -1);
	}

	default void disconnect(@Nullable AudioNode node) {
		AudioContext.getInstance().disconnectNodes(this, -1, node, -1);
	}

	default void disconnect(@Nullable AudioNode node, int output) {
		AudioContext.getInstance().disconnectNodes(this, output, node, -1);
	}

	default void disconnect(@Nullable AudioNode node, int output, int input) {
		AudioContext.getInstance().disconnectNodes(this, output, node, input);
	}

	default void disconnect(AudioParam param) {
		disconnect(param, -1);
	}

	default void disconnect(AudioParam param, int output) {
		AudioContext.getInstance().disconnectParam(this, output, param);
	}

	/** channelCount (> 0), channelCountMode (0 max, 1 clamped-max, 2 explicit) and channelInterpretation (0 speakers, 1 discrete); negative keeps a value. */
	default void setChannelConfig(int channelCount, int countMode, int interpretation) {
		AudioContext.getInstance().configureNode(this, channelCount, countMode, interpretation);
	}
}
