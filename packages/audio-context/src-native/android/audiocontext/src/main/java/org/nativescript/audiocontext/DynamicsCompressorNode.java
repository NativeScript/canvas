package org.nativescript.audiocontext;

public class DynamicsCompressorNode implements NativeObject, AudioNode {
	private final String id;
	private final String thresholdId;
	private final String kneeId;
	private final String ratioId;
	private final String attackId;
	private final String releaseId;
	private final String reductionId;

	private final AudioParam threshold;
	private final AudioParam knee;
	private final AudioParam ratio;
	private final AudioParam attack;
	private final AudioParam release;
	private final AudioParam reduction;

	DynamicsCompressorNode(
		String id,
		String thresholdId,
		String kneeId,
		String ratioId,
		String attackId,
		String releaseId,
		String reductionId
	) {
		this.id = id;
		this.thresholdId = thresholdId;
		this.kneeId = kneeId;
		this.ratioId = ratioId;
		this.attackId = attackId;
		this.releaseId = releaseId;
		this.reductionId = reductionId;

		this.threshold = new AudioParam(thresholdId);
		this.knee = new AudioParam(kneeId);
		this.ratio = new AudioParam(ratioId);
		this.attack = new AudioParam(attackId);
		this.release = new AudioParam(releaseId);
		this.reduction = new AudioParam(reductionId);

		NativeFinalizer.register(this, NativeFinalizer.ResourceKind.GAIN, id);
	}

	@Override
	public String getId() {
		return id;
	}

	public AudioParam getThreshold() {
		return threshold;
	}

	public AudioParam getKnee() {
		return knee;
	}

	public AudioParam getRatio() {
		return ratio;
	}

	public AudioParam getAttack() {
		return attack;
	}

	public AudioParam getRelease() {
		return release;
	}

	public AudioParam getReduction() {
		return reduction;
	}

	@Override
	public void release() {
		AudioContext context = AudioContext.getInstance();
		context.releaseGain(id);
		context.releaseGain(thresholdId);
		context.releaseGain(kneeId);
		context.releaseGain(ratioId);
		context.releaseGain(attackId);
		context.releaseGain(releaseId);
		context.releaseGain(reductionId);
	}
}
