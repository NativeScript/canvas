import { VideoBase } from './common';
import { Screen, Application, Utils, knownFolders, path } from '@nativescript/core';
import { Source } from '..';
import { durationProperty, currentTimeProperty } from '../common';

declare var com, org;
export class Video extends VideoBase {
	_container: android.widget.LinearLayout;
	_sourceView: Source[] = [];
	_src: string;
	_autoplay: boolean;
	_loop: boolean;
	_textureView: android.view.TextureView;
	_isCustom: boolean = false;
	_playing: boolean = false;
	_canvas: any;
	_readyState: number = 0;
	_videoWidth = 0;
	_videoHeight = 0;
	private static _didInit = false;
	static BUFFER_MS = 500;
	static IS_DEBUG = false;
	_instance;
	constructor() {
		super();
		try {
			java.lang.System.loadLibrary('canvasnative');
		} catch (ex) {}

		const activity: androidx.appcompat.app.AppCompatActivity = Application.android.foregroundActivity || Application.android.startActivity;

		this._instance = new org.nativescript.canvas.media.VideoHelper(activity, knownFolders.documents().path);

		const ref = new WeakRef(this);

		this._instance.setCallback(
			new org.nativescript.canvas.media.VideoHelper.Callback({
				onDurationChange(duration) {
					const owner = ref.get();
					if (owner) {
						durationProperty.nativeValueChange(owner, duration);
						owner._notifyListener(Video.durationchangeEvent);
					}
				},

				onPlaying() {
					const owner = ref.get();
					if (owner) {
						owner._playing = true;
						if (owner._playResolve) {
							owner._playResolve();
							owner._playResolve = null;
							owner._playReject = null;
							owner._playPromise = null;
						}
						owner._notifyListener(Video.playingEvent);
					}
				},

				onVideoSizeChanged(width, height) {
					const owner = ref.get();
					if (owner) {
						owner._videoWidth = width;
						owner._videoHeight = height;
					}
				},

				onCurrentTimeChanged(time) {
					const owner = ref.get();
					if (owner) {
						currentTimeProperty.nativeValueChange(owner, time);
						owner._notifyListener(Video.timeupdateEvent);
					}
				},

				onVideoFrame() {
					const owner = ref.get();
					if (owner) {
						owner._notifyVideoFrameCallbacks();
					}
				},

				onLoadedData() {
					const owner = ref.get();
					if (owner) {
						owner._readyState = Video.HAVE_CURRENT_DATA;
						owner._notifyListener(Video.loadeddataEvent);
					}
				},

				onError(message: string) {
					const owner = ref.get();
					if (owner) {
						owner._playing = false;
						if (owner._playReject) {
							owner._playReject(new Error(message ?? 'Playback error'));
						}
						owner._playResolve = null;
						owner._playReject = null;
						owner._playPromise = null;
					}
				},
			}),
		);
		this.setNativeView(this._instance.getContainer());
	}

	get readyState() {
		return this._readyState;
	}

	static createCustomView() {
		const video = new Video();
		video._isCustom = true;
		video.width = 300;
		video.height = 150;
		return video;
	}
	getCurrentFrame(context?: WebGLRenderingContext) {
		const ctx = arguments[1] as any;
		//@ts-ignore
		const flipY = context?.__flipY ?? false;
		if (!ctx._canvas._canvas) {
			return;
		}
		const ptr = ctx._canvas._canvas.getNativeContext();

		this._instance.getCurrentFrame(!!this.isLoaded, ptr, flipY, arguments[4], arguments[5]);
	}

	getFrameForTexImage3D(nativeCtx: any, ctx: any, target: number, level: number, internalformat: number, width: number, height: number, depth: number, border: number, format: number, type: number) {
		if (!ctx._canvas._canvas) {
			return;
		}
		const ptr = ctx._canvas._canvas.getNativeContext();
		//@ts-ignore
		const flipY = nativeCtx?.__flipY ?? false;

		const drawn = this._instance.getFrameForTexImage3D(!!this.isLoaded, ptr, flipY, target, level, internalformat, width, height, depth, border);
		if (drawn) {
			return;
		}

		const bitmap = this._instance.getCurrentBitmap();
		if (bitmap) {
			const bw = bitmap.getWidth();
			const bh = bitmap.getHeight();
			const upload = bw !== width || bh !== height ? android.graphics.Bitmap.createScaledBitmap(bitmap, width, height, true) : bitmap;
			ctx.native.texImage3D(target, level, internalformat, width, height, depth, border, format, type, upload);
			if (upload !== bitmap) {
				upload.recycle();
			}
		}
	}

