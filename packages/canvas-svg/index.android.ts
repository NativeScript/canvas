import { SVGBase, backendProperty, gpuProperty, srcProperty, surfaceTypeProperty, syncProperty, type SvgBackend, type SvgSurfaceType, threadedProperty } from './common';
import { Application, Http, knownFolders, path, Screen, Utils } from '@nativescript/core';
import { SVGItem } from './Elements/SVGItem';
export * from './Elements';

declare const org;

function parseSVGDimensions(svgString) {
	const svgRegex = /<svg([^>]*)>/i;
	const match = svgString.match(svgRegex);
	if (!match) {
		return { width: 0, height: 0 };
	}
	const svgAttributes = match[1];
	const regex = /\b(width|height|viewBox)\s*=\s*"([^"]+)"/g;

	let width, height, viewBox;

	let attributeMatch;
	while ((attributeMatch = regex.exec(svgAttributes)) !== null) {
		const attributeName = attributeMatch[1];
		const attributeValue = attributeMatch[2];

		if (attributeName === 'width') {
			width = parseFloat(attributeValue) || undefined;
		} else if (attributeName === 'height') {
			height = parseFloat(attributeValue) || undefined;
		} else if (attributeName === 'viewBox') {
			viewBox = attributeValue.split(' ').map(Number);
		}
	}

	if (!width && viewBox && viewBox.length === 4) {
		const viewBoxWidth = viewBox[2];
		const viewBoxHeight = viewBox[3];
		const aspectRatio = viewBoxWidth / viewBoxHeight;
		width = height ? height * aspectRatio : 150 * aspectRatio;
	}
	if (!height && viewBox && viewBox.length === 4) {
		const viewBoxWidth = viewBox[2];
		const viewBoxHeight = viewBox[3];
		const aspectRatio = viewBoxWidth / viewBoxHeight;
		height = width ? width / aspectRatio : 300 / aspectRatio;
	}

	return { width: width ?? 0, height: height ?? 0 };
}

export class SvgData {
	native: any;

	static fromNative(value) {
		if (value) {
			const data = new SvgData();
			data.native = value;
			return data;
		}
		return null;
	}

	get width(): number {
		return this.native?.getWidth?.() ?? 0;
	}

	get height(): number {
		return this.native?.getHeight?.() ?? 0;
	}

	get data(): ArrayBuffer | null {
		const data = this.native?.getData?.();
		if (data) {
			return (<any>ArrayBuffer).from(data);
		}
		return null;
	}
}

export class Svg extends SVGBase {
	_svg;

	constructor() {
		super();
		const context = Application.android.foregroundActivity || Application.android.startActivity || Utils.android.getApplicationContext();
		this._svg = new org.nativescript.canvas.svg.NSCSVG(context);
		const owner = new WeakRef(this);
		this._svg.setContextListener(
			new org.nativescript.canvas.svg.NSCSVG.ContextListener({
				onContextLost() {
					owner.get()?.__notifyContextLost();
				},
				onContextRestored() {
					owner.get()?.__notifyContextRestored();
				},
			}),
		);
		this.on('layoutChanged', (args) => {
			// A percentage root size is relative to the view, so a new layout can change it.
			this.__rootSizeValid = false;
			this.__redraw();
		});
	}

	createNativeView() {
		return this._svg;
	}

	get native() {
		return this._svg;
	}

	setSvgData(value: SvgData) {
		if (value && value.native) {
			this._svg.loadData(value.native);
		}
	}

	[srcProperty.setNative](value: string) {
		this.__loadSrc(value);
	}

	[syncProperty.setNative](value: boolean) {
		this._svg.setSync(value);
	}

	[gpuProperty.setNative](value: boolean) {
		this._svg.setGpu(value);
	}

	[threadedProperty.setNative](value: boolean) {
		this._svg.setThreaded(value);
	}

	[backendProperty.setNative](value: SvgBackend) {
		const Backend = org.nativescript.canvas.svg.NSCSVG.Backend;
		switch (value) {
			case 'gl':
				this._svg.setBackend(Backend.Gl);
				break;
			case 'vulkan':
				this._svg.setBackend(Backend.Vulkan);
				break;
			case 'metal':
				// No Metal on Android; auto picks whatever the device actually has.
				this._svg.setBackend(Backend.Auto);
				break;
			default:
				this._svg.setBackend(Backend.Auto);
				break;
		}
	}

	[surfaceTypeProperty.setNative](value: SvgSurfaceType) {
		const SurfaceType = org.nativescript.canvas.svg.NSCSVG.SurfaceType;
		this._svg.setSurfaceType(value === 'surface' ? SurfaceType.Surface : SurfaceType.Texture);
	}

