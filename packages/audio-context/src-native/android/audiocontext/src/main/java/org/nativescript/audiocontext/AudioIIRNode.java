package org.nativescript.audiocontext;

import java.nio.FloatBuffer;

public class AudioIIRNode implements NativeObject, AudioNode {
	private final String id;
	private final double[] feedforward;
	private final double[] feedback;
	private final String contextId;

	AudioIIRNode(String id) {
		this(id, null, null, null);
	}

	AudioIIRNode(String id, double[] feedforward, double[] feedback, String contextId) {
		this.id = id;
		this.feedforward = feedforward != null ? feedforward.clone() : null;
		this.feedback = feedback != null ? feedback.clone() : null;
		this.contextId = contextId;
		NativeFinalizer.register(this, NativeFinalizer.ResourceKind.IIR, id);
	}

	public String getId() {
		return id;
	}

	@Override
	public void release() {
		AudioContext.getInstance().releaseIIR(id);
	}

	public void getFrequencyResponse(double[] frequencyHz, double[] magResponse, double[] phaseResponse) {
		if (frequencyHz == null || magResponse == null || phaseResponse == null) return;
		int n = Math.min(Math.min(frequencyHz.length, magResponse.length), phaseResponse.length);
		if (n <= 0) return;

		double sr = 48000.0;
		if (contextId != null) {
			try {
				int s = AudioContext.getInstance().getSampleRate(contextId);
				if (s > 0) sr = s;
			} catch (Throwable ignored) {
			}
		}

		try {
			if (AudioContext.getInstance().getIIRFrequencyResponse(id, frequencyHz, magResponse, phaseResponse, sr)) {
				return;
			}
		} catch (Throwable ignored) {
		}

		double[] ff = this.feedforward;
		double[] fb = this.feedback;
		if (ff == null || fb == null) {
			for (int i = 0; i < n; ++i) {
				magResponse[i] = 0.0;
				phaseResponse[i] = 0.0;
			}
			return;
		}

		double fb0 = fb.length > 0 ? fb[0] : 1.0;
		for (int i = 0; i < n; ++i) {
			double w = (2.0 * Math.PI * frequencyHz[i]) / sr;
			double nR = 0.0, nI = 0.0, dR = 0.0, dI = 0.0;
			for (int k = 0; k < ff.length; ++k) {
				double c = ff[k] / fb0;
				nR += c * Math.cos(w * k);
				nI -= c * Math.sin(w * k);
			}
			for (int k = 0; k < fb.length; ++k) {
				double c = fb[k] / fb0;
				dR += c * Math.cos(w * k);
				dI -= c * Math.sin(w * k);
			}
			double numMag = Math.hypot(nR, nI);
			double denMag = Math.hypot(dR, dI);
			magResponse[i] = denMag == 0.0 ? 0.0 : numMag / denMag;
			phaseResponse[i] = Math.atan2(nI, nR) - Math.atan2(dI, dR);
		}
	}

	public void getFrequencyResponse(float[] frequencyHz, float[] magResponse, float[] phaseResponse) {
		if (frequencyHz == null || magResponse == null || phaseResponse == null) return;
		int n = Math.min(Math.min(frequencyHz.length, magResponse.length), phaseResponse.length);
		if (n <= 0) return;

		double sr = 48000.0;
		if (contextId != null) {
			try {
				int s = AudioContext.getInstance().getSampleRate(contextId);
				if (s > 0) sr = s;
			} catch (Throwable ignored) {
			}
		}

		try {
			double[] fD = new double[n];
			double[] mD = new double[n];
			double[] pD = new double[n];
			for (int i = 0; i < n; ++i) fD[i] = frequencyHz[i];
			if (AudioContext.getInstance().getIIRFrequencyResponse(id, fD, mD, pD, sr)) {
				for (int i = 0; i < n; ++i) {
					magResponse[i] = (float) mD[i];
					phaseResponse[i] = (float) pD[i];
				}
				return;
			}
		} catch (Throwable ignored) {
		}

		double[] ff = this.feedforward;
		double[] fb = this.feedback;
		if (ff == null || fb == null) {
			for (int i = 0; i < n; ++i) {
				magResponse[i] = 0.0f;
				phaseResponse[i] = 0.0f;
			}
			return;
		}

		double fb0 = fb.length > 0 ? fb[0] : 1.0;
		for (int i = 0; i < n; ++i) {
			double w = (2.0 * Math.PI * (double) frequencyHz[i]) / sr;
			double nR = 0.0, nI = 0.0, dR = 0.0, dI = 0.0;
			for (int k = 0; k < ff.length; ++k) {
				double c = ff[k] / fb0;
				nR += c * Math.cos(w * k);
				nI -= c * Math.sin(w * k);
			}
			for (int k = 0; k < fb.length; ++k) {
				double c = fb[k] / fb0;
				dR += c * Math.cos(w * k);
				dI -= c * Math.sin(w * k);
			}
			double numMag = Math.hypot(nR, nI);
			double denMag = Math.hypot(dR, dI);
			double mag = denMag == 0.0 ? 0.0 : numMag / denMag;
			double ph = Math.atan2(nI, nR) - Math.atan2(dI, dR);
			magResponse[i] = (float) mag;
			phaseResponse[i] = (float) ph;
		}
	}

	public void getFrequencyResponse(FloatBuffer frequencyHz, FloatBuffer magResponse, FloatBuffer phaseResponse) {
		if (frequencyHz == null || magResponse == null || phaseResponse == null) return;
		int n = Math.min(Math.min(frequencyHz.remaining(), magResponse.remaining()), phaseResponse.remaining());
		if (n <= 0) return;

		int posF = frequencyHz.position();
		int posM = magResponse.position();
		int posP = phaseResponse.position();

		double sr = 48000.0;
		if (contextId != null) {
			try {
				int s = AudioContext.getInstance().getSampleRate(contextId);
				if (s > 0) sr = s;
			} catch (Throwable ignored) {
			}
		}

		if (AudioContext.getInstance().getIIRFrequencyResponse(id, frequencyHz, magResponse, phaseResponse, sr)) {
			return;
		}

		double[] ff = this.feedforward;
		double[] fb = this.feedback;
		if (ff == null || fb == null) {
			for (int i = 0; i < n; ++i) {
				magResponse.put(posM + i, 0.0f);
				phaseResponse.put(posP + i, 0.0f);
			}
			return;
		}

		double fb0 = fb.length > 0 ? fb[0] : 1.0;
		for (int i = 0; i < n; ++i) {
			double w = (2.0 * Math.PI * (double) frequencyHz.get(posF + i)) / sr;
			double nR = 0.0, nI = 0.0, dR = 0.0, dI = 0.0;
			for (int k = 0; k < ff.length; ++k) {
				double c = ff[k] / fb0;
				nR += c * Math.cos(w * k);
				nI -= c * Math.sin(w * k);
			}
			for (int k = 0; k < fb.length; ++k) {
				double c = fb[k] / fb0;
				dR += c * Math.cos(w * k);
				dI -= c * Math.sin(w * k);
			}
			double numMag = Math.hypot(nR, nI);
			double denMag = Math.hypot(dR, dI);
			double mag = denMag == 0.0 ? 0.0 : numMag / denMag;
			double ph = Math.atan2(nI, nR) - Math.atan2(dI, dR);
			magResponse.put(posM + i, (float) mag);
			phaseResponse.put(posP + i, (float) ph);
		}
	}
}
