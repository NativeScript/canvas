import { SVGBase, backendProperty, gpuProperty, srcProperty, syncProperty, type SvgBackend, threadedProperty } from './common';
import './canvas-image';
import { Http, knownFolders, path, Screen, Utils } from '@nativescript/core';
import { SVGItem } from './Elements/SVGItem';

export * from './Elements';

declare const NSCSVG, CanvasSVGHelper;

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
		return this.native?.width ?? 0;
	}

	get height(): number {
		return this.native?.height ?? 0;
	}

	get data(): ArrayBuffer | null {
		const data = this.native?.data;
		if (data) {
			return interop.bufferFromData(data);
		}
		return null;
	}
}

export class Svg extends SVGBase {
	_svg: NSCSVG;

	constructor() {
		super();
		this._svg = NSCSVG.alloc().initWithFrame(CGRectZero);
		this._svg.backgroundColor = UIColor.clearColor;
		const owner = new WeakRef(this);
		this._svg.onContextLost = () => {
			owner.get()?.__notifyContextLost();
		};
		this._svg.onContextRestored = () => {
			owner.get()?.__notifyContextRestored();
		};
		this.on('layoutChanged', () => {
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
		this._svg.sync = value;
	}

	[gpuProperty.setNative](value: boolean) {
		this._svg.gpu = value;
	}

	[threadedProperty.setNative](value: boolean) {
		this._svg.threaded = value;
	}

	[backendProperty.setNative](value: SvgBackend) {
		// Metal is the only GPU backend on Apple platforms; anything else means auto.
		this._svg.backend = value === 'metal' ? NSCSVG.Backend.Metal : NSCSVG.Backend.Auto;
	}

	/** Which rasterizer is actually running: `auto` until a surface exists. */
	get activeBackend(): SvgBackend {
		return this._svg?.activeBackend === NSCSVG.Backend.Metal ? 'metal' : 'auto';
	}

	public onLayout(left: number, top: number, right: number, bottom: number): void {
		super.onLayout(left, top, right, bottom);
		this.__redraw();
	}

	public onMeasure(widthMeasureSpec: number, heightMeasureSpec: number) {
		const nativeView = this.nativeView;

		if (nativeView) {
			const width = Utils.layout.getMeasureSpecSize(widthMeasureSpec);
			const height = Utils.layout.getMeasureSpecSize(heightMeasureSpec);
			this.setMeasuredDimension(width, height);
		}
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
			// Renders into the view's own pixels directly, with no intermediate buffer, and
			// nothing allocated per frame.
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
			if (value.indexOf('<svg') > -1) {
				const { width, height } = parseSVGDimensions(value);
				if (width > 0 && height > 0) {
					const native = NSCSVG.fromSVGStringSync(value);
					return SvgData.fromNative(native);
				}
			} else {
				let nativeSvg;
				try {
					if (value.startsWith('~')) {
						nativeSvg = NSCSVG.fromPathSync(path.join(knownFolders.currentApp().path, value.replace('~', '')));
					} else if (value.startsWith('/')) {
						nativeSvg = NSCSVG.fromPathSync(value);
					} else if (value.startsWith('http')) {
						nativeSvg = NSCSVG.fromRemoteSync(value);
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
				if (value.indexOf('<svg') > -1) {
					const { width, height } = parseSVGDimensions(value);
					if (width > 0 && height > 0) {
						NSCSVG.fromString(value, (svg) => {
							const ret = SvgData.fromNative(svg);
							resolve(ret);
						});
						return;
					}
				} else {
					const cb = (nativeSvg) => {
						if (nativeSvg) {
							const ret = SvgData.fromNative(nativeSvg);
							resolve(ret);
						} else {
							reject(new Error('Failed to parse SVG'));
						}
					};
					if (value.startsWith('~')) {
						NSCSVG.fromPath(path.join(knownFolders.currentApp().path, value.replace('~', '')), cb);
						return;
					} else if (value.startsWith('/')) {
						NSCSVG.fromPath(value, cb);
						return;
					} else if (value.startsWith('http')) {
						NSCSVG.fromRemote(value, cb);
						return;
					}
				}
			}
			reject(new Error('Source is not valid'));
		});
	}
}
