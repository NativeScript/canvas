declare class NSCAnalyserNode extends NSCAudioNode {
	static alloc(): NSCAnalyserNode; // inherited from NSObject

	static new(): NSCAnalyserNode; // inherited from NSObject

	fftSize: number;

	readonly frequencyBinCount: number;

	maxDecibels: number;

	minDecibels: number;

	smoothingTimeConstant: number;

	constructor(o: { context: NSCAudioContext });

	appendBufferToRing(buffer: AVAudioPCMBuffer): void;

	getByteFrequencyData(data: NSMutableData): void;

	getByteTimeDomainData(data: NSMutableData): void;

	getFloatFrequencyData(data: NSMutableData): void;

	getFloatTimeDomainData(data: NSMutableData): void;

	initWithContext(context: NSCAudioContext): this;

	mixerNode(): AVAudioNode;

	setAcceptingFrames(accept: boolean): void;
}

declare class NSCAudioBuffer extends NSObject {
	static alloc(): NSCAudioBuffer; // inherited from NSObject

	static new(): NSCAudioBuffer; // inherited from NSObject

	readonly duration: number;

	readonly length: number;

	readonly numberOfChannels: number;

	readonly sampleRate: number;

	constructor(o: { context: NSCAudioContext; id: string; buffer: AVAudioPCMBuffer });

	constructor(o: { length: number; numberOfChannels: number; sampleRate: number });

	copyFromChannel(destination: NSMutableArray<any>, channel: number, startInChannel: number): void;

	copyToChannel(source: any, channel: number, startInChannel: number): void;

	getBuffer(): AVAudioPCMBuffer;

	getChannelData(channel: number): NSMutableData;

	initWithContextIdBuffer(context: NSCAudioContext, identifier: string, buffer: AVAudioPCMBuffer): this;

	initWithLengthNumberOfChannelsSampleRate(length: number, numberOfChannels: number, sampleRate: number): this;
}

declare class NSCAudioBufferSourceNode extends NSCAudioScheduledSourceNode {
	static alloc(): NSCAudioBufferSourceNode; // inherited from NSObject

	static new(): NSCAudioBufferSourceNode; // inherited from NSObject

	buffer: NSCAudioBuffer;

	loop: boolean;

	constructor(o: { context: NSCAudioContext; buffer: NSCAudioBuffer });

	getPlaybackRateParam(): NSCAudioParam;

	initWithContextBuffer(context: NSCAudioContext, buffer: NSCAudioBuffer): this;
}

declare class NSCAudioContext extends NSObject {
	static alloc(): NSCAudioContext; // inherited from NSObject

	static marshalMutableData(data: NSMutableData): NSMutableData;

	static new(): NSCAudioContext; // inherited from NSObject

	static startEngineWithRetryAttemptsLabelAsyncCompletion(engine: AVAudioEngine, attempts: number, label: string, completion: (p1: boolean) => void): boolean;

	readonly currentTime: number;

	readonly destination: NSCAudioNode;

	readonly engine: AVAudioEngine;

	readonly environmentNode: AVAudioEnvironmentNode;

	readonly format: AVAudioFormat;

	readonly sampleRate: number;

	voiceFilterByOutput: NSMutableDictionary<NSValue, NSMutableDictionary<number, NSCAudioNode>>;

	voiceGainByOutput: NSMutableDictionary<NSValue, NSMutableDictionary<number, NSCAudioNode>>;

	voicePannerByOutput: NSMutableDictionary<NSValue, NSMutableDictionary<number, NSCAudioNode>>;

	constructor(o: { sampleRate: number });

	constructor(o: { sampleRate: number; latencyHint: number });

	allAnalyserWrappers(): NSArray<NSCAnalyserNode>;

	closeAsync(completion: () => void): void;

	createAnalyserNode(): NSCAnalyserNode;

	createBiquadNode(): NSCBiquadNode;

	createBiquadNodeFrequencyQGain(type: string, frequency: number, Q: number, gain: number): NSCBiquadNode;

	createBufferSourceNode(buffer: NSCAudioBuffer): NSCAudioBufferSourceNode;

	createConstantSourceNode(offset: number): NSCConstantSourceNode;

	createConvolverNode(): NSCConvolverNode;

	createDelayNodeMaxDelayTime(delayTime: number, maxDelayTime: number): NSCDelayNode;

	createGainNode(): NSCGainNode;

	createIIRFilterNodeFeedback(feedforward: NSArray<number> | number[], feedback: NSArray<number> | number[]): NSCIIRFilterNode;

	createOscillatorNodeFrequency(type: string, frequency: number): NSCAudioOscillatorNode;

	createPannerNode(): NSCAudioPannerNode;

