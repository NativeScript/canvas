declare const enum NSCPlayerReadyState {
	HaveNothing = 0,

	HaveMetadata = 1,

	HaveCurrentData = 2,

	HaveFutureData = 3,

	HaveEnoughData = 4,
}

declare const enum NSCPlayerState {
	Idle = 0,

	Playing = 1,

	Paused = 2,

	Stopped = 3,
}

declare class NSCVideoHelper extends NSObject {
	static alloc(): NSCVideoHelper; // inherited from NSObject

	static new(): NSCVideoHelper; // inherited from NSObject

	readonly assetOutput: AVPlayerItemVideoOutput;

	autoplay: boolean;

	readonly controller: AVPlayerViewController;

	controls: boolean;

	readonly currentItem: AVPlayerItem;

	currentTime: number;

	readonly duration: number;

	readonly isInForeground: boolean;

	listener: NSCVideoHelperListener;

	loop: boolean;

	muted: boolean;

	readonly player: AVPlayer;

	playsinline: boolean;

	readonly readyState: NSCPlayerReadyState;

	src: string;

	readonly state: NSCPlayerState;

	readonly videoSize: CGSize;

	pause(): void;

	play(): void;
}

interface NSCVideoHelperListener extends NSObjectProtocol {
	onStateChange(state: NSCPlayerState): void;

	onTimeUpdate(seconds: number): void;

	onVideoFrameCallback(): void;
}
declare var NSCVideoHelperListener: {
	prototype: NSCVideoHelperListener;
};