	getFrameForTexSubImage3D(nativeCtx: any, ctx: any, target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type: number) {
		if (!ctx._canvas._canvas) {
			return;
		}
		const ptr = ctx._canvas._canvas.getNativeContext();
		//@ts-ignore
		const flipY = nativeCtx?.__flipY ?? false;

		const drawn = this._instance.getFrameForTexSubImage3D(!!this.isLoaded, ptr, flipY, target, level, xoffset, yoffset, zoffset, width, height);
		if (drawn) {
			return;
		}

		const bitmap = this._instance.getCurrentBitmap();
		if (bitmap) {
			const bw = bitmap.getWidth();
			const bh = bitmap.getHeight();
			const upload = bw !== width || bh !== height ? android.graphics.Bitmap.createScaledBitmap(bitmap, width, height, true) : bitmap;
			ctx.native.texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, upload);
			if (upload !== bitmap) {
				upload.recycle();
			}
		}
	}

	drawImageFrame(context2d: any, args: any[]) {
		if (!this._instance) {
			return;
		}
		const ptr = context2d.context.__getPointer();
		const nativePtr = long(ptr);
		const backendType: number = context2d.__engine ?? 0;
		let dirty = false;
		if (args.length === 3) {
			dirty = this._instance.drawVideoFrame2D(backendType, nativePtr, args[1], args[2]);
		} else if (args.length === 5) {
			dirty = this._instance.drawVideoFrame2D(backendType, nativePtr, args[1], args[2], args[3], args[4]);
		} else if (args.length === 9) {
			dirty = this._instance.drawVideoFrame2D(backendType, nativePtr, args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8]);
		}
		if (dirty) {
			context2d.context.__makeDirty();
		}
	}

	getVideoFrameData(): any {
		if (!this._instance) {
			return null;
		}
		return this._instance.getCurrentBitmap();
	}

	_playResolve;
	_playReject;
	_playPromise: Promise<void> | null = null;

	play() {
		if (this._playing) {
			return Promise.resolve();
		}

		if (this._playPromise) {
			return this._playPromise;
		}

		this._playPromise = new Promise<void>((resolve, reject) => {
			this._playResolve = resolve;
			this._playReject = reject;
			this._instance.play();
		});
		return this._playPromise;
	}

	pause() {
		this._instance.pause();
	}

	get muted() {
		return this._instance.getMuted();
	}

	set muted(value: boolean) {
		this._instance.setMuted(value);
	}

	get duration() {
		return this._instance.getDuration();
	}

	get currentTime() {
		return this._instance.getCurrentTime();
	}

	set currentTime(value: number) {
		this._instance.setCurrentTime(value);
	}

	get src() {
		return this._instance.getSrc();
	}

	set src(value: string) {
		if (typeof value === 'string' && value.startsWith('~/')) {
			value = path.join(knownFolders.currentApp().path, value.replace('~', ''));
		}
		this._instance.setSrc(value);
	}

	//@ts-ignore
	get autoplay() {
		return this._instance.getAutoplay();
	}

	set autoplay(value: boolean) {
		this._instance.setAutoplay(value);
	}

	get controls() {
		return this._instance.getControls();
	}

	set controls(enabled: boolean) {
		this._instance.setControls(enabled);
	}

	// @ts-ignore
	get loop() {
		return this._instance.getLoop();
	}

	set loop(value: boolean) {
		this._instance.setLoop(value);
	}

	createNativeView(): Object {
		this._instance.setPlayer();
		return this._instance.getPlayerView();
	}

	initNativeView() {
		super.initNativeView();
	}

	_addChildFromBuilder(name: string, value: any) {
		if (value instanceof Source) {
			this._sourceView.push(value);
		}
	}

	onLoaded() {
		super.onLoaded();
		if (this._sourceView.length > 0) {
			this.src = this._sourceView[0].src;
		}
	}

	// @ts-ignore
	get width() {
		if (this.getMeasuredWidth() > 0) {
			return this.getMeasuredWidth();
		}
		return this._instance.getWidth();
	}

	set width(value) {
		this.style.width = value;
		if (this._isCustom) {
			this._layoutNative();
		}
	}

	// @ts-ignore
	get height() {
		if (this.getMeasuredHeight() > 0) {
			return this.getMeasuredHeight();
		}
		return this._instance.getHeight();
	}

	set height(value) {
		this.style.height = value;
		if (this._isCustom) {
			this._layoutNative();
		}
	}

	_layoutNative() {
		if (!this.parent) {
			//@ts-ignore
			if ((typeof this.width === 'string' && this.width.indexOf('%')) || (typeof this.height === 'string' && this.height.indexOf('%'))) {
				return;
			}
			if (!this._isCustom) {
				return;
			}

			const size = this._realSize;
			size.width = size.width * Screen.mainScreen.scale;
			size.height = size.height * Screen.mainScreen.scale;
			this._instance.layoutNative(size.width, size.height);
		}
	}
}
