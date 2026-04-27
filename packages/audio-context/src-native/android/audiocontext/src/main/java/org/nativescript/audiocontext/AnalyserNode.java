package org.nativescript.audiocontext;

public class AnalyserNode implements NativeObject, AudioNode {
	private final String id;
	private int fftSize = 2048;
	private double smoothingTimeConstant = 0.8;
	private double minDecibels = -100.0;
	private double maxDecibels = -30.0;

	AnalyserNode(String id) {
		this.id = id;
		NativeFinalizer.register(this, NativeFinalizer.ResourceKind.GAIN, id);
	}

	AnalyserNode(AudioContextInstance context) {
		this(AudioContext.getInstance().createAnalyser(context.getId()));
	}

	@Override
	public String getId() {
		return id;
	}

	public int getFftSize() {
		return fftSize;
	}

	public void setFftSize(int v) {
		int p = 32;
		while (p < v && p < 32768) p <<= 1;
		if (p != v) v = p;
		if (v < 32) v = 32;
		if (v > 32768) v = 32768;
		fftSize = v;
		AudioContext.getInstance().setAnalyserFftSize(id, fftSize);
	}

	public int getFrequencyBinCount() {
		return fftSize / 2;
	}

	public double getSmoothingTimeConstant() {
		return smoothingTimeConstant;
	}

	public void setSmoothingTimeConstant(double v) {
		if (v < 0.0) v = 0.0;
		else if (v > 1.0) v = 1.0;
		smoothingTimeConstant = v;
		AudioContext.getInstance().setAnalyserSmoothingTimeConstant(id, v);
	}

	public double getMinDecibels() {
		return minDecibels;
	}

	public void setMinDecibels(double v) {
		minDecibels = v;
		AudioContext.getInstance().setAnalyserDecibels(id, (float) minDecibels, (float) maxDecibels);
	}

	public double getMaxDecibels() {
		return maxDecibels;
	}

	public void setMaxDecibels(double v) {
		maxDecibels = v;
		AudioContext.getInstance().setAnalyserDecibels(id, (float) minDecibels, (float) maxDecibels);
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

				java.util.Set<String> sources = context.getNodeSourcesSnapshot(id);
				if (sources.isEmpty()) {
					context.attachGainToVoice(destId, id, output, input);
					context.registerNodeSource(destId, id);
					return;
				}
				for (String sourceId : sources) {
					if (node instanceof AudioPannerNode) {
						context.attachPannerToVoice(sourceId, destId, output, input);
					} else if (node instanceof AudioBiquadNode) {
						context.attachBiquadToVoice(sourceId, destId, output, input);
					} else {
						context.attachGainToVoice(sourceId, destId, output, input);
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
		disconnect(node);
	}

	@Override
	public void disconnect(AudioNode node, int output, int input) {
		disconnect(node);
	}

	@Override
	public void release() {
		AudioContext.getInstance().releaseAnalyser(id);
		AudioContext.getInstance().releaseGain(id);
	}

	public void getFloatTimeDomainData(java.nio.FloatBuffer dest) {
		if (dest == null || dest.remaining() == 0) return;
		AudioContext ctx = AudioContext.getInstance();
		java.nio.FloatBuffer view = dest.duplicate();
		view.position(dest.position());
		view.limit(dest.limit());
		if (view.isDirect()) {
			java.nio.FloatBuffer slice = view.slice();
			if (ctx.getAnalyserFloatTimeDomainDataDirect(id, slice)) return;
		}
		float[] a = ctx.getAnalyserFloatTimeDomainData(id, view.remaining());
		if (a == null) return;
		int basePos = view.position();
		int n = Math.min(a.length, view.remaining());
		for (int i = 0; i < n; ++i) view.put(basePos + i, a[i]);
	}

	public void getFloatFrequencyData(java.nio.FloatBuffer dest) {
		if (dest == null || dest.remaining() == 0) return;
		AudioContext ctx = AudioContext.getInstance();
		java.nio.FloatBuffer view = dest.duplicate();
		view.position(dest.position());
		view.limit(dest.limit());
		if (view.isDirect()) {
			java.nio.FloatBuffer slice = view.slice();
			if (ctx.getAnalyserFloatFrequencyDataDirect(id, slice)) return;
		}
		float[] a = ctx.getAnalyserFloatFrequencyData(id);
		if (a == null) return;
		int basePos = view.position();
		int n = Math.min(a.length, view.remaining());
		for (int i = 0; i < n; ++i) view.put(basePos + i, a[i]);
	}

	public void getByteTimeDomainData(java.nio.ByteBuffer dest) {
		if (dest == null || dest.remaining() == 0) return;
		AudioContext ctx = AudioContext.getInstance();
		java.nio.ByteBuffer view = dest.duplicate();
		view.position(dest.position());
		view.limit(dest.limit());
		if (view.isDirect()) {
			java.nio.ByteBuffer slice = view.slice();
			if (ctx.getAnalyserByteTimeDomainDataDirect(id, slice)) return;
		}

		float[] a = ctx.getAnalyserFloatTimeDomainData(id, view.remaining());
		if (a == null) return;
		int basePos = view.position();
		int n = Math.min(a.length, view.remaining());
		for (int i = 0; i < n; ++i) {
			float v = a[i];
			if (v < -1.0f) v = -1.0f;
			else if (v > 1.0f) v = 1.0f;
			int b = (int) ((v * 0.5f + 0.5f) * 255.0f);
			if (b < 0) b = 0;
			else if (b > 255) b = 255;
			view.put(basePos + i, (byte) b);
		}
	}

	public void getByteFrequencyData(java.nio.ByteBuffer dest) {
		if (dest == null || dest.remaining() == 0) return;
		AudioContext ctx = AudioContext.getInstance();
		float lo = (float) getMinDecibels();
		float hi = (float) getMaxDecibels();
		float range = hi - lo;
		if (range <= 0.0f) range = 1.0f;

		java.nio.ByteBuffer view = dest.duplicate();
		view.position(dest.position());
		view.limit(dest.limit());
		if (view.isDirect()) {
			java.nio.ByteBuffer slice = view.slice();
			if (ctx.getAnalyserByteFrequencyDataDirect(id, slice, lo, hi)) return;
		}

		float[] a = ctx.getAnalyserFloatFrequencyData(id);
		if (a == null) return;
		int basePos = view.position();
		int n = Math.min(a.length, view.remaining());
		final float invRange = 1.0f / range;
		for (int i = 0; i < n; ++i) {
			float norm = (a[i] - lo) * invRange;
			if (norm < 0.0f) norm = 0.0f;
			else if (norm > 1.0f) norm = 1.0f;
			view.put(basePos + i, (byte) (norm * 255.0f));
		}
	}
}
