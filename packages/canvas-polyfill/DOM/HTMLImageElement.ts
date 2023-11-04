import { Element } from './Element';
import { knownFolders, path, File, Utils } from '@nativescript/core';
import { ImageAsset } from '@nativescript/canvas';
declare const NSSCanvasHelpers;
declare var NSUUID, java, NSData, android;
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

export class HTMLImageElement extends Element {
	private localUri: any;
	private _onload: any;
	private _complete: any;
	private _base64: string;
	_asset;
	_imageSource: any;
	__id: any;

	decoding = 'auto';

	get src() {
		return this.localUri;
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

	get complete() {
		return this._complete;
	}

	set complete(value) {
		this._complete = value;
		if (value) {
			this.emitter.emit('load', this);
			this.onload();
		}
	}

	constructor(props?) {
		super('img');
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
		if (this.src) {
			if (typeof this.src === 'string' && this.src.startsWith && this.src.startsWith('data:')) {
				// is base64 - convert and try again;
				this._base64 = this.src;
				const base64result = this.src.split(',')[1];
				if (!base64result) {
					return;
				}
				(async () => {
					try {
						const MIME = getMIMEforBase64String(base64result);
						const dir = knownFolders.documents().path;
						if (global.isIOS) {
							NSSCanvasHelpers.handleBase64Image(MIME, dir, base64result, (error, localUri) => {
								if (error) {
									if ((global as any).__debug_browser_polyfill_image) {
										console.log(`nativescript-browser-polyfill: Error:`, error.message);
									}
									this.emitter.emit('error', { target: this, error });
								} else {
									this.localUri = localUri;
									this._load();
								}
							});
						}
						if (global.isAndroid) {
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
											owner.emitter.emit('error', { target: ref.get(), message });
										}
									},
								})
							);
						}
					} catch (error) {
						if ((global as any).__debug_browser_polyfill_image) {
							console.log(`nativescript-browser-polyfill: Error:`, error.message);
						}
						this.emitter.emit('error', { target: this, error });
					}
				})();
				return;
			}

			if (typeof this.src === 'string') {
				this.emitter.emit('loading', { target: this });
				let async = this.decoding !== 'sync';
				if (this.src.startsWith('http')) {
					if (!async) {
						const loaded = this._asset.fromUrlSync(this.src);
						if (loaded) {
							this.width = this._asset.width;
							this.height = this._asset.height;
							this.complete = true;
						} else {
							this.emitter.emit('error', { target: this });
						}
					} else {
						this._asset
							.fromUrl(this.src)
							.then(() => {
								this.width = this._asset.width;
								this.height = this._asset.height;
								this.complete = true;
							})
							.catch((e) => {
								this.emitter.emit('error', { target: this });
							});
					}
				} else {
					if (!this.width || !this.height) {
						this.complete = false;
						if (!async) {
							const loaded = this._asset.fromFileSync(this.src);
							if (loaded) {
								this.width = this._asset.width;
								this.height = this._asset.height;
								this.complete = true;
							} else {
								this.emitter.emit('error', { target: this });
							}
						} else {
							this._asset
								.fromFile(this.src)
								.then((done) => {
									this.width = this._asset.width;
									this.height = this._asset.height;
									this.complete = true;
								})
								.catch((e) => {
									this.emitter.emit('error', { target: this });
								});
						}
					}
				}
			}
		}
	}
}
