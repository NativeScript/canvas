declare module org {
	export module nativescript {
		export module audiocontext {
			export class AnalyserNode implements org.nativescript.audiocontext.NativeObject, org.nativescript.audiocontext.AudioNode {
				public static class: java.lang.Class<org.nativescript.audiocontext.AnalyserNode>;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public disconnect(param0: org.nativescript.audiocontext.AudioNode): void;
				public getId(): string;
				public setFftSize(v: number): void;
				public setMinDecibels(v: number): void;
				public getSmoothingTimeConstant(): number;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public getMaxDecibels(): number;
				public connect(node: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public getFrequencyBinCount(): number;
				public disconnect(): void;
				public connect(param0: org.nativescript.audiocontext.AudioNode): void;
				public release(): void;
				public getMinDecibels(): number;
				public getByteTimeDomainData(v: java.nio.ByteBuffer): void;
				public getFloatTimeDomainData(i: java.nio.FloatBuffer): void;
				public getFloatFrequencyData(i: java.nio.FloatBuffer): void;
				public connect(sources: org.nativescript.audiocontext.AudioNode, context: number, object: number): void;
				public setSmoothingTimeConstant(v: number): void;
				public disconnect(sources: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(output: number): void;
				public getFftSize(): number;
				public setMaxDecibels(v: number): void;
				public getByteFrequencyData(norm: java.nio.ByteBuffer): void;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module audiocontext {
			export class AudioBiquadNode implements org.nativescript.audiocontext.NativeObject, org.nativescript.audiocontext.AudioNode {
				public static class: java.lang.Class<org.nativescript.audiocontext.AudioBiquadNode>;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public connect(param0: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(param0: org.nativescript.audiocontext.AudioNode): void;
				public constructor(context: org.nativescript.audiocontext.AudioContextInstance);
				public getId(): string;
				public release(): void;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public connect(node: org.nativescript.audiocontext.AudioNode): void;
				public getQ(): org.nativescript.audiocontext.AudioParam;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public setParams(type: string, frequency: number, Q: number, gain: number): void;
				public connect(sources: org.nativescript.audiocontext.AudioNode, context: number, object: number): void;
				public getGain(): org.nativescript.audiocontext.AudioParam;
				public disconnect(sources: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(sources: org.nativescript.audiocontext.AudioNode, context: number): void;
				public disconnect(): void;
				public disconnect(output: number): void;
				public constructor(context: org.nativescript.audiocontext.AudioContextInstance, type: string, frequency: number, Q: number, gain: number, detune: number);
				public getFrequency(): org.nativescript.audiocontext.AudioParam;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public constructor(context: org.nativescript.audiocontext.AudioContextInstance, type: string, frequency: number, Q: number, gain: number);
				public getDetune(): org.nativescript.audiocontext.AudioParam;
				public disconnect(sources: org.nativescript.audiocontext.AudioNode, context: number, object: number): void;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module audiocontext {
			export class AudioBuffer extends org.nativescript.audiocontext.NativeObject {
				public static class: java.lang.Class<org.nativescript.audiocontext.AudioBuffer>;
				public getId(): string;
				public release(): void;
				public copyToChannel(source: java.nio.ByteBuffer, channel: number, startInChannel: number): void;
				public getChannelData(this_: number): java.nio.FloatBuffer;
				public getDuration(): number;
				public copyToChannel(byteIndex: java.nio.FloatBuffer, i: number, val: number): void;
				public copyToChannel(i: androidNative.Array<number>, byteIndex: number, v: number): void;
				public getLength(): number;
				public copyFromChannel(i: java.nio.FloatBuffer, s: number, i: number): void;
				public getChannelDataRaw(channel: number): java.nio.ByteBuffer;
				public constructor(this_: number, length: number, numberOfChannels: number);
				public getSampleRate(): number;
				public getNumberOfChannels(): number;
				public copyFromChannel(dest: java.nio.ByteBuffer, channel: number, startInChannel: number): void;
				public copyFromChannel(dest: androidNative.Array<number>, channel: number, startInChannel: number): void;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module audiocontext {
			export class AudioBufferSourceNode extends org.nativescript.audiocontext.AudioScheduledSourceNode {
				public static class: java.lang.Class<org.nativescript.audiocontext.AudioBufferSourceNode>;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public connect(param0: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(destId: org.nativescript.audiocontext.AudioNode, this_: number, node: number): void;
				public disconnect(param0: org.nativescript.audiocontext.AudioNode): void;
				public getId(): string;
				public release(): void;
				public constructor(id: string, buffer: org.nativescript.audiocontext.AudioBuffer, playbackRateId: string);
				public getPlaybackRateParam(): org.nativescript.audiocontext.AudioParam;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public getBuffer(): org.nativescript.audiocontext.AudioBuffer;
				public connect(destId: org.nativescript.audiocontext.AudioNode, this_: number, node: number): void;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public disconnect(destId: org.nativescript.audiocontext.AudioNode, this_: number): void;
				public constructor(id: string, buffer: org.nativescript.audiocontext.AudioBuffer);
				public setBuffer(buffer: org.nativescript.audiocontext.AudioBuffer): void;
				public start(): void;
				public disconnect(): void;
				public disconnect(output: number): void;
				public getLoop(): boolean;
				public setLoop(loop: boolean): void;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public constructor(id: string);
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module audiocontext {
			export class AudioContext {
				public static class: java.lang.Class<org.nativescript.audiocontext.AudioContext>;
				public static getInstance(): org.nativescript.audiocontext.AudioContext;
				public createGain(context: org.nativescript.audiocontext.AudioContextInstance): org.nativescript.audiocontext.GainNode;
				public getAnalyserFloatTimeDomainDataDirect(slice: string, this_: java.nio.FloatBuffer): boolean;
				public attachPeriodicWaveToVoice(voiceId: string, waveId: string): void;
				public createBiquad(context: org.nativescript.audiocontext.AudioContextInstance, type: string, frequency: number, Q: number, gain: number, detune: number): org.nativescript.audiocontext.AudioBiquadNode;
				public createBiquad(context: org.nativescript.audiocontext.AudioContextInstance, type: string, frequency: number, Q: number, gain: number): org.nativescript.audiocontext.AudioBiquadNode;
				public copyFromChannel(this_: string, id: androidNative.Array<number>, dest: number, channel: number): void;
				public suspend(): void;
				public setPannerPartitionSize(panner: org.nativescript.audiocontext.AudioPannerNode, partitionSize: number): void;
				public createExternalPcmSource(sampleRate: number, channels: number): string;
				public createChannelSplitter(context: org.nativescript.audiocontext.AudioContextInstance, numberOfOutputs: number): org.nativescript.audiocontext.ChannelSplitterNode;
				public setPannerParams(panner: org.nativescript.audiocontext.AudioPannerNode, positionX: number, positionY: number, positionZ: number, orientationX: number, orientationY: number, orientationZ: number, pan: number, distanceModel: number, panningModel: number, refDistance: number, maxDistance: number, rolloffFactor: number, coneInnerAngle: number, coneOuterAngle: number, coneOuterGain: number): void;
				public copyFromChannel(id: string, dest: java.nio.ByteBuffer, channel: number, startInChannel: number): void;
				public constructor();
				public addEndedListener(existing: string, list: org.nativescript.audiocontext.AudioContext.EndedListener): void;
				public releaseAnalyser(id: string): void;
				public closeAsync(cb: org.nativescript.audiocontext.AudioContext.AsyncCallback): void;
				public getAnalyserFloatFrequencyData(id: string): androidNative.Array<number>;
				public createConstantSource(context: org.nativescript.audiocontext.AudioContextInstance): org.nativescript.audiocontext.ConstantSourceNode;
				public getContextCurrentTime(this_: string): number;
				public pushPcmFrames(p: string, this_: java.nio.FloatBuffer): void;
				public createOscillator(context: org.nativescript.audiocontext.AudioContextInstance, type: string, frequency: number): org.nativescript.audiocontext.AudioOscillatorNode;
				public createExternalPcmSource(context: org.nativescript.audiocontext.AudioContextInstance, sampleRate: number, channels: number): org.nativescript.audiocontext.ExternalPcmSourceNode;
				public createOscillator(context: org.nativescript.audiocontext.AudioContextInstance): org.nativescript.audiocontext.AudioOscillatorNode;
				public decodeAudioDataFromByteArrayAsync(data: androidNative.Array<number>, context: org.nativescript.audiocontext.AudioContextInstance, cb: org.nativescript.audiocontext.DecodeCallback): void;
				public createConvolver(context: org.nativescript.audiocontext.AudioContextInstance): org.nativescript.audiocontext.ConvolverNode;
				public decodeAudioData(id: string): org.nativescript.audiocontext.AudioBuffer;
				public getNumberOfChannels(id: string): number;
				public createContextInstance(): org.nativescript.audiocontext.AudioContextInstance;
				public getChannelData(id: string, channel: number): java.nio.ByteBuffer;
				public decodeAudioDataFromBufferAsync(buffer: java.nio.ByteBuffer, context: org.nativescript.audiocontext.AudioContextInstance, cb: org.nativescript.audiocontext.DecodeCallback): void;
				public getDestination(context: org.nativescript.audiocontext.AudioContextInstance): org.nativescript.audiocontext.GainNode;
				public static onNativeBufferHeld(id: string): void;
				public createBufferSource(context: org.nativescript.audiocontext.AudioContextInstance, buffer: org.nativescript.audiocontext.AudioBuffer): org.nativescript.audiocontext.AudioBufferSourceNode;
				public removeEndedListener(trackId: string, listener: org.nativescript.audiocontext.AudioContext.EndedListener): void;
				public renderOfflineAsync(trackIds: androidNative.Array<string>, frames: number, sampleRate: number, channels: number, cb: org.nativescript.audiocontext.DecodeCallback): void;
				public createChannelMerger(context: org.nativescript.audiocontext.AudioContextInstance, numberOfInputs: number): org.nativescript.audiocontext.ChannelMergerNode;
				public createPeriodicWave(context: org.nativescript.audiocontext.AudioContextInstance, real: java.nio.FloatBuffer, imag: java.nio.FloatBuffer, disableNormalization: boolean): org.nativescript.audiocontext.PeriodicWave;
				public getAnalyserByteTimeDomainDataDirect(slice: string, this_: java.nio.ByteBuffer): boolean;
				public decodeAudioDataFromFile(len: string): org.nativescript.audiocontext.AudioBuffer;
				public setAnalyserFftSize(id: string, fftSize: number): void;
				public getAudioTimeNanos(): number;
				public getLength(bytesPerSample: string): number;
				public setListenerParams(positionX: number, positionY: number, positionZ: number, forwardX: number, forwardY: number, forwardZ: number, upX: number, upY: number, upZ: number): void;
				public setAnalyserSmoothingTimeConstant(id: string, value: number): void;
				public pushPcmFrames(trackId: string, data: androidNative.Array<number>): void;
				public resumeAsync(cb: org.nativescript.audiocontext.AudioContext.AsyncCallback): void;
				public suspendAsync(cb: org.nativescript.audiocontext.AudioContext.AsyncCallback): void;
				public copyToChannel(id: string, source: java.nio.FloatBuffer, channel: number, startInChannel: number): void;
				public getAnalyserFloatTimeDomainData(id: string, count: number): androidNative.Array<number>;
				public copyToChannel(i: string, byteIndex: androidNative.Array<number>, v: number, i: number): void;
				public createDynamicsCompressor(context: org.nativescript.audiocontext.AudioContextInstance): org.nativescript.audiocontext.DynamicsCompressorNode;
				public getAnalyserByteFrequencyDataDirect(slice: string, this_: java.nio.ByteBuffer, id: number, dest: number): boolean;
				public getContextStartNanos(contextId: string): number;
				public decodeAudioDataFromFileAsync(path: string, context: org.nativescript.audiocontext.AudioContextInstance, cb: org.nativescript.audiocontext.DecodeCallback): void;
				public startBufferSource(trackId: string, loop: boolean): void;
				public createPanner(context: org.nativescript.audiocontext.AudioContextInstance): org.nativescript.audiocontext.AudioPannerNode;
				public decodeAudioDataFromBuffer(slice: java.nio.ByteBuffer): org.nativescript.audiocontext.AudioBuffer;
				public configureExternalPcmSource(trackId: string, sampleRate: number, channels: number): void;
				public registerContextTrack(contextId: string, trackId: string): void;
				public createWaveShaper(context: org.nativescript.audiocontext.AudioContextInstance): org.nativescript.audiocontext.WaveShaperNode;
				public createIIR(context: org.nativescript.audiocontext.AudioContextInstance, feedforward: androidNative.Array<number>, feedback: androidNative.Array<number>): org.nativescript.audiocontext.AudioIIRNode;
				public decodeAudioDataAsync(base64: string, context: org.nativescript.audiocontext.AudioContextInstance, cb: org.nativescript.audiocontext.DecodeCallback): void;
				public getAudioBuffer(id: string): org.nativescript.audiocontext.AudioBuffer;
				public decodeAudioDataFromByteArray(data: androidNative.Array<number>): org.nativescript.audiocontext.AudioBuffer;
				public copyToChannel(id: string, source: java.nio.ByteBuffer, channel: number, startInChannel: number): void;
				public endExternalPcmSource(trackId: string): void;
				public createPeriodicWave(context: org.nativescript.audiocontext.AudioContextInstance, real: androidNative.Array<number>, imag: androidNative.Array<number>, disableNormalization: boolean): org.nativescript.audiocontext.PeriodicWave;
				public resume(): void;
				public setSinkId(t: string): boolean;
				public copyFromChannel(i: string, this_: java.util.List<java.lang.Float>, id: number, dest: number): void;
				public getContextTrackIds(contextId: string): androidNative.Array<string>;
				public static onNativeBufferReleased(id: string): void;
				public setAnalyserDecibels(id: string, minDecibels: number, maxDecibels: number): void;
				public createDelay(context: org.nativescript.audiocontext.AudioContextInstance): org.nativescript.audiocontext.DelayNode;
				public createContextInstance(sampleRate: number, latencyHintSec: number): org.nativescript.audiocontext.AudioContextInstance;
				public createAnalyser(context: org.nativescript.audiocontext.AudioContextInstance): org.nativescript.audiocontext.AnalyserNode;
				public unregisterContextTrack(contextId: string, trackId: string): void;
				public releasePeriodicWave(id: string): void;
				public getSampleRate(id: string): number;
				public stopTrack(id: string): void;
				public startOscillator(id: string, type: string, frequency: number): void;
				public setListenerParams(contextId: string, positionX: number, positionY: number, positionZ: number, forwardX: number, forwardY: number, forwardZ: number, upX: number, upY: number, upZ: number): void;
				public copyFromChannel(id: string, dest: java.nio.FloatBuffer, channel: number, startInChannel: number): void;
				public getAnalyserFloatFrequencyDataDirect(slice: string, this_: java.nio.FloatBuffer): boolean;
				public getDuration(id: string): number;
				public registerBuffer(id: string, bb: java.nio.ByteBuffer, sampleRate: number, channels: number): void;
			}
			export module AudioContext {
				export class AsyncCallback {
					public static class: java.lang.Class<org.nativescript.audiocontext.AudioContext.AsyncCallback>;
					/**
					 * Constructs a new instance of the org.nativescript.audiocontext.AudioContext$AsyncCallback interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
					 */
					public constructor(implementation: { onComplete(param0: boolean): void });
					public constructor();
					public onComplete(param0: boolean): void;
				}
				export class ByteArrayMediaDataSource {
					public static class: java.lang.Class<org.nativescript.audiocontext.AudioContext.ByteArrayMediaDataSource>;
					public close(): void;
					public readAt(position: number, buffer: androidNative.Array<number>, offset: number, size: number): number;
					public getSize(): number;
				}
				export class ByteBufferMediaDataSource {
					public static class: java.lang.Class<org.nativescript.audiocontext.AudioContext.ByteBufferMediaDataSource>;
					public close(): void;
					public readAt(position: number, buffer: androidNative.Array<number>, offset: number, size: number): number;
					public getSize(): number;
				}
				export class EndedListener {
					public static class: java.lang.Class<org.nativescript.audiocontext.AudioContext.EndedListener>;
					/**
					 * Constructs a new instance of the org.nativescript.audiocontext.AudioContext$EndedListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
					 */
					public constructor(implementation: { onEnded(param0: string): void });
					public constructor();
					public onEnded(param0: string): void;
				}
				export class ExtractorSetter {
					public static class: java.lang.Class<org.nativescript.audiocontext.AudioContext.ExtractorSetter>;
					/**
					 * Constructs a new instance of the org.nativescript.audiocontext.AudioContext$ExtractorSetter interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
					 */
					public constructor(implementation: { set(param0: globalAndroid.media.MediaExtractor): void });
					public constructor();
					public set(param0: globalAndroid.media.MediaExtractor): void;
				}
				export class ParamEvent {
					public static class: java.lang.Class<org.nativescript.audiocontext.AudioContext.ParamEvent>;
					public type: number;
					public rate: number;
					public time: number;
					public value: number;
					public constructor(type: number, rate: number, time: number, value: number);
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module audiocontext {
			export class AudioContextInstance {
				public static class: java.lang.Class<org.nativescript.audiocontext.AudioContextInstance>;
				public setSinkId(deviceId: string): boolean;
				public getId(): string;
				public setListenerParams(positionX: number, positionY: number, positionZ: number, forwardX: number, forwardY: number, forwardZ: number, upX: number, upY: number, upZ: number): void;
				public getListenerForwardYParam(): org.nativescript.audiocontext.AudioParam;
				public getListenerUpXParam(): org.nativescript.audiocontext.AudioParam;
				public getListenerUpYParam(): org.nativescript.audiocontext.AudioParam;
				public resumeAsync(cb: org.nativescript.audiocontext.AudioContext.AsyncCallback): void;
				public suspendAsync(cb: org.nativescript.audiocontext.AudioContext.AsyncCallback): void;
				public constructor();
				public createOscillatorNodeFrequency(type: string, frequency: number): org.nativescript.audiocontext.AudioOscillatorNode;
				public getListenerPositionZParam(): org.nativescript.audiocontext.AudioParam;
				public getListenerUpZParam(): org.nativescript.audiocontext.AudioParam;
				public getSampleRate(): number;
				public closeAsync(cb: org.nativescript.audiocontext.AudioContext.AsyncCallback): void;
				public getListenerForwardZParam(): org.nativescript.audiocontext.AudioParam;
				public renderOfflineAsync(frames: number, sampleRate: number, channels: number, cb: org.nativescript.audiocontext.DecodeCallback): void;
				public createSourceNodeFromMediaPlayer(ok: any): org.nativescript.audiocontext.ExternalPcmSourceNode;
				public getListenerPositionYParam(): org.nativescript.audiocontext.AudioParam;
				public getDestination(): org.nativescript.audiocontext.GainNode;
				public release(): void;
				public detachSource(source: org.nativescript.audiocontext.ExternalPcmSourceNode): void;
				public getListenerPositionXParam(): org.nativescript.audiocontext.AudioParam;
				public createExternalPcmSource(sampleRate: number, channels: number): org.nativescript.audiocontext.ExternalPcmSourceNode;
				public getCurrentTime(): number;
				public getListenerForwardXParam(): org.nativescript.audiocontext.AudioParam;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module audiocontext {
			export class AudioIIRNode implements org.nativescript.audiocontext.NativeObject, org.nativescript.audiocontext.AudioNode {
				public static class: java.lang.Class<org.nativescript.audiocontext.AudioIIRNode>;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public connect(param0: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(param0: org.nativescript.audiocontext.AudioNode): void;
				public getId(): string;
				public release(): void;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public connect(node: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public getFrequencyResponse(i: java.nio.FloatBuffer, c: java.nio.FloatBuffer, k: java.nio.FloatBuffer): void;
				public connect(sources: org.nativescript.audiocontext.AudioNode, context: number, object: number): void;
				public disconnect(sources: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(): void;
				public disconnect(output: number): void;
				public getFrequencyResponse(i: androidNative.Array<number>, c: androidNative.Array<number>, k: androidNative.Array<number>): void;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public getFrequencyResponse(i: androidNative.Array<number>, i: androidNative.Array<number>, fD: androidNative.Array<number>): void;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module audiocontext {
			export class AudioNode {
				public static class: java.lang.Class<org.nativescript.audiocontext.AudioNode>;
				/**
				 * Constructs a new instance of the org.nativescript.audiocontext.AudioNode interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
				 */
				public constructor(implementation: { connect(param0: org.nativescript.audiocontext.AudioNode): void; disconnect(param0: org.nativescript.audiocontext.AudioNode): void; connect(node: org.nativescript.audiocontext.AudioNode, output: number): void; connect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void; disconnect(): void; disconnect(output: number): void; disconnect(node: org.nativescript.audiocontext.AudioNode, output: number): void; disconnect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void });
				public constructor();
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public connect(param0: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(param0: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(): void;
				public disconnect(output: number): void;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module audiocontext {
			export class AudioOscillatorNode extends org.nativescript.audiocontext.AudioScheduledSourceNode {
				public static class: java.lang.Class<org.nativescript.audiocontext.AudioOscillatorNode>;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public getType(): string;
				public connect(param0: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(destId: org.nativescript.audiocontext.AudioNode, this_: number, node: number): void;
				public disconnect(param0: org.nativescript.audiocontext.AudioNode): void;
				public constructor(context: org.nativescript.audiocontext.AudioContextInstance);
				public getId(): string;
				public release(): void;
				public setPeriodicWave(pw: org.nativescript.audiocontext.PeriodicWave): void;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public connect(destId: org.nativescript.audiocontext.AudioNode, this_: number, node: number): void;
				public constructor(context: org.nativescript.audiocontext.AudioContextInstance, type: string, frequency: number);
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public disconnect(destId: org.nativescript.audiocontext.AudioNode, this_: number): void;
				public start(): void;
				public disconnect(): void;
				public disconnect(output: number): void;
				public setType(mType: string): void;
				public getFrequency(): org.nativescript.audiocontext.AudioParam;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public constructor(id: string);
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module audiocontext {
			export class AudioPannerNode implements org.nativescript.audiocontext.NativeObject, org.nativescript.audiocontext.AudioNode {
				public static class: java.lang.Class<org.nativescript.audiocontext.AudioPannerNode>;
				public getPositionXParam(): org.nativescript.audiocontext.AudioParam;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public disconnect(param0: org.nativescript.audiocontext.AudioNode): void;
				public getId(): string;
				public getRefDistance(): number;
				public getOrientationYParam(): org.nativescript.audiocontext.AudioParam;
				public setPanningModel(v: number): void;
				public getRolloffFactor(): number;
				public setRefDistance(v: number): void;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public setPosition(x: number, y: number, z: number): void;
				public connect(node: org.nativescript.audiocontext.AudioNode): void;
				public getConeOuterGain(): number;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public setOrientation(x: number, y: number, z: number): void;
				public disconnect(sources: org.nativescript.audiocontext.AudioNode, context: number): void;
				public disconnect(): void;
				public getPositionYParam(): org.nativescript.audiocontext.AudioParam;
				public getOrientationZParam(): org.nativescript.audiocontext.AudioParam;
				public setMaxDistance(v: number): void;
				public setHRTFPartitionSize(size: number): void;
				public getConeOuterAngle(): number;
				public constructor(id: string);
				public setPan(p: number): void;
				public getConeInnerAngle(): number;
				public setDistanceModel(v: number): void;
				public connect(param0: org.nativescript.audiocontext.AudioNode): void;
				public getDistanceModel(): number;
				public connect(sources: org.nativescript.audiocontext.AudioNode, context: number, destId: number): void;
				public setConeInnerAngle(v: number): void;
				public setConeOuterAngle(v: number): void;
				public release(): void;
				public setRolloffFactor(v: number): void;
				public disconnect(sources: org.nativescript.audiocontext.AudioNode, context: number, destId: number): void;
				public setHRTF(left: java.nio.FloatBuffer, right: java.nio.FloatBuffer): void;
				public getPositionZParam(): org.nativescript.audiocontext.AudioParam;
				public getPanningModel(): number;
				public getPan(): org.nativescript.audiocontext.AudioParam;
				public getMaxDistance(): number;
				public disconnect(sources: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(output: number): void;
				public attachToVoice(voiceId: string): void;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public setConeOuterGain(v: number): void;
				public getOrientationXParam(): org.nativescript.audiocontext.AudioParam;
				public detach(): void;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module audiocontext {
			export class AudioParam extends org.nativescript.audiocontext.NativeObject {
				public static class: java.lang.Class<org.nativescript.audiocontext.AudioParam>;
				public getId(): string;
				public getAutomationRateCode(): number;
				public release(): void;
				public getAutomationRate(): string;
				public setValue(px: number): void;
				public setAutomationRate(rate: string): void;
				public cancelAndHoldAtTime(heldValue: number, t: number): void;
				public cancelScheduledValues(t: number): void;
				public linearRampToValueAtTime(v: number, t: number): void;
				public setValueAtTime(v: number, t: number): void;
				public getValue(): number;
				public getValuesForRange(out: number, this_: number, startTime: number): androidNative.Array<number>;
			}
			export module AudioParam {
				export class Type {
					public static class: java.lang.Class<org.nativescript.audiocontext.AudioParam.Type>;
					public static GAIN: org.nativescript.audiocontext.AudioParam.Type;
					public static BIQUAD_FREQUENCY: org.nativescript.audiocontext.AudioParam.Type;
					public static BIQUAD_Q: org.nativescript.audiocontext.AudioParam.Type;
					public static BIQUAD_GAIN: org.nativescript.audiocontext.AudioParam.Type;
					public static BIQUAD_DETUNE: org.nativescript.audiocontext.AudioParam.Type;
					public static PANNER_POSITION_X: org.nativescript.audiocontext.AudioParam.Type;
					public static PANNER_POSITION_Y: org.nativescript.audiocontext.AudioParam.Type;
					public static PANNER_POSITION_Z: org.nativescript.audiocontext.AudioParam.Type;
					public static PANNER_ORIENTATION_X: org.nativescript.audiocontext.AudioParam.Type;
					public static PANNER_ORIENTATION_Y: org.nativescript.audiocontext.AudioParam.Type;
					public static PANNER_ORIENTATION_Z: org.nativescript.audiocontext.AudioParam.Type;
					public static PANNER_PAN: org.nativescript.audiocontext.AudioParam.Type;
					public static LISTENER_POSITION_X: org.nativescript.audiocontext.AudioParam.Type;
					public static LISTENER_POSITION_Y: org.nativescript.audiocontext.AudioParam.Type;
					public static LISTENER_POSITION_Z: org.nativescript.audiocontext.AudioParam.Type;
					public static LISTENER_FORWARD_X: org.nativescript.audiocontext.AudioParam.Type;
					public static LISTENER_FORWARD_Y: org.nativescript.audiocontext.AudioParam.Type;
					public static LISTENER_FORWARD_Z: org.nativescript.audiocontext.AudioParam.Type;
					public static LISTENER_UP_X: org.nativescript.audiocontext.AudioParam.Type;
					public static LISTENER_UP_Y: org.nativescript.audiocontext.AudioParam.Type;
					public static LISTENER_UP_Z: org.nativescript.audiocontext.AudioParam.Type;
					public static values(): androidNative.Array<org.nativescript.audiocontext.AudioParam.Type>;
					public static valueOf(name: string): org.nativescript.audiocontext.AudioParam.Type;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module audiocontext {
			export class AudioScheduledSourceNode implements org.nativescript.audiocontext.NativeObject, org.nativescript.audiocontext.AudioNode {
				public static class: java.lang.Class<org.nativescript.audiocontext.AudioScheduledSourceNode>;
				public id: string;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public connect(param0: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(destId: org.nativescript.audiocontext.AudioNode, this_: number, node: number): void;
				public disconnect(param0: org.nativescript.audiocontext.AudioNode): void;
				public getId(): string;
				public release(): void;
				public addEndedListener(listener: org.nativescript.audiocontext.AudioContext.EndedListener): void;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public connect(destId: org.nativescript.audiocontext.AudioNode, this_: number, node: number): void;
				public connect(node: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(destId: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public disconnect(destId: org.nativescript.audiocontext.AudioNode, this_: number): void;
				public stop(): void;
				public removeEndedListener(listener: org.nativescript.audiocontext.AudioContext.EndedListener): void;
				public start(): void;
				public disconnect(): void;
				public disconnect(output: number): void;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public constructor(id: string);
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module audiocontext {
			export class ChannelMergerNode implements org.nativescript.audiocontext.NativeObject, org.nativescript.audiocontext.AudioNode {
				public static class: java.lang.Class<org.nativescript.audiocontext.ChannelMergerNode>;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public connect(param0: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(param0: org.nativescript.audiocontext.AudioNode): void;
				public getId(): string;
				public release(): void;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public getNumberOfInputs(): number;
				public connect(node: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public connect(sources: org.nativescript.audiocontext.AudioNode, context: number, object: number): void;
				public disconnect(sources: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(sources: org.nativescript.audiocontext.AudioNode, context: number): void;
				public disconnect(): void;
				public disconnect(output: number): void;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public disconnect(sources: org.nativescript.audiocontext.AudioNode, context: number, object: number): void;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module audiocontext {
			export class ChannelSplitterNode implements org.nativescript.audiocontext.NativeObject, org.nativescript.audiocontext.AudioNode {
				public static class: java.lang.Class<org.nativescript.audiocontext.ChannelSplitterNode>;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public connect(param0: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(param0: org.nativescript.audiocontext.AudioNode): void;
				public getId(): string;
				public getNumberOfOutputs(): number;
				public release(): void;
				public connect(sourceOutput: org.nativescript.audiocontext.AudioNode, sources: number, context: number): void;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public disconnect(sourceOutput: org.nativescript.audiocontext.AudioNode, sources: number, context: number): void;
				public connect(node: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public disconnect(sources: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(): void;
				public disconnect(output: number): void;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public disconnect(sourceOutput: org.nativescript.audiocontext.AudioNode, sources: number): void;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module audiocontext {
			export class ConstantSourceNode extends org.nativescript.audiocontext.AudioScheduledSourceNode {
				public static class: java.lang.Class<org.nativescript.audiocontext.ConstantSourceNode>;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public connect(param0: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(destId: org.nativescript.audiocontext.AudioNode, this_: number, node: number): void;
				public disconnect(param0: org.nativescript.audiocontext.AudioNode): void;
				public constructor(context: org.nativescript.audiocontext.AudioContextInstance);
				public getId(): string;
				public release(): void;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public connect(destId: org.nativescript.audiocontext.AudioNode, this_: number, node: number): void;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public getOffset(): number;
				public disconnect(destId: org.nativescript.audiocontext.AudioNode, this_: number): void;
				public stop(): void;
				public getOffsetParam(): org.nativescript.audiocontext.AudioParam;
				public start(): void;
				public disconnect(): void;
				public disconnect(output: number): void;
				public setOffset(v: number): void;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public constructor(id: string);
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module audiocontext {
			export class ConvolverNode implements org.nativescript.audiocontext.NativeObject, org.nativescript.audiocontext.AudioNode {
				public static class: java.lang.Class<org.nativescript.audiocontext.ConvolverNode>;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public connect(param0: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(param0: org.nativescript.audiocontext.AudioNode): void;
				public getId(): string;
				public release(): void;
				public setNormalize(v: boolean): void;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public getBuffer(): org.nativescript.audiocontext.AudioBuffer;
				public setValue(v: number): void;
				public connect(node: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public connect(sources: org.nativescript.audiocontext.AudioNode, context: number, object: number): void;
				public setBuffer(buffer: org.nativescript.audiocontext.AudioBuffer): void;
				public getNormalize(): boolean;
				public disconnect(sources: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(): void;
				public disconnect(output: number): void;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module audiocontext {
			export class DecodeCallback {
				public static class: java.lang.Class<org.nativescript.audiocontext.DecodeCallback>;
				/**
				 * Constructs a new instance of the org.nativescript.audiocontext.DecodeCallback interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
				 */
				public constructor(implementation: { onResult(param0: org.nativescript.audiocontext.AudioBuffer): void });
				public constructor();
				public onResult(param0: org.nativescript.audiocontext.AudioBuffer): void;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module audiocontext {
			export class DelayNode implements org.nativescript.audiocontext.NativeObject, org.nativescript.audiocontext.AudioNode {
				public static class: java.lang.Class<org.nativescript.audiocontext.DelayNode>;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public connect(param0: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(param0: org.nativescript.audiocontext.AudioNode): void;
				public getId(): string;
				public release(): void;
				public getDelayTime(): org.nativescript.audiocontext.AudioParam;
				public getMaxDelayTime(): number;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public connect(node: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public connect(sources: org.nativescript.audiocontext.AudioNode, context: number, object: number): void;
				public disconnect(sources: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(): void;
				public disconnect(output: number): void;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public setMaxDelayTime(v: number): void;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module audiocontext {
			export class DynamicsCompressorNode implements org.nativescript.audiocontext.NativeObject, org.nativescript.audiocontext.AudioNode {
				public static class: java.lang.Class<org.nativescript.audiocontext.DynamicsCompressorNode>;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public connect(param0: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(param0: org.nativescript.audiocontext.AudioNode): void;
				public getId(): string;
				public release(): void;
				public getKnee(): org.nativescript.audiocontext.AudioParam;
				public getThreshold(): org.nativescript.audiocontext.AudioParam;
				public getReduction(): org.nativescript.audiocontext.AudioParam;
				public getRelease(): org.nativescript.audiocontext.AudioParam;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public connect(node: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public connect(sources: org.nativescript.audiocontext.AudioNode, context: number, object: number): void;
				public disconnect(sources: org.nativescript.audiocontext.AudioNode): void;
				public getAttack(): org.nativescript.audiocontext.AudioParam;
				public disconnect(sources: org.nativescript.audiocontext.AudioNode, context: number): void;
				public disconnect(): void;
				public disconnect(output: number): void;
				public getRatio(): org.nativescript.audiocontext.AudioParam;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public disconnect(sources: org.nativescript.audiocontext.AudioNode, context: number, object: number): void;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module audiocontext {
			export class ExternalPcmSourceNode extends org.nativescript.audiocontext.AudioScheduledSourceNode {
				public static class: java.lang.Class<org.nativescript.audiocontext.ExternalPcmSourceNode>;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public connect(param0: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(destId: org.nativescript.audiocontext.AudioNode, this_: number, node: number): void;
				public disconnect(param0: org.nativescript.audiocontext.AudioNode): void;
				public getId(): string;
				public release(): void;
				public pushFrames(data: java.nio.FloatBuffer): void;
				public getPlaybackRateParam(): org.nativescript.audiocontext.AudioParam;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public connect(destId: org.nativescript.audiocontext.AudioNode, this_: number, node: number): void;
				public configureFormat(sampleRate: number, channels: number): void;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public disconnect(destId: org.nativescript.audiocontext.AudioNode, this_: number): void;
				public constructor(id: string, sampleRate: number, channels: number);
				public stop(): void;
				public constructor(id: string, sampleRate: number, channels: number, playbackRateId: string);
				public getSampleRate(): number;
				public disconnect(): void;
				public disconnect(output: number): void;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public getChannels(): number;
				public constructor(id: string);
				public endStream(): void;
				public pushFrames(data: androidNative.Array<number>): void;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module audiocontext {
			export class GainNode implements org.nativescript.audiocontext.NativeObject, org.nativescript.audiocontext.AudioNode {
				public static class: java.lang.Class<org.nativescript.audiocontext.GainNode>;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public connect(param0: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(param0: org.nativescript.audiocontext.AudioNode): void;
				public getId(): string;
				public release(): void;
				public linearRampToValueAtTime(v: number, t: number): void;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public setValueAtTime(v: number, t: number): void;
				public setValue(v: number): void;
				public connect(node: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public connect(sources: org.nativescript.audiocontext.AudioNode, context: number, object: number): void;
				public getGain(): org.nativescript.audiocontext.AudioParam;
				public disconnect(sources: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(sources: org.nativescript.audiocontext.AudioNode, context: number): void;
				public disconnect(): void;
				public disconnect(output: number): void;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public disconnect(sources: org.nativescript.audiocontext.AudioNode, context: number, object: number): void;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module audiocontext {
			export class MediaPlayerAudioTapAdapter {
				public static class: java.lang.Class<org.nativescript.audiocontext.MediaPlayerAudioTapAdapter>;
				public static attachToNode(push: globalAndroid.media.MediaPlayer, end: any): boolean;
				public static detachByNode(externalNode: any): void;
				public static attach(adapter: globalAndroid.media.MediaPlayer, node: any): any;
				public static detach(e: globalAndroid.media.MediaPlayer): void;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module audiocontext {
			export class NativeFinalizer {
				public static class: java.lang.Class<org.nativescript.audiocontext.NativeFinalizer>;
				public static register(wrapper: any, kind: org.nativescript.audiocontext.NativeFinalizer.ResourceKind, id: string): void;
				public constructor();
			}
			export module NativeFinalizer {
				export class ResourceDescriptor {
					public static class: java.lang.Class<org.nativescript.audiocontext.NativeFinalizer.ResourceDescriptor>;
				}
				export class ResourceKind {
					public static class: java.lang.Class<org.nativescript.audiocontext.NativeFinalizer.ResourceKind>;
					public static BUFFER: org.nativescript.audiocontext.NativeFinalizer.ResourceKind;
					public static GAIN: org.nativescript.audiocontext.NativeFinalizer.ResourceKind;
					public static BIQUAD: org.nativescript.audiocontext.NativeFinalizer.ResourceKind;
					public static PANNER: org.nativescript.audiocontext.NativeFinalizer.ResourceKind;
					public static IIR: org.nativescript.audiocontext.NativeFinalizer.ResourceKind;
					public static PERIODICWAVE: org.nativescript.audiocontext.NativeFinalizer.ResourceKind;
					public static values(): androidNative.Array<org.nativescript.audiocontext.NativeFinalizer.ResourceKind>;
					public static valueOf(name: string): org.nativescript.audiocontext.NativeFinalizer.ResourceKind;
				}
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module audiocontext {
			export class NativeObject {
				public static class: java.lang.Class<org.nativescript.audiocontext.NativeObject>;
				/**
				 * Constructs a new instance of the org.nativescript.audiocontext.NativeObject interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
				 */
				public constructor(implementation: { getId(): string; release(): void });
				public constructor();
				public getId(): string;
				public release(): void;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module audiocontext {
			export class PeriodicWave extends org.nativescript.audiocontext.NativeObject {
				public static class: java.lang.Class<org.nativescript.audiocontext.PeriodicWave>;
				public getId(): string;
				public release(): void;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module audiocontext {
			export class StereoPannerNode extends org.nativescript.audiocontext.AudioPannerNode {
				public static class: java.lang.Class<org.nativescript.audiocontext.StereoPannerNode>;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public connect(param0: org.nativescript.audiocontext.AudioNode): void;
				public connect(sources: org.nativescript.audiocontext.AudioNode, context: number, destId: number): void;
				public disconnect(param0: org.nativescript.audiocontext.AudioNode): void;
				public getId(): string;
				public release(): void;
				public disconnect(sources: org.nativescript.audiocontext.AudioNode, context: number, destId: number): void;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public disconnect(sources: org.nativescript.audiocontext.AudioNode, context: number): void;
				public disconnect(): void;
				public disconnect(output: number): void;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
			}
		}
	}
}

declare module org {
	export module nativescript {
		export module audiocontext {
			export class WaveShaperNode implements org.nativescript.audiocontext.NativeObject, org.nativescript.audiocontext.AudioNode {
				public static class: java.lang.Class<org.nativescript.audiocontext.WaveShaperNode>;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public getOversample(): string;
				public connect(param0: org.nativescript.audiocontext.AudioNode): void;
				public setCurve(this_: androidNative.Array<number>): void;
				public setCurveFromData(data: java.nio.ByteBuffer): void;
				public disconnect(param0: org.nativescript.audiocontext.AudioNode): void;
				public getId(): string;
				public release(): void;
				public connect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public setValue(v: number): void;
				public connect(node: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number): void;
				public getCurve(): androidNative.Array<number>;
				public setOversample(v: string): void;
				public connect(sources: org.nativescript.audiocontext.AudioNode, context: number, object: number): void;
				public disconnect(sources: org.nativescript.audiocontext.AudioNode): void;
				public disconnect(): void;
				public disconnect(output: number): void;
				public disconnect(node: org.nativescript.audiocontext.AudioNode, output: number, input: number): void;
				public setCurveFromData(data: java.nio.FloatBuffer): void;
			}
		}
	}
}

//Generics information:
