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
	public void release() {
		AudioContext.getInstance().releaseGain(id);
	}
}