	createSourceNodeFromMediaPlayer(player: AVPlayer): NSCAudioNode;

	createStereoPannerNode(pan: number): NSCStereoPannerNode;

	createWaveShaperNode(): NSCWaveShaperNode;

	currentSinkId(): string;

	decodeAudioData(base64: string): NSCAudioBuffer;

	decodeAudioDataAsync(base64: string, completion: (p1: NSCAudioBuffer) => void): void;

	decodeAudioDataFromData(data: NSData): NSCAudioBuffer;

	decodeAudioDataFromDataAsync(data: NSData, completion: (p1: NSCAudioBuffer) => void): void;

	decodeAudioDataFromFile(path: string): NSCAudioBuffer;

	decodeAudioDataFromFileAsync(path: string, completion: (p1: NSCAudioBuffer) => void): void;

	decrementActiveCount(): void;

	detachNodeFromEngine(node: AVAudioNode, engine: AVAudioEngine): void;

	detachSource(source: NSCAudioNode): void;

	ensureEnvironmentNodeAttached(): void;

	ensureMainMixerConnectedToOutput(): void;

	extraLatencySeconds(): number;

	getListenerForwardXParam(): NSCAudioParam;

	getListenerForwardYParam(): NSCAudioParam;

	getListenerForwardZParam(): NSCAudioParam;

	getListenerPositionXParam(): NSCAudioParam;

	getListenerPositionYParam(): NSCAudioParam;

	getListenerPositionZParam(): NSCAudioParam;

	getListenerUpXParam(): NSCAudioParam;

	getListenerUpYParam(): NSCAudioParam;

	getListenerUpZParam(): NSCAudioParam;

	hasActiveAudio(): boolean;

	incrementActiveCount(): void;

	initWithSampleRate(sampleRate: number): this;

	initWithSampleRateLatencyHint(sampleRate: number, latencyHint: number): this;

	isNodeAttachedToEngine(node: AVAudioNode, engine: AVAudioEngine): boolean;

	nodeWrapperForAVNode(avNode: AVAudioNode): NSCAudioNode;

	registerNodeWrapper(node: NSCAudioNode): void;

	registerPendingNode(node: NSCAudioBufferSourceNode): void;

	resume(): void;

	resumeAllPendingNodes(): void;

	resumeAsync(completion: (p1: boolean) => void): void;

	setListenerOrientation(forwardX: number, forwardY: number, forwardZ: number, upX: number, upY: number, upZ: number): void;

	setListenerPosition(x: number, y: number, z: number): void;

	setSinkId(deviceId: string): boolean;

	suspend(): void;

	suspendAsync(completion: (p1: boolean) => void): void;

	unregisterPendingNode(node: NSCAudioBufferSourceNode): void;
}

declare function NSCAudioContext_cancelScheduledResume(engine: AVAudioEngine): void;

declare function NSCAudioContext_scheduleResumeOnEngineStart(engine: AVAudioEngine, delay: number): void;

declare class NSCAudioNode extends NSObject {
	static alloc(): NSCAudioNode; // inherited from NSObject

	static new(): NSCAudioNode; // inherited from NSObject

	readonly avNode: AVAudioNode;

	context: NSCAudioContext;

	constructor(o: { context: NSCAudioContext; node: AVAudioNode });

	connectTo(destination: NSCAudioNode): void;

	connectToOutput(destination: NSCAudioNode, output: number): void;

	connectToOutputInput(destination: NSCAudioNode, output: number, input: number): void;

	disconnect(): void;

	disconnectOutput(output: number): void;

	disconnectTo(destination: NSCAudioNode): void;

	disconnectToOutput(destination: NSCAudioNode, output: number): void;

	disconnectToOutputInput(destination: NSCAudioNode, output: number, input: number): void;

	handleConnectFromOutputInput(source: NSCAudioNode, output: number, input: number): void;

	handleConnectedToOutputInput(destination: NSCAudioNode, output: number, input: number): void;

	handleDisconnectFromOutputInput(source: NSCAudioNode, output: number, input: number): void;

	handleDisconnectedFromOutputInput(destination: NSCAudioNode, output: number, input: number): void;

	initWithContextNode(context: NSCAudioContext, node: AVAudioNode): this;
}

declare class NSCAudioOscillatorNode extends NSCAudioScheduledSourceNode {
	static alloc(): NSCAudioOscillatorNode; // inherited from NSObject

	static new(): NSCAudioOscillatorNode; // inherited from NSObject

	frequency: number;

	readonly frequencyParam: NSCAudioParam;

	type: string;

	constructor(o: { context: NSCAudioContext });

	constructor(o: { context: NSCAudioContext; type: string; frequency: number });

