package org.nativescript.audiocontext;

import androidx.annotation.NonNull;
import androidx.annotation.Nullable;

import java.nio.ByteBuffer;
import java.util.UUID;

/** An AudioWorkletNode: a source whose quanta come from a JS processor in a Worker. */
public class WorkletSourceNode extends AudioScheduledSourceNode {
	private final AudioParam[] params;
	// The ring's memory belongs to the JS SharedArrayBuffer; JS keeps it alive past stop().
	private final ByteBuffer ring;

	private WorkletSourceNode(String id, ByteBuffer ring, AudioParam[] params) {
		super(id);
		this.ring = ring;
		this.params = params;
	}

	@Nullable
	static WorkletSourceNode create(@NonNull AudioContextInstance context, @NonNull ByteBuffer ring, int channels, @NonNull int[] layout, @Nullable double[] defaults) {
		int count = defaults == null ? 0 : defaults.length;
		String[] ids = new String[count];
		AudioParam[] params = new AudioParam[count];
		String prefix = "worklet-param-" + UUID.randomUUID() + "-";
		for (int i = 0; i < count; i++) ids[i] = prefix + i;
		String id = AudioContext.getInstance().createWorkletSource(context.getId(), ring, channels, layout, ids);
		if (id == null) return null;
		for (int i = 0; i < count; i++) {
			params[i] = new AudioParam(ids[i]);
			params[i].setValue(defaults[i]);
		}
		WorkletSourceNode node = new WorkletSourceNode(id, ring, params);
		node.contextId = context.getId();
		return node;
	}

	public int getParamCount() {
		return params.length;
	}

	public AudioParam getParam(int index) {
		return params[index];
	}
}
