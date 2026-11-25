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

	@Override
	public void release() {
		AudioContext.getInstance().releaseGain(id);
	}
}
