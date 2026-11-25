package org.nativescript.audiocontext;

import java.util.Arrays;

public class AudioParam implements NativeObject {
	public enum Type {
		GAIN,
		BIQUAD_FREQUENCY,
		BIQUAD_Q,
		BIQUAD_GAIN,
		BIQUAD_DETUNE,
		PANNER_POSITION_X,
		PANNER_POSITION_Y,
		PANNER_POSITION_Z,
		PANNER_ORIENTATION_X,
		PANNER_ORIENTATION_Y,
		PANNER_ORIENTATION_Z,
		PANNER_PAN
		,
		LISTENER_POSITION_X,
		LISTENER_POSITION_Y,
		LISTENER_POSITION_Z,
		LISTENER_FORWARD_X,
		LISTENER_FORWARD_Y,
		LISTENER_FORWARD_Z,
		LISTENER_UP_X,
		LISTENER_UP_Y,
		LISTENER_UP_Z
	}

	private final String id;
	private final Type type;
	private final AudioPannerNode owner;
	private final AudioContextInstance contextOwner;

	// 0 = k-rate, 1 = a-rate
	private int automationRate = 1;

	AudioParam(String id) {
		this(id, Type.GAIN, (AudioPannerNode) null);
	}

	AudioParam(String id, Type type) {
		this(id, type, (AudioPannerNode) null);
	}

	private AudioParam(String id, Type type, AudioPannerNode owner) {
		this.id = id;
		this.type = type;
		this.owner = owner;
		this.contextOwner = null;
		if (owner == null && type == Type.GAIN) {
			NativeFinalizer.register(this, NativeFinalizer.ResourceKind.GAIN, id);
		}
	}

	private AudioParam(String id, Type type, AudioContextInstance contextOwner) {
		this.id = id;
		this.type = type;
		this.owner = null;
		this.contextOwner = contextOwner;
	}

	AudioParam(AudioPannerNode owner, Type type) {
		this(owner.getId(), type, owner);
	}

	AudioParam(AudioContextInstance owner, Type type) {
		this(owner.getId(), type, owner);
	}

	@Override
	public String getId() {
		return id;
	}

	public Type getType() {
		return type;
	}

	public void setValue(double v) {
		if (owner != null) {
			switch (type) {
				case PANNER_POSITION_X:
				case PANNER_POSITION_Y:
				case PANNER_POSITION_Z:
					owner.__setPositionComponent(type, v);
					return;
				case PANNER_ORIENTATION_X:
				case PANNER_ORIENTATION_Y:
				case PANNER_ORIENTATION_Z:
					owner.__setOrientationComponent(type, v);
					return;
				case PANNER_PAN:
					owner.setPan(v);
					return;
				default:
					break;
			}
		}
		if (contextOwner != null) {
			String ctx = contextOwner.getId();
			double px = AudioContext.getInstance().getListenerParamValue(ctx, Type.LISTENER_POSITION_X.ordinal());
			double py = AudioContext.getInstance().getListenerParamValue(ctx, Type.LISTENER_POSITION_Y.ordinal());
			double pz = AudioContext.getInstance().getListenerParamValue(ctx, Type.LISTENER_POSITION_Z.ordinal());
			double fx = AudioContext.getInstance().getListenerParamValue(ctx, Type.LISTENER_FORWARD_X.ordinal());
			double fy = AudioContext.getInstance().getListenerParamValue(ctx, Type.LISTENER_FORWARD_Y.ordinal());
			double fz = AudioContext.getInstance().getListenerParamValue(ctx, Type.LISTENER_FORWARD_Z.ordinal());
			double ux = AudioContext.getInstance().getListenerParamValue(ctx, Type.LISTENER_UP_X.ordinal());
			double uy = AudioContext.getInstance().getListenerParamValue(ctx, Type.LISTENER_UP_Y.ordinal());
			double uz = AudioContext.getInstance().getListenerParamValue(ctx, Type.LISTENER_UP_Z.ordinal());
			switch (type) {
				case LISTENER_POSITION_X: px = v; break;
				case LISTENER_POSITION_Y: py = v; break;
				case LISTENER_POSITION_Z: pz = v; break;
				case LISTENER_FORWARD_X: fx = v; break;
				case LISTENER_FORWARD_Y: fy = v; break;
				case LISTENER_FORWARD_Z: fz = v; break;
				case LISTENER_UP_X: ux = v; break;
				case LISTENER_UP_Y: uy = v; break;
				case LISTENER_UP_Z: uz = v; break;
				default: break;
			}
			contextOwner.setListenerParams(px, py, pz, fx, fy, fz, ux, uy, uz);
			return;
		}
		AudioContext.getInstance().setGain(id, v);
	}

	public double getValue() {
		if (owner != null) {
			switch (type) {
				case PANNER_POSITION_X:
					return owner.__getPositionX();
				case PANNER_POSITION_Y:
					return owner.__getPositionY();
				case PANNER_POSITION_Z:
					return owner.__getPositionZ();
				case PANNER_ORIENTATION_X:
					return owner.__getOrientationX();
				case PANNER_ORIENTATION_Y:
					return owner.__getOrientationY();
				case PANNER_ORIENTATION_Z:
					return owner.__getOrientationZ();
				case PANNER_PAN:
					return owner.__getPan();
				default:
					break;
			}
		}
		if (contextOwner != null) {
			return AudioContext.getInstance().getListenerParamValue(contextOwner.getId(), this.type.ordinal());
		}
		return AudioContext.getInstance().getGain(id);
	}

