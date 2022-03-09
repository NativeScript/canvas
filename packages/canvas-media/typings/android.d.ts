/// <reference path="android-declarations.d.ts"/>

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export abstract class AbstractConcatenatedTimeline {
					public static class: java.lang.Class<com.google.android.exoplayer2.AbstractConcatenatedTimeline>;
					public constructor(param0: boolean, param1: com.google.android.exoplayer2.source.ShuffleOrder);
					public getNextWindowIndex(param0: number, param1: number, param2: boolean): number;
					public getPreviousWindowIndex(param0: number, param1: number, param2: boolean): number;
					public getPeriod(param0: number, param1: com.google.android.exoplayer2.Timeline.Period, param2: boolean): com.google.android.exoplayer2.Timeline.Period;
					public static getChildTimelineUidFromConcatenatedUid(param0: any): any;
					public getPeriodByUid(param0: any, param1: com.google.android.exoplayer2.Timeline.Period): com.google.android.exoplayer2.Timeline.Period;
					public getUidOfPeriod(param0: number): any;
					public getWindow(param0: number, param1: com.google.android.exoplayer2.Timeline.Window, param2: number): com.google.android.exoplayer2.Timeline.Window;
					public getLastWindowIndex(param0: boolean): number;
					public static getConcatenatedUid(param0: any, param1: any): any;
					public getChildIndexByPeriodIndex(param0: number): number;
					public getChildIndexByWindowIndex(param0: number): number;
					public getChildIndexByChildUid(param0: any): number;
					public getTimelineByChildIndex(param0: number): com.google.android.exoplayer2.Timeline;
					public getChildUidByChildIndex(param0: number): any;
					public static getChildPeriodUidFromConcatenatedUid(param0: any): any;
					public getFirstWindowIndex(param0: boolean): number;
					public getFirstPeriodIndexByChildIndex(param0: number): number;
					public getIndexOfPeriod(param0: any): number;
					public getFirstWindowIndexByChildIndex(param0: number): number;
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class AudioBecomingNoisyManager {
					public static class: java.lang.Class<com.google.android.exoplayer2.AudioBecomingNoisyManager>;
					public setEnabled(param0: boolean): void;
					public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.os.Handler, param2: com.google.android.exoplayer2.AudioBecomingNoisyManager.EventListener);
				}
				export module AudioBecomingNoisyManager {
					export class AudioBecomingNoisyReceiver {
						public static class: java.lang.Class<com.google.android.exoplayer2.AudioBecomingNoisyManager.AudioBecomingNoisyReceiver>;
						public onReceive(param0: globalAndroid.content.Context, param1: globalAndroid.content.Intent): void;
						public run(): void;
						public constructor(param0: com.google.android.exoplayer2.AudioBecomingNoisyManager, param1: globalAndroid.os.Handler, param2: com.google.android.exoplayer2.AudioBecomingNoisyManager.EventListener);
					}
					export class EventListener {
						public static class: java.lang.Class<com.google.android.exoplayer2.AudioBecomingNoisyManager.EventListener>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.AudioBecomingNoisyManager$EventListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							onAudioBecomingNoisy(): void;
						});
						public constructor();
						public onAudioBecomingNoisy(): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class AudioFocusManager {
					public static class: java.lang.Class<com.google.android.exoplayer2.AudioFocusManager>;
					public static PLAYER_COMMAND_DO_NOT_PLAY: number;
					public static PLAYER_COMMAND_WAIT_FOR_CALLBACK: number;
					public static PLAYER_COMMAND_PLAY_WHEN_READY: number;
					public updateAudioFocus(param0: boolean, param1: number): number;
					public getVolumeMultiplier(): number;
					public release(): void;
					public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.os.Handler, param2: com.google.android.exoplayer2.AudioFocusManager.PlayerControl);
					public setAudioAttributes(param0: com.google.android.exoplayer2.audio.AudioAttributes): void;
				}
				export module AudioFocusManager {
					export class AudioFocusListener {
						public static class: java.lang.Class<com.google.android.exoplayer2.AudioFocusManager.AudioFocusListener>;
						public constructor(param0: com.google.android.exoplayer2.AudioFocusManager, param1: globalAndroid.os.Handler);
						public onAudioFocusChange(param0: number): void;
					}
					export class PlayerCommand {
						public static class: java.lang.Class<com.google.android.exoplayer2.AudioFocusManager.PlayerCommand>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.AudioFocusManager$PlayerCommand interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
						});
						public constructor();
					}
					export class PlayerControl {
						public static class: java.lang.Class<com.google.android.exoplayer2.AudioFocusManager.PlayerControl>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.AudioFocusManager$PlayerControl interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							setVolumeMultiplier(param0: number): void;
							executePlayerCommand(param0: number): void;
						});
						public constructor();
						public executePlayerCommand(param0: number): void;
						public setVolumeMultiplier(param0: number): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export abstract class BaseRenderer implements com.google.android.exoplayer2.Renderer, com.google.android.exoplayer2.RendererCapabilities {
					public static class: java.lang.Class<com.google.android.exoplayer2.BaseRenderer>;
					public isReady(): boolean;
					public init(param0: number, param1: com.google.android.exoplayer2.analytics.PlayerId): void;
					public onStopped(): void;
					public render(param0: number, param1: number): void;
					public static getTunnelingSupport(param0: number): number;
					public getFormatHolder(): com.google.android.exoplayer2.FormatHolder;
					public getStream(): com.google.android.exoplayer2.source.SampleStream;
					public supportsFormat(param0: com.google.android.exoplayer2.Format): number;
					public static create(param0: number): number;
					public disable(): void;
					public getStreamFormats(): androidNative.Array<com.google.android.exoplayer2.Format>;
					public onStreamChanged(param0: androidNative.Array<com.google.android.exoplayer2.Format>, param1: number, param2: number): void;
					public getIndex(): number;
					public setCurrentStreamFinal(): void;
					public getCapabilities(): com.google.android.exoplayer2.RendererCapabilities;
					public hasReadStreamToEnd(): boolean;
					public enable(param0: com.google.android.exoplayer2.RendererConfiguration, param1: androidNative.Array<com.google.android.exoplayer2.Format>, param2: com.google.android.exoplayer2.source.SampleStream, param3: number, param4: boolean, param5: boolean, param6: number, param7: number): void;
					public isCurrentStreamFinal(): boolean;
					public supportsMixedMimeTypeAdaptation(): number;
					public getPlayerId(): com.google.android.exoplayer2.analytics.PlayerId;
					public getMediaClock(): com.google.android.exoplayer2.util.MediaClock;
					public stop(): void;
					public static getFormatSupport(param0: number): number;
					public onStarted(): void;
					public static getDecoderSupport(param0: number): number;
					public getState(): number;
					public getLastResetPositionUs(): number;
					public replaceStream(param0: androidNative.Array<com.google.android.exoplayer2.Format>, param1: com.google.android.exoplayer2.source.SampleStream, param2: number, param3: number): void;
					public getConfiguration(): com.google.android.exoplayer2.RendererConfiguration;
					public isSourceReady(): boolean;
					public static create(param0: number, param1: number, param2: number): number;
					public start(): void;
					public onReset(): void;
					public onDisabled(): void;
					public onEnabled(param0: boolean, param1: boolean): void;
					public resetPosition(param0: number): void;
					public createRendererException(param0: java.lang.Throwable, param1: com.google.android.exoplayer2.Format, param2: boolean, param3: number): com.google.android.exoplayer2.ExoPlaybackException;
					public setPlaybackSpeed(param0: number, param1: number): void;
					public getReadingPositionUs(): number;
					public constructor(param0: number);
					public getTrackType(): number;
					public reset(): void;
					public readSource(param0: com.google.android.exoplayer2.FormatHolder, param1: com.google.android.exoplayer2.decoder.DecoderInputBuffer, param2: number): number;
					public maybeThrowStreamError(): void;
					public isEnded(): boolean;
					public static getAdaptiveSupport(param0: number): number;
					public static getHardwareAccelerationSupport(param0: number): number;
					public static create(param0: number, param1: number, param2: number, param3: number, param4: number): number;
					public handleMessage(param0: number, param1: any): void;
					public getName(): string;
					public createRendererException(param0: java.lang.Throwable, param1: com.google.android.exoplayer2.Format, param2: number): com.google.android.exoplayer2.ExoPlaybackException;
					public skipSource(param0: number): number;
					public onPositionReset(param0: number, param1: boolean): void;
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class DefaultLivePlaybackSpeedControl extends com.google.android.exoplayer2.LivePlaybackSpeedControl {
					public static class: java.lang.Class<com.google.android.exoplayer2.DefaultLivePlaybackSpeedControl>;
					public static DEFAULT_FALLBACK_MIN_PLAYBACK_SPEED: number;
					public static DEFAULT_FALLBACK_MAX_PLAYBACK_SPEED: number;
					public static DEFAULT_MIN_UPDATE_INTERVAL_MS: number;
					public static DEFAULT_PROPORTIONAL_CONTROL_FACTOR: number;
					public static DEFAULT_TARGET_LIVE_OFFSET_INCREMENT_ON_REBUFFER_MS: number;
					public static DEFAULT_MIN_POSSIBLE_LIVE_OFFSET_SMOOTHING_FACTOR: number;
					public static DEFAULT_MAX_LIVE_OFFSET_ERROR_MS_FOR_UNIT_SPEED: number;
					public setTargetLiveOffsetOverrideUs(param0: number): void;
					public notifyRebuffer(): void;
					public getAdjustedPlaybackSpeed(param0: number, param1: number): number;
					public getTargetLiveOffsetUs(): number;
					public setLiveConfiguration(param0: com.google.android.exoplayer2.MediaItem.LiveConfiguration): void;
				}
				export module DefaultLivePlaybackSpeedControl {
					export class Builder {
						public static class: java.lang.Class<com.google.android.exoplayer2.DefaultLivePlaybackSpeedControl.Builder>;
						public setMinUpdateIntervalMs(param0: number): com.google.android.exoplayer2.DefaultLivePlaybackSpeedControl.Builder;
						public setFallbackMinPlaybackSpeed(param0: number): com.google.android.exoplayer2.DefaultLivePlaybackSpeedControl.Builder;
						public constructor();
						public setProportionalControlFactor(param0: number): com.google.android.exoplayer2.DefaultLivePlaybackSpeedControl.Builder;
						public setFallbackMaxPlaybackSpeed(param0: number): com.google.android.exoplayer2.DefaultLivePlaybackSpeedControl.Builder;
						public setMaxLiveOffsetErrorMsForUnitSpeed(param0: number): com.google.android.exoplayer2.DefaultLivePlaybackSpeedControl.Builder;
						public build(): com.google.android.exoplayer2.DefaultLivePlaybackSpeedControl;
						public setMinPossibleLiveOffsetSmoothingFactor(param0: number): com.google.android.exoplayer2.DefaultLivePlaybackSpeedControl.Builder;
						public setTargetLiveOffsetIncrementOnRebufferMs(param0: number): com.google.android.exoplayer2.DefaultLivePlaybackSpeedControl.Builder;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class DefaultLoadControl extends com.google.android.exoplayer2.LoadControl {
					public static class: java.lang.Class<com.google.android.exoplayer2.DefaultLoadControl>;
					public static DEFAULT_MIN_BUFFER_MS: number;
					public static DEFAULT_MAX_BUFFER_MS: number;
					public static DEFAULT_BUFFER_FOR_PLAYBACK_MS: number;
					public static DEFAULT_BUFFER_FOR_PLAYBACK_AFTER_REBUFFER_MS: number;
					public static DEFAULT_TARGET_BUFFER_BYTES: number;
					public static DEFAULT_PRIORITIZE_TIME_OVER_SIZE_THRESHOLDS: boolean;
					public static DEFAULT_BACK_BUFFER_DURATION_MS: number;
					public static DEFAULT_RETAIN_BACK_BUFFER_FROM_KEYFRAME: boolean;
					public static DEFAULT_VIDEO_BUFFER_SIZE: number;
					public static DEFAULT_AUDIO_BUFFER_SIZE: number;
					public static DEFAULT_TEXT_BUFFER_SIZE: number;
					public static DEFAULT_METADATA_BUFFER_SIZE: number;
					public static DEFAULT_CAMERA_MOTION_BUFFER_SIZE: number;
					public static DEFAULT_IMAGE_BUFFER_SIZE: number;
					public static DEFAULT_MUXED_BUFFER_SIZE: number;
					public static DEFAULT_MIN_BUFFER_SIZE: number;
					public onPrepared(): void;
					public onStopped(): void;
					public getBackBufferDurationUs(): number;
					public constructor();
					public retainBackBufferFromKeyframe(): boolean;
					public shouldStartPlayback(param0: number, param1: number, param2: boolean, param3: number): boolean;
					public onTracksSelected(param0: androidNative.Array<com.google.android.exoplayer2.Renderer>, param1: com.google.android.exoplayer2.source.TrackGroupArray, param2: androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection>): void;
					public shouldContinueLoading(param0: number, param1: number, param2: number): boolean;
					public constructor(param0: com.google.android.exoplayer2.upstream.DefaultAllocator, param1: number, param2: number, param3: number, param4: number, param5: number, param6: boolean, param7: number, param8: boolean);
					public calculateTargetBufferBytes(param0: androidNative.Array<com.google.android.exoplayer2.Renderer>, param1: androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection>): number;
					public onReleased(): void;
					public getAllocator(): com.google.android.exoplayer2.upstream.Allocator;
				}
				export module DefaultLoadControl {
					export class Builder {
						public static class: java.lang.Class<com.google.android.exoplayer2.DefaultLoadControl.Builder>;
						public constructor();
						/** @deprecated */
						public createDefaultLoadControl(): com.google.android.exoplayer2.DefaultLoadControl;
						public setAllocator(param0: com.google.android.exoplayer2.upstream.DefaultAllocator): com.google.android.exoplayer2.DefaultLoadControl.Builder;
						public setBackBuffer(param0: number, param1: boolean): com.google.android.exoplayer2.DefaultLoadControl.Builder;
						public build(): com.google.android.exoplayer2.DefaultLoadControl;
						public setBufferDurationsMs(param0: number, param1: number, param2: number, param3: number): com.google.android.exoplayer2.DefaultLoadControl.Builder;
						public setTargetBufferBytes(param0: number): com.google.android.exoplayer2.DefaultLoadControl.Builder;
						public setPrioritizeTimeOverSizeThresholds(param0: boolean): com.google.android.exoplayer2.DefaultLoadControl.Builder;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class DefaultMediaClock extends com.google.android.exoplayer2.util.MediaClock {
					public static class: java.lang.Class<com.google.android.exoplayer2.DefaultMediaClock>;
					public getPositionUs(): number;
					public constructor(param0: com.google.android.exoplayer2.DefaultMediaClock.PlaybackParametersListener, param1: com.google.android.exoplayer2.util.Clock);
					public onRendererEnabled(param0: com.google.android.exoplayer2.Renderer): void;
					public onRendererDisabled(param0: com.google.android.exoplayer2.Renderer): void;
					public syncAndGetPositionUs(param0: boolean): number;
					public resetPosition(param0: number): void;
					public stop(): void;
					public start(): void;
					public setPlaybackParameters(param0: com.google.android.exoplayer2.PlaybackParameters): void;
					public getPlaybackParameters(): com.google.android.exoplayer2.PlaybackParameters;
				}
				export module DefaultMediaClock {
					export class PlaybackParametersListener {
						public static class: java.lang.Class<com.google.android.exoplayer2.DefaultMediaClock.PlaybackParametersListener>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.DefaultMediaClock$PlaybackParametersListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							onPlaybackParametersChanged(param0: com.google.android.exoplayer2.PlaybackParameters): void;
						});
						public constructor();
						public onPlaybackParametersChanged(param0: com.google.android.exoplayer2.PlaybackParameters): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class DefaultRenderersFactory extends com.google.android.exoplayer2.RenderersFactory {
					public static class: java.lang.Class<com.google.android.exoplayer2.DefaultRenderersFactory>;
					public static DEFAULT_ALLOWED_VIDEO_JOINING_TIME_MS: number;
					public static EXTENSION_RENDERER_MODE_OFF: number;
					public static EXTENSION_RENDERER_MODE_ON: number;
					public static EXTENSION_RENDERER_MODE_PREFER: number;
					public static MAX_DROPPED_VIDEO_FRAME_COUNT_TO_NOTIFY: number;
					public buildAudioRenderers(param0: globalAndroid.content.Context, param1: number, param2: com.google.android.exoplayer2.mediacodec.MediaCodecSelector, param3: boolean, param4: com.google.android.exoplayer2.audio.AudioSink, param5: globalAndroid.os.Handler, param6: com.google.android.exoplayer2.audio.AudioRendererEventListener, param7: java.util.ArrayList<com.google.android.exoplayer2.Renderer>): void;
					public buildAudioSink(param0: globalAndroid.content.Context, param1: boolean, param2: boolean, param3: boolean): com.google.android.exoplayer2.audio.AudioSink;
					public getCodecAdapterFactory(): com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.Factory;
					public forceDisableMediaCodecAsynchronousQueueing(): com.google.android.exoplayer2.DefaultRenderersFactory;
					public setEnableAudioTrackPlaybackParams(param0: boolean): com.google.android.exoplayer2.DefaultRenderersFactory;
					public forceEnableMediaCodecAsynchronousQueueing(): com.google.android.exoplayer2.DefaultRenderersFactory;
					public setEnableDecoderFallback(param0: boolean): com.google.android.exoplayer2.DefaultRenderersFactory;
					public setEnableAudioOffload(param0: boolean): com.google.android.exoplayer2.DefaultRenderersFactory;
					public constructor(param0: globalAndroid.content.Context);
					public createRenderers(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.video.VideoRendererEventListener, param2: com.google.android.exoplayer2.audio.AudioRendererEventListener, param3: com.google.android.exoplayer2.text.TextOutput, param4: com.google.android.exoplayer2.metadata.MetadataOutput): androidNative.Array<com.google.android.exoplayer2.Renderer>;
					public setMediaCodecSelector(param0: com.google.android.exoplayer2.mediacodec.MediaCodecSelector): com.google.android.exoplayer2.DefaultRenderersFactory;
					public buildTextRenderers(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.text.TextOutput, param2: globalAndroid.os.Looper, param3: number, param4: java.util.ArrayList<com.google.android.exoplayer2.Renderer>): void;
					public buildMiscellaneousRenderers(param0: globalAndroid.content.Context, param1: globalAndroid.os.Handler, param2: number, param3: java.util.ArrayList<com.google.android.exoplayer2.Renderer>): void;
					public setEnableAudioFloatOutput(param0: boolean): com.google.android.exoplayer2.DefaultRenderersFactory;
					public setExtensionRendererMode(param0: number): com.google.android.exoplayer2.DefaultRenderersFactory;
					public experimentalSetSynchronizeCodecInteractionsWithQueueingEnabled(param0: boolean): com.google.android.exoplayer2.DefaultRenderersFactory;
					public setAllowedVideoJoiningTimeMs(param0: number): com.google.android.exoplayer2.DefaultRenderersFactory;
					public buildVideoRenderers(param0: globalAndroid.content.Context, param1: number, param2: com.google.android.exoplayer2.mediacodec.MediaCodecSelector, param3: boolean, param4: globalAndroid.os.Handler, param5: com.google.android.exoplayer2.video.VideoRendererEventListener, param6: number, param7: java.util.ArrayList<com.google.android.exoplayer2.Renderer>): void;
					public buildCameraMotionRenderers(param0: globalAndroid.content.Context, param1: number, param2: java.util.ArrayList<com.google.android.exoplayer2.Renderer>): void;
					public experimentalSetImmediateCodecStartAfterFlushEnabled(param0: boolean): com.google.android.exoplayer2.DefaultRenderersFactory;
					public buildMetadataRenderers(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.metadata.MetadataOutput, param2: globalAndroid.os.Looper, param3: number, param4: java.util.ArrayList<com.google.android.exoplayer2.Renderer>): void;
				}
				export module DefaultRenderersFactory {
					export class ExtensionRendererMode {
						public static class: java.lang.Class<com.google.android.exoplayer2.DefaultRenderersFactory.ExtensionRendererMode>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.DefaultRenderersFactory$ExtensionRendererMode interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
						});
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class ExoPlaybackException {
					public static class: java.lang.Class<com.google.android.exoplayer2.ExoPlaybackException>;
					public static TYPE_SOURCE: number;
					public static TYPE_RENDERER: number;
					public static TYPE_UNEXPECTED: number;
					public static TYPE_REMOTE: number;
					public type: number;
					public rendererName: string;
					public rendererIndex: number;
					public rendererFormat: com.google.android.exoplayer2.Format;
					public rendererFormatSupport: number;
					public mediaPeriodId: com.google.android.exoplayer2.source.MediaPeriodId;
					public static CREATOR: com.google.android.exoplayer2.Bundleable.Creator<com.google.android.exoplayer2.ExoPlaybackException>;
					public static createForRemote(param0: string): com.google.android.exoplayer2.ExoPlaybackException;
					public errorInfoEquals(param0: com.google.android.exoplayer2.PlaybackException): boolean;
					public getSourceException(): java.io.IOException;
					public getRendererException(): java.lang.Exception;
					public toBundle(): globalAndroid.os.Bundle;
					public getUnexpectedException(): java.lang.RuntimeException;
					public static createForRenderer(param0: java.lang.Throwable, param1: string, param2: number, param3: com.google.android.exoplayer2.Format, param4: number, param5: boolean, param6: number): com.google.android.exoplayer2.ExoPlaybackException;
					/** @deprecated */
					public static createForUnexpected(param0: java.lang.RuntimeException): com.google.android.exoplayer2.ExoPlaybackException;
					public static createForSource(param0: java.io.IOException, param1: number): com.google.android.exoplayer2.ExoPlaybackException;
					public static createForUnexpected(param0: java.lang.RuntimeException, param1: number): com.google.android.exoplayer2.ExoPlaybackException;
				}
				export module ExoPlaybackException {
					export class Type {
						public static class: java.lang.Class<com.google.android.exoplayer2.ExoPlaybackException.Type>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.ExoPlaybackException$Type interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
						});
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class ExoPlayer {
					public static class: java.lang.Class<com.google.android.exoplayer2.ExoPlayer>;
					/**
					 * Constructs a new instance of the com.google.android.exoplayer2.ExoPlayer interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
					 */
					public constructor(implementation: {
						getPlayerError(): com.google.android.exoplayer2.ExoPlaybackException;
						getAudioComponent(): com.google.android.exoplayer2.ExoPlayer.AudioComponent;
						getVideoComponent(): com.google.android.exoplayer2.ExoPlayer.VideoComponent;
						getTextComponent(): com.google.android.exoplayer2.ExoPlayer.TextComponent;
						getDeviceComponent(): com.google.android.exoplayer2.ExoPlayer.DeviceComponent;
						addAudioOffloadListener(param0: com.google.android.exoplayer2.ExoPlayer.AudioOffloadListener): void;
						removeAudioOffloadListener(param0: com.google.android.exoplayer2.ExoPlayer.AudioOffloadListener): void;
						getAnalyticsCollector(): com.google.android.exoplayer2.analytics.AnalyticsCollector;
						addAnalyticsListener(param0: com.google.android.exoplayer2.analytics.AnalyticsListener): void;
						removeAnalyticsListener(param0: com.google.android.exoplayer2.analytics.AnalyticsListener): void;
						getRendererCount(): number;
						getRendererType(param0: number): number;
						getRenderer(param0: number): com.google.android.exoplayer2.Renderer;
						getTrackSelector(): com.google.android.exoplayer2.trackselection.TrackSelector;
						getPlaybackLooper(): globalAndroid.os.Looper;
						getClock(): com.google.android.exoplayer2.util.Clock;
						retry(): void;
						prepare(param0: com.google.android.exoplayer2.source.MediaSource): void;
						prepare(param0: com.google.android.exoplayer2.source.MediaSource, param1: boolean, param2: boolean): void;
						setMediaSources(param0: java.util.List<com.google.android.exoplayer2.source.MediaSource>): void;
						setMediaSources(param0: java.util.List<com.google.android.exoplayer2.source.MediaSource>, param1: boolean): void;
						setMediaSources(param0: java.util.List<com.google.android.exoplayer2.source.MediaSource>, param1: number, param2: number): void;
						setMediaSource(param0: com.google.android.exoplayer2.source.MediaSource): void;
						setMediaSource(param0: com.google.android.exoplayer2.source.MediaSource, param1: number): void;
						setMediaSource(param0: com.google.android.exoplayer2.source.MediaSource, param1: boolean): void;
						addMediaSource(param0: com.google.android.exoplayer2.source.MediaSource): void;
						addMediaSource(param0: number, param1: com.google.android.exoplayer2.source.MediaSource): void;
						addMediaSources(param0: java.util.List<com.google.android.exoplayer2.source.MediaSource>): void;
						addMediaSources(param0: number, param1: java.util.List<com.google.android.exoplayer2.source.MediaSource>): void;
						setShuffleOrder(param0: com.google.android.exoplayer2.source.ShuffleOrder): void;
						setAudioAttributes(param0: com.google.android.exoplayer2.audio.AudioAttributes, param1: boolean): void;
						setAudioSessionId(param0: number): void;
						getAudioSessionId(): number;
						setAuxEffectInfo(param0: com.google.android.exoplayer2.audio.AuxEffectInfo): void;
						clearAuxEffectInfo(): void;
						setSkipSilenceEnabled(param0: boolean): void;
						getSkipSilenceEnabled(): boolean;
						setVideoScalingMode(param0: number): void;
						getVideoScalingMode(): number;
						setVideoChangeFrameRateStrategy(param0: number): void;
						getVideoChangeFrameRateStrategy(): number;
						setVideoFrameMetadataListener(param0: com.google.android.exoplayer2.video.VideoFrameMetadataListener): void;
						clearVideoFrameMetadataListener(param0: com.google.android.exoplayer2.video.VideoFrameMetadataListener): void;
						setCameraMotionListener(param0: com.google.android.exoplayer2.video.spherical.CameraMotionListener): void;
						clearCameraMotionListener(param0: com.google.android.exoplayer2.video.spherical.CameraMotionListener): void;
						createMessage(param0: com.google.android.exoplayer2.PlayerMessage.Target): com.google.android.exoplayer2.PlayerMessage;
						setSeekParameters(param0: com.google.android.exoplayer2.SeekParameters): void;
						getSeekParameters(): com.google.android.exoplayer2.SeekParameters;
						setForegroundMode(param0: boolean): void;
						setPauseAtEndOfMediaItems(param0: boolean): void;
						getPauseAtEndOfMediaItems(): boolean;
						getAudioFormat(): com.google.android.exoplayer2.Format;
						getVideoFormat(): com.google.android.exoplayer2.Format;
						getAudioDecoderCounters(): com.google.android.exoplayer2.decoder.DecoderCounters;
						getVideoDecoderCounters(): com.google.android.exoplayer2.decoder.DecoderCounters;
						setHandleAudioBecomingNoisy(param0: boolean): void;
						setHandleWakeLock(param0: boolean): void;
						setWakeMode(param0: number): void;
						setPriorityTaskManager(param0: com.google.android.exoplayer2.util.PriorityTaskManager): void;
						experimentalSetOffloadSchedulingEnabled(param0: boolean): void;
						experimentalIsSleepingForOffload(): boolean;
						getPlayerError(): com.google.android.exoplayer2.PlaybackException;
					});
					public constructor();
					public static DEFAULT_RELEASE_TIMEOUT_MS: number;
					public static DEFAULT_DETACH_SURFACE_TIMEOUT_MS: number;
					public setAudioSessionId(param0: number): void;
					public getRendererCount(): number;
					public setVideoFrameMetadataListener(param0: com.google.android.exoplayer2.video.VideoFrameMetadataListener): void;
					public setWakeMode(param0: number): void;
					public getPauseAtEndOfMediaItems(): boolean;
					public getAudioFormat(): com.google.android.exoplayer2.Format;
					public clearAuxEffectInfo(): void;
					public removeAudioOffloadListener(param0: com.google.android.exoplayer2.ExoPlayer.AudioOffloadListener): void;
					public getVideoChangeFrameRateStrategy(): number;
					/** @deprecated */
					public getDeviceComponent(): com.google.android.exoplayer2.ExoPlayer.DeviceComponent;
					public createMessage(param0: com.google.android.exoplayer2.PlayerMessage.Target): com.google.android.exoplayer2.PlayerMessage;
					public getClock(): com.google.android.exoplayer2.util.Clock;
					public setAuxEffectInfo(param0: com.google.android.exoplayer2.audio.AuxEffectInfo): void;
					public clearCameraMotionListener(param0: com.google.android.exoplayer2.video.spherical.CameraMotionListener): void;
					public setMediaSources(param0: java.util.List<com.google.android.exoplayer2.source.MediaSource>, param1: number, param2: number): void;
					/** @deprecated */
					public getTextComponent(): com.google.android.exoplayer2.ExoPlayer.TextComponent;
					public setShuffleOrder(param0: com.google.android.exoplayer2.source.ShuffleOrder): void;
					public setSkipSilenceEnabled(param0: boolean): void;
					public getAudioDecoderCounters(): com.google.android.exoplayer2.decoder.DecoderCounters;
					public addAudioOffloadListener(param0: com.google.android.exoplayer2.ExoPlayer.AudioOffloadListener): void;
					public setMediaSources(param0: java.util.List<com.google.android.exoplayer2.source.MediaSource>, param1: boolean): void;
					public getVideoFormat(): com.google.android.exoplayer2.Format;
					public setHandleAudioBecomingNoisy(param0: boolean): void;
					/** @deprecated */
					public getVideoComponent(): com.google.android.exoplayer2.ExoPlayer.VideoComponent;
					public getAudioSessionId(): number;
					public setMediaSource(param0: com.google.android.exoplayer2.source.MediaSource, param1: number): void;
					/** @deprecated */
					public getAudioComponent(): com.google.android.exoplayer2.ExoPlayer.AudioComponent;
					public getPlayerError(): com.google.android.exoplayer2.ExoPlaybackException;
					/** @deprecated */
					public setHandleWakeLock(param0: boolean): void;
					public getSkipSilenceEnabled(): boolean;
					public getSeekParameters(): com.google.android.exoplayer2.SeekParameters;
					public removeAnalyticsListener(param0: com.google.android.exoplayer2.analytics.AnalyticsListener): void;
					public getTrackSelector(): com.google.android.exoplayer2.trackselection.TrackSelector;
					public setForegroundMode(param0: boolean): void;
					public setPriorityTaskManager(param0: com.google.android.exoplayer2.util.PriorityTaskManager): void;
					public addMediaSources(param0: java.util.List<com.google.android.exoplayer2.source.MediaSource>): void;
					public setAudioAttributes(param0: com.google.android.exoplayer2.audio.AudioAttributes, param1: boolean): void;
					public experimentalIsSleepingForOffload(): boolean;
					public getPlaybackLooper(): globalAndroid.os.Looper;
					public setMediaSource(param0: com.google.android.exoplayer2.source.MediaSource): void;
					public addMediaSource(param0: com.google.android.exoplayer2.source.MediaSource): void;
					public setMediaSources(param0: java.util.List<com.google.android.exoplayer2.source.MediaSource>): void;
					public getVideoDecoderCounters(): com.google.android.exoplayer2.decoder.DecoderCounters;
					public addAnalyticsListener(param0: com.google.android.exoplayer2.analytics.AnalyticsListener): void;
					/** @deprecated */
					public prepare(param0: com.google.android.exoplayer2.source.MediaSource, param1: boolean, param2: boolean): void;
					/** @deprecated */
					public retry(): void;
					public addMediaSources(param0: number, param1: java.util.List<com.google.android.exoplayer2.source.MediaSource>): void;
					public getRenderer(param0: number): com.google.android.exoplayer2.Renderer;
					/** @deprecated */
					public prepare(param0: com.google.android.exoplayer2.source.MediaSource): void;
					public addMediaSource(param0: number, param1: com.google.android.exoplayer2.source.MediaSource): void;
					public getAnalyticsCollector(): com.google.android.exoplayer2.analytics.AnalyticsCollector;
					public setVideoChangeFrameRateStrategy(param0: number): void;
					public setVideoScalingMode(param0: number): void;
					public getVideoScalingMode(): number;
					public experimentalSetOffloadSchedulingEnabled(param0: boolean): void;
					public setPauseAtEndOfMediaItems(param0: boolean): void;
					public clearVideoFrameMetadataListener(param0: com.google.android.exoplayer2.video.VideoFrameMetadataListener): void;
					public setMediaSource(param0: com.google.android.exoplayer2.source.MediaSource, param1: boolean): void;
					public setCameraMotionListener(param0: com.google.android.exoplayer2.video.spherical.CameraMotionListener): void;
					public setSeekParameters(param0: com.google.android.exoplayer2.SeekParameters): void;
					public getRendererType(param0: number): number;
				}
				export module ExoPlayer {
					export class AudioComponent {
						public static class: java.lang.Class<com.google.android.exoplayer2.ExoPlayer.AudioComponent>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.ExoPlayer$AudioComponent interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							setAudioAttributes(param0: com.google.android.exoplayer2.audio.AudioAttributes, param1: boolean): void;
							getAudioAttributes(): com.google.android.exoplayer2.audio.AudioAttributes;
							setAudioSessionId(param0: number): void;
							getAudioSessionId(): number;
							setAuxEffectInfo(param0: com.google.android.exoplayer2.audio.AuxEffectInfo): void;
							clearAuxEffectInfo(): void;
							setVolume(param0: number): void;
							getVolume(): number;
							setSkipSilenceEnabled(param0: boolean): void;
							getSkipSilenceEnabled(): boolean;
						});
						public constructor();
						/** @deprecated */
						public setVolume(param0: number): void;
						/** @deprecated */
						public getAudioSessionId(): number;
						/** @deprecated */
						public clearAuxEffectInfo(): void;
						/** @deprecated */
						public setAudioSessionId(param0: number): void;
						/** @deprecated */
						public getVolume(): number;
						/** @deprecated */
						public setAuxEffectInfo(param0: com.google.android.exoplayer2.audio.AuxEffectInfo): void;
						/** @deprecated */
						public getSkipSilenceEnabled(): boolean;
						/** @deprecated */
						public getAudioAttributes(): com.google.android.exoplayer2.audio.AudioAttributes;
						/** @deprecated */
						public setSkipSilenceEnabled(param0: boolean): void;
						/** @deprecated */
						public setAudioAttributes(param0: com.google.android.exoplayer2.audio.AudioAttributes, param1: boolean): void;
					}
					export class AudioOffloadListener {
						public static class: java.lang.Class<com.google.android.exoplayer2.ExoPlayer.AudioOffloadListener>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.ExoPlayer$AudioOffloadListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							onExperimentalOffloadSchedulingEnabledChanged(param0: boolean): void;
							onExperimentalSleepingForOffloadChanged(param0: boolean): void;
						});
						public constructor();
						public onExperimentalSleepingForOffloadChanged(param0: boolean): void;
						public onExperimentalOffloadSchedulingEnabledChanged(param0: boolean): void;
					}
					export class Builder {
						public static class: java.lang.Class<com.google.android.exoplayer2.ExoPlayer.Builder>;
						public constructor(param0: globalAndroid.content.Context);
						public setAnalyticsCollector(param0: com.google.android.exoplayer2.analytics.AnalyticsCollector): com.google.android.exoplayer2.ExoPlayer.Builder;
						public setAudioAttributes(param0: com.google.android.exoplayer2.audio.AudioAttributes, param1: boolean): com.google.android.exoplayer2.ExoPlayer.Builder;
						public setSeekBackIncrementMs(param0: number): com.google.android.exoplayer2.ExoPlayer.Builder;
						public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.RenderersFactory, param2: com.google.android.exoplayer2.source.MediaSource.Factory);
						public setPriorityTaskManager(param0: com.google.android.exoplayer2.util.PriorityTaskManager): com.google.android.exoplayer2.ExoPlayer.Builder;
						public setVideoChangeFrameRateStrategy(param0: number): com.google.android.exoplayer2.ExoPlayer.Builder;
						public build(): com.google.android.exoplayer2.ExoPlayer;
						public setVideoScalingMode(param0: number): com.google.android.exoplayer2.ExoPlayer.Builder;
						public setDetachSurfaceTimeoutMs(param0: number): com.google.android.exoplayer2.ExoPlayer.Builder;
						public setUseLazyPreparation(param0: boolean): com.google.android.exoplayer2.ExoPlayer.Builder;
						public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.source.MediaSource.Factory);
						public setReleaseTimeoutMs(param0: number): com.google.android.exoplayer2.ExoPlayer.Builder;
						public setSeekForwardIncrementMs(param0: number): com.google.android.exoplayer2.ExoPlayer.Builder;
						public setHandleAudioBecomingNoisy(param0: boolean): com.google.android.exoplayer2.ExoPlayer.Builder;
						public setClock(param0: com.google.android.exoplayer2.util.Clock): com.google.android.exoplayer2.ExoPlayer.Builder;
						public setSkipSilenceEnabled(param0: boolean): com.google.android.exoplayer2.ExoPlayer.Builder;
						public setTrackSelector(param0: com.google.android.exoplayer2.trackselection.TrackSelector): com.google.android.exoplayer2.ExoPlayer.Builder;
						public setRenderersFactory(param0: com.google.android.exoplayer2.RenderersFactory): com.google.android.exoplayer2.ExoPlayer.Builder;
						public setWakeMode(param0: number): com.google.android.exoplayer2.ExoPlayer.Builder;
						public setLooper(param0: globalAndroid.os.Looper): com.google.android.exoplayer2.ExoPlayer.Builder;
						public setPauseAtEndOfMediaItems(param0: boolean): com.google.android.exoplayer2.ExoPlayer.Builder;
						public setLoadControl(param0: com.google.android.exoplayer2.LoadControl): com.google.android.exoplayer2.ExoPlayer.Builder;
						public setLivePlaybackSpeedControl(param0: com.google.android.exoplayer2.LivePlaybackSpeedControl): com.google.android.exoplayer2.ExoPlayer.Builder;
						public setMediaSourceFactory(param0: com.google.android.exoplayer2.source.MediaSource.Factory): com.google.android.exoplayer2.ExoPlayer.Builder;
						public setBandwidthMeter(param0: com.google.android.exoplayer2.upstream.BandwidthMeter): com.google.android.exoplayer2.ExoPlayer.Builder;
						public experimentalSetForegroundModeTimeoutMs(param0: number): com.google.android.exoplayer2.ExoPlayer.Builder;
						public setSeekParameters(param0: com.google.android.exoplayer2.SeekParameters): com.google.android.exoplayer2.ExoPlayer.Builder;
						public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.RenderersFactory);
						public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.RenderersFactory, param2: com.google.android.exoplayer2.source.MediaSource.Factory, param3: com.google.android.exoplayer2.trackselection.TrackSelector, param4: com.google.android.exoplayer2.LoadControl, param5: com.google.android.exoplayer2.upstream.BandwidthMeter, param6: com.google.android.exoplayer2.analytics.AnalyticsCollector);
					}
					export class DeviceComponent {
						public static class: java.lang.Class<com.google.android.exoplayer2.ExoPlayer.DeviceComponent>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.ExoPlayer$DeviceComponent interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							getDeviceInfo(): com.google.android.exoplayer2.DeviceInfo;
							getDeviceVolume(): number;
							isDeviceMuted(): boolean;
							setDeviceVolume(param0: number): void;
							increaseDeviceVolume(): void;
							decreaseDeviceVolume(): void;
							setDeviceMuted(param0: boolean): void;
						});
						public constructor();
						/** @deprecated */
						public getDeviceVolume(): number;
						/** @deprecated */
						public increaseDeviceVolume(): void;
						/** @deprecated */
						public isDeviceMuted(): boolean;
						/** @deprecated */
						public decreaseDeviceVolume(): void;
						/** @deprecated */
						public setDeviceMuted(param0: boolean): void;
						/** @deprecated */
						public setDeviceVolume(param0: number): void;
						/** @deprecated */
						public getDeviceInfo(): com.google.android.exoplayer2.DeviceInfo;
					}
					export class TextComponent {
						public static class: java.lang.Class<com.google.android.exoplayer2.ExoPlayer.TextComponent>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.ExoPlayer$TextComponent interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							getCurrentCues(): java.util.List<com.google.android.exoplayer2.text.Cue>;
						});
						public constructor();
						/** @deprecated */
						public getCurrentCues(): java.util.List<com.google.android.exoplayer2.text.Cue>;
					}
					export class VideoComponent {
						public static class: java.lang.Class<com.google.android.exoplayer2.ExoPlayer.VideoComponent>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.ExoPlayer$VideoComponent interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							setVideoScalingMode(param0: number): void;
							getVideoScalingMode(): number;
							setVideoChangeFrameRateStrategy(param0: number): void;
							getVideoChangeFrameRateStrategy(): number;
							setVideoFrameMetadataListener(param0: com.google.android.exoplayer2.video.VideoFrameMetadataListener): void;
							clearVideoFrameMetadataListener(param0: com.google.android.exoplayer2.video.VideoFrameMetadataListener): void;
							setCameraMotionListener(param0: com.google.android.exoplayer2.video.spherical.CameraMotionListener): void;
							clearCameraMotionListener(param0: com.google.android.exoplayer2.video.spherical.CameraMotionListener): void;
							clearVideoSurface(): void;
							clearVideoSurface(param0: globalAndroid.view.Surface): void;
							setVideoSurface(param0: globalAndroid.view.Surface): void;
							setVideoSurfaceHolder(param0: globalAndroid.view.SurfaceHolder): void;
							clearVideoSurfaceHolder(param0: globalAndroid.view.SurfaceHolder): void;
							setVideoSurfaceView(param0: globalAndroid.view.SurfaceView): void;
							clearVideoSurfaceView(param0: globalAndroid.view.SurfaceView): void;
							setVideoTextureView(param0: globalAndroid.view.TextureView): void;
							clearVideoTextureView(param0: globalAndroid.view.TextureView): void;
							getVideoSize(): com.google.android.exoplayer2.video.VideoSize;
						});
						public constructor();
						/** @deprecated */
						public setVideoSurfaceView(param0: globalAndroid.view.SurfaceView): void;
						/** @deprecated */
						public clearVideoSurface(): void;
						/** @deprecated */
						public clearVideoSurface(param0: globalAndroid.view.Surface): void;
						/** @deprecated */
						public setVideoSurfaceHolder(param0: globalAndroid.view.SurfaceHolder): void;
						/** @deprecated */
						public setVideoScalingMode(param0: number): void;
						/** @deprecated */
						public clearVideoSurfaceHolder(param0: globalAndroid.view.SurfaceHolder): void;
						/** @deprecated */
						public setVideoFrameMetadataListener(param0: com.google.android.exoplayer2.video.VideoFrameMetadataListener): void;
						/** @deprecated */
						public getVideoSize(): com.google.android.exoplayer2.video.VideoSize;
						/** @deprecated */
						public clearVideoSurfaceView(param0: globalAndroid.view.SurfaceView): void;
						/** @deprecated */
						public setVideoSurface(param0: globalAndroid.view.Surface): void;
						/** @deprecated */
						public getVideoScalingMode(): number;
						/** @deprecated */
						public clearVideoFrameMetadataListener(param0: com.google.android.exoplayer2.video.VideoFrameMetadataListener): void;
						/** @deprecated */
						public clearVideoTextureView(param0: globalAndroid.view.TextureView): void;
						/** @deprecated */
						public setVideoChangeFrameRateStrategy(param0: number): void;
						/** @deprecated */
						public setCameraMotionListener(param0: com.google.android.exoplayer2.video.spherical.CameraMotionListener): void;
						/** @deprecated */
						public setVideoTextureView(param0: globalAndroid.view.TextureView): void;
						/** @deprecated */
						public getVideoChangeFrameRateStrategy(): number;
						/** @deprecated */
						public clearCameraMotionListener(param0: com.google.android.exoplayer2.video.spherical.CameraMotionListener): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class ExoPlayerImpl implements com.google.android.exoplayer2.ExoPlayer, com.google.android.exoplayer2.ExoPlayer.AudioComponent, com.google.android.exoplayer2.ExoPlayer.VideoComponent, com.google.android.exoplayer2.ExoPlayer.TextComponent, com.google.android.exoplayer2.ExoPlayer.DeviceComponent {
					public static class: java.lang.Class<com.google.android.exoplayer2.ExoPlayerImpl>;
					public setAudioSessionId(param0: number): void;
					public getPlayWhenReady(): boolean;
					public getRendererCount(): number;
					public clearVideoSurface(): void;
					/** @deprecated */
					public setDeviceVolume(param0: number): void;
					public setMediaItems(param0: java.util.List<com.google.android.exoplayer2.MediaItem>, param1: number, param2: number): void;
					/** @deprecated */
					public getSkipSilenceEnabled(): boolean;
					public getAudioFormat(): com.google.android.exoplayer2.Format;
					public getContentPosition(): number;
					public getDeviceVolume(): number;
					public getVideoChangeFrameRateStrategy(): number;
					public createMessage(param0: com.google.android.exoplayer2.PlayerMessage.Target): com.google.android.exoplayer2.PlayerMessage;
					public getClock(): com.google.android.exoplayer2.util.Clock;
					public getDeviceInfo(): com.google.android.exoplayer2.DeviceInfo;
					public clearCameraMotionListener(param0: com.google.android.exoplayer2.video.spherical.CameraMotionListener): void;
					public getPlaybackSuppressionReason(): number;
					public setMediaItems(param0: java.util.List<com.google.android.exoplayer2.MediaItem>, param1: boolean): void;
					public getCurrentPeriodIndex(): number;
					/** @deprecated */
					public setDeviceMuted(param0: boolean): void;
					/** @deprecated */
					public clearVideoFrameMetadataListener(param0: com.google.android.exoplayer2.video.VideoFrameMetadataListener): void;
					/** @deprecated */
					public getTextComponent(): com.google.android.exoplayer2.ExoPlayer.TextComponent;
					public setShuffleOrder(param0: com.google.android.exoplayer2.source.ShuffleOrder): void;
					public getAudioDecoderCounters(): com.google.android.exoplayer2.decoder.DecoderCounters;
					public addAudioOffloadListener(param0: com.google.android.exoplayer2.ExoPlayer.AudioOffloadListener): void;
					public prepare(): void;
					public moveMediaItems(param0: number, param1: number, param2: number): void;
					public setHandleAudioBecomingNoisy(param0: boolean): void;
					/** @deprecated */
					public getVideoComponent(): com.google.android.exoplayer2.ExoPlayer.VideoComponent;
					public clearVideoSurfaceHolder(param0: globalAndroid.view.SurfaceHolder): void;
					public decreaseDeviceVolume(): void;
					public getAudioAttributes(): com.google.android.exoplayer2.audio.AudioAttributes;
					public getPlayerError(): com.google.android.exoplayer2.ExoPlaybackException;
					public setVideoSurfaceView(param0: globalAndroid.view.SurfaceView): void;
					/** @deprecated */
					public setHandleWakeLock(param0: boolean): void;
					public getSkipSilenceEnabled(): boolean;
					public getContentBufferedPosition(): number;
					public getPlaylistMetadata(): com.google.android.exoplayer2.MediaMetadata;
					public removeAnalyticsListener(param0: com.google.android.exoplayer2.analytics.AnalyticsListener): void;
					/** @deprecated */
					public decreaseDeviceVolume(): void;
					public getTrackSelectionParameters(): com.google.android.exoplayer2.trackselection.TrackSelectionParameters;
					/** @deprecated */
					public setVolume(param0: number): void;
					/** @deprecated */
					public getVolume(): number;
					public addMediaSources(param0: java.util.List<com.google.android.exoplayer2.source.MediaSource>): void;
					public setRepeatMode(param0: number): void;
					public constructor(param0: com.google.android.exoplayer2.ExoPlayer.Builder, param1: com.google.android.exoplayer2.Player);
					public setTrackSelectionParameters(param0: com.google.android.exoplayer2.trackselection.TrackSelectionParameters): void;
					public getPlaybackState(): number;
					public getCurrentTimeline(): com.google.android.exoplayer2.Timeline;
					/** @deprecated */
					public clearCameraMotionListener(param0: com.google.android.exoplayer2.video.spherical.CameraMotionListener): void;
					public setPlayWhenReady(param0: boolean): void;
					public getRepeatMode(): number;
					public getMaxSeekToPreviousPosition(): number;
					/** @deprecated */
					public clearVideoSurfaceHolder(param0: globalAndroid.view.SurfaceHolder): void;
					public setMediaSources(param0: java.util.List<com.google.android.exoplayer2.source.MediaSource>): void;
					public getVideoDecoderCounters(): com.google.android.exoplayer2.decoder.DecoderCounters;
					/** @deprecated */
					public setSkipSilenceEnabled(param0: boolean): void;
					/** @deprecated */
					public getVideoSize(): com.google.android.exoplayer2.video.VideoSize;
					public getDuration(): number;
					/** @deprecated */
					public retry(): void;
					/** @deprecated */
					public clearAuxEffectInfo(): void;
					public setDeviceVolume(param0: number): void;
					/** @deprecated */
					public prepare(param0: com.google.android.exoplayer2.source.MediaSource): void;
					public addMediaSource(param0: number, param1: com.google.android.exoplayer2.source.MediaSource): void;
					public clearVideoTextureView(param0: globalAndroid.view.TextureView): void;
					public getAnalyticsCollector(): com.google.android.exoplayer2.analytics.AnalyticsCollector;
					public setVideoChangeFrameRateStrategy(param0: number): void;
					public setVideoScalingMode(param0: number): void;
					public experimentalSetOffloadSchedulingEnabled(param0: boolean): void;
					public getVideoScalingMode(): number;
					public getTotalBufferedDuration(): number;
					/** @deprecated */
					public getAudioAttributes(): com.google.android.exoplayer2.audio.AudioAttributes;
					public getSeekForwardIncrement(): number;
					public isPlayingAd(): boolean;
					/** @deprecated */
					public setCameraMotionListener(param0: com.google.android.exoplayer2.video.spherical.CameraMotionListener): void;
					public clearVideoSurfaceView(param0: globalAndroid.view.SurfaceView): void;
					/** @deprecated */
					public setVideoChangeFrameRateStrategy(param0: number): void;
					/** @deprecated */
					public setVideoSurfaceHolder(param0: globalAndroid.view.SurfaceHolder): void;
					public setMediaSource(param0: com.google.android.exoplayer2.source.MediaSource, param1: boolean): void;
					public setPlaylistMetadata(param0: com.google.android.exoplayer2.MediaMetadata): void;
					public increaseDeviceVolume(): void;
					public setCameraMotionListener(param0: com.google.android.exoplayer2.video.spherical.CameraMotionListener): void;
					/** @deprecated */
					public increaseDeviceVolume(): void;
					public setPlaybackParameters(param0: com.google.android.exoplayer2.PlaybackParameters): void;
					public getCurrentTrackGroups(): com.google.android.exoplayer2.source.TrackGroupArray;
					public getVolume(): number;
					public setVideoFrameMetadataListener(param0: com.google.android.exoplayer2.video.VideoFrameMetadataListener): void;
					public setWakeMode(param0: number): void;
					public addListener(param0: com.google.android.exoplayer2.Player.Listener): void;
					public getPauseAtEndOfMediaItems(): boolean;
					public setVolume(param0: number): void;
					public clearAuxEffectInfo(): void;
					/** @deprecated */
					public clearVideoSurface(): void;
					public getCurrentTracksInfo(): com.google.android.exoplayer2.TracksInfo;
					public getAvailableCommands(): com.google.android.exoplayer2.Player.Commands;
					public removeAudioOffloadListener(param0: com.google.android.exoplayer2.ExoPlayer.AudioOffloadListener): void;
					/** @deprecated */
					public getDeviceComponent(): com.google.android.exoplayer2.ExoPlayer.DeviceComponent;
					public setAuxEffectInfo(param0: com.google.android.exoplayer2.audio.AuxEffectInfo): void;
					public setMediaSources(param0: java.util.List<com.google.android.exoplayer2.source.MediaSource>, param1: number, param2: number): void;
					public getCurrentAdGroupIndex(): number;
					public removeListener(param0: com.google.android.exoplayer2.Player.Listener): void;
					/** @deprecated */
					public setVideoScalingMode(param0: number): void;
					/** @deprecated */
					public clearVideoSurface(param0: globalAndroid.view.Surface): void;
					public setSkipSilenceEnabled(param0: boolean): void;
					/** @deprecated */
					public setVideoSurfaceView(param0: globalAndroid.view.SurfaceView): void;
					/** @deprecated */
					public getCurrentCues(): java.util.List<com.google.android.exoplayer2.text.Cue>;
					public setMediaSources(param0: java.util.List<com.google.android.exoplayer2.source.MediaSource>, param1: boolean): void;
					public getPlaybackParameters(): com.google.android.exoplayer2.PlaybackParameters;
					public getShuffleModeEnabled(): boolean;
					public getVideoFormat(): com.google.android.exoplayer2.Format;
					public getAudioSessionId(): number;
					public setMediaSource(param0: com.google.android.exoplayer2.source.MediaSource, param1: number): void;
					public stop(param0: boolean): void;
					public release(): void;
					public getCurrentAdIndexInAdGroup(): number;
					/** @deprecated */
					public getAudioComponent(): com.google.android.exoplayer2.ExoPlayer.AudioComponent;
					/** @deprecated */
					public setVideoSurface(param0: globalAndroid.view.Surface): void;
					public setHandleWakeLock(param0: boolean): void;
					public getSeekParameters(): com.google.android.exoplayer2.SeekParameters;
					public setShuffleModeEnabled(param0: boolean): void;
					public getTrackSelector(): com.google.android.exoplayer2.trackselection.TrackSelector;
					public stop(): void;
					public setForegroundMode(param0: boolean): void;
					public setPriorityTaskManager(param0: com.google.android.exoplayer2.util.PriorityTaskManager): void;
					public addMediaItems(param0: number, param1: java.util.List<com.google.android.exoplayer2.MediaItem>): void;
					public removeMediaItems(param0: number, param1: number): void;
					public getMediaMetadata(): com.google.android.exoplayer2.MediaMetadata;
					public isDeviceMuted(): boolean;
					public getCurrentMediaItemIndex(): number;
					/** @deprecated */
					public getVideoChangeFrameRateStrategy(): number;
					public setVideoSurface(param0: globalAndroid.view.Surface): void;
					public experimentalIsSleepingForOffload(): boolean;
					public isLoading(): boolean;
					public setAudioAttributes(param0: com.google.android.exoplayer2.audio.AudioAttributes, param1: boolean): void;
					public getPlaybackLooper(): globalAndroid.os.Looper;
					public setMediaSource(param0: com.google.android.exoplayer2.source.MediaSource): void;
					public addMediaSource(param0: com.google.android.exoplayer2.source.MediaSource): void;
					public getVideoSize(): com.google.android.exoplayer2.video.VideoSize;
					/** @deprecated */
					public getVideoScalingMode(): number;
					public getSeekBackIncrement(): number;
					public setVideoTextureView(param0: globalAndroid.view.TextureView): void;
					public setDeviceMuted(param0: boolean): void;
					/** @deprecated */
					public prepare(param0: com.google.android.exoplayer2.source.MediaSource, param1: boolean, param2: boolean): void;
					public clearVideoSurface(param0: globalAndroid.view.Surface): void;
					public addAnalyticsListener(param0: com.google.android.exoplayer2.analytics.AnalyticsListener): void;
					/** @deprecated */
					public setAuxEffectInfo(param0: com.google.android.exoplayer2.audio.AuxEffectInfo): void;
					public getApplicationLooper(): globalAndroid.os.Looper;
					/** @deprecated */
					public clearVideoTextureView(param0: globalAndroid.view.TextureView): void;
					public seekTo(param0: number, param1: number): void;
					public addMediaSources(param0: number, param1: java.util.List<com.google.android.exoplayer2.source.MediaSource>): void;
					/** @deprecated */
					public clearVideoSurfaceView(param0: globalAndroid.view.SurfaceView): void;
					public getCurrentPosition(): number;
					/** @deprecated */
					public setAudioAttributes(param0: com.google.android.exoplayer2.audio.AudioAttributes, param1: boolean): void;
					public getRenderer(param0: number): com.google.android.exoplayer2.Renderer;
					public getCurrentTrackSelections(): com.google.android.exoplayer2.trackselection.TrackSelectionArray;
					public getCurrentCues(): java.util.List<com.google.android.exoplayer2.text.Cue>;
					/** @deprecated */
					public setVideoTextureView(param0: globalAndroid.view.TextureView): void;
					/** @deprecated */
					public getAudioSessionId(): number;
					/** @deprecated */
					public setAudioSessionId(param0: number): void;
					/** @deprecated */
					public getDeviceVolume(): number;
					public setPauseAtEndOfMediaItems(param0: boolean): void;
					public setVideoSurfaceHolder(param0: globalAndroid.view.SurfaceHolder): void;
					public clearVideoFrameMetadataListener(param0: com.google.android.exoplayer2.video.VideoFrameMetadataListener): void;
					/** @deprecated */
					public setVideoFrameMetadataListener(param0: com.google.android.exoplayer2.video.VideoFrameMetadataListener): void;
					/** @deprecated */
					public getDeviceInfo(): com.google.android.exoplayer2.DeviceInfo;
					public setSeekParameters(param0: com.google.android.exoplayer2.SeekParameters): void;
					public getBufferedPosition(): number;
					public getRendererType(param0: number): number;
					/** @deprecated */
					public isDeviceMuted(): boolean;
				}
				export module ExoPlayerImpl {
					export class Api31 {
						public static class: java.lang.Class<com.google.android.exoplayer2.ExoPlayerImpl.Api31>;
						public static createPlayerId(): com.google.android.exoplayer2.analytics.PlayerId;
					}
					export class ComponentListener implements com.google.android.exoplayer2.video.VideoRendererEventListener, com.google.android.exoplayer2.audio.AudioRendererEventListener, com.google.android.exoplayer2.text.TextOutput, com.google.android.exoplayer2.metadata.MetadataOutput, com.google.android.exoplayer2.video.spherical.SphericalGLSurfaceView.VideoSurfaceListener, com.google.android.exoplayer2.AudioFocusManager.PlayerControl, com.google.android.exoplayer2.AudioBecomingNoisyManager.EventListener, com.google.android.exoplayer2.StreamVolumeManager.Listener, com.google.android.exoplayer2.ExoPlayer.AudioOffloadListener {
						public static class: java.lang.Class<com.google.android.exoplayer2.ExoPlayerImpl.ComponentListener>;
						public onAudioEnabled(param0: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onVideoEnabled(param0: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						/** @deprecated */
						public onVideoInputFormatChanged(param0: com.google.android.exoplayer2.Format): void;
						public onAudioInputFormatChanged(param0: com.google.android.exoplayer2.Format, param1: com.google.android.exoplayer2.decoder.DecoderReuseEvaluation): void;
						public onVideoDecoderInitialized(param0: string, param1: number, param2: number): void;
						/** @deprecated */
						public onAudioInputFormatChanged(param0: com.google.android.exoplayer2.Format): void;
						public onSurfaceTextureSizeChanged(param0: globalAndroid.graphics.SurfaceTexture, param1: number, param2: number): void;
						public onStreamTypeChanged(param0: number): void;
						public onVideoDecoderReleased(param0: string): void;
						public onCues(param0: java.util.List<com.google.android.exoplayer2.text.Cue>): void;
						public onMetadata(param0: com.google.android.exoplayer2.metadata.Metadata): void;
						public onVideoSizeChanged(param0: com.google.android.exoplayer2.video.VideoSize): void;
						public onAudioCodecError(param0: java.lang.Exception): void;
						public surfaceDestroyed(param0: globalAndroid.view.SurfaceHolder): void;
						public onVideoSurfaceCreated(param0: globalAndroid.view.Surface): void;
						public setVolumeMultiplier(param0: number): void;
						public onSurfaceTextureUpdated(param0: globalAndroid.graphics.SurfaceTexture): void;
						public onAudioDecoderReleased(param0: string): void;
						public onStreamVolumeChanged(param0: number, param1: boolean): void;
						public onVideoDisabled(param0: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onRenderedFirstFrame(param0: any, param1: number): void;
						public surfaceChanged(param0: globalAndroid.view.SurfaceHolder, param1: number, param2: number, param3: number): void;
						public onVideoCodecError(param0: java.lang.Exception): void;
						public onVideoInputFormatChanged(param0: com.google.android.exoplayer2.Format, param1: com.google.android.exoplayer2.decoder.DecoderReuseEvaluation): void;
						public onAudioBecomingNoisy(): void;
						public onExperimentalSleepingForOffloadChanged(param0: boolean): void;
						public onDroppedFrames(param0: number, param1: number): void;
						public onSurfaceTextureDestroyed(param0: globalAndroid.graphics.SurfaceTexture): boolean;
						public onAudioPositionAdvancing(param0: number): void;
						public onAudioUnderrun(param0: number, param1: number, param2: number): void;
						public onAudioDisabled(param0: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onSurfaceTextureAvailable(param0: globalAndroid.graphics.SurfaceTexture, param1: number, param2: number): void;
						public surfaceCreated(param0: globalAndroid.view.SurfaceHolder): void;
						public onAudioSinkError(param0: java.lang.Exception): void;
						public onVideoSurfaceDestroyed(param0: globalAndroid.view.Surface): void;
						public executePlayerCommand(param0: number): void;
						public onExperimentalOffloadSchedulingEnabledChanged(param0: boolean): void;
						public onSkipSilenceEnabledChanged(param0: boolean): void;
						public onVideoFrameProcessingOffset(param0: number, param1: number): void;
						public onAudioDecoderInitialized(param0: string, param1: number, param2: number): void;
					}
					export class FrameMetadataListener implements com.google.android.exoplayer2.video.VideoFrameMetadataListener, com.google.android.exoplayer2.video.spherical.CameraMotionListener, com.google.android.exoplayer2.PlayerMessage.Target {
						public static class: java.lang.Class<com.google.android.exoplayer2.ExoPlayerImpl.FrameMetadataListener>;
						public static MSG_SET_VIDEO_FRAME_METADATA_LISTENER: number;
						public static MSG_SET_CAMERA_MOTION_LISTENER: number;
						public static MSG_SET_SPHERICAL_SURFACE_VIEW: number;
						public onCameraMotionReset(): void;
						public onVideoFrameAboutToBeRendered(param0: number, param1: number, param2: com.google.android.exoplayer2.Format, param3: globalAndroid.media.MediaFormat): void;
						public handleMessage(param0: number, param1: any): void;
						public onCameraMotion(param0: number, param1: androidNative.Array<number>): void;
					}
					export class MediaSourceHolderSnapshot extends com.google.android.exoplayer2.MediaSourceInfoHolder {
						public static class: java.lang.Class<com.google.android.exoplayer2.ExoPlayerImpl.MediaSourceHolderSnapshot>;
						public constructor(param0: any, param1: com.google.android.exoplayer2.Timeline);
						public getTimeline(): com.google.android.exoplayer2.Timeline;
						public getUid(): any;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class ExoPlayerImplInternal implements com.google.android.exoplayer2.source.MediaPeriod.Callback, com.google.android.exoplayer2.trackselection.TrackSelector.InvalidationListener, com.google.android.exoplayer2.MediaSourceList.MediaSourceListInfoRefreshListener, com.google.android.exoplayer2.DefaultMediaClock.PlaybackParametersListener, com.google.android.exoplayer2.PlayerMessage.Sender {
					public static class: java.lang.Class<com.google.android.exoplayer2.ExoPlayerImplInternal>;
					public onContinueLoadingRequested(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
					public constructor(param0: androidNative.Array<com.google.android.exoplayer2.Renderer>, param1: com.google.android.exoplayer2.trackselection.TrackSelector, param2: com.google.android.exoplayer2.trackselection.TrackSelectorResult, param3: com.google.android.exoplayer2.LoadControl, param4: com.google.android.exoplayer2.upstream.BandwidthMeter, param5: number, param6: boolean, param7: com.google.android.exoplayer2.analytics.AnalyticsCollector, param8: com.google.android.exoplayer2.SeekParameters, param9: com.google.android.exoplayer2.LivePlaybackSpeedControl, param10: number, param11: boolean, param12: globalAndroid.os.Looper, param13: com.google.android.exoplayer2.util.Clock, param14: com.google.android.exoplayer2.ExoPlayerImplInternal.PlaybackInfoUpdateListener, param15: com.google.android.exoplayer2.analytics.PlayerId);
					public getPlaybackLooper(): globalAndroid.os.Looper;
					public setPauseAtEndOfWindow(param0: boolean): void;
					public removeMediaSources(param0: number, param1: number, param2: com.google.android.exoplayer2.source.ShuffleOrder): void;
					public setShuffleOrder(param0: com.google.android.exoplayer2.source.ShuffleOrder): void;
					public sendMessage(param0: com.google.android.exoplayer2.PlayerMessage): void;
					public onPrepared(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
					public seekTo(param0: com.google.android.exoplayer2.Timeline, param1: number, param2: number): void;
					public prepare(): void;
					public onPlaylistUpdateRequested(): void;
					public moveMediaSources(param0: number, param1: number, param2: number, param3: com.google.android.exoplayer2.source.ShuffleOrder): void;
					public experimentalSetForegroundModeTimeoutMs(param0: number): void;
					public onTrackSelectionsInvalidated(): void;
					public setMediaSources(param0: java.util.List<com.google.android.exoplayer2.MediaSourceList.MediaSourceHolder>, param1: number, param2: number, param3: com.google.android.exoplayer2.source.ShuffleOrder): void;
					public experimentalSetOffloadSchedulingEnabled(param0: boolean): void;
					public handleMessage(param0: globalAndroid.os.Message): boolean;
					public addMediaSources(param0: number, param1: java.util.List<com.google.android.exoplayer2.MediaSourceList.MediaSourceHolder>, param2: com.google.android.exoplayer2.source.ShuffleOrder): void;
					public onContinueLoadingRequested(param0: any): void;
					public setPlayWhenReady(param0: boolean, param1: number): void;
					public setForegroundMode(param0: boolean): boolean;
					public onPlaybackParametersChanged(param0: com.google.android.exoplayer2.PlaybackParameters): void;
					public setShuffleModeEnabled(param0: boolean): void;
					public setSeekParameters(param0: com.google.android.exoplayer2.SeekParameters): void;
					public stop(): void;
					public release(): boolean;
					public setRepeatMode(param0: number): void;
					public setPlaybackParameters(param0: com.google.android.exoplayer2.PlaybackParameters): void;
				}
				export module ExoPlayerImplInternal {
					export class MediaSourceListUpdateMessage {
						public static class: java.lang.Class<com.google.android.exoplayer2.ExoPlayerImplInternal.MediaSourceListUpdateMessage>;
					}
					export class MoveMediaItemsMessage {
						public static class: java.lang.Class<com.google.android.exoplayer2.ExoPlayerImplInternal.MoveMediaItemsMessage>;
						public fromIndex: number;
						public toIndex: number;
						public newFromIndex: number;
						public shuffleOrder: com.google.android.exoplayer2.source.ShuffleOrder;
						public constructor(param0: number, param1: number, param2: number, param3: com.google.android.exoplayer2.source.ShuffleOrder);
					}
					export class PendingMessageInfo extends java.lang.Comparable<com.google.android.exoplayer2.ExoPlayerImplInternal.PendingMessageInfo> {
						public static class: java.lang.Class<com.google.android.exoplayer2.ExoPlayerImplInternal.PendingMessageInfo>;
						public message: com.google.android.exoplayer2.PlayerMessage;
						public resolvedPeriodIndex: number;
						public resolvedPeriodTimeUs: number;
						public resolvedPeriodUid: any;
						public compareTo(param0: com.google.android.exoplayer2.ExoPlayerImplInternal.PendingMessageInfo): number;
						public constructor(param0: com.google.android.exoplayer2.PlayerMessage);
						public setResolvedPosition(param0: number, param1: number, param2: any): void;
					}
					export class PlaybackInfoUpdate {
						public static class: java.lang.Class<com.google.android.exoplayer2.ExoPlayerImplInternal.PlaybackInfoUpdate>;
						public playbackInfo: com.google.android.exoplayer2.PlaybackInfo;
						public operationAcks: number;
						public positionDiscontinuity: boolean;
						public discontinuityReason: number;
						public hasPlayWhenReadyChangeReason: boolean;
						public playWhenReadyChangeReason: number;
						public setPlayWhenReadyChangeReason(param0: number): void;
						public incrementPendingOperationAcks(param0: number): void;
						public setPlaybackInfo(param0: com.google.android.exoplayer2.PlaybackInfo): void;
						public setPositionDiscontinuity(param0: number): void;
						public constructor(param0: com.google.android.exoplayer2.PlaybackInfo);
					}
					export class PlaybackInfoUpdateListener {
						public static class: java.lang.Class<com.google.android.exoplayer2.ExoPlayerImplInternal.PlaybackInfoUpdateListener>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.ExoPlayerImplInternal$PlaybackInfoUpdateListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							onPlaybackInfoUpdate(param0: com.google.android.exoplayer2.ExoPlayerImplInternal.PlaybackInfoUpdate): void;
						});
						public constructor();
						public onPlaybackInfoUpdate(param0: com.google.android.exoplayer2.ExoPlayerImplInternal.PlaybackInfoUpdate): void;
					}
					export class PositionUpdateForPlaylistChange {
						public static class: java.lang.Class<com.google.android.exoplayer2.ExoPlayerImplInternal.PositionUpdateForPlaylistChange>;
						public periodId: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId;
						public periodPositionUs: number;
						public requestedContentPositionUs: number;
						public forceBufferingState: boolean;
						public endPlayback: boolean;
						public setTargetLiveOffset: boolean;
						public constructor(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param1: number, param2: number, param3: boolean, param4: boolean, param5: boolean);
					}
					export class SeekPosition {
						public static class: java.lang.Class<com.google.android.exoplayer2.ExoPlayerImplInternal.SeekPosition>;
						public timeline: com.google.android.exoplayer2.Timeline;
						public windowIndex: number;
						public windowPositionUs: number;
						public constructor(param0: com.google.android.exoplayer2.Timeline, param1: number, param2: number);
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class ExoTimeoutException {
					public static class: java.lang.Class<com.google.android.exoplayer2.ExoTimeoutException>;
					public static TIMEOUT_OPERATION_UNDEFINED: number;
					public static TIMEOUT_OPERATION_RELEASE: number;
					public static TIMEOUT_OPERATION_SET_FOREGROUND_MODE: number;
					public static TIMEOUT_OPERATION_DETACH_SURFACE: number;
					public timeoutOperation: number;
					public constructor(param0: number);
				}
				export module ExoTimeoutException {
					export class TimeoutOperation {
						public static class: java.lang.Class<com.google.android.exoplayer2.ExoTimeoutException.TimeoutOperation>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.ExoTimeoutException$TimeoutOperation interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
						});
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class FormatHolder {
					public static class: java.lang.Class<com.google.android.exoplayer2.FormatHolder>;
					public drmSession: com.google.android.exoplayer2.drm.DrmSession;
					public format: com.google.android.exoplayer2.Format;
					public constructor();
					public clear(): void;
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class LivePlaybackSpeedControl {
					public static class: java.lang.Class<com.google.android.exoplayer2.LivePlaybackSpeedControl>;
					/**
					 * Constructs a new instance of the com.google.android.exoplayer2.LivePlaybackSpeedControl interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
					 */
					public constructor(implementation: {
						setLiveConfiguration(param0: com.google.android.exoplayer2.MediaItem.LiveConfiguration): void;
						setTargetLiveOffsetOverrideUs(param0: number): void;
						notifyRebuffer(): void;
						getAdjustedPlaybackSpeed(param0: number, param1: number): number;
						getTargetLiveOffsetUs(): number;
					});
					public constructor();
					public setTargetLiveOffsetOverrideUs(param0: number): void;
					public notifyRebuffer(): void;
					public getAdjustedPlaybackSpeed(param0: number, param1: number): number;
					public getTargetLiveOffsetUs(): number;
					public setLiveConfiguration(param0: com.google.android.exoplayer2.MediaItem.LiveConfiguration): void;
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class LoadControl {
					public static class: java.lang.Class<com.google.android.exoplayer2.LoadControl>;
					/**
					 * Constructs a new instance of the com.google.android.exoplayer2.LoadControl interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
					 */
					public constructor(implementation: {
						onPrepared(): void;
						onTracksSelected(param0: androidNative.Array<com.google.android.exoplayer2.Renderer>, param1: com.google.android.exoplayer2.source.TrackGroupArray, param2: androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection>): void;
						onStopped(): void;
						onReleased(): void;
						getAllocator(): com.google.android.exoplayer2.upstream.Allocator;
						getBackBufferDurationUs(): number;
						retainBackBufferFromKeyframe(): boolean;
						shouldContinueLoading(param0: number, param1: number, param2: number): boolean;
						shouldStartPlayback(param0: number, param1: number, param2: boolean, param3: number): boolean;
					});
					public constructor();
					public onPrepared(): void;
					public onStopped(): void;
					public getBackBufferDurationUs(): number;
					public retainBackBufferFromKeyframe(): boolean;
					public shouldStartPlayback(param0: number, param1: number, param2: boolean, param3: number): boolean;
					public onTracksSelected(param0: androidNative.Array<com.google.android.exoplayer2.Renderer>, param1: com.google.android.exoplayer2.source.TrackGroupArray, param2: androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection>): void;
					public shouldContinueLoading(param0: number, param1: number, param2: number): boolean;
					public onReleased(): void;
					public getAllocator(): com.google.android.exoplayer2.upstream.Allocator;
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class MediaPeriodHolder {
					public static class: java.lang.Class<com.google.android.exoplayer2.MediaPeriodHolder>;
					public mediaPeriod: com.google.android.exoplayer2.source.MediaPeriod;
					public uid: any;
					public sampleStreams: androidNative.Array<com.google.android.exoplayer2.source.SampleStream>;
					public prepared: boolean;
					public hasEnabledTracks: boolean;
					public info: com.google.android.exoplayer2.MediaPeriodInfo;
					public allRenderersInCorrectState: boolean;
					public reevaluateBuffer(param0: number): void;
					public setNext(param0: com.google.android.exoplayer2.MediaPeriodHolder): void;
					public getTrackSelectorResult(): com.google.android.exoplayer2.trackselection.TrackSelectorResult;
					public getStartPositionRendererTime(): number;
					public applyTrackSelection(param0: com.google.android.exoplayer2.trackselection.TrackSelectorResult, param1: number, param2: boolean): number;
					public setRendererOffset(param0: number): void;
					public getTrackGroups(): com.google.android.exoplayer2.source.TrackGroupArray;
					public getBufferedPositionUs(): number;
					public release(): void;
					public handlePrepared(param0: number, param1: com.google.android.exoplayer2.Timeline): void;
					public toRendererTime(param0: number): number;
					public getNext(): com.google.android.exoplayer2.MediaPeriodHolder;
					public getRendererOffset(): number;
					public isFullyBuffered(): boolean;
					public applyTrackSelection(param0: com.google.android.exoplayer2.trackselection.TrackSelectorResult, param1: number, param2: boolean, param3: androidNative.Array<boolean>): number;
					public continueLoading(param0: number): void;
					public updateClipping(): void;
					public getNextLoadPositionUs(): number;
					public toPeriodTime(param0: number): number;
					public constructor(param0: androidNative.Array<com.google.android.exoplayer2.RendererCapabilities>, param1: number, param2: com.google.android.exoplayer2.trackselection.TrackSelector, param3: com.google.android.exoplayer2.upstream.Allocator, param4: com.google.android.exoplayer2.MediaSourceList, param5: com.google.android.exoplayer2.MediaPeriodInfo, param6: com.google.android.exoplayer2.trackselection.TrackSelectorResult);
					public selectTracks(param0: number, param1: com.google.android.exoplayer2.Timeline): com.google.android.exoplayer2.trackselection.TrackSelectorResult;
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class MediaPeriodInfo {
					public static class: java.lang.Class<com.google.android.exoplayer2.MediaPeriodInfo>;
					public id: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId;
					public startPositionUs: number;
					public requestedContentPositionUs: number;
					public endPositionUs: number;
					public durationUs: number;
					public isFollowedByTransitionToSameStream: boolean;
					public isLastInTimelinePeriod: boolean;
					public isLastInTimelineWindow: boolean;
					public isFinal: boolean;
					public equals(param0: any): boolean;
					public copyWithRequestedContentPositionUs(param0: number): com.google.android.exoplayer2.MediaPeriodInfo;
					public hashCode(): number;
					public copyWithStartPositionUs(param0: number): com.google.android.exoplayer2.MediaPeriodInfo;
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class MediaPeriodQueue {
					public static class: java.lang.Class<com.google.android.exoplayer2.MediaPeriodQueue>;
					public static INITIAL_RENDERER_POSITION_OFFSET_US: number;
					public reevaluateBuffer(param0: number): void;
					public enqueueNextMediaPeriodHolder(param0: androidNative.Array<com.google.android.exoplayer2.RendererCapabilities>, param1: com.google.android.exoplayer2.trackselection.TrackSelector, param2: com.google.android.exoplayer2.upstream.Allocator, param3: com.google.android.exoplayer2.MediaSourceList, param4: com.google.android.exoplayer2.MediaPeriodInfo, param5: com.google.android.exoplayer2.trackselection.TrackSelectorResult): com.google.android.exoplayer2.MediaPeriodHolder;
					public updateRepeatMode(param0: com.google.android.exoplayer2.Timeline, param1: number): boolean;
					public updateQueuedPeriods(param0: com.google.android.exoplayer2.Timeline, param1: number, param2: number): boolean;
					public shouldLoadNextMediaPeriod(): boolean;
					public getPlayingPeriod(): com.google.android.exoplayer2.MediaPeriodHolder;
					public advanceReadingPeriod(): com.google.android.exoplayer2.MediaPeriodHolder;
					public resolveMediaPeriodIdForAdsAfterPeriodPositionChange(param0: com.google.android.exoplayer2.Timeline, param1: any, param2: number): com.google.android.exoplayer2.source.MediaSource.MediaPeriodId;
					public getLoadingPeriod(): com.google.android.exoplayer2.MediaPeriodHolder;
					public removeAfter(param0: com.google.android.exoplayer2.MediaPeriodHolder): boolean;
					public isLoading(param0: com.google.android.exoplayer2.source.MediaPeriod): boolean;
					public getReadingPeriod(): com.google.android.exoplayer2.MediaPeriodHolder;
					public resolveMediaPeriodIdForAds(param0: com.google.android.exoplayer2.Timeline, param1: any, param2: number): com.google.android.exoplayer2.source.MediaSource.MediaPeriodId;
					public constructor(param0: com.google.android.exoplayer2.analytics.AnalyticsCollector, param1: globalAndroid.os.Handler);
					public getNextMediaPeriodInfo(param0: number, param1: com.google.android.exoplayer2.PlaybackInfo): com.google.android.exoplayer2.MediaPeriodInfo;
					public updateShuffleModeEnabled(param0: com.google.android.exoplayer2.Timeline, param1: boolean): boolean;
					public clear(): void;
					public getUpdatedMediaPeriodInfo(param0: com.google.android.exoplayer2.Timeline, param1: com.google.android.exoplayer2.MediaPeriodInfo): com.google.android.exoplayer2.MediaPeriodInfo;
					public advancePlayingPeriod(): com.google.android.exoplayer2.MediaPeriodHolder;
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class MediaSourceInfoHolder {
					public static class: java.lang.Class<com.google.android.exoplayer2.MediaSourceInfoHolder>;
					/**
					 * Constructs a new instance of the com.google.android.exoplayer2.MediaSourceInfoHolder interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
					 */
					public constructor(implementation: {
						getUid(): any;
						getTimeline(): com.google.android.exoplayer2.Timeline;
					});
					public constructor();
					public getUid(): any;
					public getTimeline(): com.google.android.exoplayer2.Timeline;
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class MediaSourceList {
					public static class: java.lang.Class<com.google.android.exoplayer2.MediaSourceList>;
					public createPeriod(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param1: com.google.android.exoplayer2.upstream.Allocator, param2: number): com.google.android.exoplayer2.source.MediaPeriod;
					public setMediaSources(param0: java.util.List<com.google.android.exoplayer2.MediaSourceList.MediaSourceHolder>, param1: com.google.android.exoplayer2.source.ShuffleOrder): com.google.android.exoplayer2.Timeline;
					public moveMediaSource(param0: number, param1: number, param2: com.google.android.exoplayer2.source.ShuffleOrder): com.google.android.exoplayer2.Timeline;
					public setShuffleOrder(param0: com.google.android.exoplayer2.source.ShuffleOrder): com.google.android.exoplayer2.Timeline;
					public constructor(param0: com.google.android.exoplayer2.MediaSourceList.MediaSourceListInfoRefreshListener, param1: com.google.android.exoplayer2.analytics.AnalyticsCollector, param2: globalAndroid.os.Handler, param3: com.google.android.exoplayer2.analytics.PlayerId);
					public isPrepared(): boolean;
					public release(): void;
					public getSize(): number;
					public removeMediaSourceRange(param0: number, param1: number, param2: com.google.android.exoplayer2.source.ShuffleOrder): com.google.android.exoplayer2.Timeline;
					public moveMediaSourceRange(param0: number, param1: number, param2: number, param3: com.google.android.exoplayer2.source.ShuffleOrder): com.google.android.exoplayer2.Timeline;
					public clear(param0: com.google.android.exoplayer2.source.ShuffleOrder): com.google.android.exoplayer2.Timeline;
					public prepare(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
					public addMediaSources(param0: number, param1: java.util.List<com.google.android.exoplayer2.MediaSourceList.MediaSourceHolder>, param2: com.google.android.exoplayer2.source.ShuffleOrder): com.google.android.exoplayer2.Timeline;
					public releasePeriod(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
					public createTimeline(): com.google.android.exoplayer2.Timeline;
				}
				export module MediaSourceList {
					export class ForwardingEventListener implements com.google.android.exoplayer2.source.MediaSourceEventListener, com.google.android.exoplayer2.drm.DrmSessionEventListener {
						public static class: java.lang.Class<com.google.android.exoplayer2.MediaSourceList.ForwardingEventListener>;
						public onDownstreamFormatChanged(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
						public onDrmSessionReleased(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
						public onDrmKeysLoaded(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
						public onDrmSessionAcquired(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: number): void;
						public onLoadCanceled(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData): void;
						public onLoadCompleted(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData): void;
						public constructor(param0: com.google.android.exoplayer2.MediaSourceList, param1: com.google.android.exoplayer2.MediaSourceList.MediaSourceHolder);
						public onDrmKeysRestored(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
						public onLoadStarted(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData): void;
						/** @deprecated */
						public onDrmSessionAcquired(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
						public onUpstreamDiscarded(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
						public onLoadError(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData, param4: java.io.IOException, param5: boolean): void;
						public onDrmKeysRemoved(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
						public onDrmSessionManagerError(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: java.lang.Exception): void;
					}
					export class MediaSourceAndListener {
						public static class: java.lang.Class<com.google.android.exoplayer2.MediaSourceList.MediaSourceAndListener>;
						public mediaSource: com.google.android.exoplayer2.source.MediaSource;
						public caller: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller;
						public eventListener: com.google.android.exoplayer2.MediaSourceList.ForwardingEventListener;
						public constructor(param0: com.google.android.exoplayer2.source.MediaSource, param1: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param2: com.google.android.exoplayer2.MediaSourceList.ForwardingEventListener);
					}
					export class MediaSourceHolder extends com.google.android.exoplayer2.MediaSourceInfoHolder {
						public static class: java.lang.Class<com.google.android.exoplayer2.MediaSourceList.MediaSourceHolder>;
						public mediaSource: com.google.android.exoplayer2.source.MaskingMediaSource;
						public uid: any;
						public activeMediaPeriodIds: java.util.List<com.google.android.exoplayer2.source.MediaSource.MediaPeriodId>;
						public firstWindowIndexInChild: number;
						public isRemoved: boolean;
						public reset(param0: number): void;
						public constructor(param0: com.google.android.exoplayer2.source.MediaSource, param1: boolean);
						public getTimeline(): com.google.android.exoplayer2.Timeline;
						public getUid(): any;
					}
					export class MediaSourceListInfoRefreshListener {
						public static class: java.lang.Class<com.google.android.exoplayer2.MediaSourceList.MediaSourceListInfoRefreshListener>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.MediaSourceList$MediaSourceListInfoRefreshListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							onPlaylistUpdateRequested(): void;
						});
						public constructor();
						public onPlaylistUpdateRequested(): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class MetadataRetriever {
					public static class: java.lang.Class<com.google.android.exoplayer2.MetadataRetriever>;
					public static retrieveMetadata(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.MediaItem): com.google.common.util.concurrent.ListenableFuture<com.google.android.exoplayer2.source.TrackGroupArray>;
					public static retrieveMetadata(param0: com.google.android.exoplayer2.source.MediaSource.Factory, param1: com.google.android.exoplayer2.MediaItem): com.google.common.util.concurrent.ListenableFuture<com.google.android.exoplayer2.source.TrackGroupArray>;
				}
				export module MetadataRetriever {
					export class MetadataRetrieverInternal {
						public static class: java.lang.Class<com.google.android.exoplayer2.MetadataRetriever.MetadataRetrieverInternal>;
						public retrieveMetadata(param0: com.google.android.exoplayer2.MediaItem): com.google.common.util.concurrent.ListenableFuture<com.google.android.exoplayer2.source.TrackGroupArray>;
						public constructor(param0: com.google.android.exoplayer2.source.MediaSource.Factory, param1: com.google.android.exoplayer2.util.Clock);
					}
					export module MetadataRetrieverInternal {
						export class MediaSourceHandlerCallback {
							public static class: java.lang.Class<com.google.android.exoplayer2.MetadataRetriever.MetadataRetrieverInternal.MediaSourceHandlerCallback>;
							public handleMessage(param0: globalAndroid.os.Message): boolean;
							public constructor(param0: com.google.android.exoplayer2.MetadataRetriever.MetadataRetrieverInternal);
						}
						export module MediaSourceHandlerCallback {
							export class MediaSourceCaller extends com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller {
								public static class: java.lang.Class<com.google.android.exoplayer2.MetadataRetriever.MetadataRetrieverInternal.MediaSourceHandlerCallback.MediaSourceCaller>;
								public constructor(param0: com.google.android.exoplayer2.MetadataRetriever.MetadataRetrieverInternal.MediaSourceHandlerCallback);
								public onSourceInfoRefreshed(param0: com.google.android.exoplayer2.source.MediaSource, param1: com.google.android.exoplayer2.Timeline): void;
							}
							export module MediaSourceCaller {
								export class MediaPeriodCallback extends com.google.android.exoplayer2.source.MediaPeriod.Callback {
									public static class: java.lang.Class<com.google.android.exoplayer2.MetadataRetriever.MetadataRetrieverInternal.MediaSourceHandlerCallback.MediaSourceCaller.MediaPeriodCallback>;
									public onContinueLoadingRequested(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
									public onContinueLoadingRequested(param0: any): void;
									public onPrepared(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
								}
							}
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export abstract class NoSampleRenderer implements com.google.android.exoplayer2.Renderer, com.google.android.exoplayer2.RendererCapabilities {
					public static class: java.lang.Class<com.google.android.exoplayer2.NoSampleRenderer>;
					public isReady(): boolean;
					public init(param0: number, param1: com.google.android.exoplayer2.analytics.PlayerId): void;
					public onRendererOffsetChanged(param0: number): void;
					public onStopped(): void;
					public render(param0: number, param1: number): void;
					public replaceStream(param0: androidNative.Array<com.google.android.exoplayer2.Format>, param1: com.google.android.exoplayer2.source.SampleStream, param2: number, param3: number): void;
					public getConfiguration(): com.google.android.exoplayer2.RendererConfiguration;
					public static getTunnelingSupport(param0: number): number;
					public static create(param0: number, param1: number, param2: number): number;
					public start(): void;
					public onReset(): void;
					public getStream(): com.google.android.exoplayer2.source.SampleStream;
					public supportsFormat(param0: com.google.android.exoplayer2.Format): number;
					public onDisabled(): void;
					public static create(param0: number): number;
					public disable(): void;
					public constructor();
					public resetPosition(param0: number): void;
					public setPlaybackSpeed(param0: number, param1: number): void;
					public getReadingPositionUs(): number;
					public getTrackType(): number;
					public getIndex(): number;
					public setCurrentStreamFinal(): void;
					public reset(): void;
					public getCapabilities(): com.google.android.exoplayer2.RendererCapabilities;
					public onEnabled(param0: boolean): void;
					public hasReadStreamToEnd(): boolean;
					public enable(param0: com.google.android.exoplayer2.RendererConfiguration, param1: androidNative.Array<com.google.android.exoplayer2.Format>, param2: com.google.android.exoplayer2.source.SampleStream, param3: number, param4: boolean, param5: boolean, param6: number, param7: number): void;
					public maybeThrowStreamError(): void;
					public isEnded(): boolean;
					public static getAdaptiveSupport(param0: number): number;
					public static getHardwareAccelerationSupport(param0: number): number;
					public isCurrentStreamFinal(): boolean;
					public supportsMixedMimeTypeAdaptation(): number;
					public static create(param0: number, param1: number, param2: number, param3: number, param4: number): number;
					public handleMessage(param0: number, param1: any): void;
					public getName(): string;
					public getMediaClock(): com.google.android.exoplayer2.util.MediaClock;
					public stop(): void;
					public onPositionReset(param0: number, param1: boolean): void;
					public static getFormatSupport(param0: number): number;
					public onStarted(): void;
					public static getDecoderSupport(param0: number): number;
					public getState(): number;
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class PlaybackInfo {
					public static class: java.lang.Class<com.google.android.exoplayer2.PlaybackInfo>;
					public timeline: com.google.android.exoplayer2.Timeline;
					public periodId: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId;
					public requestedContentPositionUs: number;
					public discontinuityStartPositionUs: number;
					public playbackState: number;
					public playbackError: com.google.android.exoplayer2.ExoPlaybackException;
					public isLoading: boolean;
					public trackGroups: com.google.android.exoplayer2.source.TrackGroupArray;
					public trackSelectorResult: com.google.android.exoplayer2.trackselection.TrackSelectorResult;
					public staticMetadata: java.util.List<com.google.android.exoplayer2.metadata.Metadata>;
					public loadingMediaPeriodId: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId;
					public playWhenReady: boolean;
					public playbackSuppressionReason: number;
					public playbackParameters: com.google.android.exoplayer2.PlaybackParameters;
					public offloadSchedulingEnabled: boolean;
					public sleepingForOffload: boolean;
					public bufferedPositionUs: number;
					public totalBufferedDurationUs: number;
					public positionUs: number;
					public static getDummyPeriodForEmptyTimeline(): com.google.android.exoplayer2.source.MediaSource.MediaPeriodId;
					public copyWithNewPosition(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param1: number, param2: number, param3: number, param4: number, param5: com.google.android.exoplayer2.source.TrackGroupArray, param6: com.google.android.exoplayer2.trackselection.TrackSelectorResult, param7: java.util.List<com.google.android.exoplayer2.metadata.Metadata>): com.google.android.exoplayer2.PlaybackInfo;
					public copyWithIsLoading(param0: boolean): com.google.android.exoplayer2.PlaybackInfo;
					public copyWithPlaybackError(param0: com.google.android.exoplayer2.ExoPlaybackException): com.google.android.exoplayer2.PlaybackInfo;
					public copyWithPlaybackParameters(param0: com.google.android.exoplayer2.PlaybackParameters): com.google.android.exoplayer2.PlaybackInfo;
					public copyWithLoadingMediaPeriodId(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): com.google.android.exoplayer2.PlaybackInfo;
					public copyWithTimeline(param0: com.google.android.exoplayer2.Timeline): com.google.android.exoplayer2.PlaybackInfo;
					public constructor(param0: com.google.android.exoplayer2.Timeline, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: number, param3: number, param4: number, param5: com.google.android.exoplayer2.ExoPlaybackException, param6: boolean, param7: com.google.android.exoplayer2.source.TrackGroupArray, param8: com.google.android.exoplayer2.trackselection.TrackSelectorResult, param9: java.util.List<com.google.android.exoplayer2.metadata.Metadata>, param10: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param11: boolean, param12: number, param13: com.google.android.exoplayer2.PlaybackParameters, param14: number, param15: number, param16: number, param17: boolean, param18: boolean);
					public copyWithOffloadSchedulingEnabled(param0: boolean): com.google.android.exoplayer2.PlaybackInfo;
					public static createDummy(param0: com.google.android.exoplayer2.trackselection.TrackSelectorResult): com.google.android.exoplayer2.PlaybackInfo;
					public copyWithSleepingForOffload(param0: boolean): com.google.android.exoplayer2.PlaybackInfo;
					public copyWithPlayWhenReady(param0: boolean, param1: number): com.google.android.exoplayer2.PlaybackInfo;
					public copyWithPlaybackState(param0: number): com.google.android.exoplayer2.PlaybackInfo;
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class PlayerMessage {
					public static class: java.lang.Class<com.google.android.exoplayer2.PlayerMessage>;
					public getType(): number;
					public getPositionMs(): number;
					public getTimeline(): com.google.android.exoplayer2.Timeline;
					public cancel(): com.google.android.exoplayer2.PlayerMessage;
					public send(): com.google.android.exoplayer2.PlayerMessage;
					/** @deprecated */
					public setHandler(param0: globalAndroid.os.Handler): com.google.android.exoplayer2.PlayerMessage;
					public blockUntilDelivered(): boolean;
					public getPayload(): any;
					public getMediaItemIndex(): number;
					public markAsProcessed(param0: boolean): void;
					public getLooper(): globalAndroid.os.Looper;
					public isCanceled(): boolean;
					public setPosition(param0: number, param1: number): com.google.android.exoplayer2.PlayerMessage;
					public setPayload(param0: any): com.google.android.exoplayer2.PlayerMessage;
					public setDeleteAfterDelivery(param0: boolean): com.google.android.exoplayer2.PlayerMessage;
					public blockUntilDelivered(param0: number): boolean;
					public setLooper(param0: globalAndroid.os.Looper): com.google.android.exoplayer2.PlayerMessage;
					public setType(param0: number): com.google.android.exoplayer2.PlayerMessage;
					public constructor(param0: com.google.android.exoplayer2.PlayerMessage.Sender, param1: com.google.android.exoplayer2.PlayerMessage.Target, param2: com.google.android.exoplayer2.Timeline, param3: number, param4: com.google.android.exoplayer2.util.Clock, param5: globalAndroid.os.Looper);
					public getDeleteAfterDelivery(): boolean;
					public getTarget(): com.google.android.exoplayer2.PlayerMessage.Target;
					public setPosition(param0: number): com.google.android.exoplayer2.PlayerMessage;
				}
				export module PlayerMessage {
					export class Sender {
						public static class: java.lang.Class<com.google.android.exoplayer2.PlayerMessage.Sender>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.PlayerMessage$Sender interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							sendMessage(param0: com.google.android.exoplayer2.PlayerMessage): void;
						});
						public constructor();
						public sendMessage(param0: com.google.android.exoplayer2.PlayerMessage): void;
					}
					export class Target {
						public static class: java.lang.Class<com.google.android.exoplayer2.PlayerMessage.Target>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.PlayerMessage$Target interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							handleMessage(param0: number, param1: any): void;
						});
						public constructor();
						public handleMessage(param0: number, param1: any): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class PlaylistTimeline extends com.google.android.exoplayer2.AbstractConcatenatedTimeline {
					public static class: java.lang.Class<com.google.android.exoplayer2.PlaylistTimeline>;
					public getChildIndexByPeriodIndex(param0: number): number;
					public getChildIndexByWindowIndex(param0: number): number;
					public getWindowCount(): number;
					public getChildIndexByChildUid(param0: any): number;
					public getTimelineByChildIndex(param0: number): com.google.android.exoplayer2.Timeline;
					public constructor(param0: boolean, param1: com.google.android.exoplayer2.source.ShuffleOrder);
					public getChildUidByChildIndex(param0: number): any;
					public getFirstPeriodIndexByChildIndex(param0: number): number;
					public constructor(param0: java.util.Collection<any>, param1: com.google.android.exoplayer2.source.ShuffleOrder);
					public getPeriodCount(): number;
					public getFirstWindowIndexByChildIndex(param0: number): number;
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class Renderer extends com.google.android.exoplayer2.PlayerMessage.Target {
					public static class: java.lang.Class<com.google.android.exoplayer2.Renderer>;
					/**
					 * Constructs a new instance of the com.google.android.exoplayer2.Renderer interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
					 */
					public constructor(implementation: {
						getName(): string;
						getTrackType(): number;
						getCapabilities(): com.google.android.exoplayer2.RendererCapabilities;
						init(param0: number, param1: com.google.android.exoplayer2.analytics.PlayerId): void;
						getMediaClock(): com.google.android.exoplayer2.util.MediaClock;
						getState(): number;
						enable(param0: com.google.android.exoplayer2.RendererConfiguration, param1: androidNative.Array<com.google.android.exoplayer2.Format>, param2: com.google.android.exoplayer2.source.SampleStream, param3: number, param4: boolean, param5: boolean, param6: number, param7: number): void;
						start(): void;
						replaceStream(param0: androidNative.Array<com.google.android.exoplayer2.Format>, param1: com.google.android.exoplayer2.source.SampleStream, param2: number, param3: number): void;
						getStream(): com.google.android.exoplayer2.source.SampleStream;
						hasReadStreamToEnd(): boolean;
						getReadingPositionUs(): number;
						setCurrentStreamFinal(): void;
						isCurrentStreamFinal(): boolean;
						maybeThrowStreamError(): void;
						resetPosition(param0: number): void;
						setPlaybackSpeed(param0: number, param1: number): void;
						render(param0: number, param1: number): void;
						isReady(): boolean;
						isEnded(): boolean;
						stop(): void;
						disable(): void;
						reset(): void;
						handleMessage(param0: number, param1: any): void;
					});
					public constructor();
					public static MSG_SET_AUDIO_SESSION_ID: number;
					public static MSG_SET_SKIP_SILENCE_ENABLED: number;
					public static STATE_DISABLED: number;
					public static MSG_CUSTOM_BASE: number;
					public static MSG_SET_VIDEO_OUTPUT: number;
					public static MSG_SET_WAKEUP_LISTENER: number;
					public static STATE_STARTED: number;
					public static MSG_SET_SCALING_MODE: number;
					public static MSG_SET_AUDIO_ATTRIBUTES: number;
					public static MSG_SET_CHANGE_FRAME_RATE_STRATEGY: number;
					public static STATE_ENABLED: number;
					public static MSG_SET_VOLUME: number;
					public static MSG_SET_AUX_EFFECT_INFO: number;
					public static MSG_SET_VIDEO_FRAME_METADATA_LISTENER: number;
					public static MSG_SET_CAMERA_MOTION_LISTENER: number;
					public isReady(): boolean;
					public setCurrentStreamFinal(): void;
					public reset(): void;
					public getCapabilities(): com.google.android.exoplayer2.RendererCapabilities;
					public init(param0: number, param1: com.google.android.exoplayer2.analytics.PlayerId): void;
					public render(param0: number, param1: number): void;
					public hasReadStreamToEnd(): boolean;
					public replaceStream(param0: androidNative.Array<com.google.android.exoplayer2.Format>, param1: com.google.android.exoplayer2.source.SampleStream, param2: number, param3: number): void;
					public enable(param0: com.google.android.exoplayer2.RendererConfiguration, param1: androidNative.Array<com.google.android.exoplayer2.Format>, param2: com.google.android.exoplayer2.source.SampleStream, param3: number, param4: boolean, param5: boolean, param6: number, param7: number): void;
					public maybeThrowStreamError(): void;
					public isEnded(): boolean;
					public start(): void;
					public isCurrentStreamFinal(): boolean;
					public getStream(): com.google.android.exoplayer2.source.SampleStream;
					public disable(): void;
					public getName(): string;
					public handleMessage(param0: number, param1: any): void;
					public resetPosition(param0: number): void;
					public getMediaClock(): com.google.android.exoplayer2.util.MediaClock;
					public stop(): void;
					public setPlaybackSpeed(param0: number, param1: number): void;
					public getReadingPositionUs(): number;
					public getTrackType(): number;
					public getState(): number;
				}
				export module Renderer {
					export class MessageType {
						public static class: java.lang.Class<com.google.android.exoplayer2.Renderer.MessageType>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.Renderer$MessageType interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
						});
						public constructor();
					}
					export class State {
						public static class: java.lang.Class<com.google.android.exoplayer2.Renderer.State>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.Renderer$State interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
						});
						public constructor();
					}
					export class WakeupListener {
						public static class: java.lang.Class<com.google.android.exoplayer2.Renderer.WakeupListener>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.Renderer$WakeupListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							onSleep(param0: number): void;
							onWakeup(): void;
						});
						public constructor();
						public onSleep(param0: number): void;
						public onWakeup(): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class RendererCapabilities {
					public static class: java.lang.Class<com.google.android.exoplayer2.RendererCapabilities>;
					/**
					 * Constructs a new instance of the com.google.android.exoplayer2.RendererCapabilities interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
					 */
					public constructor(implementation: {
						create(param0: number): number;
						create(param0: number, param1: number, param2: number): number;
						create(param0: number, param1: number, param2: number, param3: number, param4: number): number;
						getFormatSupport(param0: number): number;
						getAdaptiveSupport(param0: number): number;
						getTunnelingSupport(param0: number): number;
						getHardwareAccelerationSupport(param0: number): number;
						getDecoderSupport(param0: number): number;
						getName(): string;
						getTrackType(): number;
						supportsFormat(param0: com.google.android.exoplayer2.Format): number;
						supportsMixedMimeTypeAdaptation(): number;
					});
					public constructor();
					public static TUNNELING_SUPPORT_MASK: number;
					public static ADAPTIVE_SUPPORT_MASK: number;
					public static FORMAT_SUPPORT_MASK: number;
					public static FORMAT_UNSUPPORTED_DRM: number;
					public static ADAPTIVE_SEAMLESS: number;
					public static HARDWARE_ACCELERATION_SUPPORT_MASK: number;
					public static HARDWARE_ACCELERATION_SUPPORTED: number;
					public static DECODER_SUPPORT_FALLBACK: number;
					public static TUNNELING_NOT_SUPPORTED: number;
					public static HARDWARE_ACCELERATION_NOT_SUPPORTED: number;
					public static MODE_SUPPORT_MASK: number;
					public static DECODER_SUPPORT_PRIMARY: number;
					public static FORMAT_UNSUPPORTED_TYPE: number;
					public static ADAPTIVE_NOT_SEAMLESS: number;
					public static ADAPTIVE_NOT_SUPPORTED: number;
					public static FORMAT_EXCEEDS_CAPABILITIES: number;
					public static FORMAT_HANDLED: number;
					public static TUNNELING_SUPPORTED: number;
					public static FORMAT_UNSUPPORTED_SUBTYPE: number;
					public static create(param0: number): number;
					public getName(): string;
					public static getTunnelingSupport(param0: number): number;
					public static create(param0: number, param1: number, param2: number): number;
					public static getFormatSupport(param0: number): number;
					public static getAdaptiveSupport(param0: number): number;
					public static getHardwareAccelerationSupport(param0: number): number;
					public static getDecoderSupport(param0: number): number;
					public getTrackType(): number;
					public supportsMixedMimeTypeAdaptation(): number;
					public static create(param0: number, param1: number, param2: number, param3: number, param4: number): number;
					public supportsFormat(param0: com.google.android.exoplayer2.Format): number;
				}
				export module RendererCapabilities {
					export class AdaptiveSupport {
						public static class: java.lang.Class<com.google.android.exoplayer2.RendererCapabilities.AdaptiveSupport>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.RendererCapabilities$AdaptiveSupport interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
						});
						public constructor();
					}
					export class Capabilities {
						public static class: java.lang.Class<com.google.android.exoplayer2.RendererCapabilities.Capabilities>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.RendererCapabilities$Capabilities interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
						});
						public constructor();
					}
					export class DecoderSupport {
						public static class: java.lang.Class<com.google.android.exoplayer2.RendererCapabilities.DecoderSupport>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.RendererCapabilities$DecoderSupport interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
						});
						public constructor();
					}
					export class FormatSupport {
						public static class: java.lang.Class<com.google.android.exoplayer2.RendererCapabilities.FormatSupport>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.RendererCapabilities$FormatSupport interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
						});
						public constructor();
					}
					export class HardwareAccelerationSupport {
						public static class: java.lang.Class<com.google.android.exoplayer2.RendererCapabilities.HardwareAccelerationSupport>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.RendererCapabilities$HardwareAccelerationSupport interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
						});
						public constructor();
					}
					export class TunnelingSupport {
						public static class: java.lang.Class<com.google.android.exoplayer2.RendererCapabilities.TunnelingSupport>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.RendererCapabilities$TunnelingSupport interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
						});
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class RendererConfiguration {
					public static class: java.lang.Class<com.google.android.exoplayer2.RendererConfiguration>;
					public static DEFAULT: com.google.android.exoplayer2.RendererConfiguration;
					public tunneling: boolean;
					public equals(param0: any): boolean;
					public constructor(param0: boolean);
					public hashCode(): number;
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class RenderersFactory {
					public static class: java.lang.Class<com.google.android.exoplayer2.RenderersFactory>;
					/**
					 * Constructs a new instance of the com.google.android.exoplayer2.RenderersFactory interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
					 */
					public constructor(implementation: {
						createRenderers(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.video.VideoRendererEventListener, param2: com.google.android.exoplayer2.audio.AudioRendererEventListener, param3: com.google.android.exoplayer2.text.TextOutput, param4: com.google.android.exoplayer2.metadata.MetadataOutput): androidNative.Array<com.google.android.exoplayer2.Renderer>;
					});
					public constructor();
					public createRenderers(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.video.VideoRendererEventListener, param2: com.google.android.exoplayer2.audio.AudioRendererEventListener, param3: com.google.android.exoplayer2.text.TextOutput, param4: com.google.android.exoplayer2.metadata.MetadataOutput): androidNative.Array<com.google.android.exoplayer2.Renderer>;
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class SeekParameters {
					public static class: java.lang.Class<com.google.android.exoplayer2.SeekParameters>;
					public static EXACT: com.google.android.exoplayer2.SeekParameters;
					public static CLOSEST_SYNC: com.google.android.exoplayer2.SeekParameters;
					public static PREVIOUS_SYNC: com.google.android.exoplayer2.SeekParameters;
					public static NEXT_SYNC: com.google.android.exoplayer2.SeekParameters;
					public static DEFAULT: com.google.android.exoplayer2.SeekParameters;
					public toleranceBeforeUs: number;
					public toleranceAfterUs: number;
					public equals(param0: any): boolean;
					public hashCode(): number;
					public resolveSeekPositionUs(param0: number, param1: number, param2: number): number;
					public constructor(param0: number, param1: number);
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class SimpleExoPlayer implements com.google.android.exoplayer2.ExoPlayer, com.google.android.exoplayer2.ExoPlayer.AudioComponent, com.google.android.exoplayer2.ExoPlayer.VideoComponent, com.google.android.exoplayer2.ExoPlayer.TextComponent, com.google.android.exoplayer2.ExoPlayer.DeviceComponent {
					public static class: java.lang.Class<com.google.android.exoplayer2.SimpleExoPlayer>;
					public setAudioSessionId(param0: number): void;
					public getPlayWhenReady(): boolean;
					public getRendererCount(): number;
					public clearVideoSurface(): void;
					/** @deprecated */
					public setDeviceVolume(param0: number): void;
					public setMediaItems(param0: java.util.List<com.google.android.exoplayer2.MediaItem>, param1: number, param2: number): void;
					/** @deprecated */
					public getSkipSilenceEnabled(): boolean;
					/** @deprecated */
					public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.RenderersFactory, param2: com.google.android.exoplayer2.trackselection.TrackSelector, param3: com.google.android.exoplayer2.source.MediaSource.Factory, param4: com.google.android.exoplayer2.LoadControl, param5: com.google.android.exoplayer2.upstream.BandwidthMeter, param6: com.google.android.exoplayer2.analytics.AnalyticsCollector, param7: boolean, param8: com.google.android.exoplayer2.util.Clock, param9: globalAndroid.os.Looper);
					public constructor(param0: com.google.android.exoplayer2.SimpleExoPlayer.Builder);
					public getAudioFormat(): com.google.android.exoplayer2.Format;
					public getContentPosition(): number;
					public getDeviceVolume(): number;
					public getVideoChangeFrameRateStrategy(): number;
					public createMessage(param0: com.google.android.exoplayer2.PlayerMessage.Target): com.google.android.exoplayer2.PlayerMessage;
					public getClock(): com.google.android.exoplayer2.util.Clock;
					public getDeviceInfo(): com.google.android.exoplayer2.DeviceInfo;
					public clearCameraMotionListener(param0: com.google.android.exoplayer2.video.spherical.CameraMotionListener): void;
					public getPlaybackSuppressionReason(): number;
					public setMediaItems(param0: java.util.List<com.google.android.exoplayer2.MediaItem>, param1: boolean): void;
					public getCurrentPeriodIndex(): number;
					/** @deprecated */
					public setDeviceMuted(param0: boolean): void;
					/** @deprecated */
					public clearVideoFrameMetadataListener(param0: com.google.android.exoplayer2.video.VideoFrameMetadataListener): void;
					public getAudioDecoderCounters(): com.google.android.exoplayer2.decoder.DecoderCounters;
					public setShuffleOrder(param0: com.google.android.exoplayer2.source.ShuffleOrder): void;
					/** @deprecated */
					public getTextComponent(): com.google.android.exoplayer2.ExoPlayer.TextComponent;
					public addAudioOffloadListener(param0: com.google.android.exoplayer2.ExoPlayer.AudioOffloadListener): void;
					public prepare(): void;
					public moveMediaItems(param0: number, param1: number, param2: number): void;
					public setHandleAudioBecomingNoisy(param0: boolean): void;
					/** @deprecated */
					public getVideoComponent(): com.google.android.exoplayer2.ExoPlayer.VideoComponent;
					public clearVideoSurfaceHolder(param0: globalAndroid.view.SurfaceHolder): void;
					public decreaseDeviceVolume(): void;
					public getAudioAttributes(): com.google.android.exoplayer2.audio.AudioAttributes;
					public setVideoSurfaceView(param0: globalAndroid.view.SurfaceView): void;
					public getPlayerError(): com.google.android.exoplayer2.ExoPlaybackException;
					/** @deprecated */
					public setHandleWakeLock(param0: boolean): void;
					public getSkipSilenceEnabled(): boolean;
					public getContentBufferedPosition(): number;
					public getPlaylistMetadata(): com.google.android.exoplayer2.MediaMetadata;
					public removeAnalyticsListener(param0: com.google.android.exoplayer2.analytics.AnalyticsListener): void;
					/** @deprecated */
					public decreaseDeviceVolume(): void;
					public getTrackSelectionParameters(): com.google.android.exoplayer2.trackselection.TrackSelectionParameters;
					/** @deprecated */
					public setVolume(param0: number): void;
					/** @deprecated */
					public getVolume(): number;
					public addMediaSources(param0: java.util.List<com.google.android.exoplayer2.source.MediaSource>): void;
					public setRepeatMode(param0: number): void;
					public setTrackSelectionParameters(param0: com.google.android.exoplayer2.trackselection.TrackSelectionParameters): void;
					public getPlaybackState(): number;
					public getCurrentTimeline(): com.google.android.exoplayer2.Timeline;
					/** @deprecated */
					public clearCameraMotionListener(param0: com.google.android.exoplayer2.video.spherical.CameraMotionListener): void;
					public setPlayWhenReady(param0: boolean): void;
					public getRepeatMode(): number;
					public getMaxSeekToPreviousPosition(): number;
					/** @deprecated */
					public clearVideoSurfaceHolder(param0: globalAndroid.view.SurfaceHolder): void;
					public getVideoDecoderCounters(): com.google.android.exoplayer2.decoder.DecoderCounters;
					public setMediaSources(param0: java.util.List<com.google.android.exoplayer2.source.MediaSource>): void;
					/** @deprecated */
					public setSkipSilenceEnabled(param0: boolean): void;
					/** @deprecated */
					public getVideoSize(): com.google.android.exoplayer2.video.VideoSize;
					public getDeviceComponent(): com.google.android.exoplayer2.ExoPlayer.DeviceComponent;
					public getDuration(): number;
					/** @deprecated */
					public retry(): void;
					/** @deprecated */
					public clearAuxEffectInfo(): void;
					public setDeviceVolume(param0: number): void;
					/** @deprecated */
					public prepare(param0: com.google.android.exoplayer2.source.MediaSource): void;
					public addMediaSource(param0: number, param1: com.google.android.exoplayer2.source.MediaSource): void;
					public clearVideoTextureView(param0: globalAndroid.view.TextureView): void;
					public getAnalyticsCollector(): com.google.android.exoplayer2.analytics.AnalyticsCollector;
					public setVideoChangeFrameRateStrategy(param0: number): void;
					public setVideoScalingMode(param0: number): void;
					public experimentalSetOffloadSchedulingEnabled(param0: boolean): void;
					public getVideoScalingMode(): number;
					public getTotalBufferedDuration(): number;
					/** @deprecated */
					public getAudioAttributes(): com.google.android.exoplayer2.audio.AudioAttributes;
					public getSeekForwardIncrement(): number;
					public isPlayingAd(): boolean;
					/** @deprecated */
					public setCameraMotionListener(param0: com.google.android.exoplayer2.video.spherical.CameraMotionListener): void;
					public clearVideoSurfaceView(param0: globalAndroid.view.SurfaceView): void;
					/** @deprecated */
					public setVideoChangeFrameRateStrategy(param0: number): void;
					/** @deprecated */
					public setVideoSurfaceHolder(param0: globalAndroid.view.SurfaceHolder): void;
					public setMediaSource(param0: com.google.android.exoplayer2.source.MediaSource, param1: boolean): void;
					public setPlaylistMetadata(param0: com.google.android.exoplayer2.MediaMetadata): void;
					public increaseDeviceVolume(): void;
					public setCameraMotionListener(param0: com.google.android.exoplayer2.video.spherical.CameraMotionListener): void;
					/** @deprecated */
					public increaseDeviceVolume(): void;
					public setPlaybackParameters(param0: com.google.android.exoplayer2.PlaybackParameters): void;
					public getAudioComponent(): com.google.android.exoplayer2.ExoPlayer.AudioComponent;
					public getCurrentTrackGroups(): com.google.android.exoplayer2.source.TrackGroupArray;
					public getVolume(): number;
					public setVideoFrameMetadataListener(param0: com.google.android.exoplayer2.video.VideoFrameMetadataListener): void;
					public setWakeMode(param0: number): void;
					public addListener(param0: com.google.android.exoplayer2.Player.Listener): void;
					public setVolume(param0: number): void;
					public getPauseAtEndOfMediaItems(): boolean;
					public clearAuxEffectInfo(): void;
					/** @deprecated */
					public clearVideoSurface(): void;
					public getCurrentTracksInfo(): com.google.android.exoplayer2.TracksInfo;
					public getAvailableCommands(): com.google.android.exoplayer2.Player.Commands;
					public removeAudioOffloadListener(param0: com.google.android.exoplayer2.ExoPlayer.AudioOffloadListener): void;
					/** @deprecated */
					public getDeviceComponent(): com.google.android.exoplayer2.ExoPlayer.DeviceComponent;
					public setAuxEffectInfo(param0: com.google.android.exoplayer2.audio.AuxEffectInfo): void;
					public removeListener(param0: com.google.android.exoplayer2.Player.Listener): void;
					public setMediaSources(param0: java.util.List<com.google.android.exoplayer2.source.MediaSource>, param1: number, param2: number): void;
					public getCurrentAdGroupIndex(): number;
					/** @deprecated */
					public setVideoScalingMode(param0: number): void;
					/** @deprecated */
					public clearVideoSurface(param0: globalAndroid.view.Surface): void;
					public getTextComponent(): com.google.android.exoplayer2.ExoPlayer.TextComponent;
					public setSkipSilenceEnabled(param0: boolean): void;
					/** @deprecated */
					public setVideoSurfaceView(param0: globalAndroid.view.SurfaceView): void;
					/** @deprecated */
					public getCurrentCues(): java.util.List<com.google.android.exoplayer2.text.Cue>;
					public setMediaSources(param0: java.util.List<com.google.android.exoplayer2.source.MediaSource>, param1: boolean): void;
					public getPlaybackParameters(): com.google.android.exoplayer2.PlaybackParameters;
					public getVideoFormat(): com.google.android.exoplayer2.Format;
					public getShuffleModeEnabled(): boolean;
					public getAudioSessionId(): number;
					public setMediaSource(param0: com.google.android.exoplayer2.source.MediaSource, param1: number): void;
					public release(): void;
					public getCurrentAdIndexInAdGroup(): number;
					/** @deprecated */
					public getAudioComponent(): com.google.android.exoplayer2.ExoPlayer.AudioComponent;
					/** @deprecated */
					public setVideoSurface(param0: globalAndroid.view.Surface): void;
					public getSeekParameters(): com.google.android.exoplayer2.SeekParameters;
					public setShuffleModeEnabled(param0: boolean): void;
					public getTrackSelector(): com.google.android.exoplayer2.trackselection.TrackSelector;
					public stop(): void;
					public setForegroundMode(param0: boolean): void;
					public setPriorityTaskManager(param0: com.google.android.exoplayer2.util.PriorityTaskManager): void;
					public addMediaItems(param0: number, param1: java.util.List<com.google.android.exoplayer2.MediaItem>): void;
					public removeMediaItems(param0: number, param1: number): void;
					public getMediaMetadata(): com.google.android.exoplayer2.MediaMetadata;
					public isDeviceMuted(): boolean;
					public getCurrentMediaItemIndex(): number;
					/** @deprecated */
					public getVideoChangeFrameRateStrategy(): number;
					public setVideoSurface(param0: globalAndroid.view.Surface): void;
					public experimentalIsSleepingForOffload(): boolean;
					public setAudioAttributes(param0: com.google.android.exoplayer2.audio.AudioAttributes, param1: boolean): void;
					public isLoading(): boolean;
					public getPlaybackLooper(): globalAndroid.os.Looper;
					public setMediaSource(param0: com.google.android.exoplayer2.source.MediaSource): void;
					public addMediaSource(param0: com.google.android.exoplayer2.source.MediaSource): void;
					public getVideoSize(): com.google.android.exoplayer2.video.VideoSize;
					/** @deprecated */
					public getVideoScalingMode(): number;
					public setVideoTextureView(param0: globalAndroid.view.TextureView): void;
					public getSeekBackIncrement(): number;
					public setDeviceMuted(param0: boolean): void;
					public clearVideoSurface(param0: globalAndroid.view.Surface): void;
					public addAnalyticsListener(param0: com.google.android.exoplayer2.analytics.AnalyticsListener): void;
					/** @deprecated */
					public prepare(param0: com.google.android.exoplayer2.source.MediaSource, param1: boolean, param2: boolean): void;
					/** @deprecated */
					public setAuxEffectInfo(param0: com.google.android.exoplayer2.audio.AuxEffectInfo): void;
					public getApplicationLooper(): globalAndroid.os.Looper;
					/** @deprecated */
					public clearVideoTextureView(param0: globalAndroid.view.TextureView): void;
					public getVideoComponent(): com.google.android.exoplayer2.ExoPlayer.VideoComponent;
					public seekTo(param0: number, param1: number): void;
					public addMediaSources(param0: number, param1: java.util.List<com.google.android.exoplayer2.source.MediaSource>): void;
					/** @deprecated */
					public clearVideoSurfaceView(param0: globalAndroid.view.SurfaceView): void;
					public getCurrentPosition(): number;
					/** @deprecated */
					public setAudioAttributes(param0: com.google.android.exoplayer2.audio.AudioAttributes, param1: boolean): void;
					public getRenderer(param0: number): com.google.android.exoplayer2.Renderer;
					public getCurrentCues(): java.util.List<com.google.android.exoplayer2.text.Cue>;
					public getCurrentTrackSelections(): com.google.android.exoplayer2.trackselection.TrackSelectionArray;
					/** @deprecated */
					public setVideoTextureView(param0: globalAndroid.view.TextureView): void;
					/** @deprecated */
					public getAudioSessionId(): number;
					/** @deprecated */
					public setAudioSessionId(param0: number): void;
					/** @deprecated */
					public getDeviceVolume(): number;
					public setPauseAtEndOfMediaItems(param0: boolean): void;
					public setVideoSurfaceHolder(param0: globalAndroid.view.SurfaceHolder): void;
					public clearVideoFrameMetadataListener(param0: com.google.android.exoplayer2.video.VideoFrameMetadataListener): void;
					/** @deprecated */
					public setVideoFrameMetadataListener(param0: com.google.android.exoplayer2.video.VideoFrameMetadataListener): void;
					/** @deprecated */
					public getDeviceInfo(): com.google.android.exoplayer2.DeviceInfo;
					/** @deprecated */
					public stop(param0: boolean): void;
					public setSeekParameters(param0: com.google.android.exoplayer2.SeekParameters): void;
					public getBufferedPosition(): number;
					public getRendererType(param0: number): number;
					/** @deprecated */
					public isDeviceMuted(): boolean;
				}
				export module SimpleExoPlayer {
					export class Builder {
						public static class: java.lang.Class<com.google.android.exoplayer2.SimpleExoPlayer.Builder>;
						/** @deprecated */
						public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.extractor.ExtractorsFactory);
						/** @deprecated */
						public setLivePlaybackSpeedControl(param0: com.google.android.exoplayer2.LivePlaybackSpeedControl): com.google.android.exoplayer2.SimpleExoPlayer.Builder;
						/** @deprecated */
						public setHandleAudioBecomingNoisy(param0: boolean): com.google.android.exoplayer2.SimpleExoPlayer.Builder;
						/** @deprecated */
						public setWakeMode(param0: number): com.google.android.exoplayer2.SimpleExoPlayer.Builder;
						/** @deprecated */
						public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.RenderersFactory, param2: com.google.android.exoplayer2.trackselection.TrackSelector, param3: com.google.android.exoplayer2.source.MediaSource.Factory, param4: com.google.android.exoplayer2.LoadControl, param5: com.google.android.exoplayer2.upstream.BandwidthMeter, param6: com.google.android.exoplayer2.analytics.AnalyticsCollector);
						/** @deprecated */
						public setVideoScalingMode(param0: number): com.google.android.exoplayer2.SimpleExoPlayer.Builder;
						/** @deprecated */
						public setClock(param0: com.google.android.exoplayer2.util.Clock): com.google.android.exoplayer2.SimpleExoPlayer.Builder;
						/** @deprecated */
						public setLooper(param0: globalAndroid.os.Looper): com.google.android.exoplayer2.SimpleExoPlayer.Builder;
						/** @deprecated */
						public setAnalyticsCollector(param0: com.google.android.exoplayer2.analytics.AnalyticsCollector): com.google.android.exoplayer2.SimpleExoPlayer.Builder;
						/** @deprecated */
						public setAudioAttributes(param0: com.google.android.exoplayer2.audio.AudioAttributes, param1: boolean): com.google.android.exoplayer2.SimpleExoPlayer.Builder;
						/** @deprecated */
						public build(): com.google.android.exoplayer2.SimpleExoPlayer;
						/** @deprecated */
						public setVideoChangeFrameRateStrategy(param0: number): com.google.android.exoplayer2.SimpleExoPlayer.Builder;
						/** @deprecated */
						public setSeekParameters(param0: com.google.android.exoplayer2.SeekParameters): com.google.android.exoplayer2.SimpleExoPlayer.Builder;
						/** @deprecated */
						public experimentalSetForegroundModeTimeoutMs(param0: number): com.google.android.exoplayer2.SimpleExoPlayer.Builder;
						/** @deprecated */
						public setDetachSurfaceTimeoutMs(param0: number): com.google.android.exoplayer2.SimpleExoPlayer.Builder;
						/** @deprecated */
						public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.RenderersFactory);
						/** @deprecated */
						public setPauseAtEndOfMediaItems(param0: boolean): com.google.android.exoplayer2.SimpleExoPlayer.Builder;
						/** @deprecated */
						public setReleaseTimeoutMs(param0: number): com.google.android.exoplayer2.SimpleExoPlayer.Builder;
						/** @deprecated */
						public setTrackSelector(param0: com.google.android.exoplayer2.trackselection.TrackSelector): com.google.android.exoplayer2.SimpleExoPlayer.Builder;
						/** @deprecated */
						public setSkipSilenceEnabled(param0: boolean): com.google.android.exoplayer2.SimpleExoPlayer.Builder;
						/** @deprecated */
						public setMediaSourceFactory(param0: com.google.android.exoplayer2.source.MediaSource.Factory): com.google.android.exoplayer2.SimpleExoPlayer.Builder;
						/** @deprecated */
						public setBandwidthMeter(param0: com.google.android.exoplayer2.upstream.BandwidthMeter): com.google.android.exoplayer2.SimpleExoPlayer.Builder;
						/** @deprecated */
						public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.RenderersFactory, param2: com.google.android.exoplayer2.extractor.ExtractorsFactory);
						/** @deprecated */
						public setPriorityTaskManager(param0: com.google.android.exoplayer2.util.PriorityTaskManager): com.google.android.exoplayer2.SimpleExoPlayer.Builder;
						/** @deprecated */
						public setLoadControl(param0: com.google.android.exoplayer2.LoadControl): com.google.android.exoplayer2.SimpleExoPlayer.Builder;
						/** @deprecated */
						public constructor(param0: globalAndroid.content.Context);
						/** @deprecated */
						public setUseLazyPreparation(param0: boolean): com.google.android.exoplayer2.SimpleExoPlayer.Builder;
						/** @deprecated */
						public setSeekBackIncrementMs(param0: number): com.google.android.exoplayer2.SimpleExoPlayer.Builder;
						/** @deprecated */
						public setSeekForwardIncrementMs(param0: number): com.google.android.exoplayer2.SimpleExoPlayer.Builder;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class StreamVolumeManager {
					public static class: java.lang.Class<com.google.android.exoplayer2.StreamVolumeManager>;
					public increaseVolume(): void;
					public getMaxVolume(): number;
					public setMuted(param0: boolean): void;
					public isMuted(): boolean;
					public getVolume(): number;
					public decreaseVolume(): void;
					public release(): void;
					public setStreamType(param0: number): void;
					public setVolume(param0: number): void;
					public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.os.Handler, param2: com.google.android.exoplayer2.StreamVolumeManager.Listener);
					public getMinVolume(): number;
				}
				export module StreamVolumeManager {
					export class Listener {
						public static class: java.lang.Class<com.google.android.exoplayer2.StreamVolumeManager.Listener>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.StreamVolumeManager$Listener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							onStreamTypeChanged(param0: number): void;
							onStreamVolumeChanged(param0: number, param1: boolean): void;
						});
						public constructor();
						public onStreamTypeChanged(param0: number): void;
						public onStreamVolumeChanged(param0: number, param1: boolean): void;
					}
					export class VolumeChangeReceiver {
						public static class: java.lang.Class<com.google.android.exoplayer2.StreamVolumeManager.VolumeChangeReceiver>;
						public onReceive(param0: globalAndroid.content.Context, param1: globalAndroid.content.Intent): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class WakeLockManager {
					public static class: java.lang.Class<com.google.android.exoplayer2.WakeLockManager>;
					public constructor(param0: globalAndroid.content.Context);
					public setEnabled(param0: boolean): void;
					public setStayAwake(param0: boolean): void;
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export class WifiLockManager {
					public static class: java.lang.Class<com.google.android.exoplayer2.WifiLockManager>;
					public constructor(param0: globalAndroid.content.Context);
					public setEnabled(param0: boolean): void;
					public setStayAwake(param0: boolean): void;
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module analytics {
					export class AnalyticsCollector implements com.google.android.exoplayer2.source.MediaSourceEventListener, com.google.android.exoplayer2.upstream.BandwidthMeter.EventListener, com.google.android.exoplayer2.drm.DrmSessionEventListener {
						public static class: java.lang.Class<com.google.android.exoplayer2.analytics.AnalyticsCollector>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.analytics.AnalyticsCollector interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							addListener(param0: com.google.android.exoplayer2.analytics.AnalyticsListener): void;
							removeListener(param0: com.google.android.exoplayer2.analytics.AnalyticsListener): void;
							setPlayer(param0: com.google.android.exoplayer2.Player, param1: globalAndroid.os.Looper): void;
							release(): void;
							updateMediaPeriodQueueInfo(param0: java.util.List<com.google.android.exoplayer2.source.MediaSource.MediaPeriodId>, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
							notifySeekStarted(): void;
							onAudioEnabled(param0: com.google.android.exoplayer2.decoder.DecoderCounters): void;
							onAudioDecoderInitialized(param0: string, param1: number, param2: number): void;
							onAudioInputFormatChanged(param0: com.google.android.exoplayer2.Format, param1: com.google.android.exoplayer2.decoder.DecoderReuseEvaluation): void;
							onAudioPositionAdvancing(param0: number): void;
							onAudioUnderrun(param0: number, param1: number, param2: number): void;
							onAudioDecoderReleased(param0: string): void;
							onAudioDisabled(param0: com.google.android.exoplayer2.decoder.DecoderCounters): void;
							onAudioSinkError(param0: java.lang.Exception): void;
							onAudioCodecError(param0: java.lang.Exception): void;
							onVideoEnabled(param0: com.google.android.exoplayer2.decoder.DecoderCounters): void;
							onVideoDecoderInitialized(param0: string, param1: number, param2: number): void;
							onVideoInputFormatChanged(param0: com.google.android.exoplayer2.Format, param1: com.google.android.exoplayer2.decoder.DecoderReuseEvaluation): void;
							onDroppedFrames(param0: number, param1: number): void;
							onVideoDecoderReleased(param0: string): void;
							onVideoDisabled(param0: com.google.android.exoplayer2.decoder.DecoderCounters): void;
							onRenderedFirstFrame(param0: any, param1: number): void;
							onVideoFrameProcessingOffset(param0: number, param1: number): void;
							onVideoCodecError(param0: java.lang.Exception): void;
							onLoadStarted(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData): void;
							onLoadCompleted(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData): void;
							onLoadCanceled(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData): void;
							onLoadError(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData, param4: java.io.IOException, param5: boolean): void;
							onUpstreamDiscarded(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
							onDownstreamFormatChanged(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
							onBandwidthSample(param0: number, param1: number, param2: number): void;
							onDrmSessionAcquired(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
							onDrmSessionAcquired(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: number): void;
							onDrmKeysLoaded(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
							onDrmSessionManagerError(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: java.lang.Exception): void;
							onDrmKeysRestored(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
							onDrmKeysRemoved(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
							onDrmSessionReleased(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
						});
						public constructor();
						public onAudioEnabled(param0: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onDrmSessionReleased(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
						public onVideoEnabled(param0: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public addListener(param0: com.google.android.exoplayer2.analytics.AnalyticsListener): void;
						public onAudioInputFormatChanged(param0: com.google.android.exoplayer2.Format, param1: com.google.android.exoplayer2.decoder.DecoderReuseEvaluation): void;
						public onDrmSessionAcquired(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: number): void;
						public updateMediaPeriodQueueInfo(param0: java.util.List<com.google.android.exoplayer2.source.MediaSource.MediaPeriodId>, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
						public onLoadCanceled(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData): void;
						public onVideoDecoderInitialized(param0: string, param1: number, param2: number): void;
						public onLoadCompleted(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData): void;
						public onDrmKeysRestored(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
						public onVideoDecoderReleased(param0: string): void;
						/** @deprecated */
						public onDrmSessionAcquired(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
						public onUpstreamDiscarded(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
						public onDrmKeysRemoved(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
						public onAudioCodecError(param0: java.lang.Exception): void;
						public onDrmSessionManagerError(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: java.lang.Exception): void;
						public onAudioDecoderReleased(param0: string): void;
						public onVideoDisabled(param0: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onRenderedFirstFrame(param0: any, param1: number): void;
						public onDownstreamFormatChanged(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
						public onVideoCodecError(param0: java.lang.Exception): void;
						public onVideoInputFormatChanged(param0: com.google.android.exoplayer2.Format, param1: com.google.android.exoplayer2.decoder.DecoderReuseEvaluation): void;
						public onDroppedFrames(param0: number, param1: number): void;
						public onDrmKeysLoaded(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
						public onAudioPositionAdvancing(param0: number): void;
						public release(): void;
						public notifySeekStarted(): void;
						public onAudioUnderrun(param0: number, param1: number, param2: number): void;
						public onAudioDisabled(param0: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onBandwidthSample(param0: number, param1: number, param2: number): void;
						public onAudioSinkError(param0: java.lang.Exception): void;
						public onLoadStarted(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData): void;
						public setPlayer(param0: com.google.android.exoplayer2.Player, param1: globalAndroid.os.Looper): void;
						public removeListener(param0: com.google.android.exoplayer2.analytics.AnalyticsListener): void;
						public onLoadError(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData, param4: java.io.IOException, param5: boolean): void;
						public onVideoFrameProcessingOffset(param0: number, param1: number): void;
						public onAudioDecoderInitialized(param0: string, param1: number, param2: number): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module analytics {
					export class AnalyticsListener {
						public static class: java.lang.Class<com.google.android.exoplayer2.analytics.AnalyticsListener>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.analytics.AnalyticsListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							onPlayerStateChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean, param2: number): void;
							onPlaybackStateChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
							onPlayWhenReadyChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean, param2: number): void;
							onPlaybackSuppressionReasonChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
							onIsPlayingChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean): void;
							onTimelineChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
							onMediaItemTransition(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.MediaItem, param2: number): void;
							onPositionDiscontinuity(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
							onPositionDiscontinuity(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Player.PositionInfo, param2: com.google.android.exoplayer2.Player.PositionInfo, param3: number): void;
							onSeekStarted(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
							onSeekProcessed(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
							onPlaybackParametersChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.PlaybackParameters): void;
							onSeekBackIncrementChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
							onSeekForwardIncrementChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
							onMaxSeekToPreviousPositionChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
							onRepeatModeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
							onShuffleModeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean): void;
							onIsLoadingChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean): void;
							onLoadingChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean): void;
							onAvailableCommandsChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Player.Commands): void;
							onPlayerError(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.PlaybackException): void;
							onPlayerErrorChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.PlaybackException): void;
							onTracksChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.TrackGroupArray, param2: com.google.android.exoplayer2.trackselection.TrackSelectionArray): void;
							onTracksInfoChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.TracksInfo): void;
							onTrackSelectionParametersChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.trackselection.TrackSelectionParameters): void;
							onMediaMetadataChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.MediaMetadata): void;
							onPlaylistMetadataChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.MediaMetadata): void;
							onLoadStarted(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.LoadEventInfo, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
							onLoadCompleted(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.LoadEventInfo, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
							onLoadCanceled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.LoadEventInfo, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
							onLoadError(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.LoadEventInfo, param2: com.google.android.exoplayer2.source.MediaLoadData, param3: java.io.IOException, param4: boolean): void;
							onDownstreamFormatChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.MediaLoadData): void;
							onUpstreamDiscarded(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.MediaLoadData): void;
							onBandwidthEstimate(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: number, param3: number): void;
							onMetadata(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.metadata.Metadata): void;
							onCues(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: java.util.List<com.google.android.exoplayer2.text.Cue>): void;
							onDecoderEnabled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: com.google.android.exoplayer2.decoder.DecoderCounters): void;
							onDecoderInitialized(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: string, param3: number): void;
							onDecoderInputFormatChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: com.google.android.exoplayer2.Format): void;
							onDecoderDisabled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: com.google.android.exoplayer2.decoder.DecoderCounters): void;
							onAudioEnabled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.decoder.DecoderCounters): void;
							onAudioDecoderInitialized(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string, param2: number, param3: number): void;
							onAudioDecoderInitialized(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string, param2: number): void;
							onAudioInputFormatChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Format): void;
							onAudioInputFormatChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Format, param2: com.google.android.exoplayer2.decoder.DecoderReuseEvaluation): void;
							onAudioPositionAdvancing(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
							onAudioUnderrun(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: number, param3: number): void;
							onAudioDecoderReleased(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string): void;
							onAudioDisabled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.decoder.DecoderCounters): void;
							onAudioSessionIdChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
							onAudioAttributesChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.audio.AudioAttributes): void;
							onSkipSilenceEnabledChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean): void;
							onAudioSinkError(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: java.lang.Exception): void;
							onAudioCodecError(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: java.lang.Exception): void;
							onVolumeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
							onDeviceInfoChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.DeviceInfo): void;
							onDeviceVolumeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: boolean): void;
							onVideoEnabled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.decoder.DecoderCounters): void;
							onVideoDecoderInitialized(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string, param2: number, param3: number): void;
							onVideoDecoderInitialized(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string, param2: number): void;
							onVideoInputFormatChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Format): void;
							onVideoInputFormatChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Format, param2: com.google.android.exoplayer2.decoder.DecoderReuseEvaluation): void;
							onDroppedVideoFrames(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: number): void;
							onVideoDecoderReleased(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string): void;
							onVideoDisabled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.decoder.DecoderCounters): void;
							onVideoFrameProcessingOffset(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: number): void;
							onVideoCodecError(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: java.lang.Exception): void;
							onRenderedFirstFrame(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: any, param2: number): void;
							onVideoSizeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.video.VideoSize): void;
							onVideoSizeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: number, param3: number, param4: number): void;
							onSurfaceSizeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: number): void;
							onDrmSessionAcquired(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
							onDrmSessionAcquired(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
							onDrmKeysLoaded(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
							onDrmSessionManagerError(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: java.lang.Exception): void;
							onDrmKeysRestored(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
							onDrmKeysRemoved(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
							onDrmSessionReleased(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
							onPlayerReleased(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
							onEvents(param0: com.google.android.exoplayer2.Player, param1: com.google.android.exoplayer2.analytics.AnalyticsListener.Events): void;
						});
						public constructor();
						public static EVENT_AUDIO_DECODER_RELEASED: number;
						public static EVENT_DRM_KEYS_REMOVED: number;
						public static EVENT_POSITION_DISCONTINUITY: number;
						public static EVENT_SEEK_BACK_INCREMENT_CHANGED: number;
						public static EVENT_TRACKS_CHANGED: number;
						public static EVENT_SEEK_FORWARD_INCREMENT_CHANGED: number;
						public static EVENT_TIMELINE_CHANGED: number;
						public static EVENT_DRM_SESSION_RELEASED: number;
						public static EVENT_VIDEO_CODEC_ERROR: number;
						public static EVENT_AVAILABLE_COMMANDS_CHANGED: number;
						public static EVENT_AUDIO_ATTRIBUTES_CHANGED: number;
						public static EVENT_AUDIO_CODEC_ERROR: number;
						public static EVENT_PLAYBACK_PARAMETERS_CHANGED: number;
						public static EVENT_UPSTREAM_DISCARDED: number;
						public static EVENT_DOWNSTREAM_FORMAT_CHANGED: number;
						public static EVENT_DEVICE_INFO_CHANGED: number;
						public static EVENT_AUDIO_INPUT_FORMAT_CHANGED: number;
						public static EVENT_AUDIO_SESSION_ID: number;
						public static EVENT_DRM_SESSION_ACQUIRED: number;
						public static EVENT_SURFACE_SIZE_CHANGED: number;
						public static EVENT_PLAYER_ERROR: number;
						public static EVENT_LOAD_COMPLETED: number;
						public static EVENT_VIDEO_FRAME_PROCESSING_OFFSET: number;
						public static EVENT_AUDIO_UNDERRUN: number;
						public static EVENT_DEVICE_VOLUME_CHANGED: number;
						public static EVENT_METADATA: number;
						public static EVENT_AUDIO_DECODER_INITIALIZED: number;
						public static EVENT_REPEAT_MODE_CHANGED: number;
						public static EVENT_AUDIO_ENABLED: number;
						public static EVENT_VIDEO_SIZE_CHANGED: number;
						public static EVENT_LOAD_STARTED: number;
						public static EVENT_VIDEO_DECODER_RELEASED: number;
						public static EVENT_PLAYLIST_METADATA_CHANGED: number;
						public static EVENT_IS_PLAYING_CHANGED: number;
						public static EVENT_PLAYER_RELEASED: number;
						public static EVENT_PLAYBACK_SUPPRESSION_REASON_CHANGED: number;
						public static EVENT_VIDEO_ENABLED: number;
						public static EVENT_VIDEO_DECODER_INITIALIZED: number;
						public static EVENT_PLAY_WHEN_READY_CHANGED: number;
						public static EVENT_RENDERED_FIRST_FRAME: number;
						public static EVENT_AUDIO_DISABLED: number;
						public static EVENT_VIDEO_DISABLED: number;
						public static EVENT_LOAD_ERROR: number;
						public static EVENT_SHUFFLE_MODE_ENABLED_CHANGED: number;
						public static EVENT_DRM_KEYS_LOADED: number;
						public static EVENT_MEDIA_ITEM_TRANSITION: number;
						public static EVENT_BANDWIDTH_ESTIMATE: number;
						public static EVENT_IS_LOADING_CHANGED: number;
						public static EVENT_VOLUME_CHANGED: number;
						public static EVENT_PLAYBACK_STATE_CHANGED: number;
						public static EVENT_MEDIA_METADATA_CHANGED: number;
						public static EVENT_CUES: number;
						public static EVENT_MAX_SEEK_TO_PREVIOUS_POSITION_CHANGED: number;
						public static EVENT_DRM_KEYS_RESTORED: number;
						public static EVENT_DRM_SESSION_MANAGER_ERROR: number;
						public static EVENT_DROPPED_VIDEO_FRAMES: number;
						public static EVENT_SKIP_SILENCE_ENABLED_CHANGED: number;
						public static EVENT_AUDIO_SINK_ERROR: number;
						public static EVENT_AUDIO_POSITION_ADVANCING: number;
						public static EVENT_LOAD_CANCELED: number;
						public static EVENT_TRACK_SELECTION_PARAMETERS_CHANGED: number;
						public static EVENT_VIDEO_INPUT_FORMAT_CHANGED: number;
						public onVideoEnabled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onTrackSelectionParametersChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.trackselection.TrackSelectionParameters): void;
						public onUpstreamDiscarded(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.MediaLoadData): void;
						/** @deprecated */
						public onSeekStarted(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						/** @deprecated */
						public onVideoDecoderInitialized(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string, param2: number): void;
						public onDrmSessionAcquired(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onVolumeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onRenderedFirstFrame(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: any, param2: number): void;
						public onPlayerReleased(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						public onAudioPositionAdvancing(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onTracksInfoChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.TracksInfo): void;
						public onMaxSeekToPreviousPositionChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onCues(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: java.util.List<com.google.android.exoplayer2.text.Cue>): void;
						public onVideoInputFormatChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Format, param2: com.google.android.exoplayer2.decoder.DecoderReuseEvaluation): void;
						/** @deprecated */
						public onPlayerStateChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean, param2: number): void;
						public onIsPlayingChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean): void;
						public onMediaItemTransition(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.MediaItem, param2: number): void;
						public onDroppedVideoFrames(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: number): void;
						/** @deprecated */
						public onDecoderInputFormatChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: com.google.android.exoplayer2.Format): void;
						public onDrmSessionReleased(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						/** @deprecated */
						public onTracksChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.TrackGroupArray, param2: com.google.android.exoplayer2.trackselection.TrackSelectionArray): void;
						public onLoadCanceled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.LoadEventInfo, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
						public onSeekBackIncrementChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onAudioDecoderReleased(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string): void;
						/** @deprecated */
						public onSeekProcessed(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						public onIsLoadingChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean): void;
						public onTimelineChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onEvents(param0: com.google.android.exoplayer2.Player, param1: com.google.android.exoplayer2.analytics.AnalyticsListener.Events): void;
						public onVideoFrameProcessingOffset(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: number): void;
						/** @deprecated */
						public onAudioInputFormatChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Format): void;
						/** @deprecated */
						public onDecoderDisabled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onDownstreamFormatChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.MediaLoadData): void;
						public onPlayWhenReadyChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean, param2: number): void;
						public onVideoDisabled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onAudioInputFormatChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Format, param2: com.google.android.exoplayer2.decoder.DecoderReuseEvaluation): void;
						public onLoadStarted(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.LoadEventInfo, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
						public onAudioCodecError(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: java.lang.Exception): void;
						/** @deprecated */
						public onVideoInputFormatChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Format): void;
						public onDrmKeysRemoved(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						public onPlaybackSuppressionReasonChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onBandwidthEstimate(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: number, param3: number): void;
						public onLoadCompleted(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.LoadEventInfo, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
						public onAudioDecoderInitialized(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string, param2: number, param3: number): void;
						public onVideoDecoderInitialized(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string, param2: number, param3: number): void;
						public onDrmKeysLoaded(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						public onPlayerErrorChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.PlaybackException): void;
						public onLoadError(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.LoadEventInfo, param2: com.google.android.exoplayer2.source.MediaLoadData, param3: java.io.IOException, param4: boolean): void;
						public onRepeatModeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onShuffleModeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean): void;
						public onPlaylistMetadataChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.MediaMetadata): void;
						/** @deprecated */
						public onAudioDecoderInitialized(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string, param2: number): void;
						/** @deprecated */
						public onPositionDiscontinuity(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onVideoCodecError(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: java.lang.Exception): void;
						/** @deprecated */
						public onVideoSizeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: number, param3: number, param4: number): void;
						/** @deprecated */
						public onDecoderEnabled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onAudioDisabled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onDeviceInfoChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.DeviceInfo): void;
						public onDrmKeysRestored(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						public onSkipSilenceEnabledChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean): void;
						public onPlayerError(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.PlaybackException): void;
						public onVideoSizeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.video.VideoSize): void;
						public onPlaybackParametersChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.PlaybackParameters): void;
						public onMediaMetadataChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.MediaMetadata): void;
						public onAudioAttributesChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.audio.AudioAttributes): void;
						public onSurfaceSizeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: number): void;
						/** @deprecated */
						public onLoadingChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean): void;
						public onSeekForwardIncrementChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onDrmSessionManagerError(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: java.lang.Exception): void;
						public onPositionDiscontinuity(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Player.PositionInfo, param2: com.google.android.exoplayer2.Player.PositionInfo, param3: number): void;
						public onAvailableCommandsChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Player.Commands): void;
						public onPlaybackStateChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onAudioSessionIdChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						/** @deprecated */
						public onDrmSessionAcquired(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						public onAudioEnabled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onVideoDecoderReleased(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string): void;
						public onAudioSinkError(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: java.lang.Exception): void;
						public onMetadata(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.metadata.Metadata): void;
						public onAudioUnderrun(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: number, param3: number): void;
						/** @deprecated */
						public onDecoderInitialized(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: string, param3: number): void;
						public onDeviceVolumeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: boolean): void;
					}
					export module AnalyticsListener {
						export class EventFlags {
							public static class: java.lang.Class<com.google.android.exoplayer2.analytics.AnalyticsListener.EventFlags>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.analytics.AnalyticsListener$EventFlags interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
							});
							public constructor();
						}
						export class EventTime {
							public static class: java.lang.Class<com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime>;
							public realtimeMs: number;
							public timeline: com.google.android.exoplayer2.Timeline;
							public windowIndex: number;
							public mediaPeriodId: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId;
							public eventPlaybackPositionMs: number;
							public currentTimeline: com.google.android.exoplayer2.Timeline;
							public currentWindowIndex: number;
							public currentMediaPeriodId: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId;
							public currentPlaybackPositionMs: number;
							public totalBufferedDurationMs: number;
							public constructor(param0: number, param1: com.google.android.exoplayer2.Timeline, param2: number, param3: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param4: number, param5: com.google.android.exoplayer2.Timeline, param6: number, param7: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param8: number, param9: number);
							public hashCode(): number;
							public equals(param0: any): boolean;
						}
						export class Events {
							public static class: java.lang.Class<com.google.android.exoplayer2.analytics.AnalyticsListener.Events>;
							public get(param0: number): number;
							public constructor(param0: com.google.android.exoplayer2.util.FlagSet, param1: globalAndroid.util.SparseArray<com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime>);
							public size(): number;
							public containsAny(param0: androidNative.Array<number>): boolean;
							public getEventTime(param0: number): com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime;
							public contains(param0: number): boolean;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module analytics {
					export class DefaultAnalyticsCollector extends com.google.android.exoplayer2.analytics.AnalyticsCollector {
						public static class: java.lang.Class<com.google.android.exoplayer2.analytics.DefaultAnalyticsCollector>;
						public onPlayWhenReadyChanged(param0: boolean, param1: number): void;
						public onSeekProcessed(): void;
						public onDrmSessionReleased(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
						public onVideoEnabled(param0: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public sendEvent(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: com.google.android.exoplayer2.util.ListenerSet.Event<com.google.android.exoplayer2.analytics.AnalyticsListener>): void;
						public onPlaylistMetadataChanged(param0: com.google.android.exoplayer2.MediaMetadata): void;
						public onPlaybackSuppressionReasonChanged(param0: number): void;
						public onDeviceVolumeChanged(param0: number, param1: boolean): void;
						public onSeekForwardIncrementChanged(param0: number): void;
						public updateMediaPeriodQueueInfo(param0: java.util.List<com.google.android.exoplayer2.source.MediaSource.MediaPeriodId>, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
						public onLoadCanceled(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData): void;
						public onVideoDecoderInitialized(param0: string, param1: number, param2: number): void;
						public onTracksInfoChanged(param0: com.google.android.exoplayer2.TracksInfo): void;
						public onPlayerError(param0: com.google.android.exoplayer2.PlaybackException): void;
						public onCues(param0: java.util.List<com.google.android.exoplayer2.text.Cue>): void;
						/** @deprecated */
						public onDrmSessionAcquired(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
						public onVideoSizeChanged(param0: com.google.android.exoplayer2.video.VideoSize): void;
						public onDrmKeysRemoved(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
						public onAudioCodecError(param0: java.lang.Exception): void;
						public onIsPlayingChanged(param0: boolean): void;
						public onAudioDecoderReleased(param0: string): void;
						public onVideoDisabled(param0: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onDownstreamFormatChanged(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
						public onVideoCodecError(param0: java.lang.Exception): void;
						public onPlaybackStateChanged(param0: number): void;
						public onSeekBackIncrementChanged(param0: number): void;
						public onVideoInputFormatChanged(param0: com.google.android.exoplayer2.Format, param1: com.google.android.exoplayer2.decoder.DecoderReuseEvaluation): void;
						public onDroppedFrames(param0: number, param1: number): void;
						public onShuffleModeEnabledChanged(param0: boolean): void;
						public onAudioPositionAdvancing(param0: number): void;
						public onAvailableCommandsChanged(param0: com.google.android.exoplayer2.Player.Commands): void;
						public notifySeekStarted(): void;
						public onAudioUnderrun(param0: number, param1: number, param2: number): void;
						public onEvents(param0: com.google.android.exoplayer2.Player, param1: com.google.android.exoplayer2.Player.Events): void;
						public onAudioSinkError(param0: java.lang.Exception): void;
						public onLoadStarted(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData): void;
						public onIsLoadingChanged(param0: boolean): void;
						public onMaxSeekToPreviousPositionChanged(param0: number): void;
						public constructor(param0: com.google.android.exoplayer2.util.Clock);
						public removeListener(param0: com.google.android.exoplayer2.analytics.AnalyticsListener): void;
						public onLoadError(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData, param4: java.io.IOException, param5: boolean): void;
						public onSkipSilenceEnabledChanged(param0: boolean): void;
						public onVideoFrameProcessingOffset(param0: number, param1: number): void;
						public onRepeatModeChanged(param0: number): void;
						public onPositionDiscontinuity(param0: number): void;
						public onAudioDecoderInitialized(param0: string, param1: number, param2: number): void;
						public onMediaMetadataChanged(param0: com.google.android.exoplayer2.MediaMetadata): void;
						public onAudioEnabled(param0: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onRenderedFirstFrame(): void;
						public addListener(param0: com.google.android.exoplayer2.analytics.AnalyticsListener): void;
						public onPlayerErrorChanged(param0: com.google.android.exoplayer2.PlaybackException): void;
						public onAudioInputFormatChanged(param0: com.google.android.exoplayer2.Format, param1: com.google.android.exoplayer2.decoder.DecoderReuseEvaluation): void;
						public onDrmSessionAcquired(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: number): void;
						public onVolumeChanged(param0: number): void;
						public onPlaybackParametersChanged(param0: com.google.android.exoplayer2.PlaybackParameters): void;
						public onLoadCompleted(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData): void;
						public onDrmKeysRestored(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
						public onVideoDecoderReleased(param0: string): void;
						public onUpstreamDiscarded(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
						public onMetadata(param0: com.google.android.exoplayer2.metadata.Metadata): void;
						public generateCurrentPlayerMediaPeriodEventTime(): com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime;
						public onTimelineChanged(param0: com.google.android.exoplayer2.Timeline, param1: number): void;
						public onDrmSessionManagerError(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: java.lang.Exception): void;
						public generateEventTime(param0: com.google.android.exoplayer2.Timeline, param1: number, param2: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime;
						public onAudioSessionIdChanged(param0: number): void;
						public onRenderedFirstFrame(param0: any, param1: number): void;
						public onTracksChanged(param0: com.google.android.exoplayer2.source.TrackGroupArray, param1: com.google.android.exoplayer2.trackselection.TrackSelectionArray): void;
						public onPlayerStateChanged(param0: boolean, param1: number): void;
						public onAudioAttributesChanged(param0: com.google.android.exoplayer2.audio.AudioAttributes): void;
						public onDrmKeysLoaded(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
						public onTrackSelectionParametersChanged(param0: com.google.android.exoplayer2.trackselection.TrackSelectionParameters): void;
						public onDeviceInfoChanged(param0: com.google.android.exoplayer2.DeviceInfo): void;
						public release(): void;
						public onSurfaceSizeChanged(param0: number, param1: number): void;
						public onAudioDisabled(param0: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onBandwidthSample(param0: number, param1: number, param2: number): void;
						public setPlayer(param0: com.google.android.exoplayer2.Player, param1: globalAndroid.os.Looper): void;
						public onPositionDiscontinuity(param0: com.google.android.exoplayer2.Player.PositionInfo, param1: com.google.android.exoplayer2.Player.PositionInfo, param2: number): void;
						public onMediaItemTransition(param0: com.google.android.exoplayer2.MediaItem, param1: number): void;
						public onLoadingChanged(param0: boolean): void;
					}
					export module DefaultAnalyticsCollector {
						export class MediaPeriodQueueTracker {
							public static class: java.lang.Class<com.google.android.exoplayer2.analytics.DefaultAnalyticsCollector.MediaPeriodQueueTracker>;
							public getReadingMediaPeriod(): com.google.android.exoplayer2.source.MediaSource.MediaPeriodId;
							public getLoadingMediaPeriod(): com.google.android.exoplayer2.source.MediaSource.MediaPeriodId;
							public getPlayingMediaPeriod(): com.google.android.exoplayer2.source.MediaSource.MediaPeriodId;
							public onQueueUpdated(param0: java.util.List<com.google.android.exoplayer2.source.MediaSource.MediaPeriodId>, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.Player): void;
							public getCurrentPlayerMediaPeriod(): com.google.android.exoplayer2.source.MediaSource.MediaPeriodId;
							public onPositionDiscontinuity(param0: com.google.android.exoplayer2.Player): void;
							public constructor(param0: com.google.android.exoplayer2.Timeline.Period);
							public getMediaPeriodIdTimeline(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): com.google.android.exoplayer2.Timeline;
							public onTimelineChanged(param0: com.google.android.exoplayer2.Player): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module analytics {
					export class DefaultPlaybackSessionManager extends com.google.android.exoplayer2.analytics.PlaybackSessionManager {
						public static class: java.lang.Class<com.google.android.exoplayer2.analytics.DefaultPlaybackSessionManager>;
						public static DEFAULT_SESSION_ID_GENERATOR: com.google.common.base.Supplier<string>;
						public belongsToSession(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string): boolean;
						public updateSessionsWithTimelineChange(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						public getActiveSessionId(): string;
						public constructor();
						public constructor(param0: com.google.common.base.Supplier<string>);
						public finishAllSessions(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						public setListener(param0: com.google.android.exoplayer2.analytics.PlaybackSessionManager.Listener): void;
						public getSessionForMediaPeriodId(param0: com.google.android.exoplayer2.Timeline, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): string;
						public updateSessionsWithDiscontinuity(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public updateSessions(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
					}
					export module DefaultPlaybackSessionManager {
						export class SessionDescriptor {
							public static class: java.lang.Class<com.google.android.exoplayer2.analytics.DefaultPlaybackSessionManager.SessionDescriptor>;
							public isFinishedAtEventTime(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): boolean;
							public constructor(param0: com.google.android.exoplayer2.analytics.DefaultPlaybackSessionManager, param1: string, param2: number, param3: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId);
							public belongsToSession(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): boolean;
							public tryResolvingToNewTimeline(param0: com.google.android.exoplayer2.Timeline, param1: com.google.android.exoplayer2.Timeline): boolean;
							public maybeSetWindowSequenceNumber(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module analytics {
					export class MediaMetricsListener implements com.google.android.exoplayer2.analytics.AnalyticsListener, com.google.android.exoplayer2.analytics.PlaybackSessionManager.Listener {
						public static class: java.lang.Class<com.google.android.exoplayer2.analytics.MediaMetricsListener>;
						public onVideoEnabled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onTrackSelectionParametersChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.trackselection.TrackSelectionParameters): void;
						public static create(param0: globalAndroid.content.Context): com.google.android.exoplayer2.analytics.MediaMetricsListener;
						public onUpstreamDiscarded(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.MediaLoadData): void;
						/** @deprecated */
						public onSeekStarted(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						/** @deprecated */
						public onVideoDecoderInitialized(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string, param2: number): void;
						public onDrmSessionAcquired(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onVolumeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onRenderedFirstFrame(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: any, param2: number): void;
						public getLogSessionId(): globalAndroid.media.metrics.LogSessionId;
						public onPlayerReleased(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						public onAudioPositionAdvancing(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onTracksInfoChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.TracksInfo): void;
						public onMaxSeekToPreviousPositionChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onCues(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: java.util.List<com.google.android.exoplayer2.text.Cue>): void;
						public onVideoInputFormatChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Format, param2: com.google.android.exoplayer2.decoder.DecoderReuseEvaluation): void;
						/** @deprecated */
						public onPlayerStateChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean, param2: number): void;
						public onIsPlayingChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean): void;
						public onMediaItemTransition(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.MediaItem, param2: number): void;
						public onDroppedVideoFrames(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: number): void;
						/** @deprecated */
						public onDecoderInputFormatChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: com.google.android.exoplayer2.Format): void;
						public onDrmSessionReleased(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						/** @deprecated */
						public onTracksChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.TrackGroupArray, param2: com.google.android.exoplayer2.trackselection.TrackSelectionArray): void;
						public onLoadCanceled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.LoadEventInfo, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
						public onSeekBackIncrementChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onAudioDecoderReleased(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string): void;
						public onSessionFinished(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string, param2: boolean): void;
						/** @deprecated */
						public onSeekProcessed(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						public onSessionActive(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string): void;
						public onIsLoadingChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean): void;
						public onTimelineChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onEvents(param0: com.google.android.exoplayer2.Player, param1: com.google.android.exoplayer2.analytics.AnalyticsListener.Events): void;
						public onVideoFrameProcessingOffset(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: number): void;
						/** @deprecated */
						public onAudioInputFormatChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Format): void;
						/** @deprecated */
						public onDecoderDisabled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onDownstreamFormatChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.MediaLoadData): void;
						public onVideoDisabled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onPlayWhenReadyChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean, param2: number): void;
						public onAudioInputFormatChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Format, param2: com.google.android.exoplayer2.decoder.DecoderReuseEvaluation): void;
						public onAdPlaybackStarted(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string, param2: string): void;
						public onLoadStarted(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.LoadEventInfo, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
						public onAudioCodecError(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: java.lang.Exception): void;
						/** @deprecated */
						public onVideoInputFormatChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Format): void;
						public onDrmKeysRemoved(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						public onBandwidthEstimate(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: number, param3: number): void;
						public onPlaybackSuppressionReasonChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onLoadCompleted(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.LoadEventInfo, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
						public onAudioDecoderInitialized(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string, param2: number, param3: number): void;
						public onVideoDecoderInitialized(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string, param2: number, param3: number): void;
						public onDrmKeysLoaded(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						public onPlayerErrorChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.PlaybackException): void;
						public onLoadError(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.LoadEventInfo, param2: com.google.android.exoplayer2.source.MediaLoadData, param3: java.io.IOException, param4: boolean): void;
						public onRepeatModeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onShuffleModeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean): void;
						public onPlaylistMetadataChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.MediaMetadata): void;
						/** @deprecated */
						public onAudioDecoderInitialized(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string, param2: number): void;
						/** @deprecated */
						public onPositionDiscontinuity(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onVideoCodecError(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: java.lang.Exception): void;
						/** @deprecated */
						public onVideoSizeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: number, param3: number, param4: number): void;
						/** @deprecated */
						public onDecoderEnabled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onAudioDisabled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onDeviceInfoChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.DeviceInfo): void;
						public onDrmKeysRestored(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						public onSessionCreated(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string): void;
						public onSkipSilenceEnabledChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean): void;
						public onVideoSizeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.video.VideoSize): void;
						public onPlayerError(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.PlaybackException): void;
						public onPlaybackParametersChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.PlaybackParameters): void;
						public onMediaMetadataChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.MediaMetadata): void;
						public onAudioAttributesChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.audio.AudioAttributes): void;
						public onSurfaceSizeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: number): void;
						/** @deprecated */
						public onLoadingChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean): void;
						public onSeekForwardIncrementChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onDrmSessionManagerError(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: java.lang.Exception): void;
						public onPositionDiscontinuity(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Player.PositionInfo, param2: com.google.android.exoplayer2.Player.PositionInfo, param3: number): void;
						public onAvailableCommandsChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Player.Commands): void;
						public onPlaybackStateChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onAudioSessionIdChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						/** @deprecated */
						public onDrmSessionAcquired(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						public onAudioEnabled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onVideoDecoderReleased(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string): void;
						public onAudioSinkError(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: java.lang.Exception): void;
						public onMetadata(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.metadata.Metadata): void;
						public onAudioUnderrun(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: number, param3: number): void;
						/** @deprecated */
						public onDecoderInitialized(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: string, param3: number): void;
						public onDeviceVolumeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: boolean): void;
					}
					export module MediaMetricsListener {
						export class ErrorInfo {
							public static class: java.lang.Class<com.google.android.exoplayer2.analytics.MediaMetricsListener.ErrorInfo>;
							public errorCode: number;
							public subErrorCode: number;
							public constructor(param0: number, param1: number);
						}
						export class PendingFormatUpdate {
							public static class: java.lang.Class<com.google.android.exoplayer2.analytics.MediaMetricsListener.PendingFormatUpdate>;
							public format: com.google.android.exoplayer2.Format;
							public selectionReason: number;
							public sessionId: string;
							public constructor(param0: com.google.android.exoplayer2.Format, param1: number, param2: string);
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module analytics {
					export class PlaybackSessionManager {
						public static class: java.lang.Class<com.google.android.exoplayer2.analytics.PlaybackSessionManager>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.analytics.PlaybackSessionManager interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							setListener(param0: com.google.android.exoplayer2.analytics.PlaybackSessionManager.Listener): void;
							getSessionForMediaPeriodId(param0: com.google.android.exoplayer2.Timeline, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): string;
							belongsToSession(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string): boolean;
							updateSessions(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
							updateSessionsWithTimelineChange(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
							updateSessionsWithDiscontinuity(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
							getActiveSessionId(): string;
							finishAllSessions(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						});
						public constructor();
						public belongsToSession(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string): boolean;
						public updateSessionsWithTimelineChange(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						public getActiveSessionId(): string;
						public finishAllSessions(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						public setListener(param0: com.google.android.exoplayer2.analytics.PlaybackSessionManager.Listener): void;
						public getSessionForMediaPeriodId(param0: com.google.android.exoplayer2.Timeline, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): string;
						public updateSessionsWithDiscontinuity(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public updateSessions(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
					}
					export module PlaybackSessionManager {
						export class Listener {
							public static class: java.lang.Class<com.google.android.exoplayer2.analytics.PlaybackSessionManager.Listener>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.analytics.PlaybackSessionManager$Listener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onSessionCreated(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string): void;
								onSessionActive(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string): void;
								onAdPlaybackStarted(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string, param2: string): void;
								onSessionFinished(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string, param2: boolean): void;
							});
							public constructor();
							public onSessionActive(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string): void;
							public onSessionFinished(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string, param2: boolean): void;
							public onAdPlaybackStarted(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string, param2: string): void;
							public onSessionCreated(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module analytics {
					export class PlaybackStats {
						public static class: java.lang.Class<com.google.android.exoplayer2.analytics.PlaybackStats>;
						public static PLAYBACK_STATE_NOT_STARTED: number;
						public static PLAYBACK_STATE_JOINING_BACKGROUND: number;
						public static PLAYBACK_STATE_JOINING_FOREGROUND: number;
						public static PLAYBACK_STATE_PLAYING: number;
						public static PLAYBACK_STATE_PAUSED: number;
						public static PLAYBACK_STATE_SEEKING: number;
						public static PLAYBACK_STATE_BUFFERING: number;
						public static PLAYBACK_STATE_PAUSED_BUFFERING: number;
						public static PLAYBACK_STATE_SUPPRESSED: number;
						public static PLAYBACK_STATE_SUPPRESSED_BUFFERING: number;
						public static PLAYBACK_STATE_ENDED: number;
						public static PLAYBACK_STATE_STOPPED: number;
						public static PLAYBACK_STATE_FAILED: number;
						public static PLAYBACK_STATE_INTERRUPTED_BY_AD: number;
						public static PLAYBACK_STATE_ABANDONED: number;
						public static EMPTY: com.google.android.exoplayer2.analytics.PlaybackStats;
						public playbackCount: number;
						public playbackStateHistory: java.util.List<com.google.android.exoplayer2.analytics.PlaybackStats.EventTimeAndPlaybackState>;
						public mediaTimeHistory: java.util.List<androidNative.Array<number>>;
						public firstReportedTimeMs: number;
						public foregroundPlaybackCount: number;
						public abandonedBeforeReadyCount: number;
						public endedCount: number;
						public backgroundJoiningCount: number;
						public totalValidJoinTimeMs: number;
						public validJoinTimeCount: number;
						public totalPauseCount: number;
						public totalPauseBufferCount: number;
						public totalSeekCount: number;
						public totalRebufferCount: number;
						public maxRebufferTimeMs: number;
						public adPlaybackCount: number;
						public videoFormatHistory: java.util.List<com.google.android.exoplayer2.analytics.PlaybackStats.EventTimeAndFormat>;
						public audioFormatHistory: java.util.List<com.google.android.exoplayer2.analytics.PlaybackStats.EventTimeAndFormat>;
						public totalVideoFormatHeightTimeMs: number;
						public totalVideoFormatHeightTimeProduct: number;
						public totalVideoFormatBitrateTimeMs: number;
						public totalVideoFormatBitrateTimeProduct: number;
						public totalAudioFormatTimeMs: number;
						public totalAudioFormatBitrateTimeProduct: number;
						public initialVideoFormatHeightCount: number;
						public initialVideoFormatBitrateCount: number;
						public totalInitialVideoFormatHeight: number;
						public totalInitialVideoFormatBitrate: number;
						public initialAudioFormatBitrateCount: number;
						public totalInitialAudioFormatBitrate: number;
						public totalBandwidthTimeMs: number;
						public totalBandwidthBytes: number;
						public totalDroppedFrames: number;
						public totalAudioUnderruns: number;
						public fatalErrorPlaybackCount: number;
						public fatalErrorCount: number;
						public nonFatalErrorCount: number;
						public fatalErrorHistory: java.util.List<com.google.android.exoplayer2.analytics.PlaybackStats.EventTimeAndException>;
						public nonFatalErrorHistory: java.util.List<com.google.android.exoplayer2.analytics.PlaybackStats.EventTimeAndException>;
						public getMeanSeekTimeMs(): number;
						public getMeanTimeBetweenRebuffers(): number;
						public getMeanInitialVideoFormatBitrate(): number;
						public getMediaTimeMsAtRealtimeMs(param0: number): number;
						public getMeanSingleRebufferTimeMs(): number;
						public getAudioUnderrunRate(): number;
						public getJoinTimeRatio(): number;
						public getMeanTimeBetweenFatalErrors(): number;
						public getMeanVideoFormatHeight(): number;
						public getTotalElapsedTimeMs(): number;
						public getWaitTimeRatio(): number;
						public getFatalErrorRate(): number;
						public getTotalPlayTimeMs(): number;
						public getMeanNonFatalErrorCount(): number;
						public getMeanPlayTimeMs(): number;
						public getPlaybackStateDurationMs(param0: number): number;
						public getFatalErrorRatio(): number;
						public getDroppedFramesRate(): number;
						public getMeanWaitTimeMs(): number;
						public getMeanJoinTimeMs(): number;
						public getMeanElapsedTimeMs(): number;
						public getTotalPlayAndWaitTimeMs(): number;
						public getMeanPauseCount(): number;
						public getRebufferTimeRatio(): number;
						public getMeanPauseBufferCount(): number;
						public getMeanTimeBetweenNonFatalErrors(): number;
						public getRebufferRate(): number;
						public getMeanInitialAudioFormatBitrate(): number;
						public static merge(param0: androidNative.Array<com.google.android.exoplayer2.analytics.PlaybackStats>): com.google.android.exoplayer2.analytics.PlaybackStats;
						public getAbandonedBeforeReadyRatio(): number;
						public getMeanPausedTimeMs(): number;
						public getTotalWaitTimeMs(): number;
						public getTotalPausedTimeMs(): number;
						public getMeanRebufferCount(): number;
						public getTotalSeekTimeMs(): number;
						public getTotalRebufferTimeMs(): number;
						public getMeanBandwidth(): number;
						public getMeanRebufferTimeMs(): number;
						public getMeanVideoFormatBitrate(): number;
						public getMeanAudioFormatBitrate(): number;
						public getPlaybackStateAtTime(param0: number): number;
						public getSeekTimeRatio(): number;
						public getNonFatalErrorRate(): number;
						public getMeanSingleSeekTimeMs(): number;
						public getMeanPlayAndWaitTimeMs(): number;
						public getMeanSeekCount(): number;
						public getEndedRatio(): number;
						public getTotalJoinTimeMs(): number;
						public getMeanInitialVideoFormatHeight(): number;
					}
					export module PlaybackStats {
						export class EventTimeAndException {
							public static class: java.lang.Class<com.google.android.exoplayer2.analytics.PlaybackStats.EventTimeAndException>;
							public eventTime: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime;
							public exception: java.lang.Exception;
							public constructor(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: java.lang.Exception);
							public hashCode(): number;
							public equals(param0: any): boolean;
						}
						export class EventTimeAndFormat {
							public static class: java.lang.Class<com.google.android.exoplayer2.analytics.PlaybackStats.EventTimeAndFormat>;
							public eventTime: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime;
							public format: com.google.android.exoplayer2.Format;
							public hashCode(): number;
							public equals(param0: any): boolean;
							public constructor(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Format);
						}
						export class EventTimeAndPlaybackState {
							public static class: java.lang.Class<com.google.android.exoplayer2.analytics.PlaybackStats.EventTimeAndPlaybackState>;
							public eventTime: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime;
							public playbackState: number;
							public hashCode(): number;
							public constructor(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number);
							public equals(param0: any): boolean;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module analytics {
					export class PlaybackStatsListener implements com.google.android.exoplayer2.analytics.AnalyticsListener, com.google.android.exoplayer2.analytics.PlaybackSessionManager.Listener {
						public static class: java.lang.Class<com.google.android.exoplayer2.analytics.PlaybackStatsListener>;
						public onVideoEnabled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onTrackSelectionParametersChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.trackselection.TrackSelectionParameters): void;
						public onUpstreamDiscarded(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.MediaLoadData): void;
						/** @deprecated */
						public onSeekStarted(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						/** @deprecated */
						public onVideoDecoderInitialized(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string, param2: number): void;
						public onDrmSessionAcquired(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onVolumeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onRenderedFirstFrame(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: any, param2: number): void;
						public onPlayerReleased(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						public onAudioPositionAdvancing(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onTracksInfoChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.TracksInfo): void;
						public onMaxSeekToPreviousPositionChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onCues(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: java.util.List<com.google.android.exoplayer2.text.Cue>): void;
						public onVideoInputFormatChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Format, param2: com.google.android.exoplayer2.decoder.DecoderReuseEvaluation): void;
						/** @deprecated */
						public onPlayerStateChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean, param2: number): void;
						public onIsPlayingChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean): void;
						public onMediaItemTransition(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.MediaItem, param2: number): void;
						public onDroppedVideoFrames(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: number): void;
						public constructor(param0: boolean, param1: com.google.android.exoplayer2.analytics.PlaybackStatsListener.Callback);
						/** @deprecated */
						public onDecoderInputFormatChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: com.google.android.exoplayer2.Format): void;
						public onDrmSessionReleased(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						/** @deprecated */
						public onTracksChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.TrackGroupArray, param2: com.google.android.exoplayer2.trackselection.TrackSelectionArray): void;
						public onLoadCanceled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.LoadEventInfo, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
						public onSeekBackIncrementChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onAudioDecoderReleased(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string): void;
						public onSessionFinished(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string, param2: boolean): void;
						/** @deprecated */
						public onSeekProcessed(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						public onSessionActive(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string): void;
						public onIsLoadingChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean): void;
						public onTimelineChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onEvents(param0: com.google.android.exoplayer2.Player, param1: com.google.android.exoplayer2.analytics.AnalyticsListener.Events): void;
						public onVideoFrameProcessingOffset(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: number): void;
						/** @deprecated */
						public onAudioInputFormatChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Format): void;
						/** @deprecated */
						public onDecoderDisabled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onDownstreamFormatChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.MediaLoadData): void;
						public onPlayWhenReadyChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean, param2: number): void;
						public onVideoDisabled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onAudioInputFormatChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Format, param2: com.google.android.exoplayer2.decoder.DecoderReuseEvaluation): void;
						public onAdPlaybackStarted(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string, param2: string): void;
						public onLoadStarted(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.LoadEventInfo, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
						public onAudioCodecError(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: java.lang.Exception): void;
						/** @deprecated */
						public onVideoInputFormatChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Format): void;
						public onDrmKeysRemoved(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						public onBandwidthEstimate(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: number, param3: number): void;
						public onPlaybackSuppressionReasonChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public getPlaybackStats(): com.google.android.exoplayer2.analytics.PlaybackStats;
						public onLoadCompleted(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.LoadEventInfo, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
						public getCombinedPlaybackStats(): com.google.android.exoplayer2.analytics.PlaybackStats;
						public onAudioDecoderInitialized(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string, param2: number, param3: number): void;
						public onVideoDecoderInitialized(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string, param2: number, param3: number): void;
						public onDrmKeysLoaded(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						public onPlayerErrorChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.PlaybackException): void;
						public onLoadError(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.LoadEventInfo, param2: com.google.android.exoplayer2.source.MediaLoadData, param3: java.io.IOException, param4: boolean): void;
						public onRepeatModeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onShuffleModeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean): void;
						public onPlaylistMetadataChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.MediaMetadata): void;
						/** @deprecated */
						public onAudioDecoderInitialized(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string, param2: number): void;
						/** @deprecated */
						public onPositionDiscontinuity(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onVideoCodecError(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: java.lang.Exception): void;
						/** @deprecated */
						public onVideoSizeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: number, param3: number, param4: number): void;
						/** @deprecated */
						public onDecoderEnabled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onAudioDisabled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onDeviceInfoChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.DeviceInfo): void;
						public onDrmKeysRestored(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						public onSessionCreated(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string): void;
						public onSkipSilenceEnabledChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean): void;
						public onVideoSizeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.video.VideoSize): void;
						public onPlayerError(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.PlaybackException): void;
						public onPlaybackParametersChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.PlaybackParameters): void;
						public onMediaMetadataChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.MediaMetadata): void;
						public onAudioAttributesChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.audio.AudioAttributes): void;
						public onSurfaceSizeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: number): void;
						/** @deprecated */
						public onLoadingChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean): void;
						public onSeekForwardIncrementChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onDrmSessionManagerError(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: java.lang.Exception): void;
						public onPositionDiscontinuity(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Player.PositionInfo, param2: com.google.android.exoplayer2.Player.PositionInfo, param3: number): void;
						public onAvailableCommandsChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Player.Commands): void;
						public onPlaybackStateChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onAudioSessionIdChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						/** @deprecated */
						public onDrmSessionAcquired(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						public onAudioEnabled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onVideoDecoderReleased(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string): void;
						public onAudioSinkError(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: java.lang.Exception): void;
						public onMetadata(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.metadata.Metadata): void;
						public onAudioUnderrun(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: number, param3: number): void;
						/** @deprecated */
						public onDecoderInitialized(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: string, param3: number): void;
						public onDeviceVolumeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: boolean): void;
					}
					export module PlaybackStatsListener {
						export class Callback {
							public static class: java.lang.Class<com.google.android.exoplayer2.analytics.PlaybackStatsListener.Callback>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.analytics.PlaybackStatsListener$Callback interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onPlaybackStatsReady(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.analytics.PlaybackStats): void;
							});
							public constructor();
							public onPlaybackStatsReady(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.analytics.PlaybackStats): void;
						}
						export class PlaybackStatsTracker {
							public static class: java.lang.Class<com.google.android.exoplayer2.analytics.PlaybackStatsListener.PlaybackStatsTracker>;
							public build(param0: boolean): com.google.android.exoplayer2.analytics.PlaybackStats;
							public constructor(param0: boolean, param1: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime);
							public onEvents(param0: com.google.android.exoplayer2.Player, param1: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param2: boolean, param3: number, param4: boolean, param5: number, param6: boolean, param7: boolean, param8: com.google.android.exoplayer2.PlaybackException, param9: java.lang.Exception, param10: number, param11: number, param12: com.google.android.exoplayer2.Format, param13: com.google.android.exoplayer2.Format, param14: com.google.android.exoplayer2.video.VideoSize): void;
							public onFinished(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean, param2: number): void;
							public onForeground(): void;
							public onInterruptedByAd(): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module analytics {
					export class PlayerId {
						public static class: java.lang.Class<com.google.android.exoplayer2.analytics.PlayerId>;
						public static UNSET: com.google.android.exoplayer2.analytics.PlayerId;
						public constructor(param0: globalAndroid.media.metrics.LogSessionId);
						public constructor();
						public getLogSessionId(): globalAndroid.media.metrics.LogSessionId;
					}
					export module PlayerId {
						export class LogSessionIdApi31 {
							public static class: java.lang.Class<com.google.android.exoplayer2.analytics.PlayerId.LogSessionIdApi31>;
							public static UNSET: com.google.android.exoplayer2.analytics.PlayerId.LogSessionIdApi31;
							public logSessionId: globalAndroid.media.metrics.LogSessionId;
							public constructor(param0: globalAndroid.media.metrics.LogSessionId);
						}
					}
				}
			}
		}
	}
}


declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module audio {
					export class AudioCapabilities {
						public static class: java.lang.Class<com.google.android.exoplayer2.audio.AudioCapabilities>;
						public static DEFAULT_AUDIO_CAPABILITIES: com.google.android.exoplayer2.audio.AudioCapabilities;
						public getMaxChannelCount(): number;
						public constructor(param0: androidNative.Array<number>, param1: number);
						public supportsEncoding(param0: number): boolean;
						public static getCapabilities(param0: globalAndroid.content.Context): com.google.android.exoplayer2.audio.AudioCapabilities;
						public equals(param0: any): boolean;
						public hashCode(): number;
						public toString(): string;
					}
					export module AudioCapabilities {
						export class Api29 {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.AudioCapabilities.Api29>;
							public static getDirectPlaybackSupportedEncodings(): androidNative.Array<number>;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module audio {
					export class AudioCapabilitiesReceiver {
						public static class: java.lang.Class<com.google.android.exoplayer2.audio.AudioCapabilitiesReceiver>;
						public unregister(): void;
						public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.audio.AudioCapabilitiesReceiver.Listener);
						public register(): com.google.android.exoplayer2.audio.AudioCapabilities;
					}
					export module AudioCapabilitiesReceiver {
						export class ExternalSurroundSoundSettingObserver {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.AudioCapabilitiesReceiver.ExternalSurroundSoundSettingObserver>;
							public unregister(): void;
							public onChange(param0: boolean): void;
							public constructor(param0: com.google.android.exoplayer2.audio.AudioCapabilitiesReceiver, param1: globalAndroid.os.Handler, param2: globalAndroid.content.ContentResolver, param3: globalAndroid.net.Uri);
							public register(): void;
						}
						export class HdmiAudioPlugBroadcastReceiver {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.AudioCapabilitiesReceiver.HdmiAudioPlugBroadcastReceiver>;
							public onReceive(param0: globalAndroid.content.Context, param1: globalAndroid.content.Intent): void;
						}
						export class Listener {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.AudioCapabilitiesReceiver.Listener>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.audio.AudioCapabilitiesReceiver$Listener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onAudioCapabilitiesChanged(param0: com.google.android.exoplayer2.audio.AudioCapabilities): void;
							});
							public constructor();
							public onAudioCapabilitiesChanged(param0: com.google.android.exoplayer2.audio.AudioCapabilities): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module audio {
					export class AudioProcessor {
						public static class: java.lang.Class<com.google.android.exoplayer2.audio.AudioProcessor>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.audio.AudioProcessor interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							configure(param0: com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat): com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat;
							isActive(): boolean;
							queueInput(param0: java.nio.ByteBuffer): void;
							queueEndOfStream(): void;
							getOutput(): java.nio.ByteBuffer;
							isEnded(): boolean;
							flush(): void;
							reset(): void;
							<clinit>(): void;
						});
						public constructor();
						public static EMPTY_BUFFER: java.nio.ByteBuffer;
						public queueEndOfStream(): void;
						public getOutput(): java.nio.ByteBuffer;
						public flush(): void;
						public queueInput(param0: java.nio.ByteBuffer): void;
						public isEnded(): boolean;
						public reset(): void;
						public configure(param0: com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat): com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat;
						public isActive(): boolean;
					}
					export module AudioProcessor {
						export class AudioFormat {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat>;
							public static NOT_SET: com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat;
							public sampleRate: number;
							public channelCount: number;
							public encoding: number;
							public bytesPerFrame: number;
							public constructor(param0: number, param1: number, param2: number);
							public toString(): string;
						}
						export class UnhandledAudioFormatException {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.AudioProcessor.UnhandledAudioFormatException>;
							public constructor(param0: com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat);
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module audio {
					export class AudioRendererEventListener {
						public static class: java.lang.Class<com.google.android.exoplayer2.audio.AudioRendererEventListener>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.audio.AudioRendererEventListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							onAudioEnabled(param0: com.google.android.exoplayer2.decoder.DecoderCounters): void;
							onAudioDecoderInitialized(param0: string, param1: number, param2: number): void;
							onAudioInputFormatChanged(param0: com.google.android.exoplayer2.Format): void;
							onAudioInputFormatChanged(param0: com.google.android.exoplayer2.Format, param1: com.google.android.exoplayer2.decoder.DecoderReuseEvaluation): void;
							onAudioPositionAdvancing(param0: number): void;
							onAudioUnderrun(param0: number, param1: number, param2: number): void;
							onAudioDecoderReleased(param0: string): void;
							onAudioDisabled(param0: com.google.android.exoplayer2.decoder.DecoderCounters): void;
							onSkipSilenceEnabledChanged(param0: boolean): void;
							onAudioCodecError(param0: java.lang.Exception): void;
							onAudioSinkError(param0: java.lang.Exception): void;
						});
						public constructor();
						public onAudioEnabled(param0: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						/** @deprecated */
						public onAudioInputFormatChanged(param0: com.google.android.exoplayer2.Format): void;
						public onAudioSinkError(param0: java.lang.Exception): void;
						public onAudioInputFormatChanged(param0: com.google.android.exoplayer2.Format, param1: com.google.android.exoplayer2.decoder.DecoderReuseEvaluation): void;
						public onAudioCodecError(param0: java.lang.Exception): void;
						public onAudioPositionAdvancing(param0: number): void;
						public onSkipSilenceEnabledChanged(param0: boolean): void;
						public onAudioUnderrun(param0: number, param1: number, param2: number): void;
						public onAudioDecoderReleased(param0: string): void;
						public onAudioDisabled(param0: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onAudioDecoderInitialized(param0: string, param1: number, param2: number): void;
					}
					export module AudioRendererEventListener {
						export class EventDispatcher {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.AudioRendererEventListener.EventDispatcher>;
							public audioSinkError(param0: java.lang.Exception): void;
							public constructor(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.audio.AudioRendererEventListener);
							public skipSilenceEnabledChanged(param0: boolean): void;
							public enabled(param0: com.google.android.exoplayer2.decoder.DecoderCounters): void;
							public audioCodecError(param0: java.lang.Exception): void;
							public underrun(param0: number, param1: number, param2: number): void;
							public disabled(param0: com.google.android.exoplayer2.decoder.DecoderCounters): void;
							public decoderInitialized(param0: string, param1: number, param2: number): void;
							public decoderReleased(param0: string): void;
							public inputFormatChanged(param0: com.google.android.exoplayer2.Format, param1: com.google.android.exoplayer2.decoder.DecoderReuseEvaluation): void;
							public positionAdvancing(param0: number): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module audio {
					export class AudioSink {
						public static class: java.lang.Class<com.google.android.exoplayer2.audio.AudioSink>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.audio.AudioSink interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							setListener(param0: com.google.android.exoplayer2.audio.AudioSink.Listener): void;
							setPlayerId(param0: com.google.android.exoplayer2.analytics.PlayerId): void;
							supportsFormat(param0: com.google.android.exoplayer2.Format): boolean;
							getFormatSupport(param0: com.google.android.exoplayer2.Format): number;
							getCurrentPositionUs(param0: boolean): number;
							configure(param0: com.google.android.exoplayer2.Format, param1: number, param2: androidNative.Array<number>): void;
							play(): void;
							handleDiscontinuity(): void;
							handleBuffer(param0: java.nio.ByteBuffer, param1: number, param2: number): boolean;
							playToEndOfStream(): void;
							isEnded(): boolean;
							hasPendingData(): boolean;
							setPlaybackParameters(param0: com.google.android.exoplayer2.PlaybackParameters): void;
							getPlaybackParameters(): com.google.android.exoplayer2.PlaybackParameters;
							setSkipSilenceEnabled(param0: boolean): void;
							getSkipSilenceEnabled(): boolean;
							setAudioAttributes(param0: com.google.android.exoplayer2.audio.AudioAttributes): void;
							getAudioAttributes(): com.google.android.exoplayer2.audio.AudioAttributes;
							setAudioSessionId(param0: number): void;
							setAuxEffectInfo(param0: com.google.android.exoplayer2.audio.AuxEffectInfo): void;
							enableTunnelingV21(): void;
							disableTunneling(): void;
							setVolume(param0: number): void;
							pause(): void;
							flush(): void;
							experimentalFlushWithoutAudioTrackRelease(): void;
							reset(): void;
						});
						public constructor();
						public static SINK_FORMAT_UNSUPPORTED: number;
						public static SINK_FORMAT_SUPPORTED_DIRECTLY: number;
						public static CURRENT_POSITION_NOT_SET: number;
						public static SINK_FORMAT_SUPPORTED_WITH_TRANSCODING: number;
						public setAudioAttributes(param0: com.google.android.exoplayer2.audio.AudioAttributes): void;
						public pause(): void;
						public setPlayerId(param0: com.google.android.exoplayer2.analytics.PlayerId): void;
						public handleDiscontinuity(): void;
						public getFormatSupport(param0: com.google.android.exoplayer2.Format): number;
						public getPlaybackParameters(): com.google.android.exoplayer2.PlaybackParameters;
						public setPlaybackParameters(param0: com.google.android.exoplayer2.PlaybackParameters): void;
						public setAuxEffectInfo(param0: com.google.android.exoplayer2.audio.AuxEffectInfo): void;
						public hasPendingData(): boolean;
						public setListener(param0: com.google.android.exoplayer2.audio.AudioSink.Listener): void;
						public isEnded(): boolean;
						public disableTunneling(): void;
						public setAudioSessionId(param0: number): void;
						public getCurrentPositionUs(param0: boolean): number;
						public getAudioAttributes(): com.google.android.exoplayer2.audio.AudioAttributes;
						public playToEndOfStream(): void;
						public experimentalFlushWithoutAudioTrackRelease(): void;
						public flush(): void;
						public setSkipSilenceEnabled(param0: boolean): void;
						public handleBuffer(param0: java.nio.ByteBuffer, param1: number, param2: number): boolean;
						public supportsFormat(param0: com.google.android.exoplayer2.Format): boolean;
						public configure(param0: com.google.android.exoplayer2.Format, param1: number, param2: androidNative.Array<number>): void;
						public enableTunnelingV21(): void;
						public play(): void;
						public setVolume(param0: number): void;
						public reset(): void;
						public getSkipSilenceEnabled(): boolean;
					}
					export module AudioSink {
						export class ConfigurationException {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.AudioSink.ConfigurationException>;
							public format: com.google.android.exoplayer2.Format;
							public constructor(param0: string, param1: com.google.android.exoplayer2.Format);
							public constructor(param0: java.lang.Throwable, param1: com.google.android.exoplayer2.Format);
						}
						export class InitializationException {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.AudioSink.InitializationException>;
							public audioTrackState: number;
							public isRecoverable: boolean;
							public format: com.google.android.exoplayer2.Format;
							public constructor(param0: number, param1: number, param2: number, param3: number, param4: com.google.android.exoplayer2.Format, param5: boolean, param6: java.lang.Exception);
						}
						export class Listener {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.AudioSink.Listener>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.audio.AudioSink$Listener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onPositionDiscontinuity(): void;
								onPositionAdvancing(param0: number): void;
								onUnderrun(param0: number, param1: number, param2: number): void;
								onSkipSilenceEnabledChanged(param0: boolean): void;
								onOffloadBufferEmptying(): void;
								onOffloadBufferFull(param0: number): void;
								onAudioSinkError(param0: java.lang.Exception): void;
							});
							public constructor();
							public onPositionAdvancing(param0: number): void;
							public onOffloadBufferEmptying(): void;
							public onSkipSilenceEnabledChanged(param0: boolean): void;
							public onOffloadBufferFull(param0: number): void;
							public onAudioSinkError(param0: java.lang.Exception): void;
							public onPositionDiscontinuity(): void;
							public onUnderrun(param0: number, param1: number, param2: number): void;
						}
						export class SinkFormatSupport {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.AudioSink.SinkFormatSupport>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.audio.AudioSink$SinkFormatSupport interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
							});
							public constructor();
						}
						export class UnexpectedDiscontinuityException {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.AudioSink.UnexpectedDiscontinuityException>;
							public actualPresentationTimeUs: number;
							public expectedPresentationTimeUs: number;
							public constructor(param0: number, param1: number);
						}
						export class WriteException {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.AudioSink.WriteException>;
							public errorCode: number;
							public isRecoverable: boolean;
							public format: com.google.android.exoplayer2.Format;
							public constructor(param0: number, param1: com.google.android.exoplayer2.Format, param2: boolean);
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module audio {
					export class AudioTimestampPoller {
						public static class: java.lang.Class<com.google.android.exoplayer2.audio.AudioTimestampPoller>;
						public getTimestampPositionFrames(): number;
						public acceptTimestamp(): void;
						public hasTimestamp(): boolean;
						public maybePollTimestamp(param0: number): boolean;
						public rejectTimestamp(): void;
						public hasAdvancingTimestamp(): boolean;
						public getTimestampSystemTimeUs(): number;
						public constructor(param0: globalAndroid.media.AudioTrack);
						public reset(): void;
					}
					export module AudioTimestampPoller {
						export class AudioTimestampV19 {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.AudioTimestampPoller.AudioTimestampV19>;
							public getTimestampSystemTimeUs(): number;
							public maybeUpdateTimestamp(): boolean;
							public constructor(param0: globalAndroid.media.AudioTrack);
							public getTimestampPositionFrames(): number;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module audio {
					export class AudioTrackPositionTracker {
						public static class: java.lang.Class<com.google.android.exoplayer2.audio.AudioTrackPositionTracker>;
						public constructor(param0: com.google.android.exoplayer2.audio.AudioTrackPositionTracker.Listener);
						public getCurrentPositionUs(param0: boolean): number;
						public getPendingBufferDurationMs(param0: number): number;
						public isPlaying(): boolean;
						public setAudioTrackPlaybackSpeed(param0: number): void;
						public setAudioTrack(param0: globalAndroid.media.AudioTrack, param1: boolean, param2: number, param3: number, param4: number): void;
						public pause(): boolean;
						public mayHandleBuffer(param0: number): boolean;
						public hasPendingData(param0: number): boolean;
						public isStalled(param0: number): boolean;
						public start(): void;
						public handleEndOfStream(param0: number): void;
						public getAvailableBufferSize(param0: number): number;
						public reset(): void;
					}
					export module AudioTrackPositionTracker {
						export class Listener {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.AudioTrackPositionTracker.Listener>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.audio.AudioTrackPositionTracker$Listener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onPositionAdvancing(param0: number): void;
								onPositionFramesMismatch(param0: number, param1: number, param2: number, param3: number): void;
								onSystemTimeUsMismatch(param0: number, param1: number, param2: number, param3: number): void;
								onInvalidLatency(param0: number): void;
								onUnderrun(param0: number, param1: number): void;
							});
							public constructor();
							public onPositionAdvancing(param0: number): void;
							public onSystemTimeUsMismatch(param0: number, param1: number, param2: number, param3: number): void;
							public onUnderrun(param0: number, param1: number): void;
							public onInvalidLatency(param0: number): void;
							public onPositionFramesMismatch(param0: number, param1: number, param2: number, param3: number): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module audio {
					export abstract class BaseAudioProcessor extends com.google.android.exoplayer2.audio.AudioProcessor {
						public static class: java.lang.Class<com.google.android.exoplayer2.audio.BaseAudioProcessor>;
						public inputAudioFormat: com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat;
						public outputAudioFormat: com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat;
						public constructor();
						public queueEndOfStream(): void;
						public onConfigure(param0: com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat): com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat;
						public flush(): void;
						public queueInput(param0: java.nio.ByteBuffer): void;
						public onQueueEndOfStream(): void;
						public replaceOutputBuffer(param0: number): java.nio.ByteBuffer;
						public configure(param0: com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat): com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat;
						public hasPendingOutput(): boolean;
						public onFlush(): void;
						public getOutput(): java.nio.ByteBuffer;
						public onReset(): void;
						public isEnded(): boolean;
						public reset(): void;
						public isActive(): boolean;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module audio {
					export class ChannelMappingAudioProcessor extends com.google.android.exoplayer2.audio.BaseAudioProcessor {
						public static class: java.lang.Class<com.google.android.exoplayer2.audio.ChannelMappingAudioProcessor>;
						public onFlush(): void;
						public queueEndOfStream(): void;
						public getOutput(): java.nio.ByteBuffer;
						public setChannelMap(param0: androidNative.Array<number>): void;
						public onConfigure(param0: com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat): com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat;
						public onReset(): void;
						public flush(): void;
						public queueInput(param0: java.nio.ByteBuffer): void;
						public isEnded(): boolean;
						public reset(): void;
						public configure(param0: com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat): com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat;
						public isActive(): boolean;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module audio {
					export abstract class DecoderAudioRenderer<T>  extends com.google.android.exoplayer2.BaseRenderer implements com.google.android.exoplayer2.util.MediaClock  {
						public static class: java.lang.Class<com.google.android.exoplayer2.audio.DecoderAudioRenderer<any>>;
						public sinkSupportsFormat(param0: com.google.android.exoplayer2.Format): boolean;
						public onPositionReset(param0: number, param1: boolean): void;
						public replaceStream(param0: androidNative.Array<com.google.android.exoplayer2.Format>, param1: com.google.android.exoplayer2.source.SampleStream, param2: number, param3: number): void;
						public getPositionUs(): number;
						public static getHardwareAccelerationSupport(param0: number): number;
						public canReuseDecoder(param0: string, param1: com.google.android.exoplayer2.Format, param2: com.google.android.exoplayer2.Format): com.google.android.exoplayer2.decoder.DecoderReuseEvaluation;
						public static create(param0: number): number;
						public getPlaybackParameters(): com.google.android.exoplayer2.PlaybackParameters;
						public experimentalSetEnableKeepAudioTrackOnSeek(param0: boolean): void;
						public static getTunnelingSupport(param0: number): number;
						public getStream(): com.google.android.exoplayer2.source.SampleStream;
						public onDisabled(): void;
						public isEnded(): boolean;
						public setPlaybackSpeed(param0: number, param1: number): void;
						public constructor();
						public onPositionDiscontinuity(): void;
						public setCurrentStreamFinal(): void;
						public isReady(): boolean;
						public disable(): void;
						public handleMessage(param0: number, param1: any): void;
						public supportsFormat(param0: com.google.android.exoplayer2.Format): number;
						public start(): void;
						public getState(): number;
						public static create(param0: number, param1: number, param2: number, param3: number, param4: number): number;
						public onStopped(): void;
						public getReadingPositionUs(): number;
						public constructor(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.audio.AudioRendererEventListener, param2: androidNative.Array<com.google.android.exoplayer2.audio.AudioProcessor>);
						public reset(): void;
						public constructor(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.audio.AudioRendererEventListener, param2: com.google.android.exoplayer2.audio.AudioSink);
						public onQueueInputBuffer(param0: com.google.android.exoplayer2.decoder.DecoderInputBuffer): void;
						public onStarted(): void;
						public render(param0: number, param1: number): void;
						public getName(): string;
						public getMediaClock(): com.google.android.exoplayer2.util.MediaClock;
						public isCurrentStreamFinal(): boolean;
						public setPlaybackParameters(param0: com.google.android.exoplayer2.PlaybackParameters): void;
						public static getAdaptiveSupport(param0: number): number;
						public getCapabilities(): com.google.android.exoplayer2.RendererCapabilities;
						public constructor(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.audio.AudioRendererEventListener, param2: com.google.android.exoplayer2.audio.AudioCapabilities, param3: androidNative.Array<com.google.android.exoplayer2.audio.AudioProcessor>);
						public enable(param0: com.google.android.exoplayer2.RendererConfiguration, param1: androidNative.Array<com.google.android.exoplayer2.Format>, param2: com.google.android.exoplayer2.source.SampleStream, param3: number, param4: boolean, param5: boolean, param6: number, param7: number): void;
						public getOutputFormat(param0: any): com.google.android.exoplayer2.Format;
						public resetPosition(param0: number): void;
						public init(param0: number, param1: com.google.android.exoplayer2.analytics.PlayerId): void;
						public hasReadStreamToEnd(): boolean;
						public getTrackType(): number;
						public supportsMixedMimeTypeAdaptation(): number;
						public stop(): void;
						public createDecoder(param0: com.google.android.exoplayer2.Format, param1: com.google.android.exoplayer2.decoder.CryptoConfig): any;
						public getSinkFormatSupport(param0: com.google.android.exoplayer2.Format): number;
						public onEnabled(param0: boolean, param1: boolean): void;
						public static getFormatSupport(param0: number): number;
						public static create(param0: number, param1: number, param2: number): number;
						public constructor(param0: number);
						public maybeThrowStreamError(): void;
						public static getDecoderSupport(param0: number): number;
						public supportsFormatInternal(param0: com.google.android.exoplayer2.Format): number;
					}
					export module DecoderAudioRenderer {
						export class AudioSinkListener extends com.google.android.exoplayer2.audio.AudioSink.Listener {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.DecoderAudioRenderer.AudioSinkListener>;
							public onPositionAdvancing(param0: number): void;
							public onOffloadBufferEmptying(): void;
							public onSkipSilenceEnabledChanged(param0: boolean): void;
							public onOffloadBufferFull(param0: number): void;
							public onAudioSinkError(param0: java.lang.Exception): void;
							public onPositionDiscontinuity(): void;
							public onUnderrun(param0: number, param1: number, param2: number): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module audio {
					export class DefaultAudioSink extends com.google.android.exoplayer2.audio.AudioSink {
						public static class: java.lang.Class<com.google.android.exoplayer2.audio.DefaultAudioSink>;
						public static DEFAULT_PLAYBACK_SPEED: number;
						public static MIN_PLAYBACK_SPEED: number;
						public static MAX_PLAYBACK_SPEED: number;
						public static MIN_PITCH: number;
						public static MAX_PITCH: number;
						public static OFFLOAD_MODE_DISABLED: number;
						public static OFFLOAD_MODE_ENABLED_GAPLESS_REQUIRED: number;
						public static OFFLOAD_MODE_ENABLED_GAPLESS_NOT_REQUIRED: number;
						public static OFFLOAD_MODE_ENABLED_GAPLESS_DISABLED: number;
						public static OUTPUT_MODE_PCM: number;
						public static OUTPUT_MODE_OFFLOAD: number;
						public static OUTPUT_MODE_PASSTHROUGH: number;
						public static failOnSpuriousAudioTimestamp: boolean;
						public setAudioAttributes(param0: com.google.android.exoplayer2.audio.AudioAttributes): void;
						public pause(): void;
						public setPlayerId(param0: com.google.android.exoplayer2.analytics.PlayerId): void;
						public handleDiscontinuity(): void;
						/** @deprecated */
						public constructor(param0: com.google.android.exoplayer2.audio.AudioCapabilities, param1: androidNative.Array<com.google.android.exoplayer2.audio.AudioProcessor>, param2: boolean);
						public getFormatSupport(param0: com.google.android.exoplayer2.Format): number;
						public getPlaybackParameters(): com.google.android.exoplayer2.PlaybackParameters;
						public setPlaybackParameters(param0: com.google.android.exoplayer2.PlaybackParameters): void;
						public setAuxEffectInfo(param0: com.google.android.exoplayer2.audio.AuxEffectInfo): void;
						/** @deprecated */
						public constructor(param0: com.google.android.exoplayer2.audio.AudioCapabilities, param1: com.google.android.exoplayer2.audio.DefaultAudioSink.AudioProcessorChain, param2: boolean, param3: boolean, param4: number);
						public hasPendingData(): boolean;
						public setListener(param0: com.google.android.exoplayer2.audio.AudioSink.Listener): void;
						public isEnded(): boolean;
						public disableTunneling(): void;
						/** @deprecated */
						public constructor(param0: com.google.android.exoplayer2.audio.AudioCapabilities, param1: androidNative.Array<com.google.android.exoplayer2.audio.AudioProcessor>);
						public setAudioSessionId(param0: number): void;
						public getCurrentPositionUs(param0: boolean): number;
						public getAudioAttributes(): com.google.android.exoplayer2.audio.AudioAttributes;
						public playToEndOfStream(): void;
						public experimentalFlushWithoutAudioTrackRelease(): void;
						public flush(): void;
						public setSkipSilenceEnabled(param0: boolean): void;
						public handleBuffer(param0: java.nio.ByteBuffer, param1: number, param2: number): boolean;
						public supportsFormat(param0: com.google.android.exoplayer2.Format): boolean;
						public configure(param0: com.google.android.exoplayer2.Format, param1: number, param2: androidNative.Array<number>): void;
						public enableTunnelingV21(): void;
						public play(): void;
						public setVolume(param0: number): void;
						public reset(): void;
						public getSkipSilenceEnabled(): boolean;
					}
					export module DefaultAudioSink {
						export class Api31 {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.DefaultAudioSink.Api31>;
							public static setLogSessionIdOnAudioTrack(param0: globalAndroid.media.AudioTrack, param1: com.google.android.exoplayer2.analytics.PlayerId): void;
						}
						export class AudioProcessorChain {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.DefaultAudioSink.AudioProcessorChain>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.audio.DefaultAudioSink$AudioProcessorChain interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								getAudioProcessors(): androidNative.Array<com.google.android.exoplayer2.audio.AudioProcessor>;
								applyPlaybackParameters(param0: com.google.android.exoplayer2.PlaybackParameters): com.google.android.exoplayer2.PlaybackParameters;
								applySkipSilenceEnabled(param0: boolean): boolean;
								getMediaDuration(param0: number): number;
								getSkippedOutputFrameCount(): number;
							});
							public constructor();
							public getAudioProcessors(): androidNative.Array<com.google.android.exoplayer2.audio.AudioProcessor>;
							public applySkipSilenceEnabled(param0: boolean): boolean;
							public getMediaDuration(param0: number): number;
							public getSkippedOutputFrameCount(): number;
							public applyPlaybackParameters(param0: com.google.android.exoplayer2.PlaybackParameters): com.google.android.exoplayer2.PlaybackParameters;
						}
						export class AudioTrackBufferSizeProvider {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.DefaultAudioSink.AudioTrackBufferSizeProvider>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.audio.DefaultAudioSink$AudioTrackBufferSizeProvider interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								getBufferSizeInBytes(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number): number;
								<clinit>(): void;
							});
							public constructor();
							public static DEFAULT: com.google.android.exoplayer2.audio.DefaultAudioSink.AudioTrackBufferSizeProvider;
							public getBufferSizeInBytes(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number): number;
						}
						export class Builder {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.DefaultAudioSink.Builder>;
							public constructor();
							public setAudioProcessorChain(param0: com.google.android.exoplayer2.audio.DefaultAudioSink.AudioProcessorChain): com.google.android.exoplayer2.audio.DefaultAudioSink.Builder;
							public setAudioCapabilities(param0: com.google.android.exoplayer2.audio.AudioCapabilities): com.google.android.exoplayer2.audio.DefaultAudioSink.Builder;
							public setAudioTrackBufferSizeProvider(param0: com.google.android.exoplayer2.audio.DefaultAudioSink.AudioTrackBufferSizeProvider): com.google.android.exoplayer2.audio.DefaultAudioSink.Builder;
							public setOffloadMode(param0: number): com.google.android.exoplayer2.audio.DefaultAudioSink.Builder;
							public setEnableAudioTrackPlaybackParams(param0: boolean): com.google.android.exoplayer2.audio.DefaultAudioSink.Builder;
							public setAudioProcessors(param0: androidNative.Array<com.google.android.exoplayer2.audio.AudioProcessor>): com.google.android.exoplayer2.audio.DefaultAudioSink.Builder;
							public build(): com.google.android.exoplayer2.audio.DefaultAudioSink;
							public setEnableFloatOutput(param0: boolean): com.google.android.exoplayer2.audio.DefaultAudioSink.Builder;
						}
						export class Configuration {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.DefaultAudioSink.Configuration>;
							public inputFormat: com.google.android.exoplayer2.Format;
							public inputPcmFrameSize: number;
							public outputMode: number;
							public outputPcmFrameSize: number;
							public outputSampleRate: number;
							public outputChannelConfig: number;
							public outputEncoding: number;
							public bufferSize: number;
							public availableAudioProcessors: androidNative.Array<com.google.android.exoplayer2.audio.AudioProcessor>;
							public buildAudioTrack(param0: boolean, param1: com.google.android.exoplayer2.audio.AudioAttributes, param2: number): globalAndroid.media.AudioTrack;
							public inputFramesToDurationUs(param0: number): number;
							public constructor(param0: com.google.android.exoplayer2.Format, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: androidNative.Array<com.google.android.exoplayer2.audio.AudioProcessor>);
							public copyWithBufferSize(param0: number): com.google.android.exoplayer2.audio.DefaultAudioSink.Configuration;
							public canReuseAudioTrack(param0: com.google.android.exoplayer2.audio.DefaultAudioSink.Configuration): boolean;
							public outputModeIsOffload(): boolean;
							public framesToDurationUs(param0: number): number;
						}
						export class DefaultAudioProcessorChain extends com.google.android.exoplayer2.audio.DefaultAudioSink.AudioProcessorChain {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.DefaultAudioSink.DefaultAudioProcessorChain>;
							public getAudioProcessors(): androidNative.Array<com.google.android.exoplayer2.audio.AudioProcessor>;
							public constructor(param0: androidNative.Array<com.google.android.exoplayer2.audio.AudioProcessor>);
							public applySkipSilenceEnabled(param0: boolean): boolean;
							public getMediaDuration(param0: number): number;
							public getSkippedOutputFrameCount(): number;
							public constructor(param0: androidNative.Array<com.google.android.exoplayer2.audio.AudioProcessor>, param1: com.google.android.exoplayer2.audio.SilenceSkippingAudioProcessor, param2: com.google.android.exoplayer2.audio.SonicAudioProcessor);
							public applyPlaybackParameters(param0: com.google.android.exoplayer2.PlaybackParameters): com.google.android.exoplayer2.PlaybackParameters;
						}
						export class InvalidAudioTrackTimestampException {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.DefaultAudioSink.InvalidAudioTrackTimestampException>;
						}
						export class MediaPositionParameters {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.DefaultAudioSink.MediaPositionParameters>;
							public playbackParameters: com.google.android.exoplayer2.PlaybackParameters;
							public skipSilence: boolean;
							public mediaTimeUs: number;
							public audioTrackPositionUs: number;
						}
						export class OffloadMode {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.DefaultAudioSink.OffloadMode>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.audio.DefaultAudioSink$OffloadMode interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
							});
							public constructor();
						}
						export class OutputMode {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.DefaultAudioSink.OutputMode>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.audio.DefaultAudioSink$OutputMode interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
							});
							public constructor();
						}
						export class PendingExceptionHolder<T>  extends java.lang.Object {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.DefaultAudioSink.PendingExceptionHolder<any>>;
							public clear(): void;
							public throwExceptionIfDeadlineIsReached(param0: T): void;
							public constructor(param0: number);
						}
						export class PositionTrackerListener extends com.google.android.exoplayer2.audio.AudioTrackPositionTracker.Listener {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.DefaultAudioSink.PositionTrackerListener>;
							public onPositionAdvancing(param0: number): void;
							public onSystemTimeUsMismatch(param0: number, param1: number, param2: number, param3: number): void;
							public onUnderrun(param0: number, param1: number): void;
							public onInvalidLatency(param0: number): void;
							public onPositionFramesMismatch(param0: number, param1: number, param2: number, param3: number): void;
						}
						export class StreamEventCallbackV29 {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.DefaultAudioSink.StreamEventCallbackV29>;
							public unregister(param0: globalAndroid.media.AudioTrack): void;
							public constructor(param0: com.google.android.exoplayer2.audio.DefaultAudioSink);
							public register(param0: globalAndroid.media.AudioTrack): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module audio {
					export class DefaultAudioTrackBufferSizeProvider extends com.google.android.exoplayer2.audio.DefaultAudioSink.AudioTrackBufferSizeProvider {
						public static class: java.lang.Class<com.google.android.exoplayer2.audio.DefaultAudioTrackBufferSizeProvider>;
						public minPcmBufferDurationUs: number;
						public maxPcmBufferDurationUs: number;
						public pcmBufferMultiplicationFactor: number;
						public passthroughBufferDurationUs: number;
						public offloadBufferDurationUs: number;
						public ac3BufferMultiplicationFactor: number;
						public static durationUsToBytes(param0: number, param1: number, param2: number): number;
						public getPcmBufferSizeInBytes(param0: number, param1: number, param2: number): number;
						public getOffloadBufferSizeInBytes(param0: number): number;
						public constructor(param0: com.google.android.exoplayer2.audio.DefaultAudioTrackBufferSizeProvider.Builder);
						public get1xBufferSizeInBytes(param0: number, param1: number, param2: number, param3: number, param4: number): number;
						public static getMaximumEncodedRateBytesPerSecond(param0: number): number;
						public getBufferSizeInBytes(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number): number;
						public getPassthroughBufferSizeInBytes(param0: number): number;
					}
					export module DefaultAudioTrackBufferSizeProvider {
						export class Builder {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.DefaultAudioTrackBufferSizeProvider.Builder>;
							public constructor();
							public setPcmBufferMultiplicationFactor(param0: number): com.google.android.exoplayer2.audio.DefaultAudioTrackBufferSizeProvider.Builder;
							public setPassthroughBufferDurationUs(param0: number): com.google.android.exoplayer2.audio.DefaultAudioTrackBufferSizeProvider.Builder;
							public build(): com.google.android.exoplayer2.audio.DefaultAudioTrackBufferSizeProvider;
							public setMinPcmBufferDurationUs(param0: number): com.google.android.exoplayer2.audio.DefaultAudioTrackBufferSizeProvider.Builder;
							public setAc3BufferMultiplicationFactor(param0: number): com.google.android.exoplayer2.audio.DefaultAudioTrackBufferSizeProvider.Builder;
							public setMaxPcmBufferDurationUs(param0: number): com.google.android.exoplayer2.audio.DefaultAudioTrackBufferSizeProvider.Builder;
							public setOffloadBufferDurationUs(param0: number): com.google.android.exoplayer2.audio.DefaultAudioTrackBufferSizeProvider.Builder;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module audio {
					export class FloatResamplingAudioProcessor extends com.google.android.exoplayer2.audio.BaseAudioProcessor {
						public static class: java.lang.Class<com.google.android.exoplayer2.audio.FloatResamplingAudioProcessor>;
						public queueEndOfStream(): void;
						public getOutput(): java.nio.ByteBuffer;
						public onConfigure(param0: com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat): com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat;
						public flush(): void;
						public queueInput(param0: java.nio.ByteBuffer): void;
						public isEnded(): boolean;
						public reset(): void;
						public configure(param0: com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat): com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat;
						public isActive(): boolean;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module audio {
					export class ForwardingAudioSink extends com.google.android.exoplayer2.audio.AudioSink {
						public static class: java.lang.Class<com.google.android.exoplayer2.audio.ForwardingAudioSink>;
						public setAudioAttributes(param0: com.google.android.exoplayer2.audio.AudioAttributes): void;
						public pause(): void;
						public setPlayerId(param0: com.google.android.exoplayer2.analytics.PlayerId): void;
						public handleDiscontinuity(): void;
						public getFormatSupport(param0: com.google.android.exoplayer2.Format): number;
						public getPlaybackParameters(): com.google.android.exoplayer2.PlaybackParameters;
						public setPlaybackParameters(param0: com.google.android.exoplayer2.PlaybackParameters): void;
						public setAuxEffectInfo(param0: com.google.android.exoplayer2.audio.AuxEffectInfo): void;
						public hasPendingData(): boolean;
						public setListener(param0: com.google.android.exoplayer2.audio.AudioSink.Listener): void;
						public isEnded(): boolean;
						public disableTunneling(): void;
						public setAudioSessionId(param0: number): void;
						public getCurrentPositionUs(param0: boolean): number;
						public getAudioAttributes(): com.google.android.exoplayer2.audio.AudioAttributes;
						public playToEndOfStream(): void;
						public experimentalFlushWithoutAudioTrackRelease(): void;
						public flush(): void;
						public setSkipSilenceEnabled(param0: boolean): void;
						public handleBuffer(param0: java.nio.ByteBuffer, param1: number, param2: number): boolean;
						public supportsFormat(param0: com.google.android.exoplayer2.Format): boolean;
						public configure(param0: com.google.android.exoplayer2.Format, param1: number, param2: androidNative.Array<number>): void;
						public constructor(param0: com.google.android.exoplayer2.audio.AudioSink);
						public enableTunnelingV21(): void;
						public play(): void;
						public setVolume(param0: number): void;
						public reset(): void;
						public getSkipSilenceEnabled(): boolean;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module audio {
					export class MediaCodecAudioRenderer extends com.google.android.exoplayer2.mediacodec.MediaCodecRenderer implements com.google.android.exoplayer2.util.MediaClock {
						public static class: java.lang.Class<com.google.android.exoplayer2.audio.MediaCodecAudioRenderer>;
						public supportsFormat(param0: com.google.android.exoplayer2.mediacodec.MediaCodecSelector, param1: com.google.android.exoplayer2.Format): number;
						public constructor(param0: number, param1: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.Factory, param2: com.google.android.exoplayer2.mediacodec.MediaCodecSelector, param3: boolean, param4: number);
						public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.mediacodec.MediaCodecSelector, param2: globalAndroid.os.Handler, param3: com.google.android.exoplayer2.audio.AudioRendererEventListener, param4: com.google.android.exoplayer2.audio.AudioCapabilities, param5: androidNative.Array<com.google.android.exoplayer2.audio.AudioProcessor>);
						public onPositionReset(param0: number, param1: boolean): void;
						public replaceStream(param0: androidNative.Array<com.google.android.exoplayer2.Format>, param1: com.google.android.exoplayer2.source.SampleStream, param2: number, param3: number): void;
						public getPositionUs(): number;
						public static getHardwareAccelerationSupport(param0: number): number;
						public static create(param0: number): number;
						public getPlaybackParameters(): com.google.android.exoplayer2.PlaybackParameters;
						public experimentalSetEnableKeepAudioTrackOnSeek(param0: boolean): void;
						public static getTunnelingSupport(param0: number): number;
						public getStream(): com.google.android.exoplayer2.source.SampleStream;
						public getCodecMaxInputSize(param0: com.google.android.exoplayer2.mediacodec.MediaCodecInfo, param1: com.google.android.exoplayer2.Format, param2: androidNative.Array<com.google.android.exoplayer2.Format>): number;
						public onDisabled(): void;
						public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.mediacodec.MediaCodecSelector, param2: globalAndroid.os.Handler, param3: com.google.android.exoplayer2.audio.AudioRendererEventListener);
						public isEnded(): boolean;
						public getDecoderInfos(param0: com.google.android.exoplayer2.mediacodec.MediaCodecSelector, param1: com.google.android.exoplayer2.Format, param2: boolean): java.util.List<com.google.android.exoplayer2.mediacodec.MediaCodecInfo>;
						public setPlaybackSpeed(param0: number, param1: number): void;
						public onPositionDiscontinuity(): void;
						public setCurrentStreamFinal(): void;
						public onCodecInitialized(param0: string, param1: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.Configuration, param2: number, param3: number): void;
						public isReady(): boolean;
						public disable(): void;
						public processOutputBuffer(param0: number, param1: number, param2: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter, param3: java.nio.ByteBuffer, param4: number, param5: number, param6: number, param7: number, param8: boolean, param9: boolean, param10: com.google.android.exoplayer2.Format): boolean;
						public handleMessage(param0: number, param1: any): void;
						public canReuseCodec(param0: com.google.android.exoplayer2.mediacodec.MediaCodecInfo, param1: com.google.android.exoplayer2.Format, param2: com.google.android.exoplayer2.Format): com.google.android.exoplayer2.decoder.DecoderReuseEvaluation;
						public supportsFormat(param0: com.google.android.exoplayer2.Format): number;
						public start(): void;
						public onCodecError(param0: java.lang.Exception): void;
						public getState(): number;
						public onReset(): void;
						public static create(param0: number, param1: number, param2: number, param3: number, param4: number): number;
						public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.mediacodec.MediaCodecSelector, param2: globalAndroid.os.Handler, param3: com.google.android.exoplayer2.audio.AudioRendererEventListener, param4: com.google.android.exoplayer2.audio.AudioSink);
						public onStopped(): void;
						public getReadingPositionUs(): number;
						public reset(): void;
						public onQueueInputBuffer(param0: com.google.android.exoplayer2.decoder.DecoderInputBuffer): void;
						public onStarted(): void;
						public render(param0: number, param1: number): void;
						public getName(): string;
						public getMediaClock(): com.google.android.exoplayer2.util.MediaClock;
						public isCurrentStreamFinal(): boolean;
						public getMediaCodecConfiguration(param0: com.google.android.exoplayer2.mediacodec.MediaCodecInfo, param1: com.google.android.exoplayer2.Format, param2: globalAndroid.media.MediaCrypto, param3: number): com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.Configuration;
						public getMediaFormat(param0: com.google.android.exoplayer2.Format, param1: string, param2: number, param3: number): globalAndroid.media.MediaFormat;
						public shouldUseBypass(param0: com.google.android.exoplayer2.Format): boolean;
						public setPlaybackParameters(param0: com.google.android.exoplayer2.PlaybackParameters): void;
						public static getAdaptiveSupport(param0: number): number;
						public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.mediacodec.MediaCodecSelector);
						public onCodecReleased(param0: string): void;
						public onInputFormatChanged(param0: com.google.android.exoplayer2.FormatHolder): com.google.android.exoplayer2.decoder.DecoderReuseEvaluation;
						public getCapabilities(): com.google.android.exoplayer2.RendererCapabilities;
						public enable(param0: com.google.android.exoplayer2.RendererConfiguration, param1: androidNative.Array<com.google.android.exoplayer2.Format>, param2: com.google.android.exoplayer2.source.SampleStream, param3: number, param4: boolean, param5: boolean, param6: number, param7: number): void;
						public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.Factory, param2: com.google.android.exoplayer2.mediacodec.MediaCodecSelector, param3: boolean, param4: globalAndroid.os.Handler, param5: com.google.android.exoplayer2.audio.AudioRendererEventListener, param6: com.google.android.exoplayer2.audio.AudioSink);
						public resetPosition(param0: number): void;
						public init(param0: number, param1: com.google.android.exoplayer2.analytics.PlayerId): void;
						public hasReadStreamToEnd(): boolean;
						public onProcessedStreamChange(): void;
						public getTrackType(): number;
						public supportsMixedMimeTypeAdaptation(): number;
						public stop(): void;
						public onOutputFormatChanged(param0: com.google.android.exoplayer2.Format, param1: globalAndroid.media.MediaFormat): void;
						public onEnabled(param0: boolean, param1: boolean): void;
						public static getFormatSupport(param0: number): number;
						public static create(param0: number, param1: number, param2: number): number;
						public constructor(param0: number);
						public maybeThrowStreamError(): void;
						public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.mediacodec.MediaCodecSelector, param2: boolean, param3: globalAndroid.os.Handler, param4: com.google.android.exoplayer2.audio.AudioRendererEventListener, param5: com.google.android.exoplayer2.audio.AudioSink);
						public renderToEndOfStream(): void;
						public static getDecoderSupport(param0: number): number;
						public getCodecOperatingRateV23(param0: number, param1: com.google.android.exoplayer2.Format, param2: androidNative.Array<com.google.android.exoplayer2.Format>): number;
					}
					export module MediaCodecAudioRenderer {
						export class AudioSinkListener extends com.google.android.exoplayer2.audio.AudioSink.Listener {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.MediaCodecAudioRenderer.AudioSinkListener>;
							public onPositionAdvancing(param0: number): void;
							public onOffloadBufferEmptying(): void;
							public onSkipSilenceEnabledChanged(param0: boolean): void;
							public onOffloadBufferFull(param0: number): void;
							public onAudioSinkError(param0: java.lang.Exception): void;
							public onPositionDiscontinuity(): void;
							public onUnderrun(param0: number, param1: number, param2: number): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module audio {
					export class ResamplingAudioProcessor extends com.google.android.exoplayer2.audio.BaseAudioProcessor {
						public static class: java.lang.Class<com.google.android.exoplayer2.audio.ResamplingAudioProcessor>;
						public queueEndOfStream(): void;
						public getOutput(): java.nio.ByteBuffer;
						public onConfigure(param0: com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat): com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat;
						public flush(): void;
						public queueInput(param0: java.nio.ByteBuffer): void;
						public isEnded(): boolean;
						public reset(): void;
						public configure(param0: com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat): com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat;
						public isActive(): boolean;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module audio {
					export class SilenceSkippingAudioProcessor extends com.google.android.exoplayer2.audio.BaseAudioProcessor {
						public static class: java.lang.Class<com.google.android.exoplayer2.audio.SilenceSkippingAudioProcessor>;
						public static DEFAULT_MINIMUM_SILENCE_DURATION_US: number;
						public static DEFAULT_PADDING_SILENCE_US: number;
						public static DEFAULT_SILENCE_THRESHOLD_LEVEL: number;
						public constructor();
						public queueEndOfStream(): void;
						public onConfigure(param0: com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat): com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat;
						public flush(): void;
						public queueInput(param0: java.nio.ByteBuffer): void;
						public onQueueEndOfStream(): void;
						public setEnabled(param0: boolean): void;
						public configure(param0: com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat): com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat;
						public onFlush(): void;
						public getOutput(): java.nio.ByteBuffer;
						public getSkippedFrames(): number;
						public onReset(): void;
						public isEnded(): boolean;
						public constructor(param0: number, param1: number, param2: number);
						public reset(): void;
						public isActive(): boolean;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module audio {
					export class Sonic {
						public static class: java.lang.Class<com.google.android.exoplayer2.audio.Sonic>;
						public queueEndOfStream(): void;
						public getPendingInputBytes(): number;
						public queueInput(param0: java.nio.ShortBuffer): void;
						public flush(): void;
						public getOutputSize(): number;
						public constructor(param0: number, param1: number, param2: number, param3: number, param4: number);
						public getOutput(param0: java.nio.ShortBuffer): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module audio {
					export class SonicAudioProcessor extends com.google.android.exoplayer2.audio.AudioProcessor {
						public static class: java.lang.Class<com.google.android.exoplayer2.audio.SonicAudioProcessor>;
						public static SAMPLE_RATE_NO_CHANGE: number;
						public constructor();
						public queueEndOfStream(): void;
						public setPitch(param0: number): void;
						public flush(): void;
						public queueInput(param0: java.nio.ByteBuffer): void;
						public configure(param0: com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat): com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat;
						public setSpeed(param0: number): void;
						public getOutput(): java.nio.ByteBuffer;
						public getMediaDuration(param0: number): number;
						public setOutputSampleRateHz(param0: number): void;
						public isEnded(): boolean;
						public reset(): void;
						public isActive(): boolean;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module audio {
					export class TeeAudioProcessor extends com.google.android.exoplayer2.audio.BaseAudioProcessor {
						public static class: java.lang.Class<com.google.android.exoplayer2.audio.TeeAudioProcessor>;
						public constructor();
						public queueEndOfStream(): void;
						public onConfigure(param0: com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat): com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat;
						public flush(): void;
						public queueInput(param0: java.nio.ByteBuffer): void;
						public onQueueEndOfStream(): void;
						public configure(param0: com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat): com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat;
						public onFlush(): void;
						public getOutput(): java.nio.ByteBuffer;
						public onReset(): void;
						public isEnded(): boolean;
						public constructor(param0: com.google.android.exoplayer2.audio.TeeAudioProcessor.AudioBufferSink);
						public reset(): void;
						public isActive(): boolean;
					}
					export module TeeAudioProcessor {
						export class AudioBufferSink {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.TeeAudioProcessor.AudioBufferSink>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.audio.TeeAudioProcessor$AudioBufferSink interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								flush(param0: number, param1: number, param2: number): void;
								handleBuffer(param0: java.nio.ByteBuffer): void;
							});
							public constructor();
							public flush(param0: number, param1: number, param2: number): void;
							public handleBuffer(param0: java.nio.ByteBuffer): void;
						}
						export class WavFileAudioBufferSink extends com.google.android.exoplayer2.audio.TeeAudioProcessor.AudioBufferSink {
							public static class: java.lang.Class<com.google.android.exoplayer2.audio.TeeAudioProcessor.WavFileAudioBufferSink>;
							public flush(param0: number, param1: number, param2: number): void;
							public handleBuffer(param0: java.nio.ByteBuffer): void;
							public constructor(param0: string);
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module audio {
					export class TrimmingAudioProcessor extends com.google.android.exoplayer2.audio.BaseAudioProcessor {
						public static class: java.lang.Class<com.google.android.exoplayer2.audio.TrimmingAudioProcessor>;
						public constructor();
						public queueEndOfStream(): void;
						public onConfigure(param0: com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat): com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat;
						public flush(): void;
						public queueInput(param0: java.nio.ByteBuffer): void;
						public onQueueEndOfStream(): void;
						public configure(param0: com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat): com.google.android.exoplayer2.audio.AudioProcessor.AudioFormat;
						public setTrimFrameCount(param0: number, param1: number): void;
						public onFlush(): void;
						public getOutput(): java.nio.ByteBuffer;
						public resetTrimmedFrameCount(): void;
						public getTrimmedFrameCount(): number;
						public onReset(): void;
						public isEnded(): boolean;
						public reset(): void;
						public isActive(): boolean;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module core {
					export class BuildConfig {
						public static class: java.lang.Class<com.google.android.exoplayer2.core.BuildConfig>;
						public static DEBUG: boolean;
						public static LIBRARY_PACKAGE_NAME: string;
						public static BUILD_TYPE: string;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module decoder {
					export class DecoderCounters {
						public static class: java.lang.Class<com.google.android.exoplayer2.decoder.DecoderCounters>;
						public decoderInitCount: number;
						public decoderReleaseCount: number;
						public queuedInputBufferCount: number;
						public skippedInputBufferCount: number;
						public renderedOutputBufferCount: number;
						public skippedOutputBufferCount: number;
						public droppedBufferCount: number;
						public droppedInputBufferCount: number;
						public maxConsecutiveDroppedBufferCount: number;
						public droppedToKeyframeCount: number;
						public totalVideoFrameProcessingOffsetUs: number;
						public videoFrameProcessingOffsetCount: number;
						public constructor();
						public merge(param0: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public addVideoFrameProcessingOffset(param0: number): void;
						public ensureUpdated(): void;
						public toString(): string;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module decoder {
					export class DecoderReuseEvaluation {
						public static class: java.lang.Class<com.google.android.exoplayer2.decoder.DecoderReuseEvaluation>;
						public static REUSE_RESULT_NO: number;
						public static REUSE_RESULT_YES_WITH_FLUSH: number;
						public static REUSE_RESULT_YES_WITH_RECONFIGURATION: number;
						public static REUSE_RESULT_YES_WITHOUT_RECONFIGURATION: number;
						public static DISCARD_REASON_REUSE_NOT_IMPLEMENTED: number;
						public static DISCARD_REASON_WORKAROUND: number;
						public static DISCARD_REASON_APP_OVERRIDE: number;
						public static DISCARD_REASON_MIME_TYPE_CHANGED: number;
						public static DISCARD_REASON_OPERATING_RATE_CHANGED: number;
						public static DISCARD_REASON_INITIALIZATION_DATA_CHANGED: number;
						public static DISCARD_REASON_MAX_INPUT_SIZE_EXCEEDED: number;
						public static DISCARD_REASON_DRM_SESSION_CHANGED: number;
						public static DISCARD_REASON_VIDEO_MAX_RESOLUTION_EXCEEDED: number;
						public static DISCARD_REASON_VIDEO_RESOLUTION_CHANGED: number;
						public static DISCARD_REASON_VIDEO_ROTATION_CHANGED: number;
						public static DISCARD_REASON_VIDEO_COLOR_INFO_CHANGED: number;
						public static DISCARD_REASON_AUDIO_CHANNEL_COUNT_CHANGED: number;
						public static DISCARD_REASON_AUDIO_SAMPLE_RATE_CHANGED: number;
						public static DISCARD_REASON_AUDIO_ENCODING_CHANGED: number;
						public decoderName: string;
						public oldFormat: com.google.android.exoplayer2.Format;
						public newFormat: com.google.android.exoplayer2.Format;
						public result: number;
						public discardReasons: number;
						public constructor(param0: string, param1: com.google.android.exoplayer2.Format, param2: com.google.android.exoplayer2.Format, param3: number, param4: number);
						public equals(param0: any): boolean;
						public hashCode(): number;
					}
					export module DecoderReuseEvaluation {
						export class DecoderDiscardReasons {
							public static class: java.lang.Class<com.google.android.exoplayer2.decoder.DecoderReuseEvaluation.DecoderDiscardReasons>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.decoder.DecoderReuseEvaluation$DecoderDiscardReasons interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
							});
							public constructor();
						}
						export class DecoderReuseResult {
							public static class: java.lang.Class<com.google.android.exoplayer2.decoder.DecoderReuseEvaluation.DecoderReuseResult>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.decoder.DecoderReuseEvaluation$DecoderReuseResult interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
							});
							public constructor();
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module drm {
					export class ClearKeyUtil {
						public static class: java.lang.Class<com.google.android.exoplayer2.drm.ClearKeyUtil>;
						public static adjustRequestData(param0: androidNative.Array<number>): androidNative.Array<number>;
						public static adjustResponseData(param0: androidNative.Array<number>): androidNative.Array<number>;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module drm {
					export class DefaultDrmSession extends com.google.android.exoplayer2.drm.DrmSession {
						public static class: java.lang.Class<com.google.android.exoplayer2.drm.DefaultDrmSession>;
						public schemeDatas: java.util.List<com.google.android.exoplayer2.drm.DrmInitData.SchemeData>;
						public provision(): void;
						public onProvisionCompleted(): void;
						public hasSessionId(param0: androidNative.Array<number>): boolean;
						public onProvisionError(param0: java.lang.Exception, param1: boolean): void;
						public constructor(param0: java.util.UUID, param1: com.google.android.exoplayer2.drm.ExoMediaDrm, param2: com.google.android.exoplayer2.drm.DefaultDrmSession.ProvisioningManager, param3: com.google.android.exoplayer2.drm.DefaultDrmSession.ReferenceCountListener, param4: java.util.List<com.google.android.exoplayer2.drm.DrmInitData.SchemeData>, param5: number, param6: boolean, param7: boolean, param8: androidNative.Array<number>, param9: java.util.HashMap<string,string>, param10: com.google.android.exoplayer2.drm.MediaDrmCallback, param11: globalAndroid.os.Looper, param12: com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy, param13: com.google.android.exoplayer2.analytics.PlayerId);
						public requiresSecureDecoder(param0: string): boolean;
						public static replaceSession(param0: com.google.android.exoplayer2.drm.DrmSession, param1: com.google.android.exoplayer2.drm.DrmSession): void;
						public getSchemeUuid(): java.util.UUID;
						public getCryptoConfig(): com.google.android.exoplayer2.decoder.CryptoConfig;
						public release(param0: com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher): void;
						public getError(): com.google.android.exoplayer2.drm.DrmSession.DrmSessionException;
						public onMediaDrmEvent(param0: number): void;
						public getState(): number;
						public acquire(param0: com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher): void;
						public queryKeyStatus(): java.util.Map<string,string>;
						public getOfflineLicenseKeySetId(): androidNative.Array<number>;
						public playClearSamplesWithoutKeys(): boolean;
					}
					export module DefaultDrmSession {
						export class ProvisioningManager {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.DefaultDrmSession.ProvisioningManager>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.drm.DefaultDrmSession$ProvisioningManager interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								provisionRequired(param0: com.google.android.exoplayer2.drm.DefaultDrmSession): void;
								onProvisionError(param0: java.lang.Exception, param1: boolean): void;
								onProvisionCompleted(): void;
							});
							public constructor();
							public provisionRequired(param0: com.google.android.exoplayer2.drm.DefaultDrmSession): void;
							public onProvisionError(param0: java.lang.Exception, param1: boolean): void;
							public onProvisionCompleted(): void;
						}
						export class ReferenceCountListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.DefaultDrmSession.ReferenceCountListener>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.drm.DefaultDrmSession$ReferenceCountListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onReferenceCountIncremented(param0: com.google.android.exoplayer2.drm.DefaultDrmSession, param1: number): void;
								onReferenceCountDecremented(param0: com.google.android.exoplayer2.drm.DefaultDrmSession, param1: number): void;
							});
							public constructor();
							public onReferenceCountDecremented(param0: com.google.android.exoplayer2.drm.DefaultDrmSession, param1: number): void;
							public onReferenceCountIncremented(param0: com.google.android.exoplayer2.drm.DefaultDrmSession, param1: number): void;
						}
						export class RequestHandler {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.DefaultDrmSession.RequestHandler>;
							public constructor(param0: com.google.android.exoplayer2.drm.DefaultDrmSession, param1: globalAndroid.os.Looper);
							public release(): void;
							public handleMessage(param0: globalAndroid.os.Message): void;
						}
						export class RequestTask {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.DefaultDrmSession.RequestTask>;
							public taskId: number;
							public allowRetry: boolean;
							public startTimeMs: number;
							public request: any;
							public errorCount: number;
							public constructor(param0: number, param1: boolean, param2: number, param3: any);
						}
						export class ResponseHandler {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.DefaultDrmSession.ResponseHandler>;
							public constructor(param0: com.google.android.exoplayer2.drm.DefaultDrmSession, param1: globalAndroid.os.Looper);
							public handleMessage(param0: globalAndroid.os.Message): void;
						}
						export class UnexpectedDrmSessionException {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.DefaultDrmSession.UnexpectedDrmSessionException>;
							public constructor(param0: java.lang.Throwable);
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module drm {
					export class DefaultDrmSessionManager extends com.google.android.exoplayer2.drm.DrmSessionManager {
						public static class: java.lang.Class<com.google.android.exoplayer2.drm.DefaultDrmSessionManager>;
						public static PLAYREADY_CUSTOM_DATA_KEY: string;
						public static MODE_PLAYBACK: number;
						public static MODE_QUERY: number;
						public static MODE_DOWNLOAD: number;
						public static MODE_RELEASE: number;
						public static INITIAL_DRM_REQUEST_RETRY_COUNT: number;
						public static DEFAULT_SESSION_KEEPALIVE_MS: number;
						/** @deprecated */
						public static getDummyDrmSessionManager(): com.google.android.exoplayer2.drm.DrmSessionManager;
						public preacquireSession(param0: com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher, param1: com.google.android.exoplayer2.Format): com.google.android.exoplayer2.drm.DrmSessionManager.DrmSessionReference;
						public prepare(): void;
						public setPlayer(param0: globalAndroid.os.Looper, param1: com.google.android.exoplayer2.analytics.PlayerId): void;
						public acquireSession(param0: com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher, param1: com.google.android.exoplayer2.Format): com.google.android.exoplayer2.drm.DrmSession;
						/** @deprecated */
						public constructor(param0: java.util.UUID, param1: com.google.android.exoplayer2.drm.ExoMediaDrm, param2: com.google.android.exoplayer2.drm.MediaDrmCallback, param3: java.util.HashMap<string,string>, param4: boolean);
						public setMode(param0: number, param1: androidNative.Array<number>): void;
						/** @deprecated */
						public constructor(param0: java.util.UUID, param1: com.google.android.exoplayer2.drm.ExoMediaDrm, param2: com.google.android.exoplayer2.drm.MediaDrmCallback, param3: java.util.HashMap<string,string>);
						/** @deprecated */
						public constructor(param0: java.util.UUID, param1: com.google.android.exoplayer2.drm.ExoMediaDrm, param2: com.google.android.exoplayer2.drm.MediaDrmCallback, param3: java.util.HashMap<string,string>, param4: boolean, param5: number);
						public release(): void;
						public getCryptoType(param0: com.google.android.exoplayer2.Format): number;
					}
					export module DefaultDrmSessionManager {
						export class Builder {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.DefaultDrmSessionManager.Builder>;
							public constructor();
							public setKeyRequestParameters(param0: java.util.Map<string,string>): com.google.android.exoplayer2.drm.DefaultDrmSessionManager.Builder;
							public build(param0: com.google.android.exoplayer2.drm.MediaDrmCallback): com.google.android.exoplayer2.drm.DefaultDrmSessionManager;
							public setPlayClearSamplesWithoutKeys(param0: boolean): com.google.android.exoplayer2.drm.DefaultDrmSessionManager.Builder;
							public setLoadErrorHandlingPolicy(param0: com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy): com.google.android.exoplayer2.drm.DefaultDrmSessionManager.Builder;
							public setUuidAndExoMediaDrmProvider(param0: java.util.UUID, param1: com.google.android.exoplayer2.drm.ExoMediaDrm.Provider): com.google.android.exoplayer2.drm.DefaultDrmSessionManager.Builder;
							public setSessionKeepaliveMs(param0: number): com.google.android.exoplayer2.drm.DefaultDrmSessionManager.Builder;
							public setMultiSession(param0: boolean): com.google.android.exoplayer2.drm.DefaultDrmSessionManager.Builder;
							public setUseDrmSessionsForClearContent(param0: androidNative.Array<number>): com.google.android.exoplayer2.drm.DefaultDrmSessionManager.Builder;
						}
						export class MediaDrmEventListener extends com.google.android.exoplayer2.drm.ExoMediaDrm.OnEventListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.DefaultDrmSessionManager.MediaDrmEventListener>;
							public onEvent(param0: com.google.android.exoplayer2.drm.ExoMediaDrm, param1: androidNative.Array<number>, param2: number, param3: number, param4: androidNative.Array<number>): void;
						}
						export class MediaDrmHandler {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.DefaultDrmSessionManager.MediaDrmHandler>;
							public handleMessage(param0: globalAndroid.os.Message): void;
							public constructor(param0: com.google.android.exoplayer2.drm.DefaultDrmSessionManager, param1: globalAndroid.os.Looper);
						}
						export class MissingSchemeDataException {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.DefaultDrmSessionManager.MissingSchemeDataException>;
						}
						export class Mode {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.DefaultDrmSessionManager.Mode>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.drm.DefaultDrmSessionManager$Mode interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
							});
							public constructor();
						}
						export class PreacquiredSessionReference extends com.google.android.exoplayer2.drm.DrmSessionManager.DrmSessionReference {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.DefaultDrmSessionManager.PreacquiredSessionReference>;
							public constructor(param0: com.google.android.exoplayer2.drm.DefaultDrmSessionManager, param1: com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher);
							public release(): void;
							public acquire(param0: com.google.android.exoplayer2.Format): void;
						}
						export class ProvisioningManagerImpl extends com.google.android.exoplayer2.drm.DefaultDrmSession.ProvisioningManager {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.DefaultDrmSessionManager.ProvisioningManagerImpl>;
							public onSessionFullyReleased(param0: com.google.android.exoplayer2.drm.DefaultDrmSession): void;
							public provisionRequired(param0: com.google.android.exoplayer2.drm.DefaultDrmSession): void;
							public constructor(param0: com.google.android.exoplayer2.drm.DefaultDrmSessionManager);
							public onProvisionError(param0: java.lang.Exception, param1: boolean): void;
							public onProvisionCompleted(): void;
						}
						export class ReferenceCountListenerImpl extends com.google.android.exoplayer2.drm.DefaultDrmSession.ReferenceCountListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.DefaultDrmSessionManager.ReferenceCountListenerImpl>;
							public onReferenceCountDecremented(param0: com.google.android.exoplayer2.drm.DefaultDrmSession, param1: number): void;
							public onReferenceCountIncremented(param0: com.google.android.exoplayer2.drm.DefaultDrmSession, param1: number): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module drm {
					export class DefaultDrmSessionManagerProvider extends com.google.android.exoplayer2.drm.DrmSessionManagerProvider {
						public static class: java.lang.Class<com.google.android.exoplayer2.drm.DefaultDrmSessionManagerProvider>;
						public constructor();
						public get(param0: com.google.android.exoplayer2.MediaItem): com.google.android.exoplayer2.drm.DrmSessionManager;
						public setDrmUserAgent(param0: string): void;
						public setDrmHttpDataSourceFactory(param0: com.google.android.exoplayer2.upstream.HttpDataSource.Factory): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module drm {
					export class DrmSession {
						public static class: java.lang.Class<com.google.android.exoplayer2.drm.DrmSession>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.drm.DrmSession interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							replaceSession(param0: com.google.android.exoplayer2.drm.DrmSession, param1: com.google.android.exoplayer2.drm.DrmSession): void;
							getState(): number;
							playClearSamplesWithoutKeys(): boolean;
							getError(): com.google.android.exoplayer2.drm.DrmSession.DrmSessionException;
							getSchemeUuid(): java.util.UUID;
							getCryptoConfig(): com.google.android.exoplayer2.decoder.CryptoConfig;
							queryKeyStatus(): java.util.Map<string,string>;
							getOfflineLicenseKeySetId(): androidNative.Array<number>;
							requiresSecureDecoder(param0: string): boolean;
							acquire(param0: com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher): void;
							release(param0: com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher): void;
						});
						public constructor();
						public static STATE_OPENING: number;
						public static STATE_OPENED_WITH_KEYS: number;
						public static STATE_RELEASED: number;
						public static STATE_OPENED: number;
						public static STATE_ERROR: number;
						public getSchemeUuid(): java.util.UUID;
						public getCryptoConfig(): com.google.android.exoplayer2.decoder.CryptoConfig;
						public release(param0: com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher): void;
						public getError(): com.google.android.exoplayer2.drm.DrmSession.DrmSessionException;
						public getState(): number;
						public acquire(param0: com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher): void;
						public queryKeyStatus(): java.util.Map<string,string>;
						public getOfflineLicenseKeySetId(): androidNative.Array<number>;
						public requiresSecureDecoder(param0: string): boolean;
						public static replaceSession(param0: com.google.android.exoplayer2.drm.DrmSession, param1: com.google.android.exoplayer2.drm.DrmSession): void;
						public playClearSamplesWithoutKeys(): boolean;
					}
					export module DrmSession {
						export class DrmSessionException {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.DrmSession.DrmSessionException>;
							public errorCode: number;
							public constructor(param0: java.lang.Throwable, param1: number);
						}
						export class State {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.DrmSession.State>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.drm.DrmSession$State interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
							});
							public constructor();
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module drm {
					export class DrmSessionEventListener {
						public static class: java.lang.Class<com.google.android.exoplayer2.drm.DrmSessionEventListener>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.drm.DrmSessionEventListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							onDrmSessionAcquired(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
							onDrmSessionAcquired(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: number): void;
							onDrmKeysLoaded(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
							onDrmSessionManagerError(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: java.lang.Exception): void;
							onDrmKeysRestored(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
							onDrmKeysRemoved(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
							onDrmSessionReleased(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
						});
						public constructor();
						public onDrmKeysRestored(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
						public onDrmSessionReleased(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
						/** @deprecated */
						public onDrmSessionAcquired(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
						public onDrmKeysLoaded(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
						public onDrmSessionAcquired(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: number): void;
						public onDrmKeysRemoved(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
						public onDrmSessionManagerError(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: java.lang.Exception): void;
					}
					export module DrmSessionEventListener {
						export class EventDispatcher {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher>;
							public windowIndex: number;
							public mediaPeriodId: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId;
							public constructor();
							public drmKeysLoaded(): void;
							public drmSessionManagerError(param0: java.lang.Exception): void;
							public drmSessionReleased(): void;
							public drmSessionAcquired(param0: number): void;
							public drmKeysRemoved(): void;
							public addEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
							public removeEventListener(param0: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
							public drmKeysRestored(): void;
							public withParameters(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher;
						}
						export module EventDispatcher {
							export class ListenerAndHandler {
								public static class: java.lang.Class<com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher.ListenerAndHandler>;
								public handler: globalAndroid.os.Handler;
								public listener: com.google.android.exoplayer2.drm.DrmSessionEventListener;
								public constructor(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.drm.DrmSessionEventListener);
							}
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module drm {
					export class DrmSessionManager {
						public static class: java.lang.Class<com.google.android.exoplayer2.drm.DrmSessionManager>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.drm.DrmSessionManager interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							getDummyDrmSessionManager(): com.google.android.exoplayer2.drm.DrmSessionManager;
							prepare(): void;
							release(): void;
							setPlayer(param0: globalAndroid.os.Looper, param1: com.google.android.exoplayer2.analytics.PlayerId): void;
							preacquireSession(param0: com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher, param1: com.google.android.exoplayer2.Format): com.google.android.exoplayer2.drm.DrmSessionManager.DrmSessionReference;
							acquireSession(param0: com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher, param1: com.google.android.exoplayer2.Format): com.google.android.exoplayer2.drm.DrmSession;
							getCryptoType(param0: com.google.android.exoplayer2.Format): number;
							<clinit>(): void;
						});
						public constructor();
						public static DRM_UNSUPPORTED: com.google.android.exoplayer2.drm.DrmSessionManager;
						public static DUMMY: com.google.android.exoplayer2.drm.DrmSessionManager;
						/** @deprecated */
						public static getDummyDrmSessionManager(): com.google.android.exoplayer2.drm.DrmSessionManager;
						public preacquireSession(param0: com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher, param1: com.google.android.exoplayer2.Format): com.google.android.exoplayer2.drm.DrmSessionManager.DrmSessionReference;
						public prepare(): void;
						public setPlayer(param0: globalAndroid.os.Looper, param1: com.google.android.exoplayer2.analytics.PlayerId): void;
						public acquireSession(param0: com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher, param1: com.google.android.exoplayer2.Format): com.google.android.exoplayer2.drm.DrmSession;
						public release(): void;
						public getCryptoType(param0: com.google.android.exoplayer2.Format): number;
					}
					export module DrmSessionManager {
						export class DrmSessionReference {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.DrmSessionManager.DrmSessionReference>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.drm.DrmSessionManager$DrmSessionReference interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								release(): void;
								lambda$static$0(): void;
								<clinit>(): void;
							});
							public constructor();
							public static EMPTY: com.google.android.exoplayer2.drm.DrmSessionManager.DrmSessionReference;
							public release(): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module drm {
					export class DrmSessionManagerProvider {
						public static class: java.lang.Class<com.google.android.exoplayer2.drm.DrmSessionManagerProvider>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.drm.DrmSessionManagerProvider interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							get(param0: com.google.android.exoplayer2.MediaItem): com.google.android.exoplayer2.drm.DrmSessionManager;
						});
						public constructor();
						public get(param0: com.google.android.exoplayer2.MediaItem): com.google.android.exoplayer2.drm.DrmSessionManager;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module drm {
					export class DrmUtil {
						public static class: java.lang.Class<com.google.android.exoplayer2.drm.DrmUtil>;
						public static ERROR_SOURCE_EXO_MEDIA_DRM: number;
						public static ERROR_SOURCE_LICENSE_ACQUISITION: number;
						public static ERROR_SOURCE_PROVISIONING: number;
						public static getErrorCodeForMediaDrmException(param0: java.lang.Exception, param1: number): number;
					}
					export module DrmUtil {
						export class Api18 {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.DrmUtil.Api18>;
							public static isDeniedByServerException(param0: java.lang.Throwable): boolean;
							public static isNotProvisionedException(param0: java.lang.Throwable): boolean;
						}
						export class Api21 {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.DrmUtil.Api21>;
							public static mediaDrmStateExceptionToErrorCode(param0: java.lang.Throwable): number;
							public static isMediaDrmStateException(param0: java.lang.Throwable): boolean;
						}
						export class Api23 {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.DrmUtil.Api23>;
							public static isMediaDrmResetException(param0: java.lang.Throwable): boolean;
						}
						export class ErrorSource {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.DrmUtil.ErrorSource>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.drm.DrmUtil$ErrorSource interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
							});
							public constructor();
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module drm {
					export class DummyExoMediaDrm extends com.google.android.exoplayer2.drm.ExoMediaDrm {
						public static class: java.lang.Class<com.google.android.exoplayer2.drm.DummyExoMediaDrm>;
						public constructor();
						public getMetrics(): any;
						public setPlayerIdForSession(param0: androidNative.Array<number>, param1: com.google.android.exoplayer2.analytics.PlayerId): void;
						public acquire(): void;
						public getProvisionRequest(): com.google.android.exoplayer2.drm.ExoMediaDrm.ProvisionRequest;
						public restoreKeys(param0: androidNative.Array<number>, param1: androidNative.Array<number>): void;
						public getPropertyString(param0: string): string;
						public provideKeyResponse(param0: androidNative.Array<number>, param1: androidNative.Array<number>): androidNative.Array<number>;
						public getPropertyByteArray(param0: string): androidNative.Array<number>;
						public release(): void;
						public getCryptoType(): number;
						public getKeyRequest(param0: androidNative.Array<number>, param1: java.util.List<com.google.android.exoplayer2.drm.DrmInitData.SchemeData>, param2: number, param3: java.util.HashMap<string,string>): com.google.android.exoplayer2.drm.ExoMediaDrm.KeyRequest;
						public setPropertyString(param0: string, param1: string): void;
						public setOnKeyStatusChangeListener(param0: com.google.android.exoplayer2.drm.ExoMediaDrm.OnKeyStatusChangeListener): void;
						public setOnEventListener(param0: com.google.android.exoplayer2.drm.ExoMediaDrm.OnEventListener): void;
						public openSession(): androidNative.Array<number>;
						public setPropertyByteArray(param0: string, param1: androidNative.Array<number>): void;
						public setOnExpirationUpdateListener(param0: com.google.android.exoplayer2.drm.ExoMediaDrm.OnExpirationUpdateListener): void;
						public requiresSecureDecoder(param0: androidNative.Array<number>, param1: string): boolean;
						public provideProvisionResponse(param0: androidNative.Array<number>): void;
						public closeSession(param0: androidNative.Array<number>): void;
						public static getInstance(): com.google.android.exoplayer2.drm.DummyExoMediaDrm;
						public queryKeyStatus(param0: androidNative.Array<number>): java.util.Map<string,string>;
						public createCryptoConfig(param0: androidNative.Array<number>): com.google.android.exoplayer2.decoder.CryptoConfig;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module drm {
					export class ErrorStateDrmSession extends com.google.android.exoplayer2.drm.DrmSession {
						public static class: java.lang.Class<com.google.android.exoplayer2.drm.ErrorStateDrmSession>;
						public getSchemeUuid(): java.util.UUID;
						public getCryptoConfig(): com.google.android.exoplayer2.decoder.CryptoConfig;
						public release(param0: com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher): void;
						public getError(): com.google.android.exoplayer2.drm.DrmSession.DrmSessionException;
						public constructor(param0: com.google.android.exoplayer2.drm.DrmSession.DrmSessionException);
						public getState(): number;
						public acquire(param0: com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher): void;
						public queryKeyStatus(): java.util.Map<string,string>;
						public getOfflineLicenseKeySetId(): androidNative.Array<number>;
						public requiresSecureDecoder(param0: string): boolean;
						public playClearSamplesWithoutKeys(): boolean;
						public static replaceSession(param0: com.google.android.exoplayer2.drm.DrmSession, param1: com.google.android.exoplayer2.drm.DrmSession): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module drm {
					export class ExoMediaDrm {
						public static class: java.lang.Class<com.google.android.exoplayer2.drm.ExoMediaDrm>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.drm.ExoMediaDrm interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							setOnEventListener(param0: com.google.android.exoplayer2.drm.ExoMediaDrm.OnEventListener): void;
							setOnKeyStatusChangeListener(param0: com.google.android.exoplayer2.drm.ExoMediaDrm.OnKeyStatusChangeListener): void;
							setOnExpirationUpdateListener(param0: com.google.android.exoplayer2.drm.ExoMediaDrm.OnExpirationUpdateListener): void;
							openSession(): androidNative.Array<number>;
							closeSession(param0: androidNative.Array<number>): void;
							setPlayerIdForSession(param0: androidNative.Array<number>, param1: com.google.android.exoplayer2.analytics.PlayerId): void;
							getKeyRequest(param0: androidNative.Array<number>, param1: java.util.List<com.google.android.exoplayer2.drm.DrmInitData.SchemeData>, param2: number, param3: java.util.HashMap<string,string>): com.google.android.exoplayer2.drm.ExoMediaDrm.KeyRequest;
							provideKeyResponse(param0: androidNative.Array<number>, param1: androidNative.Array<number>): androidNative.Array<number>;
							getProvisionRequest(): com.google.android.exoplayer2.drm.ExoMediaDrm.ProvisionRequest;
							provideProvisionResponse(param0: androidNative.Array<number>): void;
							queryKeyStatus(param0: androidNative.Array<number>): java.util.Map<string,string>;
							requiresSecureDecoder(param0: androidNative.Array<number>, param1: string): boolean;
							acquire(): void;
							release(): void;
							restoreKeys(param0: androidNative.Array<number>, param1: androidNative.Array<number>): void;
							getMetrics(): any;
							getPropertyString(param0: string): string;
							getPropertyByteArray(param0: string): androidNative.Array<number>;
							setPropertyString(param0: string, param1: string): void;
							setPropertyByteArray(param0: string, param1: androidNative.Array<number>): void;
							createCryptoConfig(param0: androidNative.Array<number>): com.google.android.exoplayer2.decoder.CryptoConfig;
							getCryptoType(): number;
						});
						public constructor();
						public static KEY_TYPE_RELEASE: number;
						public static KEY_TYPE_STREAMING: number;
						public static EVENT_KEY_REQUIRED: number;
						public static EVENT_KEY_EXPIRED: number;
						public static EVENT_PROVISION_REQUIRED: number;
						public static KEY_TYPE_OFFLINE: number;
						public setPlayerIdForSession(param0: androidNative.Array<number>, param1: com.google.android.exoplayer2.analytics.PlayerId): void;
						public getMetrics(): any;
						public acquire(): void;
						public getProvisionRequest(): com.google.android.exoplayer2.drm.ExoMediaDrm.ProvisionRequest;
						public restoreKeys(param0: androidNative.Array<number>, param1: androidNative.Array<number>): void;
						public getPropertyString(param0: string): string;
						public provideKeyResponse(param0: androidNative.Array<number>, param1: androidNative.Array<number>): androidNative.Array<number>;
						public getPropertyByteArray(param0: string): androidNative.Array<number>;
						public release(): void;
						public getCryptoType(): number;
						public getKeyRequest(param0: androidNative.Array<number>, param1: java.util.List<com.google.android.exoplayer2.drm.DrmInitData.SchemeData>, param2: number, param3: java.util.HashMap<string,string>): com.google.android.exoplayer2.drm.ExoMediaDrm.KeyRequest;
						public setPropertyString(param0: string, param1: string): void;
						public setOnKeyStatusChangeListener(param0: com.google.android.exoplayer2.drm.ExoMediaDrm.OnKeyStatusChangeListener): void;
						public setOnEventListener(param0: com.google.android.exoplayer2.drm.ExoMediaDrm.OnEventListener): void;
						public openSession(): androidNative.Array<number>;
						public setPropertyByteArray(param0: string, param1: androidNative.Array<number>): void;
						public setOnExpirationUpdateListener(param0: com.google.android.exoplayer2.drm.ExoMediaDrm.OnExpirationUpdateListener): void;
						public requiresSecureDecoder(param0: androidNative.Array<number>, param1: string): boolean;
						public provideProvisionResponse(param0: androidNative.Array<number>): void;
						public closeSession(param0: androidNative.Array<number>): void;
						public queryKeyStatus(param0: androidNative.Array<number>): java.util.Map<string,string>;
						public createCryptoConfig(param0: androidNative.Array<number>): com.google.android.exoplayer2.decoder.CryptoConfig;
					}
					export module ExoMediaDrm {
						export class AppManagedProvider extends com.google.android.exoplayer2.drm.ExoMediaDrm.Provider {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.ExoMediaDrm.AppManagedProvider>;
							public constructor(param0: com.google.android.exoplayer2.drm.ExoMediaDrm);
							public acquireExoMediaDrm(param0: java.util.UUID): com.google.android.exoplayer2.drm.ExoMediaDrm;
						}
						export class KeyRequest {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.ExoMediaDrm.KeyRequest>;
							public static REQUEST_TYPE_UNKNOWN: number;
							public static REQUEST_TYPE_INITIAL: number;
							public static REQUEST_TYPE_RENEWAL: number;
							public static REQUEST_TYPE_RELEASE: number;
							public static REQUEST_TYPE_NONE: number;
							public static REQUEST_TYPE_UPDATE: number;
							public getData(): androidNative.Array<number>;
							public getRequestType(): number;
							public constructor(param0: androidNative.Array<number>, param1: string);
							public getLicenseServerUrl(): string;
							public constructor(param0: androidNative.Array<number>, param1: string, param2: number);
						}
						export module KeyRequest {
							export class RequestType {
								public static class: java.lang.Class<com.google.android.exoplayer2.drm.ExoMediaDrm.KeyRequest.RequestType>;
								/**
								 * Constructs a new instance of the com.google.android.exoplayer2.drm.ExoMediaDrm$KeyRequest$RequestType interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
								 */
								public constructor(implementation: {
								});
								public constructor();
							}
						}
						export class KeyStatus {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.ExoMediaDrm.KeyStatus>;
							public getStatusCode(): number;
							public getKeyId(): androidNative.Array<number>;
							public constructor(param0: number, param1: androidNative.Array<number>);
						}
						export class OnEventListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.ExoMediaDrm.OnEventListener>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.drm.ExoMediaDrm$OnEventListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onEvent(param0: com.google.android.exoplayer2.drm.ExoMediaDrm, param1: androidNative.Array<number>, param2: number, param3: number, param4: androidNative.Array<number>): void;
							});
							public constructor();
							public onEvent(param0: com.google.android.exoplayer2.drm.ExoMediaDrm, param1: androidNative.Array<number>, param2: number, param3: number, param4: androidNative.Array<number>): void;
						}
						export class OnExpirationUpdateListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.ExoMediaDrm.OnExpirationUpdateListener>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.drm.ExoMediaDrm$OnExpirationUpdateListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onExpirationUpdate(param0: com.google.android.exoplayer2.drm.ExoMediaDrm, param1: androidNative.Array<number>, param2: number): void;
							});
							public constructor();
							public onExpirationUpdate(param0: com.google.android.exoplayer2.drm.ExoMediaDrm, param1: androidNative.Array<number>, param2: number): void;
						}
						export class OnKeyStatusChangeListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.ExoMediaDrm.OnKeyStatusChangeListener>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.drm.ExoMediaDrm$OnKeyStatusChangeListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onKeyStatusChange(param0: com.google.android.exoplayer2.drm.ExoMediaDrm, param1: androidNative.Array<number>, param2: java.util.List<com.google.android.exoplayer2.drm.ExoMediaDrm.KeyStatus>, param3: boolean): void;
							});
							public constructor();
							public onKeyStatusChange(param0: com.google.android.exoplayer2.drm.ExoMediaDrm, param1: androidNative.Array<number>, param2: java.util.List<com.google.android.exoplayer2.drm.ExoMediaDrm.KeyStatus>, param3: boolean): void;
						}
						export class Provider {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.ExoMediaDrm.Provider>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.drm.ExoMediaDrm$Provider interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								acquireExoMediaDrm(param0: java.util.UUID): com.google.android.exoplayer2.drm.ExoMediaDrm;
							});
							public constructor();
							public acquireExoMediaDrm(param0: java.util.UUID): com.google.android.exoplayer2.drm.ExoMediaDrm;
						}
						export class ProvisionRequest {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.ExoMediaDrm.ProvisionRequest>;
							public getData(): androidNative.Array<number>;
							public getDefaultUrl(): string;
							public constructor(param0: androidNative.Array<number>, param1: string);
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module drm {
					export class FrameworkCryptoConfig {
						public static class: java.lang.Class<com.google.android.exoplayer2.drm.FrameworkCryptoConfig>;
						public static WORKAROUND_DEVICE_NEEDS_KEYS_TO_CONFIGURE_CODEC: boolean;
						public uuid: java.util.UUID;
						public sessionId: androidNative.Array<number>;
						public forceAllowInsecureDecoderComponents: boolean;
						public constructor(param0: java.util.UUID, param1: androidNative.Array<number>, param2: boolean);
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module drm {
					export class FrameworkMediaDrm extends com.google.android.exoplayer2.drm.ExoMediaDrm {
						public static class: java.lang.Class<com.google.android.exoplayer2.drm.FrameworkMediaDrm>;
						public static DEFAULT_PROVIDER: com.google.android.exoplayer2.drm.ExoMediaDrm.Provider;
						public static newInstance(param0: java.util.UUID): com.google.android.exoplayer2.drm.FrameworkMediaDrm;
						public setPlayerIdForSession(param0: androidNative.Array<number>, param1: com.google.android.exoplayer2.analytics.PlayerId): void;
						public getMetrics(): any;
						public getProvisionRequest(): com.google.android.exoplayer2.drm.ExoMediaDrm.ProvisionRequest;
						public getPropertyString(param0: string): string;
						public getPropertyByteArray(param0: string): androidNative.Array<number>;
						public getCryptoType(): number;
						public getKeyRequest(param0: androidNative.Array<number>, param1: java.util.List<com.google.android.exoplayer2.drm.DrmInitData.SchemeData>, param2: number, param3: java.util.HashMap<string,string>): com.google.android.exoplayer2.drm.ExoMediaDrm.KeyRequest;
						public setPropertyString(param0: string, param1: string): void;
						public setOnEventListener(param0: com.google.android.exoplayer2.drm.ExoMediaDrm.OnEventListener): void;
						public openSession(): androidNative.Array<number>;
						public setPropertyByteArray(param0: string, param1: androidNative.Array<number>): void;
						public requiresSecureDecoder(param0: androidNative.Array<number>, param1: string): boolean;
						public provideProvisionResponse(param0: androidNative.Array<number>): void;
						public static isCryptoSchemeSupported(param0: java.util.UUID): boolean;
						public createCryptoConfig(param0: androidNative.Array<number>): com.google.android.exoplayer2.decoder.CryptoConfig;
						public acquire(): void;
						public restoreKeys(param0: androidNative.Array<number>, param1: androidNative.Array<number>): void;
						public provideKeyResponse(param0: androidNative.Array<number>, param1: androidNative.Array<number>): androidNative.Array<number>;
						public release(): void;
						public setOnKeyStatusChangeListener(param0: com.google.android.exoplayer2.drm.ExoMediaDrm.OnKeyStatusChangeListener): void;
						public setOnExpirationUpdateListener(param0: com.google.android.exoplayer2.drm.ExoMediaDrm.OnExpirationUpdateListener): void;
						public closeSession(param0: androidNative.Array<number>): void;
						public createCryptoConfig(param0: androidNative.Array<number>): com.google.android.exoplayer2.drm.FrameworkCryptoConfig;
						public queryKeyStatus(param0: androidNative.Array<number>): java.util.Map<string,string>;
					}
					export module FrameworkMediaDrm {
						export class Api31 {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.FrameworkMediaDrm.Api31>;
							public static requiresSecureDecoder(param0: globalAndroid.media.MediaDrm, param1: string): boolean;
							public static setLogSessionIdOnMediaDrmSession(param0: globalAndroid.media.MediaDrm, param1: androidNative.Array<number>, param2: com.google.android.exoplayer2.analytics.PlayerId): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module drm {
					export class HttpMediaDrmCallback extends com.google.android.exoplayer2.drm.MediaDrmCallback {
						public static class: java.lang.Class<com.google.android.exoplayer2.drm.HttpMediaDrmCallback>;
						public executeKeyRequest(param0: java.util.UUID, param1: com.google.android.exoplayer2.drm.ExoMediaDrm.KeyRequest): androidNative.Array<number>;
						public clearAllKeyRequestProperties(): void;
						public constructor(param0: string, param1: boolean, param2: com.google.android.exoplayer2.upstream.HttpDataSource.Factory);
						public clearKeyRequestProperty(param0: string): void;
						public setKeyRequestProperty(param0: string, param1: string): void;
						public constructor(param0: string, param1: com.google.android.exoplayer2.upstream.HttpDataSource.Factory);
						public executeProvisionRequest(param0: java.util.UUID, param1: com.google.android.exoplayer2.drm.ExoMediaDrm.ProvisionRequest): androidNative.Array<number>;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module drm {
					export class KeysExpiredException {
						public static class: java.lang.Class<com.google.android.exoplayer2.drm.KeysExpiredException>;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module drm {
					export class LocalMediaDrmCallback extends com.google.android.exoplayer2.drm.MediaDrmCallback {
						public static class: java.lang.Class<com.google.android.exoplayer2.drm.LocalMediaDrmCallback>;
						public constructor(param0: androidNative.Array<number>);
						public executeKeyRequest(param0: java.util.UUID, param1: com.google.android.exoplayer2.drm.ExoMediaDrm.KeyRequest): androidNative.Array<number>;
						public executeProvisionRequest(param0: java.util.UUID, param1: com.google.android.exoplayer2.drm.ExoMediaDrm.ProvisionRequest): androidNative.Array<number>;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module drm {
					export class MediaDrmCallback {
						public static class: java.lang.Class<com.google.android.exoplayer2.drm.MediaDrmCallback>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.drm.MediaDrmCallback interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							executeProvisionRequest(param0: java.util.UUID, param1: com.google.android.exoplayer2.drm.ExoMediaDrm.ProvisionRequest): androidNative.Array<number>;
							executeKeyRequest(param0: java.util.UUID, param1: com.google.android.exoplayer2.drm.ExoMediaDrm.KeyRequest): androidNative.Array<number>;
						});
						public constructor();
						public executeKeyRequest(param0: java.util.UUID, param1: com.google.android.exoplayer2.drm.ExoMediaDrm.KeyRequest): androidNative.Array<number>;
						public executeProvisionRequest(param0: java.util.UUID, param1: com.google.android.exoplayer2.drm.ExoMediaDrm.ProvisionRequest): androidNative.Array<number>;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module drm {
					export class MediaDrmCallbackException {
						public static class: java.lang.Class<com.google.android.exoplayer2.drm.MediaDrmCallbackException>;
						public dataSpec: com.google.android.exoplayer2.upstream.DataSpec;
						public uriAfterRedirects: globalAndroid.net.Uri;
						public responseHeaders: java.util.Map<string,java.util.List<string>>;
						public bytesLoaded: number;
						public constructor(param0: com.google.android.exoplayer2.upstream.DataSpec, param1: globalAndroid.net.Uri, param2: java.util.Map<string,java.util.List<string>>, param3: number, param4: java.lang.Throwable);
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module drm {
					export class OfflineLicenseHelper {
						public static class: java.lang.Class<com.google.android.exoplayer2.drm.OfflineLicenseHelper>;
						public static newWidevineInstance(param0: string, param1: com.google.android.exoplayer2.upstream.HttpDataSource.Factory, param2: com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher): com.google.android.exoplayer2.drm.OfflineLicenseHelper;
						public releaseLicense(param0: androidNative.Array<number>): void;
						public constructor(param0: com.google.android.exoplayer2.drm.DefaultDrmSessionManager, param1: com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher);
						/** @deprecated */
						public constructor(param0: java.util.UUID, param1: com.google.android.exoplayer2.drm.ExoMediaDrm.Provider, param2: com.google.android.exoplayer2.drm.MediaDrmCallback, param3: java.util.Map<string,string>, param4: com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher);
						public static newWidevineInstance(param0: string, param1: boolean, param2: com.google.android.exoplayer2.upstream.HttpDataSource.Factory, param3: java.util.Map<string,string>, param4: com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher): com.google.android.exoplayer2.drm.OfflineLicenseHelper;
						public downloadLicense(param0: com.google.android.exoplayer2.Format): androidNative.Array<number>;
						public getLicenseDurationRemainingSec(param0: androidNative.Array<number>): globalAndroid.util.Pair<java.lang.Long,java.lang.Long>;
						public static newWidevineInstance(param0: string, param1: boolean, param2: com.google.android.exoplayer2.upstream.HttpDataSource.Factory, param3: com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher): com.google.android.exoplayer2.drm.OfflineLicenseHelper;
						public release(): void;
						public renewLicense(param0: androidNative.Array<number>): androidNative.Array<number>;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module drm {
					export class UnsupportedDrmException {
						public static class: java.lang.Class<com.google.android.exoplayer2.drm.UnsupportedDrmException>;
						public static REASON_UNSUPPORTED_SCHEME: number;
						public static REASON_INSTANTIATION_ERROR: number;
						public reason: number;
						public constructor(param0: number);
						public constructor(param0: number, param1: java.lang.Exception);
					}
					export module UnsupportedDrmException {
						export class Reason {
							public static class: java.lang.Class<com.google.android.exoplayer2.drm.UnsupportedDrmException.Reason>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.drm.UnsupportedDrmException$Reason interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
							});
							public constructor();
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module drm {
					export class WidevineUtil {
						public static class: java.lang.Class<com.google.android.exoplayer2.drm.WidevineUtil>;
						public static PROPERTY_LICENSE_DURATION_REMAINING: string;
						public static PROPERTY_PLAYBACK_DURATION_REMAINING: string;
						public static getLicenseDurationRemainingSec(param0: com.google.android.exoplayer2.drm.DrmSession): globalAndroid.util.Pair<java.lang.Long,java.lang.Long>;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module mediacodec {
					export class AsynchronousMediaCodecAdapter extends com.google.android.exoplayer2.mediacodec.MediaCodecAdapter {
						public static class: java.lang.Class<com.google.android.exoplayer2.mediacodec.AsynchronousMediaCodecAdapter>;
						public needsReconfiguration(): boolean;
						public getOutputBuffer(param0: number): java.nio.ByteBuffer;
						public dequeueOutputBufferIndex(param0: globalAndroid.media.MediaCodec.BufferInfo): number;
						public getMetrics(): any;
						public releaseOutputBuffer(param0: number, param1: number): void;
						public queueSecureInputBuffer(param0: number, param1: number, param2: com.google.android.exoplayer2.decoder.CryptoInfo, param3: number, param4: number): void;
						public setVideoScalingMode(param0: number): void;
						public getInputBuffer(param0: number): java.nio.ByteBuffer;
						public flush(): void;
						public release(): void;
						public dequeueInputBufferIndex(): number;
						public setOnFrameRenderedListener(param0: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.OnFrameRenderedListener, param1: globalAndroid.os.Handler): void;
						public setParameters(param0: globalAndroid.os.Bundle): void;
						public queueInputBuffer(param0: number, param1: number, param2: number, param3: number, param4: number): void;
						public getOutputFormat(): globalAndroid.media.MediaFormat;
						public releaseOutputBuffer(param0: number, param1: boolean): void;
						public setOutputSurface(param0: globalAndroid.view.Surface): void;
					}
					export module AsynchronousMediaCodecAdapter {
						export class Factory extends com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.Factory {
							public static class: java.lang.Class<com.google.android.exoplayer2.mediacodec.AsynchronousMediaCodecAdapter.Factory>;
							public createAdapter(param0: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.Configuration): com.google.android.exoplayer2.mediacodec.AsynchronousMediaCodecAdapter;
							public createAdapter(param0: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.Configuration): com.google.android.exoplayer2.mediacodec.MediaCodecAdapter;
							public constructor(param0: number, param1: boolean, param2: boolean);
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module mediacodec {
					export class AsynchronousMediaCodecBufferEnqueuer {
						public static class: java.lang.Class<com.google.android.exoplayer2.mediacodec.AsynchronousMediaCodecBufferEnqueuer>;
						public shutdown(): void;
						public start(): void;
						public queueSecureInputBuffer(param0: number, param1: number, param2: com.google.android.exoplayer2.decoder.CryptoInfo, param3: number, param4: number): void;
						public queueInputBuffer(param0: number, param1: number, param2: number, param3: number, param4: number): void;
						public constructor(param0: globalAndroid.media.MediaCodec, param1: globalAndroid.os.HandlerThread);
						public flush(): void;
						public waitUntilQueueingComplete(): void;
					}
					export module AsynchronousMediaCodecBufferEnqueuer {
						export class MessageParams {
							public static class: java.lang.Class<com.google.android.exoplayer2.mediacodec.AsynchronousMediaCodecBufferEnqueuer.MessageParams>;
							public index: number;
							public offset: number;
							public size: number;
							public cryptoInfo: globalAndroid.media.MediaCodec.CryptoInfo;
							public presentationTimeUs: number;
							public flags: number;
							public setQueueParams(param0: number, param1: number, param2: number, param3: number, param4: number): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module mediacodec {
					export class AsynchronousMediaCodecCallback {
						public static class: java.lang.Class<com.google.android.exoplayer2.mediacodec.AsynchronousMediaCodecCallback>;
						public onOutputBufferAvailable(param0: globalAndroid.media.MediaCodec, param1: number, param2: globalAndroid.media.MediaCodec.BufferInfo): void;
						public onError(param0: globalAndroid.media.MediaCodec, param1: globalAndroid.media.MediaCodec.CodecException): void;
						public shutdown(): void;
						public dequeueOutputBufferIndex(param0: globalAndroid.media.MediaCodec.BufferInfo): number;
						public flush(param0: globalAndroid.media.MediaCodec): void;
						public getOutputFormat(): globalAndroid.media.MediaFormat;
						public onOutputFormatChanged(param0: globalAndroid.media.MediaCodec, param1: globalAndroid.media.MediaFormat): void;
						public onInputBufferAvailable(param0: globalAndroid.media.MediaCodec, param1: number): void;
						public initialize(param0: globalAndroid.media.MediaCodec): void;
						public dequeueInputBufferIndex(): number;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module mediacodec {
					export class BatchBuffer {
						public static class: java.lang.Class<com.google.android.exoplayer2.mediacodec.BatchBuffer>;
						public static DEFAULT_MAX_SAMPLE_COUNT: number;
						public append(param0: com.google.android.exoplayer2.decoder.DecoderInputBuffer): boolean;
						public constructor();
						public getLastSampleTimeUs(): number;
						public setMaxSampleCount(param0: number): void;
						public getFirstSampleTimeUs(): number;
						public getSampleCount(): number;
						public clear(): void;
						public hasSamples(): boolean;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module mediacodec {
					export class C2Mp3TimestampTracker {
						public static class: java.lang.Class<com.google.android.exoplayer2.mediacodec.C2Mp3TimestampTracker>;
						public updateAndGetPresentationTimeUs(param0: com.google.android.exoplayer2.Format, param1: com.google.android.exoplayer2.decoder.DecoderInputBuffer): number;
						public reset(): void;
						public getLastOutputBufferPresentationTimeUs(param0: com.google.android.exoplayer2.Format): number;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module mediacodec {
					export class DefaultMediaCodecAdapterFactory extends com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.Factory {
						public static class: java.lang.Class<com.google.android.exoplayer2.mediacodec.DefaultMediaCodecAdapterFactory>;
						public constructor();
						public forceDisableAsynchronous(): com.google.android.exoplayer2.mediacodec.DefaultMediaCodecAdapterFactory;
						public experimentalSetSynchronizeCodecInteractionsWithQueueingEnabled(param0: boolean): void;
						public forceEnableAsynchronous(): com.google.android.exoplayer2.mediacodec.DefaultMediaCodecAdapterFactory;
						public experimentalSetImmediateCodecStartAfterFlushEnabled(param0: boolean): void;
						public createAdapter(param0: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.Configuration): com.google.android.exoplayer2.mediacodec.MediaCodecAdapter;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module mediacodec {
					export class IntArrayQueue {
						public static class: java.lang.Class<com.google.android.exoplayer2.mediacodec.IntArrayQueue>;
						public constructor();
						public capacity(): number;
						public add(param0: number): void;
						public isEmpty(): boolean;
						public remove(): number;
						public clear(): void;
						public size(): number;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module mediacodec {
					export class MediaCodecAdapter {
						public static class: java.lang.Class<com.google.android.exoplayer2.mediacodec.MediaCodecAdapter>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.mediacodec.MediaCodecAdapter interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							dequeueInputBufferIndex(): number;
							dequeueOutputBufferIndex(param0: globalAndroid.media.MediaCodec.BufferInfo): number;
							getOutputFormat(): globalAndroid.media.MediaFormat;
							getInputBuffer(param0: number): java.nio.ByteBuffer;
							getOutputBuffer(param0: number): java.nio.ByteBuffer;
							queueInputBuffer(param0: number, param1: number, param2: number, param3: number, param4: number): void;
							queueSecureInputBuffer(param0: number, param1: number, param2: com.google.android.exoplayer2.decoder.CryptoInfo, param3: number, param4: number): void;
							releaseOutputBuffer(param0: number, param1: boolean): void;
							releaseOutputBuffer(param0: number, param1: number): void;
							flush(): void;
							release(): void;
							setOnFrameRenderedListener(param0: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.OnFrameRenderedListener, param1: globalAndroid.os.Handler): void;
							setOutputSurface(param0: globalAndroid.view.Surface): void;
							setParameters(param0: globalAndroid.os.Bundle): void;
							setVideoScalingMode(param0: number): void;
							needsReconfiguration(): boolean;
							getMetrics(): any;
						});
						public constructor();
						public needsReconfiguration(): boolean;
						public getOutputBuffer(param0: number): java.nio.ByteBuffer;
						public dequeueOutputBufferIndex(param0: globalAndroid.media.MediaCodec.BufferInfo): number;
						public getMetrics(): any;
						public releaseOutputBuffer(param0: number, param1: number): void;
						public queueSecureInputBuffer(param0: number, param1: number, param2: com.google.android.exoplayer2.decoder.CryptoInfo, param3: number, param4: number): void;
						public setVideoScalingMode(param0: number): void;
						public getInputBuffer(param0: number): java.nio.ByteBuffer;
						public flush(): void;
						public release(): void;
						public dequeueInputBufferIndex(): number;
						public setOnFrameRenderedListener(param0: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.OnFrameRenderedListener, param1: globalAndroid.os.Handler): void;
						public setParameters(param0: globalAndroid.os.Bundle): void;
						public queueInputBuffer(param0: number, param1: number, param2: number, param3: number, param4: number): void;
						public getOutputFormat(): globalAndroid.media.MediaFormat;
						public releaseOutputBuffer(param0: number, param1: boolean): void;
						public setOutputSurface(param0: globalAndroid.view.Surface): void;
					}
					export module MediaCodecAdapter {
						export class Configuration {
							public static class: java.lang.Class<com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.Configuration>;
							public codecInfo: com.google.android.exoplayer2.mediacodec.MediaCodecInfo;
							public mediaFormat: globalAndroid.media.MediaFormat;
							public format: com.google.android.exoplayer2.Format;
							public surface: globalAndroid.view.Surface;
							public crypto: globalAndroid.media.MediaCrypto;
							public flags: number;
							public static createForAudioDecoding(param0: com.google.android.exoplayer2.mediacodec.MediaCodecInfo, param1: globalAndroid.media.MediaFormat, param2: com.google.android.exoplayer2.Format, param3: globalAndroid.media.MediaCrypto): com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.Configuration;
							public static createForVideoDecoding(param0: com.google.android.exoplayer2.mediacodec.MediaCodecInfo, param1: globalAndroid.media.MediaFormat, param2: com.google.android.exoplayer2.Format, param3: globalAndroid.view.Surface, param4: globalAndroid.media.MediaCrypto): com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.Configuration;
						}
						export class Factory {
							public static class: java.lang.Class<com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.Factory>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.mediacodec.MediaCodecAdapter$Factory interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								createAdapter(param0: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.Configuration): com.google.android.exoplayer2.mediacodec.MediaCodecAdapter;
								<clinit>(): void;
							});
							public constructor();
							public static DEFAULT: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.Factory;
							public createAdapter(param0: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.Configuration): com.google.android.exoplayer2.mediacodec.MediaCodecAdapter;
						}
						export class OnFrameRenderedListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.OnFrameRenderedListener>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.mediacodec.MediaCodecAdapter$OnFrameRenderedListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onFrameRendered(param0: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter, param1: number, param2: number): void;
							});
							public constructor();
							public onFrameRendered(param0: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter, param1: number, param2: number): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module mediacodec {
					export class MediaCodecDecoderException {
						public static class: java.lang.Class<com.google.android.exoplayer2.mediacodec.MediaCodecDecoderException>;
						public codecInfo: com.google.android.exoplayer2.mediacodec.MediaCodecInfo;
						public diagnosticInfo: string;
						public constructor(param0: java.lang.Throwable, param1: com.google.android.exoplayer2.mediacodec.MediaCodecInfo);
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module mediacodec {
					export class MediaCodecInfo {
						public static class: java.lang.Class<com.google.android.exoplayer2.mediacodec.MediaCodecInfo>;
						public static TAG: string;
						public static MAX_SUPPORTED_INSTANCES_UNKNOWN: number;
						public name: string;
						public mimeType: string;
						public codecMimeType: string;
						public capabilities: globalAndroid.media.MediaCodecInfo.CodecCapabilities;
						public adaptive: boolean;
						public tunneling: boolean;
						public secure: boolean;
						public hardwareAccelerated: boolean;
						public softwareOnly: boolean;
						public vendor: boolean;
						/** @deprecated */
						public isSeamlessAdaptationSupported(param0: com.google.android.exoplayer2.Format, param1: com.google.android.exoplayer2.Format, param2: boolean): boolean;
						public getProfileLevels(): androidNative.Array<globalAndroid.media.MediaCodecInfo.CodecProfileLevel>;
						public static newInstance(param0: string, param1: string, param2: string, param3: globalAndroid.media.MediaCodecInfo.CodecCapabilities, param4: boolean, param5: boolean, param6: boolean, param7: boolean, param8: boolean): com.google.android.exoplayer2.mediacodec.MediaCodecInfo;
						public isVideoSizeAndRateSupportedV21(param0: number, param1: number, param2: number): boolean;
						public isSeamlessAdaptationSupported(param0: com.google.android.exoplayer2.Format): boolean;
						public isHdr10PlusOutOfBandMetadataSupported(): boolean;
						public alignVideoSizeV21(param0: number, param1: number): globalAndroid.graphics.Point;
						public toString(): string;
						public isFormatSupported(param0: com.google.android.exoplayer2.Format): boolean;
						public isAudioChannelCountSupportedV21(param0: number): boolean;
						public canReuseCodec(param0: com.google.android.exoplayer2.Format, param1: com.google.android.exoplayer2.Format): com.google.android.exoplayer2.decoder.DecoderReuseEvaluation;
						public getMaxSupportedInstances(): number;
						public isAudioSampleRateSupportedV21(param0: number): boolean;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module mediacodec {
					export abstract class MediaCodecRenderer extends com.google.android.exoplayer2.BaseRenderer {
						public static class: java.lang.Class<com.google.android.exoplayer2.mediacodec.MediaCodecRenderer>;
						public static CODEC_OPERATING_RATE_UNSET: number;
						public decoderCounters: com.google.android.exoplayer2.decoder.DecoderCounters;
						public supportsFormat(param0: com.google.android.exoplayer2.mediacodec.MediaCodecSelector, param1: com.google.android.exoplayer2.Format): number;
						public shouldReinitCodec(): boolean;
						public resetCodecStateForRelease(): void;
						public getCodecInfo(): com.google.android.exoplayer2.mediacodec.MediaCodecInfo;
						public constructor(param0: number, param1: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.Factory, param2: com.google.android.exoplayer2.mediacodec.MediaCodecSelector, param3: boolean, param4: number);
						public onPositionReset(param0: number, param1: boolean): void;
						public replaceStream(param0: androidNative.Array<com.google.android.exoplayer2.Format>, param1: com.google.android.exoplayer2.source.SampleStream, param2: number, param3: number): void;
						public static getHardwareAccelerationSupport(param0: number): number;
						public static create(param0: number): number;
						public static getTunnelingSupport(param0: number): number;
						public getStream(): com.google.android.exoplayer2.source.SampleStream;
						public onDisabled(): void;
						public isEnded(): boolean;
						public getCodecOutputMediaFormat(): globalAndroid.media.MediaFormat;
						public releaseCodec(): void;
						public resetCodecStateForFlush(): void;
						public getDecoderInfos(param0: com.google.android.exoplayer2.mediacodec.MediaCodecSelector, param1: com.google.android.exoplayer2.Format, param2: boolean): java.util.List<com.google.android.exoplayer2.mediacodec.MediaCodecInfo>;
						public setPlaybackSpeed(param0: number, param1: number): void;
						public updateCodecOperatingRate(): boolean;
						public setCurrentStreamFinal(): void;
						public createDecoderException(param0: java.lang.Throwable, param1: com.google.android.exoplayer2.mediacodec.MediaCodecInfo): com.google.android.exoplayer2.mediacodec.MediaCodecDecoderException;
						public onStreamChanged(param0: androidNative.Array<com.google.android.exoplayer2.Format>, param1: number, param2: number): void;
						public onCodecInitialized(param0: string, param1: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.Configuration, param2: number, param3: number): void;
						public setPendingPlaybackException(param0: com.google.android.exoplayer2.ExoPlaybackException): void;
						public isReady(): boolean;
						public disable(): void;
						public processOutputBuffer(param0: number, param1: number, param2: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter, param3: java.nio.ByteBuffer, param4: number, param5: number, param6: number, param7: number, param8: boolean, param9: boolean, param10: com.google.android.exoplayer2.Format): boolean;
						public canReuseCodec(param0: com.google.android.exoplayer2.mediacodec.MediaCodecInfo, param1: com.google.android.exoplayer2.Format, param2: com.google.android.exoplayer2.Format): com.google.android.exoplayer2.decoder.DecoderReuseEvaluation;
						public supportsFormat(param0: com.google.android.exoplayer2.Format): number;
						public start(): void;
						public onCodecError(param0: java.lang.Exception): void;
						public getState(): number;
						public handleInputBufferSupplementalData(param0: com.google.android.exoplayer2.decoder.DecoderInputBuffer): void;
						public onReset(): void;
						public static create(param0: number, param1: number, param2: number, param3: number, param4: number): number;
						public onStopped(): void;
						public getReadingPositionUs(): number;
						public onProcessedOutputBuffer(param0: number): void;
						public reset(): void;
						public flushOrReleaseCodec(): boolean;
						public onQueueInputBuffer(param0: com.google.android.exoplayer2.decoder.DecoderInputBuffer): void;
						public getCodecOperatingRate(): number;
						public getCodecNeedsEosPropagation(): boolean;
						public maybeInitCodecOrBypass(): void;
						public onStarted(): void;
						public render(param0: number, param1: number): void;
						public getName(): string;
						public getMediaClock(): com.google.android.exoplayer2.util.MediaClock;
						public isCurrentStreamFinal(): boolean;
						public getMediaCodecConfiguration(param0: com.google.android.exoplayer2.mediacodec.MediaCodecInfo, param1: com.google.android.exoplayer2.Format, param2: globalAndroid.media.MediaCrypto, param3: number): com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.Configuration;
						public shouldUseBypass(param0: com.google.android.exoplayer2.Format): boolean;
						public setRenderTimeLimitMs(param0: number): void;
						public updateOutputFormatForTime(param0: number): void;
						public static getAdaptiveSupport(param0: number): number;
						public onCodecReleased(param0: string): void;
						public getOutputStreamOffsetUs(): number;
						public onInputFormatChanged(param0: com.google.android.exoplayer2.FormatHolder): com.google.android.exoplayer2.decoder.DecoderReuseEvaluation;
						public getCapabilities(): com.google.android.exoplayer2.RendererCapabilities;
						public enable(param0: com.google.android.exoplayer2.RendererConfiguration, param1: androidNative.Array<com.google.android.exoplayer2.Format>, param2: com.google.android.exoplayer2.source.SampleStream, param3: number, param4: boolean, param5: boolean, param6: number, param7: number): void;
						public resetPosition(param0: number): void;
						public getCodec(): com.google.android.exoplayer2.mediacodec.MediaCodecAdapter;
						public init(param0: number, param1: com.google.android.exoplayer2.analytics.PlayerId): void;
						public hasReadStreamToEnd(): boolean;
						public getPlaybackSpeed(): number;
						public supportsMixedMimeTypeAdaptation(): number;
						public onProcessedStreamChange(): void;
						public getTrackType(): number;
						public stop(): void;
						public flushOrReinitializeCodec(): boolean;
						public onOutputFormatChanged(param0: com.google.android.exoplayer2.Format, param1: globalAndroid.media.MediaFormat): void;
						public onEnabled(param0: boolean, param1: boolean): void;
						public static getFormatSupport(param0: number): number;
						public shouldInitCodec(param0: com.google.android.exoplayer2.mediacodec.MediaCodecInfo): boolean;
						public static create(param0: number, param1: number, param2: number): number;
						public constructor(param0: number);
						public setPendingOutputEndOfStream(): void;
						public maybeThrowStreamError(): void;
						public renderToEndOfStream(): void;
						public static getDecoderSupport(param0: number): number;
						public static supportsFormatDrm(param0: com.google.android.exoplayer2.Format): boolean;
						public getCodecOperatingRateV23(param0: number, param1: com.google.android.exoplayer2.Format, param2: androidNative.Array<com.google.android.exoplayer2.Format>): number;
					}
					export module MediaCodecRenderer {
						export class Api31 {
							public static class: java.lang.Class<com.google.android.exoplayer2.mediacodec.MediaCodecRenderer.Api31>;
							public static setLogSessionIdToMediaCodecFormat(param0: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.Configuration, param1: com.google.android.exoplayer2.analytics.PlayerId): void;
						}
						export class DecoderInitializationException {
							public static class: java.lang.Class<com.google.android.exoplayer2.mediacodec.MediaCodecRenderer.DecoderInitializationException>;
							public mimeType: string;
							public secureDecoderRequired: boolean;
							public codecInfo: com.google.android.exoplayer2.mediacodec.MediaCodecInfo;
							public diagnosticInfo: string;
							public fallbackDecoderInitializationException: com.google.android.exoplayer2.mediacodec.MediaCodecRenderer.DecoderInitializationException;
							public constructor(param0: com.google.android.exoplayer2.Format, param1: java.lang.Throwable, param2: boolean, param3: com.google.android.exoplayer2.mediacodec.MediaCodecInfo);
							public constructor(param0: com.google.android.exoplayer2.Format, param1: java.lang.Throwable, param2: boolean, param3: number);
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module mediacodec {
					export class MediaCodecSelector {
						public static class: java.lang.Class<com.google.android.exoplayer2.mediacodec.MediaCodecSelector>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.mediacodec.MediaCodecSelector interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							getDecoderInfos(param0: string, param1: boolean, param2: boolean): java.util.List<com.google.android.exoplayer2.mediacodec.MediaCodecInfo>;
							<clinit>(): void;
						});
						public constructor();
						public static DEFAULT: com.google.android.exoplayer2.mediacodec.MediaCodecSelector;
						public getDecoderInfos(param0: string, param1: boolean, param2: boolean): java.util.List<com.google.android.exoplayer2.mediacodec.MediaCodecInfo>;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module mediacodec {
					export class MediaCodecUtil {
						public static class: java.lang.Class<com.google.android.exoplayer2.mediacodec.MediaCodecUtil>;
						public static clearDecoderInfoCache(): void;
						public static getDecoderInfos(param0: string, param1: boolean, param2: boolean): java.util.List<com.google.android.exoplayer2.mediacodec.MediaCodecInfo>;
						public static getDecoderInfo(param0: string, param1: boolean, param2: boolean): com.google.android.exoplayer2.mediacodec.MediaCodecInfo;
						public static maxH264DecodableFrameSize(): number;
						public static getDecoderInfosSortedByFormatSupport(param0: java.util.List<com.google.android.exoplayer2.mediacodec.MediaCodecInfo>, param1: com.google.android.exoplayer2.Format): java.util.List<com.google.android.exoplayer2.mediacodec.MediaCodecInfo>;
						public static getCodecProfileAndLevel(param0: com.google.android.exoplayer2.Format): globalAndroid.util.Pair<java.lang.Integer,java.lang.Integer>;
						public static warmDecoderInfoCache(param0: string, param1: boolean, param2: boolean): void;
						public static getDecryptOnlyDecoderInfo(): com.google.android.exoplayer2.mediacodec.MediaCodecInfo;
						public static getAlternativeCodecMimeType(param0: com.google.android.exoplayer2.Format): string;
					}
					export module MediaCodecUtil {
						export class CodecKey {
							public static class: java.lang.Class<com.google.android.exoplayer2.mediacodec.MediaCodecUtil.CodecKey>;
							public mimeType: string;
							public secure: boolean;
							public tunneling: boolean;
							public constructor(param0: string, param1: boolean, param2: boolean);
							public hashCode(): number;
							public equals(param0: any): boolean;
						}
						export class DecoderQueryException {
							public static class: java.lang.Class<com.google.android.exoplayer2.mediacodec.MediaCodecUtil.DecoderQueryException>;
						}
						export class MediaCodecListCompat {
							public static class: java.lang.Class<com.google.android.exoplayer2.mediacodec.MediaCodecUtil.MediaCodecListCompat>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.mediacodec.MediaCodecUtil$MediaCodecListCompat interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								getCodecCount(): number;
								getCodecInfoAt(param0: number): globalAndroid.media.MediaCodecInfo;
								secureDecodersExplicit(): boolean;
								isFeatureSupported(param0: string, param1: string, param2: globalAndroid.media.MediaCodecInfo.CodecCapabilities): boolean;
								isFeatureRequired(param0: string, param1: string, param2: globalAndroid.media.MediaCodecInfo.CodecCapabilities): boolean;
							});
							public constructor();
							public secureDecodersExplicit(): boolean;
							public getCodecInfoAt(param0: number): globalAndroid.media.MediaCodecInfo;
							public isFeatureSupported(param0: string, param1: string, param2: globalAndroid.media.MediaCodecInfo.CodecCapabilities): boolean;
							public isFeatureRequired(param0: string, param1: string, param2: globalAndroid.media.MediaCodecInfo.CodecCapabilities): boolean;
							public getCodecCount(): number;
						}
						export class MediaCodecListCompatV16 extends com.google.android.exoplayer2.mediacodec.MediaCodecUtil.MediaCodecListCompat {
							public static class: java.lang.Class<com.google.android.exoplayer2.mediacodec.MediaCodecUtil.MediaCodecListCompatV16>;
							public secureDecodersExplicit(): boolean;
							public getCodecInfoAt(param0: number): globalAndroid.media.MediaCodecInfo;
							public isFeatureSupported(param0: string, param1: string, param2: globalAndroid.media.MediaCodecInfo.CodecCapabilities): boolean;
							public isFeatureRequired(param0: string, param1: string, param2: globalAndroid.media.MediaCodecInfo.CodecCapabilities): boolean;
							public getCodecCount(): number;
						}
						export class MediaCodecListCompatV21 extends com.google.android.exoplayer2.mediacodec.MediaCodecUtil.MediaCodecListCompat {
							public static class: java.lang.Class<com.google.android.exoplayer2.mediacodec.MediaCodecUtil.MediaCodecListCompatV21>;
							public secureDecodersExplicit(): boolean;
							public constructor(param0: boolean, param1: boolean);
							public getCodecInfoAt(param0: number): globalAndroid.media.MediaCodecInfo;
							public isFeatureSupported(param0: string, param1: string, param2: globalAndroid.media.MediaCodecInfo.CodecCapabilities): boolean;
							public isFeatureRequired(param0: string, param1: string, param2: globalAndroid.media.MediaCodecInfo.CodecCapabilities): boolean;
							public getCodecCount(): number;
						}
						export class ScoreProvider<T>  extends java.lang.Object {
							public static class: java.lang.Class<com.google.android.exoplayer2.mediacodec.MediaCodecUtil.ScoreProvider<any>>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.mediacodec.MediaCodecUtil$ScoreProvider interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								getScore(param0: T): number;
							});
							public constructor();
							public getScore(param0: T): number;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module mediacodec {
					export class SynchronousMediaCodecAdapter extends com.google.android.exoplayer2.mediacodec.MediaCodecAdapter {
						public static class: java.lang.Class<com.google.android.exoplayer2.mediacodec.SynchronousMediaCodecAdapter>;
						public needsReconfiguration(): boolean;
						public getOutputBuffer(param0: number): java.nio.ByteBuffer;
						public dequeueOutputBufferIndex(param0: globalAndroid.media.MediaCodec.BufferInfo): number;
						public getMetrics(): any;
						public releaseOutputBuffer(param0: number, param1: number): void;
						public queueSecureInputBuffer(param0: number, param1: number, param2: com.google.android.exoplayer2.decoder.CryptoInfo, param3: number, param4: number): void;
						public setVideoScalingMode(param0: number): void;
						public getInputBuffer(param0: number): java.nio.ByteBuffer;
						public flush(): void;
						public release(): void;
						public dequeueInputBufferIndex(): number;
						public setOnFrameRenderedListener(param0: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.OnFrameRenderedListener, param1: globalAndroid.os.Handler): void;
						public setParameters(param0: globalAndroid.os.Bundle): void;
						public queueInputBuffer(param0: number, param1: number, param2: number, param3: number, param4: number): void;
						public getOutputFormat(): globalAndroid.media.MediaFormat;
						public releaseOutputBuffer(param0: number, param1: boolean): void;
						public setOutputSurface(param0: globalAndroid.view.Surface): void;
					}
					export module SynchronousMediaCodecAdapter {
						export class Factory extends com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.Factory {
							public static class: java.lang.Class<com.google.android.exoplayer2.mediacodec.SynchronousMediaCodecAdapter.Factory>;
							public constructor();
							public createCodec(param0: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.Configuration): globalAndroid.media.MediaCodec;
							public createAdapter(param0: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.Configuration): com.google.android.exoplayer2.mediacodec.MediaCodecAdapter;
						}
					}
				}
			}
		}
	}
}


declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module metadata {
					export class MetadataDecoderFactory {
						public static class: java.lang.Class<com.google.android.exoplayer2.metadata.MetadataDecoderFactory>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.metadata.MetadataDecoderFactory interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							supportsFormat(param0: com.google.android.exoplayer2.Format): boolean;
							createDecoder(param0: com.google.android.exoplayer2.Format): com.google.android.exoplayer2.metadata.MetadataDecoder;
							<clinit>(): void;
						});
						public constructor();
						public static DEFAULT: com.google.android.exoplayer2.metadata.MetadataDecoderFactory;
						public createDecoder(param0: com.google.android.exoplayer2.Format): com.google.android.exoplayer2.metadata.MetadataDecoder;
						public supportsFormat(param0: com.google.android.exoplayer2.Format): boolean;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module metadata {
					export class MetadataOutput {
						public static class: java.lang.Class<com.google.android.exoplayer2.metadata.MetadataOutput>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.metadata.MetadataOutput interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							onMetadata(param0: com.google.android.exoplayer2.metadata.Metadata): void;
						});
						public constructor();
						public onMetadata(param0: com.google.android.exoplayer2.metadata.Metadata): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module metadata {
					export class MetadataRenderer extends com.google.android.exoplayer2.BaseRenderer {
						public static class: java.lang.Class<com.google.android.exoplayer2.metadata.MetadataRenderer>;
						public handleMessage(param0: globalAndroid.os.Message): boolean;
						public render(param0: number, param1: number): void;
						public getName(): string;
						public onPositionReset(param0: number, param1: boolean): void;
						public replaceStream(param0: androidNative.Array<com.google.android.exoplayer2.Format>, param1: com.google.android.exoplayer2.source.SampleStream, param2: number, param3: number): void;
						public getMediaClock(): com.google.android.exoplayer2.util.MediaClock;
						public isCurrentStreamFinal(): boolean;
						public static getHardwareAccelerationSupport(param0: number): number;
						public static create(param0: number): number;
						public static getTunnelingSupport(param0: number): number;
						public getStream(): com.google.android.exoplayer2.source.SampleStream;
						public constructor(param0: com.google.android.exoplayer2.metadata.MetadataOutput, param1: globalAndroid.os.Looper);
						public constructor(param0: com.google.android.exoplayer2.metadata.MetadataOutput, param1: globalAndroid.os.Looper, param2: com.google.android.exoplayer2.metadata.MetadataDecoderFactory);
						public static getAdaptiveSupport(param0: number): number;
						public onDisabled(): void;
						public isEnded(): boolean;
						public getCapabilities(): com.google.android.exoplayer2.RendererCapabilities;
						public enable(param0: com.google.android.exoplayer2.RendererConfiguration, param1: androidNative.Array<com.google.android.exoplayer2.Format>, param2: com.google.android.exoplayer2.source.SampleStream, param3: number, param4: boolean, param5: boolean, param6: number, param7: number): void;
						public setPlaybackSpeed(param0: number, param1: number): void;
						public setCurrentStreamFinal(): void;
						public onStreamChanged(param0: androidNative.Array<com.google.android.exoplayer2.Format>, param1: number, param2: number): void;
						public resetPosition(param0: number): void;
						public init(param0: number, param1: com.google.android.exoplayer2.analytics.PlayerId): void;
						public hasReadStreamToEnd(): boolean;
						public isReady(): boolean;
						public disable(): void;
						public getTrackType(): number;
						public supportsMixedMimeTypeAdaptation(): number;
						public stop(): void;
						public handleMessage(param0: number, param1: any): void;
						public static getFormatSupport(param0: number): number;
						public supportsFormat(param0: com.google.android.exoplayer2.Format): number;
						public start(): void;
						public static create(param0: number, param1: number, param2: number): number;
						public constructor(param0: number);
						public getState(): number;
						public maybeThrowStreamError(): void;
						public static create(param0: number, param1: number, param2: number, param3: number, param4: number): number;
						public static getDecoderSupport(param0: number): number;
						public getReadingPositionUs(): number;
						public reset(): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module offline {
					export class DefaultDownloadIndex extends com.google.android.exoplayer2.offline.WritableDownloadIndex {
						public static class: java.lang.Class<com.google.android.exoplayer2.offline.DefaultDownloadIndex>;
						public setStopReason(param0: string, param1: number): void;
						public setDownloadingStatesToQueued(): void;
						public getDownloads(param0: androidNative.Array<number>): com.google.android.exoplayer2.offline.DownloadCursor;
						public putDownload(param0: com.google.android.exoplayer2.offline.Download): void;
						public getDownload(param0: string): com.google.android.exoplayer2.offline.Download;
						public setStatesToRemoving(): void;
						public constructor(param0: com.google.android.exoplayer2.database.DatabaseProvider, param1: string);
						public constructor(param0: com.google.android.exoplayer2.database.DatabaseProvider);
						public removeDownload(param0: string): void;
						public setStopReason(param0: number): void;
					}
					export module DefaultDownloadIndex {
						export class DownloadCursorImpl extends com.google.android.exoplayer2.offline.DownloadCursor {
							public static class: java.lang.Class<com.google.android.exoplayer2.offline.DefaultDownloadIndex.DownloadCursorImpl>;
							public moveToNext(): boolean;
							public close(): void;
							public isAfterLast(): boolean;
							public getPosition(): number;
							public moveToPosition(param0: number): boolean;
							public isClosed(): boolean;
							public moveToLast(): boolean;
							public getCount(): number;
							public moveToFirst(): boolean;
							public isFirst(): boolean;
							public getDownload(): com.google.android.exoplayer2.offline.Download;
							public isLast(): boolean;
							public moveToPrevious(): boolean;
							public isBeforeFirst(): boolean;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module offline {
					export class DefaultDownloaderFactory extends com.google.android.exoplayer2.offline.DownloaderFactory {
						public static class: java.lang.Class<com.google.android.exoplayer2.offline.DefaultDownloaderFactory>;
						public createDownloader(param0: com.google.android.exoplayer2.offline.DownloadRequest): com.google.android.exoplayer2.offline.Downloader;
						/** @deprecated */
						public constructor(param0: com.google.android.exoplayer2.upstream.cache.CacheDataSource.Factory);
						public constructor(param0: com.google.android.exoplayer2.upstream.cache.CacheDataSource.Factory, param1: java.util.concurrent.Executor);
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module offline {
					export class Download {
						public static class: java.lang.Class<com.google.android.exoplayer2.offline.Download>;
						public static STATE_QUEUED: number;
						public static STATE_STOPPED: number;
						public static STATE_DOWNLOADING: number;
						public static STATE_COMPLETED: number;
						public static STATE_FAILED: number;
						public static STATE_REMOVING: number;
						public static STATE_RESTARTING: number;
						public static FAILURE_REASON_NONE: number;
						public static FAILURE_REASON_UNKNOWN: number;
						public static STOP_REASON_NONE: number;
						public request: com.google.android.exoplayer2.offline.DownloadRequest;
						public state: number;
						public startTimeMs: number;
						public updateTimeMs: number;
						public contentLength: number;
						public stopReason: number;
						public failureReason: number;
						public getBytesDownloaded(): number;
						public getPercentDownloaded(): number;
						public constructor(param0: com.google.android.exoplayer2.offline.DownloadRequest, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: com.google.android.exoplayer2.offline.DownloadProgress);
						public isTerminalState(): boolean;
						public constructor(param0: com.google.android.exoplayer2.offline.DownloadRequest, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number);
					}
					export module Download {
						export class FailureReason {
							public static class: java.lang.Class<com.google.android.exoplayer2.offline.Download.FailureReason>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.offline.Download$FailureReason interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
							});
							public constructor();
						}
						export class State {
							public static class: java.lang.Class<com.google.android.exoplayer2.offline.Download.State>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.offline.Download$State interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
							});
							public constructor();
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module offline {
					export class DownloadCursor {
						public static class: java.lang.Class<com.google.android.exoplayer2.offline.DownloadCursor>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.offline.DownloadCursor interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							getDownload(): com.google.android.exoplayer2.offline.Download;
							getCount(): number;
							getPosition(): number;
							moveToPosition(param0: number): boolean;
							moveToFirst(): boolean;
							moveToLast(): boolean;
							moveToNext(): boolean;
							moveToPrevious(): boolean;
							isFirst(): boolean;
							isLast(): boolean;
							isBeforeFirst(): boolean;
							isAfterLast(): boolean;
							isClosed(): boolean;
							close(): void;
						});
						public constructor();
						public isFirst(): boolean;
						public close(): void;
						public getPosition(): number;
						public getCount(): number;
						public isBeforeFirst(): boolean;
						public isLast(): boolean;
						public moveToFirst(): boolean;
						public isClosed(): boolean;
						public getDownload(): com.google.android.exoplayer2.offline.Download;
						public moveToPrevious(): boolean;
						public moveToLast(): boolean;
						public moveToPosition(param0: number): boolean;
						public isAfterLast(): boolean;
						public moveToNext(): boolean;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module offline {
					export class DownloadException {
						public static class: java.lang.Class<com.google.android.exoplayer2.offline.DownloadException>;
						public constructor(param0: java.lang.Throwable);
						public constructor(param0: string);
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module offline {
					export class DownloadHelper {
						public static class: java.lang.Class<com.google.android.exoplayer2.offline.DownloadHelper>;
						public static DEFAULT_TRACK_SELECTOR_PARAMETERS_WITHOUT_CONTEXT: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters;
						public static DEFAULT_TRACK_SELECTOR_PARAMETERS_WITHOUT_VIEWPORT: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters;
						public static DEFAULT_TRACK_SELECTOR_PARAMETERS: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters;
						/** @deprecated */
						public static forHls(param0: globalAndroid.net.Uri, param1: com.google.android.exoplayer2.upstream.DataSource.Factory, param2: com.google.android.exoplayer2.RenderersFactory, param3: com.google.android.exoplayer2.drm.DrmSessionManager, param4: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters): com.google.android.exoplayer2.offline.DownloadHelper;
						public getTrackGroups(param0: number): com.google.android.exoplayer2.source.TrackGroupArray;
						public static createMediaSource(param0: com.google.android.exoplayer2.offline.DownloadRequest, param1: com.google.android.exoplayer2.upstream.DataSource.Factory): com.google.android.exoplayer2.source.MediaSource;
						public addTrackSelection(param0: number, param1: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters): void;
						public addTrackSelectionForSingleRenderer(param0: number, param1: number, param2: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters, param3: java.util.List<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.SelectionOverride>): void;
						public static forMediaItem(param0: com.google.android.exoplayer2.MediaItem, param1: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters, param2: com.google.android.exoplayer2.RenderersFactory, param3: com.google.android.exoplayer2.upstream.DataSource.Factory, param4: com.google.android.exoplayer2.drm.DrmSessionManager): com.google.android.exoplayer2.offline.DownloadHelper;
						public prepare(param0: com.google.android.exoplayer2.offline.DownloadHelper.Callback): void;
						/** @deprecated */
						public static forSmoothStreaming(param0: globalAndroid.net.Uri, param1: com.google.android.exoplayer2.upstream.DataSource.Factory, param2: com.google.android.exoplayer2.RenderersFactory, param3: com.google.android.exoplayer2.drm.DrmSessionManager, param4: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters): com.google.android.exoplayer2.offline.DownloadHelper;
						public addTextLanguagesToSelection(param0: boolean, param1: androidNative.Array<string>): void;
						public addAudioLanguagesToSelection(param0: androidNative.Array<string>): void;
						public static getRendererCapabilities(param0: com.google.android.exoplayer2.RenderersFactory): androidNative.Array<com.google.android.exoplayer2.RendererCapabilities>;
						public getDownloadRequest(param0: androidNative.Array<number>): com.google.android.exoplayer2.offline.DownloadRequest;
						/** @deprecated */
						public static forProgressive(param0: globalAndroid.content.Context, param1: globalAndroid.net.Uri, param2: string): com.google.android.exoplayer2.offline.DownloadHelper;
						public constructor(param0: com.google.android.exoplayer2.MediaItem, param1: com.google.android.exoplayer2.source.MediaSource, param2: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters, param3: androidNative.Array<com.google.android.exoplayer2.RendererCapabilities>);
						/** @deprecated */
						public static forDash(param0: globalAndroid.content.Context, param1: globalAndroid.net.Uri, param2: com.google.android.exoplayer2.upstream.DataSource.Factory, param3: com.google.android.exoplayer2.RenderersFactory): com.google.android.exoplayer2.offline.DownloadHelper;
						/** @deprecated */
						public static forHls(param0: globalAndroid.content.Context, param1: globalAndroid.net.Uri, param2: com.google.android.exoplayer2.upstream.DataSource.Factory, param3: com.google.android.exoplayer2.RenderersFactory): com.google.android.exoplayer2.offline.DownloadHelper;
						public static getDefaultTrackSelectorParameters(param0: globalAndroid.content.Context): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters;
						public getDownloadRequest(param0: string, param1: androidNative.Array<number>): com.google.android.exoplayer2.offline.DownloadRequest;
						public static createMediaSource(param0: com.google.android.exoplayer2.offline.DownloadRequest, param1: com.google.android.exoplayer2.upstream.DataSource.Factory, param2: com.google.android.exoplayer2.drm.DrmSessionManager): com.google.android.exoplayer2.source.MediaSource;
						public getPeriodCount(): number;
						public static forMediaItem(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.MediaItem, param2: com.google.android.exoplayer2.RenderersFactory, param3: com.google.android.exoplayer2.upstream.DataSource.Factory): com.google.android.exoplayer2.offline.DownloadHelper;
						public getMappedTrackInfo(param0: number): com.google.android.exoplayer2.trackselection.MappingTrackSelector.MappedTrackInfo;
						public static forMediaItem(param0: com.google.android.exoplayer2.MediaItem, param1: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters, param2: com.google.android.exoplayer2.RenderersFactory, param3: com.google.android.exoplayer2.upstream.DataSource.Factory): com.google.android.exoplayer2.offline.DownloadHelper;
						/** @deprecated */
						public static forSmoothStreaming(param0: globalAndroid.content.Context, param1: globalAndroid.net.Uri, param2: com.google.android.exoplayer2.upstream.DataSource.Factory, param3: com.google.android.exoplayer2.RenderersFactory): com.google.android.exoplayer2.offline.DownloadHelper;
						/** @deprecated */
						public static forSmoothStreaming(param0: globalAndroid.net.Uri, param1: com.google.android.exoplayer2.upstream.DataSource.Factory, param2: com.google.android.exoplayer2.RenderersFactory): com.google.android.exoplayer2.offline.DownloadHelper;
						/** @deprecated */
						public static forDash(param0: globalAndroid.net.Uri, param1: com.google.android.exoplayer2.upstream.DataSource.Factory, param2: com.google.android.exoplayer2.RenderersFactory, param3: com.google.android.exoplayer2.drm.DrmSessionManager, param4: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters): com.google.android.exoplayer2.offline.DownloadHelper;
						public release(): void;
						public getTrackSelections(param0: number, param1: number): java.util.List<com.google.android.exoplayer2.trackselection.ExoTrackSelection>;
						public clearTrackSelections(param0: number): void;
						public static forMediaItem(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.MediaItem): com.google.android.exoplayer2.offline.DownloadHelper;
						public replaceTrackSelections(param0: number, param1: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters): void;
						/** @deprecated */
						public static forProgressive(param0: globalAndroid.content.Context, param1: globalAndroid.net.Uri): com.google.android.exoplayer2.offline.DownloadHelper;
						public getManifest(): any;
					}
					export module DownloadHelper {
						export class Callback {
							public static class: java.lang.Class<com.google.android.exoplayer2.offline.DownloadHelper.Callback>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.offline.DownloadHelper$Callback interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onPrepared(param0: com.google.android.exoplayer2.offline.DownloadHelper): void;
								onPrepareError(param0: com.google.android.exoplayer2.offline.DownloadHelper, param1: java.io.IOException): void;
							});
							public constructor();
							public onPrepareError(param0: com.google.android.exoplayer2.offline.DownloadHelper, param1: java.io.IOException): void;
							public onPrepared(param0: com.google.android.exoplayer2.offline.DownloadHelper): void;
						}
						export class DownloadTrackSelection extends com.google.android.exoplayer2.trackselection.BaseTrackSelection {
							public static class: java.lang.Class<com.google.android.exoplayer2.offline.DownloadHelper.DownloadTrackSelection>;
							public updateSelectedTrack(param0: number, param1: number, param2: number, param3: java.util.List<any>, param4: androidNative.Array<com.google.android.exoplayer2.source.chunk.MediaChunkIterator>): void;
							public onDiscontinuity(): void;
							public blacklist(param0: number, param1: number): boolean;
							public isBlacklisted(param0: number, param1: number): boolean;
							public onPlaybackSpeed(param0: number): void;
							public constructor(param0: com.google.android.exoplayer2.source.TrackGroup, param1: androidNative.Array<number>, param2: number);
							public getSelectionReason(): number;
							public shouldCancelChunkLoad(param0: number, param1: com.google.android.exoplayer2.source.chunk.Chunk, param2: java.util.List<any>): boolean;
							public getSelectedIndexInTrackGroup(): number;
							public getSelectedFormat(): com.google.android.exoplayer2.Format;
							public disable(): void;
							public getSelectionData(): any;
							public onPlayWhenReadyChanged(param0: boolean): void;
							public getSelectedIndex(): number;
							public enable(): void;
							public constructor(param0: com.google.android.exoplayer2.source.TrackGroup, param1: androidNative.Array<number>);
							public onRebuffer(): void;
							public evaluateQueueSize(param0: number, param1: java.util.List<any>): number;
						}
						export module DownloadTrackSelection {
							export class Factory extends com.google.android.exoplayer2.trackselection.ExoTrackSelection.Factory {
								public static class: java.lang.Class<com.google.android.exoplayer2.offline.DownloadHelper.DownloadTrackSelection.Factory>;
								public createTrackSelections(param0: androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection.Definition>, param1: com.google.android.exoplayer2.upstream.BandwidthMeter, param2: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param3: com.google.android.exoplayer2.Timeline): androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection>;
							}
						}
						export class FakeBandwidthMeter extends com.google.android.exoplayer2.upstream.BandwidthMeter {
							public static class: java.lang.Class<com.google.android.exoplayer2.offline.DownloadHelper.FakeBandwidthMeter>;
							public getBitrateEstimate(): number;
							public addEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.upstream.BandwidthMeter.EventListener): void;
							public removeEventListener(param0: com.google.android.exoplayer2.upstream.BandwidthMeter.EventListener): void;
							public getTransferListener(): com.google.android.exoplayer2.upstream.TransferListener;
							public getTimeToFirstByteEstimateUs(): number;
						}
						export class LiveContentUnsupportedException {
							public static class: java.lang.Class<com.google.android.exoplayer2.offline.DownloadHelper.LiveContentUnsupportedException>;
							public constructor();
						}
						export class MediaPreparer implements com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, com.google.android.exoplayer2.source.MediaPeriod.Callback {
							public static class: java.lang.Class<com.google.android.exoplayer2.offline.DownloadHelper.MediaPreparer>;
							public timeline: com.google.android.exoplayer2.Timeline;
							public mediaPeriods: androidNative.Array<com.google.android.exoplayer2.source.MediaPeriod>;
							public onSourceInfoRefreshed(param0: com.google.android.exoplayer2.source.MediaSource, param1: com.google.android.exoplayer2.Timeline): void;
							public onContinueLoadingRequested(param0: any): void;
							public handleMessage(param0: globalAndroid.os.Message): boolean;
							public release(): void;
							public onPrepared(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
							public onContinueLoadingRequested(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
							public constructor(param0: com.google.android.exoplayer2.source.MediaSource, param1: com.google.android.exoplayer2.offline.DownloadHelper);
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module offline {
					export class DownloadIndex {
						public static class: java.lang.Class<com.google.android.exoplayer2.offline.DownloadIndex>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.offline.DownloadIndex interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							getDownload(param0: string): com.google.android.exoplayer2.offline.Download;
							getDownloads(param0: androidNative.Array<number>): com.google.android.exoplayer2.offline.DownloadCursor;
						});
						public constructor();
						public getDownloads(param0: androidNative.Array<number>): com.google.android.exoplayer2.offline.DownloadCursor;
						public getDownload(param0: string): com.google.android.exoplayer2.offline.Download;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module offline {
					export class DownloadManager {
						public static class: java.lang.Class<com.google.android.exoplayer2.offline.DownloadManager>;
						public static DEFAULT_MAX_PARALLEL_DOWNLOADS: number;
						public static DEFAULT_MIN_RETRY_COUNT: number;
						public static DEFAULT_REQUIREMENTS: com.google.android.exoplayer2.scheduler.Requirements;
						public resumeDownloads(): void;
						public addDownload(param0: com.google.android.exoplayer2.offline.DownloadRequest, param1: number): void;
						public setMaxParallelDownloads(param0: number): void;
						public getCurrentDownloads(): java.util.List<com.google.android.exoplayer2.offline.Download>;
						/** @deprecated */
						public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.database.DatabaseProvider, param2: com.google.android.exoplayer2.upstream.cache.Cache, param3: com.google.android.exoplayer2.upstream.DataSource.Factory);
						public isInitialized(): boolean;
						public getDownloadsPaused(): boolean;
						public getNotMetRequirements(): number;
						public setRequirements(param0: com.google.android.exoplayer2.scheduler.Requirements): void;
						public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.database.DatabaseProvider, param2: com.google.android.exoplayer2.upstream.cache.Cache, param3: com.google.android.exoplayer2.upstream.DataSource.Factory, param4: java.util.concurrent.Executor);
						public setMinRetryCount(param0: number): void;
						public getMinRetryCount(): number;
						public addDownload(param0: com.google.android.exoplayer2.offline.DownloadRequest): void;
						public isWaitingForRequirements(): boolean;
						public getRequirements(): com.google.android.exoplayer2.scheduler.Requirements;
						public release(): void;
						public removeDownload(param0: string): void;
						public isIdle(): boolean;
						public setStopReason(param0: string, param1: number): void;
						public pauseDownloads(): void;
						public getDownloadIndex(): com.google.android.exoplayer2.offline.DownloadIndex;
						public removeAllDownloads(): void;
						public addListener(param0: com.google.android.exoplayer2.offline.DownloadManager.Listener): void;
						public getMaxParallelDownloads(): number;
						public removeListener(param0: com.google.android.exoplayer2.offline.DownloadManager.Listener): void;
						public getApplicationLooper(): globalAndroid.os.Looper;
						public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.offline.WritableDownloadIndex, param2: com.google.android.exoplayer2.offline.DownloaderFactory);
					}
					export module DownloadManager {
						export class DownloadUpdate {
							public static class: java.lang.Class<com.google.android.exoplayer2.offline.DownloadManager.DownloadUpdate>;
							public download: com.google.android.exoplayer2.offline.Download;
							public isRemove: boolean;
							public downloads: java.util.List<com.google.android.exoplayer2.offline.Download>;
							public finalException: java.lang.Exception;
							public constructor(param0: com.google.android.exoplayer2.offline.Download, param1: boolean, param2: java.util.List<com.google.android.exoplayer2.offline.Download>, param3: java.lang.Exception);
						}
						export class InternalHandler {
							public static class: java.lang.Class<com.google.android.exoplayer2.offline.DownloadManager.InternalHandler>;
							public released: boolean;
							public constructor(param0: globalAndroid.os.HandlerThread, param1: com.google.android.exoplayer2.offline.WritableDownloadIndex, param2: com.google.android.exoplayer2.offline.DownloaderFactory, param3: globalAndroid.os.Handler, param4: number, param5: number, param6: boolean);
							public handleMessage(param0: globalAndroid.os.Message): void;
						}
						export class Listener {
							public static class: java.lang.Class<com.google.android.exoplayer2.offline.DownloadManager.Listener>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.offline.DownloadManager$Listener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onInitialized(param0: com.google.android.exoplayer2.offline.DownloadManager): void;
								onDownloadsPausedChanged(param0: com.google.android.exoplayer2.offline.DownloadManager, param1: boolean): void;
								onDownloadChanged(param0: com.google.android.exoplayer2.offline.DownloadManager, param1: com.google.android.exoplayer2.offline.Download, param2: java.lang.Exception): void;
								onDownloadRemoved(param0: com.google.android.exoplayer2.offline.DownloadManager, param1: com.google.android.exoplayer2.offline.Download): void;
								onIdle(param0: com.google.android.exoplayer2.offline.DownloadManager): void;
								onRequirementsStateChanged(param0: com.google.android.exoplayer2.offline.DownloadManager, param1: com.google.android.exoplayer2.scheduler.Requirements, param2: number): void;
								onWaitingForRequirementsChanged(param0: com.google.android.exoplayer2.offline.DownloadManager, param1: boolean): void;
							});
							public constructor();
							public onInitialized(param0: com.google.android.exoplayer2.offline.DownloadManager): void;
							public onRequirementsStateChanged(param0: com.google.android.exoplayer2.offline.DownloadManager, param1: com.google.android.exoplayer2.scheduler.Requirements, param2: number): void;
							public onWaitingForRequirementsChanged(param0: com.google.android.exoplayer2.offline.DownloadManager, param1: boolean): void;
							public onIdle(param0: com.google.android.exoplayer2.offline.DownloadManager): void;
							public onDownloadsPausedChanged(param0: com.google.android.exoplayer2.offline.DownloadManager, param1: boolean): void;
							public onDownloadRemoved(param0: com.google.android.exoplayer2.offline.DownloadManager, param1: com.google.android.exoplayer2.offline.Download): void;
							public onDownloadChanged(param0: com.google.android.exoplayer2.offline.DownloadManager, param1: com.google.android.exoplayer2.offline.Download, param2: java.lang.Exception): void;
						}
						export class Task implements com.google.android.exoplayer2.offline.Downloader.ProgressListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.offline.DownloadManager.Task>;
							public cancel(param0: boolean): void;
							public onProgress(param0: number, param1: number, param2: number): void;
							public run(): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module offline {
					export class DownloadProgress {
						public static class: java.lang.Class<com.google.android.exoplayer2.offline.DownloadProgress>;
						public bytesDownloaded: number;
						public percentDownloaded: number;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module offline {
					export class DownloadRequest {
						public static class: java.lang.Class<com.google.android.exoplayer2.offline.DownloadRequest>;
						public id: string;
						public uri: globalAndroid.net.Uri;
						public mimeType: string;
						public streamKeys: java.util.List<com.google.android.exoplayer2.offline.StreamKey>;
						public keySetId: androidNative.Array<number>;
						public customCacheKey: string;
						public data: androidNative.Array<number>;
						public static CREATOR: globalAndroid.os.Parcelable.Creator<com.google.android.exoplayer2.offline.DownloadRequest>;
						public copyWithId(param0: string): com.google.android.exoplayer2.offline.DownloadRequest;
						public toMediaItem(): com.google.android.exoplayer2.MediaItem;
						public describeContents(): number;
						public copyWithKeySetId(param0: androidNative.Array<number>): com.google.android.exoplayer2.offline.DownloadRequest;
						public copyWithMergedRequest(param0: com.google.android.exoplayer2.offline.DownloadRequest): com.google.android.exoplayer2.offline.DownloadRequest;
						public writeToParcel(param0: globalAndroid.os.Parcel, param1: number): void;
						public equals(param0: any): boolean;
						public hashCode(): number;
						public toString(): string;
					}
					export module DownloadRequest {
						export class Builder {
							public static class: java.lang.Class<com.google.android.exoplayer2.offline.DownloadRequest.Builder>;
							public build(): com.google.android.exoplayer2.offline.DownloadRequest;
							public constructor(param0: string, param1: globalAndroid.net.Uri);
							public setKeySetId(param0: androidNative.Array<number>): com.google.android.exoplayer2.offline.DownloadRequest.Builder;
							public setMimeType(param0: string): com.google.android.exoplayer2.offline.DownloadRequest.Builder;
							public setData(param0: androidNative.Array<number>): com.google.android.exoplayer2.offline.DownloadRequest.Builder;
							public setStreamKeys(param0: java.util.List<com.google.android.exoplayer2.offline.StreamKey>): com.google.android.exoplayer2.offline.DownloadRequest.Builder;
							public setCustomCacheKey(param0: string): com.google.android.exoplayer2.offline.DownloadRequest.Builder;
						}
						export class UnsupportedRequestException {
							public static class: java.lang.Class<com.google.android.exoplayer2.offline.DownloadRequest.UnsupportedRequestException>;
							public constructor();
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module offline {
					export abstract class DownloadService {
						public static class: java.lang.Class<com.google.android.exoplayer2.offline.DownloadService>;
						public static ACTION_INIT: string;
						public static ACTION_ADD_DOWNLOAD: string;
						public static ACTION_REMOVE_DOWNLOAD: string;
						public static ACTION_REMOVE_ALL_DOWNLOADS: string;
						public static ACTION_RESUME_DOWNLOADS: string;
						public static ACTION_PAUSE_DOWNLOADS: string;
						public static ACTION_SET_STOP_REASON: string;
						public static ACTION_SET_REQUIREMENTS: string;
						public static KEY_DOWNLOAD_REQUEST: string;
						public static KEY_CONTENT_ID: string;
						public static KEY_STOP_REASON: string;
						public static KEY_REQUIREMENTS: string;
						public static KEY_FOREGROUND: string;
						public static FOREGROUND_NOTIFICATION_ID_NONE: number;
						public static DEFAULT_FOREGROUND_NOTIFICATION_UPDATE_INTERVAL: number;
						public getDownloadManager(): com.google.android.exoplayer2.offline.DownloadManager;
						public static sendResumeDownloads(param0: globalAndroid.content.Context, param1: java.lang.Class<any>, param2: boolean): void;
						public getScheduler(): com.google.android.exoplayer2.scheduler.Scheduler;
						public static buildAddDownloadIntent(param0: globalAndroid.content.Context, param1: java.lang.Class<any>, param2: com.google.android.exoplayer2.offline.DownloadRequest, param3: boolean): globalAndroid.content.Intent;
						public getForegroundNotification(param0: java.util.List<com.google.android.exoplayer2.offline.Download>, param1: number): globalAndroid.app.Notification;
						public static sendSetStopReason(param0: globalAndroid.content.Context, param1: java.lang.Class<any>, param2: string, param3: number, param4: boolean): void;
						public static buildPauseDownloadsIntent(param0: globalAndroid.content.Context, param1: java.lang.Class<any>, param2: boolean): globalAndroid.content.Intent;
						public static start(param0: globalAndroid.content.Context, param1: java.lang.Class<any>): void;
						public onDestroy(): void;
						public onTaskRemoved(param0: globalAndroid.content.Intent): void;
						public static buildSetRequirementsIntent(param0: globalAndroid.content.Context, param1: java.lang.Class<any>, param2: com.google.android.exoplayer2.scheduler.Requirements, param3: boolean): globalAndroid.content.Intent;
						public constructor(param0: number, param1: number);
						public static buildRemoveAllDownloadsIntent(param0: globalAndroid.content.Context, param1: java.lang.Class<any>, param2: boolean): globalAndroid.content.Intent;
						public static sendSetRequirements(param0: globalAndroid.content.Context, param1: java.lang.Class<any>, param2: com.google.android.exoplayer2.scheduler.Requirements, param3: boolean): void;
						public static buildAddDownloadIntent(param0: globalAndroid.content.Context, param1: java.lang.Class<any>, param2: com.google.android.exoplayer2.offline.DownloadRequest, param3: number, param4: boolean): globalAndroid.content.Intent;
						public static sendRemoveAllDownloads(param0: globalAndroid.content.Context, param1: java.lang.Class<any>, param2: boolean): void;
						public onBind(param0: globalAndroid.content.Intent): globalAndroid.os.IBinder;
						public invalidateForegroundNotification(): void;
						public static buildSetStopReasonIntent(param0: globalAndroid.content.Context, param1: java.lang.Class<any>, param2: string, param3: number, param4: boolean): globalAndroid.content.Intent;
						public static sendAddDownload(param0: globalAndroid.content.Context, param1: java.lang.Class<any>, param2: com.google.android.exoplayer2.offline.DownloadRequest, param3: boolean): void;
						public static buildResumeDownloadsIntent(param0: globalAndroid.content.Context, param1: java.lang.Class<any>, param2: boolean): globalAndroid.content.Intent;
						public static sendPauseDownloads(param0: globalAndroid.content.Context, param1: java.lang.Class<any>, param2: boolean): void;
						public static sendAddDownload(param0: globalAndroid.content.Context, param1: java.lang.Class<any>, param2: com.google.android.exoplayer2.offline.DownloadRequest, param3: number, param4: boolean): void;
						public static sendRemoveDownload(param0: globalAndroid.content.Context, param1: java.lang.Class<any>, param2: string, param3: boolean): void;
						public constructor(param0: number);
						public static buildRemoveDownloadIntent(param0: globalAndroid.content.Context, param1: java.lang.Class<any>, param2: string, param3: boolean): globalAndroid.content.Intent;
						public onCreate(): void;
						public static startForeground(param0: globalAndroid.content.Context, param1: java.lang.Class<any>): void;
						/** @deprecated */
						public constructor(param0: number, param1: number, param2: string, param3: number);
						public constructor(param0: number, param1: number, param2: string, param3: number, param4: number);
						public onStartCommand(param0: globalAndroid.content.Intent, param1: number, param2: number): number;
					}
					export module DownloadService {
						export class DownloadManagerHelper extends com.google.android.exoplayer2.offline.DownloadManager.Listener {
							public static class: java.lang.Class<com.google.android.exoplayer2.offline.DownloadService.DownloadManagerHelper>;
							public updateScheduler(): boolean;
							public onInitialized(param0: com.google.android.exoplayer2.offline.DownloadManager): void;
							public onRequirementsStateChanged(param0: com.google.android.exoplayer2.offline.DownloadManager, param1: com.google.android.exoplayer2.scheduler.Requirements, param2: number): void;
							public onWaitingForRequirementsChanged(param0: com.google.android.exoplayer2.offline.DownloadManager, param1: boolean): void;
							public onIdle(param0: com.google.android.exoplayer2.offline.DownloadManager): void;
							public onDownloadsPausedChanged(param0: com.google.android.exoplayer2.offline.DownloadManager, param1: boolean): void;
							public attachService(param0: com.google.android.exoplayer2.offline.DownloadService): void;
							public detachService(param0: com.google.android.exoplayer2.offline.DownloadService): void;
							public onDownloadRemoved(param0: com.google.android.exoplayer2.offline.DownloadManager, param1: com.google.android.exoplayer2.offline.Download): void;
							public onDownloadChanged(param0: com.google.android.exoplayer2.offline.DownloadManager, param1: com.google.android.exoplayer2.offline.Download, param2: java.lang.Exception): void;
						}
						export class ForegroundNotificationUpdater {
							public static class: java.lang.Class<com.google.android.exoplayer2.offline.DownloadService.ForegroundNotificationUpdater>;
							public startPeriodicUpdates(): void;
							public stopPeriodicUpdates(): void;
							public constructor(param0: com.google.android.exoplayer2.offline.DownloadService, param1: number, param2: number);
							public invalidate(): void;
							public showNotificationIfNotAlready(): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module offline {
					export class Downloader {
						public static class: java.lang.Class<com.google.android.exoplayer2.offline.Downloader>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.offline.Downloader interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							download(param0: com.google.android.exoplayer2.offline.Downloader.ProgressListener): void;
							cancel(): void;
							remove(): void;
						});
						public constructor();
						public cancel(): void;
						public download(param0: com.google.android.exoplayer2.offline.Downloader.ProgressListener): void;
						public remove(): void;
					}
					export module Downloader {
						export class ProgressListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.offline.Downloader.ProgressListener>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.offline.Downloader$ProgressListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onProgress(param0: number, param1: number, param2: number): void;
							});
							public constructor();
							public onProgress(param0: number, param1: number, param2: number): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module offline {
					export class DownloaderFactory {
						public static class: java.lang.Class<com.google.android.exoplayer2.offline.DownloaderFactory>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.offline.DownloaderFactory interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							createDownloader(param0: com.google.android.exoplayer2.offline.DownloadRequest): com.google.android.exoplayer2.offline.Downloader;
						});
						public constructor();
						public createDownloader(param0: com.google.android.exoplayer2.offline.DownloadRequest): com.google.android.exoplayer2.offline.Downloader;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module offline {
					export class FilterableManifest<T>  extends java.lang.Object {
						public static class: java.lang.Class<com.google.android.exoplayer2.offline.FilterableManifest<any>>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.offline.FilterableManifest<any> interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							copy(param0: java.util.List<com.google.android.exoplayer2.offline.StreamKey>): T;
						});
						public constructor();
						public copy(param0: java.util.List<com.google.android.exoplayer2.offline.StreamKey>): T;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module offline {
					export class FilteringManifestParser<T>  extends com.google.android.exoplayer2.upstream.ParsingLoadable.Parser<any> {
						public static class: java.lang.Class<com.google.android.exoplayer2.offline.FilteringManifestParser<any>>;
						public parse(param0: globalAndroid.net.Uri, param1: java.io.InputStream): any;
						public constructor(param0: com.google.android.exoplayer2.upstream.ParsingLoadable.Parser<any>, param1: java.util.List<com.google.android.exoplayer2.offline.StreamKey>);
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module offline {
					export class ProgressiveDownloader extends com.google.android.exoplayer2.offline.Downloader {
						public static class: java.lang.Class<com.google.android.exoplayer2.offline.ProgressiveDownloader>;
						public constructor(param0: com.google.android.exoplayer2.MediaItem, param1: com.google.android.exoplayer2.upstream.cache.CacheDataSource.Factory);
						public constructor(param0: com.google.android.exoplayer2.MediaItem, param1: com.google.android.exoplayer2.upstream.cache.CacheDataSource.Factory, param2: java.util.concurrent.Executor);
						public cancel(): void;
						public download(param0: com.google.android.exoplayer2.offline.Downloader.ProgressListener): void;
						public remove(): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module offline {
					export abstract class SegmentDownloader<M>  extends com.google.android.exoplayer2.offline.Downloader {
						public static class: java.lang.Class<com.google.android.exoplayer2.offline.SegmentDownloader<any>>;
						public cancel(): void;
						public getManifest(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: boolean): any;
						public constructor(param0: com.google.android.exoplayer2.MediaItem, param1: com.google.android.exoplayer2.upstream.ParsingLoadable.Parser<any>, param2: com.google.android.exoplayer2.upstream.cache.CacheDataSource.Factory, param3: java.util.concurrent.Executor);
						public execute(param0: com.google.android.exoplayer2.util.RunnableFutureTask, param1: boolean): any;
						public static getCompressibleDataSpec(param0: globalAndroid.net.Uri): com.google.android.exoplayer2.upstream.DataSpec;
						public download(param0: com.google.android.exoplayer2.offline.Downloader.ProgressListener): void;
						public remove(): void;
						public getSegments(param0: com.google.android.exoplayer2.upstream.DataSource, param1: any, param2: boolean): java.util.List<com.google.android.exoplayer2.offline.SegmentDownloader.Segment>;
					}
					export module SegmentDownloader {
						export class ProgressNotifier extends com.google.android.exoplayer2.upstream.cache.CacheWriter.ProgressListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.offline.SegmentDownloader.ProgressNotifier>;
							public constructor(param0: com.google.android.exoplayer2.offline.Downloader.ProgressListener, param1: number, param2: number, param3: number, param4: number);
							public onProgress(param0: number, param1: number, param2: number): void;
							public onSegmentDownloaded(): void;
						}
						export class Segment extends java.lang.Comparable<com.google.android.exoplayer2.offline.SegmentDownloader.Segment> {
							public static class: java.lang.Class<com.google.android.exoplayer2.offline.SegmentDownloader.Segment>;
							public startTimeUs: number;
							public dataSpec: com.google.android.exoplayer2.upstream.DataSpec;
							public constructor(param0: number, param1: com.google.android.exoplayer2.upstream.DataSpec);
							public compareTo(param0: com.google.android.exoplayer2.offline.SegmentDownloader.Segment): number;
						}
						export class SegmentDownloadRunnable extends com.google.android.exoplayer2.util.RunnableFutureTask<java.lang.Void,java.io.IOException> {
							public static class: java.lang.Class<com.google.android.exoplayer2.offline.SegmentDownloader.SegmentDownloadRunnable>;
							public segment: com.google.android.exoplayer2.offline.SegmentDownloader.Segment;
							public dataSource: com.google.android.exoplayer2.upstream.cache.CacheDataSource;
							public temporaryBuffer: androidNative.Array<number>;
							public doWork(): java.lang.Void;
							public cancelWork(): void;
							public constructor(param0: com.google.android.exoplayer2.offline.SegmentDownloader.Segment, param1: com.google.android.exoplayer2.upstream.cache.CacheDataSource, param2: com.google.android.exoplayer2.offline.SegmentDownloader.ProgressNotifier, param3: androidNative.Array<number>);
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module offline {
					export class WritableDownloadIndex extends com.google.android.exoplayer2.offline.DownloadIndex {
						public static class: java.lang.Class<com.google.android.exoplayer2.offline.WritableDownloadIndex>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.offline.WritableDownloadIndex interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							putDownload(param0: com.google.android.exoplayer2.offline.Download): void;
							removeDownload(param0: string): void;
							setDownloadingStatesToQueued(): void;
							setStatesToRemoving(): void;
							setStopReason(param0: number): void;
							setStopReason(param0: string, param1: number): void;
							getDownload(param0: string): com.google.android.exoplayer2.offline.Download;
							getDownloads(param0: androidNative.Array<number>): com.google.android.exoplayer2.offline.DownloadCursor;
						});
						public constructor();
						public setStopReason(param0: string, param1: number): void;
						public setDownloadingStatesToQueued(): void;
						public putDownload(param0: com.google.android.exoplayer2.offline.Download): void;
						public getDownloads(param0: androidNative.Array<number>): com.google.android.exoplayer2.offline.DownloadCursor;
						public getDownload(param0: string): com.google.android.exoplayer2.offline.Download;
						public setStatesToRemoving(): void;
						public removeDownload(param0: string): void;
						public setStopReason(param0: number): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module scheduler {
					export class PlatformScheduler extends com.google.android.exoplayer2.scheduler.Scheduler {
						public static class: java.lang.Class<com.google.android.exoplayer2.scheduler.PlatformScheduler>;
						public getSupportedRequirements(param0: com.google.android.exoplayer2.scheduler.Requirements): com.google.android.exoplayer2.scheduler.Requirements;
						public constructor(param0: globalAndroid.content.Context, param1: number);
						public cancel(): boolean;
						public schedule(param0: com.google.android.exoplayer2.scheduler.Requirements, param1: string, param2: string): boolean;
					}
					export module PlatformScheduler {
						export class PlatformSchedulerService {
							public static class: java.lang.Class<com.google.android.exoplayer2.scheduler.PlatformScheduler.PlatformSchedulerService>;
							public constructor();
							public onStopJob(param0: any): boolean;
							public onStartJob(param0: any): boolean;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module scheduler {
					export class Requirements {
						public static class: java.lang.Class<com.google.android.exoplayer2.scheduler.Requirements>;
						public static NETWORK: number;
						public static NETWORK_UNMETERED: number;
						public static DEVICE_IDLE: number;
						public static DEVICE_CHARGING: number;
						public static DEVICE_STORAGE_NOT_LOW: number;
						public static CREATOR: globalAndroid.os.Parcelable.Creator<com.google.android.exoplayer2.scheduler.Requirements>;
						public isStorageNotLowRequired(): boolean;
						public describeContents(): number;
						public isIdleRequired(): boolean;
						public isChargingRequired(): boolean;
						public writeToParcel(param0: globalAndroid.os.Parcel, param1: number): void;
						public equals(param0: any): boolean;
						public hashCode(): number;
						public isNetworkRequired(): boolean;
						public getRequirements(): number;
						public checkRequirements(param0: globalAndroid.content.Context): boolean;
						public constructor(param0: number);
						public isUnmeteredNetworkRequired(): boolean;
						public getNotMetRequirements(param0: globalAndroid.content.Context): number;
						public filterRequirements(param0: number): com.google.android.exoplayer2.scheduler.Requirements;
					}
					export module Requirements {
						export class RequirementFlags {
							public static class: java.lang.Class<com.google.android.exoplayer2.scheduler.Requirements.RequirementFlags>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.scheduler.Requirements$RequirementFlags interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
							});
							public constructor();
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module scheduler {
					export class RequirementsWatcher {
						public static class: java.lang.Class<com.google.android.exoplayer2.scheduler.RequirementsWatcher>;
						public start(): number;
						public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.scheduler.RequirementsWatcher.Listener, param2: com.google.android.exoplayer2.scheduler.Requirements);
						public getRequirements(): com.google.android.exoplayer2.scheduler.Requirements;
						public stop(): void;
					}
					export module RequirementsWatcher {
						export class DeviceStatusChangeReceiver {
							public static class: java.lang.Class<com.google.android.exoplayer2.scheduler.RequirementsWatcher.DeviceStatusChangeReceiver>;
							public onReceive(param0: globalAndroid.content.Context, param1: globalAndroid.content.Intent): void;
						}
						export class Listener {
							public static class: java.lang.Class<com.google.android.exoplayer2.scheduler.RequirementsWatcher.Listener>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.scheduler.RequirementsWatcher$Listener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onRequirementsStateChanged(param0: com.google.android.exoplayer2.scheduler.RequirementsWatcher, param1: number): void;
							});
							public constructor();
							public onRequirementsStateChanged(param0: com.google.android.exoplayer2.scheduler.RequirementsWatcher, param1: number): void;
						}
						export class NetworkCallback {
							public static class: java.lang.Class<com.google.android.exoplayer2.scheduler.RequirementsWatcher.NetworkCallback>;
							public onLost(param0: globalAndroid.net.Network): void;
							public onBlockedStatusChanged(param0: globalAndroid.net.Network, param1: boolean): void;
							public onAvailable(param0: globalAndroid.net.Network): void;
							public onCapabilitiesChanged(param0: globalAndroid.net.Network, param1: globalAndroid.net.NetworkCapabilities): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module scheduler {
					export class Scheduler {
						public static class: java.lang.Class<com.google.android.exoplayer2.scheduler.Scheduler>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.scheduler.Scheduler interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							schedule(param0: com.google.android.exoplayer2.scheduler.Requirements, param1: string, param2: string): boolean;
							cancel(): boolean;
							getSupportedRequirements(param0: com.google.android.exoplayer2.scheduler.Requirements): com.google.android.exoplayer2.scheduler.Requirements;
						});
						public constructor();
						public getSupportedRequirements(param0: com.google.android.exoplayer2.scheduler.Requirements): com.google.android.exoplayer2.scheduler.Requirements;
						public cancel(): boolean;
						public schedule(param0: com.google.android.exoplayer2.scheduler.Requirements, param1: string, param2: string): boolean;
					}
				}
			}
		}
	}
}


declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export abstract class BaseMediaSource extends com.google.android.exoplayer2.source.MediaSource {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.BaseMediaSource>;
						public disableInternal(): void;
						public enableInternal(): void;
						public createPeriod(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param1: com.google.android.exoplayer2.upstream.Allocator, param2: number): com.google.android.exoplayer2.source.MediaPeriod;
						public getPlayerId(): com.google.android.exoplayer2.analytics.PlayerId;
						public getInitialTimeline(): com.google.android.exoplayer2.Timeline;
						public removeDrmEventListener(param0: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
						public createEventDispatcher(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): com.google.android.exoplayer2.source.MediaSourceEventListener.EventDispatcher;
						public createEventDispatcher(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: number): com.google.android.exoplayer2.source.MediaSourceEventListener.EventDispatcher;
						public getMediaItem(): com.google.android.exoplayer2.MediaItem;
						public enable(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public constructor();
						public isSingleWindow(): boolean;
						/** @deprecated */
						public prepareSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param1: com.google.android.exoplayer2.upstream.TransferListener): void;
						public prepareSourceInternal(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
						public maybeThrowSourceInfoRefreshError(): void;
						public createDrmEventDispatcher(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher;
						public addEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
						public createDrmEventDispatcher(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher;
						public isEnabled(): boolean;
						public refreshSourceInfo(param0: com.google.android.exoplayer2.Timeline): void;
						public addDrmEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
						public releaseSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public releaseSourceInternal(): void;
						public disable(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public releasePeriod(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
						public prepareSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param1: com.google.android.exoplayer2.upstream.TransferListener, param2: com.google.android.exoplayer2.analytics.PlayerId): void;
						public removeEventListener(param0: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
						public createEventDispatcher(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param1: number): com.google.android.exoplayer2.source.MediaSourceEventListener.EventDispatcher;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class BehindLiveWindowException {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.BehindLiveWindowException>;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class BundledExtractorsAdapter extends com.google.android.exoplayer2.source.ProgressiveMediaExtractor {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.BundledExtractorsAdapter>;
						public disableSeekingOnMp3Streams(): void;
						public getCurrentInputPosition(): number;
						public init(param0: com.google.android.exoplayer2.upstream.DataReader, param1: globalAndroid.net.Uri, param2: java.util.Map<string,java.util.List<string>>, param3: number, param4: number, param5: com.google.android.exoplayer2.extractor.ExtractorOutput): void;
						public seek(param0: number, param1: number): void;
						public constructor(param0: com.google.android.exoplayer2.extractor.ExtractorsFactory);
						public release(): void;
						public read(param0: com.google.android.exoplayer2.extractor.PositionHolder): number;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class ClippingMediaPeriod implements com.google.android.exoplayer2.source.MediaPeriod, com.google.android.exoplayer2.source.MediaPeriod.Callback {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.ClippingMediaPeriod>;
						public mediaPeriod: com.google.android.exoplayer2.source.MediaPeriod;
						public selectTracks(param0: androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection>, param1: androidNative.Array<boolean>, param2: androidNative.Array<com.google.android.exoplayer2.source.SampleStream>, param3: androidNative.Array<boolean>, param4: number): number;
						public readDiscontinuity(): number;
						public isLoading(): boolean;
						public setClippingError(param0: com.google.android.exoplayer2.source.ClippingMediaSource.IllegalClippingException): void;
						public onPrepared(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
						public continueLoading(param0: number): boolean;
						public discardBuffer(param0: number, param1: boolean): void;
						public constructor(param0: com.google.android.exoplayer2.source.MediaPeriod, param1: boolean, param2: number, param3: number);
						public prepare(param0: com.google.android.exoplayer2.source.MediaPeriod.Callback, param1: number): void;
						public getAdjustedSeekPositionUs(param0: number, param1: com.google.android.exoplayer2.SeekParameters): number;
						public getNextLoadPositionUs(): number;
						public seekToUs(param0: number): number;
						public onContinueLoadingRequested(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
						public updateClipping(param0: number, param1: number): void;
						public getBufferedPositionUs(): number;
						public getStreamKeys(param0: java.util.List<com.google.android.exoplayer2.trackselection.ExoTrackSelection>): java.util.List<com.google.android.exoplayer2.offline.StreamKey>;
						public reevaluateBuffer(param0: number): void;
						public onContinueLoadingRequested(param0: any): void;
						public getTrackGroups(): com.google.android.exoplayer2.source.TrackGroupArray;
						public maybeThrowPrepareError(): void;
					}
					export module ClippingMediaPeriod {
						export class ClippingSampleStream extends com.google.android.exoplayer2.source.SampleStream {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.ClippingMediaPeriod.ClippingSampleStream>;
							public childStream: com.google.android.exoplayer2.source.SampleStream;
							public clearSentEos(): void;
							public isReady(): boolean;
							public skipData(param0: number): number;
							public constructor(param0: com.google.android.exoplayer2.source.ClippingMediaPeriod, param1: com.google.android.exoplayer2.source.SampleStream);
							public maybeThrowError(): void;
							public readData(param0: com.google.android.exoplayer2.FormatHolder, param1: com.google.android.exoplayer2.decoder.DecoderInputBuffer, param2: number): number;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class ClippingMediaSource extends com.google.android.exoplayer2.source.CompositeMediaSource<java.lang.Void> {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.ClippingMediaSource>;
						public enable(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public constructor();
						public isSingleWindow(): boolean;
						/** @deprecated */
						public prepareSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param1: com.google.android.exoplayer2.upstream.TransferListener): void;
						public constructor(param0: com.google.android.exoplayer2.source.MediaSource, param1: number, param2: number);
						public prepareSourceInternal(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
						public maybeThrowSourceInfoRefreshError(): void;
						public createPeriod(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param1: com.google.android.exoplayer2.upstream.Allocator, param2: number): com.google.android.exoplayer2.source.MediaPeriod;
						public addEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
						public getInitialTimeline(): com.google.android.exoplayer2.Timeline;
						public constructor(param0: com.google.android.exoplayer2.source.MediaSource, param1: number);
						public constructor(param0: com.google.android.exoplayer2.source.MediaSource, param1: number, param2: number, param3: boolean, param4: boolean, param5: boolean);
						public addDrmEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
						public releaseSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public releaseSourceInternal(): void;
						public removeDrmEventListener(param0: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
						public onChildSourceInfoRefreshed(param0: any, param1: com.google.android.exoplayer2.source.MediaSource, param2: com.google.android.exoplayer2.Timeline): void;
						public disable(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public releasePeriod(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
						public prepareSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param1: com.google.android.exoplayer2.upstream.TransferListener, param2: com.google.android.exoplayer2.analytics.PlayerId): void;
						public removeEventListener(param0: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
						public getMediaItem(): com.google.android.exoplayer2.MediaItem;
						public onChildSourceInfoRefreshed(param0: java.lang.Void, param1: com.google.android.exoplayer2.source.MediaSource, param2: com.google.android.exoplayer2.Timeline): void;
					}
					export module ClippingMediaSource {
						export class ClippingTimeline extends com.google.android.exoplayer2.source.ForwardingTimeline {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.ClippingMediaSource.ClippingTimeline>;
							public constructor(param0: com.google.android.exoplayer2.Timeline, param1: number, param2: number);
							public getWindow(param0: number, param1: com.google.android.exoplayer2.Timeline.Window, param2: number): com.google.android.exoplayer2.Timeline.Window;
							public getPeriod(param0: number, param1: com.google.android.exoplayer2.Timeline.Period, param2: boolean): com.google.android.exoplayer2.Timeline.Period;
							public constructor(param0: com.google.android.exoplayer2.Timeline);
						}
						export class IllegalClippingException {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.ClippingMediaSource.IllegalClippingException>;
							public static REASON_INVALID_PERIOD_COUNT: number;
							public static REASON_NOT_SEEKABLE_TO_START: number;
							public static REASON_START_EXCEEDS_END: number;
							public reason: number;
							public constructor(param0: number);
						}
						export module IllegalClippingException {
							export class Reason {
								public static class: java.lang.Class<com.google.android.exoplayer2.source.ClippingMediaSource.IllegalClippingException.Reason>;
								/**
								 * Constructs a new instance of the com.google.android.exoplayer2.source.ClippingMediaSource$IllegalClippingException$Reason interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
								 */
								public constructor(implementation: {
								});
								public constructor();
							}
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export abstract class CompositeMediaSource<T>  extends com.google.android.exoplayer2.source.BaseMediaSource {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.CompositeMediaSource<any>>;
						public disableInternal(): void;
						public prepareChildSource(param0: any, param1: com.google.android.exoplayer2.source.MediaSource): void;
						public enableInternal(): void;
						public createPeriod(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param1: com.google.android.exoplayer2.upstream.Allocator, param2: number): com.google.android.exoplayer2.source.MediaPeriod;
						public releaseChildSource(param0: any): void;
						public getInitialTimeline(): com.google.android.exoplayer2.Timeline;
						public removeDrmEventListener(param0: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
						public onChildSourceInfoRefreshed(param0: any, param1: com.google.android.exoplayer2.source.MediaSource, param2: com.google.android.exoplayer2.Timeline): void;
						public enableChildSource(param0: any): void;
						public disableChildSource(param0: any): void;
						public getMediaItem(): com.google.android.exoplayer2.MediaItem;
						public enable(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public constructor();
						public getMediaTimeForChildMediaTime(param0: any, param1: number): number;
						public isSingleWindow(): boolean;
						/** @deprecated */
						public prepareSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param1: com.google.android.exoplayer2.upstream.TransferListener): void;
						public prepareSourceInternal(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
						public maybeThrowSourceInfoRefreshError(): void;
						public addEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
						public addDrmEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
						public releaseSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public releaseSourceInternal(): void;
						public disable(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public getWindowIndexForChildWindowIndex(param0: any, param1: number): number;
						public releasePeriod(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
						public prepareSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param1: com.google.android.exoplayer2.upstream.TransferListener, param2: com.google.android.exoplayer2.analytics.PlayerId): void;
						public getMediaPeriodIdForChildMediaPeriodId(param0: any, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): com.google.android.exoplayer2.source.MediaSource.MediaPeriodId;
						public removeEventListener(param0: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
					}
					export module CompositeMediaSource {
						export class ForwardingEventListener implements com.google.android.exoplayer2.source.MediaSourceEventListener, com.google.android.exoplayer2.drm.DrmSessionEventListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.CompositeMediaSource.ForwardingEventListener>;
							public onLoadStarted(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData): void;
							public onDrmSessionAcquired(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: number): void;
							/** @deprecated */
							public onDrmSessionAcquired(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
							public onUpstreamDiscarded(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
							public onDrmKeysRestored(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
							public onDrmKeysLoaded(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
							public onDrmSessionManagerError(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: java.lang.Exception): void;
							public onLoadCanceled(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData): void;
							public onDrmSessionReleased(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
							public onDownstreamFormatChanged(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
							public onDrmKeysRemoved(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
							public constructor(param0: any);
							public onLoadCompleted(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData): void;
							public onLoadError(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData, param4: java.io.IOException, param5: boolean): void;
						}
						export class MediaSourceAndListener<T>  extends java.lang.Object {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.CompositeMediaSource.MediaSourceAndListener<any>>;
							public mediaSource: com.google.android.exoplayer2.source.MediaSource;
							public caller: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller;
							public eventListener: com.google.android.exoplayer2.source.CompositeMediaSource.ForwardingEventListener;
							public constructor(param0: com.google.android.exoplayer2.source.MediaSource, param1: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param2: com.google.android.exoplayer2.source.CompositeMediaSource.ForwardingEventListener);
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class CompositeSequenceableLoader extends com.google.android.exoplayer2.source.SequenceableLoader {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.CompositeSequenceableLoader>;
						public loaders: androidNative.Array<com.google.android.exoplayer2.source.SequenceableLoader>;
						public isLoading(): boolean;
						public constructor(param0: androidNative.Array<com.google.android.exoplayer2.source.SequenceableLoader>);
						public getNextLoadPositionUs(): number;
						public getBufferedPositionUs(): number;
						public reevaluateBuffer(param0: number): void;
						public continueLoading(param0: number): boolean;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class CompositeSequenceableLoaderFactory {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.CompositeSequenceableLoaderFactory>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.source.CompositeSequenceableLoaderFactory interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							createCompositeSequenceableLoader(param0: androidNative.Array<com.google.android.exoplayer2.source.SequenceableLoader>): com.google.android.exoplayer2.source.SequenceableLoader;
						});
						public constructor();
						public createCompositeSequenceableLoader(param0: androidNative.Array<com.google.android.exoplayer2.source.SequenceableLoader>): com.google.android.exoplayer2.source.SequenceableLoader;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class ConcatenatingMediaSource extends com.google.android.exoplayer2.source.CompositeMediaSource<com.google.android.exoplayer2.source.ConcatenatingMediaSource.MediaSourceHolder> {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.ConcatenatingMediaSource>;
						public constructor(param0: boolean, param1: androidNative.Array<com.google.android.exoplayer2.source.MediaSource>);
						public disableInternal(): void;
						public removeMediaSourceRange(param0: number, param1: number): void;
						public createPeriod(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param1: com.google.android.exoplayer2.upstream.Allocator, param2: number): com.google.android.exoplayer2.source.MediaPeriod;
						public removeDrmEventListener(param0: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
						public onChildSourceInfoRefreshed(param0: any, param1: com.google.android.exoplayer2.source.MediaSource, param2: com.google.android.exoplayer2.Timeline): void;
						public constructor(param0: androidNative.Array<com.google.android.exoplayer2.source.MediaSource>);
						public addMediaSources(param0: number, param1: java.util.Collection<com.google.android.exoplayer2.source.MediaSource>): void;
						public onChildSourceInfoRefreshed(param0: com.google.android.exoplayer2.source.ConcatenatingMediaSource.MediaSourceHolder, param1: com.google.android.exoplayer2.source.MediaSource, param2: com.google.android.exoplayer2.Timeline): void;
						public getWindowIndexForChildWindowIndex(param0: com.google.android.exoplayer2.source.ConcatenatingMediaSource.MediaSourceHolder, param1: number): number;
						public addMediaSource(param0: com.google.android.exoplayer2.source.MediaSource, param1: globalAndroid.os.Handler, param2: java.lang.Runnable): void;
						public enable(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public constructor();
						public isSingleWindow(): boolean;
						/** @deprecated */
						public prepareSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param1: com.google.android.exoplayer2.upstream.TransferListener): void;
						public removeMediaSource(param0: number, param1: globalAndroid.os.Handler, param2: java.lang.Runnable): com.google.android.exoplayer2.source.MediaSource;
						public prepareSourceInternal(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
						public constructor(param0: boolean, param1: boolean, param2: com.google.android.exoplayer2.source.ShuffleOrder, param3: androidNative.Array<com.google.android.exoplayer2.source.MediaSource>);
						public addDrmEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
						public getWindowIndexForChildWindowIndex(param0: any, param1: number): number;
						public getSize(): number;
						public getMediaSource(param0: number): com.google.android.exoplayer2.source.MediaSource;
						public releasePeriod(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
						public clear(): void;
						public getMediaPeriodIdForChildMediaPeriodId(param0: any, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): com.google.android.exoplayer2.source.MediaSource.MediaPeriodId;
						public removeEventListener(param0: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
						public addMediaSource(param0: number, param1: com.google.android.exoplayer2.source.MediaSource, param2: globalAndroid.os.Handler, param3: java.lang.Runnable): void;
						public enableInternal(): void;
						public moveMediaSource(param0: number, param1: number, param2: globalAndroid.os.Handler, param3: java.lang.Runnable): void;
						public setShuffleOrder(param0: com.google.android.exoplayer2.source.ShuffleOrder): void;
						public getInitialTimeline(): com.google.android.exoplayer2.Timeline;
						public getMediaPeriodIdForChildMediaPeriodId(param0: com.google.android.exoplayer2.source.ConcatenatingMediaSource.MediaSourceHolder, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): com.google.android.exoplayer2.source.MediaSource.MediaPeriodId;
						public addMediaSources(param0: java.util.Collection<com.google.android.exoplayer2.source.MediaSource>, param1: globalAndroid.os.Handler, param2: java.lang.Runnable): void;
						public moveMediaSource(param0: number, param1: number): void;
						public addMediaSources(param0: java.util.Collection<com.google.android.exoplayer2.source.MediaSource>): void;
						public constructor(param0: boolean, param1: com.google.android.exoplayer2.source.ShuffleOrder, param2: androidNative.Array<com.google.android.exoplayer2.source.MediaSource>);
						public addMediaSources(param0: number, param1: java.util.Collection<com.google.android.exoplayer2.source.MediaSource>, param2: globalAndroid.os.Handler, param3: java.lang.Runnable): void;
						public getMediaItem(): com.google.android.exoplayer2.MediaItem;
						public addMediaSource(param0: com.google.android.exoplayer2.source.MediaSource): void;
						public maybeThrowSourceInfoRefreshError(): void;
						public clear(param0: globalAndroid.os.Handler, param1: java.lang.Runnable): void;
						public addEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
						public releaseSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public releaseSourceInternal(): void;
						public disable(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public removeMediaSource(param0: number): com.google.android.exoplayer2.source.MediaSource;
						public prepareSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param1: com.google.android.exoplayer2.upstream.TransferListener, param2: com.google.android.exoplayer2.analytics.PlayerId): void;
						public addMediaSource(param0: number, param1: com.google.android.exoplayer2.source.MediaSource): void;
						public setShuffleOrder(param0: com.google.android.exoplayer2.source.ShuffleOrder, param1: globalAndroid.os.Handler, param2: java.lang.Runnable): void;
						public removeMediaSourceRange(param0: number, param1: number, param2: globalAndroid.os.Handler, param3: java.lang.Runnable): void;
					}
					export module ConcatenatingMediaSource {
						export class ConcatenatedTimeline extends com.google.android.exoplayer2.AbstractConcatenatedTimeline {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.ConcatenatingMediaSource.ConcatenatedTimeline>;
							public getWindowCount(): number;
							public constructor(param0: java.util.Collection<com.google.android.exoplayer2.source.ConcatenatingMediaSource.MediaSourceHolder>, param1: com.google.android.exoplayer2.source.ShuffleOrder, param2: boolean);
							public getChildIndexByPeriodIndex(param0: number): number;
							public getChildIndexByWindowIndex(param0: number): number;
							public getChildUidByChildIndex(param0: number): any;
							public getChildIndexByChildUid(param0: any): number;
							public getTimelineByChildIndex(param0: number): com.google.android.exoplayer2.Timeline;
							public getFirstPeriodIndexByChildIndex(param0: number): number;
							public getFirstWindowIndexByChildIndex(param0: number): number;
							public constructor(param0: boolean, param1: com.google.android.exoplayer2.source.ShuffleOrder);
							public getPeriodCount(): number;
						}
						export class FakeMediaSource extends com.google.android.exoplayer2.source.BaseMediaSource {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.ConcatenatingMediaSource.FakeMediaSource>;
							public getMediaItem(): com.google.android.exoplayer2.MediaItem;
							public removeDrmEventListener(param0: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
							public releaseSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
							public removeEventListener(param0: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
							public releaseSourceInternal(): void;
							public maybeThrowSourceInfoRefreshError(): void;
							public addEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
							public disable(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
							public isSingleWindow(): boolean;
							public enable(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
							public prepareSourceInternal(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
							public getInitialTimeline(): com.google.android.exoplayer2.Timeline;
							public releasePeriod(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
							public createPeriod(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param1: com.google.android.exoplayer2.upstream.Allocator, param2: number): com.google.android.exoplayer2.source.MediaPeriod;
							public prepareSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param1: com.google.android.exoplayer2.upstream.TransferListener, param2: com.google.android.exoplayer2.analytics.PlayerId): void;
							public addDrmEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
							/** @deprecated */
							public prepareSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param1: com.google.android.exoplayer2.upstream.TransferListener): void;
						}
						export class HandlerAndRunnable {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.ConcatenatingMediaSource.HandlerAndRunnable>;
							public dispatch(): void;
							public constructor(param0: globalAndroid.os.Handler, param1: java.lang.Runnable);
						}
						export class MediaSourceHolder {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.ConcatenatingMediaSource.MediaSourceHolder>;
							public mediaSource: com.google.android.exoplayer2.source.MaskingMediaSource;
							public uid: any;
							public activeMediaPeriodIds: java.util.List<com.google.android.exoplayer2.source.MediaSource.MediaPeriodId>;
							public childIndex: number;
							public firstWindowIndexInChild: number;
							public isRemoved: boolean;
							public constructor(param0: com.google.android.exoplayer2.source.MediaSource, param1: boolean);
							public reset(param0: number, param1: number): void;
						}
						export class MessageData<T>  extends java.lang.Object {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.ConcatenatingMediaSource.MessageData<any>>;
							public index: number;
							public customData: T;
							public onCompletionAction: com.google.android.exoplayer2.source.ConcatenatingMediaSource.HandlerAndRunnable;
							public constructor(param0: number, param1: T, param2: com.google.android.exoplayer2.source.ConcatenatingMediaSource.HandlerAndRunnable);
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class DefaultCompositeSequenceableLoaderFactory extends com.google.android.exoplayer2.source.CompositeSequenceableLoaderFactory {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.DefaultCompositeSequenceableLoaderFactory>;
						public constructor();
						public createCompositeSequenceableLoader(param0: androidNative.Array<com.google.android.exoplayer2.source.SequenceableLoader>): com.google.android.exoplayer2.source.SequenceableLoader;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class DefaultMediaSourceFactory extends com.google.android.exoplayer2.source.MediaSourceFactory {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.DefaultMediaSourceFactory>;
						public setLiveMinOffsetMs(param0: number): com.google.android.exoplayer2.source.DefaultMediaSourceFactory;
						public constructor(param0: globalAndroid.content.Context);
						public getSupportedTypes(): androidNative.Array<number>;
						public constructor(param0: com.google.android.exoplayer2.upstream.DataSource.Factory, param1: com.google.android.exoplayer2.extractor.ExtractorsFactory);
						public setLiveTargetOffsetMs(param0: number): com.google.android.exoplayer2.source.DefaultMediaSourceFactory;
						public createMediaSource(param0: com.google.android.exoplayer2.MediaItem): com.google.android.exoplayer2.source.MediaSource;
						public setLoadErrorHandlingPolicy(param0: com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy): com.google.android.exoplayer2.source.DefaultMediaSourceFactory;
						public setLiveMinSpeed(param0: number): com.google.android.exoplayer2.source.DefaultMediaSourceFactory;
						public setLoadErrorHandlingPolicy(param0: com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy): com.google.android.exoplayer2.source.MediaSource.Factory;
						public setLiveMaxOffsetMs(param0: number): com.google.android.exoplayer2.source.DefaultMediaSourceFactory;
						public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.extractor.ExtractorsFactory);
						public setLiveMaxSpeed(param0: number): com.google.android.exoplayer2.source.DefaultMediaSourceFactory;
						public setDrmSessionManagerProvider(param0: com.google.android.exoplayer2.drm.DrmSessionManagerProvider): com.google.android.exoplayer2.source.DefaultMediaSourceFactory;
						public setDrmSessionManagerProvider(param0: com.google.android.exoplayer2.drm.DrmSessionManagerProvider): com.google.android.exoplayer2.source.MediaSource.Factory;
						public constructor(param0: com.google.android.exoplayer2.upstream.DataSource.Factory);
						public setServerSideAdInsertionMediaSourceFactory(param0: com.google.android.exoplayer2.source.MediaSource.Factory): com.google.android.exoplayer2.source.DefaultMediaSourceFactory;
						public experimentalUseProgressiveMediaSourceForSubtitles(param0: boolean): com.google.android.exoplayer2.source.DefaultMediaSourceFactory;
						public setAdViewProvider(param0: com.google.android.exoplayer2.ui.AdViewProvider): com.google.android.exoplayer2.source.DefaultMediaSourceFactory;
						public setAdsLoaderProvider(param0: com.google.android.exoplayer2.source.ads.AdsLoader.Provider): com.google.android.exoplayer2.source.DefaultMediaSourceFactory;
					}
					export module DefaultMediaSourceFactory {
						export class AdsLoaderProvider extends com.google.android.exoplayer2.source.ads.AdsLoader.Provider {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.DefaultMediaSourceFactory.AdsLoaderProvider>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.source.DefaultMediaSourceFactory$AdsLoaderProvider interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								getAdsLoader(param0: com.google.android.exoplayer2.MediaItem.AdsConfiguration): com.google.android.exoplayer2.source.ads.AdsLoader;
							});
							public constructor();
							public getAdsLoader(param0: com.google.android.exoplayer2.MediaItem.AdsConfiguration): com.google.android.exoplayer2.source.ads.AdsLoader;
						}
						export class DelegateFactoryLoader {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.DefaultMediaSourceFactory.DelegateFactoryLoader>;
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSource.Factory, param1: com.google.android.exoplayer2.extractor.ExtractorsFactory);
							public setLoadErrorHandlingPolicy(param0: com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy): void;
							public getSupportedTypes(): androidNative.Array<number>;
							public setDrmSessionManagerProvider(param0: com.google.android.exoplayer2.drm.DrmSessionManagerProvider): void;
							public getMediaSourceFactory(param0: number): com.google.android.exoplayer2.source.MediaSource.Factory;
						}
						export class UnknownSubtitlesExtractor {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.DefaultMediaSourceFactory.UnknownSubtitlesExtractor>;
							public init(param0: com.google.android.exoplayer2.extractor.ExtractorOutput): void;
							public sniff(param0: com.google.android.exoplayer2.extractor.ExtractorInput): boolean;
							public seek(param0: number, param1: number): void;
							public constructor(param0: com.google.android.exoplayer2.Format);
							public release(): void;
							public read(param0: com.google.android.exoplayer2.extractor.ExtractorInput, param1: com.google.android.exoplayer2.extractor.PositionHolder): number;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class EmptySampleStream extends com.google.android.exoplayer2.source.SampleStream {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.EmptySampleStream>;
						public constructor();
						public readData(param0: com.google.android.exoplayer2.FormatHolder, param1: com.google.android.exoplayer2.decoder.DecoderInputBuffer, param2: number): number;
						public isReady(): boolean;
						public maybeThrowError(): void;
						public skipData(param0: number): number;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export abstract class ForwardingTimeline {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.ForwardingTimeline>;
						public timeline: com.google.android.exoplayer2.Timeline;
						public getPeriod(param0: number, param1: com.google.android.exoplayer2.Timeline.Period, param2: boolean): com.google.android.exoplayer2.Timeline.Period;
						public getWindowCount(): number;
						public getPreviousWindowIndex(param0: number, param1: number, param2: boolean): number;
						public getWindow(param0: number, param1: com.google.android.exoplayer2.Timeline.Window, param2: number): com.google.android.exoplayer2.Timeline.Window;
						public getIndexOfPeriod(param0: any): number;
						public constructor(param0: com.google.android.exoplayer2.Timeline);
						public getPeriodCount(): number;
						public getNextWindowIndex(param0: number, param1: number, param2: boolean): number;
						public getLastWindowIndex(param0: boolean): number;
						public getFirstWindowIndex(param0: boolean): number;
						public getUidOfPeriod(param0: number): any;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class IcyDataSource extends com.google.android.exoplayer2.upstream.DataSource {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.IcyDataSource>;
						public getResponseHeaders(): java.util.Map<string,java.util.List<string>>;
						public open(param0: com.google.android.exoplayer2.upstream.DataSpec): number;
						public constructor(param0: com.google.android.exoplayer2.upstream.DataSource, param1: number, param2: com.google.android.exoplayer2.source.IcyDataSource.Listener);
						public close(): void;
						public read(param0: androidNative.Array<number>, param1: number, param2: number): number;
						public addTransferListener(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
						public getUri(): globalAndroid.net.Uri;
					}
					export module IcyDataSource {
						export class Listener {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.IcyDataSource.Listener>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.source.IcyDataSource$Listener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onIcyMetadata(param0: com.google.android.exoplayer2.util.ParsableByteArray): void;
							});
							public constructor();
							public onIcyMetadata(param0: com.google.android.exoplayer2.util.ParsableByteArray): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class LoadEventInfo {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.LoadEventInfo>;
						public loadTaskId: number;
						public dataSpec: com.google.android.exoplayer2.upstream.DataSpec;
						public uri: globalAndroid.net.Uri;
						public responseHeaders: java.util.Map<string,java.util.List<string>>;
						public elapsedRealtimeMs: number;
						public loadDurationMs: number;
						public bytesLoaded: number;
						public constructor(param0: number, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: number);
						public constructor(param0: number, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: globalAndroid.net.Uri, param3: java.util.Map<string,java.util.List<string>>, param4: number, param5: number, param6: number);
						public static getNewId(): number;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class LoopingMediaSource extends com.google.android.exoplayer2.source.CompositeMediaSource<java.lang.Void> {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.LoopingMediaSource>;
						public enable(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public constructor();
						public isSingleWindow(): boolean;
						/** @deprecated */
						public prepareSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param1: com.google.android.exoplayer2.upstream.TransferListener): void;
						public prepareSourceInternal(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
						public createPeriod(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param1: com.google.android.exoplayer2.upstream.Allocator, param2: number): com.google.android.exoplayer2.source.MediaPeriod;
						public maybeThrowSourceInfoRefreshError(): void;
						public addEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
						public getMediaPeriodIdForChildMediaPeriodId(param0: java.lang.Void, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): com.google.android.exoplayer2.source.MediaSource.MediaPeriodId;
						public getInitialTimeline(): com.google.android.exoplayer2.Timeline;
						public constructor(param0: com.google.android.exoplayer2.source.MediaSource, param1: number);
						public addDrmEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
						public releaseSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public removeDrmEventListener(param0: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
						public onChildSourceInfoRefreshed(param0: any, param1: com.google.android.exoplayer2.source.MediaSource, param2: com.google.android.exoplayer2.Timeline): void;
						public disable(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public constructor(param0: com.google.android.exoplayer2.source.MediaSource);
						public releasePeriod(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
						public prepareSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param1: com.google.android.exoplayer2.upstream.TransferListener, param2: com.google.android.exoplayer2.analytics.PlayerId): void;
						public getMediaPeriodIdForChildMediaPeriodId(param0: any, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): com.google.android.exoplayer2.source.MediaSource.MediaPeriodId;
						public removeEventListener(param0: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
						public getMediaItem(): com.google.android.exoplayer2.MediaItem;
						public onChildSourceInfoRefreshed(param0: java.lang.Void, param1: com.google.android.exoplayer2.source.MediaSource, param2: com.google.android.exoplayer2.Timeline): void;
					}
					export module LoopingMediaSource {
						export class InfinitelyLoopingTimeline extends com.google.android.exoplayer2.source.ForwardingTimeline {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.LoopingMediaSource.InfinitelyLoopingTimeline>;
							public getNextWindowIndex(param0: number, param1: number, param2: boolean): number;
							public getPreviousWindowIndex(param0: number, param1: number, param2: boolean): number;
							public constructor(param0: com.google.android.exoplayer2.Timeline);
						}
						export class LoopingTimeline extends com.google.android.exoplayer2.AbstractConcatenatedTimeline {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.LoopingMediaSource.LoopingTimeline>;
							public getWindowCount(): number;
							public getChildIndexByPeriodIndex(param0: number): number;
							public getChildIndexByWindowIndex(param0: number): number;
							public getChildUidByChildIndex(param0: number): any;
							public getChildIndexByChildUid(param0: any): number;
							public getTimelineByChildIndex(param0: number): com.google.android.exoplayer2.Timeline;
							public getFirstPeriodIndexByChildIndex(param0: number): number;
							public constructor(param0: com.google.android.exoplayer2.Timeline, param1: number);
							public getFirstWindowIndexByChildIndex(param0: number): number;
							public constructor(param0: boolean, param1: com.google.android.exoplayer2.source.ShuffleOrder);
							public getPeriodCount(): number;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class MaskingMediaPeriod implements com.google.android.exoplayer2.source.MediaPeriod, com.google.android.exoplayer2.source.MediaPeriod.Callback {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.MaskingMediaPeriod>;
						public id: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId;
						public selectTracks(param0: androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection>, param1: androidNative.Array<boolean>, param2: androidNative.Array<com.google.android.exoplayer2.source.SampleStream>, param3: androidNative.Array<boolean>, param4: number): number;
						public isLoading(): boolean;
						public getPreparePositionUs(): number;
						public setMediaSource(param0: com.google.android.exoplayer2.source.MediaSource): void;
						public onPrepared(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
						public overridePreparePositionUs(param0: number): void;
						public prepare(param0: com.google.android.exoplayer2.source.MediaPeriod.Callback, param1: number): void;
						public getAdjustedSeekPositionUs(param0: number, param1: com.google.android.exoplayer2.SeekParameters): number;
						public getNextLoadPositionUs(): number;
						public releasePeriod(): void;
						public getStreamKeys(param0: java.util.List<com.google.android.exoplayer2.trackselection.ExoTrackSelection>): java.util.List<com.google.android.exoplayer2.offline.StreamKey>;
						public reevaluateBuffer(param0: number): void;
						public getTrackGroups(): com.google.android.exoplayer2.source.TrackGroupArray;
						public constructor(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param1: com.google.android.exoplayer2.upstream.Allocator, param2: number);
						public setPrepareListener(param0: com.google.android.exoplayer2.source.MaskingMediaPeriod.PrepareListener): void;
						public readDiscontinuity(): number;
						public continueLoading(param0: number): boolean;
						public getPreparePositionOverrideUs(): number;
						public discardBuffer(param0: number, param1: boolean): void;
						public createPeriod(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
						public seekToUs(param0: number): number;
						public onContinueLoadingRequested(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
						public getBufferedPositionUs(): number;
						public onContinueLoadingRequested(param0: any): void;
						public maybeThrowPrepareError(): void;
					}
					export module MaskingMediaPeriod {
						export class PrepareListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.MaskingMediaPeriod.PrepareListener>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.source.MaskingMediaPeriod$PrepareListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onPrepareComplete(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
								onPrepareError(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param1: java.io.IOException): void;
							});
							public constructor();
							public onPrepareComplete(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
							public onPrepareError(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param1: java.io.IOException): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class MaskingMediaSource extends com.google.android.exoplayer2.source.CompositeMediaSource<java.lang.Void> {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.MaskingMediaSource>;
						public constructor(param0: com.google.android.exoplayer2.source.MediaSource, param1: boolean);
						public createPeriod(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param1: com.google.android.exoplayer2.upstream.Allocator, param2: number): com.google.android.exoplayer2.source.MediaPeriod;
						public getTimeline(): com.google.android.exoplayer2.Timeline;
						public getInitialTimeline(): com.google.android.exoplayer2.Timeline;
						public removeDrmEventListener(param0: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
						public onChildSourceInfoRefreshed(param0: any, param1: com.google.android.exoplayer2.source.MediaSource, param2: com.google.android.exoplayer2.Timeline): void;
						public getMediaItem(): com.google.android.exoplayer2.MediaItem;
						public enable(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public constructor();
						public isSingleWindow(): boolean;
						/** @deprecated */
						public prepareSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param1: com.google.android.exoplayer2.upstream.TransferListener): void;
						public prepareSourceInternal(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
						public maybeThrowSourceInfoRefreshError(): void;
						public addEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
						public getMediaPeriodIdForChildMediaPeriodId(param0: java.lang.Void, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): com.google.android.exoplayer2.source.MediaSource.MediaPeriodId;
						public addDrmEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
						public releaseSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public releaseSourceInternal(): void;
						public disable(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public releasePeriod(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
						public prepareSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param1: com.google.android.exoplayer2.upstream.TransferListener, param2: com.google.android.exoplayer2.analytics.PlayerId): void;
						public getMediaPeriodIdForChildMediaPeriodId(param0: any, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): com.google.android.exoplayer2.source.MediaSource.MediaPeriodId;
						public removeEventListener(param0: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
						public createPeriod(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param1: com.google.android.exoplayer2.upstream.Allocator, param2: number): com.google.android.exoplayer2.source.MaskingMediaPeriod;
						public onChildSourceInfoRefreshed(param0: java.lang.Void, param1: com.google.android.exoplayer2.source.MediaSource, param2: com.google.android.exoplayer2.Timeline): void;
					}
					export module MaskingMediaSource {
						export class MaskingTimeline extends com.google.android.exoplayer2.source.ForwardingTimeline {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.MaskingMediaSource.MaskingTimeline>;
							public static MASKING_EXTERNAL_PERIOD_UID: any;
							public static createWithPlaceholderTimeline(param0: com.google.android.exoplayer2.MediaItem): com.google.android.exoplayer2.source.MaskingMediaSource.MaskingTimeline;
							public getIndexOfPeriod(param0: any): number;
							public cloneWithUpdatedTimeline(param0: com.google.android.exoplayer2.Timeline): com.google.android.exoplayer2.source.MaskingMediaSource.MaskingTimeline;
							public static createWithRealTimeline(param0: com.google.android.exoplayer2.Timeline, param1: any, param2: any): com.google.android.exoplayer2.source.MaskingMediaSource.MaskingTimeline;
							public getWindow(param0: number, param1: com.google.android.exoplayer2.Timeline.Window, param2: number): com.google.android.exoplayer2.Timeline.Window;
							public getPeriod(param0: number, param1: com.google.android.exoplayer2.Timeline.Period, param2: boolean): com.google.android.exoplayer2.Timeline.Period;
							public getUidOfPeriod(param0: number): any;
							public getTimeline(): com.google.android.exoplayer2.Timeline;
						}
						export class PlaceholderTimeline {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.MaskingMediaSource.PlaceholderTimeline>;
							public getWindowCount(): number;
							public getIndexOfPeriod(param0: any): number;
							public getWindow(param0: number, param1: com.google.android.exoplayer2.Timeline.Window, param2: number): com.google.android.exoplayer2.Timeline.Window;
							public getPeriod(param0: number, param1: com.google.android.exoplayer2.Timeline.Period, param2: boolean): com.google.android.exoplayer2.Timeline.Period;
							public constructor(param0: com.google.android.exoplayer2.MediaItem);
							public getPeriodCount(): number;
							public getUidOfPeriod(param0: number): any;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class MediaLoadData {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.MediaLoadData>;
						public dataType: number;
						public trackType: number;
						public trackFormat: com.google.android.exoplayer2.Format;
						public trackSelectionReason: number;
						public trackSelectionData: any;
						public mediaStartTimeMs: number;
						public mediaEndTimeMs: number;
						public constructor(param0: number, param1: number, param2: com.google.android.exoplayer2.Format, param3: number, param4: any, param5: number, param6: number);
						public constructor(param0: number);
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class MediaParserExtractorAdapter extends com.google.android.exoplayer2.source.ProgressiveMediaExtractor {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.MediaParserExtractorAdapter>;
						public static FACTORY: com.google.android.exoplayer2.source.ProgressiveMediaExtractor.Factory;
						public disableSeekingOnMp3Streams(): void;
						public getCurrentInputPosition(): number;
						public init(param0: com.google.android.exoplayer2.upstream.DataReader, param1: globalAndroid.net.Uri, param2: java.util.Map<string,java.util.List<string>>, param3: number, param4: number, param5: com.google.android.exoplayer2.extractor.ExtractorOutput): void;
						public constructor(param0: com.google.android.exoplayer2.analytics.PlayerId);
						public seek(param0: number, param1: number): void;
						public release(): void;
						public read(param0: com.google.android.exoplayer2.extractor.PositionHolder): number;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class MediaPeriod extends com.google.android.exoplayer2.source.SequenceableLoader {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.MediaPeriod>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.source.MediaPeriod interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							prepare(param0: com.google.android.exoplayer2.source.MediaPeriod.Callback, param1: number): void;
							maybeThrowPrepareError(): void;
							getTrackGroups(): com.google.android.exoplayer2.source.TrackGroupArray;
							getStreamKeys(param0: java.util.List<com.google.android.exoplayer2.trackselection.ExoTrackSelection>): java.util.List<com.google.android.exoplayer2.offline.StreamKey>;
							selectTracks(param0: androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection>, param1: androidNative.Array<boolean>, param2: androidNative.Array<com.google.android.exoplayer2.source.SampleStream>, param3: androidNative.Array<boolean>, param4: number): number;
							discardBuffer(param0: number, param1: boolean): void;
							readDiscontinuity(): number;
							seekToUs(param0: number): number;
							getAdjustedSeekPositionUs(param0: number, param1: com.google.android.exoplayer2.SeekParameters): number;
							getBufferedPositionUs(): number;
							getNextLoadPositionUs(): number;
							continueLoading(param0: number): boolean;
							isLoading(): boolean;
							reevaluateBuffer(param0: number): void;
							getBufferedPositionUs(): number;
							getNextLoadPositionUs(): number;
							continueLoading(param0: number): boolean;
							isLoading(): boolean;
							reevaluateBuffer(param0: number): void;
						});
						public constructor();
						public selectTracks(param0: androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection>, param1: androidNative.Array<boolean>, param2: androidNative.Array<com.google.android.exoplayer2.source.SampleStream>, param3: androidNative.Array<boolean>, param4: number): number;
						public readDiscontinuity(): number;
						public isLoading(): boolean;
						public continueLoading(param0: number): boolean;
						public discardBuffer(param0: number, param1: boolean): void;
						public prepare(param0: com.google.android.exoplayer2.source.MediaPeriod.Callback, param1: number): void;
						public getAdjustedSeekPositionUs(param0: number, param1: com.google.android.exoplayer2.SeekParameters): number;
						public getNextLoadPositionUs(): number;
						public seekToUs(param0: number): number;
						public getStreamKeys(param0: java.util.List<com.google.android.exoplayer2.trackselection.ExoTrackSelection>): java.util.List<com.google.android.exoplayer2.offline.StreamKey>;
						public getBufferedPositionUs(): number;
						public reevaluateBuffer(param0: number): void;
						public getTrackGroups(): com.google.android.exoplayer2.source.TrackGroupArray;
						public maybeThrowPrepareError(): void;
					}
					export module MediaPeriod {
						export class Callback extends com.google.android.exoplayer2.source.SequenceableLoader.Callback<com.google.android.exoplayer2.source.MediaPeriod> {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.MediaPeriod.Callback>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.source.MediaPeriod$Callback interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onPrepared(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
								onContinueLoadingRequested(param0: any): void;
							});
							public constructor();
							public onContinueLoadingRequested(param0: any): void;
							public onPrepared(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class MediaSource {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.MediaSource>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.source.MediaSource interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							addEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
							removeEventListener(param0: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
							addDrmEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
							removeDrmEventListener(param0: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
							getInitialTimeline(): com.google.android.exoplayer2.Timeline;
							isSingleWindow(): boolean;
							getMediaItem(): com.google.android.exoplayer2.MediaItem;
							prepareSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param1: com.google.android.exoplayer2.upstream.TransferListener): void;
							prepareSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param1: com.google.android.exoplayer2.upstream.TransferListener, param2: com.google.android.exoplayer2.analytics.PlayerId): void;
							maybeThrowSourceInfoRefreshError(): void;
							enable(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
							createPeriod(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param1: com.google.android.exoplayer2.upstream.Allocator, param2: number): com.google.android.exoplayer2.source.MediaPeriod;
							releasePeriod(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
							disable(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
							releaseSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						});
						public constructor();
						public enable(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public isSingleWindow(): boolean;
						/** @deprecated */
						public prepareSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param1: com.google.android.exoplayer2.upstream.TransferListener): void;
						public maybeThrowSourceInfoRefreshError(): void;
						public createPeriod(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param1: com.google.android.exoplayer2.upstream.Allocator, param2: number): com.google.android.exoplayer2.source.MediaPeriod;
						public addEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
						public getInitialTimeline(): com.google.android.exoplayer2.Timeline;
						public addDrmEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
						public releaseSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public removeDrmEventListener(param0: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
						public disable(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public releasePeriod(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
						public prepareSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param1: com.google.android.exoplayer2.upstream.TransferListener, param2: com.google.android.exoplayer2.analytics.PlayerId): void;
						public removeEventListener(param0: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
						public getMediaItem(): com.google.android.exoplayer2.MediaItem;
					}
					export module MediaSource {
						export class Factory {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.MediaSource.Factory>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.source.MediaSource$Factory interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								setDrmSessionManagerProvider(param0: com.google.android.exoplayer2.drm.DrmSessionManagerProvider): com.google.android.exoplayer2.source.MediaSource.Factory;
								setLoadErrorHandlingPolicy(param0: com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy): com.google.android.exoplayer2.source.MediaSource.Factory;
								getSupportedTypes(): androidNative.Array<number>;
								createMediaSource(param0: com.google.android.exoplayer2.MediaItem): com.google.android.exoplayer2.source.MediaSource;
								<clinit>(): void;
							});
							public constructor();
							public static UNSUPPORTED: com.google.android.exoplayer2.source.MediaSource.Factory;
							public setLoadErrorHandlingPolicy(param0: com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy): com.google.android.exoplayer2.source.MediaSource.Factory;
							public createMediaSource(param0: com.google.android.exoplayer2.MediaItem): com.google.android.exoplayer2.source.MediaSource;
							public getSupportedTypes(): androidNative.Array<number>;
							public setDrmSessionManagerProvider(param0: com.google.android.exoplayer2.drm.DrmSessionManagerProvider): com.google.android.exoplayer2.source.MediaSource.Factory;
						}
						export class MediaPeriodId {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.MediaSource.MediaPeriodId>;
							public copyWithPeriodUid(param0: any): com.google.android.exoplayer2.source.MediaSource.MediaPeriodId;
							public constructor(param0: com.google.android.exoplayer2.source.MediaPeriodId);
							public constructor(param0: any, param1: number, param2: number, param3: number);
							public constructor(param0: any);
							public constructor(param0: any, param1: number, param2: number);
							public copyWithWindowSequenceNumber(param0: number): com.google.android.exoplayer2.source.MediaSource.MediaPeriodId;
							public constructor(param0: any, param1: number);
						}
						export class MediaSourceCaller {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.source.MediaSource$MediaSourceCaller interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onSourceInfoRefreshed(param0: com.google.android.exoplayer2.source.MediaSource, param1: com.google.android.exoplayer2.Timeline): void;
							});
							public constructor();
							public onSourceInfoRefreshed(param0: com.google.android.exoplayer2.source.MediaSource, param1: com.google.android.exoplayer2.Timeline): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class MediaSourceEventListener {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.MediaSourceEventListener>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.source.MediaSourceEventListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							onLoadStarted(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData): void;
							onLoadCompleted(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData): void;
							onLoadCanceled(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData): void;
							onLoadError(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData, param4: java.io.IOException, param5: boolean): void;
							onUpstreamDiscarded(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
							onDownstreamFormatChanged(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
						});
						public constructor();
						public onDownstreamFormatChanged(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
						public onLoadStarted(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData): void;
						public onUpstreamDiscarded(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
						public onLoadError(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData, param4: java.io.IOException, param5: boolean): void;
						public onLoadCanceled(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData): void;
						public onLoadCompleted(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData): void;
					}
					export module MediaSourceEventListener {
						export class EventDispatcher {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.MediaSourceEventListener.EventDispatcher>;
							public windowIndex: number;
							public mediaPeriodId: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId;
							public constructor();
							public loadStarted(param0: com.google.android.exoplayer2.source.LoadEventInfo, param1: number): void;
							public loadCompleted(param0: com.google.android.exoplayer2.source.LoadEventInfo, param1: com.google.android.exoplayer2.source.MediaLoadData): void;
							public downstreamFormatChanged(param0: number, param1: com.google.android.exoplayer2.Format, param2: number, param3: any, param4: number): void;
							public withParameters(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: number): com.google.android.exoplayer2.source.MediaSourceEventListener.EventDispatcher;
							public removeEventListener(param0: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
							public downstreamFormatChanged(param0: com.google.android.exoplayer2.source.MediaLoadData): void;
							public loadStarted(param0: com.google.android.exoplayer2.source.LoadEventInfo, param1: number, param2: number, param3: com.google.android.exoplayer2.Format, param4: number, param5: any, param6: number, param7: number): void;
							public addEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
							public loadError(param0: com.google.android.exoplayer2.source.LoadEventInfo, param1: com.google.android.exoplayer2.source.MediaLoadData, param2: java.io.IOException, param3: boolean): void;
							public upstreamDiscarded(param0: com.google.android.exoplayer2.source.MediaLoadData): void;
							public upstreamDiscarded(param0: number, param1: number, param2: number): void;
							public loadCanceled(param0: com.google.android.exoplayer2.source.LoadEventInfo, param1: number, param2: number, param3: com.google.android.exoplayer2.Format, param4: number, param5: any, param6: number, param7: number): void;
							public loadError(param0: com.google.android.exoplayer2.source.LoadEventInfo, param1: number, param2: number, param3: com.google.android.exoplayer2.Format, param4: number, param5: any, param6: number, param7: number, param8: java.io.IOException, param9: boolean): void;
							public loadStarted(param0: com.google.android.exoplayer2.source.LoadEventInfo, param1: com.google.android.exoplayer2.source.MediaLoadData): void;
							public loadCanceled(param0: com.google.android.exoplayer2.source.LoadEventInfo, param1: number): void;
							public loadCanceled(param0: com.google.android.exoplayer2.source.LoadEventInfo, param1: com.google.android.exoplayer2.source.MediaLoadData): void;
							public loadCompleted(param0: com.google.android.exoplayer2.source.LoadEventInfo, param1: number, param2: number, param3: com.google.android.exoplayer2.Format, param4: number, param5: any, param6: number, param7: number): void;
							public loadError(param0: com.google.android.exoplayer2.source.LoadEventInfo, param1: number, param2: java.io.IOException, param3: boolean): void;
							public loadCompleted(param0: com.google.android.exoplayer2.source.LoadEventInfo, param1: number): void;
						}
						export module EventDispatcher {
							export class ListenerAndHandler {
								public static class: java.lang.Class<com.google.android.exoplayer2.source.MediaSourceEventListener.EventDispatcher.ListenerAndHandler>;
								public handler: globalAndroid.os.Handler;
								public listener: com.google.android.exoplayer2.source.MediaSourceEventListener;
								public constructor(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.source.MediaSourceEventListener);
							}
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class MediaSourceFactory extends com.google.android.exoplayer2.source.MediaSource.Factory {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.MediaSourceFactory>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.source.MediaSourceFactory interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							<clinit>(): void;
							setDrmSessionManagerProvider(param0: com.google.android.exoplayer2.drm.DrmSessionManagerProvider): com.google.android.exoplayer2.source.MediaSource.Factory;
							setLoadErrorHandlingPolicy(param0: com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy): com.google.android.exoplayer2.source.MediaSource.Factory;
							getSupportedTypes(): androidNative.Array<number>;
							createMediaSource(param0: com.google.android.exoplayer2.MediaItem): com.google.android.exoplayer2.source.MediaSource;
							<clinit>(): void;
						});
						public constructor();
						public static UNSUPPORTED: com.google.android.exoplayer2.source.MediaSource.Factory;
						public static UNSUPPORTED: com.google.android.exoplayer2.source.MediaSourceFactory;
						public setDrmSessionManagerProvider(param0: com.google.android.exoplayer2.drm.DrmSessionManagerProvider): com.google.android.exoplayer2.source.MediaSource.Factory;
						public getSupportedTypes(): androidNative.Array<number>;
						public createMediaSource(param0: com.google.android.exoplayer2.MediaItem): com.google.android.exoplayer2.source.MediaSource;
						public setLoadErrorHandlingPolicy(param0: com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy): com.google.android.exoplayer2.source.MediaSource.Factory;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class MergingMediaPeriod implements com.google.android.exoplayer2.source.MediaPeriod, com.google.android.exoplayer2.source.MediaPeriod.Callback {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.MergingMediaPeriod>;
						public selectTracks(param0: androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection>, param1: androidNative.Array<boolean>, param2: androidNative.Array<com.google.android.exoplayer2.source.SampleStream>, param3: androidNative.Array<boolean>, param4: number): number;
						public readDiscontinuity(): number;
						public isLoading(): boolean;
						public onPrepared(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
						public continueLoading(param0: number): boolean;
						public discardBuffer(param0: number, param1: boolean): void;
						public getChildPeriod(param0: number): com.google.android.exoplayer2.source.MediaPeriod;
						public prepare(param0: com.google.android.exoplayer2.source.MediaPeriod.Callback, param1: number): void;
						public getAdjustedSeekPositionUs(param0: number, param1: com.google.android.exoplayer2.SeekParameters): number;
						public getNextLoadPositionUs(): number;
						public seekToUs(param0: number): number;
						public onContinueLoadingRequested(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
						public constructor(param0: com.google.android.exoplayer2.source.CompositeSequenceableLoaderFactory, param1: androidNative.Array<number>, param2: androidNative.Array<com.google.android.exoplayer2.source.MediaPeriod>);
						public getBufferedPositionUs(): number;
						public getStreamKeys(param0: java.util.List<com.google.android.exoplayer2.trackselection.ExoTrackSelection>): java.util.List<com.google.android.exoplayer2.offline.StreamKey>;
						public reevaluateBuffer(param0: number): void;
						public onContinueLoadingRequested(param0: any): void;
						public getTrackGroups(): com.google.android.exoplayer2.source.TrackGroupArray;
						public maybeThrowPrepareError(): void;
					}
					export module MergingMediaPeriod {
						export class ForwardingTrackSelection extends com.google.android.exoplayer2.trackselection.ExoTrackSelection {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.MergingMediaPeriod.ForwardingTrackSelection>;
							public indexOf(param0: com.google.android.exoplayer2.Format): number;
							public onDiscontinuity(): void;
							public updateSelectedTrack(param0: number, param1: number, param2: number, param3: java.util.List<any>, param4: androidNative.Array<com.google.android.exoplayer2.source.chunk.MediaChunkIterator>): void;
							public blacklist(param0: number, param1: number): boolean;
							public isBlacklisted(param0: number, param1: number): boolean;
							public getType(): number;
							public onPlaybackSpeed(param0: number): void;
							public getSelectionReason(): number;
							public indexOf(param0: number): number;
							public shouldCancelChunkLoad(param0: number, param1: com.google.android.exoplayer2.source.chunk.Chunk, param2: java.util.List<any>): boolean;
							public getIndexInTrackGroup(param0: number): number;
							public getSelectedIndexInTrackGroup(): number;
							public length(): number;
							public getSelectedFormat(): com.google.android.exoplayer2.Format;
							public getTrackGroup(): com.google.android.exoplayer2.source.TrackGroup;
							public disable(): void;
							public getSelectionData(): any;
							public onPlayWhenReadyChanged(param0: boolean): void;
							public getFormat(param0: number): com.google.android.exoplayer2.Format;
							public enable(): void;
							public getSelectedIndex(): number;
							public constructor(param0: com.google.android.exoplayer2.trackselection.ExoTrackSelection, param1: com.google.android.exoplayer2.source.TrackGroup);
							public onRebuffer(): void;
							public evaluateQueueSize(param0: number, param1: java.util.List<any>): number;
						}
						export class TimeOffsetMediaPeriod implements com.google.android.exoplayer2.source.MediaPeriod, com.google.android.exoplayer2.source.MediaPeriod.Callback {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.MergingMediaPeriod.TimeOffsetMediaPeriod>;
							public getTrackGroups(): com.google.android.exoplayer2.source.TrackGroupArray;
							public onContinueLoadingRequested(param0: any): void;
							public getStreamKeys(param0: java.util.List<com.google.android.exoplayer2.trackselection.ExoTrackSelection>): java.util.List<com.google.android.exoplayer2.offline.StreamKey>;
							public selectTracks(param0: androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection>, param1: androidNative.Array<boolean>, param2: androidNative.Array<com.google.android.exoplayer2.source.SampleStream>, param3: androidNative.Array<boolean>, param4: number): number;
							public isLoading(): boolean;
							public continueLoading(param0: number): boolean;
							public onContinueLoadingRequested(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
							public maybeThrowPrepareError(): void;
							public getBufferedPositionUs(): number;
							public seekToUs(param0: number): number;
							public getNextLoadPositionUs(): number;
							public prepare(param0: com.google.android.exoplayer2.source.MediaPeriod.Callback, param1: number): void;
							public discardBuffer(param0: number, param1: boolean): void;
							public getAdjustedSeekPositionUs(param0: number, param1: com.google.android.exoplayer2.SeekParameters): number;
							public readDiscontinuity(): number;
							public onPrepared(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
							public constructor(param0: com.google.android.exoplayer2.source.MediaPeriod, param1: number);
							public reevaluateBuffer(param0: number): void;
						}
						export class TimeOffsetSampleStream extends com.google.android.exoplayer2.source.SampleStream {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.MergingMediaPeriod.TimeOffsetSampleStream>;
							public constructor(param0: com.google.android.exoplayer2.source.SampleStream, param1: number);
							public isReady(): boolean;
							public skipData(param0: number): number;
							public getChildStream(): com.google.android.exoplayer2.source.SampleStream;
							public maybeThrowError(): void;
							public readData(param0: com.google.android.exoplayer2.FormatHolder, param1: com.google.android.exoplayer2.decoder.DecoderInputBuffer, param2: number): number;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class MergingMediaSource extends com.google.android.exoplayer2.source.CompositeMediaSource<java.lang.Integer> {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.MergingMediaSource>;
						public constructor(param0: boolean, param1: androidNative.Array<com.google.android.exoplayer2.source.MediaSource>);
						public createPeriod(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param1: com.google.android.exoplayer2.upstream.Allocator, param2: number): com.google.android.exoplayer2.source.MediaPeriod;
						public constructor(param0: boolean, param1: boolean, param2: androidNative.Array<com.google.android.exoplayer2.source.MediaSource>);
						public getInitialTimeline(): com.google.android.exoplayer2.Timeline;
						public onChildSourceInfoRefreshed(param0: java.lang.Integer, param1: com.google.android.exoplayer2.source.MediaSource, param2: com.google.android.exoplayer2.Timeline): void;
						public removeDrmEventListener(param0: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
						public onChildSourceInfoRefreshed(param0: any, param1: com.google.android.exoplayer2.source.MediaSource, param2: com.google.android.exoplayer2.Timeline): void;
						public constructor(param0: androidNative.Array<com.google.android.exoplayer2.source.MediaSource>);
						public getMediaItem(): com.google.android.exoplayer2.MediaItem;
						public enable(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public constructor();
						public isSingleWindow(): boolean;
						/** @deprecated */
						public prepareSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param1: com.google.android.exoplayer2.upstream.TransferListener): void;
						public prepareSourceInternal(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
						public maybeThrowSourceInfoRefreshError(): void;
						public addEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
						public addDrmEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
						public releaseSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public releaseSourceInternal(): void;
						public disable(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public getMediaPeriodIdForChildMediaPeriodId(param0: java.lang.Integer, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): com.google.android.exoplayer2.source.MediaSource.MediaPeriodId;
						public releasePeriod(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
						public prepareSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param1: com.google.android.exoplayer2.upstream.TransferListener, param2: com.google.android.exoplayer2.analytics.PlayerId): void;
						public getMediaPeriodIdForChildMediaPeriodId(param0: any, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): com.google.android.exoplayer2.source.MediaSource.MediaPeriodId;
						public removeEventListener(param0: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
						public constructor(param0: boolean, param1: boolean, param2: com.google.android.exoplayer2.source.CompositeSequenceableLoaderFactory, param3: androidNative.Array<com.google.android.exoplayer2.source.MediaSource>);
					}
					export module MergingMediaSource {
						export class ClippedTimeline extends com.google.android.exoplayer2.source.ForwardingTimeline {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.MergingMediaSource.ClippedTimeline>;
							public constructor(param0: com.google.android.exoplayer2.Timeline, param1: java.util.Map<any,java.lang.Long>);
							public getWindow(param0: number, param1: com.google.android.exoplayer2.Timeline.Window, param2: number): com.google.android.exoplayer2.Timeline.Window;
							public getPeriod(param0: number, param1: com.google.android.exoplayer2.Timeline.Period, param2: boolean): com.google.android.exoplayer2.Timeline.Period;
							public constructor(param0: com.google.android.exoplayer2.Timeline);
						}
						export class IllegalMergeException {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.MergingMediaSource.IllegalMergeException>;
							public static REASON_PERIOD_COUNT_MISMATCH: number;
							public reason: number;
							public constructor(param0: number);
						}
						export module IllegalMergeException {
							export class Reason {
								public static class: java.lang.Class<com.google.android.exoplayer2.source.MergingMediaSource.IllegalMergeException.Reason>;
								/**
								 * Constructs a new instance of the com.google.android.exoplayer2.source.MergingMediaSource$IllegalMergeException$Reason interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
								 */
								public constructor(implementation: {
								});
								public constructor();
							}
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class ProgressiveMediaExtractor {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.ProgressiveMediaExtractor>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.source.ProgressiveMediaExtractor interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							init(param0: com.google.android.exoplayer2.upstream.DataReader, param1: globalAndroid.net.Uri, param2: java.util.Map<string,java.util.List<string>>, param3: number, param4: number, param5: com.google.android.exoplayer2.extractor.ExtractorOutput): void;
							release(): void;
							disableSeekingOnMp3Streams(): void;
							getCurrentInputPosition(): number;
							seek(param0: number, param1: number): void;
							read(param0: com.google.android.exoplayer2.extractor.PositionHolder): number;
						});
						public constructor();
						public disableSeekingOnMp3Streams(): void;
						public getCurrentInputPosition(): number;
						public init(param0: com.google.android.exoplayer2.upstream.DataReader, param1: globalAndroid.net.Uri, param2: java.util.Map<string,java.util.List<string>>, param3: number, param4: number, param5: com.google.android.exoplayer2.extractor.ExtractorOutput): void;
						public seek(param0: number, param1: number): void;
						public release(): void;
						public read(param0: com.google.android.exoplayer2.extractor.PositionHolder): number;
					}
					export module ProgressiveMediaExtractor {
						export class Factory {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.ProgressiveMediaExtractor.Factory>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.source.ProgressiveMediaExtractor$Factory interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								createProgressiveMediaExtractor(param0: com.google.android.exoplayer2.analytics.PlayerId): com.google.android.exoplayer2.source.ProgressiveMediaExtractor;
							});
							public constructor();
							public createProgressiveMediaExtractor(param0: com.google.android.exoplayer2.analytics.PlayerId): com.google.android.exoplayer2.source.ProgressiveMediaExtractor;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class ProgressiveMediaPeriod extends java.lang.Object {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.ProgressiveMediaPeriod>;
						public selectTracks(param0: androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection>, param1: androidNative.Array<boolean>, param2: androidNative.Array<com.google.android.exoplayer2.source.SampleStream>, param3: androidNative.Array<boolean>, param4: number): number;
						public isLoading(): boolean;
						public endTracks(): void;
						public onLoaderReleased(): void;
						public onLoadCompleted(param0: any, param1: number, param2: number): void;
						public onUpstreamFormatChanged(param0: com.google.android.exoplayer2.Format): void;
						public onLoadError(param0: any, param1: number, param2: number, param3: java.io.IOException, param4: number): com.google.android.exoplayer2.upstream.Loader.LoadErrorAction;
						public prepare(param0: com.google.android.exoplayer2.source.MediaPeriod.Callback, param1: number): void;
						public getAdjustedSeekPositionUs(param0: number, param1: com.google.android.exoplayer2.SeekParameters): number;
						public getNextLoadPositionUs(): number;
						public onLoadCompleted(param0: com.google.android.exoplayer2.source.ProgressiveMediaPeriod.ExtractingLoadable, param1: number, param2: number): void;
						public getStreamKeys(param0: java.util.List<com.google.android.exoplayer2.trackselection.ExoTrackSelection>): java.util.List<com.google.android.exoplayer2.offline.StreamKey>;
						public reevaluateBuffer(param0: number): void;
						public getTrackGroups(): com.google.android.exoplayer2.source.TrackGroupArray;
						public onLoadCanceled(param0: com.google.android.exoplayer2.source.ProgressiveMediaPeriod.ExtractingLoadable, param1: number, param2: number, param3: boolean): void;
						public constructor(param0: globalAndroid.net.Uri, param1: com.google.android.exoplayer2.upstream.DataSource, param2: com.google.android.exoplayer2.source.ProgressiveMediaExtractor, param3: com.google.android.exoplayer2.drm.DrmSessionManager, param4: com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher, param5: com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy, param6: com.google.android.exoplayer2.source.MediaSourceEventListener.EventDispatcher, param7: com.google.android.exoplayer2.source.ProgressiveMediaPeriod.Listener, param8: com.google.android.exoplayer2.upstream.Allocator, param9: string, param10: number);
						public readDiscontinuity(): number;
						public track(param0: number, param1: number): com.google.android.exoplayer2.extractor.TrackOutput;
						public seekMap(param0: com.google.android.exoplayer2.extractor.SeekMap): void;
						public release(): void;
						public continueLoading(param0: number): boolean;
						public discardBuffer(param0: number, param1: boolean): void;
						public onLoadError(param0: com.google.android.exoplayer2.source.ProgressiveMediaPeriod.ExtractingLoadable, param1: number, param2: number, param3: java.io.IOException, param4: number): com.google.android.exoplayer2.upstream.Loader.LoadErrorAction;
						public seekToUs(param0: number): number;
						public getBufferedPositionUs(): number;
						public maybeThrowPrepareError(): void;
						public onLoadCanceled(param0: any, param1: number, param2: number, param3: boolean): void;
					}
					export module ProgressiveMediaPeriod {
						export class ExtractingLoadable implements com.google.android.exoplayer2.upstream.Loader.Loadable, com.google.android.exoplayer2.source.IcyDataSource.Listener {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.ProgressiveMediaPeriod.ExtractingLoadable>;
							public constructor(param0: com.google.android.exoplayer2.source.ProgressiveMediaPeriod, param1: globalAndroid.net.Uri, param2: com.google.android.exoplayer2.upstream.DataSource, param3: com.google.android.exoplayer2.source.ProgressiveMediaExtractor, param4: com.google.android.exoplayer2.extractor.ExtractorOutput, param5: com.google.android.exoplayer2.util.ConditionVariable);
							public cancelLoad(): void;
							public onIcyMetadata(param0: com.google.android.exoplayer2.util.ParsableByteArray): void;
							public load(): void;
						}
						export class Listener {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.ProgressiveMediaPeriod.Listener>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.source.ProgressiveMediaPeriod$Listener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onSourceInfoRefreshed(param0: number, param1: boolean, param2: boolean): void;
							});
							public constructor();
							public onSourceInfoRefreshed(param0: number, param1: boolean, param2: boolean): void;
						}
						export class SampleStreamImpl extends com.google.android.exoplayer2.source.SampleStream {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.ProgressiveMediaPeriod.SampleStreamImpl>;
							public isReady(): boolean;
							public skipData(param0: number): number;
							public constructor(param0: com.google.android.exoplayer2.source.ProgressiveMediaPeriod, param1: number);
							public maybeThrowError(): void;
							public readData(param0: com.google.android.exoplayer2.FormatHolder, param1: com.google.android.exoplayer2.decoder.DecoderInputBuffer, param2: number): number;
						}
						export class TrackId {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.ProgressiveMediaPeriod.TrackId>;
							public id: number;
							public isIcyTrack: boolean;
							public constructor(param0: number, param1: boolean);
							public hashCode(): number;
							public equals(param0: any): boolean;
						}
						export class TrackState {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.ProgressiveMediaPeriod.TrackState>;
							public tracks: com.google.android.exoplayer2.source.TrackGroupArray;
							public trackIsAudioVideoFlags: androidNative.Array<boolean>;
							public trackEnabledStates: androidNative.Array<boolean>;
							public trackNotifiedDownstreamFormats: androidNative.Array<boolean>;
							public constructor(param0: com.google.android.exoplayer2.source.TrackGroupArray, param1: androidNative.Array<boolean>);
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class ProgressiveMediaSource extends com.google.android.exoplayer2.source.BaseMediaSource implements com.google.android.exoplayer2.source.ProgressiveMediaPeriod.Listener {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.ProgressiveMediaSource>;
						public static DEFAULT_LOADING_CHECK_INTERVAL_BYTES: number;
						public enable(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public onSourceInfoRefreshed(param0: number, param1: boolean, param2: boolean): void;
						public isSingleWindow(): boolean;
						/** @deprecated */
						public prepareSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param1: com.google.android.exoplayer2.upstream.TransferListener): void;
						public prepareSourceInternal(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
						public maybeThrowSourceInfoRefreshError(): void;
						public createPeriod(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param1: com.google.android.exoplayer2.upstream.Allocator, param2: number): com.google.android.exoplayer2.source.MediaPeriod;
						public addEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
						public getInitialTimeline(): com.google.android.exoplayer2.Timeline;
						public addDrmEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
						public releaseSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public releaseSourceInternal(): void;
						public removeDrmEventListener(param0: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
						public disable(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public releasePeriod(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
						public prepareSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param1: com.google.android.exoplayer2.upstream.TransferListener, param2: com.google.android.exoplayer2.analytics.PlayerId): void;
						public removeEventListener(param0: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
						public getMediaItem(): com.google.android.exoplayer2.MediaItem;
					}
					export module ProgressiveMediaSource {
						export class Factory extends com.google.android.exoplayer2.source.MediaSourceFactory {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.ProgressiveMediaSource.Factory>;
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSource.Factory, param1: com.google.android.exoplayer2.extractor.ExtractorsFactory);
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSource.Factory, param1: com.google.android.exoplayer2.source.ProgressiveMediaExtractor.Factory);
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSource.Factory, param1: com.google.android.exoplayer2.source.ProgressiveMediaExtractor.Factory, param2: com.google.android.exoplayer2.drm.DrmSessionManagerProvider, param3: com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy, param4: number);
							public setLoadErrorHandlingPolicy(param0: com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy): com.google.android.exoplayer2.source.MediaSource.Factory;
							public setLoadErrorHandlingPolicy(param0: com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy): com.google.android.exoplayer2.source.ProgressiveMediaSource.Factory;
							public createMediaSource(param0: com.google.android.exoplayer2.MediaItem): com.google.android.exoplayer2.source.ProgressiveMediaSource;
							public createMediaSource(param0: com.google.android.exoplayer2.MediaItem): com.google.android.exoplayer2.source.MediaSource;
							public setDrmSessionManagerProvider(param0: com.google.android.exoplayer2.drm.DrmSessionManagerProvider): com.google.android.exoplayer2.source.ProgressiveMediaSource.Factory;
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSource.Factory);
							public getSupportedTypes(): androidNative.Array<number>;
							public setContinueLoadingCheckIntervalBytes(param0: number): com.google.android.exoplayer2.source.ProgressiveMediaSource.Factory;
							public setDrmSessionManagerProvider(param0: com.google.android.exoplayer2.drm.DrmSessionManagerProvider): com.google.android.exoplayer2.source.MediaSource.Factory;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class SampleDataQueue {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.SampleDataQueue>;
						public rewind(): void;
						public sampleData(param0: com.google.android.exoplayer2.util.ParsableByteArray, param1: number): void;
						public peekToBuffer(param0: com.google.android.exoplayer2.decoder.DecoderInputBuffer, param1: com.google.android.exoplayer2.source.SampleQueue.SampleExtrasHolder): void;
						public discardDownstreamTo(param0: number): void;
						public readToBuffer(param0: com.google.android.exoplayer2.decoder.DecoderInputBuffer, param1: com.google.android.exoplayer2.source.SampleQueue.SampleExtrasHolder): void;
						public getTotalBytesWritten(): number;
						public discardUpstreamSampleBytes(param0: number): void;
						public constructor(param0: com.google.android.exoplayer2.upstream.Allocator);
						public reset(): void;
						public sampleData(param0: com.google.android.exoplayer2.upstream.DataReader, param1: number, param2: boolean): number;
					}
					export module SampleDataQueue {
						export class AllocationNode extends com.google.android.exoplayer2.upstream.Allocator.AllocationNode {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.SampleDataQueue.AllocationNode>;
							public startPosition: number;
							public endPosition: number;
							public allocation: com.google.android.exoplayer2.upstream.Allocation;
							public reset(param0: number, param1: number): void;
							public next(): com.google.android.exoplayer2.upstream.Allocator.AllocationNode;
							public translateOffset(param0: number): number;
							public constructor(param0: number, param1: number);
							public getAllocation(): com.google.android.exoplayer2.upstream.Allocation;
							public initialize(param0: com.google.android.exoplayer2.upstream.Allocation, param1: com.google.android.exoplayer2.source.SampleDataQueue.AllocationNode): void;
							public clear(): com.google.android.exoplayer2.source.SampleDataQueue.AllocationNode;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class SampleQueue {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.SampleQueue>;
						public getWriteIndex(): number;
						public getReadIndex(): number;
						public peekSourceId(): number;
						public discardSampleMetadataToRead(): number;
						public skip(param0: number): void;
						public invalidateUpstreamFormatAdjustment(): void;
						public getFirstTimestampUs(): number;
						public read(param0: com.google.android.exoplayer2.FormatHolder, param1: com.google.android.exoplayer2.decoder.DecoderInputBuffer, param2: number, param3: boolean): number;
						/** @deprecated */
						public static createWithDrm(param0: com.google.android.exoplayer2.upstream.Allocator, param1: globalAndroid.os.Looper, param2: com.google.android.exoplayer2.drm.DrmSessionManager, param3: com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher): com.google.android.exoplayer2.source.SampleQueue;
						public getFirstIndex(): number;
						public preRelease(): void;
						public getUpstreamFormat(): com.google.android.exoplayer2.Format;
						public getLargestQueuedTimestampUs(): number;
						public sampleData(param0: com.google.android.exoplayer2.upstream.DataReader, param1: number, param2: boolean, param3: number): number;
						public discardTo(param0: number, param1: boolean, param2: boolean): void;
						public sourceId(param0: number): void;
						public discardToRead(): void;
						public setStartTimeUs(param0: number): void;
						public static createWithDrm(param0: com.google.android.exoplayer2.upstream.Allocator, param1: com.google.android.exoplayer2.drm.DrmSessionManager, param2: com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher): com.google.android.exoplayer2.source.SampleQueue;
						public seekTo(param0: number): boolean;
						public getAdjustedUpstreamFormat(param0: com.google.android.exoplayer2.Format): com.google.android.exoplayer2.Format;
						public format(param0: com.google.android.exoplayer2.Format): void;
						public discardUpstreamSamples(param0: number): void;
						public isLastSampleQueued(): boolean;
						public release(): void;
						public setSampleOffsetUs(param0: number): void;
						public isReady(param0: boolean): boolean;
						public getSkipCount(param0: number, param1: boolean): number;
						public discardUpstreamFrom(param0: number): void;
						public static createWithoutDrm(param0: com.google.android.exoplayer2.upstream.Allocator): com.google.android.exoplayer2.source.SampleQueue;
						public reset(param0: boolean): void;
						public constructor(param0: com.google.android.exoplayer2.upstream.Allocator, param1: com.google.android.exoplayer2.drm.DrmSessionManager, param2: com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher);
						public getLargestReadTimestampUs(): number;
						public seekTo(param0: number, param1: boolean): boolean;
						public setUpstreamFormatChangeListener(param0: com.google.android.exoplayer2.source.SampleQueue.UpstreamFormatChangedListener): void;
						public discardToEnd(): void;
						public sampleData(param0: com.google.android.exoplayer2.util.ParsableByteArray, param1: number, param2: number): void;
						public splice(): void;
						public maybeThrowError(): void;
						public reset(): void;
						public sampleMetadata(param0: number, param1: number, param2: number, param3: number, param4: com.google.android.exoplayer2.extractor.TrackOutput.CryptoData): void;
					}
					export module SampleQueue {
						export class SampleExtrasHolder {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.SampleQueue.SampleExtrasHolder>;
							public size: number;
							public offset: number;
							public cryptoData: com.google.android.exoplayer2.extractor.TrackOutput.CryptoData;
						}
						export class SharedSampleMetadata {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.SampleQueue.SharedSampleMetadata>;
							public format: com.google.android.exoplayer2.Format;
							public drmSessionReference: com.google.android.exoplayer2.drm.DrmSessionManager.DrmSessionReference;
						}
						export class UpstreamFormatChangedListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.SampleQueue.UpstreamFormatChangedListener>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.source.SampleQueue$UpstreamFormatChangedListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onUpstreamFormatChanged(param0: com.google.android.exoplayer2.Format): void;
							});
							public constructor();
							public onUpstreamFormatChanged(param0: com.google.android.exoplayer2.Format): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class SampleStream {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.SampleStream>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.source.SampleStream interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							isReady(): boolean;
							maybeThrowError(): void;
							readData(param0: com.google.android.exoplayer2.FormatHolder, param1: com.google.android.exoplayer2.decoder.DecoderInputBuffer, param2: number): number;
							skipData(param0: number): number;
						});
						public constructor();
						public static FLAG_REQUIRE_FORMAT: number;
						public static FLAG_OMIT_SAMPLE_DATA: number;
						public static FLAG_PEEK: number;
						public readData(param0: com.google.android.exoplayer2.FormatHolder, param1: com.google.android.exoplayer2.decoder.DecoderInputBuffer, param2: number): number;
						public isReady(): boolean;
						public maybeThrowError(): void;
						public skipData(param0: number): number;
					}
					export module SampleStream {
						export class ReadDataResult {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.SampleStream.ReadDataResult>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.source.SampleStream$ReadDataResult interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
							});
							public constructor();
						}
						export class ReadFlags {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.SampleStream.ReadFlags>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.source.SampleStream$ReadFlags interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
							});
							public constructor();
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class SequenceableLoader {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.SequenceableLoader>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.source.SequenceableLoader interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							getBufferedPositionUs(): number;
							getNextLoadPositionUs(): number;
							continueLoading(param0: number): boolean;
							isLoading(): boolean;
							reevaluateBuffer(param0: number): void;
						});
						public constructor();
						public isLoading(): boolean;
						public getNextLoadPositionUs(): number;
						public getBufferedPositionUs(): number;
						public reevaluateBuffer(param0: number): void;
						public continueLoading(param0: number): boolean;
					}
					export module SequenceableLoader {
						export class Callback<T>  extends java.lang.Object {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.SequenceableLoader.Callback<any>>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.source.SequenceableLoader$Callback interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onContinueLoadingRequested(param0: T): void;
							});
							public constructor();
							public onContinueLoadingRequested(param0: T): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class ShuffleOrder {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.ShuffleOrder>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.source.ShuffleOrder interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							getLength(): number;
							getNextIndex(param0: number): number;
							getPreviousIndex(param0: number): number;
							getLastIndex(): number;
							getFirstIndex(): number;
							cloneAndInsert(param0: number, param1: number): com.google.android.exoplayer2.source.ShuffleOrder;
							cloneAndRemove(param0: number, param1: number): com.google.android.exoplayer2.source.ShuffleOrder;
							cloneAndClear(): com.google.android.exoplayer2.source.ShuffleOrder;
						});
						public constructor();
						public getPreviousIndex(param0: number): number;
						public cloneAndInsert(param0: number, param1: number): com.google.android.exoplayer2.source.ShuffleOrder;
						public getLastIndex(): number;
						public getNextIndex(param0: number): number;
						public cloneAndRemove(param0: number, param1: number): com.google.android.exoplayer2.source.ShuffleOrder;
						public getLength(): number;
						public getFirstIndex(): number;
						public cloneAndClear(): com.google.android.exoplayer2.source.ShuffleOrder;
					}
					export module ShuffleOrder {
						export class DefaultShuffleOrder extends com.google.android.exoplayer2.source.ShuffleOrder {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.ShuffleOrder.DefaultShuffleOrder>;
							public getLength(): number;
							public cloneAndRemove(param0: number, param1: number): com.google.android.exoplayer2.source.ShuffleOrder;
							public constructor(param0: number, param1: number);
							public getLastIndex(): number;
							public getNextIndex(param0: number): number;
							public getFirstIndex(): number;
							public cloneAndInsert(param0: number, param1: number): com.google.android.exoplayer2.source.ShuffleOrder;
							public cloneAndClear(): com.google.android.exoplayer2.source.ShuffleOrder;
							public constructor(param0: androidNative.Array<number>, param1: number);
							public getPreviousIndex(param0: number): number;
							public constructor(param0: number);
						}
						export class UnshuffledShuffleOrder extends com.google.android.exoplayer2.source.ShuffleOrder {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.ShuffleOrder.UnshuffledShuffleOrder>;
							public getLength(): number;
							public cloneAndRemove(param0: number, param1: number): com.google.android.exoplayer2.source.ShuffleOrder;
							public getLastIndex(): number;
							public getNextIndex(param0: number): number;
							public getFirstIndex(): number;
							public cloneAndInsert(param0: number, param1: number): com.google.android.exoplayer2.source.ShuffleOrder;
							public cloneAndClear(): com.google.android.exoplayer2.source.ShuffleOrder;
							public getPreviousIndex(param0: number): number;
							public constructor(param0: number);
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class SilenceMediaSource extends com.google.android.exoplayer2.source.BaseMediaSource {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.SilenceMediaSource>;
						public static MEDIA_ID: string;
						public enable(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public constructor();
						public isSingleWindow(): boolean;
						/** @deprecated */
						public prepareSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param1: com.google.android.exoplayer2.upstream.TransferListener): void;
						public prepareSourceInternal(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
						public maybeThrowSourceInfoRefreshError(): void;
						public createPeriod(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param1: com.google.android.exoplayer2.upstream.Allocator, param2: number): com.google.android.exoplayer2.source.MediaPeriod;
						public addEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
						public getInitialTimeline(): com.google.android.exoplayer2.Timeline;
						public addDrmEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
						public releaseSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public releaseSourceInternal(): void;
						public removeDrmEventListener(param0: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
						public disable(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public constructor(param0: number);
						public releasePeriod(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
						public prepareSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param1: com.google.android.exoplayer2.upstream.TransferListener, param2: com.google.android.exoplayer2.analytics.PlayerId): void;
						public removeEventListener(param0: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
						public getMediaItem(): com.google.android.exoplayer2.MediaItem;
					}
					export module SilenceMediaSource {
						export class Factory {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.SilenceMediaSource.Factory>;
							public constructor();
							public createMediaSource(): com.google.android.exoplayer2.source.SilenceMediaSource;
							public setTag(param0: any): com.google.android.exoplayer2.source.SilenceMediaSource.Factory;
							public setDurationUs(param0: number): com.google.android.exoplayer2.source.SilenceMediaSource.Factory;
						}
						export class SilenceMediaPeriod extends com.google.android.exoplayer2.source.MediaPeriod {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.SilenceMediaSource.SilenceMediaPeriod>;
							public getTrackGroups(): com.google.android.exoplayer2.source.TrackGroupArray;
							public selectTracks(param0: androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection>, param1: androidNative.Array<boolean>, param2: androidNative.Array<com.google.android.exoplayer2.source.SampleStream>, param3: androidNative.Array<boolean>, param4: number): number;
							public getStreamKeys(param0: java.util.List<com.google.android.exoplayer2.trackselection.ExoTrackSelection>): java.util.List<com.google.android.exoplayer2.offline.StreamKey>;
							public isLoading(): boolean;
							public continueLoading(param0: number): boolean;
							public constructor(param0: number);
							public maybeThrowPrepareError(): void;
							public getBufferedPositionUs(): number;
							public seekToUs(param0: number): number;
							public getNextLoadPositionUs(): number;
							public prepare(param0: com.google.android.exoplayer2.source.MediaPeriod.Callback, param1: number): void;
							public discardBuffer(param0: number, param1: boolean): void;
							public getAdjustedSeekPositionUs(param0: number, param1: com.google.android.exoplayer2.SeekParameters): number;
							public readDiscontinuity(): number;
							public reevaluateBuffer(param0: number): void;
						}
						export class SilenceSampleStream extends com.google.android.exoplayer2.source.SampleStream {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.SilenceMediaSource.SilenceSampleStream>;
							public isReady(): boolean;
							public skipData(param0: number): number;
							public maybeThrowError(): void;
							public readData(param0: com.google.android.exoplayer2.FormatHolder, param1: com.google.android.exoplayer2.decoder.DecoderInputBuffer, param2: number): number;
							public constructor(param0: number);
							public seekTo(param0: number): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class SinglePeriodTimeline {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.SinglePeriodTimeline>;
						public constructor(param0: number, param1: boolean, param2: boolean, param3: boolean, param4: any, param5: com.google.android.exoplayer2.MediaItem);
						/** @deprecated */
						public constructor(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: boolean, param8: boolean, param9: boolean, param10: any, param11: any);
						public getPeriod(param0: number, param1: com.google.android.exoplayer2.Timeline.Period, param2: boolean): com.google.android.exoplayer2.Timeline.Period;
						/** @deprecated */
						public constructor(param0: number, param1: number, param2: number, param3: number, param4: boolean, param5: boolean, param6: boolean, param7: any, param8: any);
						public constructor(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: boolean, param8: boolean, param9: boolean, param10: any, param11: com.google.android.exoplayer2.MediaItem, param12: com.google.android.exoplayer2.MediaItem.LiveConfiguration);
						public constructor(param0: number, param1: number, param2: number, param3: number, param4: boolean, param5: boolean, param6: boolean, param7: any, param8: com.google.android.exoplayer2.MediaItem);
						/** @deprecated */
						public constructor(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: boolean, param8: boolean, param9: any, param10: com.google.android.exoplayer2.MediaItem, param11: com.google.android.exoplayer2.MediaItem.LiveConfiguration);
						public getWindowCount(): number;
						public getIndexOfPeriod(param0: any): number;
						public getUidOfPeriod(param0: number): any;
						/** @deprecated */
						public constructor(param0: number, param1: boolean, param2: boolean, param3: boolean, param4: any, param5: any);
						public getWindow(param0: number, param1: com.google.android.exoplayer2.Timeline.Window, param2: number): com.google.android.exoplayer2.Timeline.Window;
						public getPeriodCount(): number;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class SingleSampleMediaPeriod extends java.lang.Object {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.SingleSampleMediaPeriod>;
						public selectTracks(param0: androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection>, param1: androidNative.Array<boolean>, param2: androidNative.Array<com.google.android.exoplayer2.source.SampleStream>, param3: androidNative.Array<boolean>, param4: number): number;
						public readDiscontinuity(): number;
						public isLoading(): boolean;
						public release(): void;
						public continueLoading(param0: number): boolean;
						public onLoadCompleted(param0: any, param1: number, param2: number): void;
						public onLoadError(param0: any, param1: number, param2: number, param3: java.io.IOException, param4: number): com.google.android.exoplayer2.upstream.Loader.LoadErrorAction;
						public discardBuffer(param0: number, param1: boolean): void;
						public prepare(param0: com.google.android.exoplayer2.source.MediaPeriod.Callback, param1: number): void;
						public onLoadCanceled(param0: com.google.android.exoplayer2.source.SingleSampleMediaPeriod.SourceLoadable, param1: number, param2: number, param3: boolean): void;
						public getAdjustedSeekPositionUs(param0: number, param1: com.google.android.exoplayer2.SeekParameters): number;
						public getNextLoadPositionUs(): number;
						public seekToUs(param0: number): number;
						public constructor(param0: com.google.android.exoplayer2.upstream.DataSpec, param1: com.google.android.exoplayer2.upstream.DataSource.Factory, param2: com.google.android.exoplayer2.upstream.TransferListener, param3: com.google.android.exoplayer2.Format, param4: number, param5: com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy, param6: com.google.android.exoplayer2.source.MediaSourceEventListener.EventDispatcher, param7: boolean);
						public getBufferedPositionUs(): number;
						public onLoadError(param0: com.google.android.exoplayer2.source.SingleSampleMediaPeriod.SourceLoadable, param1: number, param2: number, param3: java.io.IOException, param4: number): com.google.android.exoplayer2.upstream.Loader.LoadErrorAction;
						public getStreamKeys(param0: java.util.List<com.google.android.exoplayer2.trackselection.ExoTrackSelection>): java.util.List<com.google.android.exoplayer2.offline.StreamKey>;
						public reevaluateBuffer(param0: number): void;
						public getTrackGroups(): com.google.android.exoplayer2.source.TrackGroupArray;
						public onLoadCompleted(param0: com.google.android.exoplayer2.source.SingleSampleMediaPeriod.SourceLoadable, param1: number, param2: number): void;
						public maybeThrowPrepareError(): void;
						public onLoadCanceled(param0: any, param1: number, param2: number, param3: boolean): void;
					}
					export module SingleSampleMediaPeriod {
						export class SampleStreamImpl extends com.google.android.exoplayer2.source.SampleStream {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.SingleSampleMediaPeriod.SampleStreamImpl>;
							public isReady(): boolean;
							public skipData(param0: number): number;
							public maybeThrowError(): void;
							public readData(param0: com.google.android.exoplayer2.FormatHolder, param1: com.google.android.exoplayer2.decoder.DecoderInputBuffer, param2: number): number;
							public reset(): void;
						}
						export class SourceLoadable extends com.google.android.exoplayer2.upstream.Loader.Loadable {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.SingleSampleMediaPeriod.SourceLoadable>;
							public loadTaskId: number;
							public dataSpec: com.google.android.exoplayer2.upstream.DataSpec;
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSpec, param1: com.google.android.exoplayer2.upstream.DataSource);
							public cancelLoad(): void;
							public load(): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class SingleSampleMediaSource extends com.google.android.exoplayer2.source.BaseMediaSource {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.SingleSampleMediaSource>;
						public enable(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public isSingleWindow(): boolean;
						/** @deprecated */
						public prepareSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param1: com.google.android.exoplayer2.upstream.TransferListener): void;
						public prepareSourceInternal(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
						public maybeThrowSourceInfoRefreshError(): void;
						public createPeriod(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param1: com.google.android.exoplayer2.upstream.Allocator, param2: number): com.google.android.exoplayer2.source.MediaPeriod;
						public addEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
						public getInitialTimeline(): com.google.android.exoplayer2.Timeline;
						public addDrmEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
						public releaseSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public releaseSourceInternal(): void;
						public removeDrmEventListener(param0: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
						public disable(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
						public releasePeriod(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
						public prepareSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param1: com.google.android.exoplayer2.upstream.TransferListener, param2: com.google.android.exoplayer2.analytics.PlayerId): void;
						public removeEventListener(param0: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
						public getMediaItem(): com.google.android.exoplayer2.MediaItem;
					}
					export module SingleSampleMediaSource {
						export class Factory {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.SingleSampleMediaSource.Factory>;
							public createMediaSource(param0: com.google.android.exoplayer2.MediaItem.SubtitleConfiguration, param1: number): com.google.android.exoplayer2.source.SingleSampleMediaSource;
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSource.Factory);
							public setTag(param0: any): com.google.android.exoplayer2.source.SingleSampleMediaSource.Factory;
							public setTrackId(param0: string): com.google.android.exoplayer2.source.SingleSampleMediaSource.Factory;
							public setLoadErrorHandlingPolicy(param0: com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy): com.google.android.exoplayer2.source.SingleSampleMediaSource.Factory;
							public setTreatLoadErrorsAsEndOfStream(param0: boolean): com.google.android.exoplayer2.source.SingleSampleMediaSource.Factory;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class SpannedData<V>  extends java.lang.Object {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.SpannedData<any>>;
						public constructor();
						public discardTo(param0: number): void;
						public get(param0: number): V;
						public isEmpty(): boolean;
						public discardFrom(param0: number): void;
						public appendSpan(param0: number, param1: V): void;
						public getEndValue(): V;
						public clear(): void;
						public constructor(param0: com.google.android.exoplayer2.util.Consumer<V>);
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export class UnrecognizedInputFormatException {
						public static class: java.lang.Class<com.google.android.exoplayer2.source.UnrecognizedInputFormatException>;
						public uri: globalAndroid.net.Uri;
						public constructor(param0: string, param1: globalAndroid.net.Uri);
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export module ads {
						export class AdsLoader {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.ads.AdsLoader>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.source.ads.AdsLoader interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								setPlayer(param0: com.google.android.exoplayer2.Player): void;
								release(): void;
								setSupportedContentTypes(param0: androidNative.Array<number>): void;
								start(param0: com.google.android.exoplayer2.source.ads.AdsMediaSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: any, param3: com.google.android.exoplayer2.ui.AdViewProvider, param4: com.google.android.exoplayer2.source.ads.AdsLoader.EventListener): void;
								stop(param0: com.google.android.exoplayer2.source.ads.AdsMediaSource, param1: com.google.android.exoplayer2.source.ads.AdsLoader.EventListener): void;
								handlePrepareComplete(param0: com.google.android.exoplayer2.source.ads.AdsMediaSource, param1: number, param2: number): void;
								handlePrepareError(param0: com.google.android.exoplayer2.source.ads.AdsMediaSource, param1: number, param2: number, param3: java.io.IOException): void;
							});
							public constructor();
							public stop(param0: com.google.android.exoplayer2.source.ads.AdsMediaSource, param1: com.google.android.exoplayer2.source.ads.AdsLoader.EventListener): void;
							public setPlayer(param0: com.google.android.exoplayer2.Player): void;
							public setSupportedContentTypes(param0: androidNative.Array<number>): void;
							public release(): void;
							public handlePrepareError(param0: com.google.android.exoplayer2.source.ads.AdsMediaSource, param1: number, param2: number, param3: java.io.IOException): void;
							public start(param0: com.google.android.exoplayer2.source.ads.AdsMediaSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: any, param3: com.google.android.exoplayer2.ui.AdViewProvider, param4: com.google.android.exoplayer2.source.ads.AdsLoader.EventListener): void;
							public handlePrepareComplete(param0: com.google.android.exoplayer2.source.ads.AdsMediaSource, param1: number, param2: number): void;
						}
						export module AdsLoader {
							export class EventListener {
								public static class: java.lang.Class<com.google.android.exoplayer2.source.ads.AdsLoader.EventListener>;
								/**
								 * Constructs a new instance of the com.google.android.exoplayer2.source.ads.AdsLoader$EventListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
								 */
								public constructor(implementation: {
									onAdPlaybackState(param0: com.google.android.exoplayer2.source.ads.AdPlaybackState): void;
									onAdLoadError(param0: com.google.android.exoplayer2.source.ads.AdsMediaSource.AdLoadException, param1: com.google.android.exoplayer2.upstream.DataSpec): void;
									onAdClicked(): void;
									onAdTapped(): void;
								});
								public constructor();
								public onAdPlaybackState(param0: com.google.android.exoplayer2.source.ads.AdPlaybackState): void;
								public onAdLoadError(param0: com.google.android.exoplayer2.source.ads.AdsMediaSource.AdLoadException, param1: com.google.android.exoplayer2.upstream.DataSpec): void;
								public onAdClicked(): void;
								public onAdTapped(): void;
							}
							export class Provider {
								public static class: java.lang.Class<com.google.android.exoplayer2.source.ads.AdsLoader.Provider>;
								/**
								 * Constructs a new instance of the com.google.android.exoplayer2.source.ads.AdsLoader$Provider interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
								 */
								public constructor(implementation: {
									getAdsLoader(param0: com.google.android.exoplayer2.MediaItem.AdsConfiguration): com.google.android.exoplayer2.source.ads.AdsLoader;
								});
								public constructor();
								public getAdsLoader(param0: com.google.android.exoplayer2.MediaItem.AdsConfiguration): com.google.android.exoplayer2.source.ads.AdsLoader;
							}
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export module ads {
						export class AdsMediaSource extends com.google.android.exoplayer2.source.CompositeMediaSource<com.google.android.exoplayer2.source.MediaSource.MediaPeriodId> {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.ads.AdsMediaSource>;
							public constructor();
							public getMediaPeriodIdForChildMediaPeriodId(param0: any, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): com.google.android.exoplayer2.source.MediaSource.MediaPeriodId;
							public getMediaItem(): com.google.android.exoplayer2.MediaItem;
							public removeDrmEventListener(param0: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
							public releaseSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
							public removeEventListener(param0: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
							public releaseSourceInternal(): void;
							public constructor(param0: com.google.android.exoplayer2.source.MediaSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: any, param3: com.google.android.exoplayer2.source.MediaSource.Factory, param4: com.google.android.exoplayer2.source.ads.AdsLoader, param5: com.google.android.exoplayer2.ui.AdViewProvider);
							public maybeThrowSourceInfoRefreshError(): void;
							public addEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
							public disable(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
							public isSingleWindow(): boolean;
							public onChildSourceInfoRefreshed(param0: any, param1: com.google.android.exoplayer2.source.MediaSource, param2: com.google.android.exoplayer2.Timeline): void;
							public onChildSourceInfoRefreshed(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param1: com.google.android.exoplayer2.source.MediaSource, param2: com.google.android.exoplayer2.Timeline): void;
							public enable(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
							public prepareSourceInternal(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
							public getMediaPeriodIdForChildMediaPeriodId(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): com.google.android.exoplayer2.source.MediaSource.MediaPeriodId;
							public getInitialTimeline(): com.google.android.exoplayer2.Timeline;
							public releasePeriod(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
							public createPeriod(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param1: com.google.android.exoplayer2.upstream.Allocator, param2: number): com.google.android.exoplayer2.source.MediaPeriod;
							public prepareSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param1: com.google.android.exoplayer2.upstream.TransferListener, param2: com.google.android.exoplayer2.analytics.PlayerId): void;
							public addDrmEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
							/** @deprecated */
							public prepareSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param1: com.google.android.exoplayer2.upstream.TransferListener): void;
						}
						export module AdsMediaSource {
							export class AdLoadException {
								public static class: java.lang.Class<com.google.android.exoplayer2.source.ads.AdsMediaSource.AdLoadException>;
								public static TYPE_AD: number;
								public static TYPE_AD_GROUP: number;
								public static TYPE_ALL_ADS: number;
								public static TYPE_UNEXPECTED: number;
								public type: number;
								public static createForAdGroup(param0: java.lang.Exception, param1: number): com.google.android.exoplayer2.source.ads.AdsMediaSource.AdLoadException;
								public static createForUnexpected(param0: java.lang.RuntimeException): com.google.android.exoplayer2.source.ads.AdsMediaSource.AdLoadException;
								public static createForAd(param0: java.lang.Exception): com.google.android.exoplayer2.source.ads.AdsMediaSource.AdLoadException;
								public static createForAllAds(param0: java.lang.Exception): com.google.android.exoplayer2.source.ads.AdsMediaSource.AdLoadException;
								public getRuntimeExceptionForUnexpected(): java.lang.RuntimeException;
							}
							export module AdLoadException {
								export class Type {
									public static class: java.lang.Class<com.google.android.exoplayer2.source.ads.AdsMediaSource.AdLoadException.Type>;
									/**
									 * Constructs a new instance of the com.google.android.exoplayer2.source.ads.AdsMediaSource$AdLoadException$Type interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
									 */
									public constructor(implementation: {
									});
									public constructor();
								}
							}
							export class AdMediaSourceHolder {
								public static class: java.lang.Class<com.google.android.exoplayer2.source.ads.AdsMediaSource.AdMediaSourceHolder>;
								public getDurationUs(): number;
								public createMediaPeriod(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param1: com.google.android.exoplayer2.upstream.Allocator, param2: number): com.google.android.exoplayer2.source.MediaPeriod;
								public hasMediaSource(): boolean;
								public isInactive(): boolean;
								public handleSourceInfoRefresh(param0: com.google.android.exoplayer2.Timeline): void;
								public constructor(param0: com.google.android.exoplayer2.source.ads.AdsMediaSource, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId);
								public releaseMediaPeriod(param0: com.google.android.exoplayer2.source.MaskingMediaPeriod): void;
								public release(): void;
								public initializeWithMediaSource(param0: com.google.android.exoplayer2.source.MediaSource, param1: globalAndroid.net.Uri): void;
							}
							export class AdPrepareListener extends com.google.android.exoplayer2.source.MaskingMediaPeriod.PrepareListener {
								public static class: java.lang.Class<com.google.android.exoplayer2.source.ads.AdsMediaSource.AdPrepareListener>;
								public onPrepareError(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param1: java.io.IOException): void;
								public constructor(param0: com.google.android.exoplayer2.source.ads.AdsMediaSource, param1: globalAndroid.net.Uri);
								public onPrepareComplete(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
							}
							export class ComponentListener extends com.google.android.exoplayer2.source.ads.AdsLoader.EventListener {
								public static class: java.lang.Class<com.google.android.exoplayer2.source.ads.AdsMediaSource.ComponentListener>;
								public constructor(param0: com.google.android.exoplayer2.source.ads.AdsMediaSource);
								public onAdPlaybackState(param0: com.google.android.exoplayer2.source.ads.AdPlaybackState): void;
								public onAdLoadError(param0: com.google.android.exoplayer2.source.ads.AdsMediaSource.AdLoadException, param1: com.google.android.exoplayer2.upstream.DataSpec): void;
								public onAdClicked(): void;
								public onAdTapped(): void;
								public stop(): void;
							}
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export module ads {
						export class ServerSideAdInsertionMediaSource extends com.google.android.exoplayer2.source.BaseMediaSource implements com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, com.google.android.exoplayer2.source.MediaSourceEventListener, com.google.android.exoplayer2.drm.DrmSessionEventListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.ads.ServerSideAdInsertionMediaSource>;
							public getMediaItem(): com.google.android.exoplayer2.MediaItem;
							public onDrmSessionAcquired(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: number): void;
							public onLoadStarted(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData): void;
							public removeDrmEventListener(param0: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
							public onDrmKeysRestored(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
							public removeEventListener(param0: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
							public onDrmKeysLoaded(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
							public onLoadCanceled(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData): void;
							public addEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.source.MediaSourceEventListener): void;
							public constructor(param0: com.google.android.exoplayer2.source.MediaSource, param1: com.google.android.exoplayer2.source.ads.ServerSideAdInsertionMediaSource.AdPlaybackStateUpdater);
							public enable(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
							public disableInternal(): void;
							public onSourceInfoRefreshed(param0: com.google.android.exoplayer2.source.MediaSource, param1: com.google.android.exoplayer2.Timeline): void;
							public getInitialTimeline(): com.google.android.exoplayer2.Timeline;
							public onDrmSessionReleased(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
							public onDownstreamFormatChanged(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
							public createPeriod(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param1: com.google.android.exoplayer2.upstream.Allocator, param2: number): com.google.android.exoplayer2.source.MediaPeriod;
							public prepareSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param1: com.google.android.exoplayer2.upstream.TransferListener, param2: com.google.android.exoplayer2.analytics.PlayerId): void;
							public onDrmKeysRemoved(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
							public onLoadError(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData, param4: java.io.IOException, param5: boolean): void;
							/** @deprecated */
							public prepareSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller, param1: com.google.android.exoplayer2.upstream.TransferListener): void;
							public constructor();
							public enableInternal(): void;
							/** @deprecated */
							public onDrmSessionAcquired(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId): void;
							public onUpstreamDiscarded(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
							public releaseSource(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
							public releaseSourceInternal(): void;
							public onDrmSessionManagerError(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: java.lang.Exception): void;
							public maybeThrowSourceInfoRefreshError(): void;
							public disable(param0: com.google.android.exoplayer2.source.MediaSource.MediaSourceCaller): void;
							public isSingleWindow(): boolean;
							public prepareSourceInternal(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
							public setAdPlaybackStates(param0: com.google.common.collect.ImmutableMap<any,com.google.android.exoplayer2.source.ads.AdPlaybackState>): void;
							public releasePeriod(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
							public addDrmEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.drm.DrmSessionEventListener): void;
							public onLoadCompleted(param0: number, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.LoadEventInfo, param3: com.google.android.exoplayer2.source.MediaLoadData): void;
						}
						export module ServerSideAdInsertionMediaSource {
							export class AdPlaybackStateUpdater {
								public static class: java.lang.Class<com.google.android.exoplayer2.source.ads.ServerSideAdInsertionMediaSource.AdPlaybackStateUpdater>;
								/**
								 * Constructs a new instance of the com.google.android.exoplayer2.source.ads.ServerSideAdInsertionMediaSource$AdPlaybackStateUpdater interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
								 */
								public constructor(implementation: {
									onAdPlaybackStateUpdateRequested(param0: com.google.android.exoplayer2.Timeline): boolean;
								});
								public constructor();
								public onAdPlaybackStateUpdateRequested(param0: com.google.android.exoplayer2.Timeline): boolean;
							}
							export class MediaPeriodImpl extends com.google.android.exoplayer2.source.MediaPeriod {
								public static class: java.lang.Class<com.google.android.exoplayer2.source.ads.ServerSideAdInsertionMediaSource.MediaPeriodImpl>;
								public sharedPeriod: com.google.android.exoplayer2.source.ads.ServerSideAdInsertionMediaSource.SharedMediaPeriod;
								public mediaPeriodId: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId;
								public mediaSourceEventDispatcher: com.google.android.exoplayer2.source.MediaSourceEventListener.EventDispatcher;
								public drmEventDispatcher: com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher;
								public callback: com.google.android.exoplayer2.source.MediaPeriod.Callback;
								public lastStartPositionUs: number;
								public hasNotifiedDownstreamFormatChange: androidNative.Array<boolean>;
								public constructor(param0: com.google.android.exoplayer2.source.ads.ServerSideAdInsertionMediaSource.SharedMediaPeriod, param1: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param2: com.google.android.exoplayer2.source.MediaSourceEventListener.EventDispatcher, param3: com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher);
								public prepare(param0: com.google.android.exoplayer2.source.MediaPeriod.Callback, param1: number): void;
								public discardBuffer(param0: number, param1: boolean): void;
								public continueLoading(param0: number): boolean;
								public readDiscontinuity(): number;
								public getNextLoadPositionUs(): number;
								public reevaluateBuffer(param0: number): void;
								public getTrackGroups(): com.google.android.exoplayer2.source.TrackGroupArray;
								public getStreamKeys(param0: java.util.List<com.google.android.exoplayer2.trackselection.ExoTrackSelection>): java.util.List<com.google.android.exoplayer2.offline.StreamKey>;
								public maybeThrowPrepareError(): void;
								public getAdjustedSeekPositionUs(param0: number, param1: com.google.android.exoplayer2.SeekParameters): number;
								public isLoading(): boolean;
								public selectTracks(param0: androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection>, param1: androidNative.Array<boolean>, param2: androidNative.Array<com.google.android.exoplayer2.source.SampleStream>, param3: androidNative.Array<boolean>, param4: number): number;
								public seekToUs(param0: number): number;
								public getBufferedPositionUs(): number;
							}
							export class SampleStreamImpl extends com.google.android.exoplayer2.source.SampleStream {
								public static class: java.lang.Class<com.google.android.exoplayer2.source.ads.ServerSideAdInsertionMediaSource.SampleStreamImpl>;
								public skipData(param0: number): number;
								public isReady(): boolean;
								public constructor(param0: com.google.android.exoplayer2.source.ads.ServerSideAdInsertionMediaSource.MediaPeriodImpl, param1: number);
								public maybeThrowError(): void;
								public readData(param0: com.google.android.exoplayer2.FormatHolder, param1: com.google.android.exoplayer2.decoder.DecoderInputBuffer, param2: number): number;
							}
							export class ServerSideAdInsertionTimeline extends com.google.android.exoplayer2.source.ForwardingTimeline {
								public static class: java.lang.Class<com.google.android.exoplayer2.source.ads.ServerSideAdInsertionMediaSource.ServerSideAdInsertionTimeline>;
								public getWindow(param0: number, param1: com.google.android.exoplayer2.Timeline.Window, param2: number): com.google.android.exoplayer2.Timeline.Window;
								public constructor(param0: com.google.android.exoplayer2.Timeline, param1: com.google.common.collect.ImmutableMap<any,com.google.android.exoplayer2.source.ads.AdPlaybackState>);
								public constructor(param0: com.google.android.exoplayer2.Timeline);
								public getPeriod(param0: number, param1: com.google.android.exoplayer2.Timeline.Period, param2: boolean): com.google.android.exoplayer2.Timeline.Period;
							}
							export class SharedMediaPeriod extends com.google.android.exoplayer2.source.MediaPeriod.Callback {
								public static class: java.lang.Class<com.google.android.exoplayer2.source.ads.ServerSideAdInsertionMediaSource.SharedMediaPeriod>;
								public trackSelections: androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection>;
								public sampleStreams: androidNative.Array<com.google.android.exoplayer2.source.SampleStream>;
								public lastDownstreamFormatChangeData: androidNative.Array<com.google.android.exoplayer2.source.MediaLoadData>;
								public onLoadFinished(param0: com.google.android.exoplayer2.source.LoadEventInfo): void;
								public onContinueLoadingRequested(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
								public onDownstreamFormatChanged(param0: com.google.android.exoplayer2.source.ads.ServerSideAdInsertionMediaSource.MediaPeriodImpl, param1: com.google.android.exoplayer2.source.MediaLoadData): void;
								public readDiscontinuity(param0: com.google.android.exoplayer2.source.ads.ServerSideAdInsertionMediaSource.MediaPeriodImpl): number;
								public constructor(param0: com.google.android.exoplayer2.source.MediaPeriod, param1: any, param2: com.google.android.exoplayer2.source.ads.AdPlaybackState);
								public isUnused(): boolean;
								public seekToUs(param0: com.google.android.exoplayer2.source.ads.ServerSideAdInsertionMediaSource.MediaPeriodImpl, param1: number): number;
								public getBufferedPositionUs(param0: com.google.android.exoplayer2.source.ads.ServerSideAdInsertionMediaSource.MediaPeriodImpl): number;
								public canReuseMediaPeriod(param0: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param1: number): boolean;
								public discardBuffer(param0: com.google.android.exoplayer2.source.ads.ServerSideAdInsertionMediaSource.MediaPeriodImpl, param1: number, param2: boolean): void;
								public updateAdPlaybackState(param0: com.google.android.exoplayer2.source.ads.AdPlaybackState): void;
								public getStreamKeys(param0: java.util.List<com.google.android.exoplayer2.trackselection.ExoTrackSelection>): java.util.List<com.google.android.exoplayer2.offline.StreamKey>;
								public getAdjustedSeekPositionUs(param0: com.google.android.exoplayer2.source.ads.ServerSideAdInsertionMediaSource.MediaPeriodImpl, param1: number, param2: com.google.android.exoplayer2.SeekParameters): number;
								public release(param0: com.google.android.exoplayer2.source.MediaSource): void;
								public getMediaPeriodForEvent(param0: com.google.android.exoplayer2.source.MediaLoadData): com.google.android.exoplayer2.source.ads.ServerSideAdInsertionMediaSource.MediaPeriodImpl;
								public continueLoading(param0: com.google.android.exoplayer2.source.ads.ServerSideAdInsertionMediaSource.MediaPeriodImpl, param1: number): boolean;
								public onContinueLoadingRequested(param0: any): void;
								public skipData(param0: com.google.android.exoplayer2.source.ads.ServerSideAdInsertionMediaSource.MediaPeriodImpl, param1: number, param2: number): number;
								public onLoadStarted(param0: com.google.android.exoplayer2.source.LoadEventInfo, param1: com.google.android.exoplayer2.source.MediaLoadData): void;
								public remove(param0: com.google.android.exoplayer2.source.ads.ServerSideAdInsertionMediaSource.MediaPeriodImpl): void;
								public onPrepared(param0: com.google.android.exoplayer2.source.MediaPeriod): void;
								public selectTracks(param0: com.google.android.exoplayer2.source.ads.ServerSideAdInsertionMediaSource.MediaPeriodImpl, param1: androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection>, param2: androidNative.Array<boolean>, param3: androidNative.Array<com.google.android.exoplayer2.source.SampleStream>, param4: androidNative.Array<boolean>, param5: number): number;
								public maybeThrowError(param0: number): void;
								public isLoading(param0: com.google.android.exoplayer2.source.ads.ServerSideAdInsertionMediaSource.MediaPeriodImpl): boolean;
								public isReady(param0: number): boolean;
								public add(param0: com.google.android.exoplayer2.source.ads.ServerSideAdInsertionMediaSource.MediaPeriodImpl): void;
								public getNextLoadPositionUs(param0: com.google.android.exoplayer2.source.ads.ServerSideAdInsertionMediaSource.MediaPeriodImpl): number;
								public getTrackGroups(): com.google.android.exoplayer2.source.TrackGroupArray;
								public prepare(param0: com.google.android.exoplayer2.source.ads.ServerSideAdInsertionMediaSource.MediaPeriodImpl, param1: number): void;
								public readData(param0: com.google.android.exoplayer2.source.ads.ServerSideAdInsertionMediaSource.MediaPeriodImpl, param1: number, param2: com.google.android.exoplayer2.FormatHolder, param3: com.google.android.exoplayer2.decoder.DecoderInputBuffer, param4: number): number;
								public maybeThrowPrepareError(): void;
								public reevaluateBuffer(param0: com.google.android.exoplayer2.source.ads.ServerSideAdInsertionMediaSource.MediaPeriodImpl, param1: number): void;
							}
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export module ads {
						export class ServerSideAdInsertionUtil {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.ads.ServerSideAdInsertionUtil>;
							public static addAdGroupToAdPlaybackState(param0: com.google.android.exoplayer2.source.ads.AdPlaybackState, param1: number, param2: number, param3: androidNative.Array<number>): com.google.android.exoplayer2.source.ads.AdPlaybackState;
							public static getStreamPositionUsForAd(param0: number, param1: number, param2: number, param3: com.google.android.exoplayer2.source.ads.AdPlaybackState): number;
							public static getStreamPositionUs(param0: com.google.android.exoplayer2.Player, param1: com.google.android.exoplayer2.source.ads.AdPlaybackState): number;
							public static getMediaPeriodPositionUsForContent(param0: number, param1: number, param2: com.google.android.exoplayer2.source.ads.AdPlaybackState): number;
							public static getAdCountInGroup(param0: com.google.android.exoplayer2.source.ads.AdPlaybackState, param1: number): number;
							public static getMediaPeriodPositionUsForAd(param0: number, param1: number, param2: number, param3: com.google.android.exoplayer2.source.ads.AdPlaybackState): number;
							public static getStreamPositionUsForContent(param0: number, param1: number, param2: com.google.android.exoplayer2.source.ads.AdPlaybackState): number;
							public static getMediaPeriodPositionUs(param0: number, param1: com.google.android.exoplayer2.source.MediaPeriodId, param2: com.google.android.exoplayer2.source.ads.AdPlaybackState): number;
							public static getStreamPositionUs(param0: number, param1: com.google.android.exoplayer2.source.MediaPeriodId, param2: com.google.android.exoplayer2.source.ads.AdPlaybackState): number;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export module ads {
						export class SinglePeriodAdTimeline extends com.google.android.exoplayer2.source.ForwardingTimeline {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.ads.SinglePeriodAdTimeline>;
							public getPeriod(param0: number, param1: com.google.android.exoplayer2.Timeline.Period, param2: boolean): com.google.android.exoplayer2.Timeline.Period;
							public constructor(param0: com.google.android.exoplayer2.Timeline);
							public constructor(param0: com.google.android.exoplayer2.Timeline, param1: com.google.android.exoplayer2.source.ads.AdPlaybackState);
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export module chunk {
						export abstract class BaseMediaChunk extends com.google.android.exoplayer2.source.chunk.MediaChunk {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.chunk.BaseMediaChunk>;
							public clippedStartTimeUs: number;
							public clippedEndTimeUs: number;
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: com.google.android.exoplayer2.Format, param3: number, param4: any, param5: number, param6: number, param7: number);
							public cancelLoad(): void;
							public init(param0: com.google.android.exoplayer2.source.chunk.BaseMediaChunkOutput): void;
							public load(): void;
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: number, param3: com.google.android.exoplayer2.Format, param4: number, param5: any, param6: number, param7: number);
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: com.google.android.exoplayer2.Format, param3: number, param4: any, param5: number, param6: number, param7: number, param8: number, param9: number);
							public getFirstSampleIndex(param0: number): number;
							public getOutput(): com.google.android.exoplayer2.source.chunk.BaseMediaChunkOutput;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export module chunk {
						export abstract class BaseMediaChunkIterator extends com.google.android.exoplayer2.source.chunk.MediaChunkIterator {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.chunk.BaseMediaChunkIterator>;
							public isEnded(): boolean;
							public checkInBounds(): void;
							public getCurrentIndex(): number;
							public constructor(param0: number, param1: number);
							public getChunkEndTimeUs(): number;
							public getChunkStartTimeUs(): number;
							public next(): boolean;
							public getDataSpec(): com.google.android.exoplayer2.upstream.DataSpec;
							public reset(): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export module chunk {
						export class BaseMediaChunkOutput extends com.google.android.exoplayer2.source.chunk.ChunkExtractor.TrackOutputProvider {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.chunk.BaseMediaChunkOutput>;
							public constructor(param0: androidNative.Array<number>, param1: androidNative.Array<com.google.android.exoplayer2.source.SampleQueue>);
							public track(param0: number, param1: number): com.google.android.exoplayer2.extractor.TrackOutput;
							public getWriteIndices(): androidNative.Array<number>;
							public setSampleOffsetUs(param0: number): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export module chunk {
						export class BundledChunkExtractor extends com.google.android.exoplayer2.source.chunk.ChunkExtractor {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.chunk.BundledChunkExtractor>;
							public static FACTORY: com.google.android.exoplayer2.source.chunk.ChunkExtractor.Factory;
							public init(param0: com.google.android.exoplayer2.source.chunk.ChunkExtractor.TrackOutputProvider, param1: number, param2: number): void;
							public getChunkIndex(): com.google.android.exoplayer2.extractor.ChunkIndex;
							public track(param0: number, param1: number): com.google.android.exoplayer2.extractor.TrackOutput;
							public release(): void;
							public getSampleFormats(): androidNative.Array<com.google.android.exoplayer2.Format>;
							public constructor(param0: com.google.android.exoplayer2.extractor.Extractor, param1: number, param2: com.google.android.exoplayer2.Format);
							public read(param0: com.google.android.exoplayer2.extractor.ExtractorInput): boolean;
							public endTracks(): void;
							public seekMap(param0: com.google.android.exoplayer2.extractor.SeekMap): void;
						}
						export module BundledChunkExtractor {
							export class BindingTrackOutput {
								public static class: java.lang.Class<com.google.android.exoplayer2.source.chunk.BundledChunkExtractor.BindingTrackOutput>;
								public sampleFormat: com.google.android.exoplayer2.Format;
								public bind(param0: com.google.android.exoplayer2.source.chunk.ChunkExtractor.TrackOutputProvider, param1: number): void;
								public sampleMetadata(param0: number, param1: number, param2: number, param3: number, param4: com.google.android.exoplayer2.extractor.TrackOutput.CryptoData): void;
								public sampleData(param0: com.google.android.exoplayer2.util.ParsableByteArray, param1: number, param2: number): void;
								public constructor(param0: number, param1: number, param2: com.google.android.exoplayer2.Format);
								public format(param0: com.google.android.exoplayer2.Format): void;
								public sampleData(param0: com.google.android.exoplayer2.upstream.DataReader, param1: number, param2: boolean, param3: number): number;
							}
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export module chunk {
						export abstract class Chunk extends com.google.android.exoplayer2.upstream.Loader.Loadable {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.chunk.Chunk>;
							public loadTaskId: number;
							public dataSpec: com.google.android.exoplayer2.upstream.DataSpec;
							public type: number;
							public trackFormat: com.google.android.exoplayer2.Format;
							public trackSelectionReason: number;
							public trackSelectionData: any;
							public startTimeUs: number;
							public endTimeUs: number;
							public dataSource: com.google.android.exoplayer2.upstream.StatsDataSource;
							public cancelLoad(): void;
							public getUri(): globalAndroid.net.Uri;
							public getDurationUs(): number;
							public load(): void;
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: number, param3: com.google.android.exoplayer2.Format, param4: number, param5: any, param6: number, param7: number);
							public bytesLoaded(): number;
							public getResponseHeaders(): java.util.Map<string,java.util.List<string>>;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export module chunk {
						export class ChunkExtractor {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.chunk.ChunkExtractor>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.source.chunk.ChunkExtractor interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								getChunkIndex(): com.google.android.exoplayer2.extractor.ChunkIndex;
								getSampleFormats(): androidNative.Array<com.google.android.exoplayer2.Format>;
								init(param0: com.google.android.exoplayer2.source.chunk.ChunkExtractor.TrackOutputProvider, param1: number, param2: number): void;
								release(): void;
								read(param0: com.google.android.exoplayer2.extractor.ExtractorInput): boolean;
							});
							public constructor();
							public init(param0: com.google.android.exoplayer2.source.chunk.ChunkExtractor.TrackOutputProvider, param1: number, param2: number): void;
							public getChunkIndex(): com.google.android.exoplayer2.extractor.ChunkIndex;
							public release(): void;
							public getSampleFormats(): androidNative.Array<com.google.android.exoplayer2.Format>;
							public read(param0: com.google.android.exoplayer2.extractor.ExtractorInput): boolean;
						}
						export module ChunkExtractor {
							export class Factory {
								public static class: java.lang.Class<com.google.android.exoplayer2.source.chunk.ChunkExtractor.Factory>;
								/**
								 * Constructs a new instance of the com.google.android.exoplayer2.source.chunk.ChunkExtractor$Factory interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
								 */
								public constructor(implementation: {
									createProgressiveMediaExtractor(param0: number, param1: com.google.android.exoplayer2.Format, param2: boolean, param3: java.util.List<com.google.android.exoplayer2.Format>, param4: com.google.android.exoplayer2.extractor.TrackOutput, param5: com.google.android.exoplayer2.analytics.PlayerId): com.google.android.exoplayer2.source.chunk.ChunkExtractor;
								});
								public constructor();
								public createProgressiveMediaExtractor(param0: number, param1: com.google.android.exoplayer2.Format, param2: boolean, param3: java.util.List<com.google.android.exoplayer2.Format>, param4: com.google.android.exoplayer2.extractor.TrackOutput, param5: com.google.android.exoplayer2.analytics.PlayerId): com.google.android.exoplayer2.source.chunk.ChunkExtractor;
							}
							export class TrackOutputProvider {
								public static class: java.lang.Class<com.google.android.exoplayer2.source.chunk.ChunkExtractor.TrackOutputProvider>;
								/**
								 * Constructs a new instance of the com.google.android.exoplayer2.source.chunk.ChunkExtractor$TrackOutputProvider interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
								 */
								public constructor(implementation: {
									track(param0: number, param1: number): com.google.android.exoplayer2.extractor.TrackOutput;
								});
								public constructor();
								public track(param0: number, param1: number): com.google.android.exoplayer2.extractor.TrackOutput;
							}
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export module chunk {
						export class ChunkHolder {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.chunk.ChunkHolder>;
							public chunk: com.google.android.exoplayer2.source.chunk.Chunk;
							public endOfStream: boolean;
							public constructor();
							public clear(): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export module chunk {
						export class ChunkSampleStream<T>  extends java.lang.Object {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.chunk.ChunkSampleStream<any>>;
							public primaryTrackType: number;
							public onLoadCanceled(param0: com.google.android.exoplayer2.source.chunk.Chunk, param1: number, param2: number, param3: boolean): void;
							public selectEmbeddedTrack(param0: number, param1: number): com.google.android.exoplayer2.source.chunk.ChunkSampleStream.EmbeddedSampleStream;
							public onLoadError(param0: T, param1: number, param2: number, param3: java.io.IOException, param4: number): com.google.android.exoplayer2.upstream.Loader.LoadErrorAction;
							public isReady(): boolean;
							public isLoading(): boolean;
							public skipData(param0: number): number;
							public constructor(param0: number, param1: androidNative.Array<number>, param2: androidNative.Array<com.google.android.exoplayer2.Format>, param3: T, param4: com.google.android.exoplayer2.source.SequenceableLoader.Callback<com.google.android.exoplayer2.source.chunk.ChunkSampleStream<T>>, param5: com.google.android.exoplayer2.upstream.Allocator, param6: number, param7: com.google.android.exoplayer2.drm.DrmSessionManager, param8: com.google.android.exoplayer2.drm.DrmSessionEventListener.EventDispatcher, param9: com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy, param10: com.google.android.exoplayer2.source.MediaSourceEventListener.EventDispatcher);
							public release(): void;
							public continueLoading(param0: number): boolean;
							public readData(param0: com.google.android.exoplayer2.FormatHolder, param1: com.google.android.exoplayer2.decoder.DecoderInputBuffer, param2: number): number;
							public onLoadCanceled(param0: T, param1: number, param2: number, param3: boolean): void;
							public getChunkSource(): T;
							public getBufferedPositionUs(): number;
							public getNextLoadPositionUs(): number;
							public release(param0: com.google.android.exoplayer2.source.chunk.ChunkSampleStream.ReleaseCallback<T>): void;
							public onLoadError(param0: com.google.android.exoplayer2.source.chunk.Chunk, param1: number, param2: number, param3: java.io.IOException, param4: number): com.google.android.exoplayer2.upstream.Loader.LoadErrorAction;
							public discardBuffer(param0: number, param1: boolean): void;
							public seekToUs(param0: number): void;
							public onLoaderReleased(): void;
							public getAdjustedSeekPositionUs(param0: number, param1: com.google.android.exoplayer2.SeekParameters): number;
							public onLoadCompleted(param0: com.google.android.exoplayer2.source.chunk.Chunk, param1: number, param2: number): void;
							public onLoadCompleted(param0: T, param1: number, param2: number): void;
							public maybeThrowError(): void;
							public reevaluateBuffer(param0: number): void;
						}
						export module ChunkSampleStream {
							export class EmbeddedSampleStream extends com.google.android.exoplayer2.source.SampleStream {
								public static class: java.lang.Class<com.google.android.exoplayer2.source.chunk.ChunkSampleStream.EmbeddedSampleStream>;
								public parent: com.google.android.exoplayer2.source.chunk.ChunkSampleStream<any>;
								public skipData(param0: number): number;
								public isReady(): boolean;
								public release(): void;
								public constructor(param0: com.google.android.exoplayer2.source.chunk.ChunkSampleStream<any>, param1: com.google.android.exoplayer2.source.SampleQueue, param2: number);
								public maybeThrowError(): void;
								public readData(param0: com.google.android.exoplayer2.FormatHolder, param1: com.google.android.exoplayer2.decoder.DecoderInputBuffer, param2: number): number;
							}
							export class ReleaseCallback<T>  extends java.lang.Object {
								public static class: java.lang.Class<com.google.android.exoplayer2.source.chunk.ChunkSampleStream.ReleaseCallback<any>>;
								/**
								 * Constructs a new instance of the com.google.android.exoplayer2.source.chunk.ChunkSampleStream$ReleaseCallback interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
								 */
								public constructor(implementation: {
									onSampleStreamReleased(param0: com.google.android.exoplayer2.source.chunk.ChunkSampleStream<T>): void;
								});
								public constructor();
								public onSampleStreamReleased(param0: com.google.android.exoplayer2.source.chunk.ChunkSampleStream<T>): void;
							}
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export module chunk {
						export class ChunkSource {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.chunk.ChunkSource>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.source.chunk.ChunkSource interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								getAdjustedSeekPositionUs(param0: number, param1: com.google.android.exoplayer2.SeekParameters): number;
								maybeThrowError(): void;
								getPreferredQueueSize(param0: number, param1: java.util.List<any>): number;
								shouldCancelLoad(param0: number, param1: com.google.android.exoplayer2.source.chunk.Chunk, param2: java.util.List<any>): boolean;
								getNextChunk(param0: number, param1: number, param2: java.util.List<any>, param3: com.google.android.exoplayer2.source.chunk.ChunkHolder): void;
								onChunkLoadCompleted(param0: com.google.android.exoplayer2.source.chunk.Chunk): void;
								onChunkLoadError(param0: com.google.android.exoplayer2.source.chunk.Chunk, param1: boolean, param2: com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy.LoadErrorInfo, param3: com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy): boolean;
								release(): void;
							});
							public constructor();
							public getNextChunk(param0: number, param1: number, param2: java.util.List<any>, param3: com.google.android.exoplayer2.source.chunk.ChunkHolder): void;
							public onChunkLoadCompleted(param0: com.google.android.exoplayer2.source.chunk.Chunk): void;
							public onChunkLoadError(param0: com.google.android.exoplayer2.source.chunk.Chunk, param1: boolean, param2: com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy.LoadErrorInfo, param3: com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy): boolean;
							public shouldCancelLoad(param0: number, param1: com.google.android.exoplayer2.source.chunk.Chunk, param2: java.util.List<any>): boolean;
							public getAdjustedSeekPositionUs(param0: number, param1: com.google.android.exoplayer2.SeekParameters): number;
							public release(): void;
							public maybeThrowError(): void;
							public getPreferredQueueSize(param0: number, param1: java.util.List<any>): number;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export module chunk {
						export class ContainerMediaChunk extends com.google.android.exoplayer2.source.chunk.BaseMediaChunk {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.chunk.ContainerMediaChunk>;
							public isLoadCompleted(): boolean;
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: com.google.android.exoplayer2.Format, param3: number, param4: any, param5: number, param6: number, param7: number);
							public cancelLoad(): void;
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: com.google.android.exoplayer2.Format, param3: number, param4: any, param5: number, param6: number, param7: number, param8: number, param9: number, param10: number, param11: number, param12: com.google.android.exoplayer2.source.chunk.ChunkExtractor);
							public load(): void;
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: number, param3: com.google.android.exoplayer2.Format, param4: number, param5: any, param6: number, param7: number);
							public getNextChunkIndex(): number;
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: com.google.android.exoplayer2.Format, param3: number, param4: any, param5: number, param6: number, param7: number, param8: number, param9: number);
							public getTrackOutputProvider(param0: com.google.android.exoplayer2.source.chunk.BaseMediaChunkOutput): com.google.android.exoplayer2.source.chunk.ChunkExtractor.TrackOutputProvider;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export module chunk {
						export abstract class DataChunk extends com.google.android.exoplayer2.source.chunk.Chunk {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.chunk.DataChunk>;
							public cancelLoad(): void;
							public consume(param0: androidNative.Array<number>, param1: number): void;
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: number, param3: com.google.android.exoplayer2.Format, param4: number, param5: any, param6: androidNative.Array<number>);
							public load(): void;
							public getDataHolder(): androidNative.Array<number>;
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: number, param3: com.google.android.exoplayer2.Format, param4: number, param5: any, param6: number, param7: number);
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export module chunk {
						export class InitializationChunk extends com.google.android.exoplayer2.source.chunk.Chunk {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.chunk.InitializationChunk>;
							public cancelLoad(): void;
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: com.google.android.exoplayer2.Format, param3: number, param4: any, param5: com.google.android.exoplayer2.source.chunk.ChunkExtractor);
							public load(): void;
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: number, param3: com.google.android.exoplayer2.Format, param4: number, param5: any, param6: number, param7: number);
							public init(param0: com.google.android.exoplayer2.source.chunk.ChunkExtractor.TrackOutputProvider): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export module chunk {
						export abstract class MediaChunk extends com.google.android.exoplayer2.source.chunk.Chunk {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.chunk.MediaChunk>;
							public chunkIndex: number;
							public isLoadCompleted(): boolean;
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: com.google.android.exoplayer2.Format, param3: number, param4: any, param5: number, param6: number, param7: number);
							public cancelLoad(): void;
							public load(): void;
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: number, param3: com.google.android.exoplayer2.Format, param4: number, param5: any, param6: number, param7: number);
							public getNextChunkIndex(): number;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export module chunk {
						export class MediaChunkIterator {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.chunk.MediaChunkIterator>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.source.chunk.MediaChunkIterator interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								isEnded(): boolean;
								next(): boolean;
								getDataSpec(): com.google.android.exoplayer2.upstream.DataSpec;
								getChunkStartTimeUs(): number;
								getChunkEndTimeUs(): number;
								reset(): void;
								<clinit>(): void;
							});
							public constructor();
							public static EMPTY: com.google.android.exoplayer2.source.chunk.MediaChunkIterator;
							public isEnded(): boolean;
							public getChunkEndTimeUs(): number;
							public getChunkStartTimeUs(): number;
							public next(): boolean;
							public getDataSpec(): com.google.android.exoplayer2.upstream.DataSpec;
							public reset(): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export module chunk {
						export class MediaParserChunkExtractor extends com.google.android.exoplayer2.source.chunk.ChunkExtractor {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.chunk.MediaParserChunkExtractor>;
							public static FACTORY: com.google.android.exoplayer2.source.chunk.ChunkExtractor.Factory;
							public init(param0: com.google.android.exoplayer2.source.chunk.ChunkExtractor.TrackOutputProvider, param1: number, param2: number): void;
							public getChunkIndex(): com.google.android.exoplayer2.extractor.ChunkIndex;
							public release(): void;
							public constructor(param0: number, param1: com.google.android.exoplayer2.Format, param2: java.util.List<com.google.android.exoplayer2.Format>, param3: com.google.android.exoplayer2.analytics.PlayerId);
							public getSampleFormats(): androidNative.Array<com.google.android.exoplayer2.Format>;
							public read(param0: com.google.android.exoplayer2.extractor.ExtractorInput): boolean;
						}
						export module MediaParserChunkExtractor {
							export class TrackOutputProviderAdapter {
								public static class: java.lang.Class<com.google.android.exoplayer2.source.chunk.MediaParserChunkExtractor.TrackOutputProviderAdapter>;
								public seekMap(param0: com.google.android.exoplayer2.extractor.SeekMap): void;
								public track(param0: number, param1: number): com.google.android.exoplayer2.extractor.TrackOutput;
								public endTracks(): void;
							}
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export module chunk {
						export class SingleSampleMediaChunk extends com.google.android.exoplayer2.source.chunk.BaseMediaChunk {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.chunk.SingleSampleMediaChunk>;
							public isLoadCompleted(): boolean;
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: com.google.android.exoplayer2.Format, param3: number, param4: any, param5: number, param6: number, param7: number);
							public cancelLoad(): void;
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: com.google.android.exoplayer2.Format, param3: number, param4: any, param5: number, param6: number, param7: number, param8: number, param9: com.google.android.exoplayer2.Format);
							public load(): void;
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: number, param3: com.google.android.exoplayer2.Format, param4: number, param5: any, param6: number, param7: number);
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: com.google.android.exoplayer2.Format, param3: number, param4: any, param5: number, param6: number, param7: number, param8: number, param9: number);
						}
					}
				}
			}
		}
	}
}


declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export module mediaparser {
						export class InputReaderAdapterV30 {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.mediaparser.InputReaderAdapterV30>;
							public constructor();
							public setDataReader(param0: com.google.android.exoplayer2.upstream.DataReader, param1: number): void;
							public getLength(): number;
							public getAndResetSeekPosition(): number;
							public getPosition(): number;
							public setCurrentPosition(param0: number): void;
							public seekToPosition(param0: number): void;
							public read(param0: androidNative.Array<number>, param1: number, param2: number): number;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export module mediaparser {
						export class MediaParserUtil {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.mediaparser.MediaParserUtil>;
							public static PARAMETER_IN_BAND_CRYPTO_INFO: string;
							public static PARAMETER_INCLUDE_SUPPLEMENTAL_DATA: string;
							public static PARAMETER_EAGERLY_EXPOSE_TRACK_TYPE: string;
							public static PARAMETER_EXPOSE_DUMMY_SEEK_MAP: string;
							public static PARAMETER_EXPOSE_CHUNK_INDEX_AS_MEDIA_FORMAT: string;
							public static PARAMETER_OVERRIDE_IN_BAND_CAPTION_DECLARATIONS: string;
							public static PARAMETER_EXPOSE_CAPTION_FORMATS: string;
							public static PARAMETER_IGNORE_TIMESTAMP_OFFSET: string;
							public static toCaptionsMediaFormat(param0: com.google.android.exoplayer2.Format): globalAndroid.media.MediaFormat;
							public static setLogSessionIdOnMediaParser(param0: globalAndroid.media.MediaParser, param1: com.google.android.exoplayer2.analytics.PlayerId): void;
						}
						export module MediaParserUtil {
							export class Api31 {
								public static class: java.lang.Class<com.google.android.exoplayer2.source.mediaparser.MediaParserUtil.Api31>;
								public static setLogSessionIdOnMediaParser(param0: globalAndroid.media.MediaParser, param1: com.google.android.exoplayer2.analytics.PlayerId): void;
							}
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module source {
					export module mediaparser {
						export class OutputConsumerAdapterV30 {
							public static class: java.lang.Class<com.google.android.exoplayer2.source.mediaparser.OutputConsumerAdapterV30>;
							public constructor();
							public onTrackCountFound(param0: number): void;
							public setMuxedCaptionFormats(param0: java.util.List<com.google.android.exoplayer2.Format>): void;
							public getSampleFormats(): androidNative.Array<com.google.android.exoplayer2.Format>;
							public onSampleCompleted(param0: number, param1: number, param2: number, param3: number, param4: number, param5: globalAndroid.media.MediaCodec.CryptoInfo): void;
							public setTimestampAdjuster(param0: com.google.android.exoplayer2.util.TimestampAdjuster): void;
							public setExtractorOutput(param0: com.google.android.exoplayer2.extractor.ExtractorOutput): void;
							public getDummySeekMap(): globalAndroid.media.MediaParser.SeekMap;
							public onTrackDataFound(param0: number, param1: globalAndroid.media.MediaParser.TrackData): void;
							public constructor(param0: com.google.android.exoplayer2.Format, param1: number, param2: boolean);
							public getChunkIndex(): com.google.android.exoplayer2.extractor.ChunkIndex;
							public setSampleTimestampUpperLimitFilterUs(param0: number): void;
							public setSelectedParserName(param0: string): void;
							public onSeekMapFound(param0: globalAndroid.media.MediaParser.SeekMap): void;
							public onSampleDataFound(param0: number, param1: globalAndroid.media.MediaParser.InputReader): void;
							public getSeekPoints(param0: number): globalAndroid.util.Pair<globalAndroid.media.MediaParser.SeekPoint,globalAndroid.media.MediaParser.SeekPoint>;
							public disableSeeking(): void;
						}
						export module OutputConsumerAdapterV30 {
							export class DataReaderAdapter {
								public static class: java.lang.Class<com.google.android.exoplayer2.source.mediaparser.OutputConsumerAdapterV30.DataReaderAdapter>;
								public input: globalAndroid.media.MediaParser.InputReader;
								public read(param0: androidNative.Array<number>, param1: number, param2: number): number;
							}
							export class SeekMapAdapter {
								public static class: java.lang.Class<com.google.android.exoplayer2.source.mediaparser.OutputConsumerAdapterV30.SeekMapAdapter>;
								public getDurationUs(): number;
								public getSeekPoints(param0: number): com.google.android.exoplayer2.extractor.SeekMap.SeekPoints;
								public constructor(param0: globalAndroid.media.MediaParser.SeekMap);
								public isSeekable(): boolean;
							}
						}
					}
				}
			}
		}
	}
}


declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module text {
					export class ExoplayerCuesDecoder {
						public static class: java.lang.Class<com.google.android.exoplayer2.text.ExoplayerCuesDecoder>;
						public setPositionUs(param0: number): void;
						public queueInputBuffer(param0: com.google.android.exoplayer2.text.SubtitleInputBuffer): void;
						public constructor();
						public dequeueInputBuffer(): com.google.android.exoplayer2.text.SubtitleInputBuffer;
						public dequeueOutputBuffer(): com.google.android.exoplayer2.text.SubtitleOutputBuffer;
						public flush(): void;
						public release(): void;
						public getName(): string;
					}
					export module ExoplayerCuesDecoder {
						export class SingleEventSubtitle {
							public static class: java.lang.Class<com.google.android.exoplayer2.text.ExoplayerCuesDecoder.SingleEventSubtitle>;
							public getCues(param0: number): java.util.List<com.google.android.exoplayer2.text.Cue>;
							public getEventTime(param0: number): number;
							public getNextEventTimeIndex(param0: number): number;
							public constructor(param0: number, param1: com.google.common.collect.ImmutableList<com.google.android.exoplayer2.text.Cue>);
							public getEventTimeCount(): number;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module text {
					export class SubtitleDecoderFactory {
						public static class: java.lang.Class<com.google.android.exoplayer2.text.SubtitleDecoderFactory>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.text.SubtitleDecoderFactory interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							supportsFormat(param0: com.google.android.exoplayer2.Format): boolean;
							createDecoder(param0: com.google.android.exoplayer2.Format): com.google.android.exoplayer2.text.SubtitleDecoder;
							<clinit>(): void;
						});
						public constructor();
						public static DEFAULT: com.google.android.exoplayer2.text.SubtitleDecoderFactory;
						public supportsFormat(param0: com.google.android.exoplayer2.Format): boolean;
						public createDecoder(param0: com.google.android.exoplayer2.Format): com.google.android.exoplayer2.text.SubtitleDecoder;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module text {
					export class TextOutput {
						public static class: java.lang.Class<com.google.android.exoplayer2.text.TextOutput>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.text.TextOutput interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							onCues(param0: java.util.List<com.google.android.exoplayer2.text.Cue>): void;
						});
						public constructor();
						public onCues(param0: java.util.List<com.google.android.exoplayer2.text.Cue>): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module text {
					export class TextRenderer extends com.google.android.exoplayer2.BaseRenderer {
						public static class: java.lang.Class<com.google.android.exoplayer2.text.TextRenderer>;
						public handleMessage(param0: globalAndroid.os.Message): boolean;
						public render(param0: number, param1: number): void;
						public getName(): string;
						public onPositionReset(param0: number, param1: boolean): void;
						public replaceStream(param0: androidNative.Array<com.google.android.exoplayer2.Format>, param1: com.google.android.exoplayer2.source.SampleStream, param2: number, param3: number): void;
						public getMediaClock(): com.google.android.exoplayer2.util.MediaClock;
						public isCurrentStreamFinal(): boolean;
						public static getHardwareAccelerationSupport(param0: number): number;
						public static create(param0: number): number;
						public static getTunnelingSupport(param0: number): number;
						public getStream(): com.google.android.exoplayer2.source.SampleStream;
						public static getAdaptiveSupport(param0: number): number;
						public onDisabled(): void;
						public isEnded(): boolean;
						public getCapabilities(): com.google.android.exoplayer2.RendererCapabilities;
						public enable(param0: com.google.android.exoplayer2.RendererConfiguration, param1: androidNative.Array<com.google.android.exoplayer2.Format>, param2: com.google.android.exoplayer2.source.SampleStream, param3: number, param4: boolean, param5: boolean, param6: number, param7: number): void;
						public setPlaybackSpeed(param0: number, param1: number): void;
						public setCurrentStreamFinal(): void;
						public constructor(param0: com.google.android.exoplayer2.text.TextOutput, param1: globalAndroid.os.Looper, param2: com.google.android.exoplayer2.text.SubtitleDecoderFactory);
						public onStreamChanged(param0: androidNative.Array<com.google.android.exoplayer2.Format>, param1: number, param2: number): void;
						public resetPosition(param0: number): void;
						public init(param0: number, param1: com.google.android.exoplayer2.analytics.PlayerId): void;
						public hasReadStreamToEnd(): boolean;
						public isReady(): boolean;
						public disable(): void;
						public getTrackType(): number;
						public supportsMixedMimeTypeAdaptation(): number;
						public stop(): void;
						public constructor(param0: com.google.android.exoplayer2.text.TextOutput, param1: globalAndroid.os.Looper);
						public handleMessage(param0: number, param1: any): void;
						public setFinalStreamEndPositionUs(param0: number): void;
						public static getFormatSupport(param0: number): number;
						public supportsFormat(param0: com.google.android.exoplayer2.Format): number;
						public start(): void;
						public static create(param0: number, param1: number, param2: number): number;
						public constructor(param0: number);
						public getState(): number;
						public maybeThrowStreamError(): void;
						public static create(param0: number, param1: number, param2: number, param3: number, param4: number): number;
						public static getDecoderSupport(param0: number): number;
						public getReadingPositionUs(): number;
						public reset(): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module trackselection {
					export class AdaptiveTrackSelection extends com.google.android.exoplayer2.trackselection.BaseTrackSelection {
						public static class: java.lang.Class<com.google.android.exoplayer2.trackselection.AdaptiveTrackSelection>;
						public static DEFAULT_MIN_DURATION_FOR_QUALITY_INCREASE_MS: number;
						public static DEFAULT_MAX_DURATION_FOR_QUALITY_DECREASE_MS: number;
						public static DEFAULT_MIN_DURATION_TO_RETAIN_AFTER_DISCARD_MS: number;
						public static DEFAULT_MAX_WIDTH_TO_DISCARD: number;
						public static DEFAULT_MAX_HEIGHT_TO_DISCARD: number;
						public static DEFAULT_BANDWIDTH_FRACTION: number;
						public static DEFAULT_BUFFERED_FRACTION_TO_LIVE_EDGE_FOR_QUALITY_INCREASE: number;
						public constructor(param0: com.google.android.exoplayer2.source.TrackGroup, param1: androidNative.Array<number>, param2: number);
						public shouldEvaluateQueueSize(param0: number, param1: java.util.List<any>): boolean;
						public getMinDurationToRetainAfterDiscardUs(): number;
						public onPlayWhenReadyChanged(param0: boolean): void;
						public getSelectionData(): any;
						public canSelectFormat(param0: com.google.android.exoplayer2.Format, param1: number, param2: number): boolean;
						public getSelectedFormat(): com.google.android.exoplayer2.Format;
						public getSelectedIndexInTrackGroup(): number;
						public onPlaybackSpeed(param0: number): void;
						public getSelectionReason(): number;
						public disable(): void;
						public updateSelectedTrack(param0: number, param1: number, param2: number, param3: java.util.List<any>, param4: androidNative.Array<com.google.android.exoplayer2.source.chunk.MediaChunkIterator>): void;
						public enable(): void;
						public constructor(param0: com.google.android.exoplayer2.source.TrackGroup, param1: androidNative.Array<number>);
						public constructor(param0: com.google.android.exoplayer2.source.TrackGroup, param1: androidNative.Array<number>, param2: com.google.android.exoplayer2.upstream.BandwidthMeter);
						public blacklist(param0: number, param1: number): boolean;
						public evaluateQueueSize(param0: number, param1: java.util.List<any>): number;
						public onRebuffer(): void;
						public shouldCancelChunkLoad(param0: number, param1: com.google.android.exoplayer2.source.chunk.Chunk, param2: java.util.List<any>): boolean;
						public getSelectedIndex(): number;
						public constructor(param0: com.google.android.exoplayer2.source.TrackGroup, param1: androidNative.Array<number>, param2: number, param3: com.google.android.exoplayer2.upstream.BandwidthMeter, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: number, param11: java.util.List<com.google.android.exoplayer2.trackselection.AdaptiveTrackSelection.AdaptationCheckpoint>, param12: com.google.android.exoplayer2.util.Clock);
						public isBlacklisted(param0: number, param1: number): boolean;
						public onDiscontinuity(): void;
					}
					export module AdaptiveTrackSelection {
						export class AdaptationCheckpoint {
							public static class: java.lang.Class<com.google.android.exoplayer2.trackselection.AdaptiveTrackSelection.AdaptationCheckpoint>;
							public totalBandwidth: number;
							public allocatedBandwidth: number;
							public constructor(param0: number, param1: number);
							public hashCode(): number;
							public equals(param0: any): boolean;
						}
						export class Factory extends com.google.android.exoplayer2.trackselection.ExoTrackSelection.Factory {
							public static class: java.lang.Class<com.google.android.exoplayer2.trackselection.AdaptiveTrackSelection.Factory>;
							public constructor();
							public constructor(param0: number, param1: number, param2: number, param3: number, param4: number, param5: com.google.android.exoplayer2.util.Clock);
							public createTrackSelections(param0: androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection.Definition>, param1: com.google.android.exoplayer2.upstream.BandwidthMeter, param2: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param3: com.google.android.exoplayer2.Timeline): androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection>;
							public constructor(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number);
							public createAdaptiveTrackSelection(param0: com.google.android.exoplayer2.source.TrackGroup, param1: androidNative.Array<number>, param2: number, param3: com.google.android.exoplayer2.upstream.BandwidthMeter, param4: com.google.common.collect.ImmutableList<com.google.android.exoplayer2.trackselection.AdaptiveTrackSelection.AdaptationCheckpoint>): com.google.android.exoplayer2.trackselection.AdaptiveTrackSelection;
							public constructor(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: com.google.android.exoplayer2.util.Clock);
							public constructor(param0: number, param1: number, param2: number, param3: number);
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module trackselection {
					export abstract class BaseTrackSelection extends com.google.android.exoplayer2.trackselection.ExoTrackSelection {
						public static class: java.lang.Class<com.google.android.exoplayer2.trackselection.BaseTrackSelection>;
						public group: com.google.android.exoplayer2.source.TrackGroup;
						public tracks: androidNative.Array<number>;
						public onPlayWhenReadyChanged(param0: boolean): void;
						public getSelectedIndexInTrackGroup(): number;
						public onPlaybackSpeed(param0: number): void;
						public hashCode(): number;
						public equals(param0: any): boolean;
						public getSelectionReason(): number;
						public updateSelectedTrack(param0: number, param1: number, param2: number, param3: java.util.List<any>, param4: androidNative.Array<com.google.android.exoplayer2.source.chunk.MediaChunkIterator>): void;
						public blacklist(param0: number, param1: number): boolean;
						public shouldCancelChunkLoad(param0: number, param1: com.google.android.exoplayer2.source.chunk.Chunk, param2: java.util.List<any>): boolean;
						public indexOf(param0: com.google.android.exoplayer2.Format): number;
						public onDiscontinuity(): void;
						public constructor(param0: com.google.android.exoplayer2.source.TrackGroup, param1: androidNative.Array<number>, param2: number);
						public getIndexInTrackGroup(param0: number): number;
						public getSelectionData(): any;
						public length(): number;
						public getSelectedFormat(): com.google.android.exoplayer2.Format;
						public getType(): number;
						public getTrackGroup(): com.google.android.exoplayer2.source.TrackGroup;
						public disable(): void;
						public enable(): void;
						public constructor(param0: com.google.android.exoplayer2.source.TrackGroup, param1: androidNative.Array<number>);
						public evaluateQueueSize(param0: number, param1: java.util.List<any>): number;
						public onRebuffer(): void;
						public getFormat(param0: number): com.google.android.exoplayer2.Format;
						public getSelectedIndex(): number;
						public indexOf(param0: number): number;
						public isBlacklisted(param0: number, param1: number): boolean;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module trackselection {
					export class DefaultTrackSelector extends com.google.android.exoplayer2.trackselection.MappingTrackSelector {
						public static class: java.lang.Class<com.google.android.exoplayer2.trackselection.DefaultTrackSelector>;
						public static SELECTION_ELIGIBILITY_NO: number;
						public static SELECTION_ELIGIBILITY_FIXED: number;
						public static SELECTION_ELIGIBILITY_ADAPTIVE: number;
						public constructor();
						/** @deprecated */
						public constructor(param0: com.google.android.exoplayer2.trackselection.ExoTrackSelection.Factory);
						public selectTracks(param0: androidNative.Array<com.google.android.exoplayer2.RendererCapabilities>, param1: com.google.android.exoplayer2.source.TrackGroupArray, param2: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param3: com.google.android.exoplayer2.Timeline): com.google.android.exoplayer2.trackselection.TrackSelectorResult;
						public constructor(param0: globalAndroid.content.Context);
						public setParameters(param0: com.google.android.exoplayer2.trackselection.TrackSelectionParameters): void;
						public static normalizeUndeterminedLanguageToNull(param0: string): string;
						public setParameters(param0: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder): void;
						public constructor(param0: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters, param1: com.google.android.exoplayer2.trackselection.ExoTrackSelection.Factory);
						public selectOtherTrack(param0: number, param1: com.google.android.exoplayer2.source.TrackGroupArray, param2: androidNative.Array<androidNative.Array<number>>, param3: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters): com.google.android.exoplayer2.trackselection.ExoTrackSelection.Definition;
						public isSetParametersSupported(): boolean;
						public static getFormatLanguageScore(param0: com.google.android.exoplayer2.Format, param1: string, param2: boolean): number;
						public selectVideoTrack(param0: com.google.android.exoplayer2.trackselection.MappingTrackSelector.MappedTrackInfo, param1: androidNative.Array<androidNative.Array<androidNative.Array<number>>>, param2: androidNative.Array<number>, param3: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters): globalAndroid.util.Pair<com.google.android.exoplayer2.trackselection.ExoTrackSelection.Definition,java.lang.Integer>;
						/** @deprecated */
						public constructor();
						public getParameters(): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters;
						public static isSupported(param0: number, param1: boolean): boolean;
						public getParameters(): com.google.android.exoplayer2.trackselection.TrackSelectionParameters;
						public buildUponParameters(): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
						public selectAudioTrack(param0: com.google.android.exoplayer2.trackselection.MappingTrackSelector.MappedTrackInfo, param1: androidNative.Array<androidNative.Array<androidNative.Array<number>>>, param2: androidNative.Array<number>, param3: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters): globalAndroid.util.Pair<com.google.android.exoplayer2.trackselection.ExoTrackSelection.Definition,java.lang.Integer>;
						public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.trackselection.ExoTrackSelection.Factory);
						public selectTextTrack(param0: com.google.android.exoplayer2.trackselection.MappingTrackSelector.MappedTrackInfo, param1: androidNative.Array<androidNative.Array<androidNative.Array<number>>>, param2: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters, param3: string): globalAndroid.util.Pair<com.google.android.exoplayer2.trackselection.ExoTrackSelection.Definition,java.lang.Integer>;
						public selectAllTracks(param0: com.google.android.exoplayer2.trackselection.MappingTrackSelector.MappedTrackInfo, param1: androidNative.Array<androidNative.Array<androidNative.Array<number>>>, param2: androidNative.Array<number>, param3: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters): androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection.Definition>;
						public selectTracks(param0: com.google.android.exoplayer2.trackselection.MappingTrackSelector.MappedTrackInfo, param1: androidNative.Array<androidNative.Array<androidNative.Array<number>>>, param2: androidNative.Array<number>, param3: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param4: com.google.android.exoplayer2.Timeline): globalAndroid.util.Pair<androidNative.Array<com.google.android.exoplayer2.RendererConfiguration>,androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection>>;
					}
					export module DefaultTrackSelector {
						export class AudioTrackInfo extends com.google.android.exoplayer2.trackselection.DefaultTrackSelector.TrackInfo<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.AudioTrackInfo> implements java.lang.Comparable<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.AudioTrackInfo>  {
							public static class: java.lang.Class<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.AudioTrackInfo>;
							public isCompatibleForAdaptationWith(param0: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.AudioTrackInfo): boolean;
							public compareTo(param0: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.AudioTrackInfo): number;
							public getSelectionEligibility(): number;
							public isCompatibleForAdaptationWith(param0: any): boolean;
							public static compareSelections(param0: java.util.List<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.AudioTrackInfo>, param1: java.util.List<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.AudioTrackInfo>): number;
							public static createForTrackGroup(param0: number, param1: com.google.android.exoplayer2.source.TrackGroup, param2: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters, param3: androidNative.Array<number>, param4: boolean): com.google.common.collect.ImmutableList<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.AudioTrackInfo>;
							public constructor(param0: number, param1: com.google.android.exoplayer2.source.TrackGroup, param2: number);
							public constructor(param0: number, param1: com.google.android.exoplayer2.source.TrackGroup, param2: number, param3: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters, param4: number, param5: boolean);
						}
						export class OtherTrackScore extends java.lang.Comparable<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.OtherTrackScore> {
							public static class: java.lang.Class<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.OtherTrackScore>;
							public compareTo(param0: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.OtherTrackScore): number;
							public constructor(param0: com.google.android.exoplayer2.Format, param1: number);
						}
						export class Parameters {
							public static class: java.lang.Class<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters>;
							public static DEFAULT_WITHOUT_CONTEXT: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters;
							public static DEFAULT: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters;
							public disabledTextTrackSelectionFlags: number;
							public exceedVideoConstraintsIfNecessary: boolean;
							public allowVideoMixedMimeTypeAdaptiveness: boolean;
							public allowVideoNonSeamlessAdaptiveness: boolean;
							public allowVideoMixedDecoderSupportAdaptiveness: boolean;
							public exceedAudioConstraintsIfNecessary: boolean;
							public allowAudioMixedMimeTypeAdaptiveness: boolean;
							public allowAudioMixedSampleRateAdaptiveness: boolean;
							public allowAudioMixedChannelCountAdaptiveness: boolean;
							public allowAudioMixedDecoderSupportAdaptiveness: boolean;
							public exceedRendererCapabilitiesIfNecessary: boolean;
							public tunnelingEnabled: boolean;
							public allowMultipleAdaptiveSelections: boolean;
							public static CREATOR: com.google.android.exoplayer2.Bundleable.Creator<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters>;
							public getRendererDisabled(param0: number): boolean;
							/** @deprecated */
							public getSelectionOverride(param0: number, param1: com.google.android.exoplayer2.source.TrackGroupArray): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.SelectionOverride;
							public static getDefaults(param0: globalAndroid.content.Context): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters;
							public hashCode(): number;
							public buildUpon(): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public toBundle(): globalAndroid.os.Bundle;
							public equals(param0: any): boolean;
							/** @deprecated */
							public hasSelectionOverride(param0: number, param1: com.google.android.exoplayer2.source.TrackGroupArray): boolean;
						}
						export class ParametersBuilder {
							public static class: java.lang.Class<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder>;
							/** @deprecated */
							public clearSelectionOverrides(param0: number): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setTunnelingEnabled(param0: boolean): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public set(param0: com.google.android.exoplayer2.trackselection.TrackSelectionParameters): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setPreferredVideoRoleFlags(param0: number): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setTrackSelectionOverrides(param0: com.google.android.exoplayer2.trackselection.TrackSelectionOverrides): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setSelectUndeterminedTextLanguage(param0: boolean): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							/** @deprecated */
							public clearSelectionOverride(param0: number, param1: com.google.android.exoplayer2.source.TrackGroupArray): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public clearVideoSizeConstraints(): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setPreferredTextLanguageAndRoleFlagsToCaptioningManagerSettings(param0: globalAndroid.content.Context): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public clearViewportSizeConstraints(): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setMaxAudioChannelCount(param0: number): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setPreferredAudioLanguage(param0: string): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setMaxAudioBitrate(param0: number): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setPreferredAudioLanguages(param0: androidNative.Array<string>): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public build(): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters;
							public setViewportSizeToPhysicalDisplaySize(param0: globalAndroid.content.Context, param1: boolean): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setExceedAudioConstraintsIfNecessary(param0: boolean): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setDisabledTextTrackSelectionFlags(param0: number): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setForceHighestSupportedBitrate(param0: boolean): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setPreferredTextLanguages(param0: androidNative.Array<string>): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setDisabledTrackTypes(param0: java.util.Set<java.lang.Integer>): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setAllowAudioMixedMimeTypeAdaptiveness(param0: boolean): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setMaxVideoSize(param0: number, param1: number): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setMaxVideoFrameRate(param0: number): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setMaxVideoSizeSd(): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setMaxVideoBitrate(param0: number): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setExceedRendererCapabilitiesIfNecessary(param0: boolean): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setViewportSize(param0: number, param1: number, param2: boolean): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setAllowVideoNonSeamlessAdaptiveness(param0: boolean): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setMinVideoFrameRate(param0: number): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setForceLowestBitrate(param0: boolean): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setAllowAudioMixedSampleRateAdaptiveness(param0: boolean): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							/** @deprecated */
							public clearSelectionOverrides(): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setPreferredVideoMimeTypes(param0: androidNative.Array<string>): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setAllowMultipleAdaptiveSelections(param0: boolean): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setMinVideoBitrate(param0: number): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setAllowAudioMixedChannelCountAdaptiveness(param0: boolean): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setAllowVideoMixedMimeTypeAdaptiveness(param0: boolean): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setRendererDisabled(param0: number, param1: boolean): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setPreferredTextLanguage(param0: string): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setExceedVideoConstraintsIfNecessary(param0: boolean): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							/** @deprecated */
							public setSelectionOverride(param0: number, param1: com.google.android.exoplayer2.source.TrackGroupArray, param2: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.SelectionOverride): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setPreferredAudioMimeType(param0: string): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setMinVideoSize(param0: number, param1: number): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setPreferredAudioMimeTypes(param0: androidNative.Array<string>): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							/** @deprecated */
							public constructor();
							public setPreferredVideoMimeType(param0: string): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setAllowAudioMixedDecoderSupportAdaptiveness(param0: boolean): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setPreferredTextRoleFlags(param0: number): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setAllowVideoMixedDecoderSupportAdaptiveness(param0: boolean): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public setPreferredAudioRoleFlags(param0: number): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.ParametersBuilder;
							public constructor(param0: globalAndroid.content.Context);
						}
						export class SelectionOverride {
							public static class: java.lang.Class<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.SelectionOverride>;
							public groupIndex: number;
							public tracks: androidNative.Array<number>;
							public length: number;
							public type: number;
							public static CREATOR: com.google.android.exoplayer2.Bundleable.Creator<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.SelectionOverride>;
							public hashCode(): number;
							public constructor(param0: number, param1: androidNative.Array<number>, param2: number);
							public constructor(param0: number, param1: androidNative.Array<number>);
							public toBundle(): globalAndroid.os.Bundle;
							public equals(param0: any): boolean;
							public containsTrack(param0: number): boolean;
						}
						export class TextTrackInfo extends com.google.android.exoplayer2.trackselection.DefaultTrackSelector.TrackInfo<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.TextTrackInfo> implements java.lang.Comparable<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.TextTrackInfo>  {
							public static class: java.lang.Class<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.TextTrackInfo>;
							public compareTo(param0: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.TextTrackInfo): number;
							public isCompatibleForAdaptationWith(param0: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.TextTrackInfo): boolean;
							public static compareSelections(param0: java.util.List<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.TextTrackInfo>, param1: java.util.List<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.TextTrackInfo>): number;
							public getSelectionEligibility(): number;
							public isCompatibleForAdaptationWith(param0: any): boolean;
							public constructor(param0: number, param1: com.google.android.exoplayer2.source.TrackGroup, param2: number);
							public static createForTrackGroup(param0: number, param1: com.google.android.exoplayer2.source.TrackGroup, param2: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters, param3: androidNative.Array<number>, param4: string): com.google.common.collect.ImmutableList<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.TextTrackInfo>;
							public constructor(param0: number, param1: com.google.android.exoplayer2.source.TrackGroup, param2: number, param3: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters, param4: number, param5: string);
						}
						export abstract class TrackInfo<T>  extends java.lang.Object {
							public static class: java.lang.Class<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.TrackInfo<any>>;
							public rendererIndex: number;
							public trackGroup: com.google.android.exoplayer2.source.TrackGroup;
							public trackIndex: number;
							public format: com.google.android.exoplayer2.Format;
							public isCompatibleForAdaptationWith(param0: T): boolean;
							public getSelectionEligibility(): number;
							public constructor(param0: number, param1: com.google.android.exoplayer2.source.TrackGroup, param2: number);
						}
						export module TrackInfo {
							export class Factory<T>  extends java.lang.Object {
								public static class: java.lang.Class<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.TrackInfo.Factory<any>>;
								/**
								 * Constructs a new instance of the com.google.android.exoplayer2.trackselection.DefaultTrackSelector$TrackInfo$Factory interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
								 */
								public constructor(implementation: {
									create(param0: number, param1: com.google.android.exoplayer2.source.TrackGroup, param2: androidNative.Array<number>): java.util.List<T>;
								});
								public constructor();
								public create(param0: number, param1: com.google.android.exoplayer2.source.TrackGroup, param2: androidNative.Array<number>): java.util.List<T>;
							}
						}
						export class VideoTrackInfo extends com.google.android.exoplayer2.trackselection.DefaultTrackSelector.TrackInfo<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.VideoTrackInfo> {
							public static class: java.lang.Class<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.VideoTrackInfo>;
							public static createForTrackGroup(param0: number, param1: com.google.android.exoplayer2.source.TrackGroup, param2: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters, param3: androidNative.Array<number>, param4: number): com.google.common.collect.ImmutableList<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.VideoTrackInfo>;
							public constructor(param0: number, param1: com.google.android.exoplayer2.source.TrackGroup, param2: number, param3: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters, param4: number, param5: number, param6: boolean);
							public getSelectionEligibility(): number;
							public isCompatibleForAdaptationWith(param0: any): boolean;
							public isCompatibleForAdaptationWith(param0: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.VideoTrackInfo): boolean;
							public constructor(param0: number, param1: com.google.android.exoplayer2.source.TrackGroup, param2: number);
							public static compareSelections(param0: java.util.List<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.VideoTrackInfo>, param1: java.util.List<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.VideoTrackInfo>): number;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module trackselection {
					export class ExoTrackSelection {
						public static class: java.lang.Class<com.google.android.exoplayer2.trackselection.ExoTrackSelection>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.trackselection.ExoTrackSelection interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							enable(): void;
							disable(): void;
							getSelectedFormat(): com.google.android.exoplayer2.Format;
							getSelectedIndexInTrackGroup(): number;
							getSelectedIndex(): number;
							getSelectionReason(): number;
							getSelectionData(): any;
							onPlaybackSpeed(param0: number): void;
							onDiscontinuity(): void;
							onRebuffer(): void;
							onPlayWhenReadyChanged(param0: boolean): void;
							updateSelectedTrack(param0: number, param1: number, param2: number, param3: java.util.List<any>, param4: androidNative.Array<com.google.android.exoplayer2.source.chunk.MediaChunkIterator>): void;
							evaluateQueueSize(param0: number, param1: java.util.List<any>): number;
							shouldCancelChunkLoad(param0: number, param1: com.google.android.exoplayer2.source.chunk.Chunk, param2: java.util.List<any>): boolean;
							blacklist(param0: number, param1: number): boolean;
							isBlacklisted(param0: number, param1: number): boolean;
						});
						public constructor();
						public onPlayWhenReadyChanged(param0: boolean): void;
						public getSelectionData(): any;
						public getSelectedFormat(): com.google.android.exoplayer2.Format;
						public getSelectedIndexInTrackGroup(): number;
						public onPlaybackSpeed(param0: number): void;
						public getSelectionReason(): number;
						public disable(): void;
						public updateSelectedTrack(param0: number, param1: number, param2: number, param3: java.util.List<any>, param4: androidNative.Array<com.google.android.exoplayer2.source.chunk.MediaChunkIterator>): void;
						public enable(): void;
						public blacklist(param0: number, param1: number): boolean;
						public evaluateQueueSize(param0: number, param1: java.util.List<any>): number;
						public onRebuffer(): void;
						public shouldCancelChunkLoad(param0: number, param1: com.google.android.exoplayer2.source.chunk.Chunk, param2: java.util.List<any>): boolean;
						public getSelectedIndex(): number;
						public isBlacklisted(param0: number, param1: number): boolean;
						public onDiscontinuity(): void;
					}
					export module ExoTrackSelection {
						export class Definition {
							public static class: java.lang.Class<com.google.android.exoplayer2.trackselection.ExoTrackSelection.Definition>;
							public group: com.google.android.exoplayer2.source.TrackGroup;
							public tracks: androidNative.Array<number>;
							public type: number;
							public constructor(param0: com.google.android.exoplayer2.source.TrackGroup, param1: androidNative.Array<number>, param2: number);
							public constructor(param0: com.google.android.exoplayer2.source.TrackGroup, param1: androidNative.Array<number>);
						}
						export class Factory {
							public static class: java.lang.Class<com.google.android.exoplayer2.trackselection.ExoTrackSelection.Factory>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.trackselection.ExoTrackSelection$Factory interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								createTrackSelections(param0: androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection.Definition>, param1: com.google.android.exoplayer2.upstream.BandwidthMeter, param2: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param3: com.google.android.exoplayer2.Timeline): androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection>;
							});
							public constructor();
							public createTrackSelections(param0: androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection.Definition>, param1: com.google.android.exoplayer2.upstream.BandwidthMeter, param2: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param3: com.google.android.exoplayer2.Timeline): androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection>;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module trackselection {
					export class FixedTrackSelection extends com.google.android.exoplayer2.trackselection.BaseTrackSelection {
						public static class: java.lang.Class<com.google.android.exoplayer2.trackselection.FixedTrackSelection>;
						public constructor(param0: com.google.android.exoplayer2.source.TrackGroup, param1: androidNative.Array<number>, param2: number);
						public constructor(param0: com.google.android.exoplayer2.source.TrackGroup, param1: number, param2: number);
						public onPlayWhenReadyChanged(param0: boolean): void;
						public getSelectionData(): any;
						public getSelectedFormat(): com.google.android.exoplayer2.Format;
						public getSelectedIndexInTrackGroup(): number;
						public onPlaybackSpeed(param0: number): void;
						public getSelectionReason(): number;
						public disable(): void;
						public updateSelectedTrack(param0: number, param1: number, param2: number, param3: java.util.List<any>, param4: androidNative.Array<com.google.android.exoplayer2.source.chunk.MediaChunkIterator>): void;
						public enable(): void;
						public constructor(param0: com.google.android.exoplayer2.source.TrackGroup, param1: androidNative.Array<number>);
						public constructor(param0: com.google.android.exoplayer2.source.TrackGroup, param1: number);
						public blacklist(param0: number, param1: number): boolean;
						public evaluateQueueSize(param0: number, param1: java.util.List<any>): number;
						public onRebuffer(): void;
						public shouldCancelChunkLoad(param0: number, param1: com.google.android.exoplayer2.source.chunk.Chunk, param2: java.util.List<any>): boolean;
						public constructor(param0: com.google.android.exoplayer2.source.TrackGroup, param1: number, param2: number, param3: number, param4: any);
						public getSelectedIndex(): number;
						public isBlacklisted(param0: number, param1: number): boolean;
						public onDiscontinuity(): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module trackselection {
					export abstract class MappingTrackSelector extends com.google.android.exoplayer2.trackselection.TrackSelector {
						public static class: java.lang.Class<com.google.android.exoplayer2.trackselection.MappingTrackSelector>;
						public constructor();
						public selectTracks(param0: androidNative.Array<com.google.android.exoplayer2.RendererCapabilities>, param1: com.google.android.exoplayer2.source.TrackGroupArray, param2: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param3: com.google.android.exoplayer2.Timeline): com.google.android.exoplayer2.trackselection.TrackSelectorResult;
						public onSelectionActivated(param0: any): void;
						public getCurrentMappedTrackInfo(): com.google.android.exoplayer2.trackselection.MappingTrackSelector.MappedTrackInfo;
						public selectTracks(param0: com.google.android.exoplayer2.trackselection.MappingTrackSelector.MappedTrackInfo, param1: androidNative.Array<androidNative.Array<androidNative.Array<number>>>, param2: androidNative.Array<number>, param3: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param4: com.google.android.exoplayer2.Timeline): globalAndroid.util.Pair<androidNative.Array<com.google.android.exoplayer2.RendererConfiguration>,androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection>>;
					}
					export module MappingTrackSelector {
						export class MappedTrackInfo {
							public static class: java.lang.Class<com.google.android.exoplayer2.trackselection.MappingTrackSelector.MappedTrackInfo>;
							public static RENDERER_SUPPORT_NO_TRACKS: number;
							public static RENDERER_SUPPORT_UNSUPPORTED_TRACKS: number;
							public static RENDERER_SUPPORT_EXCEEDS_CAPABILITIES_TRACKS: number;
							public static RENDERER_SUPPORT_PLAYABLE_TRACKS: number;
							public getTrackGroups(param0: number): com.google.android.exoplayer2.source.TrackGroupArray;
							public getRendererType(param0: number): number;
							public getCapabilities(param0: number, param1: number, param2: number): number;
							public getRendererCount(): number;
							public getRendererSupport(param0: number): number;
							public getTypeSupport(param0: number): number;
							public getUnmappedTrackGroups(): com.google.android.exoplayer2.source.TrackGroupArray;
							public getRendererName(param0: number): string;
							public getTrackSupport(param0: number, param1: number, param2: number): number;
							public getAdaptiveSupport(param0: number, param1: number, param2: boolean): number;
							public getAdaptiveSupport(param0: number, param1: number, param2: androidNative.Array<number>): number;
						}
						export module MappedTrackInfo {
							export class RendererSupport {
								public static class: java.lang.Class<com.google.android.exoplayer2.trackselection.MappingTrackSelector.MappedTrackInfo.RendererSupport>;
								/**
								 * Constructs a new instance of the com.google.android.exoplayer2.trackselection.MappingTrackSelector$MappedTrackInfo$RendererSupport interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
								 */
								public constructor(implementation: {
								});
								public constructor();
							}
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module trackselection {
					export class RandomTrackSelection extends com.google.android.exoplayer2.trackselection.BaseTrackSelection {
						public static class: java.lang.Class<com.google.android.exoplayer2.trackselection.RandomTrackSelection>;
						public constructor(param0: com.google.android.exoplayer2.source.TrackGroup, param1: androidNative.Array<number>, param2: number);
						public onPlayWhenReadyChanged(param0: boolean): void;
						public constructor(param0: com.google.android.exoplayer2.source.TrackGroup, param1: androidNative.Array<number>, param2: number, param3: java.util.Random);
						public getSelectionData(): any;
						public getSelectedFormat(): com.google.android.exoplayer2.Format;
						public getSelectedIndexInTrackGroup(): number;
						public onPlaybackSpeed(param0: number): void;
						public getSelectionReason(): number;
						public disable(): void;
						public updateSelectedTrack(param0: number, param1: number, param2: number, param3: java.util.List<any>, param4: androidNative.Array<com.google.android.exoplayer2.source.chunk.MediaChunkIterator>): void;
						public enable(): void;
						public constructor(param0: com.google.android.exoplayer2.source.TrackGroup, param1: androidNative.Array<number>);
						public blacklist(param0: number, param1: number): boolean;
						public evaluateQueueSize(param0: number, param1: java.util.List<any>): number;
						public onRebuffer(): void;
						public shouldCancelChunkLoad(param0: number, param1: com.google.android.exoplayer2.source.chunk.Chunk, param2: java.util.List<any>): boolean;
						public getSelectedIndex(): number;
						public isBlacklisted(param0: number, param1: number): boolean;
						public onDiscontinuity(): void;
					}
					export module RandomTrackSelection {
						export class Factory extends com.google.android.exoplayer2.trackselection.ExoTrackSelection.Factory {
							public static class: java.lang.Class<com.google.android.exoplayer2.trackselection.RandomTrackSelection.Factory>;
							public constructor();
							public createTrackSelections(param0: androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection.Definition>, param1: com.google.android.exoplayer2.upstream.BandwidthMeter, param2: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param3: com.google.android.exoplayer2.Timeline): androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection>;
							public constructor(param0: number);
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module trackselection {
					export class TrackSelectionUtil {
						public static class: java.lang.Class<com.google.android.exoplayer2.trackselection.TrackSelectionUtil>;
						public static createTrackSelectionsForDefinitions(param0: androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection.Definition>, param1: com.google.android.exoplayer2.trackselection.TrackSelectionUtil.AdaptiveTrackSelectionFactory): androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection>;
						public static createFallbackOptions(param0: com.google.android.exoplayer2.trackselection.ExoTrackSelection): com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy.FallbackOptions;
						public static updateParametersWithOverride(param0: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters, param1: number, param2: com.google.android.exoplayer2.source.TrackGroupArray, param3: boolean, param4: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.SelectionOverride): com.google.android.exoplayer2.trackselection.DefaultTrackSelector.Parameters;
					}
					export module TrackSelectionUtil {
						export class AdaptiveTrackSelectionFactory {
							public static class: java.lang.Class<com.google.android.exoplayer2.trackselection.TrackSelectionUtil.AdaptiveTrackSelectionFactory>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.trackselection.TrackSelectionUtil$AdaptiveTrackSelectionFactory interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								createAdaptiveTrackSelection(param0: com.google.android.exoplayer2.trackselection.ExoTrackSelection.Definition): com.google.android.exoplayer2.trackselection.ExoTrackSelection;
							});
							public constructor();
							public createAdaptiveTrackSelection(param0: com.google.android.exoplayer2.trackselection.ExoTrackSelection.Definition): com.google.android.exoplayer2.trackselection.ExoTrackSelection;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module trackselection {
					export abstract class TrackSelector {
						public static class: java.lang.Class<com.google.android.exoplayer2.trackselection.TrackSelector>;
						public isSetParametersSupported(): boolean;
						public constructor();
						public selectTracks(param0: androidNative.Array<com.google.android.exoplayer2.RendererCapabilities>, param1: com.google.android.exoplayer2.source.TrackGroupArray, param2: com.google.android.exoplayer2.source.MediaSource.MediaPeriodId, param3: com.google.android.exoplayer2.Timeline): com.google.android.exoplayer2.trackselection.TrackSelectorResult;
						public setParameters(param0: com.google.android.exoplayer2.trackselection.TrackSelectionParameters): void;
						public invalidate(): void;
						public getBandwidthMeter(): com.google.android.exoplayer2.upstream.BandwidthMeter;
						public onSelectionActivated(param0: any): void;
						public init(param0: com.google.android.exoplayer2.trackselection.TrackSelector.InvalidationListener, param1: com.google.android.exoplayer2.upstream.BandwidthMeter): void;
						public getParameters(): com.google.android.exoplayer2.trackselection.TrackSelectionParameters;
					}
					export module TrackSelector {
						export class InvalidationListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.trackselection.TrackSelector.InvalidationListener>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.trackselection.TrackSelector$InvalidationListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onTrackSelectionsInvalidated(): void;
							});
							public constructor();
							public onTrackSelectionsInvalidated(): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module trackselection {
					export class TrackSelectorResult {
						public static class: java.lang.Class<com.google.android.exoplayer2.trackselection.TrackSelectorResult>;
						public length: number;
						public rendererConfigurations: androidNative.Array<com.google.android.exoplayer2.RendererConfiguration>;
						public selections: androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection>;
						public tracksInfo: com.google.android.exoplayer2.TracksInfo;
						public info: any;
						public isRendererEnabled(param0: number): boolean;
						/** @deprecated */
						public constructor(param0: androidNative.Array<com.google.android.exoplayer2.RendererConfiguration>, param1: androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection>, param2: any);
						public isEquivalent(param0: com.google.android.exoplayer2.trackselection.TrackSelectorResult): boolean;
						public isEquivalent(param0: com.google.android.exoplayer2.trackselection.TrackSelectorResult, param1: number): boolean;
						public constructor(param0: androidNative.Array<com.google.android.exoplayer2.RendererConfiguration>, param1: androidNative.Array<com.google.android.exoplayer2.trackselection.ExoTrackSelection>, param2: com.google.android.exoplayer2.TracksInfo, param3: any);
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module ui {
					export class AspectRatioFrameLayout {
						public static class: java.lang.Class<com.google.android.exoplayer2.ui.AspectRatioFrameLayout>;
						public static RESIZE_MODE_FIT: number;
						public static RESIZE_MODE_FIXED_WIDTH: number;
						public static RESIZE_MODE_FIXED_HEIGHT: number;
						public static RESIZE_MODE_FILL: number;
						public static RESIZE_MODE_ZOOM: number;
						public setAspectRatio(param0: number): void;
						public constructor(param0: globalAndroid.content.Context);
						public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet);
						public setAspectRatioListener(param0: com.google.android.exoplayer2.ui.AspectRatioFrameLayout.AspectRatioListener): void;
						public setResizeMode(param0: number): void;
						public getResizeMode(): number;
						public onMeasure(param0: number, param1: number): void;
					}
					export module AspectRatioFrameLayout {
						export class AspectRatioListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.AspectRatioFrameLayout.AspectRatioListener>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.ui.AspectRatioFrameLayout$AspectRatioListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onAspectRatioUpdated(param0: number, param1: number, param2: boolean): void;
							});
							public constructor();
							public onAspectRatioUpdated(param0: number, param1: number, param2: boolean): void;
						}
						export class AspectRatioUpdateDispatcher {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.AspectRatioFrameLayout.AspectRatioUpdateDispatcher>;
							public scheduleUpdate(param0: number, param1: number, param2: boolean): void;
							public run(): void;
						}
						export class ResizeMode {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.AspectRatioFrameLayout.ResizeMode>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.ui.AspectRatioFrameLayout$ResizeMode interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
							});
							public constructor();
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module ui {
					export class BuildConfig {
						public static class: java.lang.Class<com.google.android.exoplayer2.ui.BuildConfig>;
						public static DEBUG: boolean;
						public static LIBRARY_PACKAGE_NAME: string;
						public static BUILD_TYPE: string;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module ui {
					export class CanvasSubtitleOutput implements com.google.android.exoplayer2.ui.SubtitleView.Output {
						public static class: java.lang.Class<com.google.android.exoplayer2.ui.CanvasSubtitleOutput>;
						public constructor(param0: globalAndroid.content.Context);
						public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet);
						public update(param0: java.util.List<com.google.android.exoplayer2.text.Cue>, param1: com.google.android.exoplayer2.ui.CaptionStyleCompat, param2: number, param3: number, param4: number): void;
						public dispatchDraw(param0: globalAndroid.graphics.Canvas): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module ui {
					export class CaptionStyleCompat {
						public static class: java.lang.Class<com.google.android.exoplayer2.ui.CaptionStyleCompat>;
						public static EDGE_TYPE_NONE: number;
						public static EDGE_TYPE_OUTLINE: number;
						public static EDGE_TYPE_DROP_SHADOW: number;
						public static EDGE_TYPE_RAISED: number;
						public static EDGE_TYPE_DEPRESSED: number;
						public static USE_TRACK_COLOR_SETTINGS: number;
						public static DEFAULT: com.google.android.exoplayer2.ui.CaptionStyleCompat;
						public foregroundColor: number;
						public backgroundColor: number;
						public windowColor: number;
						public edgeType: number;
						public edgeColor: number;
						public typeface: globalAndroid.graphics.Typeface;
						public static createFromCaptionStyle(param0: globalAndroid.view.accessibility.CaptioningManager.CaptionStyle): com.google.android.exoplayer2.ui.CaptionStyleCompat;
						public constructor(param0: number, param1: number, param2: number, param3: number, param4: number, param5: globalAndroid.graphics.Typeface);
					}
					export module CaptionStyleCompat {
						export class EdgeType {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.CaptionStyleCompat.EdgeType>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.ui.CaptionStyleCompat$EdgeType interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
							});
							public constructor();
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module ui {
					export class DefaultMediaDescriptionAdapter extends com.google.android.exoplayer2.ui.PlayerNotificationManager.MediaDescriptionAdapter {
						public static class: java.lang.Class<com.google.android.exoplayer2.ui.DefaultMediaDescriptionAdapter>;
						public createCurrentContentIntent(param0: com.google.android.exoplayer2.Player): globalAndroid.app.PendingIntent;
						public getCurrentSubText(param0: com.google.android.exoplayer2.Player): string;
						public getCurrentContentText(param0: com.google.android.exoplayer2.Player): string;
						public getCurrentLargeIcon(param0: com.google.android.exoplayer2.Player, param1: com.google.android.exoplayer2.ui.PlayerNotificationManager.BitmapCallback): globalAndroid.graphics.Bitmap;
						public constructor(param0: globalAndroid.app.PendingIntent);
						public getCurrentContentTitle(param0: com.google.android.exoplayer2.Player): string;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module ui {
					export class DefaultTimeBar implements com.google.android.exoplayer2.ui.TimeBar {
						public static class: java.lang.Class<com.google.android.exoplayer2.ui.DefaultTimeBar>;
						public static DEFAULT_BAR_HEIGHT_DP: number;
						public static DEFAULT_TOUCH_TARGET_HEIGHT_DP: number;
						public static DEFAULT_AD_MARKER_WIDTH_DP: number;
						public static DEFAULT_SCRUBBER_ENABLED_SIZE_DP: number;
						public static DEFAULT_SCRUBBER_DISABLED_SIZE_DP: number;
						public static DEFAULT_SCRUBBER_DRAGGED_SIZE_DP: number;
						public static DEFAULT_PLAYED_COLOR: number;
						public static DEFAULT_UNPLAYED_COLOR: number;
						public static DEFAULT_BUFFERED_COLOR: number;
						public static DEFAULT_SCRUBBER_COLOR: number;
						public static DEFAULT_AD_MARKER_COLOR: number;
						public static DEFAULT_PLAYED_AD_MARKER_COLOR: number;
						public static BAR_GRAVITY_CENTER: number;
						public static BAR_GRAVITY_BOTTOM: number;
						public constructor(param0: globalAndroid.content.Context);
						public setKeyCountIncrement(param0: number): void;
						public onInitializeAccessibilityNodeInfo(param0: globalAndroid.view.accessibility.AccessibilityNodeInfo): void;
						public setAdGroupTimesMs(param0: androidNative.Array<number>, param1: androidNative.Array<boolean>, param2: number): void;
						public setKeyTimeIncrement(param0: number): void;
						public setPlayedColor(param0: number): void;
						public onLayout(param0: boolean, param1: number, param2: number, param3: number, param4: number): void;
						public drawableStateChanged(): void;
						public onInitializeAccessibilityEvent(param0: globalAndroid.view.accessibility.AccessibilityEvent): void;
						public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet, param2: number, param3: globalAndroid.util.AttributeSet, param4: number);
						public hideScrubber(param0: boolean): void;
						public setBufferedColor(param0: number): void;
						public removeListener(param0: com.google.android.exoplayer2.ui.TimeBar.OnScrubListener): void;
						public showScrubber(): void;
						public setBufferedPosition(param0: number): void;
						public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet);
						public setScrubberColor(param0: number): void;
						public setUnplayedColor(param0: number): void;
						public onMeasure(param0: number, param1: number): void;
						public showScrubber(param0: number): void;
						public hideScrubber(param0: number): void;
						public setPlayedAdMarkerColor(param0: number): void;
						public onTouchEvent(param0: globalAndroid.view.MotionEvent): boolean;
						public onRtlPropertiesChanged(param0: number): void;
						public onFocusChanged(param0: boolean, param1: number, param2: globalAndroid.graphics.Rect): void;
						public performAccessibilityAction(param0: number, param1: globalAndroid.os.Bundle): boolean;
						public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet, param2: number, param3: globalAndroid.util.AttributeSet);
						public setEnabled(param0: boolean): void;
						public jumpDrawablesToCurrentState(): void;
						public getPreferredUpdateDelay(): number;
						public onDraw(param0: globalAndroid.graphics.Canvas): void;
						public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet, param2: number);
						public onKeyDown(param0: number, param1: globalAndroid.view.KeyEvent): boolean;
						public setAdMarkerColor(param0: number): void;
						public setPosition(param0: number): void;
						public setDuration(param0: number): void;
						public addListener(param0: com.google.android.exoplayer2.ui.TimeBar.OnScrubListener): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module ui {
					export class DefaultTrackNameProvider extends com.google.android.exoplayer2.ui.TrackNameProvider {
						public static class: java.lang.Class<com.google.android.exoplayer2.ui.DefaultTrackNameProvider>;
						public getTrackName(param0: com.google.android.exoplayer2.Format): string;
						public constructor(param0: globalAndroid.content.res.Resources);
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module ui {
					export class DownloadNotificationHelper {
						public static class: java.lang.Class<com.google.android.exoplayer2.ui.DownloadNotificationHelper>;
						/** @deprecated */
						public buildProgressNotification(param0: globalAndroid.content.Context, param1: number, param2: globalAndroid.app.PendingIntent, param3: string, param4: java.util.List<com.google.android.exoplayer2.offline.Download>): globalAndroid.app.Notification;
						public buildProgressNotification(param0: globalAndroid.content.Context, param1: number, param2: globalAndroid.app.PendingIntent, param3: string, param4: java.util.List<com.google.android.exoplayer2.offline.Download>, param5: number): globalAndroid.app.Notification;
						public buildDownloadFailedNotification(param0: globalAndroid.content.Context, param1: number, param2: globalAndroid.app.PendingIntent, param3: string): globalAndroid.app.Notification;
						public constructor(param0: globalAndroid.content.Context, param1: string);
						public buildDownloadCompletedNotification(param0: globalAndroid.content.Context, param1: number, param2: globalAndroid.app.PendingIntent, param3: string): globalAndroid.app.Notification;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module ui {
					export class HtmlUtils {
						public static class: java.lang.Class<com.google.android.exoplayer2.ui.HtmlUtils>;
						public static cssAllClassDescendantsSelector(param0: string): string;
						public static toCssRgba(param0: number): string;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module ui {
					export class PlayerControlView {
						public static class: java.lang.Class<com.google.android.exoplayer2.ui.PlayerControlView>;
						public static DEFAULT_SHOW_TIMEOUT_MS: number;
						public static DEFAULT_REPEAT_TOGGLE_MODES: number;
						public static DEFAULT_TIME_BAR_MIN_UPDATE_INTERVAL_MS: number;
						public static MAX_WINDOWS_FOR_MULTI_WINDOW_TIME_BAR: number;
						public getPlayer(): com.google.android.exoplayer2.Player;
						public constructor(param0: globalAndroid.content.Context);
						public setRepeatToggleModes(param0: number): void;
						public getShowShuffleButton(): boolean;
						public removeVisibilityListener(param0: com.google.android.exoplayer2.ui.PlayerControlView.VisibilityListener): void;
						public onAttachedToWindow(): void;
						public setShowTimeoutMs(param0: number): void;
						public setProgressUpdateListener(param0: com.google.android.exoplayer2.ui.PlayerControlView.ProgressUpdateListener): void;
						public setShowFastForwardButton(param0: boolean): void;
						public setTimeBarMinUpdateInterval(param0: number): void;
						public addVisibilityListener(param0: com.google.android.exoplayer2.ui.PlayerControlView.VisibilityListener): void;
						public setExtraAdGroupMarkers(param0: androidNative.Array<number>, param1: androidNative.Array<boolean>): void;
						public isVisible(): boolean;
						public dispatchMediaKeyEvent(param0: globalAndroid.view.KeyEvent): boolean;
						public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet);
						public dispatchKeyEvent(param0: globalAndroid.view.KeyEvent): boolean;
						public getRepeatToggleModes(): number;
						public getShowVrButton(): boolean;
						public dispatchTouchEvent(param0: globalAndroid.view.MotionEvent): boolean;
						public setVrButtonListener(param0: globalAndroid.view.View.OnClickListener): void;
						public setShowVrButton(param0: boolean): void;
						public setShowPreviousButton(param0: boolean): void;
						public setShowNextButton(param0: boolean): void;
						public onDetachedFromWindow(): void;
						public setShowRewindButton(param0: boolean): void;
						public hide(): void;
						public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet, param2: number, param3: globalAndroid.util.AttributeSet);
						public setShowMultiWindowTimeBar(param0: boolean): void;
						public setPlayer(param0: com.google.android.exoplayer2.Player): void;
						public setShowShuffleButton(param0: boolean): void;
						public show(): void;
						public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet, param2: number);
						public getShowTimeoutMs(): number;
					}
					export module PlayerControlView {
						export class Api21 {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.PlayerControlView.Api21>;
							public static isAccessibilityFocused(param0: globalAndroid.view.View): boolean;
						}
						export class ComponentListener extends com.google.android.exoplayer2.ui.TimeBar.OnScrubListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.PlayerControlView.ComponentListener>;
							public onScrubStop(param0: com.google.android.exoplayer2.ui.TimeBar, param1: number, param2: boolean): void;
							public onClick(param0: globalAndroid.view.View): void;
							public onScrubMove(param0: com.google.android.exoplayer2.ui.TimeBar, param1: number): void;
							public onEvents(param0: com.google.android.exoplayer2.Player, param1: com.google.android.exoplayer2.Player.Events): void;
							public onScrubStart(param0: com.google.android.exoplayer2.ui.TimeBar, param1: number): void;
						}
						export class ProgressUpdateListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.PlayerControlView.ProgressUpdateListener>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.ui.PlayerControlView$ProgressUpdateListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onProgressUpdate(param0: number, param1: number): void;
							});
							public constructor();
							public onProgressUpdate(param0: number, param1: number): void;
						}
						export class VisibilityListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.PlayerControlView.VisibilityListener>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.ui.PlayerControlView$VisibilityListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onVisibilityChange(param0: number): void;
							});
							public constructor();
							public onVisibilityChange(param0: number): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module ui {
					export class PlayerNotificationManager {
						public static class: java.lang.Class<com.google.android.exoplayer2.ui.PlayerNotificationManager>;
						public static ACTION_PLAY: string;
						public static ACTION_PAUSE: string;
						public static ACTION_PREVIOUS: string;
						public static ACTION_NEXT: string;
						public static ACTION_FAST_FORWARD: string;
						public static ACTION_REWIND: string;
						public static ACTION_STOP: string;
						public static EXTRA_INSTANCE_ID: string;
						public setVisibility(param0: number): void;
						public setUseFastForwardAction(param0: boolean): void;
						public setUseRewindAction(param0: boolean): void;
						public setUseChronometer(param0: boolean): void;
						public setPriority(param0: number): void;
						public setSmallIcon(param0: number): void;
						public setUsePlayPauseActions(param0: boolean): void;
						public getOngoing(param0: com.google.android.exoplayer2.Player): boolean;
						public setUseRewindActionInCompactView(param0: boolean): void;
						public setColorized(param0: boolean): void;
						public constructor(param0: globalAndroid.content.Context, param1: string, param2: number, param3: com.google.android.exoplayer2.ui.PlayerNotificationManager.MediaDescriptionAdapter, param4: com.google.android.exoplayer2.ui.PlayerNotificationManager.NotificationListener, param5: com.google.android.exoplayer2.ui.PlayerNotificationManager.CustomActionReceiver, param6: number, param7: number, param8: number, param9: number, param10: number, param11: number, param12: number, param13: number, param14: string);
						public setUseFastForwardActionInCompactView(param0: boolean): void;
						public setDefaults(param0: number): void;
						public setColor(param0: number): void;
						public getActions(param0: com.google.android.exoplayer2.Player): java.util.List<string>;
						public setUsePreviousAction(param0: boolean): void;
						public setUsePreviousActionInCompactView(param0: boolean): void;
						public setPlayer(param0: com.google.android.exoplayer2.Player): void;
						public setUseNextAction(param0: boolean): void;
						public setUseNextActionInCompactView(param0: boolean): void;
						public invalidate(): void;
						public setUseStopAction(param0: boolean): void;
						public getActionIndicesForCompactView(param0: java.util.List<string>, param1: com.google.android.exoplayer2.Player): androidNative.Array<number>;
						public createNotification(param0: com.google.android.exoplayer2.Player, param1: androidx.core.app.NotificationCompat.Builder, param2: boolean, param3: globalAndroid.graphics.Bitmap): androidx.core.app.NotificationCompat.Builder;
						public setBadgeIconType(param0: number): void;
						public setMediaSessionToken(param0: globalAndroid.support.v4.media.session.MediaSessionCompat.Token): void;
					}
					export module PlayerNotificationManager {
						export class BitmapCallback {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.PlayerNotificationManager.BitmapCallback>;
							public onBitmap(param0: globalAndroid.graphics.Bitmap): void;
						}
						export class Builder {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.PlayerNotificationManager.Builder>;
							public context: globalAndroid.content.Context;
							public notificationId: number;
							public channelId: string;
							public notificationListener: com.google.android.exoplayer2.ui.PlayerNotificationManager.NotificationListener;
							public customActionReceiver: com.google.android.exoplayer2.ui.PlayerNotificationManager.CustomActionReceiver;
							public mediaDescriptionAdapter: com.google.android.exoplayer2.ui.PlayerNotificationManager.MediaDescriptionAdapter;
							public channelNameResourceId: number;
							public channelDescriptionResourceId: number;
							public channelImportance: number;
							public smallIconResourceId: number;
							public rewindActionIconResourceId: number;
							public playActionIconResourceId: number;
							public pauseActionIconResourceId: number;
							public stopActionIconResourceId: number;
							public fastForwardActionIconResourceId: number;
							public previousActionIconResourceId: number;
							public nextActionIconResourceId: number;
							public groupKey: string;
							public build(): com.google.android.exoplayer2.ui.PlayerNotificationManager;
							public setChannelDescriptionResourceId(param0: number): com.google.android.exoplayer2.ui.PlayerNotificationManager.Builder;
							public setNextActionIconResourceId(param0: number): com.google.android.exoplayer2.ui.PlayerNotificationManager.Builder;
							public setFastForwardActionIconResourceId(param0: number): com.google.android.exoplayer2.ui.PlayerNotificationManager.Builder;
							/** @deprecated */
							public constructor(param0: globalAndroid.content.Context, param1: number, param2: string, param3: com.google.android.exoplayer2.ui.PlayerNotificationManager.MediaDescriptionAdapter);
							public setCustomActionReceiver(param0: com.google.android.exoplayer2.ui.PlayerNotificationManager.CustomActionReceiver): com.google.android.exoplayer2.ui.PlayerNotificationManager.Builder;
							public setStopActionIconResourceId(param0: number): com.google.android.exoplayer2.ui.PlayerNotificationManager.Builder;
							public setNotificationListener(param0: com.google.android.exoplayer2.ui.PlayerNotificationManager.NotificationListener): com.google.android.exoplayer2.ui.PlayerNotificationManager.Builder;
							public setSmallIconResourceId(param0: number): com.google.android.exoplayer2.ui.PlayerNotificationManager.Builder;
							public setGroup(param0: string): com.google.android.exoplayer2.ui.PlayerNotificationManager.Builder;
							public setMediaDescriptionAdapter(param0: com.google.android.exoplayer2.ui.PlayerNotificationManager.MediaDescriptionAdapter): com.google.android.exoplayer2.ui.PlayerNotificationManager.Builder;
							public setPreviousActionIconResourceId(param0: number): com.google.android.exoplayer2.ui.PlayerNotificationManager.Builder;
							public constructor(param0: globalAndroid.content.Context, param1: number, param2: string);
							public setRewindActionIconResourceId(param0: number): com.google.android.exoplayer2.ui.PlayerNotificationManager.Builder;
							public setChannelNameResourceId(param0: number): com.google.android.exoplayer2.ui.PlayerNotificationManager.Builder;
							public setChannelImportance(param0: number): com.google.android.exoplayer2.ui.PlayerNotificationManager.Builder;
							public setPlayActionIconResourceId(param0: number): com.google.android.exoplayer2.ui.PlayerNotificationManager.Builder;
							public setPauseActionIconResourceId(param0: number): com.google.android.exoplayer2.ui.PlayerNotificationManager.Builder;
						}
						export class CustomActionReceiver {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.PlayerNotificationManager.CustomActionReceiver>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.ui.PlayerNotificationManager$CustomActionReceiver interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								createCustomActions(param0: globalAndroid.content.Context, param1: number): java.util.Map<string,androidx.core.app.NotificationCompat.Action>;
								getCustomActions(param0: com.google.android.exoplayer2.Player): java.util.List<string>;
								onCustomAction(param0: com.google.android.exoplayer2.Player, param1: string, param2: globalAndroid.content.Intent): void;
							});
							public constructor();
							public getCustomActions(param0: com.google.android.exoplayer2.Player): java.util.List<string>;
							public onCustomAction(param0: com.google.android.exoplayer2.Player, param1: string, param2: globalAndroid.content.Intent): void;
							public createCustomActions(param0: globalAndroid.content.Context, param1: number): java.util.Map<string,androidx.core.app.NotificationCompat.Action>;
						}
						export class MediaDescriptionAdapter {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.PlayerNotificationManager.MediaDescriptionAdapter>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.ui.PlayerNotificationManager$MediaDescriptionAdapter interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								getCurrentContentTitle(param0: com.google.android.exoplayer2.Player): string;
								createCurrentContentIntent(param0: com.google.android.exoplayer2.Player): globalAndroid.app.PendingIntent;
								getCurrentContentText(param0: com.google.android.exoplayer2.Player): string;
								getCurrentSubText(param0: com.google.android.exoplayer2.Player): string;
								getCurrentLargeIcon(param0: com.google.android.exoplayer2.Player, param1: com.google.android.exoplayer2.ui.PlayerNotificationManager.BitmapCallback): globalAndroid.graphics.Bitmap;
							});
							public constructor();
							public getCurrentContentText(param0: com.google.android.exoplayer2.Player): string;
							public getCurrentLargeIcon(param0: com.google.android.exoplayer2.Player, param1: com.google.android.exoplayer2.ui.PlayerNotificationManager.BitmapCallback): globalAndroid.graphics.Bitmap;
							public getCurrentContentTitle(param0: com.google.android.exoplayer2.Player): string;
							public getCurrentSubText(param0: com.google.android.exoplayer2.Player): string;
							public createCurrentContentIntent(param0: com.google.android.exoplayer2.Player): globalAndroid.app.PendingIntent;
						}
						export class NotificationBroadcastReceiver {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.PlayerNotificationManager.NotificationBroadcastReceiver>;
							public onReceive(param0: globalAndroid.content.Context, param1: globalAndroid.content.Intent): void;
						}
						export class NotificationListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.PlayerNotificationManager.NotificationListener>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.ui.PlayerNotificationManager$NotificationListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onNotificationCancelled(param0: number, param1: boolean): void;
								onNotificationPosted(param0: number, param1: globalAndroid.app.Notification, param2: boolean): void;
							});
							public constructor();
							public onNotificationPosted(param0: number, param1: globalAndroid.app.Notification, param2: boolean): void;
							public onNotificationCancelled(param0: number, param1: boolean): void;
						}
						export class PlayerListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.PlayerNotificationManager.PlayerListener>;
							public onEvents(param0: com.google.android.exoplayer2.Player, param1: com.google.android.exoplayer2.Player.Events): void;
						}
						export class Priority {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.PlayerNotificationManager.Priority>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.ui.PlayerNotificationManager$Priority interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
							});
							public constructor();
						}
						export class Visibility {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.PlayerNotificationManager.Visibility>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.ui.PlayerNotificationManager$Visibility interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
							});
							public constructor();
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module ui {
					export class PlayerView {
						public static class: java.lang.Class<com.google.android.exoplayer2.ui.PlayerView>;
						public static SHOW_BUFFERING_NEVER: number;
						public static SHOW_BUFFERING_WHEN_PLAYING: number;
						public static SHOW_BUFFERING_ALWAYS: number;
						public isControllerVisible(): boolean;
						public getPlayer(): com.google.android.exoplayer2.Player;
						public constructor(param0: globalAndroid.content.Context);
						public static switchTargetView(param0: com.google.android.exoplayer2.Player, param1: com.google.android.exoplayer2.ui.PlayerView, param2: com.google.android.exoplayer2.ui.PlayerView): void;
						public setControllerHideOnTouch(param0: boolean): void;
						public getControllerHideOnTouch(): boolean;
						public performClick(): boolean;
						public onPause(): void;
						public setControllerAutoShow(param0: boolean): void;
						public getDefaultArtwork(): globalAndroid.graphics.drawable.Drawable;
						public getOverlayFrameLayout(): globalAndroid.widget.FrameLayout;
						public setResizeMode(param0: number): void;
						public dispatchKeyEvent(param0: globalAndroid.view.KeyEvent): boolean;
						public setShowBuffering(param0: number): void;
						public onTouchEvent(param0: globalAndroid.view.MotionEvent): boolean;
						public getControllerShowTimeoutMs(): number;
						public onResume(): void;
						public setShowPreviousButton(param0: boolean): void;
						public setShowRewindButton(param0: boolean): void;
						public getSubtitleView(): com.google.android.exoplayer2.ui.SubtitleView;
						public setCustomErrorMessage(param0: string): void;
						public setShowMultiWindowTimeBar(param0: boolean): void;
						public setControllerVisibilityListener(param0: com.google.android.exoplayer2.ui.PlayerControlView.VisibilityListener): void;
						public onContentAspectRatioChanged(param0: com.google.android.exoplayer2.ui.AspectRatioFrameLayout, param1: number): void;
						public setRepeatToggleModes(param0: number): void;
						public setVisibility(param0: number): void;
						public setControllerShowTimeoutMs(param0: number): void;
						public setUseArtwork(param0: boolean): void;
						public setShowFastForwardButton(param0: boolean): void;
						public getVideoSurfaceView(): globalAndroid.view.View;
						public setExtraAdGroupMarkers(param0: androidNative.Array<number>, param1: androidNative.Array<boolean>): void;
						public getUseController(): boolean;
						public dispatchMediaKeyEvent(param0: globalAndroid.view.KeyEvent): boolean;
						public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet);
						public setAspectRatioListener(param0: com.google.android.exoplayer2.ui.AspectRatioFrameLayout.AspectRatioListener): void;
						public getAdOverlayInfos(): java.util.List<com.google.android.exoplayer2.ui.AdOverlayInfo>;
						public setShutterBackgroundColor(param0: number): void;
						public setUseController(param0: boolean): void;
						public setShowNextButton(param0: boolean): void;
						public setErrorMessageProvider(param0: com.google.android.exoplayer2.util.ErrorMessageProvider<any>): void;
						public getResizeMode(): number;
						public setKeepContentOnPlayerReset(param0: boolean): void;
						public getUseArtwork(): boolean;
						public setPlayer(param0: com.google.android.exoplayer2.Player): void;
						public setShowShuffleButton(param0: boolean): void;
						public setControllerHideDuringAds(param0: boolean): void;
						public getAdViewGroup(): globalAndroid.view.ViewGroup;
						public hideController(): void;
						public getControllerAutoShow(): boolean;
						public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet, param2: number);
						public setDefaultArtwork(param0: globalAndroid.graphics.drawable.Drawable): void;
						public onTrackballEvent(param0: globalAndroid.view.MotionEvent): boolean;
						public showController(): void;
					}
					export module PlayerView {
						export class ComponentListener extends com.google.android.exoplayer2.ui.PlayerControlView.VisibilityListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.PlayerView.ComponentListener>;
							public onPositionDiscontinuity(param0: com.google.android.exoplayer2.Player.PositionInfo, param1: com.google.android.exoplayer2.Player.PositionInfo, param2: number): void;
							public onVisibilityChange(param0: number): void;
							public onCues(param0: java.util.List<com.google.android.exoplayer2.text.Cue>): void;
							public onClick(param0: globalAndroid.view.View): void;
							public onPlaybackStateChanged(param0: number): void;
							public constructor(param0: com.google.android.exoplayer2.ui.PlayerView);
							public onRenderedFirstFrame(): void;
							public onPlayWhenReadyChanged(param0: boolean, param1: number): void;
							public onVideoSizeChanged(param0: com.google.android.exoplayer2.video.VideoSize): void;
							public onTracksInfoChanged(param0: com.google.android.exoplayer2.TracksInfo): void;
							public onLayoutChange(param0: globalAndroid.view.View, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number): void;
						}
						export class ShowBuffering {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.PlayerView.ShowBuffering>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.ui.PlayerView$ShowBuffering interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
							});
							public constructor();
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module ui {
					export class SpannedToHtmlConverter {
						public static class: java.lang.Class<com.google.android.exoplayer2.ui.SpannedToHtmlConverter>;
						public static convert(param0: string, param1: number): com.google.android.exoplayer2.ui.SpannedToHtmlConverter.HtmlAndCss;
					}
					export module SpannedToHtmlConverter {
						export class HtmlAndCss {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.SpannedToHtmlConverter.HtmlAndCss>;
							public html: string;
							public cssRuleSets: java.util.Map<string,string>;
						}
						export class SpanInfo {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.SpannedToHtmlConverter.SpanInfo>;
							public start: number;
							public end: number;
							public openingTag: string;
							public closingTag: string;
						}
						export class Transition {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.SpannedToHtmlConverter.Transition>;
							public constructor();
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module ui {
					export class StyledPlayerControlView {
						public static class: java.lang.Class<com.google.android.exoplayer2.ui.StyledPlayerControlView>;
						public static DEFAULT_SHOW_TIMEOUT_MS: number;
						public static DEFAULT_REPEAT_TOGGLE_MODES: number;
						public static DEFAULT_TIME_BAR_MIN_UPDATE_INTERVAL_MS: number;
						public static MAX_WINDOWS_FOR_MULTI_WINDOW_TIME_BAR: number;
						public setAnimationEnabled(param0: boolean): void;
						public getPlayer(): com.google.android.exoplayer2.Player;
						public constructor(param0: globalAndroid.content.Context);
						public setRepeatToggleModes(param0: number): void;
						public getShowShuffleButton(): boolean;
						public onAttachedToWindow(): void;
						public addVisibilityListener(param0: com.google.android.exoplayer2.ui.StyledPlayerControlView.VisibilityListener): void;
						public setShowTimeoutMs(param0: number): void;
						public setShowFastForwardButton(param0: boolean): void;
						public setTimeBarMinUpdateInterval(param0: number): void;
						public onLayout(param0: boolean, param1: number, param2: number, param3: number, param4: number): void;
						public setShowSubtitleButton(param0: boolean): void;
						public setExtraAdGroupMarkers(param0: androidNative.Array<number>, param1: androidNative.Array<boolean>): void;
						public isVisible(): boolean;
						public dispatchMediaKeyEvent(param0: globalAndroid.view.KeyEvent): boolean;
						public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet);
						public setOnFullScreenModeChangedListener(param0: com.google.android.exoplayer2.ui.StyledPlayerControlView.OnFullScreenModeChangedListener): void;
						public dispatchKeyEvent(param0: globalAndroid.view.KeyEvent): boolean;
						public getRepeatToggleModes(): number;
						public getShowVrButton(): boolean;
						public setProgressUpdateListener(param0: com.google.android.exoplayer2.ui.StyledPlayerControlView.ProgressUpdateListener): void;
						public isAnimationEnabled(): boolean;
						public setVrButtonListener(param0: globalAndroid.view.View.OnClickListener): void;
						public setShowVrButton(param0: boolean): void;
						public setShowPreviousButton(param0: boolean): void;
						public setShowNextButton(param0: boolean): void;
						public onDetachedFromWindow(): void;
						public setShowRewindButton(param0: boolean): void;
						public hide(): void;
						public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet, param2: number, param3: globalAndroid.util.AttributeSet);
						public isFullyVisible(): boolean;
						public getShowSubtitleButton(): boolean;
						public setShowMultiWindowTimeBar(param0: boolean): void;
						public setPlayer(param0: com.google.android.exoplayer2.Player): void;
						public setShowShuffleButton(param0: boolean): void;
						public show(): void;
						public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet, param2: number);
						public removeVisibilityListener(param0: com.google.android.exoplayer2.ui.StyledPlayerControlView.VisibilityListener): void;
						public hideImmediately(): void;
						public getShowTimeoutMs(): number;
					}
					export module StyledPlayerControlView {
						export class AudioTrackSelectionAdapter extends com.google.android.exoplayer2.ui.StyledPlayerControlView.TrackSelectionAdapter {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.StyledPlayerControlView.AudioTrackSelectionAdapter>;
							public onBindViewHolderAtZeroPosition(param0: com.google.android.exoplayer2.ui.StyledPlayerControlView.SubSettingViewHolder): void;
							public init(param0: java.util.List<com.google.android.exoplayer2.ui.StyledPlayerControlView.TrackInformation>): void;
							public onTrackSelection(param0: string): void;
						}
						export class ComponentListener extends com.google.android.exoplayer2.ui.TimeBar.OnScrubListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.StyledPlayerControlView.ComponentListener>;
							public onScrubStop(param0: com.google.android.exoplayer2.ui.TimeBar, param1: number, param2: boolean): void;
							public onClick(param0: globalAndroid.view.View): void;
							public onScrubMove(param0: com.google.android.exoplayer2.ui.TimeBar, param1: number): void;
							public onDismiss(): void;
							public onEvents(param0: com.google.android.exoplayer2.Player, param1: com.google.android.exoplayer2.Player.Events): void;
							public onScrubStart(param0: com.google.android.exoplayer2.ui.TimeBar, param1: number): void;
						}
						export class OnFullScreenModeChangedListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.StyledPlayerControlView.OnFullScreenModeChangedListener>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.ui.StyledPlayerControlView$OnFullScreenModeChangedListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onFullScreenModeChanged(param0: boolean): void;
							});
							public constructor();
							public onFullScreenModeChanged(param0: boolean): void;
						}
						export class PlaybackSpeedAdapter extends androidx.recyclerview.widget.RecyclerView.Adapter<com.google.android.exoplayer2.ui.StyledPlayerControlView.SubSettingViewHolder> {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.StyledPlayerControlView.PlaybackSpeedAdapter>;
							public updateSelectedIndex(param0: number): void;
							public onCreateViewHolder(param0: globalAndroid.view.ViewGroup, param1: number): com.google.android.exoplayer2.ui.StyledPlayerControlView.SubSettingViewHolder;
							public getItemCount(): number;
							public getSelectedText(): string;
							public constructor(param0: com.google.android.exoplayer2.ui.StyledPlayerControlView, param1: androidNative.Array<string>, param2: androidNative.Array<number>);
							public onBindViewHolder(param0: com.google.android.exoplayer2.ui.StyledPlayerControlView.SubSettingViewHolder, param1: number): void;
						}
						export class ProgressUpdateListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.StyledPlayerControlView.ProgressUpdateListener>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.ui.StyledPlayerControlView$ProgressUpdateListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onProgressUpdate(param0: number, param1: number): void;
							});
							public constructor();
							public onProgressUpdate(param0: number, param1: number): void;
						}
						export class SettingViewHolder {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.StyledPlayerControlView.SettingViewHolder>;
							public constructor(param0: com.google.android.exoplayer2.ui.StyledPlayerControlView, param1: globalAndroid.view.View);
						}
						export class SettingsAdapter extends androidx.recyclerview.widget.RecyclerView.Adapter<com.google.android.exoplayer2.ui.StyledPlayerControlView.SettingViewHolder> {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.StyledPlayerControlView.SettingsAdapter>;
							public getItemId(param0: number): number;
							public constructor(param0: com.google.android.exoplayer2.ui.StyledPlayerControlView, param1: androidNative.Array<string>, param2: androidNative.Array<globalAndroid.graphics.drawable.Drawable>);
							public onCreateViewHolder(param0: globalAndroid.view.ViewGroup, param1: number): com.google.android.exoplayer2.ui.StyledPlayerControlView.SettingViewHolder;
							public setSubTextAtPosition(param0: number, param1: string): void;
							public getItemCount(): number;
							public onBindViewHolder(param0: com.google.android.exoplayer2.ui.StyledPlayerControlView.SettingViewHolder, param1: number): void;
						}
						export class SubSettingViewHolder {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.StyledPlayerControlView.SubSettingViewHolder>;
							public textView: globalAndroid.widget.TextView;
							public checkView: globalAndroid.view.View;
							public constructor(param0: globalAndroid.view.View);
						}
						export class TextTrackSelectionAdapter extends com.google.android.exoplayer2.ui.StyledPlayerControlView.TrackSelectionAdapter {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.StyledPlayerControlView.TextTrackSelectionAdapter>;
							public onBindViewHolderAtZeroPosition(param0: com.google.android.exoplayer2.ui.StyledPlayerControlView.SubSettingViewHolder): void;
							public init(param0: java.util.List<com.google.android.exoplayer2.ui.StyledPlayerControlView.TrackInformation>): void;
							public onTrackSelection(param0: string): void;
							public onBindViewHolder(param0: com.google.android.exoplayer2.ui.StyledPlayerControlView.SubSettingViewHolder, param1: number): void;
						}
						export class TrackInformation {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.StyledPlayerControlView.TrackInformation>;
							public trackGroupInfo: com.google.android.exoplayer2.TracksInfo.TrackGroupInfo;
							public trackIndex: number;
							public trackName: string;
							public constructor(param0: com.google.android.exoplayer2.TracksInfo, param1: number, param2: number, param3: string);
							public isSelected(): boolean;
						}
						export abstract class TrackSelectionAdapter extends androidx.recyclerview.widget.RecyclerView.Adapter<com.google.android.exoplayer2.ui.StyledPlayerControlView.SubSettingViewHolder> {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.StyledPlayerControlView.TrackSelectionAdapter>;
							public tracks: java.util.List<com.google.android.exoplayer2.ui.StyledPlayerControlView.TrackInformation>;
							public onBindViewHolderAtZeroPosition(param0: com.google.android.exoplayer2.ui.StyledPlayerControlView.SubSettingViewHolder): void;
							public constructor(param0: com.google.android.exoplayer2.ui.StyledPlayerControlView);
							public init(param0: java.util.List<com.google.android.exoplayer2.ui.StyledPlayerControlView.TrackInformation>): void;
							public onCreateViewHolder(param0: globalAndroid.view.ViewGroup, param1: number): com.google.android.exoplayer2.ui.StyledPlayerControlView.SubSettingViewHolder;
							public getItemCount(): number;
							public clear(): void;
							public onTrackSelection(param0: string): void;
							public onBindViewHolder(param0: com.google.android.exoplayer2.ui.StyledPlayerControlView.SubSettingViewHolder, param1: number): void;
						}
						export class VisibilityListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.StyledPlayerControlView.VisibilityListener>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.ui.StyledPlayerControlView$VisibilityListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onVisibilityChange(param0: number): void;
							});
							public constructor();
							public onVisibilityChange(param0: number): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module ui {
					export class StyledPlayerControlViewLayoutManager {
						public static class: java.lang.Class<com.google.android.exoplayer2.ui.StyledPlayerControlViewLayoutManager>;
						public setAnimationEnabled(param0: boolean): void;
						public isAnimationEnabled(): boolean;
						public getShowButton(param0: globalAndroid.view.View): boolean;
						public onAttachedToWindow(): void;
						public onDetachedFromWindow(): void;
						public hide(): void;
						public isFullyVisible(): boolean;
						public onLayout(param0: boolean, param1: number, param2: number, param3: number, param4: number): void;
						public resetHideCallbacks(): void;
						public constructor(param0: com.google.android.exoplayer2.ui.StyledPlayerControlView);
						public show(): void;
						public removeHideCallbacks(): void;
						public setShowButton(param0: globalAndroid.view.View, param1: boolean): void;
						public hideImmediately(): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module ui {
					export class StyledPlayerView {
						public static class: java.lang.Class<com.google.android.exoplayer2.ui.StyledPlayerView>;
						public static SHOW_BUFFERING_NEVER: number;
						public static SHOW_BUFFERING_WHEN_PLAYING: number;
						public static SHOW_BUFFERING_ALWAYS: number;
						public isControllerFullyVisible(): boolean;
						public getPlayer(): com.google.android.exoplayer2.Player;
						public constructor(param0: globalAndroid.content.Context);
						public setControllerHideOnTouch(param0: boolean): void;
						public getControllerHideOnTouch(): boolean;
						public performClick(): boolean;
						public onPause(): void;
						public setShowSubtitleButton(param0: boolean): void;
						public setControllerAutoShow(param0: boolean): void;
						public getDefaultArtwork(): globalAndroid.graphics.drawable.Drawable;
						public getOverlayFrameLayout(): globalAndroid.widget.FrameLayout;
						public setResizeMode(param0: number): void;
						public setControllerVisibilityListener(param0: com.google.android.exoplayer2.ui.StyledPlayerControlView.VisibilityListener): void;
						public dispatchKeyEvent(param0: globalAndroid.view.KeyEvent): boolean;
						public setShowBuffering(param0: number): void;
						public onTouchEvent(param0: globalAndroid.view.MotionEvent): boolean;
						public getControllerShowTimeoutMs(): number;
						public onResume(): void;
						public setShowPreviousButton(param0: boolean): void;
						public setShowRewindButton(param0: boolean): void;
						public getSubtitleView(): com.google.android.exoplayer2.ui.SubtitleView;
						public setCustomErrorMessage(param0: string): void;
						public setShowMultiWindowTimeBar(param0: boolean): void;
						public onContentAspectRatioChanged(param0: com.google.android.exoplayer2.ui.AspectRatioFrameLayout, param1: number): void;
						public static switchTargetView(param0: com.google.android.exoplayer2.Player, param1: com.google.android.exoplayer2.ui.StyledPlayerView, param2: com.google.android.exoplayer2.ui.StyledPlayerView): void;
						public setRepeatToggleModes(param0: number): void;
						public setVisibility(param0: number): void;
						public setControllerShowTimeoutMs(param0: number): void;
						public setUseArtwork(param0: boolean): void;
						public setShowFastForwardButton(param0: boolean): void;
						public getVideoSurfaceView(): globalAndroid.view.View;
						public setExtraAdGroupMarkers(param0: androidNative.Array<number>, param1: androidNative.Array<boolean>): void;
						public getUseController(): boolean;
						public dispatchMediaKeyEvent(param0: globalAndroid.view.KeyEvent): boolean;
						public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet);
						public setAspectRatioListener(param0: com.google.android.exoplayer2.ui.AspectRatioFrameLayout.AspectRatioListener): void;
						public getAdOverlayInfos(): java.util.List<com.google.android.exoplayer2.ui.AdOverlayInfo>;
						public setShowVrButton(param0: boolean): void;
						public setShutterBackgroundColor(param0: number): void;
						public setUseController(param0: boolean): void;
						public setShowNextButton(param0: boolean): void;
						public setErrorMessageProvider(param0: com.google.android.exoplayer2.util.ErrorMessageProvider<any>): void;
						public getResizeMode(): number;
						public setKeepContentOnPlayerReset(param0: boolean): void;
						public getUseArtwork(): boolean;
						public setPlayer(param0: com.google.android.exoplayer2.Player): void;
						public setShowShuffleButton(param0: boolean): void;
						public setControllerHideDuringAds(param0: boolean): void;
						public getAdViewGroup(): globalAndroid.view.ViewGroup;
						public hideController(): void;
						public getControllerAutoShow(): boolean;
						public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet, param2: number);
						public setDefaultArtwork(param0: globalAndroid.graphics.drawable.Drawable): void;
						public setControllerOnFullScreenModeChangedListener(param0: com.google.android.exoplayer2.ui.StyledPlayerControlView.OnFullScreenModeChangedListener): void;
						public onTrackballEvent(param0: globalAndroid.view.MotionEvent): boolean;
						public showController(): void;
					}
					export module StyledPlayerView {
						export class ComponentListener extends com.google.android.exoplayer2.ui.StyledPlayerControlView.VisibilityListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.StyledPlayerView.ComponentListener>;
							public onPositionDiscontinuity(param0: com.google.android.exoplayer2.Player.PositionInfo, param1: com.google.android.exoplayer2.Player.PositionInfo, param2: number): void;
							public onVisibilityChange(param0: number): void;
							public onCues(param0: java.util.List<com.google.android.exoplayer2.text.Cue>): void;
							public onClick(param0: globalAndroid.view.View): void;
							public onPlaybackStateChanged(param0: number): void;
							public onRenderedFirstFrame(): void;
							public constructor(param0: com.google.android.exoplayer2.ui.StyledPlayerView);
							public onPlayWhenReadyChanged(param0: boolean, param1: number): void;
							public onVideoSizeChanged(param0: com.google.android.exoplayer2.video.VideoSize): void;
							public onTracksInfoChanged(param0: com.google.android.exoplayer2.TracksInfo): void;
							public onLayoutChange(param0: globalAndroid.view.View, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number): void;
						}
						export class ShowBuffering {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.StyledPlayerView.ShowBuffering>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.ui.StyledPlayerView$ShowBuffering interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
							});
							public constructor();
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module ui {
					export class SubtitlePainter {
						public static class: java.lang.Class<com.google.android.exoplayer2.ui.SubtitlePainter>;
						public constructor(param0: globalAndroid.content.Context);
						public draw(param0: com.google.android.exoplayer2.text.Cue, param1: com.google.android.exoplayer2.ui.CaptionStyleCompat, param2: number, param3: number, param4: number, param5: globalAndroid.graphics.Canvas, param6: number, param7: number, param8: number, param9: number): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module ui {
					export class SubtitleView {
						public static class: java.lang.Class<com.google.android.exoplayer2.ui.SubtitleView>;
						public static DEFAULT_TEXT_SIZE_FRACTION: number;
						public static DEFAULT_BOTTOM_PADDING_FRACTION: number;
						public static VIEW_TYPE_CANVAS: number;
						public static VIEW_TYPE_WEB: number;
						public setApplyEmbeddedFontSizes(param0: boolean): void;
						public setFixedTextSize(param0: number, param1: number): void;
						public constructor(param0: globalAndroid.content.Context);
						public setUserDefaultTextSize(): void;
						public setBottomPaddingFraction(param0: number): void;
						public setFractionalTextSize(param0: number, param1: boolean): void;
						public onCues(param0: java.util.List<com.google.android.exoplayer2.text.Cue>): void;
						public setCues(param0: java.util.List<com.google.android.exoplayer2.text.Cue>): void;
						public setApplyEmbeddedStyles(param0: boolean): void;
						public setStyle(param0: com.google.android.exoplayer2.ui.CaptionStyleCompat): void;
						public setViewType(param0: number): void;
						public setFractionalTextSize(param0: number): void;
						public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet);
						public setUserDefaultStyle(): void;
					}
					export module SubtitleView {
						export class Output {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.SubtitleView.Output>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.ui.SubtitleView$Output interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								update(param0: java.util.List<com.google.android.exoplayer2.text.Cue>, param1: com.google.android.exoplayer2.ui.CaptionStyleCompat, param2: number, param3: number, param4: number): void;
							});
							public constructor();
							public update(param0: java.util.List<com.google.android.exoplayer2.text.Cue>, param1: com.google.android.exoplayer2.ui.CaptionStyleCompat, param2: number, param3: number, param4: number): void;
						}
						export class ViewType {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.SubtitleView.ViewType>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.ui.SubtitleView$ViewType interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
							});
							public constructor();
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module ui {
					export class SubtitleViewUtils {
						public static class: java.lang.Class<com.google.android.exoplayer2.ui.SubtitleViewUtils>;
						public static resolveTextSize(param0: number, param1: number, param2: number, param3: number): number;
						public static removeAllEmbeddedStyling(param0: com.google.android.exoplayer2.text.Cue.Builder): void;
						public static removeEmbeddedFontSizes(param0: com.google.android.exoplayer2.text.Cue.Builder): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module ui {
					export class TimeBar {
						public static class: java.lang.Class<com.google.android.exoplayer2.ui.TimeBar>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.ui.TimeBar interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							addListener(param0: com.google.android.exoplayer2.ui.TimeBar.OnScrubListener): void;
							removeListener(param0: com.google.android.exoplayer2.ui.TimeBar.OnScrubListener): void;
							setEnabled(param0: boolean): void;
							setKeyTimeIncrement(param0: number): void;
							setKeyCountIncrement(param0: number): void;
							setPosition(param0: number): void;
							setBufferedPosition(param0: number): void;
							setDuration(param0: number): void;
							getPreferredUpdateDelay(): number;
							setAdGroupTimesMs(param0: androidNative.Array<number>, param1: androidNative.Array<boolean>, param2: number): void;
						});
						public constructor();
						public getPreferredUpdateDelay(): number;
						public setKeyCountIncrement(param0: number): void;
						public removeListener(param0: com.google.android.exoplayer2.ui.TimeBar.OnScrubListener): void;
						public setPosition(param0: number): void;
						public setBufferedPosition(param0: number): void;
						public setAdGroupTimesMs(param0: androidNative.Array<number>, param1: androidNative.Array<boolean>, param2: number): void;
						public setDuration(param0: number): void;
						public setEnabled(param0: boolean): void;
						public setKeyTimeIncrement(param0: number): void;
						public addListener(param0: com.google.android.exoplayer2.ui.TimeBar.OnScrubListener): void;
					}
					export module TimeBar {
						export class OnScrubListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.TimeBar.OnScrubListener>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.ui.TimeBar$OnScrubListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onScrubStart(param0: com.google.android.exoplayer2.ui.TimeBar, param1: number): void;
								onScrubMove(param0: com.google.android.exoplayer2.ui.TimeBar, param1: number): void;
								onScrubStop(param0: com.google.android.exoplayer2.ui.TimeBar, param1: number, param2: boolean): void;
							});
							public constructor();
							public onScrubStop(param0: com.google.android.exoplayer2.ui.TimeBar, param1: number, param2: boolean): void;
							public onScrubMove(param0: com.google.android.exoplayer2.ui.TimeBar, param1: number): void;
							public onScrubStart(param0: com.google.android.exoplayer2.ui.TimeBar, param1: number): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module ui {
					export class TrackNameProvider {
						public static class: java.lang.Class<com.google.android.exoplayer2.ui.TrackNameProvider>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.ui.TrackNameProvider interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							getTrackName(param0: com.google.android.exoplayer2.Format): string;
						});
						public constructor();
						public getTrackName(param0: com.google.android.exoplayer2.Format): string;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module ui {
					export class TrackSelectionDialogBuilder {
						public static class: java.lang.Class<com.google.android.exoplayer2.ui.TrackSelectionDialogBuilder>;
						public setTrackNameProvider(param0: com.google.android.exoplayer2.ui.TrackNameProvider): com.google.android.exoplayer2.ui.TrackSelectionDialogBuilder;
						public constructor(param0: globalAndroid.content.Context, param1: string, param2: com.google.android.exoplayer2.trackselection.MappingTrackSelector.MappedTrackInfo, param3: number, param4: com.google.android.exoplayer2.ui.TrackSelectionDialogBuilder.DialogCallback);
						public setTrackFormatComparator(param0: java.util.Comparator<com.google.android.exoplayer2.Format>): void;
						public setOverrides(param0: java.util.List<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.SelectionOverride>): com.google.android.exoplayer2.ui.TrackSelectionDialogBuilder;
						public setAllowAdaptiveSelections(param0: boolean): com.google.android.exoplayer2.ui.TrackSelectionDialogBuilder;
						public setAllowMultipleOverrides(param0: boolean): com.google.android.exoplayer2.ui.TrackSelectionDialogBuilder;
						public setTheme(param0: number): com.google.android.exoplayer2.ui.TrackSelectionDialogBuilder;
						public setShowDisableOption(param0: boolean): com.google.android.exoplayer2.ui.TrackSelectionDialogBuilder;
						public setIsDisabled(param0: boolean): com.google.android.exoplayer2.ui.TrackSelectionDialogBuilder;
						public constructor(param0: globalAndroid.content.Context, param1: string, param2: com.google.android.exoplayer2.trackselection.DefaultTrackSelector, param3: number);
						public setOverride(param0: com.google.android.exoplayer2.trackselection.DefaultTrackSelector.SelectionOverride): com.google.android.exoplayer2.ui.TrackSelectionDialogBuilder;
						public build(): globalAndroid.app.Dialog;
					}
					export module TrackSelectionDialogBuilder {
						export class DialogCallback {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.TrackSelectionDialogBuilder.DialogCallback>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.ui.TrackSelectionDialogBuilder$DialogCallback interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onTracksSelected(param0: boolean, param1: java.util.List<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.SelectionOverride>): void;
							});
							public constructor();
							public onTracksSelected(param0: boolean, param1: java.util.List<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.SelectionOverride>): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module ui {
					export class TrackSelectionView {
						public static class: java.lang.Class<com.google.android.exoplayer2.ui.TrackSelectionView>;
						public setShowDisableOption(param0: boolean): void;
						public setAllowMultipleOverrides(param0: boolean): void;
						public setTrackNameProvider(param0: com.google.android.exoplayer2.ui.TrackNameProvider): void;
						public constructor(param0: globalAndroid.content.Context);
						public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet, param2: number);
						public init(param0: com.google.android.exoplayer2.trackselection.MappingTrackSelector.MappedTrackInfo, param1: number, param2: boolean, param3: java.util.List<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.SelectionOverride>, param4: java.util.Comparator<com.google.android.exoplayer2.Format>, param5: com.google.android.exoplayer2.ui.TrackSelectionView.TrackSelectionListener): void;
						public getIsDisabled(): boolean;
						public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet);
						public setAllowAdaptiveSelections(param0: boolean): void;
						public getOverrides(): java.util.List<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.SelectionOverride>;
					}
					export module TrackSelectionView {
						export class ComponentListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.TrackSelectionView.ComponentListener>;
							public onClick(param0: globalAndroid.view.View): void;
						}
						export class TrackInfo {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.TrackSelectionView.TrackInfo>;
							public groupIndex: number;
							public trackIndex: number;
							public format: com.google.android.exoplayer2.Format;
							public constructor(param0: number, param1: number, param2: com.google.android.exoplayer2.Format);
						}
						export class TrackSelectionListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.ui.TrackSelectionView.TrackSelectionListener>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.ui.TrackSelectionView$TrackSelectionListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onTrackSelectionChanged(param0: boolean, param1: java.util.List<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.SelectionOverride>): void;
							});
							public constructor();
							public onTrackSelectionChanged(param0: boolean, param1: java.util.List<com.google.android.exoplayer2.trackselection.DefaultTrackSelector.SelectionOverride>): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module ui {
					export class WebViewSubtitleOutput implements com.google.android.exoplayer2.ui.SubtitleView.Output {
						public static class: java.lang.Class<com.google.android.exoplayer2.ui.WebViewSubtitleOutput>;
						public constructor(param0: globalAndroid.content.Context);
						public destroy(): void;
						public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet);
						public update(param0: java.util.List<com.google.android.exoplayer2.text.Cue>, param1: com.google.android.exoplayer2.ui.CaptionStyleCompat, param2: number, param3: number, param4: number): void;
						public onLayout(param0: boolean, param1: number, param2: number, param3: number, param4: number): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class Allocation {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.Allocation>;
						public data: androidNative.Array<number>;
						public offset: number;
						public constructor(param0: androidNative.Array<number>, param1: number);
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class Allocator {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.Allocator>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.upstream.Allocator interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							allocate(): com.google.android.exoplayer2.upstream.Allocation;
							release(param0: com.google.android.exoplayer2.upstream.Allocation): void;
							release(param0: com.google.android.exoplayer2.upstream.Allocator.AllocationNode): void;
							trim(): void;
							getTotalBytesAllocated(): number;
							getIndividualAllocationLength(): number;
						});
						public constructor();
						public getTotalBytesAllocated(): number;
						public release(param0: com.google.android.exoplayer2.upstream.Allocator.AllocationNode): void;
						public trim(): void;
						public release(param0: com.google.android.exoplayer2.upstream.Allocation): void;
						public allocate(): com.google.android.exoplayer2.upstream.Allocation;
						public getIndividualAllocationLength(): number;
					}
					export module Allocator {
						export class AllocationNode {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.Allocator.AllocationNode>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.upstream.Allocator$AllocationNode interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								getAllocation(): com.google.android.exoplayer2.upstream.Allocation;
								next(): com.google.android.exoplayer2.upstream.Allocator.AllocationNode;
							});
							public constructor();
							public next(): com.google.android.exoplayer2.upstream.Allocator.AllocationNode;
							public getAllocation(): com.google.android.exoplayer2.upstream.Allocation;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class AssetDataSource extends com.google.android.exoplayer2.upstream.BaseDataSource {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.AssetDataSource>;
						public getResponseHeaders(): java.util.Map<string,java.util.List<string>>;
						public open(param0: com.google.android.exoplayer2.upstream.DataSpec): number;
						public constructor(param0: globalAndroid.content.Context);
						public close(): void;
						public read(param0: androidNative.Array<number>, param1: number, param2: number): number;
						public constructor(param0: boolean);
						public getUri(): globalAndroid.net.Uri;
						public addTransferListener(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
					}
					export module AssetDataSource {
						export class AssetDataSourceException extends com.google.android.exoplayer2.upstream.DataSourceException {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.AssetDataSource.AssetDataSourceException>;
							public constructor(param0: java.lang.Throwable, param1: number);
							public constructor(param0: string, param1: java.lang.Throwable, param2: number);
							/** @deprecated */
							public constructor(param0: java.io.IOException);
							public constructor(param0: string, param1: number);
							public constructor(param0: number);
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class BandwidthMeter {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.BandwidthMeter>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.upstream.BandwidthMeter interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							getBitrateEstimate(): number;
							getTimeToFirstByteEstimateUs(): number;
							getTransferListener(): com.google.android.exoplayer2.upstream.TransferListener;
							addEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.upstream.BandwidthMeter.EventListener): void;
							removeEventListener(param0: com.google.android.exoplayer2.upstream.BandwidthMeter.EventListener): void;
						});
						public constructor();
						public getTimeToFirstByteEstimateUs(): number;
						public getBitrateEstimate(): number;
						public removeEventListener(param0: com.google.android.exoplayer2.upstream.BandwidthMeter.EventListener): void;
						public addEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.upstream.BandwidthMeter.EventListener): void;
						public getTransferListener(): com.google.android.exoplayer2.upstream.TransferListener;
					}
					export module BandwidthMeter {
						export class EventListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.BandwidthMeter.EventListener>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.upstream.BandwidthMeter$EventListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onBandwidthSample(param0: number, param1: number, param2: number): void;
							});
							public constructor();
							public onBandwidthSample(param0: number, param1: number, param2: number): void;
						}
						export module EventListener {
							export class EventDispatcher {
								public static class: java.lang.Class<com.google.android.exoplayer2.upstream.BandwidthMeter.EventListener.EventDispatcher>;
								public removeListener(param0: com.google.android.exoplayer2.upstream.BandwidthMeter.EventListener): void;
								public addListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.upstream.BandwidthMeter.EventListener): void;
								public constructor();
								public bandwidthSample(param0: number, param1: number, param2: number): void;
							}
							export module EventDispatcher {
								export class HandlerAndListener {
									public static class: java.lang.Class<com.google.android.exoplayer2.upstream.BandwidthMeter.EventListener.EventDispatcher.HandlerAndListener>;
									public release(): void;
									public constructor(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.upstream.BandwidthMeter.EventListener);
								}
							}
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export abstract class BaseDataSource extends com.google.android.exoplayer2.upstream.DataSource {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.BaseDataSource>;
						public getResponseHeaders(): java.util.Map<string,java.util.List<string>>;
						public transferEnded(): void;
						public open(param0: com.google.android.exoplayer2.upstream.DataSpec): number;
						public close(): void;
						public transferStarted(param0: com.google.android.exoplayer2.upstream.DataSpec): void;
						public bytesTransferred(param0: number): void;
						public constructor(param0: boolean);
						public transferInitializing(param0: com.google.android.exoplayer2.upstream.DataSpec): void;
						public addTransferListener(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
						public getUri(): globalAndroid.net.Uri;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class BuildConfig {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.BuildConfig>;
						public static DEBUG: boolean;
						public static LIBRARY_PACKAGE_NAME: string;
						public static BUILD_TYPE: string;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class ByteArrayDataSink extends com.google.android.exoplayer2.upstream.DataSink {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.ByteArrayDataSink>;
						public constructor();
						public close(): void;
						public write(param0: androidNative.Array<number>, param1: number, param2: number): void;
						public getData(): androidNative.Array<number>;
						public open(param0: com.google.android.exoplayer2.upstream.DataSpec): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class ByteArrayDataSource extends com.google.android.exoplayer2.upstream.BaseDataSource {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.ByteArrayDataSource>;
						public getResponseHeaders(): java.util.Map<string,java.util.List<string>>;
						public constructor(param0: androidNative.Array<number>);
						public open(param0: com.google.android.exoplayer2.upstream.DataSpec): number;
						public close(): void;
						public read(param0: androidNative.Array<number>, param1: number, param2: number): number;
						public constructor(param0: boolean);
						public getUri(): globalAndroid.net.Uri;
						public addTransferListener(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class CachedRegionTracker extends com.google.android.exoplayer2.upstream.cache.Cache.Listener {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.CachedRegionTracker>;
						public static NOT_CACHED: number;
						public static CACHED_TO_END: number;
						public onSpanAdded(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: com.google.android.exoplayer2.upstream.cache.CacheSpan): void;
						public onSpanTouched(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: com.google.android.exoplayer2.upstream.cache.CacheSpan, param2: com.google.android.exoplayer2.upstream.cache.CacheSpan): void;
						public onSpanRemoved(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: com.google.android.exoplayer2.upstream.cache.CacheSpan): void;
						public release(): void;
						public constructor(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: string, param2: com.google.android.exoplayer2.extractor.ChunkIndex);
						public getRegionEndTimeMs(param0: number): number;
					}
					export module CachedRegionTracker {
						export class Region extends java.lang.Comparable<com.google.android.exoplayer2.upstream.CachedRegionTracker.Region> {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.CachedRegionTracker.Region>;
							public startOffset: number;
							public endOffset: number;
							public endOffsetIndex: number;
							public constructor(param0: number, param1: number);
							public compareTo(param0: com.google.android.exoplayer2.upstream.CachedRegionTracker.Region): number;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class ContentDataSource extends com.google.android.exoplayer2.upstream.BaseDataSource {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.ContentDataSource>;
						public getResponseHeaders(): java.util.Map<string,java.util.List<string>>;
						public open(param0: com.google.android.exoplayer2.upstream.DataSpec): number;
						public constructor(param0: globalAndroid.content.Context);
						public close(): void;
						public read(param0: androidNative.Array<number>, param1: number, param2: number): number;
						public constructor(param0: boolean);
						public getUri(): globalAndroid.net.Uri;
						public addTransferListener(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
					}
					export module ContentDataSource {
						export class Api31 {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.ContentDataSource.Api31>;
							public static disableTranscoding(param0: globalAndroid.os.Bundle): void;
						}
						export class ContentDataSourceException extends com.google.android.exoplayer2.upstream.DataSourceException {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.ContentDataSource.ContentDataSourceException>;
							public constructor(param0: java.lang.Throwable, param1: number);
							public constructor(param0: string, param1: java.lang.Throwable, param2: number);
							/** @deprecated */
							public constructor(param0: java.io.IOException);
							public constructor(param0: java.io.IOException, param1: number);
							public constructor(param0: string, param1: number);
							public constructor(param0: number);
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class DataSchemeDataSource extends com.google.android.exoplayer2.upstream.BaseDataSource {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.DataSchemeDataSource>;
						public static SCHEME_DATA: string;
						public getResponseHeaders(): java.util.Map<string,java.util.List<string>>;
						public constructor();
						public open(param0: com.google.android.exoplayer2.upstream.DataSpec): number;
						public close(): void;
						public read(param0: androidNative.Array<number>, param1: number, param2: number): number;
						public constructor(param0: boolean);
						public getUri(): globalAndroid.net.Uri;
						public addTransferListener(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class DataSink {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.DataSink>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.upstream.DataSink interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							open(param0: com.google.android.exoplayer2.upstream.DataSpec): void;
							write(param0: androidNative.Array<number>, param1: number, param2: number): void;
							close(): void;
						});
						public constructor();
						public close(): void;
						public write(param0: androidNative.Array<number>, param1: number, param2: number): void;
						public open(param0: com.google.android.exoplayer2.upstream.DataSpec): void;
					}
					export module DataSink {
						export class Factory {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.DataSink.Factory>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.upstream.DataSink$Factory interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								createDataSink(): com.google.android.exoplayer2.upstream.DataSink;
							});
							public constructor();
							public createDataSink(): com.google.android.exoplayer2.upstream.DataSink;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class DataSource {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.DataSource>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.upstream.DataSource interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							addTransferListener(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
							open(param0: com.google.android.exoplayer2.upstream.DataSpec): number;
							getUri(): globalAndroid.net.Uri;
							getResponseHeaders(): java.util.Map<string,java.util.List<string>>;
							close(): void;
						});
						public constructor();
						public getResponseHeaders(): java.util.Map<string,java.util.List<string>>;
						public open(param0: com.google.android.exoplayer2.upstream.DataSpec): number;
						public close(): void;
						public addTransferListener(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
						public getUri(): globalAndroid.net.Uri;
					}
					export module DataSource {
						export class Factory {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.DataSource.Factory>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.upstream.DataSource$Factory interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								createDataSource(): com.google.android.exoplayer2.upstream.DataSource;
							});
							public constructor();
							public createDataSource(): com.google.android.exoplayer2.upstream.DataSource;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class DataSourceException {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.DataSourceException>;
						public static POSITION_OUT_OF_RANGE: number;
						public reason: number;
						public constructor(param0: string, param1: java.lang.Throwable, param2: number);
						public constructor(param0: string, param1: number);
						public constructor(param0: number);
						public static isCausedByPositionOutOfRange(param0: java.io.IOException): boolean;
						public constructor(param0: java.lang.Throwable, param1: number);
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class DataSourceInputStream {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.DataSourceInputStream>;
						public read(param0: androidNative.Array<number>): number;
						public read(): number;
						public close(): void;
						public read(param0: androidNative.Array<number>, param1: number, param2: number): number;
						public open(): void;
						public bytesRead(): number;
						public constructor(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec);
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class DataSourceUtil {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.DataSourceUtil>;
						public static readToEnd(param0: com.google.android.exoplayer2.upstream.DataSource): androidNative.Array<number>;
						public static readExactly(param0: com.google.android.exoplayer2.upstream.DataSource, param1: number): androidNative.Array<number>;
						public static closeQuietly(param0: com.google.android.exoplayer2.upstream.DataSource): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class DataSpec {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.DataSpec>;
						public static FLAG_ALLOW_GZIP: number;
						public static FLAG_DONT_CACHE_IF_LENGTH_UNKNOWN: number;
						public static FLAG_ALLOW_CACHE_FRAGMENTATION: number;
						public static FLAG_MIGHT_NOT_USE_FULL_NETWORK_SPEED: number;
						public static HTTP_METHOD_GET: number;
						public static HTTP_METHOD_POST: number;
						public static HTTP_METHOD_HEAD: number;
						public uri: globalAndroid.net.Uri;
						public uriPositionOffset: number;
						public httpMethod: number;
						public httpBody: androidNative.Array<number>;
						public httpRequestHeaders: java.util.Map<string,string>;
						public absoluteStreamPosition: number;
						public position: number;
						public length: number;
						public key: string;
						public flags: number;
						public customData: any;
						/** @deprecated */
						public constructor(param0: globalAndroid.net.Uri, param1: number, param2: androidNative.Array<number>, param3: number, param4: number, param5: number, param6: string, param7: number);
						public getHttpMethodString(): string;
						public withRequestHeaders(param0: java.util.Map<string,string>): com.google.android.exoplayer2.upstream.DataSpec;
						public buildUpon(): com.google.android.exoplayer2.upstream.DataSpec.Builder;
						public subrange(param0: number): com.google.android.exoplayer2.upstream.DataSpec;
						public withUri(param0: globalAndroid.net.Uri): com.google.android.exoplayer2.upstream.DataSpec;
						/** @deprecated */
						public constructor(param0: globalAndroid.net.Uri, param1: number, param2: number, param3: string);
						public withAdditionalHeaders(param0: java.util.Map<string,string>): com.google.android.exoplayer2.upstream.DataSpec;
						/** @deprecated */
						public constructor(param0: globalAndroid.net.Uri, param1: number);
						/** @deprecated */
						public constructor(param0: globalAndroid.net.Uri, param1: number, param2: number, param3: number, param4: string, param5: number);
						public static getStringForHttpMethod(param0: number): string;
						/** @deprecated */
						public constructor(param0: globalAndroid.net.Uri, param1: number, param2: number, param3: string, param4: number);
						public toString(): string;
						public constructor(param0: globalAndroid.net.Uri);
						public subrange(param0: number, param1: number): com.google.android.exoplayer2.upstream.DataSpec;
						/** @deprecated */
						public constructor(param0: globalAndroid.net.Uri, param1: number, param2: androidNative.Array<number>, param3: number, param4: number, param5: number, param6: string, param7: number, param8: java.util.Map<string,string>);
						/** @deprecated */
						public constructor(param0: globalAndroid.net.Uri, param1: number, param2: number, param3: string, param4: number, param5: java.util.Map<string,string>);
						public constructor(param0: globalAndroid.net.Uri, param1: number, param2: number);
						/** @deprecated */
						public constructor(param0: globalAndroid.net.Uri, param1: androidNative.Array<number>, param2: number, param3: number, param4: number, param5: string, param6: number);
						public isFlagSet(param0: number): boolean;
					}
					export module DataSpec {
						export class Builder {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.DataSpec.Builder>;
							public constructor();
							public setKey(param0: string): com.google.android.exoplayer2.upstream.DataSpec.Builder;
							public setFlags(param0: number): com.google.android.exoplayer2.upstream.DataSpec.Builder;
							public build(): com.google.android.exoplayer2.upstream.DataSpec;
							public setLength(param0: number): com.google.android.exoplayer2.upstream.DataSpec.Builder;
							public setUriPositionOffset(param0: number): com.google.android.exoplayer2.upstream.DataSpec.Builder;
							public setCustomData(param0: any): com.google.android.exoplayer2.upstream.DataSpec.Builder;
							public setUri(param0: string): com.google.android.exoplayer2.upstream.DataSpec.Builder;
							public setHttpBody(param0: androidNative.Array<number>): com.google.android.exoplayer2.upstream.DataSpec.Builder;
							public setHttpRequestHeaders(param0: java.util.Map<string,string>): com.google.android.exoplayer2.upstream.DataSpec.Builder;
							public setUri(param0: globalAndroid.net.Uri): com.google.android.exoplayer2.upstream.DataSpec.Builder;
							public setPosition(param0: number): com.google.android.exoplayer2.upstream.DataSpec.Builder;
							public setHttpMethod(param0: number): com.google.android.exoplayer2.upstream.DataSpec.Builder;
						}
						export class Flags {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.DataSpec.Flags>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.upstream.DataSpec$Flags interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
							});
							public constructor();
						}
						export class HttpMethod {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.DataSpec.HttpMethod>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.upstream.DataSpec$HttpMethod interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
							});
							public constructor();
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class DefaultAllocator extends com.google.android.exoplayer2.upstream.Allocator {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.DefaultAllocator>;
						public getTotalBytesAllocated(): number;
						public release(param0: com.google.android.exoplayer2.upstream.Allocator.AllocationNode): void;
						public trim(): void;
						public constructor(param0: boolean, param1: number, param2: number);
						public setTargetBufferSize(param0: number): void;
						public release(param0: com.google.android.exoplayer2.upstream.Allocation): void;
						public constructor(param0: boolean, param1: number);
						public reset(): void;
						public allocate(): com.google.android.exoplayer2.upstream.Allocation;
						public getIndividualAllocationLength(): number;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class DefaultBandwidthMeter implements com.google.android.exoplayer2.upstream.BandwidthMeter, com.google.android.exoplayer2.upstream.TransferListener {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.DefaultBandwidthMeter>;
						public static DEFAULT_INITIAL_BITRATE_ESTIMATES_WIFI: com.google.common.collect.ImmutableList<java.lang.Long>;
						public static DEFAULT_INITIAL_BITRATE_ESTIMATES_2G: com.google.common.collect.ImmutableList<java.lang.Long>;
						public static DEFAULT_INITIAL_BITRATE_ESTIMATES_3G: com.google.common.collect.ImmutableList<java.lang.Long>;
						public static DEFAULT_INITIAL_BITRATE_ESTIMATES_4G: com.google.common.collect.ImmutableList<java.lang.Long>;
						public static DEFAULT_INITIAL_BITRATE_ESTIMATES_5G_NSA: com.google.common.collect.ImmutableList<java.lang.Long>;
						public static DEFAULT_INITIAL_BITRATE_ESTIMATES_5G_SA: com.google.common.collect.ImmutableList<java.lang.Long>;
						public static DEFAULT_INITIAL_BITRATE_ESTIMATE: number;
						public static DEFAULT_SLIDING_WINDOW_MAX_WEIGHT: number;
						public onTransferInitializing(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: boolean): void;
						public onTransferEnd(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: boolean): void;
						public getTimeToFirstByteEstimateUs(): number;
						public setNetworkTypeOverride(param0: number): void;
						public onTransferStart(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: boolean): void;
						/** @deprecated */
						public constructor();
						public getBitrateEstimate(): number;
						public removeEventListener(param0: com.google.android.exoplayer2.upstream.BandwidthMeter.EventListener): void;
						public static getSingletonInstance(param0: globalAndroid.content.Context): com.google.android.exoplayer2.upstream.DefaultBandwidthMeter;
						public addEventListener(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.upstream.BandwidthMeter.EventListener): void;
						public getTransferListener(): com.google.android.exoplayer2.upstream.TransferListener;
						public onBytesTransferred(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: boolean, param3: number): void;
					}
					export module DefaultBandwidthMeter {
						export class Builder {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.DefaultBandwidthMeter.Builder>;
							public setInitialBitrateEstimate(param0: number, param1: number): com.google.android.exoplayer2.upstream.DefaultBandwidthMeter.Builder;
							public build(): com.google.android.exoplayer2.upstream.DefaultBandwidthMeter;
							public setInitialBitrateEstimate(param0: number): com.google.android.exoplayer2.upstream.DefaultBandwidthMeter.Builder;
							public setInitialBitrateEstimate(param0: string): com.google.android.exoplayer2.upstream.DefaultBandwidthMeter.Builder;
							public setSlidingWindowMaxWeight(param0: number): com.google.android.exoplayer2.upstream.DefaultBandwidthMeter.Builder;
							public setClock(param0: com.google.android.exoplayer2.util.Clock): com.google.android.exoplayer2.upstream.DefaultBandwidthMeter.Builder;
							public constructor(param0: globalAndroid.content.Context);
							public setResetOnNetworkTypeChange(param0: boolean): com.google.android.exoplayer2.upstream.DefaultBandwidthMeter.Builder;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class DefaultDataSource extends com.google.android.exoplayer2.upstream.DataSource {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.DefaultDataSource>;
						public getResponseHeaders(): java.util.Map<string,java.util.List<string>>;
						public open(param0: com.google.android.exoplayer2.upstream.DataSpec): number;
						public constructor(param0: globalAndroid.content.Context, param1: boolean);
						public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.upstream.DataSource);
						public constructor(param0: globalAndroid.content.Context, param1: string, param2: number, param3: number, param4: boolean);
						public close(): void;
						public read(param0: androidNative.Array<number>, param1: number, param2: number): number;
						public addTransferListener(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
						public getUri(): globalAndroid.net.Uri;
						public constructor(param0: globalAndroid.content.Context, param1: string, param2: boolean);
					}
					export module DefaultDataSource {
						export class Factory extends com.google.android.exoplayer2.upstream.DataSource.Factory {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.DefaultDataSource.Factory>;
							public createDataSource(): com.google.android.exoplayer2.upstream.DataSource;
							public createDataSource(): com.google.android.exoplayer2.upstream.DefaultDataSource;
							public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.upstream.DataSource.Factory);
							public constructor(param0: globalAndroid.content.Context);
							public setTransferListener(param0: com.google.android.exoplayer2.upstream.TransferListener): com.google.android.exoplayer2.upstream.DefaultDataSource.Factory;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class DefaultDataSourceFactory extends com.google.android.exoplayer2.upstream.DataSource.Factory {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.DefaultDataSourceFactory>;
						public constructor(param0: globalAndroid.content.Context, param1: string, param2: com.google.android.exoplayer2.upstream.TransferListener);
						public constructor(param0: globalAndroid.content.Context);
						public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.upstream.TransferListener, param2: com.google.android.exoplayer2.upstream.DataSource.Factory);
						public createDataSource(): com.google.android.exoplayer2.upstream.DataSource;
						public constructor(param0: globalAndroid.content.Context, param1: string);
						public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.upstream.DataSource.Factory);
						public createDataSource(): com.google.android.exoplayer2.upstream.DefaultDataSource;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class DefaultHttpDataSource extends com.google.android.exoplayer2.upstream.BaseDataSource implements com.google.android.exoplayer2.upstream.HttpDataSource {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.DefaultHttpDataSource>;
						public static DEFAULT_CONNECT_TIMEOUT_MILLIS: number;
						public static DEFAULT_READ_TIMEOUT_MILLIS: number;
						public clearAllRequestProperties(): void;
						public close(): void;
						/** @deprecated */
						public constructor(param0: string, param1: number, param2: number);
						public getResponseCode(): number;
						public read(param0: androidNative.Array<number>, param1: number, param2: number): number;
						public getUri(): globalAndroid.net.Uri;
						public getResponseHeaders(): java.util.Map<string,java.util.List<string>>;
						public open(param0: com.google.android.exoplayer2.upstream.DataSpec): number;
						/** @deprecated */
						public constructor();
						/** @deprecated */
						public constructor(param0: string);
						/** @deprecated */
						public setContentTypePredicate(param0: com.google.common.base.Predicate<string>): void;
						public constructor(param0: boolean);
						/** @deprecated */
						public constructor(param0: string, param1: number, param2: number, param3: boolean, param4: com.google.android.exoplayer2.upstream.HttpDataSource.RequestProperties);
						public clearRequestProperty(param0: string): void;
						public addTransferListener(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
						public setRequestProperty(param0: string, param1: string): void;
					}
					export module DefaultHttpDataSource {
						export class Factory extends com.google.android.exoplayer2.upstream.HttpDataSource.Factory {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.DefaultHttpDataSource.Factory>;
							public constructor();
							public createDataSource(): com.google.android.exoplayer2.upstream.DefaultHttpDataSource;
							public setDefaultRequestProperties(param0: java.util.Map<string,string>): com.google.android.exoplayer2.upstream.DefaultHttpDataSource.Factory;
							public createDataSource(): com.google.android.exoplayer2.upstream.HttpDataSource;
							public createDataSource(): com.google.android.exoplayer2.upstream.DataSource;
							public setConnectTimeoutMs(param0: number): com.google.android.exoplayer2.upstream.DefaultHttpDataSource.Factory;
							public setReadTimeoutMs(param0: number): com.google.android.exoplayer2.upstream.DefaultHttpDataSource.Factory;
							public setContentTypePredicate(param0: com.google.common.base.Predicate<string>): com.google.android.exoplayer2.upstream.DefaultHttpDataSource.Factory;
							public setTransferListener(param0: com.google.android.exoplayer2.upstream.TransferListener): com.google.android.exoplayer2.upstream.DefaultHttpDataSource.Factory;
							public setDefaultRequestProperties(param0: java.util.Map<string,string>): com.google.android.exoplayer2.upstream.HttpDataSource.Factory;
							public setKeepPostFor302Redirects(param0: boolean): com.google.android.exoplayer2.upstream.DefaultHttpDataSource.Factory;
							public setUserAgent(param0: string): com.google.android.exoplayer2.upstream.DefaultHttpDataSource.Factory;
							public setAllowCrossProtocolRedirects(param0: boolean): com.google.android.exoplayer2.upstream.DefaultHttpDataSource.Factory;
						}
						export class NullFilteringHeadersMap extends com.google.common.collect.ForwardingMap<string,java.util.List<string>> {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.DefaultHttpDataSource.NullFilteringHeadersMap>;
							public size(): number;
							public hashCode(): number;
							public delegate(): java.util.Map<string,java.util.List<string>>;
							public containsKey(param0: any): boolean;
							public keySet(): java.util.Set<string>;
							public constructor(param0: java.util.Map<string,java.util.List<string>>);
							public isEmpty(): boolean;
							public equals(param0: any): boolean;
							public containsValue(param0: any): boolean;
							public get(param0: any): java.util.List<string>;
							public entrySet(): java.util.Set<java.util.Map.Entry<string,java.util.List<string>>>;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class DefaultLoadErrorHandlingPolicy extends com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.DefaultLoadErrorHandlingPolicy>;
						public static DEFAULT_MIN_LOADABLE_RETRY_COUNT: number;
						public static DEFAULT_MIN_LOADABLE_RETRY_COUNT_PROGRESSIVE_LIVE: number;
						public static DEFAULT_TRACK_EXCLUSION_MS: number;
						public static DEFAULT_TRACK_BLACKLIST_MS: number;
						public static DEFAULT_LOCATION_EXCLUSION_MS: number;
						public constructor();
						public constructor(param0: number);
						public onLoadTaskConcluded(param0: number): void;
						public isEligibleForFallback(param0: java.io.IOException): boolean;
						public getFallbackSelectionFor(param0: com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy.FallbackOptions, param1: com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy.LoadErrorInfo): com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy.FallbackSelection;
						public getRetryDelayMsFor(param0: com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy.LoadErrorInfo): number;
						public getMinimumLoadableRetryCount(param0: number): number;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class DummyDataSource extends com.google.android.exoplayer2.upstream.DataSource {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.DummyDataSource>;
						public static INSTANCE: com.google.android.exoplayer2.upstream.DummyDataSource;
						public static FACTORY: com.google.android.exoplayer2.upstream.DataSource.Factory;
						public getResponseHeaders(): java.util.Map<string,java.util.List<string>>;
						public open(param0: com.google.android.exoplayer2.upstream.DataSpec): number;
						public close(): void;
						public read(param0: androidNative.Array<number>, param1: number, param2: number): number;
						public addTransferListener(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
						public getUri(): globalAndroid.net.Uri;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class FileDataSource extends com.google.android.exoplayer2.upstream.BaseDataSource {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.FileDataSource>;
						public getResponseHeaders(): java.util.Map<string,java.util.List<string>>;
						public constructor();
						public open(param0: com.google.android.exoplayer2.upstream.DataSpec): number;
						public close(): void;
						public read(param0: androidNative.Array<number>, param1: number, param2: number): number;
						public constructor(param0: boolean);
						public getUri(): globalAndroid.net.Uri;
						public addTransferListener(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
					}
					export module FileDataSource {
						export class Api21 {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.FileDataSource.Api21>;
						}
						export class Factory extends com.google.android.exoplayer2.upstream.DataSource.Factory {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.FileDataSource.Factory>;
							public constructor();
							public createDataSource(): com.google.android.exoplayer2.upstream.DataSource;
							public createDataSource(): com.google.android.exoplayer2.upstream.FileDataSource;
							public setListener(param0: com.google.android.exoplayer2.upstream.TransferListener): com.google.android.exoplayer2.upstream.FileDataSource.Factory;
						}
						export class FileDataSourceException extends com.google.android.exoplayer2.upstream.DataSourceException {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.FileDataSource.FileDataSourceException>;
							public constructor(param0: java.lang.Throwable, param1: number);
							/** @deprecated */
							public constructor(param0: string, param1: java.io.IOException);
							public constructor(param0: string, param1: java.lang.Throwable, param2: number);
							/** @deprecated */
							public constructor(param0: java.lang.Exception);
							public constructor(param0: string, param1: number);
							public constructor(param0: number);
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class HttpDataSource extends com.google.android.exoplayer2.upstream.DataSource {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.HttpDataSource>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.upstream.HttpDataSource interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							open(param0: com.google.android.exoplayer2.upstream.DataSpec): number;
							close(): void;
							read(param0: androidNative.Array<number>, param1: number, param2: number): number;
							setRequestProperty(param0: string, param1: string): void;
							clearRequestProperty(param0: string): void;
							clearAllRequestProperties(): void;
							getResponseCode(): number;
							getResponseHeaders(): java.util.Map<string,java.util.List<string>>;
							lambda$static$0(param0: string): boolean;
							<clinit>(): void;
							addTransferListener(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
							open(param0: com.google.android.exoplayer2.upstream.DataSpec): number;
							getUri(): globalAndroid.net.Uri;
							getResponseHeaders(): java.util.Map<string,java.util.List<string>>;
							close(): void;
						});
						public constructor();
						public static REJECT_PAYWALL_TYPES: com.google.common.base.Predicate<string>;
						public clearAllRequestProperties(): void;
						public getResponseHeaders(): java.util.Map<string,java.util.List<string>>;
						public open(param0: com.google.android.exoplayer2.upstream.DataSpec): number;
						public close(): void;
						public read(param0: androidNative.Array<number>, param1: number, param2: number): number;
						public getResponseCode(): number;
						public clearRequestProperty(param0: string): void;
						public addTransferListener(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
						public getUri(): globalAndroid.net.Uri;
						public setRequestProperty(param0: string, param1: string): void;
					}
					export module HttpDataSource {
						export abstract class BaseFactory extends com.google.android.exoplayer2.upstream.HttpDataSource.Factory {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.HttpDataSource.BaseFactory>;
							public constructor();
							public createDataSource(): com.google.android.exoplayer2.upstream.HttpDataSource;
							public setDefaultRequestProperties(param0: java.util.Map<string,string>): com.google.android.exoplayer2.upstream.HttpDataSource.Factory;
							public createDataSource(): com.google.android.exoplayer2.upstream.DataSource;
							public createDataSourceInternal(param0: com.google.android.exoplayer2.upstream.HttpDataSource.RequestProperties): com.google.android.exoplayer2.upstream.HttpDataSource;
						}
						export class CleartextNotPermittedException extends com.google.android.exoplayer2.upstream.HttpDataSource.HttpDataSourceException {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.HttpDataSource.CleartextNotPermittedException>;
							/** @deprecated */
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSpec, param1: number);
							public constructor(param0: java.lang.Throwable, param1: number);
							public constructor(param0: java.io.IOException, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: number, param3: number);
							/** @deprecated */
							public constructor(param0: string, param1: java.io.IOException, param2: com.google.android.exoplayer2.upstream.DataSpec, param3: number);
							public constructor(param0: string, param1: number);
							public constructor(param0: number);
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSpec, param1: number, param2: number);
							/** @deprecated */
							public constructor(param0: java.io.IOException, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: number);
							public constructor(param0: string, param1: java.io.IOException, param2: com.google.android.exoplayer2.upstream.DataSpec, param3: number, param4: number);
							/** @deprecated */
							public constructor(param0: string, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: number);
							public constructor(param0: string, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: number, param3: number);
							public constructor(param0: string, param1: java.lang.Throwable, param2: number);
							public constructor(param0: java.io.IOException, param1: com.google.android.exoplayer2.upstream.DataSpec);
						}
						export class Factory extends com.google.android.exoplayer2.upstream.DataSource.Factory {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.HttpDataSource.Factory>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.upstream.HttpDataSource$Factory interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								createDataSource(): com.google.android.exoplayer2.upstream.HttpDataSource;
								setDefaultRequestProperties(param0: java.util.Map<string,string>): com.google.android.exoplayer2.upstream.HttpDataSource.Factory;
								createDataSource(): com.google.android.exoplayer2.upstream.DataSource;
								createDataSource(): com.google.android.exoplayer2.upstream.DataSource;
							});
							public constructor();
							public createDataSource(): com.google.android.exoplayer2.upstream.HttpDataSource;
							public setDefaultRequestProperties(param0: java.util.Map<string,string>): com.google.android.exoplayer2.upstream.HttpDataSource.Factory;
							public createDataSource(): com.google.android.exoplayer2.upstream.DataSource;
						}
						export class HttpDataSourceException extends com.google.android.exoplayer2.upstream.DataSourceException {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.HttpDataSource.HttpDataSourceException>;
							public static TYPE_OPEN: number;
							public static TYPE_READ: number;
							public static TYPE_CLOSE: number;
							public dataSpec: com.google.android.exoplayer2.upstream.DataSpec;
							public type: number;
							public constructor(param0: java.lang.Throwable, param1: number);
							/** @deprecated */
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSpec, param1: number);
							public constructor(param0: java.io.IOException, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: number, param3: number);
							/** @deprecated */
							public constructor(param0: string, param1: java.io.IOException, param2: com.google.android.exoplayer2.upstream.DataSpec, param3: number);
							public constructor(param0: string, param1: number);
							public constructor(param0: number);
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSpec, param1: number, param2: number);
							/** @deprecated */
							public constructor(param0: java.io.IOException, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: number);
							public constructor(param0: string, param1: java.io.IOException, param2: com.google.android.exoplayer2.upstream.DataSpec, param3: number, param4: number);
							public static createForIOException(param0: java.io.IOException, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: number): com.google.android.exoplayer2.upstream.HttpDataSource.HttpDataSourceException;
							/** @deprecated */
							public constructor(param0: string, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: number);
							public constructor(param0: string, param1: java.lang.Throwable, param2: number);
							public constructor(param0: string, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: number, param3: number);
						}
						export module HttpDataSourceException {
							export class Type {
								public static class: java.lang.Class<com.google.android.exoplayer2.upstream.HttpDataSource.HttpDataSourceException.Type>;
								/**
								 * Constructs a new instance of the com.google.android.exoplayer2.upstream.HttpDataSource$HttpDataSourceException$Type interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
								 */
								public constructor(implementation: {
								});
								public constructor();
							}
						}
						export class InvalidContentTypeException extends com.google.android.exoplayer2.upstream.HttpDataSource.HttpDataSourceException {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.HttpDataSource.InvalidContentTypeException>;
							public contentType: string;
							/** @deprecated */
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSpec, param1: number);
							public constructor(param0: java.lang.Throwable, param1: number);
							public constructor(param0: java.io.IOException, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: number, param3: number);
							/** @deprecated */
							public constructor(param0: string, param1: java.io.IOException, param2: com.google.android.exoplayer2.upstream.DataSpec, param3: number);
							public constructor(param0: string, param1: number);
							public constructor(param0: number);
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSpec, param1: number, param2: number);
							/** @deprecated */
							public constructor(param0: java.io.IOException, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: number);
							public constructor(param0: string, param1: com.google.android.exoplayer2.upstream.DataSpec);
							public constructor(param0: string, param1: java.io.IOException, param2: com.google.android.exoplayer2.upstream.DataSpec, param3: number, param4: number);
							/** @deprecated */
							public constructor(param0: string, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: number);
							public constructor(param0: string, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: number, param3: number);
							public constructor(param0: string, param1: java.lang.Throwable, param2: number);
						}
						export class InvalidResponseCodeException extends com.google.android.exoplayer2.upstream.HttpDataSource.HttpDataSourceException {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.HttpDataSource.InvalidResponseCodeException>;
							public responseCode: number;
							public responseMessage: string;
							public headerFields: java.util.Map<string,java.util.List<string>>;
							public responseBody: androidNative.Array<number>;
							/** @deprecated */
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSpec, param1: number);
							public constructor(param0: java.lang.Throwable, param1: number);
							public constructor(param0: number, param1: string, param2: java.io.IOException, param3: java.util.Map<string,java.util.List<string>>, param4: com.google.android.exoplayer2.upstream.DataSpec, param5: androidNative.Array<number>);
							public constructor(param0: java.io.IOException, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: number, param3: number);
							/** @deprecated */
							public constructor(param0: string, param1: java.io.IOException, param2: com.google.android.exoplayer2.upstream.DataSpec, param3: number);
							/** @deprecated */
							public constructor(param0: number, param1: java.util.Map<string,java.util.List<string>>, param2: com.google.android.exoplayer2.upstream.DataSpec);
							public constructor(param0: string, param1: number);
							/** @deprecated */
							public constructor(param0: number, param1: string, param2: java.util.Map<string,java.util.List<string>>, param3: com.google.android.exoplayer2.upstream.DataSpec);
							public constructor(param0: number);
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSpec, param1: number, param2: number);
							/** @deprecated */
							public constructor(param0: java.io.IOException, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: number);
							public constructor(param0: string, param1: java.io.IOException, param2: com.google.android.exoplayer2.upstream.DataSpec, param3: number, param4: number);
							/** @deprecated */
							public constructor(param0: string, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: number);
							public constructor(param0: string, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: number, param3: number);
							public constructor(param0: string, param1: java.lang.Throwable, param2: number);
						}
						export class RequestProperties {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.HttpDataSource.RequestProperties>;
							public constructor();
							public clearAndSet(param0: java.util.Map<string,string>): void;
							public set(param0: java.util.Map<string,string>): void;
							public clear(): void;
							public set(param0: string, param1: string): void;
							public getSnapshot(): java.util.Map<string,string>;
							public remove(param0: string): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class HttpUtil {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.HttpUtil>;
						public static getDocumentSize(param0: string): number;
						public static buildRangeRequestHeader(param0: number, param1: number): string;
						public static getContentLength(param0: string, param1: string): number;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class LoadErrorHandlingPolicy {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							getFallbackSelectionFor(param0: com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy.FallbackOptions, param1: com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy.LoadErrorInfo): com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy.FallbackSelection;
							getRetryDelayMsFor(param0: com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy.LoadErrorInfo): number;
							onLoadTaskConcluded(param0: number): void;
							getMinimumLoadableRetryCount(param0: number): number;
						});
						public constructor();
						public static FALLBACK_TYPE_LOCATION: number;
						public static FALLBACK_TYPE_TRACK: number;
						public onLoadTaskConcluded(param0: number): void;
						public getFallbackSelectionFor(param0: com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy.FallbackOptions, param1: com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy.LoadErrorInfo): com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy.FallbackSelection;
						public getRetryDelayMsFor(param0: com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy.LoadErrorInfo): number;
						public getMinimumLoadableRetryCount(param0: number): number;
					}
					export module LoadErrorHandlingPolicy {
						export class FallbackOptions {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy.FallbackOptions>;
							public numberOfLocations: number;
							public numberOfExcludedLocations: number;
							public numberOfTracks: number;
							public numberOfExcludedTracks: number;
							public isFallbackAvailable(param0: number): boolean;
							public constructor(param0: number, param1: number, param2: number, param3: number);
						}
						export class FallbackSelection {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy.FallbackSelection>;
							public type: number;
							public exclusionDurationMs: number;
							public constructor(param0: number, param1: number);
						}
						export class FallbackType {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy.FallbackType>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy$FallbackType interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
							});
							public constructor();
						}
						export class LoadErrorInfo {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy.LoadErrorInfo>;
							public loadEventInfo: com.google.android.exoplayer2.source.LoadEventInfo;
							public mediaLoadData: com.google.android.exoplayer2.source.MediaLoadData;
							public exception: java.io.IOException;
							public errorCount: number;
							public constructor(param0: com.google.android.exoplayer2.source.LoadEventInfo, param1: com.google.android.exoplayer2.source.MediaLoadData, param2: java.io.IOException, param3: number);
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class Loader extends com.google.android.exoplayer2.upstream.LoaderErrorThrower {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.Loader>;
						public static RETRY: com.google.android.exoplayer2.upstream.Loader.LoadErrorAction;
						public static RETRY_RESET_ERROR_COUNT: com.google.android.exoplayer2.upstream.Loader.LoadErrorAction;
						public static DONT_RETRY: com.google.android.exoplayer2.upstream.Loader.LoadErrorAction;
						public static DONT_RETRY_FATAL: com.google.android.exoplayer2.upstream.Loader.LoadErrorAction;
						public static createRetryAction(param0: boolean, param1: number): com.google.android.exoplayer2.upstream.Loader.LoadErrorAction;
						public isLoading(): boolean;
						public startLoading(param0: com.google.android.exoplayer2.upstream.Loader.Loadable, param1: com.google.android.exoplayer2.upstream.Loader.Callback<any>, param2: number): number;
						public hasFatalError(): boolean;
						public release(param0: com.google.android.exoplayer2.upstream.Loader.ReleaseCallback): void;
						public maybeThrowError(param0: number): void;
						public clearFatalError(): void;
						public release(): void;
						public maybeThrowError(): void;
						public cancelLoading(): void;
						public constructor(param0: string);
					}
					export module Loader {
						export class Callback<T>  extends java.lang.Object {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.Loader.Callback<any>>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.upstream.Loader$Callback interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onLoadCompleted(param0: T, param1: number, param2: number): void;
								onLoadCanceled(param0: T, param1: number, param2: number, param3: boolean): void;
								onLoadError(param0: T, param1: number, param2: number, param3: java.io.IOException, param4: number): com.google.android.exoplayer2.upstream.Loader.LoadErrorAction;
							});
							public constructor();
							public onLoadError(param0: T, param1: number, param2: number, param3: java.io.IOException, param4: number): com.google.android.exoplayer2.upstream.Loader.LoadErrorAction;
							public onLoadCompleted(param0: T, param1: number, param2: number): void;
							public onLoadCanceled(param0: T, param1: number, param2: number, param3: boolean): void;
						}
						export class LoadErrorAction {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.Loader.LoadErrorAction>;
							public isRetry(): boolean;
						}
						export class LoadTask<T>  extends globalAndroid.os.Handler implements java.lang.Runnable  {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.Loader.LoadTask<any>>;
							public defaultMinRetryCount: number;
							public constructor(param0: globalAndroid.os.Looper, param1: any, param2: com.google.android.exoplayer2.upstream.Loader.Callback<any>, param3: number, param4: number);
							public maybeThrowError(param0: number): void;
							public cancel(param0: boolean): void;
							public start(param0: number): void;
							public handleMessage(param0: globalAndroid.os.Message): void;
							public run(): void;
						}
						export class Loadable {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.Loader.Loadable>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.upstream.Loader$Loadable interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								cancelLoad(): void;
								load(): void;
							});
							public constructor();
							public cancelLoad(): void;
							public load(): void;
						}
						export class ReleaseCallback {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.Loader.ReleaseCallback>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.upstream.Loader$ReleaseCallback interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onLoaderReleased(): void;
							});
							public constructor();
							public onLoaderReleased(): void;
						}
						export class ReleaseTask {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.Loader.ReleaseTask>;
							public constructor(param0: com.google.android.exoplayer2.upstream.Loader.ReleaseCallback);
							public run(): void;
						}
						export class UnexpectedLoaderException {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.Loader.UnexpectedLoaderException>;
							public constructor(param0: java.lang.Throwable);
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class LoaderErrorThrower {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.LoaderErrorThrower>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.upstream.LoaderErrorThrower interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							maybeThrowError(): void;
							maybeThrowError(param0: number): void;
						});
						public constructor();
						public maybeThrowError(param0: number): void;
						public maybeThrowError(): void;
					}
					export module LoaderErrorThrower {
						export class Dummy extends com.google.android.exoplayer2.upstream.LoaderErrorThrower {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.LoaderErrorThrower.Dummy>;
							public constructor();
							public maybeThrowError(param0: number): void;
							public maybeThrowError(): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class ParsingLoadable<T>  extends com.google.android.exoplayer2.upstream.Loader.Loadable {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.ParsingLoadable<any>>;
						public loadTaskId: number;
						public dataSpec: com.google.android.exoplayer2.upstream.DataSpec;
						public type: number;
						public getResponseHeaders(): java.util.Map<string,java.util.List<string>>;
						public static load(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.ParsingLoadable.Parser<any>, param2: com.google.android.exoplayer2.upstream.DataSpec, param3: number): any;
						public cancelLoad(): void;
						public static load(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.ParsingLoadable.Parser<any>, param2: globalAndroid.net.Uri, param3: number): any;
						public constructor(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: number, param3: com.google.android.exoplayer2.upstream.ParsingLoadable.Parser<any>);
						public getResult(): any;
						public load(): void;
						public bytesLoaded(): number;
						public constructor(param0: com.google.android.exoplayer2.upstream.DataSource, param1: globalAndroid.net.Uri, param2: number, param3: com.google.android.exoplayer2.upstream.ParsingLoadable.Parser<any>);
						public getUri(): globalAndroid.net.Uri;
					}
					export module ParsingLoadable {
						export class Parser<T>  extends java.lang.Object {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.ParsingLoadable.Parser<any>>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.upstream.ParsingLoadable$Parser interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								parse(param0: globalAndroid.net.Uri, param1: java.io.InputStream): T;
							});
							public constructor();
							public parse(param0: globalAndroid.net.Uri, param1: java.io.InputStream): T;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class PriorityDataSource extends com.google.android.exoplayer2.upstream.DataSource {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.PriorityDataSource>;
						public getResponseHeaders(): java.util.Map<string,java.util.List<string>>;
						public open(param0: com.google.android.exoplayer2.upstream.DataSpec): number;
						public close(): void;
						public read(param0: androidNative.Array<number>, param1: number, param2: number): number;
						public constructor(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.util.PriorityTaskManager, param2: number);
						public addTransferListener(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
						public getUri(): globalAndroid.net.Uri;
					}
					export module PriorityDataSource {
						export class Factory extends com.google.android.exoplayer2.upstream.DataSource.Factory {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.PriorityDataSource.Factory>;
							public createDataSource(): com.google.android.exoplayer2.upstream.DataSource;
							public createDataSource(): com.google.android.exoplayer2.upstream.PriorityDataSource;
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSource.Factory, param1: com.google.android.exoplayer2.util.PriorityTaskManager, param2: number);
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class PriorityDataSourceFactory extends com.google.android.exoplayer2.upstream.DataSource.Factory {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.PriorityDataSourceFactory>;
						public createDataSource(): com.google.android.exoplayer2.upstream.PriorityDataSource;
						public createDataSource(): com.google.android.exoplayer2.upstream.DataSource;
						public constructor(param0: com.google.android.exoplayer2.upstream.DataSource.Factory, param1: com.google.android.exoplayer2.util.PriorityTaskManager, param2: number);
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class RawResourceDataSource extends com.google.android.exoplayer2.upstream.BaseDataSource {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.RawResourceDataSource>;
						public static RAW_RESOURCE_SCHEME: string;
						public getResponseHeaders(): java.util.Map<string,java.util.List<string>>;
						public open(param0: com.google.android.exoplayer2.upstream.DataSpec): number;
						public constructor(param0: globalAndroid.content.Context);
						public close(): void;
						public read(param0: androidNative.Array<number>, param1: number, param2: number): number;
						public constructor(param0: boolean);
						public static buildRawResourceUri(param0: number): globalAndroid.net.Uri;
						public getUri(): globalAndroid.net.Uri;
						public addTransferListener(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
					}
					export module RawResourceDataSource {
						export class RawResourceDataSourceException extends com.google.android.exoplayer2.upstream.DataSourceException {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.RawResourceDataSource.RawResourceDataSourceException>;
							/** @deprecated */
							public constructor(param0: java.lang.Throwable);
							public constructor(param0: java.lang.Throwable, param1: number);
							/** @deprecated */
							public constructor(param0: string);
							public constructor(param0: string, param1: java.lang.Throwable, param2: number);
							public constructor(param0: string, param1: number);
							public constructor(param0: number);
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class ResolvingDataSource extends com.google.android.exoplayer2.upstream.DataSource {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.ResolvingDataSource>;
						public getResponseHeaders(): java.util.Map<string,java.util.List<string>>;
						public open(param0: com.google.android.exoplayer2.upstream.DataSpec): number;
						public close(): void;
						public read(param0: androidNative.Array<number>, param1: number, param2: number): number;
						public constructor(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.ResolvingDataSource.Resolver);
						public addTransferListener(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
						public getUri(): globalAndroid.net.Uri;
					}
					export module ResolvingDataSource {
						export class Factory extends com.google.android.exoplayer2.upstream.DataSource.Factory {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.ResolvingDataSource.Factory>;
							public constructor(param0: com.google.android.exoplayer2.upstream.DataSource.Factory, param1: com.google.android.exoplayer2.upstream.ResolvingDataSource.Resolver);
							public createDataSource(): com.google.android.exoplayer2.upstream.DataSource;
							public createDataSource(): com.google.android.exoplayer2.upstream.ResolvingDataSource;
						}
						export class Resolver {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.ResolvingDataSource.Resolver>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.upstream.ResolvingDataSource$Resolver interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								resolveDataSpec(param0: com.google.android.exoplayer2.upstream.DataSpec): com.google.android.exoplayer2.upstream.DataSpec;
								resolveReportedUri(param0: globalAndroid.net.Uri): globalAndroid.net.Uri;
							});
							public constructor();
							public resolveReportedUri(param0: globalAndroid.net.Uri): globalAndroid.net.Uri;
							public resolveDataSpec(param0: com.google.android.exoplayer2.upstream.DataSpec): com.google.android.exoplayer2.upstream.DataSpec;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class SlidingPercentile {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.SlidingPercentile>;
						public constructor(param0: number);
						public getPercentile(param0: number): number;
						public addSample(param0: number, param1: number): void;
						public reset(): void;
					}
					export module SlidingPercentile {
						export class Sample {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.SlidingPercentile.Sample>;
							public index: number;
							public weight: number;
							public value: number;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class StatsDataSource extends com.google.android.exoplayer2.upstream.DataSource {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.StatsDataSource>;
						public getResponseHeaders(): java.util.Map<string,java.util.List<string>>;
						public getLastOpenedUri(): globalAndroid.net.Uri;
						public open(param0: com.google.android.exoplayer2.upstream.DataSpec): number;
						public constructor(param0: com.google.android.exoplayer2.upstream.DataSource);
						public resetBytesRead(): void;
						public getLastResponseHeaders(): java.util.Map<string,java.util.List<string>>;
						public close(): void;
						public getBytesRead(): number;
						public read(param0: androidNative.Array<number>, param1: number, param2: number): number;
						public addTransferListener(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
						public getUri(): globalAndroid.net.Uri;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class TeeDataSource extends com.google.android.exoplayer2.upstream.DataSource {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.TeeDataSource>;
						public getResponseHeaders(): java.util.Map<string,java.util.List<string>>;
						public open(param0: com.google.android.exoplayer2.upstream.DataSpec): number;
						public constructor(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSink);
						public close(): void;
						public read(param0: androidNative.Array<number>, param1: number, param2: number): number;
						public addTransferListener(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
						public getUri(): globalAndroid.net.Uri;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class TimeToFirstByteEstimator {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.TimeToFirstByteEstimator>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.upstream.TimeToFirstByteEstimator interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							getTimeToFirstByteEstimateUs(): number;
							reset(): void;
							onTransferInitializing(param0: com.google.android.exoplayer2.upstream.DataSpec): void;
							onTransferStart(param0: com.google.android.exoplayer2.upstream.DataSpec): void;
						});
						public constructor();
						public onTransferStart(param0: com.google.android.exoplayer2.upstream.DataSpec): void;
						public getTimeToFirstByteEstimateUs(): number;
						public reset(): void;
						public onTransferInitializing(param0: com.google.android.exoplayer2.upstream.DataSpec): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class TransferListener {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.TransferListener>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.upstream.TransferListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							onTransferInitializing(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: boolean): void;
							onTransferStart(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: boolean): void;
							onBytesTransferred(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: boolean, param3: number): void;
							onTransferEnd(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: boolean): void;
						});
						public constructor();
						public onTransferInitializing(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: boolean): void;
						public onTransferEnd(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: boolean): void;
						public onTransferStart(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: boolean): void;
						public onBytesTransferred(param0: com.google.android.exoplayer2.upstream.DataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: boolean, param3: number): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export class UdpDataSource extends com.google.android.exoplayer2.upstream.BaseDataSource {
						public static class: java.lang.Class<com.google.android.exoplayer2.upstream.UdpDataSource>;
						public static DEFAULT_MAX_PACKET_SIZE: number;
						public static DEFAULT_SOCKET_TIMEOUT_MILLIS: number;
						public static UDP_PORT_UNSET: number;
						public getResponseHeaders(): java.util.Map<string,java.util.List<string>>;
						public constructor();
						public open(param0: com.google.android.exoplayer2.upstream.DataSpec): number;
						public close(): void;
						public constructor(param0: number);
						public read(param0: androidNative.Array<number>, param1: number, param2: number): number;
						public constructor(param0: boolean);
						public getLocalPort(): number;
						public constructor(param0: number, param1: number);
						public getUri(): globalAndroid.net.Uri;
						public addTransferListener(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
					}
					export module UdpDataSource {
						export class UdpDataSourceException extends com.google.android.exoplayer2.upstream.DataSourceException {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.UdpDataSource.UdpDataSourceException>;
							public constructor(param0: java.lang.Throwable, param1: number);
							public constructor(param0: string, param1: java.lang.Throwable, param2: number);
							public constructor(param0: string, param1: number);
							public constructor(param0: number);
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export module cache {
						export class Cache {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.Cache>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.upstream.cache.Cache interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								getUid(): number;
								release(): void;
								addListener(param0: string, param1: com.google.android.exoplayer2.upstream.cache.Cache.Listener): java.util.NavigableSet<com.google.android.exoplayer2.upstream.cache.CacheSpan>;
								removeListener(param0: string, param1: com.google.android.exoplayer2.upstream.cache.Cache.Listener): void;
								getCachedSpans(param0: string): java.util.NavigableSet<com.google.android.exoplayer2.upstream.cache.CacheSpan>;
								getKeys(): java.util.Set<string>;
								getCacheSpace(): number;
								startReadWrite(param0: string, param1: number, param2: number): com.google.android.exoplayer2.upstream.cache.CacheSpan;
								startReadWriteNonBlocking(param0: string, param1: number, param2: number): com.google.android.exoplayer2.upstream.cache.CacheSpan;
								startFile(param0: string, param1: number, param2: number): java.io.File;
								commitFile(param0: java.io.File, param1: number): void;
								releaseHoleSpan(param0: com.google.android.exoplayer2.upstream.cache.CacheSpan): void;
								removeResource(param0: string): void;
								removeSpan(param0: com.google.android.exoplayer2.upstream.cache.CacheSpan): void;
								isCached(param0: string, param1: number, param2: number): boolean;
								getCachedLength(param0: string, param1: number, param2: number): number;
								getCachedBytes(param0: string, param1: number, param2: number): number;
								applyContentMetadataMutations(param0: string, param1: com.google.android.exoplayer2.upstream.cache.ContentMetadataMutations): void;
								getContentMetadata(param0: string): com.google.android.exoplayer2.upstream.cache.ContentMetadata;
							});
							public constructor();
							public static UID_UNSET: number;
							public releaseHoleSpan(param0: com.google.android.exoplayer2.upstream.cache.CacheSpan): void;
							public addListener(param0: string, param1: com.google.android.exoplayer2.upstream.cache.Cache.Listener): java.util.NavigableSet<com.google.android.exoplayer2.upstream.cache.CacheSpan>;
							public removeListener(param0: string, param1: com.google.android.exoplayer2.upstream.cache.Cache.Listener): void;
							public startReadWrite(param0: string, param1: number, param2: number): com.google.android.exoplayer2.upstream.cache.CacheSpan;
							public removeResource(param0: string): void;
							public release(): void;
							public getCacheSpace(): number;
							public removeSpan(param0: com.google.android.exoplayer2.upstream.cache.CacheSpan): void;
							public getCachedSpans(param0: string): java.util.NavigableSet<com.google.android.exoplayer2.upstream.cache.CacheSpan>;
							public getContentMetadata(param0: string): com.google.android.exoplayer2.upstream.cache.ContentMetadata;
							public startReadWriteNonBlocking(param0: string, param1: number, param2: number): com.google.android.exoplayer2.upstream.cache.CacheSpan;
							public getUid(): number;
							public startFile(param0: string, param1: number, param2: number): java.io.File;
							public commitFile(param0: java.io.File, param1: number): void;
							public getCachedBytes(param0: string, param1: number, param2: number): number;
							public isCached(param0: string, param1: number, param2: number): boolean;
							public applyContentMetadataMutations(param0: string, param1: com.google.android.exoplayer2.upstream.cache.ContentMetadataMutations): void;
							public getKeys(): java.util.Set<string>;
							public getCachedLength(param0: string, param1: number, param2: number): number;
						}
						export module Cache {
							export class CacheException {
								public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.Cache.CacheException>;
								public constructor(param0: java.lang.Throwable);
								public constructor(param0: string, param1: java.lang.Throwable);
								public constructor(param0: string);
							}
							export class Listener {
								public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.Cache.Listener>;
								/**
								 * Constructs a new instance of the com.google.android.exoplayer2.upstream.cache.Cache$Listener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
								 */
								public constructor(implementation: {
									onSpanAdded(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: com.google.android.exoplayer2.upstream.cache.CacheSpan): void;
									onSpanRemoved(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: com.google.android.exoplayer2.upstream.cache.CacheSpan): void;
									onSpanTouched(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: com.google.android.exoplayer2.upstream.cache.CacheSpan, param2: com.google.android.exoplayer2.upstream.cache.CacheSpan): void;
								});
								public constructor();
								public onSpanRemoved(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: com.google.android.exoplayer2.upstream.cache.CacheSpan): void;
								public onSpanTouched(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: com.google.android.exoplayer2.upstream.cache.CacheSpan, param2: com.google.android.exoplayer2.upstream.cache.CacheSpan): void;
								public onSpanAdded(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: com.google.android.exoplayer2.upstream.cache.CacheSpan): void;
							}
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export module cache {
						export class CacheDataSink extends com.google.android.exoplayer2.upstream.DataSink {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.CacheDataSink>;
							public static DEFAULT_FRAGMENT_SIZE: number;
							public static DEFAULT_BUFFER_SIZE: number;
							public constructor(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: number, param2: number);
							public close(): void;
							public constructor(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: number);
							public open(param0: com.google.android.exoplayer2.upstream.DataSpec): void;
							public write(param0: androidNative.Array<number>, param1: number, param2: number): void;
						}
						export module CacheDataSink {
							export class CacheDataSinkException extends com.google.android.exoplayer2.upstream.cache.Cache.CacheException {
								public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.CacheDataSink.CacheDataSinkException>;
								public constructor(param0: java.lang.Throwable);
								public constructor(param0: string, param1: java.lang.Throwable);
								public constructor(param0: string);
								public constructor(param0: java.io.IOException);
							}
							export class Factory extends com.google.android.exoplayer2.upstream.DataSink.Factory {
								public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.CacheDataSink.Factory>;
								public createDataSink(): com.google.android.exoplayer2.upstream.DataSink;
								public setBufferSize(param0: number): com.google.android.exoplayer2.upstream.cache.CacheDataSink.Factory;
								public constructor();
								public setCache(param0: com.google.android.exoplayer2.upstream.cache.Cache): com.google.android.exoplayer2.upstream.cache.CacheDataSink.Factory;
								public setFragmentSize(param0: number): com.google.android.exoplayer2.upstream.cache.CacheDataSink.Factory;
							}
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export module cache {
						export class CacheDataSource extends com.google.android.exoplayer2.upstream.DataSource {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.CacheDataSource>;
							public static FLAG_BLOCK_ON_CACHE: number;
							public static FLAG_IGNORE_CACHE_ON_ERROR: number;
							public static FLAG_IGNORE_CACHE_FOR_UNSET_LENGTH_REQUESTS: number;
							public static CACHE_IGNORED_REASON_ERROR: number;
							public static CACHE_IGNORED_REASON_UNSET_LENGTH: number;
							public getUri(): globalAndroid.net.Uri;
							public close(): void;
							public open(param0: com.google.android.exoplayer2.upstream.DataSpec): number;
							public constructor(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: com.google.android.exoplayer2.upstream.DataSource, param2: com.google.android.exoplayer2.upstream.DataSource, param3: com.google.android.exoplayer2.upstream.DataSink, param4: number, param5: com.google.android.exoplayer2.upstream.cache.CacheDataSource.EventListener);
							public constructor(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: com.google.android.exoplayer2.upstream.DataSource, param2: com.google.android.exoplayer2.upstream.DataSource, param3: com.google.android.exoplayer2.upstream.DataSink, param4: number, param5: com.google.android.exoplayer2.upstream.cache.CacheDataSource.EventListener, param6: com.google.android.exoplayer2.upstream.cache.CacheKeyFactory);
							public constructor(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: com.google.android.exoplayer2.upstream.DataSource);
							public addTransferListener(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
							public constructor(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: com.google.android.exoplayer2.upstream.DataSource, param2: number);
							public getCacheKeyFactory(): com.google.android.exoplayer2.upstream.cache.CacheKeyFactory;
							public getResponseHeaders(): java.util.Map<string,java.util.List<string>>;
							public getCache(): com.google.android.exoplayer2.upstream.cache.Cache;
							public read(param0: androidNative.Array<number>, param1: number, param2: number): number;
						}
						export module CacheDataSource {
							export class CacheIgnoredReason {
								public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.CacheDataSource.CacheIgnoredReason>;
								/**
								 * Constructs a new instance of the com.google.android.exoplayer2.upstream.cache.CacheDataSource$CacheIgnoredReason interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
								 */
								public constructor(implementation: {
								});
								public constructor();
							}
							export class EventListener {
								public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.CacheDataSource.EventListener>;
								/**
								 * Constructs a new instance of the com.google.android.exoplayer2.upstream.cache.CacheDataSource$EventListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
								 */
								public constructor(implementation: {
									onCachedBytesRead(param0: number, param1: number): void;
									onCacheIgnored(param0: number): void;
								});
								public constructor();
								public onCachedBytesRead(param0: number, param1: number): void;
								public onCacheIgnored(param0: number): void;
							}
							export class Factory extends com.google.android.exoplayer2.upstream.DataSource.Factory {
								public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.CacheDataSource.Factory>;
								public setUpstreamPriority(param0: number): com.google.android.exoplayer2.upstream.cache.CacheDataSource.Factory;
								public setUpstreamDataSourceFactory(param0: com.google.android.exoplayer2.upstream.DataSource.Factory): com.google.android.exoplayer2.upstream.cache.CacheDataSource.Factory;
								public setEventListener(param0: com.google.android.exoplayer2.upstream.cache.CacheDataSource.EventListener): com.google.android.exoplayer2.upstream.cache.CacheDataSource.Factory;
								public constructor();
								public getUpstreamPriorityTaskManager(): com.google.android.exoplayer2.util.PriorityTaskManager;
								public getCache(): com.google.android.exoplayer2.upstream.cache.Cache;
								public createDataSource(): com.google.android.exoplayer2.upstream.DataSource;
								public setCacheWriteDataSinkFactory(param0: com.google.android.exoplayer2.upstream.DataSink.Factory): com.google.android.exoplayer2.upstream.cache.CacheDataSource.Factory;
								public setCache(param0: com.google.android.exoplayer2.upstream.cache.Cache): com.google.android.exoplayer2.upstream.cache.CacheDataSource.Factory;
								public setFlags(param0: number): com.google.android.exoplayer2.upstream.cache.CacheDataSource.Factory;
								public setCacheReadDataSourceFactory(param0: com.google.android.exoplayer2.upstream.DataSource.Factory): com.google.android.exoplayer2.upstream.cache.CacheDataSource.Factory;
								public createDataSourceForRemovingDownload(): com.google.android.exoplayer2.upstream.cache.CacheDataSource;
								public getCacheKeyFactory(): com.google.android.exoplayer2.upstream.cache.CacheKeyFactory;
								public createDataSource(): com.google.android.exoplayer2.upstream.cache.CacheDataSource;
								public createDataSourceForDownloading(): com.google.android.exoplayer2.upstream.cache.CacheDataSource;
								public setCacheKeyFactory(param0: com.google.android.exoplayer2.upstream.cache.CacheKeyFactory): com.google.android.exoplayer2.upstream.cache.CacheDataSource.Factory;
								public setUpstreamPriorityTaskManager(param0: com.google.android.exoplayer2.util.PriorityTaskManager): com.google.android.exoplayer2.upstream.cache.CacheDataSource.Factory;
							}
							export class Flags {
								public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.CacheDataSource.Flags>;
								/**
								 * Constructs a new instance of the com.google.android.exoplayer2.upstream.cache.CacheDataSource$Flags interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
								 */
								public constructor(implementation: {
								});
								public constructor();
							}
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export module cache {
						export class CacheEvictor extends com.google.android.exoplayer2.upstream.cache.Cache.Listener {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.CacheEvictor>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.upstream.cache.CacheEvictor interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								requiresCacheSpanTouches(): boolean;
								onCacheInitialized(): void;
								onStartFile(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: string, param2: number, param3: number): void;
								onSpanAdded(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: com.google.android.exoplayer2.upstream.cache.CacheSpan): void;
								onSpanRemoved(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: com.google.android.exoplayer2.upstream.cache.CacheSpan): void;
								onSpanTouched(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: com.google.android.exoplayer2.upstream.cache.CacheSpan, param2: com.google.android.exoplayer2.upstream.cache.CacheSpan): void;
							});
							public constructor();
							public onStartFile(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: string, param2: number, param3: number): void;
							public onSpanAdded(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: com.google.android.exoplayer2.upstream.cache.CacheSpan): void;
							public onSpanTouched(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: com.google.android.exoplayer2.upstream.cache.CacheSpan, param2: com.google.android.exoplayer2.upstream.cache.CacheSpan): void;
							public requiresCacheSpanTouches(): boolean;
							public onSpanRemoved(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: com.google.android.exoplayer2.upstream.cache.CacheSpan): void;
							public onCacheInitialized(): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export module cache {
						export class CacheFileMetadata {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.CacheFileMetadata>;
							public length: number;
							public lastTouchTimestamp: number;
							public constructor(param0: number, param1: number);
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export module cache {
						export class CacheFileMetadataIndex {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.CacheFileMetadataIndex>;
							public initialize(param0: number): void;
							public getAll(): java.util.Map<string,com.google.android.exoplayer2.upstream.cache.CacheFileMetadata>;
							public constructor(param0: com.google.android.exoplayer2.database.DatabaseProvider);
							public set(param0: string, param1: number, param2: number): void;
							public static delete(param0: com.google.android.exoplayer2.database.DatabaseProvider, param1: number): void;
							public removeAll(param0: java.util.Set<string>): void;
							public remove(param0: string): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export module cache {
						export class CacheKeyFactory {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.CacheKeyFactory>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.upstream.cache.CacheKeyFactory interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								buildCacheKey(param0: com.google.android.exoplayer2.upstream.DataSpec): string;
								lambda$static$0(param0: com.google.android.exoplayer2.upstream.DataSpec): string;
								<clinit>(): void;
							});
							public constructor();
							public static DEFAULT: com.google.android.exoplayer2.upstream.cache.CacheKeyFactory;
							public buildCacheKey(param0: com.google.android.exoplayer2.upstream.DataSpec): string;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export module cache {
						export class CacheSpan extends java.lang.Comparable<com.google.android.exoplayer2.upstream.cache.CacheSpan> {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.CacheSpan>;
							public key: string;
							public position: number;
							public length: number;
							public isCached: boolean;
							public file: java.io.File;
							public lastTouchTimestamp: number;
							public isOpenEnded(): boolean;
							public isHoleSpan(): boolean;
							public constructor(param0: string, param1: number, param2: number);
							public toString(): string;
							public constructor(param0: string, param1: number, param2: number, param3: number, param4: java.io.File);
							public compareTo(param0: com.google.android.exoplayer2.upstream.cache.CacheSpan): number;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export module cache {
						export class CacheWriter {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.CacheWriter>;
							public static DEFAULT_BUFFER_SIZE_BYTES: number;
							public cancel(): void;
							public cache(): void;
							public constructor(param0: com.google.android.exoplayer2.upstream.cache.CacheDataSource, param1: com.google.android.exoplayer2.upstream.DataSpec, param2: androidNative.Array<number>, param3: com.google.android.exoplayer2.upstream.cache.CacheWriter.ProgressListener);
						}
						export module CacheWriter {
							export class ProgressListener {
								public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.CacheWriter.ProgressListener>;
								/**
								 * Constructs a new instance of the com.google.android.exoplayer2.upstream.cache.CacheWriter$ProgressListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
								 */
								public constructor(implementation: {
									onProgress(param0: number, param1: number, param2: number): void;
								});
								public constructor();
								public onProgress(param0: number, param1: number, param2: number): void;
							}
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export module cache {
						export class CachedContent {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.CachedContent>;
							public id: number;
							public key: string;
							public applyMetadataMutations(param0: com.google.android.exoplayer2.upstream.cache.ContentMetadataMutations): boolean;
							public hashCode(): number;
							public isFullyUnlocked(): boolean;
							public getMetadata(): com.google.android.exoplayer2.upstream.cache.DefaultContentMetadata;
							public lockRange(param0: number, param1: number): boolean;
							public getCachedBytesLength(param0: number, param1: number): number;
							public isFullyLocked(param0: number, param1: number): boolean;
							public addSpan(param0: com.google.android.exoplayer2.upstream.cache.SimpleCacheSpan): void;
							public setLastTouchTimestamp(param0: com.google.android.exoplayer2.upstream.cache.SimpleCacheSpan, param1: number, param2: boolean): com.google.android.exoplayer2.upstream.cache.SimpleCacheSpan;
							public unlockRange(param0: number): void;
							public getSpan(param0: number, param1: number): com.google.android.exoplayer2.upstream.cache.SimpleCacheSpan;
							public removeSpan(param0: com.google.android.exoplayer2.upstream.cache.CacheSpan): boolean;
							public constructor(param0: number, param1: string, param2: com.google.android.exoplayer2.upstream.cache.DefaultContentMetadata);
							public isEmpty(): boolean;
							public equals(param0: any): boolean;
							public getSpans(): java.util.TreeSet<com.google.android.exoplayer2.upstream.cache.SimpleCacheSpan>;
							public constructor(param0: number, param1: string);
						}
						export module CachedContent {
							export class Range {
								public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.CachedContent.Range>;
								public position: number;
								public length: number;
								public constructor(param0: number, param1: number);
								public intersects(param0: number, param1: number): boolean;
								public contains(param0: number, param1: number): boolean;
							}
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export module cache {
						export class CachedContentIndex {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.CachedContentIndex>;
							public assignIdForKey(param0: string): number;
							public maybeRemove(param0: string): void;
							public removeEmpty(): void;
							public store(): void;
							public getAll(): java.util.Collection<com.google.android.exoplayer2.upstream.cache.CachedContent>;
							public getContentMetadata(param0: string): com.google.android.exoplayer2.upstream.cache.ContentMetadata;
							public static isIndexFile(param0: string): boolean;
							public getOrAdd(param0: string): com.google.android.exoplayer2.upstream.cache.CachedContent;
							public initialize(param0: number): void;
							public constructor(param0: com.google.android.exoplayer2.database.DatabaseProvider);
							public constructor(param0: com.google.android.exoplayer2.database.DatabaseProvider, param1: java.io.File, param2: androidNative.Array<number>, param3: boolean, param4: boolean);
							public getKeyForId(param0: number): string;
							public static delete(param0: com.google.android.exoplayer2.database.DatabaseProvider, param1: number): void;
							public applyContentMetadataMutations(param0: string, param1: com.google.android.exoplayer2.upstream.cache.ContentMetadataMutations): void;
							public get(param0: string): com.google.android.exoplayer2.upstream.cache.CachedContent;
							public getKeys(): java.util.Set<string>;
						}
						export module CachedContentIndex {
							export class DatabaseStorage extends com.google.android.exoplayer2.upstream.cache.CachedContentIndex.Storage {
								public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.CachedContentIndex.DatabaseStorage>;
								public storeFully(param0: java.util.HashMap<string,com.google.android.exoplayer2.upstream.cache.CachedContent>): void;
								public storeIncremental(param0: java.util.HashMap<string,com.google.android.exoplayer2.upstream.cache.CachedContent>): void;
								public delete(): void;
								public constructor(param0: com.google.android.exoplayer2.database.DatabaseProvider);
								public static delete(param0: com.google.android.exoplayer2.database.DatabaseProvider, param1: number): void;
								public onRemove(param0: com.google.android.exoplayer2.upstream.cache.CachedContent, param1: boolean): void;
								public load(param0: java.util.HashMap<string,com.google.android.exoplayer2.upstream.cache.CachedContent>, param1: globalAndroid.util.SparseArray<string>): void;
								public exists(): boolean;
								public onUpdate(param0: com.google.android.exoplayer2.upstream.cache.CachedContent): void;
								public initialize(param0: number): void;
							}
							export class LegacyStorage extends com.google.android.exoplayer2.upstream.cache.CachedContentIndex.Storage {
								public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.CachedContentIndex.LegacyStorage>;
								public storeFully(param0: java.util.HashMap<string,com.google.android.exoplayer2.upstream.cache.CachedContent>): void;
								public storeIncremental(param0: java.util.HashMap<string,com.google.android.exoplayer2.upstream.cache.CachedContent>): void;
								public delete(): void;
								public onRemove(param0: com.google.android.exoplayer2.upstream.cache.CachedContent, param1: boolean): void;
								public constructor(param0: java.io.File, param1: androidNative.Array<number>, param2: boolean);
								public load(param0: java.util.HashMap<string,com.google.android.exoplayer2.upstream.cache.CachedContent>, param1: globalAndroid.util.SparseArray<string>): void;
								public exists(): boolean;
								public onUpdate(param0: com.google.android.exoplayer2.upstream.cache.CachedContent): void;
								public initialize(param0: number): void;
							}
							export class Storage {
								public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.CachedContentIndex.Storage>;
								/**
								 * Constructs a new instance of the com.google.android.exoplayer2.upstream.cache.CachedContentIndex$Storage interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
								 */
								public constructor(implementation: {
									initialize(param0: number): void;
									exists(): boolean;
									delete(): void;
									load(param0: java.util.HashMap<string,com.google.android.exoplayer2.upstream.cache.CachedContent>, param1: globalAndroid.util.SparseArray<string>): void;
									storeFully(param0: java.util.HashMap<string,com.google.android.exoplayer2.upstream.cache.CachedContent>): void;
									storeIncremental(param0: java.util.HashMap<string,com.google.android.exoplayer2.upstream.cache.CachedContent>): void;
									onUpdate(param0: com.google.android.exoplayer2.upstream.cache.CachedContent): void;
									onRemove(param0: com.google.android.exoplayer2.upstream.cache.CachedContent, param1: boolean): void;
								});
								public constructor();
								public storeFully(param0: java.util.HashMap<string,com.google.android.exoplayer2.upstream.cache.CachedContent>): void;
								public storeIncremental(param0: java.util.HashMap<string,com.google.android.exoplayer2.upstream.cache.CachedContent>): void;
								public delete(): void;
								public onRemove(param0: com.google.android.exoplayer2.upstream.cache.CachedContent, param1: boolean): void;
								public load(param0: java.util.HashMap<string,com.google.android.exoplayer2.upstream.cache.CachedContent>, param1: globalAndroid.util.SparseArray<string>): void;
								public exists(): boolean;
								public onUpdate(param0: com.google.android.exoplayer2.upstream.cache.CachedContent): void;
								public initialize(param0: number): void;
							}
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export module cache {
						export class ContentMetadata {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.ContentMetadata>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.upstream.cache.ContentMetadata interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								get(param0: string, param1: androidNative.Array<number>): androidNative.Array<number>;
								get(param0: string, param1: string): string;
								get(param0: string, param1: number): number;
								contains(param0: string): boolean;
								getContentLength(param0: com.google.android.exoplayer2.upstream.cache.ContentMetadata): number;
								getRedirectedUri(param0: com.google.android.exoplayer2.upstream.cache.ContentMetadata): globalAndroid.net.Uri;
							});
							public constructor();
							public static KEY_CUSTOM_PREFIX: string;
							public static KEY_CONTENT_LENGTH: string;
							public static KEY_REDIRECTED_URI: string;
							public get(param0: string, param1: number): number;
							public get(param0: string, param1: androidNative.Array<number>): androidNative.Array<number>;
							public static getRedirectedUri(param0: com.google.android.exoplayer2.upstream.cache.ContentMetadata): globalAndroid.net.Uri;
							public static getContentLength(param0: com.google.android.exoplayer2.upstream.cache.ContentMetadata): number;
							public contains(param0: string): boolean;
							public get(param0: string, param1: string): string;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export module cache {
						export class ContentMetadataMutations {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.ContentMetadataMutations>;
							public constructor();
							public static setContentLength(param0: com.google.android.exoplayer2.upstream.cache.ContentMetadataMutations, param1: number): com.google.android.exoplayer2.upstream.cache.ContentMetadataMutations;
							public remove(param0: string): com.google.android.exoplayer2.upstream.cache.ContentMetadataMutations;
							public set(param0: string, param1: androidNative.Array<number>): com.google.android.exoplayer2.upstream.cache.ContentMetadataMutations;
							public static setRedirectedUri(param0: com.google.android.exoplayer2.upstream.cache.ContentMetadataMutations, param1: globalAndroid.net.Uri): com.google.android.exoplayer2.upstream.cache.ContentMetadataMutations;
							public getEditedValues(): java.util.Map<string,any>;
							public set(param0: string, param1: string): com.google.android.exoplayer2.upstream.cache.ContentMetadataMutations;
							public set(param0: string, param1: number): com.google.android.exoplayer2.upstream.cache.ContentMetadataMutations;
							public getRemovedValues(): java.util.List<string>;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export module cache {
						export class DefaultContentMetadata extends com.google.android.exoplayer2.upstream.cache.ContentMetadata {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.DefaultContentMetadata>;
							public static EMPTY: com.google.android.exoplayer2.upstream.cache.DefaultContentMetadata;
							public constructor();
							public hashCode(): number;
							public get(param0: string, param1: number): number;
							public get(param0: string, param1: androidNative.Array<number>): androidNative.Array<number>;
							public static getRedirectedUri(param0: com.google.android.exoplayer2.upstream.cache.ContentMetadata): globalAndroid.net.Uri;
							public static getContentLength(param0: com.google.android.exoplayer2.upstream.cache.ContentMetadata): number;
							public entrySet(): java.util.Set<java.util.Map.Entry<string,androidNative.Array<number>>>;
							public copyWithMutationsApplied(param0: com.google.android.exoplayer2.upstream.cache.ContentMetadataMutations): com.google.android.exoplayer2.upstream.cache.DefaultContentMetadata;
							public contains(param0: string): boolean;
							public equals(param0: any): boolean;
							public get(param0: string, param1: string): string;
							public constructor(param0: java.util.Map<string,androidNative.Array<number>>);
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export module cache {
						export class LeastRecentlyUsedCacheEvictor extends com.google.android.exoplayer2.upstream.cache.CacheEvictor {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.LeastRecentlyUsedCacheEvictor>;
							public onStartFile(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: string, param2: number, param3: number): void;
							public onSpanAdded(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: com.google.android.exoplayer2.upstream.cache.CacheSpan): void;
							public onSpanTouched(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: com.google.android.exoplayer2.upstream.cache.CacheSpan, param2: com.google.android.exoplayer2.upstream.cache.CacheSpan): void;
							public requiresCacheSpanTouches(): boolean;
							public onSpanRemoved(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: com.google.android.exoplayer2.upstream.cache.CacheSpan): void;
							public onCacheInitialized(): void;
							public constructor(param0: number);
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export module cache {
						export class NoOpCacheEvictor extends com.google.android.exoplayer2.upstream.cache.CacheEvictor {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.NoOpCacheEvictor>;
							public constructor();
							public onStartFile(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: string, param2: number, param3: number): void;
							public onSpanAdded(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: com.google.android.exoplayer2.upstream.cache.CacheSpan): void;
							public onSpanTouched(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: com.google.android.exoplayer2.upstream.cache.CacheSpan, param2: com.google.android.exoplayer2.upstream.cache.CacheSpan): void;
							public requiresCacheSpanTouches(): boolean;
							public onSpanRemoved(param0: com.google.android.exoplayer2.upstream.cache.Cache, param1: com.google.android.exoplayer2.upstream.cache.CacheSpan): void;
							public onCacheInitialized(): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export module cache {
						export class ReusableBufferedOutputStream {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.ReusableBufferedOutputStream>;
							public constructor(param0: java.io.OutputStream);
							public close(): void;
							public reset(param0: java.io.OutputStream): void;
							public constructor(param0: java.io.OutputStream, param1: number);
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export module cache {
						export class SimpleCache extends com.google.android.exoplayer2.upstream.cache.Cache {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.SimpleCache>;
							public addListener(param0: string, param1: com.google.android.exoplayer2.upstream.cache.Cache.Listener): java.util.NavigableSet<com.google.android.exoplayer2.upstream.cache.CacheSpan>;
							public removeListener(param0: string, param1: com.google.android.exoplayer2.upstream.cache.Cache.Listener): void;
							public getCacheSpace(): number;
							public getCachedSpans(param0: string): java.util.NavigableSet<com.google.android.exoplayer2.upstream.cache.CacheSpan>;
							public getContentMetadata(param0: string): com.google.android.exoplayer2.upstream.cache.ContentMetadata;
							public startReadWriteNonBlocking(param0: string, param1: number, param2: number): com.google.android.exoplayer2.upstream.cache.CacheSpan;
							public getUid(): number;
							public getCachedBytes(param0: string, param1: number, param2: number): number;
							public isCached(param0: string, param1: number, param2: number): boolean;
							public constructor(param0: java.io.File, param1: com.google.android.exoplayer2.upstream.cache.CacheEvictor, param2: com.google.android.exoplayer2.database.DatabaseProvider);
							/** @deprecated */
							public constructor(param0: java.io.File, param1: com.google.android.exoplayer2.upstream.cache.CacheEvictor, param2: androidNative.Array<number>);
							public getKeys(): java.util.Set<string>;
							/** @deprecated */
							public constructor(param0: java.io.File, param1: com.google.android.exoplayer2.upstream.cache.CacheEvictor);
							public static isCacheFolderLocked(param0: java.io.File): boolean;
							public releaseHoleSpan(param0: com.google.android.exoplayer2.upstream.cache.CacheSpan): void;
							public checkInitialization(): void;
							/** @deprecated */
							public constructor(param0: java.io.File, param1: com.google.android.exoplayer2.upstream.cache.CacheEvictor, param2: androidNative.Array<number>, param3: boolean);
							public startReadWrite(param0: string, param1: number, param2: number): com.google.android.exoplayer2.upstream.cache.CacheSpan;
							public removeResource(param0: string): void;
							public release(): void;
							public removeSpan(param0: com.google.android.exoplayer2.upstream.cache.CacheSpan): void;
							public static delete(param0: java.io.File, param1: com.google.android.exoplayer2.database.DatabaseProvider): void;
							public startFile(param0: string, param1: number, param2: number): java.io.File;
							public commitFile(param0: java.io.File, param1: number): void;
							public applyContentMetadataMutations(param0: string, param1: com.google.android.exoplayer2.upstream.cache.ContentMetadataMutations): void;
							public constructor(param0: java.io.File, param1: com.google.android.exoplayer2.upstream.cache.CacheEvictor, param2: com.google.android.exoplayer2.database.DatabaseProvider, param3: androidNative.Array<number>, param4: boolean, param5: boolean);
							public getCachedLength(param0: string, param1: number, param2: number): number;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export module cache {
						export class SimpleCacheSpan extends com.google.android.exoplayer2.upstream.cache.CacheSpan {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.cache.SimpleCacheSpan>;
							public static createLookup(param0: string, param1: number): com.google.android.exoplayer2.upstream.cache.SimpleCacheSpan;
							public static createCacheEntry(param0: java.io.File, param1: number, param2: com.google.android.exoplayer2.upstream.cache.CachedContentIndex): com.google.android.exoplayer2.upstream.cache.SimpleCacheSpan;
							public copyWithFileAndLastTouchTimestamp(param0: java.io.File, param1: number): com.google.android.exoplayer2.upstream.cache.SimpleCacheSpan;
							public static getCacheFile(param0: java.io.File, param1: number, param2: number, param3: number): java.io.File;
							public static createHole(param0: string, param1: number, param2: number): com.google.android.exoplayer2.upstream.cache.SimpleCacheSpan;
							public static createCacheEntry(param0: java.io.File, param1: number, param2: number, param3: com.google.android.exoplayer2.upstream.cache.CachedContentIndex): com.google.android.exoplayer2.upstream.cache.SimpleCacheSpan;
						}
					}
				}
			}
		}
	}
}


declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export module crypto {
						export class AesCipherDataSink extends com.google.android.exoplayer2.upstream.DataSink {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.crypto.AesCipherDataSink>;
							public constructor(param0: androidNative.Array<number>, param1: com.google.android.exoplayer2.upstream.DataSink, param2: androidNative.Array<number>);
							public constructor(param0: androidNative.Array<number>, param1: com.google.android.exoplayer2.upstream.DataSink);
							public close(): void;
							public open(param0: com.google.android.exoplayer2.upstream.DataSpec): void;
							public write(param0: androidNative.Array<number>, param1: number, param2: number): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export module crypto {
						export class AesCipherDataSource extends com.google.android.exoplayer2.upstream.DataSource {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.crypto.AesCipherDataSource>;
							public getUri(): globalAndroid.net.Uri;
							public close(): void;
							public open(param0: com.google.android.exoplayer2.upstream.DataSpec): number;
							public constructor(param0: androidNative.Array<number>, param1: com.google.android.exoplayer2.upstream.DataSource);
							public addTransferListener(param0: com.google.android.exoplayer2.upstream.TransferListener): void;
							public getResponseHeaders(): java.util.Map<string,java.util.List<string>>;
							public read(param0: androidNative.Array<number>, param1: number, param2: number): number;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module upstream {
					export module crypto {
						export class AesFlushingCipher {
							public static class: java.lang.Class<com.google.android.exoplayer2.upstream.crypto.AesFlushingCipher>;
							public constructor(param0: number, param1: androidNative.Array<number>, param2: string, param3: number);
							public updateInPlace(param0: androidNative.Array<number>, param1: number, param2: number): void;
							public constructor(param0: number, param1: androidNative.Array<number>, param2: number, param3: number);
							public update(param0: androidNative.Array<number>, param1: number, param2: number, param3: androidNative.Array<number>, param4: number): void;
						}
					}
				}
			}
		}
	}
}



declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module util {
					export class DebugTextViewHelper {
						public static class: java.lang.Class<com.google.android.exoplayer2.util.DebugTextViewHelper>;
						public onPlaybackStateChanged(param0: number): void;
						public onPlayWhenReadyChanged(param0: boolean, param1: number): void;
						public start(): void;
						public run(): void;
						public getDebugString(): string;
						public getVideoString(): string;
						public getAudioString(): string;
						public constructor(param0: com.google.android.exoplayer2.ExoPlayer, param1: globalAndroid.widget.TextView);
						public updateAndPost(): void;
						public onPositionDiscontinuity(param0: com.google.android.exoplayer2.Player.PositionInfo, param1: com.google.android.exoplayer2.Player.PositionInfo, param2: number): void;
						public getPlayerStateString(): string;
						public stop(): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module util {
					export class EventLogger extends com.google.android.exoplayer2.analytics.AnalyticsListener {
						public static class: java.lang.Class<com.google.android.exoplayer2.util.EventLogger>;
						public onVideoEnabled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onTrackSelectionParametersChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.trackselection.TrackSelectionParameters): void;
						public onUpstreamDiscarded(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.MediaLoadData): void;
						/** @deprecated */
						public onSeekStarted(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						/** @deprecated */
						public onVideoDecoderInitialized(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string, param2: number): void;
						public onDrmSessionAcquired(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public loge(param0: string): void;
						public onVolumeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onRenderedFirstFrame(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: any, param2: number): void;
						public onPlayerReleased(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						public onAudioPositionAdvancing(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onTracksInfoChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.TracksInfo): void;
						public onMaxSeekToPreviousPositionChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onVideoInputFormatChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Format, param2: com.google.android.exoplayer2.decoder.DecoderReuseEvaluation): void;
						public onCues(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: java.util.List<com.google.android.exoplayer2.text.Cue>): void;
						public onIsPlayingChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean): void;
						public onMediaItemTransition(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.MediaItem, param2: number): void;
						/** @deprecated */
						public onPlayerStateChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean, param2: number): void;
						public onDroppedVideoFrames(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: number): void;
						public onDrmSessionReleased(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						/** @deprecated */
						public onDecoderInputFormatChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: com.google.android.exoplayer2.Format): void;
						/** @deprecated */
						public onTracksChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.TrackGroupArray, param2: com.google.android.exoplayer2.trackselection.TrackSelectionArray): void;
						public onLoadCanceled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.LoadEventInfo, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
						public logd(param0: string): void;
						public onSeekBackIncrementChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onAudioDecoderReleased(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string): void;
						/** @deprecated */
						public onSeekProcessed(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						public onIsLoadingChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean): void;
						public onTimelineChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onEvents(param0: com.google.android.exoplayer2.Player, param1: com.google.android.exoplayer2.analytics.AnalyticsListener.Events): void;
						public onVideoFrameProcessingOffset(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: number): void;
						/** @deprecated */
						public onAudioInputFormatChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Format): void;
						/** @deprecated */
						public onDecoderDisabled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onDownstreamFormatChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.MediaLoadData): void;
						public onPlayWhenReadyChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean, param2: number): void;
						public onVideoDisabled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onTracksChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.TrackGroupArray, param2: com.google.android.exoplayer2.trackselection.TrackSelectionArray): void;
						public onAudioInputFormatChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Format, param2: com.google.android.exoplayer2.decoder.DecoderReuseEvaluation): void;
						public onLoadStarted(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.LoadEventInfo, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
						public onAudioCodecError(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: java.lang.Exception): void;
						/** @deprecated */
						public onVideoInputFormatChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Format): void;
						public onDrmKeysRemoved(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						public onPlaybackSuppressionReasonChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onBandwidthEstimate(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: number, param3: number): void;
						public onLoadCompleted(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.LoadEventInfo, param2: com.google.android.exoplayer2.source.MediaLoadData): void;
						public onDrmKeysLoaded(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						public onAudioDecoderInitialized(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string, param2: number, param3: number): void;
						public onVideoDecoderInitialized(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string, param2: number, param3: number): void;
						public onPlayerErrorChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.PlaybackException): void;
						public onLoadError(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.source.LoadEventInfo, param2: com.google.android.exoplayer2.source.MediaLoadData, param3: java.io.IOException, param4: boolean): void;
						public onRepeatModeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onShuffleModeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean): void;
						public onPlaylistMetadataChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.MediaMetadata): void;
						/** @deprecated */
						public onAudioDecoderInitialized(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string, param2: number): void;
						public constructor(param0: com.google.android.exoplayer2.trackselection.MappingTrackSelector, param1: string);
						/** @deprecated */
						public onPositionDiscontinuity(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onVideoCodecError(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: java.lang.Exception): void;
						/** @deprecated */
						public onVideoSizeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: number, param3: number, param4: number): void;
						/** @deprecated */
						public onDecoderEnabled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onAudioDisabled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onDeviceInfoChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.DeviceInfo): void;
						public onDrmKeysRestored(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						public onSkipSilenceEnabledChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean): void;
						public onPlayerError(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.PlaybackException): void;
						public onVideoSizeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.video.VideoSize): void;
						public constructor(param0: com.google.android.exoplayer2.trackselection.MappingTrackSelector);
						public onVideoDecoderInitialized(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string, param2: number): void;
						public onPlaybackParametersChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.PlaybackParameters): void;
						public onMediaMetadataChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.MediaMetadata): void;
						public onAudioAttributesChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.audio.AudioAttributes): void;
						public onSurfaceSizeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: number): void;
						/** @deprecated */
						public onLoadingChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: boolean): void;
						public onSeekForwardIncrementChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onDrmSessionManagerError(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: java.lang.Exception): void;
						public onPositionDiscontinuity(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Player.PositionInfo, param2: com.google.android.exoplayer2.Player.PositionInfo, param3: number): void;
						public onAvailableCommandsChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.Player.Commands): void;
						public onPlaybackStateChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onAudioSessionIdChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number): void;
						public onAudioDecoderInitialized(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string, param2: number): void;
						/** @deprecated */
						public onDrmSessionAcquired(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime): void;
						public onAudioEnabled(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onVideoDecoderReleased(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: string): void;
						public onAudioSinkError(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: java.lang.Exception): void;
						public onMetadata(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: com.google.android.exoplayer2.metadata.Metadata): void;
						public onAudioUnderrun(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: number, param3: number): void;
						/** @deprecated */
						public onDecoderInitialized(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: string, param3: number): void;
						public onDeviceVolumeChanged(param0: com.google.android.exoplayer2.analytics.AnalyticsListener.EventTime, param1: number, param2: boolean): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module util {
					export class MediaClock {
						public static class: java.lang.Class<com.google.android.exoplayer2.util.MediaClock>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.util.MediaClock interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							getPositionUs(): number;
							setPlaybackParameters(param0: com.google.android.exoplayer2.PlaybackParameters): void;
							getPlaybackParameters(): com.google.android.exoplayer2.PlaybackParameters;
						});
						public constructor();
						public getPositionUs(): number;
						public getPlaybackParameters(): com.google.android.exoplayer2.PlaybackParameters;
						public setPlaybackParameters(param0: com.google.android.exoplayer2.PlaybackParameters): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module util {
					export class SntpClient {
						public static class: java.lang.Class<com.google.android.exoplayer2.util.SntpClient>;
						public static DEFAULT_NTP_HOST: string;
						public static initialize(param0: com.google.android.exoplayer2.upstream.Loader, param1: com.google.android.exoplayer2.util.SntpClient.InitializationCallback): void;
						public static setNtpHost(param0: string): void;
						public static getElapsedRealtimeOffsetMs(): number;
						public static getNtpHost(): string;
						public static isInitialized(): boolean;
					}
					export module SntpClient {
						export class InitializationCallback {
							public static class: java.lang.Class<com.google.android.exoplayer2.util.SntpClient.InitializationCallback>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.util.SntpClient$InitializationCallback interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onInitialized(): void;
								onInitializationFailed(param0: java.io.IOException): void;
							});
							public constructor();
							public onInitialized(): void;
							public onInitializationFailed(param0: java.io.IOException): void;
						}
						export class NtpTimeCallback extends com.google.android.exoplayer2.upstream.Loader.Callback<com.google.android.exoplayer2.upstream.Loader.Loadable> {
							public static class: java.lang.Class<com.google.android.exoplayer2.util.SntpClient.NtpTimeCallback>;
							public onLoadCompleted(param0: com.google.android.exoplayer2.upstream.Loader.Loadable, param1: number, param2: number): void;
							public onLoadError(param0: com.google.android.exoplayer2.upstream.Loader.Loadable, param1: number, param2: number, param3: java.io.IOException, param4: number): com.google.android.exoplayer2.upstream.Loader.LoadErrorAction;
							public onLoadError(param0: any, param1: number, param2: number, param3: java.io.IOException, param4: number): com.google.android.exoplayer2.upstream.Loader.LoadErrorAction;
							public onLoadCanceled(param0: com.google.android.exoplayer2.upstream.Loader.Loadable, param1: number, param2: number, param3: boolean): void;
							public onLoadCanceled(param0: any, param1: number, param2: number, param3: boolean): void;
							public onLoadCompleted(param0: any, param1: number, param2: number): void;
							public constructor(param0: com.google.android.exoplayer2.util.SntpClient.InitializationCallback);
						}
						export class NtpTimeLoadable extends com.google.android.exoplayer2.upstream.Loader.Loadable {
							public static class: java.lang.Class<com.google.android.exoplayer2.util.SntpClient.NtpTimeLoadable>;
							public cancelLoad(): void;
							public load(): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module util {
					export class StandaloneMediaClock extends com.google.android.exoplayer2.util.MediaClock {
						public static class: java.lang.Class<com.google.android.exoplayer2.util.StandaloneMediaClock>;
						public getPositionUs(): number;
						public start(): void;
						public constructor(param0: com.google.android.exoplayer2.util.Clock);
						public resetPosition(param0: number): void;
						public getPlaybackParameters(): com.google.android.exoplayer2.PlaybackParameters;
						public setPlaybackParameters(param0: com.google.android.exoplayer2.PlaybackParameters): void;
						public stop(): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module video {
					export abstract class DecoderVideoRenderer extends com.google.android.exoplayer2.BaseRenderer {
						public static class: java.lang.Class<com.google.android.exoplayer2.video.DecoderVideoRenderer>;
						public decoderCounters: com.google.android.exoplayer2.decoder.DecoderCounters;
						public shouldDropBuffersToKeyframe(param0: number, param1: number): boolean;
						public onPositionReset(param0: number, param1: boolean): void;
						public replaceStream(param0: androidNative.Array<com.google.android.exoplayer2.Format>, param1: com.google.android.exoplayer2.source.SampleStream, param2: number, param3: number): void;
						public static getHardwareAccelerationSupport(param0: number): number;
						public canReuseDecoder(param0: string, param1: com.google.android.exoplayer2.Format, param2: com.google.android.exoplayer2.Format): com.google.android.exoplayer2.decoder.DecoderReuseEvaluation;
						public static create(param0: number): number;
						public static getTunnelingSupport(param0: number): number;
						public getStream(): com.google.android.exoplayer2.source.SampleStream;
						public onDisabled(): void;
						public isEnded(): boolean;
						public setDecoderOutputMode(param0: number): void;
						public createDecoder(param0: com.google.android.exoplayer2.Format, param1: com.google.android.exoplayer2.decoder.CryptoConfig): com.google.android.exoplayer2.decoder.Decoder<com.google.android.exoplayer2.decoder.DecoderInputBuffer,any,any>;
						public setPlaybackSpeed(param0: number, param1: number): void;
						public setCurrentStreamFinal(): void;
						public onStreamChanged(param0: androidNative.Array<com.google.android.exoplayer2.Format>, param1: number, param2: number): void;
						public onInputFormatChanged(param0: com.google.android.exoplayer2.FormatHolder): void;
						public renderOutputBufferToSurface(param0: com.google.android.exoplayer2.decoder.VideoDecoderOutputBuffer, param1: globalAndroid.view.Surface): void;
						public isReady(): boolean;
						public disable(): void;
						public handleMessage(param0: number, param1: any): void;
						public setOutput(param0: any): void;
						public supportsFormat(param0: com.google.android.exoplayer2.Format): number;
						public start(): void;
						public getState(): number;
						public renderOutputBuffer(param0: com.google.android.exoplayer2.decoder.VideoDecoderOutputBuffer, param1: number, param2: com.google.android.exoplayer2.Format): void;
						public static create(param0: number, param1: number, param2: number, param3: number, param4: number): number;
						public onStopped(): void;
						public getReadingPositionUs(): number;
						public onProcessedOutputBuffer(param0: number): void;
						public reset(): void;
						public constructor(param0: number, param1: globalAndroid.os.Handler, param2: com.google.android.exoplayer2.video.VideoRendererEventListener, param3: number);
						public skipOutputBuffer(param0: com.google.android.exoplayer2.decoder.VideoDecoderOutputBuffer): void;
						public onQueueInputBuffer(param0: com.google.android.exoplayer2.decoder.DecoderInputBuffer): void;
						public onStarted(): void;
						public flushDecoder(): void;
						public render(param0: number, param1: number): void;
						public getName(): string;
						public releaseDecoder(): void;
						public maybeDropBuffersToKeyframe(param0: number): boolean;
						public getMediaClock(): com.google.android.exoplayer2.util.MediaClock;
						public isCurrentStreamFinal(): boolean;
						public static getAdaptiveSupport(param0: number): number;
						public shouldForceRenderOutputBuffer(param0: number, param1: number): boolean;
						public getCapabilities(): com.google.android.exoplayer2.RendererCapabilities;
						public enable(param0: com.google.android.exoplayer2.RendererConfiguration, param1: androidNative.Array<com.google.android.exoplayer2.Format>, param2: com.google.android.exoplayer2.source.SampleStream, param3: number, param4: boolean, param5: boolean, param6: number, param7: number): void;
						public resetPosition(param0: number): void;
						public dropOutputBuffer(param0: com.google.android.exoplayer2.decoder.VideoDecoderOutputBuffer): void;
						public updateDroppedBufferCounters(param0: number, param1: number): void;
						public init(param0: number, param1: com.google.android.exoplayer2.analytics.PlayerId): void;
						public hasReadStreamToEnd(): boolean;
						public shouldDropOutputBuffer(param0: number, param1: number): boolean;
						public getTrackType(): number;
						public supportsMixedMimeTypeAdaptation(): number;
						public stop(): void;
						public onEnabled(param0: boolean, param1: boolean): void;
						public static getFormatSupport(param0: number): number;
						public static create(param0: number, param1: number, param2: number): number;
						public constructor(param0: number);
						public maybeThrowStreamError(): void;
						public static getDecoderSupport(param0: number): number;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module video {
					export class DummySurface {
						public static class: java.lang.Class<com.google.android.exoplayer2.video.DummySurface>;
						public secure: boolean;
						public static newInstanceV17(param0: globalAndroid.content.Context, param1: boolean): com.google.android.exoplayer2.video.DummySurface;
						public release(): void;
						public static isSecureSupported(param0: globalAndroid.content.Context): boolean;
					}
					export module DummySurface {
						export class DummySurfaceThread {
							public static class: java.lang.Class<com.google.android.exoplayer2.video.DummySurface.DummySurfaceThread>;
							public constructor();
							public init(param0: number): com.google.android.exoplayer2.video.DummySurface;
							public handleMessage(param0: globalAndroid.os.Message): boolean;
							public release(): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module video {
					export class FixedFrameRateEstimator {
						public static class: java.lang.Class<com.google.android.exoplayer2.video.FixedFrameRateEstimator>;
						public static CONSECUTIVE_MATCHING_FRAME_DURATIONS_FOR_SYNC: number;
						public constructor();
						public onNextFrame(param0: number): void;
						public getFramesWithoutSyncCount(): number;
						public isSynced(): boolean;
						public getMatchingFrameDurationSumNs(): number;
						public reset(): void;
						public getFrameDurationNs(): number;
						public getFrameRate(): number;
					}
					export module FixedFrameRateEstimator {
						export class Matcher {
							public static class: java.lang.Class<com.google.android.exoplayer2.video.FixedFrameRateEstimator.Matcher>;
							public constructor();
							public isLastFrameOutlier(): boolean;
							public getMatchingFrameDurationSumNs(): number;
							public isSynced(): boolean;
							public onNextFrame(param0: number): void;
							public getFrameDurationNs(): number;
							public reset(): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module video {
					export class MediaCodecVideoDecoderException extends com.google.android.exoplayer2.mediacodec.MediaCodecDecoderException {
						public static class: java.lang.Class<com.google.android.exoplayer2.video.MediaCodecVideoDecoderException>;
						public surfaceIdentityHashCode: number;
						public isSurfaceValid: boolean;
						public constructor(param0: java.lang.Throwable, param1: com.google.android.exoplayer2.mediacodec.MediaCodecInfo, param2: globalAndroid.view.Surface);
						public constructor(param0: java.lang.Throwable, param1: com.google.android.exoplayer2.mediacodec.MediaCodecInfo);
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module video {
					export class MediaCodecVideoRenderer extends com.google.android.exoplayer2.mediacodec.MediaCodecRenderer {
						public static class: java.lang.Class<com.google.android.exoplayer2.video.MediaCodecVideoRenderer>;
						public supportsFormat(param0: com.google.android.exoplayer2.mediacodec.MediaCodecSelector, param1: com.google.android.exoplayer2.Format): number;
						public renderOutputBufferV21(param0: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter, param1: number, param2: number, param3: number): void;
						public constructor(param0: number, param1: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.Factory, param2: com.google.android.exoplayer2.mediacodec.MediaCodecSelector, param3: boolean, param4: number);
						public onPositionReset(param0: number, param1: boolean): void;
						public replaceStream(param0: androidNative.Array<com.google.android.exoplayer2.Format>, param1: com.google.android.exoplayer2.source.SampleStream, param2: number, param3: number): void;
						public static getMaxInputSize(param0: com.google.android.exoplayer2.mediacodec.MediaCodecInfo, param1: com.google.android.exoplayer2.Format): number;
						public renderOutputBuffer(param0: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter, param1: number, param2: number): void;
						public static getHardwareAccelerationSupport(param0: number): number;
						public static create(param0: number): number;
						public onProcessedTunneledBuffer(param0: number): void;
						public static getTunnelingSupport(param0: number): number;
						public getStream(): com.google.android.exoplayer2.source.SampleStream;
						public onDisabled(): void;
						public isEnded(): boolean;
						public getSurface(): globalAndroid.view.Surface;
						public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.mediacodec.MediaCodecSelector, param2: number);
						public resetCodecStateForFlush(): void;
						public codecNeedsSetOutputSurfaceWorkaround(param0: string): boolean;
						public getDecoderInfos(param0: com.google.android.exoplayer2.mediacodec.MediaCodecSelector, param1: com.google.android.exoplayer2.Format, param2: boolean): java.util.List<com.google.android.exoplayer2.mediacodec.MediaCodecInfo>;
						public setPlaybackSpeed(param0: number, param1: number): void;
						public skipOutputBuffer(param0: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter, param1: number, param2: number): void;
						public setCurrentStreamFinal(): void;
						public createDecoderException(param0: java.lang.Throwable, param1: com.google.android.exoplayer2.mediacodec.MediaCodecInfo): com.google.android.exoplayer2.mediacodec.MediaCodecDecoderException;
						public onCodecInitialized(param0: string, param1: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.Configuration, param2: number, param3: number): void;
						public shouldDropOutputBuffer(param0: number, param1: number, param2: boolean): boolean;
						public isReady(): boolean;
						public disable(): void;
						public processOutputBuffer(param0: number, param1: number, param2: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter, param3: java.nio.ByteBuffer, param4: number, param5: number, param6: number, param7: number, param8: boolean, param9: boolean, param10: com.google.android.exoplayer2.Format): boolean;
						public handleMessage(param0: number, param1: any): void;
						public canReuseCodec(param0: com.google.android.exoplayer2.mediacodec.MediaCodecInfo, param1: com.google.android.exoplayer2.Format, param2: com.google.android.exoplayer2.Format): com.google.android.exoplayer2.decoder.DecoderReuseEvaluation;
						public supportsFormat(param0: com.google.android.exoplayer2.Format): number;
						public start(): void;
						public onCodecError(param0: java.lang.Exception): void;
						public getState(): number;
						public handleInputBufferSupplementalData(param0: com.google.android.exoplayer2.decoder.DecoderInputBuffer): void;
						public onReset(): void;
						public static create(param0: number, param1: number, param2: number, param3: number, param4: number): number;
						public onStopped(): void;
						public getReadingPositionUs(): number;
						public onProcessedOutputBuffer(param0: number): void;
						public reset(): void;
						public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.mediacodec.MediaCodecSelector, param2: number, param3: globalAndroid.os.Handler, param4: com.google.android.exoplayer2.video.VideoRendererEventListener, param5: number);
						public onQueueInputBuffer(param0: com.google.android.exoplayer2.decoder.DecoderInputBuffer): void;
						public getCodecNeedsEosPropagation(): boolean;
						public onStarted(): void;
						public shouldDropBuffersToKeyframe(param0: number, param1: number, param2: boolean): boolean;
						public render(param0: number, param1: number): void;
						public getName(): string;
						public setOutputSurfaceV23(param0: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter, param1: globalAndroid.view.Surface): void;
						public getMediaClock(): com.google.android.exoplayer2.util.MediaClock;
						public isCurrentStreamFinal(): boolean;
						public getCodecMaxValues(param0: com.google.android.exoplayer2.mediacodec.MediaCodecInfo, param1: com.google.android.exoplayer2.Format, param2: androidNative.Array<com.google.android.exoplayer2.Format>): com.google.android.exoplayer2.video.MediaCodecVideoRenderer.CodecMaxValues;
						public getMediaCodecConfiguration(param0: com.google.android.exoplayer2.mediacodec.MediaCodecInfo, param1: com.google.android.exoplayer2.Format, param2: globalAndroid.media.MediaCrypto, param3: number): com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.Configuration;
						public dropOutputBuffer(param0: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter, param1: number, param2: number): void;
						public static getAdaptiveSupport(param0: number): number;
						public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.mediacodec.MediaCodecSelector);
						public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.mediacodec.MediaCodecSelector, param2: number, param3: boolean, param4: globalAndroid.os.Handler, param5: com.google.android.exoplayer2.video.VideoRendererEventListener, param6: number);
						public onCodecReleased(param0: string): void;
						public shouldForceRenderOutputBuffer(param0: number, param1: number): boolean;
						public onInputFormatChanged(param0: com.google.android.exoplayer2.FormatHolder): com.google.android.exoplayer2.decoder.DecoderReuseEvaluation;
						public getCapabilities(): com.google.android.exoplayer2.RendererCapabilities;
						public enable(param0: com.google.android.exoplayer2.RendererConfiguration, param1: androidNative.Array<com.google.android.exoplayer2.Format>, param2: com.google.android.exoplayer2.source.SampleStream, param3: number, param4: boolean, param5: boolean, param6: number, param7: number): void;
						public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.Factory, param2: com.google.android.exoplayer2.mediacodec.MediaCodecSelector, param3: number, param4: boolean, param5: globalAndroid.os.Handler, param6: com.google.android.exoplayer2.video.VideoRendererEventListener, param7: number, param8: number);
						public maybeDropBuffersToKeyframe(param0: number, param1: boolean): boolean;
						public updateVideoFrameProcessingOffsetCounters(param0: number): void;
						public resetPosition(param0: number): void;
						public updateDroppedBufferCounters(param0: number, param1: number): void;
						public getMediaFormat(param0: com.google.android.exoplayer2.Format, param1: string, param2: com.google.android.exoplayer2.video.MediaCodecVideoRenderer.CodecMaxValues, param3: number, param4: boolean, param5: number): globalAndroid.media.MediaFormat;
						public init(param0: number, param1: com.google.android.exoplayer2.analytics.PlayerId): void;
						public hasReadStreamToEnd(): boolean;
						public onProcessedStreamChange(): void;
						public getTrackType(): number;
						public supportsMixedMimeTypeAdaptation(): number;
						public stop(): void;
						public onOutputFormatChanged(param0: com.google.android.exoplayer2.Format, param1: globalAndroid.media.MediaFormat): void;
						public onEnabled(param0: boolean, param1: boolean): void;
						public static getFormatSupport(param0: number): number;
						public shouldInitCodec(param0: com.google.android.exoplayer2.mediacodec.MediaCodecInfo): boolean;
						public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.Factory, param2: com.google.android.exoplayer2.mediacodec.MediaCodecSelector, param3: number, param4: boolean, param5: globalAndroid.os.Handler, param6: com.google.android.exoplayer2.video.VideoRendererEventListener, param7: number);
						public static create(param0: number, param1: number, param2: number): number;
						public constructor(param0: number);
						public maybeThrowStreamError(): void;
						public static getDecoderSupport(param0: number): number;
						public getCodecOperatingRateV23(param0: number, param1: com.google.android.exoplayer2.Format, param2: androidNative.Array<com.google.android.exoplayer2.Format>): number;
					}
					export module MediaCodecVideoRenderer {
						export class CodecMaxValues {
							public static class: java.lang.Class<com.google.android.exoplayer2.video.MediaCodecVideoRenderer.CodecMaxValues>;
							public width: number;
							public height: number;
							public inputSize: number;
							public constructor(param0: number, param1: number, param2: number);
						}
						export class OnFrameRenderedListenerV23 extends com.google.android.exoplayer2.mediacodec.MediaCodecAdapter.OnFrameRenderedListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.video.MediaCodecVideoRenderer.OnFrameRenderedListenerV23>;
							public constructor(param0: com.google.android.exoplayer2.video.MediaCodecVideoRenderer, param1: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter);
							public handleMessage(param0: globalAndroid.os.Message): boolean;
							public onFrameRendered(param0: com.google.android.exoplayer2.mediacodec.MediaCodecAdapter, param1: number, param2: number): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module video {
					export class VideoDecoderGLSurfaceView implements com.google.android.exoplayer2.video.VideoDecoderOutputBufferRenderer {
						public static class: java.lang.Class<com.google.android.exoplayer2.video.VideoDecoderGLSurfaceView>;
						public setOutputBuffer(param0: com.google.android.exoplayer2.decoder.VideoDecoderOutputBuffer): void;
						public constructor(param0: globalAndroid.content.Context);
						/** @deprecated */
						public getVideoDecoderOutputBufferRenderer(): com.google.android.exoplayer2.video.VideoDecoderOutputBufferRenderer;
						public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet);
					}
					export module VideoDecoderGLSurfaceView {
						export class Renderer {
							public static class: java.lang.Class<com.google.android.exoplayer2.video.VideoDecoderGLSurfaceView.Renderer>;
							public onDrawFrame(param0: javax.microedition.khronos.opengles.GL10): void;
							public constructor(param0: globalAndroid.opengl.GLSurfaceView);
							public onSurfaceChanged(param0: javax.microedition.khronos.opengles.GL10, param1: number, param2: number): void;
							public setOutputBuffer(param0: com.google.android.exoplayer2.decoder.VideoDecoderOutputBuffer): void;
							public onSurfaceCreated(param0: javax.microedition.khronos.opengles.GL10, param1: javax.microedition.khronos.egl.EGLConfig): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module video {
					export class VideoDecoderOutputBufferRenderer {
						public static class: java.lang.Class<com.google.android.exoplayer2.video.VideoDecoderOutputBufferRenderer>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.video.VideoDecoderOutputBufferRenderer interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							setOutputBuffer(param0: com.google.android.exoplayer2.decoder.VideoDecoderOutputBuffer): void;
						});
						public constructor();
						public setOutputBuffer(param0: com.google.android.exoplayer2.decoder.VideoDecoderOutputBuffer): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module video {
					export class VideoFrameMetadataListener {
						public static class: java.lang.Class<com.google.android.exoplayer2.video.VideoFrameMetadataListener>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.video.VideoFrameMetadataListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							onVideoFrameAboutToBeRendered(param0: number, param1: number, param2: com.google.android.exoplayer2.Format, param3: globalAndroid.media.MediaFormat): void;
						});
						public constructor();
						public onVideoFrameAboutToBeRendered(param0: number, param1: number, param2: com.google.android.exoplayer2.Format, param3: globalAndroid.media.MediaFormat): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module video {
					export class VideoFrameReleaseHelper {
						public static class: java.lang.Class<com.google.android.exoplayer2.video.VideoFrameReleaseHelper>;
						public setChangeFrameRateStrategy(param0: number): void;
						public onSurfaceChanged(param0: globalAndroid.view.Surface): void;
						public onNextFrame(param0: number): void;
						public constructor(param0: globalAndroid.content.Context);
						public onPlaybackSpeed(param0: number): void;
						public onStarted(): void;
						public onStopped(): void;
						public adjustReleaseTime(param0: number): number;
						public onFormatChanged(param0: number): void;
						public onPositionReset(): void;
					}
					export module VideoFrameReleaseHelper {
						export class Api30 {
							public static class: java.lang.Class<com.google.android.exoplayer2.video.VideoFrameReleaseHelper.Api30>;
							public static setSurfaceFrameRate(param0: globalAndroid.view.Surface, param1: number): void;
						}
						export class DisplayHelper {
							public static class: java.lang.Class<com.google.android.exoplayer2.video.VideoFrameReleaseHelper.DisplayHelper>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.video.VideoFrameReleaseHelper$DisplayHelper interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								register(param0: com.google.android.exoplayer2.video.VideoFrameReleaseHelper.DisplayHelper.Listener): void;
								unregister(): void;
							});
							public constructor();
							public unregister(): void;
							public register(param0: com.google.android.exoplayer2.video.VideoFrameReleaseHelper.DisplayHelper.Listener): void;
						}
						export module DisplayHelper {
							export class Listener {
								public static class: java.lang.Class<com.google.android.exoplayer2.video.VideoFrameReleaseHelper.DisplayHelper.Listener>;
								/**
								 * Constructs a new instance of the com.google.android.exoplayer2.video.VideoFrameReleaseHelper$DisplayHelper$Listener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
								 */
								public constructor(implementation: {
									onDefaultDisplayChanged(param0: globalAndroid.view.Display): void;
								});
								public constructor();
								public onDefaultDisplayChanged(param0: globalAndroid.view.Display): void;
							}
						}
						export class DisplayHelperV16 extends com.google.android.exoplayer2.video.VideoFrameReleaseHelper.DisplayHelper {
							public static class: java.lang.Class<com.google.android.exoplayer2.video.VideoFrameReleaseHelper.DisplayHelperV16>;
							public unregister(): void;
							public register(param0: com.google.android.exoplayer2.video.VideoFrameReleaseHelper.DisplayHelper.Listener): void;
							public static maybeBuildNewInstance(param0: globalAndroid.content.Context): com.google.android.exoplayer2.video.VideoFrameReleaseHelper.DisplayHelper;
						}
						export class DisplayHelperV17 extends com.google.android.exoplayer2.video.VideoFrameReleaseHelper.DisplayHelper {
							public static class: java.lang.Class<com.google.android.exoplayer2.video.VideoFrameReleaseHelper.DisplayHelperV17>;
							public unregister(): void;
							public register(param0: com.google.android.exoplayer2.video.VideoFrameReleaseHelper.DisplayHelper.Listener): void;
							public onDisplayRemoved(param0: number): void;
							public static maybeBuildNewInstance(param0: globalAndroid.content.Context): com.google.android.exoplayer2.video.VideoFrameReleaseHelper.DisplayHelper;
							public onDisplayChanged(param0: number): void;
							public onDisplayAdded(param0: number): void;
						}
						export class VSyncSampler {
							public static class: java.lang.Class<com.google.android.exoplayer2.video.VideoFrameReleaseHelper.VSyncSampler>;
							public sampledVsyncTimeNs: number;
							public addObserver(): void;
							public removeObserver(): void;
							public doFrame(param0: number): void;
							public handleMessage(param0: globalAndroid.os.Message): boolean;
							public static getInstance(): com.google.android.exoplayer2.video.VideoFrameReleaseHelper.VSyncSampler;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module video {
					export class VideoRendererEventListener {
						public static class: java.lang.Class<com.google.android.exoplayer2.video.VideoRendererEventListener>;
						/**
						 * Constructs a new instance of the com.google.android.exoplayer2.video.VideoRendererEventListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							onVideoEnabled(param0: com.google.android.exoplayer2.decoder.DecoderCounters): void;
							onVideoDecoderInitialized(param0: string, param1: number, param2: number): void;
							onVideoInputFormatChanged(param0: com.google.android.exoplayer2.Format): void;
							onVideoInputFormatChanged(param0: com.google.android.exoplayer2.Format, param1: com.google.android.exoplayer2.decoder.DecoderReuseEvaluation): void;
							onDroppedFrames(param0: number, param1: number): void;
							onVideoFrameProcessingOffset(param0: number, param1: number): void;
							onVideoSizeChanged(param0: com.google.android.exoplayer2.video.VideoSize): void;
							onRenderedFirstFrame(param0: any, param1: number): void;
							onVideoDecoderReleased(param0: string): void;
							onVideoDisabled(param0: com.google.android.exoplayer2.decoder.DecoderCounters): void;
							onVideoCodecError(param0: java.lang.Exception): void;
						});
						public constructor();
						public onRenderedFirstFrame(param0: any, param1: number): void;
						public onVideoCodecError(param0: java.lang.Exception): void;
						public onVideoEnabled(param0: com.google.android.exoplayer2.decoder.DecoderCounters): void;
						public onVideoInputFormatChanged(param0: com.google.android.exoplayer2.Format, param1: com.google.android.exoplayer2.decoder.DecoderReuseEvaluation): void;
						public onVideoDecoderReleased(param0: string): void;
						public onDroppedFrames(param0: number, param1: number): void;
						/** @deprecated */
						public onVideoInputFormatChanged(param0: com.google.android.exoplayer2.Format): void;
						public onVideoSizeChanged(param0: com.google.android.exoplayer2.video.VideoSize): void;
						public onVideoFrameProcessingOffset(param0: number, param1: number): void;
						public onVideoDecoderInitialized(param0: string, param1: number, param2: number): void;
						public onVideoDisabled(param0: com.google.android.exoplayer2.decoder.DecoderCounters): void;
					}
					export module VideoRendererEventListener {
						export class EventDispatcher {
							public static class: java.lang.Class<com.google.android.exoplayer2.video.VideoRendererEventListener.EventDispatcher>;
							public reportVideoFrameProcessingOffset(param0: number, param1: number): void;
							public videoCodecError(param0: java.lang.Exception): void;
							public enabled(param0: com.google.android.exoplayer2.decoder.DecoderCounters): void;
							public disabled(param0: com.google.android.exoplayer2.decoder.DecoderCounters): void;
							public droppedFrames(param0: number, param1: number): void;
							public videoSizeChanged(param0: com.google.android.exoplayer2.video.VideoSize): void;
							public renderedFirstFrame(param0: any): void;
							public constructor(param0: globalAndroid.os.Handler, param1: com.google.android.exoplayer2.video.VideoRendererEventListener);
							public decoderInitialized(param0: string, param1: number, param2: number): void;
							public decoderReleased(param0: string): void;
							public inputFormatChanged(param0: com.google.android.exoplayer2.Format, param1: com.google.android.exoplayer2.decoder.DecoderReuseEvaluation): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module video {
					export module spherical {
						export class CameraMotionListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.video.spherical.CameraMotionListener>;
							/**
							 * Constructs a new instance of the com.google.android.exoplayer2.video.spherical.CameraMotionListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
							 */
							public constructor(implementation: {
								onCameraMotion(param0: number, param1: androidNative.Array<number>): void;
								onCameraMotionReset(): void;
							});
							public constructor();
							public onCameraMotion(param0: number, param1: androidNative.Array<number>): void;
							public onCameraMotionReset(): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module video {
					export module spherical {
						export class CameraMotionRenderer extends com.google.android.exoplayer2.BaseRenderer {
							public static class: java.lang.Class<com.google.android.exoplayer2.video.spherical.CameraMotionRenderer>;
							public isEnded(): boolean;
							public static getTunnelingSupport(param0: number): number;
							public resetPosition(param0: number): void;
							public supportsFormat(param0: com.google.android.exoplayer2.Format): number;
							public static getAdaptiveSupport(param0: number): number;
							public getMediaClock(): com.google.android.exoplayer2.util.MediaClock;
							public reset(): void;
							public disable(): void;
							public onStreamChanged(param0: androidNative.Array<com.google.android.exoplayer2.Format>, param1: number, param2: number): void;
							public isCurrentStreamFinal(): boolean;
							public onPositionReset(param0: number, param1: boolean): void;
							public stop(): void;
							public start(): void;
							public enable(param0: com.google.android.exoplayer2.RendererConfiguration, param1: androidNative.Array<com.google.android.exoplayer2.Format>, param2: com.google.android.exoplayer2.source.SampleStream, param3: number, param4: boolean, param5: boolean, param6: number, param7: number): void;
							public static getDecoderSupport(param0: number): number;
							public handleMessage(param0: number, param1: any): void;
							public getStream(): com.google.android.exoplayer2.source.SampleStream;
							public getTrackType(): number;
							public constructor();
							public hasReadStreamToEnd(): boolean;
							public replaceStream(param0: androidNative.Array<com.google.android.exoplayer2.Format>, param1: com.google.android.exoplayer2.source.SampleStream, param2: number, param3: number): void;
							public setCurrentStreamFinal(): void;
							public static create(param0: number, param1: number, param2: number, param3: number, param4: number): number;
							public getState(): number;
							public isReady(): boolean;
							public init(param0: number, param1: com.google.android.exoplayer2.analytics.PlayerId): void;
							public static create(param0: number): number;
							public constructor(param0: number);
							public supportsMixedMimeTypeAdaptation(): number;
							public maybeThrowStreamError(): void;
							public getCapabilities(): com.google.android.exoplayer2.RendererCapabilities;
							public getReadingPositionUs(): number;
							public static getFormatSupport(param0: number): number;
							public getName(): string;
							public static create(param0: number, param1: number, param2: number): number;
							public setPlaybackSpeed(param0: number, param1: number): void;
							public onDisabled(): void;
							public static getHardwareAccelerationSupport(param0: number): number;
							public render(param0: number, param1: number): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module video {
					export module spherical {
						export class FrameRotationQueue {
							public static class: java.lang.Class<com.google.android.exoplayer2.video.spherical.FrameRotationQueue>;
							public constructor();
							public pollRotationMatrix(param0: androidNative.Array<number>, param1: number): boolean;
							public setRotation(param0: number, param1: androidNative.Array<number>): void;
							public static computeRecenterMatrix(param0: androidNative.Array<number>, param1: androidNative.Array<number>): void;
							public reset(): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module video {
					export module spherical {
						export class OrientationListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.video.spherical.OrientationListener>;
							public constructor(param0: globalAndroid.view.Display, param1: androidNative.Array<com.google.android.exoplayer2.video.spherical.OrientationListener.Listener>);
							public onAccuracyChanged(param0: globalAndroid.hardware.Sensor, param1: number): void;
							public onSensorChanged(param0: globalAndroid.hardware.SensorEvent): void;
						}
						export module OrientationListener {
							export class Listener {
								public static class: java.lang.Class<com.google.android.exoplayer2.video.spherical.OrientationListener.Listener>;
								/**
								 * Constructs a new instance of the com.google.android.exoplayer2.video.spherical.OrientationListener$Listener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
								 */
								public constructor(implementation: {
									onOrientationChange(param0: androidNative.Array<number>, param1: number): void;
								});
								public constructor();
								public onOrientationChange(param0: androidNative.Array<number>, param1: number): void;
							}
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module video {
					export module spherical {
						export class Projection {
							public static class: java.lang.Class<com.google.android.exoplayer2.video.spherical.Projection>;
							public static DRAW_MODE_TRIANGLES: number;
							public static DRAW_MODE_TRIANGLES_STRIP: number;
							public static DRAW_MODE_TRIANGLES_FAN: number;
							public static TEXTURE_COORDS_PER_VERTEX: number;
							public static POSITION_COORDS_PER_VERTEX: number;
							public leftMesh: com.google.android.exoplayer2.video.spherical.Projection.Mesh;
							public rightMesh: com.google.android.exoplayer2.video.spherical.Projection.Mesh;
							public stereoMode: number;
							public singleMesh: boolean;
							public constructor(param0: com.google.android.exoplayer2.video.spherical.Projection.Mesh, param1: com.google.android.exoplayer2.video.spherical.Projection.Mesh, param2: number);
							public constructor(param0: com.google.android.exoplayer2.video.spherical.Projection.Mesh, param1: number);
							public static createEquirectangular(param0: number): com.google.android.exoplayer2.video.spherical.Projection;
							public static createEquirectangular(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number): com.google.android.exoplayer2.video.spherical.Projection;
						}
						export module Projection {
							export class DrawMode {
								public static class: java.lang.Class<com.google.android.exoplayer2.video.spherical.Projection.DrawMode>;
								/**
								 * Constructs a new instance of the com.google.android.exoplayer2.video.spherical.Projection$DrawMode interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
								 */
								public constructor(implementation: {
								});
								public constructor();
							}
							export class Mesh {
								public static class: java.lang.Class<com.google.android.exoplayer2.video.spherical.Projection.Mesh>;
								public getSubMeshCount(): number;
								public getSubMesh(param0: number): com.google.android.exoplayer2.video.spherical.Projection.SubMesh;
								public constructor(param0: androidNative.Array<com.google.android.exoplayer2.video.spherical.Projection.SubMesh>);
							}
							export class SubMesh {
								public static class: java.lang.Class<com.google.android.exoplayer2.video.spherical.Projection.SubMesh>;
								public static VIDEO_TEXTURE_ID: number;
								public textureId: number;
								public mode: number;
								public vertices: androidNative.Array<number>;
								public textureCoords: androidNative.Array<number>;
								public getVertexCount(): number;
								public constructor(param0: number, param1: androidNative.Array<number>, param2: androidNative.Array<number>, param3: number);
							}
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module video {
					export module spherical {
						export class ProjectionDecoder {
							public static class: java.lang.Class<com.google.android.exoplayer2.video.spherical.ProjectionDecoder>;
							public static decode(param0: androidNative.Array<number>, param1: number): com.google.android.exoplayer2.video.spherical.Projection;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module video {
					export module spherical {
						export class ProjectionRenderer {
							public static class: java.lang.Class<com.google.android.exoplayer2.video.spherical.ProjectionRenderer>;
							public static isSupported(param0: com.google.android.exoplayer2.video.spherical.Projection): boolean;
							public init(): void;
							public draw(param0: number, param1: androidNative.Array<number>, param2: boolean): void;
							public setProjection(param0: com.google.android.exoplayer2.video.spherical.Projection): void;
							public shutdown(): void;
						}
						export module ProjectionRenderer {
							export class MeshData {
								public static class: java.lang.Class<com.google.android.exoplayer2.video.spherical.ProjectionRenderer.MeshData>;
								public constructor(param0: com.google.android.exoplayer2.video.spherical.Projection.SubMesh);
							}
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module video {
					export module spherical {
						export class SceneRenderer implements com.google.android.exoplayer2.video.VideoFrameMetadataListener, com.google.android.exoplayer2.video.spherical.CameraMotionListener {
							public static class: java.lang.Class<com.google.android.exoplayer2.video.spherical.SceneRenderer>;
							public constructor();
							public init(): globalAndroid.graphics.SurfaceTexture;
							public drawFrame(param0: androidNative.Array<number>, param1: boolean): void;
							public onCameraMotion(param0: number, param1: androidNative.Array<number>): void;
							public shutdown(): void;
							public onVideoFrameAboutToBeRendered(param0: number, param1: number, param2: com.google.android.exoplayer2.Format, param3: globalAndroid.media.MediaFormat): void;
							public setDefaultStereoMode(param0: number): void;
							public onCameraMotionReset(): void;
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module video {
					export module spherical {
						export class SphericalGLSurfaceView {
							public static class: java.lang.Class<com.google.android.exoplayer2.video.spherical.SphericalGLSurfaceView>;
							public removeVideoSurfaceListener(param0: com.google.android.exoplayer2.video.spherical.SphericalGLSurfaceView.VideoSurfaceListener): void;
							public setUseSensorRotation(param0: boolean): void;
							public getCameraMotionListener(): com.google.android.exoplayer2.video.spherical.CameraMotionListener;
							public onDetachedFromWindow(): void;
							public getVideoSurface(): globalAndroid.view.Surface;
							public getVideoFrameMetadataListener(): com.google.android.exoplayer2.video.VideoFrameMetadataListener;
							public onPause(): void;
							public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet);
							public addVideoSurfaceListener(param0: com.google.android.exoplayer2.video.spherical.SphericalGLSurfaceView.VideoSurfaceListener): void;
							public setDefaultStereoMode(param0: number): void;
							public constructor(param0: globalAndroid.content.Context);
							public onResume(): void;
						}
						export module SphericalGLSurfaceView {
							export class Renderer implements com.google.android.exoplayer2.video.spherical.TouchTracker.Listener, com.google.android.exoplayer2.video.spherical.OrientationListener.Listener {
								public static class: java.lang.Class<com.google.android.exoplayer2.video.spherical.SphericalGLSurfaceView.Renderer>;
								public onSurfaceChanged(param0: javax.microedition.khronos.opengles.GL10, param1: number, param2: number): void;
								public onScrollChange(param0: globalAndroid.graphics.PointF): void;
								public onSingleTapUp(param0: globalAndroid.view.MotionEvent): boolean;
								public onDrawFrame(param0: javax.microedition.khronos.opengles.GL10): void;
								public onOrientationChange(param0: androidNative.Array<number>, param1: number): void;
								public constructor(param0: com.google.android.exoplayer2.video.spherical.SphericalGLSurfaceView, param1: com.google.android.exoplayer2.video.spherical.SceneRenderer);
								public onSurfaceCreated(param0: javax.microedition.khronos.opengles.GL10, param1: javax.microedition.khronos.egl.EGLConfig): void;
							}
							export class VideoSurfaceListener {
								public static class: java.lang.Class<com.google.android.exoplayer2.video.spherical.SphericalGLSurfaceView.VideoSurfaceListener>;
								/**
								 * Constructs a new instance of the com.google.android.exoplayer2.video.spherical.SphericalGLSurfaceView$VideoSurfaceListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
								 */
								public constructor(implementation: {
									onVideoSurfaceCreated(param0: globalAndroid.view.Surface): void;
									onVideoSurfaceDestroyed(param0: globalAndroid.view.Surface): void;
								});
								public constructor();
								public onVideoSurfaceDestroyed(param0: globalAndroid.view.Surface): void;
								public onVideoSurfaceCreated(param0: globalAndroid.view.Surface): void;
							}
						}
					}
				}
			}
		}
	}
}

declare module com {
	export module google {
		export module android {
			export module exoplayer2 {
				export module video {
					export module spherical {
						export class TouchTracker implements com.google.android.exoplayer2.video.spherical.OrientationListener.Listener {
							public static class: java.lang.Class<com.google.android.exoplayer2.video.spherical.TouchTracker>;
							public onDown(param0: globalAndroid.view.MotionEvent): boolean;
							public onOrientationChange(param0: androidNative.Array<number>, param1: number): void;
							public onSingleTapUp(param0: globalAndroid.view.MotionEvent): boolean;
							public onTouch(param0: globalAndroid.view.View, param1: globalAndroid.view.MotionEvent): boolean;
							public onScroll(param0: globalAndroid.view.MotionEvent, param1: globalAndroid.view.MotionEvent, param2: number, param3: number): boolean;
							public constructor(param0: globalAndroid.content.Context, param1: com.google.android.exoplayer2.video.spherical.TouchTracker.Listener, param2: number);
						}
						export module TouchTracker {
							export class Listener {
								public static class: java.lang.Class<com.google.android.exoplayer2.video.spherical.TouchTracker.Listener>;
								/**
								 * Constructs a new instance of the com.google.android.exoplayer2.video.spherical.TouchTracker$Listener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
								 */
								public constructor(implementation: {
									onScrollChange(param0: globalAndroid.graphics.PointF): void;
									onSingleTapUp(param0: globalAndroid.view.MotionEvent): boolean;
								});
								public constructor();
								public onScrollChange(param0: globalAndroid.graphics.PointF): void;
								public onSingleTapUp(param0: globalAndroid.view.MotionEvent): boolean;
							}
						}
					}
				}
			}
		}
	}
}


//Generics information:
//com.google.android.exoplayer2.audio.DecoderAudioRenderer:1
//com.google.android.exoplayer2.audio.DefaultAudioSink.PendingExceptionHolder:1
//com.google.android.exoplayer2.mediacodec.MediaCodecUtil.ScoreProvider:1
//com.google.android.exoplayer2.offline.FilterableManifest:1
//com.google.android.exoplayer2.offline.FilteringManifestParser:1
//com.google.android.exoplayer2.offline.SegmentDownloader:1
//com.google.android.exoplayer2.source.CompositeMediaSource:1
//com.google.android.exoplayer2.source.CompositeMediaSource.MediaSourceAndListener:1
//com.google.android.exoplayer2.source.ConcatenatingMediaSource.MessageData:1
//com.google.android.exoplayer2.source.SequenceableLoader.Callback:1
//com.google.android.exoplayer2.source.SpannedData:1
//com.google.android.exoplayer2.source.chunk.ChunkSampleStream:1
//com.google.android.exoplayer2.source.chunk.ChunkSampleStream.ReleaseCallback:1
//com.google.android.exoplayer2.trackselection.DefaultTrackSelector.TrackInfo:1
//com.google.android.exoplayer2.trackselection.DefaultTrackSelector.TrackInfo.Factory:1
//com.google.android.exoplayer2.upstream.Loader.Callback:1
//com.google.android.exoplayer2.upstream.Loader.LoadTask:1
//com.google.android.exoplayer2.upstream.ParsingLoadable:1
//com.google.android.exoplayer2.upstream.ParsingLoadable.Parser:1

