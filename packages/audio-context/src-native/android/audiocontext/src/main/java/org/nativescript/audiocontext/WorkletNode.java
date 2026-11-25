package org.nativescript.audiocontext;

import androidx.annotation.NonNull;
import androidx.annotation.Nullable;

import java.nio.ByteBuffer;
import java.util.UUID;

/** An AudioWorkletNode: quanta come from a JS processor in a Worker. */
public class WorkletNode implements NativeObject, AudioNode {
	private final String id;
	private final int outputs;
	private final AudioParam[] params;
	// The ring's memory belongs to the JS SharedArrayBuffer; JS keeps it alive past stop().
	private final ByteBuffer ring;

	private WorkletNode(String id, int outputs, ByteBuffer ring, AudioParam[] params) {
		this.id = id;
		this.outputs = outputs;
		this.ring = ring;
		this.params = params;
	}

	/** layout: slotWords, header, inputCounts, inputData, outputCounts, paramLengths, paramData (audio-worklet-ring.ts). */
	@Nullable
	static WorkletNode create(@NonNull AudioContextInstance context, @NonNull ByteBuffer ring, @NonNull int[] inputChannels, @NonNull int[] outputChannels, @NonNull int[] layout, @NonNull int[] outputData, @Nullable double[] defaults) {
		int count = defaults == null ? 0 : defaults.length;
		String contextId = context.getId();
		// Param ids carry an offline context's engine prefix, which routes their calls.
		int prefixEnd = contextId != null ? contextId.indexOf('_') : -1;
		String prefix = (contextId != null && contextId.startsWith("o") && prefixEnd > 0 ? contextId.substring(0, prefixEnd + 1) : "") + "worklet-param-" + UUID.randomUUID() + "-";
		String[] ids = new String[count];
		for (int i = 0; i < count; i++) ids[i] = prefix + i;
		String id = AudioContext.getInstance().createWorkletNative(contextId, ring, inputChannels, outputChannels, layout, outputData, ids);
		if (id == null) return null;
		AudioParam[] params = new AudioParam[count];
		for (int i = 0; i < count; i++) {
			params[i] = new AudioParam(ids[i]);
			params[i].setValue(defaults[i]);
		}
		return new WorkletNode(id, outputChannels.length, ring, params);
	}

	@Override
	public String getId() {
		return id;
	}

	public int getParamCount() {
		return params.length;
	}

	public AudioParam getParam(int index) {
		return params[index];
	}

	public void stop() {
		AudioContext.getInstance().stopTrack(id);
	}

	@Override
	public void release() {
		stop();
	}
}