	/** Which rasterizer is actually running: `auto` until a surface exists. */
	get activeBackend(): SvgBackend {
		const backend = this._svg?.getActiveBackend?.();
		return (backend ? String(backend.name()).toLowerCase() : 'auto') as SvgBackend;
	}

	/** Throws the GPU context away so the next frame exercises recovery. For testing. */
	debugLoseContext() {
		this._svg?.debugLoseContext?.();
	}

	__redraw() {
		if (this._attachedToDom) {
			// Cached: reading these crosses into native and allocates a string each time, and
			// only a write to them can change them. Deliberately no viewBox default either:
			// inventing one rescales the whole tree.
			this.__resolveRootSize();

			const width = this.__rootWidth;
			const height = this.__rootHeight;
			const scale = Screen.mainScreen.scale * this.__fitScale;
			const pixelWidth = Math.round(width * scale);
			const pixelHeight = Math.round(height * scale);

			this.__document.setContainerSize(width, height);
			// Renders into the view's bitmap directly, with no intermediate buffer, and nothing
			// allocated per frame.
			this._svg.renderDocument(this.__document.nativePointer, pixelWidth, pixelHeight, scale);
		}
	}

	onLoaded() {
		super.onLoaded();
		this._attachedToDom = true;
		this.__startAnimations();
	}

	onUnloaded() {
		this._attachedToDom = false;
		this.__stopAnimations();
		super.onUnloaded();
	}

	addChild(view: SVGItem) {
		this._addView(view);
		view._attached = true;
		this.__children.push(view);
	}

	removeChild(view: SVGItem) {
		if (view._attached) {
			this._removeView(view);
			view._attached = false;
			this.__children = this.__children.filter((item) => item !== view);
		}
	}

	static fromSrcSync(value: string): SvgData | null {
		if (typeof value === 'string') {
			const context = Utils.android.getApplicationContext();
			if (value.indexOf('<svg') > -1) {
				const { width, height } = parseSVGDimensions(value);
				if (width > 0 && height > 0) {
					const svg = org.nativescript.canvas.svg.NSCSVG.fromStringSync(context, width, height, value);
					return SvgData.fromNative(svg);
				}
			} else {
				let nativeSvg;
				try {
					if (value.startsWith('~')) {
						nativeSvg = org.nativescript.canvas.svg.NSCSVG.fromPathSync(context, path.join(knownFolders.currentApp().path, value.replace('~', '')));
					} else if (value.startsWith('/')) {
						nativeSvg = org.nativescript.canvas.svg.NSCSVG.fromPathSync(context, value);
					} else if (value.startsWith('http')) {
						nativeSvg = org.nativescript.canvas.svg.NSCSVG.fromRemoteSync(context, value);
					}
				} catch (error) {}
				if (nativeSvg) {
					return SvgData.fromNative(nativeSvg);
				}
			}
		}
		return null;
	}

	static fromSrc(value: string): Promise<SvgData> {
		return new Promise((resolve, reject) => {
			if (typeof value === 'string') {
				const context = Utils.android.getApplicationContext();
				if (value.indexOf('<svg') > -1) {
					const { width, height } = parseSVGDimensions(value);
					if (width > 0 && height > 0) {
						org.nativescript.canvas.svg.NSCSVG.fromString(
							context,
							width,
							height,
							value,
							new org.nativescript.canvas.svg.NSCSVG.Callback({
								onSuccess(svg) {
									const ret = SvgData.fromNative(svg);
									resolve(ret);
								},
							}),
						);
						return;
					}
				} else {
					const cb = new org.nativescript.canvas.svg.NSCSVG.Callback({
						onSuccess(nativeSvg) {
							if (nativeSvg) {
								const ret = SvgData.fromNative(nativeSvg);
								resolve(ret);
							} else {
								reject(new Error('Failed to parse SVG'));
							}
						},
					});
					if (value.startsWith('~')) {
						org.nativescript.canvas.svg.NSCSVG.fromPath(context, path.join(knownFolders.currentApp().path, value.replace('~', '')), cb);
						return;
					} else if (value.startsWith('/')) {
						org.nativescript.canvas.svg.NSCSVG.fromPath(context, value, cb);
						return;
					} else if (value.startsWith('http')) {
						org.nativescript.canvas.svg.NSCSVG.fromRemote(context, value, cb);
						return;
					}
				}
			}
			reject(new Error('Source is not valid'));
		});
	}
}