	public void setValueAtTime(double v, double t) {
		if (owner != null) {
			AudioContext.getInstance().schedulePannerSet(owner.getId(), this.type.ordinal(), this.automationRate, v, t);
			if (t <= 0.0) setValue(v);
			return;
		}
		if (contextOwner != null) {
			AudioContext.getInstance().scheduleListenerSet(contextOwner.getId(), this.type.ordinal(), this.automationRate, v, t);
			if (t <= 0.0) setValue(v);
			return;
		}
		AudioContext.getInstance().scheduleGainSet(id, this.automationRate, v, t);
		if (t <= 0.0) AudioContext.getInstance().setGain(id, v);
	}

	public void linearRampToValueAtTime(double v, double t) {
		if (owner != null) {
			AudioContext.getInstance().schedulePannerRamp(owner.getId(), this.type.ordinal(), this.automationRate, v, t);
			if (t <= 0.0) setValue(v);
			return;
		}
		if (contextOwner != null) {
			AudioContext.getInstance().scheduleListenerRamp(contextOwner.getId(), this.type.ordinal(), this.automationRate, v, t);
			if (t <= 0.0) setValue(v);
			return;
		}
		AudioContext.getInstance().scheduleGainRamp(id, this.automationRate, v, t);
	}

	public void exponentialRampToValueAtTime(double v, double t) {
		schedule(2, v, t, 0.0, 0.0, null);
	}

	public void setTargetAtTime(double v, double t, double timeConstant) {
		schedule(3, v, t, timeConstant, 0.0, null);
	}

	public void setValueCurveAtTime(java.nio.FloatBuffer values, double t, double duration) {
		if (values == null || values.remaining() < 2) return;
		java.nio.FloatBuffer curve = values.slice();
		if (!curve.isDirect()) {
			java.nio.FloatBuffer direct = java.nio.ByteBuffer.allocateDirect(curve.remaining() * 4).order(java.nio.ByteOrder.nativeOrder()).asFloatBuffer();
			direct.put(curve);
			direct.flip();
			curve = direct;
		}
		schedule(4, 0.0, t, 0.0, duration, curve);
	}

	private void schedule(int type, double v, double t, double timeConstant, double duration, java.nio.FloatBuffer curve) {
		if (owner != null) {
			AudioContext.getInstance().scheduleParamEvent(1, owner.getId(), this.type.ordinal(), type, this.automationRate, t, v, timeConstant, duration, curve);
		} else if (contextOwner != null) {
			AudioContext.getInstance().scheduleParamEvent(2, contextOwner.getId(), this.type.ordinal(), type, this.automationRate, t, v, timeConstant, duration, curve);
		} else {
			AudioContext.getInstance().scheduleParamEvent(0, id, 0, type, this.automationRate, t, v, timeConstant, duration, curve);
		}
	}

	public String getAutomationRate() {
		return this.automationRate == 0 ? "k-rate" : "a-rate";
	}

	public void setAutomationRate(String rate) {
		if ("k-rate".equals(rate)) this.automationRate = 0;
		else this.automationRate = 1;
	}

	public int getAutomationRateCode() {
		return this.automationRate;
	}

	public void cancelScheduledValues(double t) {
		if (owner != null) {
			AudioContext.getInstance().cancelPannerScheduledValues(owner.getId(), this.type.ordinal(), t);
			return;
		}
		if (contextOwner != null) {
			AudioContext.getInstance().cancelListenerScheduledValues(contextOwner.getId(), this.type.ordinal(), t);
			return;
		}
		AudioContext.getInstance().cancelGainScheduledValues(id, t);
	}

	public void cancelAndHoldAtTime(double heldValue, double t) {
		if (owner != null) {
			AudioContext.getInstance().cancelAndHoldPannerScheduledValues(owner.getId(), this.type.ordinal(), this.automationRate, heldValue, t);
			return;
		}
		if (contextOwner != null) {
			AudioContext.getInstance().cancelAndHoldListenerScheduledValues(contextOwner.getId(), this.type.ordinal(), this.automationRate, heldValue, t);
			return;
		}
		AudioContext.getInstance().cancelAndHoldGainScheduledValues(id, this.automationRate, heldValue, t);
	}

	public double[] getValuesForRange(double startTime, double sampleRate, int frameCount) {
		if (owner != null) {
			return AudioContext.getInstance().getPannerParamValues(owner.getId(), this.type.ordinal(), startTime, sampleRate, frameCount);
		}
		if (contextOwner != null) {
			return AudioContext.getInstance().getListenerParamValues(contextOwner.getId(), this.type.ordinal(), startTime, sampleRate, frameCount);
		}
		return AudioContext.getInstance().getGainParamValues(id, startTime, sampleRate, frameCount);
	}

	@Override
	public void release() {
		if (owner == null && contextOwner == null && type == Type.GAIN) {
			AudioContext.getInstance().releaseGain(id);
		}
	}
}
