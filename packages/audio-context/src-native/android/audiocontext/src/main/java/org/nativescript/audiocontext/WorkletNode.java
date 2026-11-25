package org.nativescript.audiocontext;

import androidx.annotation.NonNull;
import androidx.annotation.Nullable;

import java.nio.ByteBuffer;
import java.util.HashSet;
import java.util.Set;
import java.util.UUID;

/** An AudioWorkletNode: quanta come from a JS processor in a Worker; output o plays as voice {@link #getOutputId}. */
public class WorkletNode implements NativeObject, AudioNode {
	private final String id;
	private final int outputs;
	private final AudioParam[] params;
	// The ring's memory belongs to the JS SharedArrayBuffer; JS keeps it alive past stop().
	private final ByteBuffer ring;
	// Each output's destinations; an unconnected output is silent.
	private final Set<String>[] destinations;

	@SuppressWarnings("unchecked")
	private WorkletNode(String id, int outputs, ByteBuffer ring, AudioParam[] params) {
		this.id = id;
		this.outputs = outputs;
		this.ring = ring;
		this.params = params;
		this.destinations = new Set[outputs];
		for (int o = 0; o < outputs; o++) destinations[o] = new HashSet<>();
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

	/** The voice output {@code output} plays as. */
	public String getOutputId(int output) {
		return output == 0 ? id : id + "#" + output;
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
		if (!(node instanceof NativeObject) || output < 0 || output >= outputs) return;
		String destId = ((NativeObject) node).getId();
		if (destId == null) return;
		AudioContext context = AudioContext.getInstance();
		synchronized (destinations) {
			if (destinations[output].isEmpty()) context.setWorkletOutputConnected(id, output, true);
			destinations[output].add(destId);
		}
		if (context.isDestinationNode(destId)) return;
		String voice = getOutputId(output);
		if (node instanceof AudioPannerNode) {
			context.attachPannerToVoice(voice, destId, 0, input);
		} else if (node instanceof AudioBiquadNode) {
			context.attachBiquadToVoice(voice, destId, 0, input);
		} else {
			context.attachGainToVoice(voice, destId, 0, input);
		}
		context.registerNodeSource(destId, voice);
	}

	private void disconnectOutput(int output, @Nullable AudioNode node) {
		if (output < 0 || output >= outputs) return;
		AudioContext context = AudioContext.getInstance();
		String destId = node instanceof NativeObject ? ((NativeObject) node).getId() : null;
		synchronized (destinations) {
			if (destId != null) {
				if (!destinations[output].remove(destId)) return;
			} else {
				destinations[output].clear();
			}
			if (destinations[output].isEmpty()) context.setWorkletOutputConnected(id, output, false);
		}
		AudioContext.getInstance().disconnectNode(getOutputId(output), true, destId == null ? null : node, -1, -1);
	}

	@Override
	public void disconnect() {
		for (int o = 0; o < outputs; o++) disconnectOutput(o, null);
	}

	@Override
	public void disconnect(int output) {
		disconnectOutput(output, null);
	}

	@Override
	public void disconnect(AudioNode node) {
		if (node == null) {
			disconnect();
			return;
		}
		for (int o = 0; o < outputs; o++) disconnectOutput(o, node);
	}

	@Override
	public void disconnect(AudioNode node, int output) {
		if (node == null) disconnectOutput(output, null);
		else disconnectOutput(output, node);
	}

	@Override
	public void disconnect(AudioNode node, int output, int input) {
		disconnect(node, output);
	}
}