	initWithContext(context: NSCAudioContext): this;

	initWithContextTypeFrequency(context: NSCAudioContext, type: string, frequency: number): this;

	setPeriodicWave(wave: NSCPeriodicWave): void;
}

declare class NSCAudioPannerNode extends NSCAudioNode {
	static alloc(): NSCAudioPannerNode; // inherited from NSObject

	static new(): NSCAudioPannerNode; // inherited from NSObject

	orientationXParam: NSCAudioParam;

	orientationYParam: NSCAudioParam;

	orientationZParam: NSCAudioParam;

	positionXParam: NSCAudioParam;

	positionYParam: NSCAudioParam;

	positionZParam: NSCAudioParam;

	constructor(o: { context: NSCAudioContext });

	constructor(o: { contextAndParams: NSCAudioContext });

	attachSourceOutputInput(source: NSCAudioNode, output: number, input: number): void;

	detachSource(source: NSCAudioNode): void;

	disconnectPanner(): void;

	getConeInnerAngle(): number;

	getConeOuterAngle(): number;

	getConeOuterGain(): number;

	getDistanceModel(): number;

	getMaxDistance(): number;

	getPanningModel(): number;

	getRefDistance(): number;

	getRolloffFactor(): number;

	initWithContext(context: NSCAudioContext): this;

	initWithContextAndParams(context: NSCAudioContext, positionX: number, positionY: number, positionZ: number, orientationX: number, orientationY: number, orientationZ: number, pan: number, distanceModel: number, panningModel: number, refDistance: number, maxDistance: number, rolloffFactor: number, coneInnerAngle: number, coneOuterAngle: number, coneOuterGain: number): this;

	setConeInnerAngle(value: number): void;

	setConeOuterAngle(value: number): void;

	setConeOuterGain(value: number): void;

	setDistanceModel(value: number): void;

	setMaxDistance(value: number): void;

	setOrientation(x: number, y: number, z: number): void;

	setPan(value: number): void;

	setPanningModel(value: number): void;

	setPosition(x: number, y: number, z: number): void;

	setRefDistance(value: number): void;

	setRolloffFactor(value: number): void;
}

declare class NSCAudioParam extends NSObject {
	static alloc(): NSCAudioParam; // inherited from NSObject

	static new(): NSCAudioParam; // inherited from NSObject

	automationRate: string;

	onScheduleChanged: (p1: NSCAudioParam) => void;

	readonly value: number;

	constructor(o: { context: NSCAudioContext; defaultValue: number });

	cancelAndHoldAtTime(heldValue: number, time: number): void;

	cancelScheduledValues(time: number): void;

	currentScalarValue(): number;

	fillValuesForRangeSampleRateFrameCountInto(startTime: number, sampleRate: number, frameCount: number, outValues: interop.Pointer | interop.Reference<number>): boolean;

	getValuesForRange(startTime: number, sampleRate: number, frameCount: number): NSArray<number>;

	hasEventsAfter(time: number): boolean;

	initWithContextDefaultValue(context: NSCAudioContext, defaultValue: number): this;

	linearRampToValueAtTime(value: number, time: number): void;

	setValue(value: number): void;

	setValueAtTime(value: number, time: number): void;

	valueAtTime(time: number): number;
}

declare class NSCAudioScheduledSourceNode extends NSCAudioNode {
	static alloc(): NSCAudioScheduledSourceNode; // inherited from NSObject

	static new(): NSCAudioScheduledSourceNode; // inherited from NSObject

	onended: () => void;

	start(): void;

	startAt(when: number): void;

	stop(): void;

	stopAt(when: number): void;
}

declare class NSCBiquadNode extends NSCAudioNode {
	static alloc(): NSCBiquadNode; // inherited from NSObject

	static new(): NSCBiquadNode; // inherited from NSObject

	detuneParam: NSCAudioParam;

	frequencyParam: NSCAudioParam;

	gainParam: NSCAudioParam;

	qParam: NSCAudioParam;

	constructor(o: { context: NSCAudioContext });

	constructor(o: { context: NSCAudioContext; type: string; frequency: number; q: number; gain: number; detune: number });

	getType(): string;

	initWithContext(context: NSCAudioContext): this;

	initWithContextTypeFrequencyQGainDetune(context: NSCAudioContext, type: string, frequency: number, Q: number, gain: number, detune: number): this;

	setParams(type: string, frequency: number, Q: number, gain: number): void;

	setType(type: string): void;
}

declare class NSCConstantSourceNode extends NSCAudioScheduledSourceNode {
	static alloc(): NSCConstantSourceNode; // inherited from NSObject

	static new(): NSCConstantSourceNode; // inherited from NSObject

	readonly offsetParam: NSCAudioParam;

