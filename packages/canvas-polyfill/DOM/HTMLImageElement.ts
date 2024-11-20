import { knownFolders, path, File, Utils, Image, Screen } from '@nativescript/core';
// @ts-ignore
import { ImageAsset } from '@nativescript/canvas';
import { HTMLElement } from './HTMLElement';
import { Svg } from '@nativescript/canvas-svg';

declare const NSSCanvasHelpers;
// declare var NSUUID, java, NSData, android;
const b64Extensions = {
	'/': 'jpg',
	i: 'png',
	R: 'gif',
	U: 'webp',
};

function b64WithoutPrefix(b64) {
	return b64.split(',')[1];
}

function getMIMEforBase64String(b64) {
	let input = b64;
	if (b64.includes(',')) {
		input = b64WithoutPrefix(b64);
	}
	const first = input.charAt(0);
	const mime = b64Extensions[first];
	// todo support webp on ios
	if (__IOS__ && mime === 'webp') {
		throw new Error('Unknown Base64 MIME type: ' + b64);
	}
	if (!mime) {
		throw new Error('Unknown Base64 MIME type: ' + b64);
	}
	return mime;
}

function getUUID() {
	if (global.isIOS) {
		return NSUUID.UUID().UUIDString;
	}
	return java.util.UUID.randomUUID().toString();
}

export class HTMLImageElement extends HTMLElement {
	private localUri: any;
	private _onload: any;
	private _onerror: any;
	private _complete: any;
	private _base64: string;
	width: number = 0;
	height: number = 0;
	_asset: ImageAsset;
	_svg;
	__id: any;

	decoding = 'auto';

	private _loading = false;
	private _decodingQueue: { resolve: () => void; reject: (reason?: any) => void }[] = [];

	get src() {
		return this.localUri;
	}

	get naturalWidth() {
		return this.width;
	}

	get naturalHeight() {
		return this.height;
	}

	set src(value) {
		this.localUri = value;
		this._load();
	}

	get onload() {
		return this._onload;
	}

	set onload(value) {
		this._onload = value;
	}

	get onerror() {
		return this._onerror;
	}

	set onerror(value) {
		this._onerror = value;
	}

	get complete() {
		return this._complete;
	}

	set complete(value) {
		this._complete = value;
		if (value) {
			this.dispatchEvent({ type: 'load', target: this });
			this.onload();
		}
	}

	private _dispatchDecode(success = false) {
		for (const promise of this._decodingQueue) {
			if (success) {
				promise.resolve();
			} else {
				promise.reject();
			}
		}
		this._decodingQueue = [];
	}

	decode() {
		if (!this.src) {
			return Promise.reject(new Error('Invalid image request.'));
		}
		if (!this._loading && !this.complete) {
			return new Promise<void>((resolve, reject) => {
				this._decodingQueue.push({ resolve, reject });
				this._load();
			});
		} else if (this._loading) {
			return new Promise<void>((resolve, reject) => {
				this._decodingQueue.push({ resolve, reject });
			});
		}
		return Promise.resolve();
	}

	// we're {N} :D
	static _fromSvg(svg) {
		const img = new HTMLImageElement();
		if (svg) {
			const nativeSvg = svg?._svg;
			if (nativeSvg) {
				img._svg = svg;
				img.width = svg.width;
				img.height = svg.height;
				img.complete = true;
				return img;
			}
		}

		return null;
	}

	constructor(props?) {
		super('img');
		this.nativeElement = new Image();

		if (!this.nativeElement.__domElement) {
			this.nativeElement.__domElement = new DOMParser().parseFromString('<img></img>', 'text/html').documentElement as never;
		}

		this._asset = new ImageAsset();
		this.__id = getUUID();
		this._onload = () => {};
		if (props !== null && typeof props === 'object') {
			this.src = props.localUri;
			this.width = props.width;
			this.height = props.height;
			this._load();
		}
	}

