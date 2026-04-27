package org.nativescript.audiocontext;

public interface AudioNode {

	void connect(AudioNode node);

	void disconnect(AudioNode node);

	default void connect(AudioNode node, int output) {
		connect(node);
	}

	default void connect(AudioNode node, int output, int input) {
		connect(node);
	}

	default void disconnect() {
		disconnect(null);
	}

	default void disconnect(int output) {
		disconnect(null);
	}

	default void disconnect(AudioNode node, int output) {
		disconnect(node);
	}

	default void disconnect(AudioNode node, int output, int input) {
		disconnect(node);
	}
}