	constructor(o: { context: NSCAudioContext; offset: number });

	initWithContextOffset(context: NSCAudioContext, offset: number): this;
}

declare class NSCConvolverNode extends NSCAudioNode {
	static alloc(): NSCConvolverNode; // inherited from NSObject

	static new(): NSCConvolverNode; // inherited from NSObject

	buffer: NSCAudioBuffer;

	normalize: boolean;

	constructor(o: { context: NSCAudioContext });

	initWithContext(context: NSCAudioContext): this;
}

declare class NSCDelayNode extends NSCAudioNode {
	static alloc(): NSCDelayNode; // inherited from NSObject

	static new(): NSCDelayNode; // inherited from NSObject

	readonly delayTimeParam: NSCAudioParam;

	readonly maxDelayTime: number;

	constructor(o: { context: NSCAudioContext; delayTime: number; maxDelayTime: number });

	initWithContextDelayTimeMaxDelayTime(context: NSCAudioContext, delayTime: number, maxDelayTime: number): this;
}

declare class NSCGainNode extends NSCAudioNode {
	static alloc(): NSCGainNode; // inherited from NSObject

	static new(): NSCGainNode; // inherited from NSObject

	gain: number;

	gainParam: NSCAudioParam;

	constructor(o: { context: NSCAudioContext });

	initWithContext(context: NSCAudioContext): this;
}

declare class NSCIIRFilterNode extends NSCAudioNode {
	static alloc(): NSCIIRFilterNode; // inherited from NSObject

	static new(): NSCIIRFilterNode; // inherited from NSObject

	constructor(o: { context: NSCAudioContext; feedforward: NSArray<number> | number[]; feedback: NSArray<number> | number[] });

	getFrequencyResponseMagResponsePhaseResponse(frequencyHzData: NSData, magResponse: NSMutableData, phaseResponse: NSMutableData): void;

	initWithContextFeedforwardFeedback(context: NSCAudioContext, feedforward: NSArray<number> | number[], feedback: NSArray<number> | number[]): this;
}

declare class NSCMediaElementSourceTap extends NSObject {
	static alloc(): NSCMediaElementSourceTap; // inherited from NSObject

	static attachToPlayerContext(player: AVPlayer, context: NSCAudioContext): NSCMediaElementSourceTap;

	static new(): NSCMediaElementSourceTap; // inherited from NSObject

	readonly context: NSCAudioContext;

	readonly player: AVPlayer;

	readonly sourceNode: AVAudioSourceNode;

	detach(): void;

	getPlaybackRateParam(): NSCAudioParam;
}

declare class NSCOfflineAudioContext extends NSCAudioContext {
	static alloc(): NSCOfflineAudioContext; // inherited from NSObject

	static new(): NSCOfflineAudioContext; // inherited from NSObject

	configure(channels: number, lengthInFrames: number, sampleRate: number): void;

	createBufferSourceNodeOffline(buffer: NSCAudioBuffer): NSCAudioBufferSourceNode;

	decodeAudioDataFromDataAsyncOffline(data: NSData, completion: (p1: NSCAudioBuffer) => void): void;

	decodeAudioDataFromFileAsyncOffline(path: string, completion: (p1: NSCAudioBuffer) => void): void;

	startRenderingAsync(completion: (p1: NSCAudioBuffer) => void): void;
}

declare class NSCPeriodicWave extends NSObject {
	static alloc(): NSCPeriodicWave; // inherited from NSObject

	static new(): NSCPeriodicWave; // inherited from NSObject

	readonly disableNormalization: boolean;

	readonly imag: NSData;

	readonly real: NSData;

	constructor(o: { real: NSData; imag: NSData; disableNormalization: boolean });

	initWithRealImagDisableNormalization(real: NSData, imag: NSData, disableNormalization: boolean): this;
}

declare var NSCProducerTokenKey: interop.Pointer | interop.Reference<any>;

declare class NSCStereoPannerNode extends NSCAudioNode {
	static alloc(): NSCStereoPannerNode; // inherited from NSObject

	static new(): NSCStereoPannerNode; // inherited from NSObject

	readonly panParam: NSCAudioParam;

	constructor(o: { context: NSCAudioContext; pan: number });

	initWithContextPan(context: NSCAudioContext, pan: number): this;
}

declare class NSCWaveShaperNode extends NSCAudioNode {
	static alloc(): NSCWaveShaperNode; // inherited from NSObject

	static new(): NSCWaveShaperNode; // inherited from NSObject

	oversample: string;

	constructor(o: { context: NSCAudioContext });

	curve(): NSData;

	initWithContext(context: NSCAudioContext): this;

	setCurveFromData(floatData: NSData): void;
}