	_load() {
		this._loading = true;
		if (this.src && typeof this.src === 'string') {
			if (this.src.startsWith('blob:nativescript/')) {
				const data = (<any>URL).InternalAccessor.getData(this.src);
				const buffer = (<any>Blob).InternalAccessor.getBuffer(data.blob);

				let isSvg = data?.type?.indexOf('svg') > -1;
				let src;
				let d;
				if (!isSvg) {
					try {
						d = new TextDecoder();
						src = d.decode(buffer);
						if (typeof src === 'string') {
							isSvg = src.indexOf('<svg') > -1;
						}
					} catch (error) {}
				}

				if (isSvg) {
					d = d ?? new TextDecoder();
					src = src ?? d.decode(buffer);
					Svg.fromSrc(src)
						.then((svg) => {
							const data = svg.data;
							return this._asset.loadFromBytes(svg.width, svg.height, data as any);
						})
						.then((done: boolean) => {
							this.width = this._asset.width;
							this.height = this._asset.height;
							this.complete = done;
							this._loading = false;
							this._dispatchDecode(done);
						})
						.catch((e) => {
							this.dispatchEvent({ type: 'error', target: this, e });
							this._onerror?.();
							this._loading = false;
							this._dispatchDecode();
						});
					return;
				} else {
					this._asset
						.loadFromEncodedBytes(buffer)
						.then((done: boolean) => {
							this.width = this._asset.width;
							this.height = this._asset.height;
							this.complete = done;
							this._loading = false;
							this._dispatchDecode(done);
						})
						.catch((e) => {
							this.dispatchEvent({ type: 'error', target: this, e });
							this._onerror?.();
							this._loading = false;
							this._dispatchDecode();
						});
					return;
				}
			}

			if (this.src.startsWith && this.src.startsWith('data:')) {
				// is base64 - convert and try again;
				this._base64 = this.src;
				const base64result = this.src.split(',')[1];

				if (!base64result) {
					this.dispatchEvent({ type: 'error', target: this, error: new Error('Invalid image request.') });
					this._onerror?.();
					this._loading = false;
					this._dispatchDecode();
					return;
				}

				try {
					const MIME = getMIMEforBase64String(base64result);
					const dir = knownFolders.temp().path;
					if (__IOS__) {
						NSSCanvasHelpers.handleBase64Image(MIME, dir, base64result, (error, localUri) => {
							if (error) {
								if ((global as any).__debug_browser_polyfill_image) {
									console.log(`nativescript-browser-polyfill: Error:`, error.message);
								}
								this.dispatchEvent({ type: 'error', target: this, error });
								this._onerror?.();
								this._loading = false;
								this._dispatchDecode();
							} else {
								this.localUri = localUri;
								this._load();
							}
						});
					}
					if (__ANDROID__) {
						const ref = new WeakRef(this);
						(com as any).github.triniwiz.async.Async2.Base64.base64ToFile(
							base64result,
							dir + '/',
							new (com as any).github.triniwiz.async.Async2.Base64.Callback({
								success(response) {
									const owner = ref.get();
									if (owner) {
										owner.localUri = response.toString();
										owner._load();
									}
								},
								error(error, message) {
									if ((global as any).__debug_browser_polyfill_image) {
										console.log(`nativescript-browser-polyfill: Error:`, message);
									}
									const owner = ref.get();
									if (owner) {
										owner.dispatchEvent({ type: 'error', target: ref.get(), message });
										owner._onerror?.();
										owner._loading = false;
										owner._dispatchDecode?.();
									}
								},
							})
						);
					}
				} catch (error) {
					//	this.dispatchEvent({type: 'loading', target: this });
					if ((global as any).__debug_browser_polyfill_image) {
						console.log(`nativescript-browser-polyfill: Error:`, error.message);
					}
					this.dispatchEvent({ type: 'error', target: this, error });
					this._onerror?.();
					this._loading = false;
					this._dispatchDecode?.();
				}

				return;
			}

			this.dispatchEvent({ type: 'loading', target: this });
			let async = this.decoding !== 'sync';
			if (this.src.startsWith('http')) {
				if (!async) {
					const loaded = this._asset.fromUrlSync(this.src);
					let success = false;
					if (loaded) {
						this.width = this._asset.width;
						this.height = this._asset.height;
						this.complete = true;
						success = true;
					} else {
						this.dispatchEvent({ type: 'error', target: this });
						this._onerror?.();
					}
					this._loading = false;
					this._dispatchDecode(success);
				} else {
					this._asset
						.fromUrl(this.src)
						.then(() => {
							this.width = this._asset.width;
							this.height = this._asset.height;
							this.complete = true;
							this._loading = false;
							this._dispatchDecode(true);
						})
						.catch((e) => {
							this.dispatchEvent({ type: 'error', target: this, error: e });
							this._onerror?.();
							this._loading = false;
							this._dispatchDecode();
						});
				}
			} else {
				this.complete = false;
				let src = this.src;
				if (src.startsWith('~')) {
					src = knownFolders.currentApp().path + src.replace('~', '');
				}
				if (!async) {
					const loaded = this._asset.fromFileSync(src);
					if (loaded) {
						this.width = this._asset.width;
						this.height = this._asset.height;
						this.complete = true;
						this._loading = false;
						this._dispatchDecode(true);
					} else {
						this.dispatchEvent({ type: 'error', target: this });
						this._onerror?.();
						this._loading = false;
						this._dispatchDecode();
					}
				} else {
					this._asset
						.fromFile(src)
						.then((done) => {
							this.width = this._asset.width;
							this.height = this._asset.height;
							this.complete = true;
							this._loading = false;
							this._dispatchDecode(true);
						})
						.catch((e) => {
							this.dispatchEvent({ type: 'error', target: this, e });
							this._onerror?.();
							this._loading = false;
							this._dispatchDecode();
						});
				}
			}
		} else {
			this._loading = false;
			this._dispatchDecode();
		}
	}
}
