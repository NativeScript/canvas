// Windows: documents come from the Node-API SVG module (canvassvg.node, global.SVGModule), are
// rasterized on the CPU straight into a WriteableBitmap's pixels (BGRA, premultiplied) and shown by
// a XAML Image at the content's size in DIPs, one bitmap pixel per physical pixel.
import { SVGBase, readSrc, srcProperty, type SvgBackend } from './common';
import './canvas-image';
import { Screen } from '@nativescript/core';
import { SVGItem } from './Elements/SVGItem';
import { Helpers } from './helpers';

export * from './Elements';

declare const Microsoft: any, NSWinRT: any, SVGModule: any;

function parseSVGDimensions(svgString: string) {
	const match = svgString.match(/<svg([^>]*)>/i);
	if (!match) {
		return { width: 0, height: 0 };
	}
	const regex = /\b(width|height|viewBox)\s*=\s*"([^"]+)"/g;
	let width: number | undefined, height: number | undefined, viewBox: number[] | undefined;
	let attribute;
	while ((attribute = regex.exec(match[1])) !== null) {
		if (attribute[1] === 'width') {
			width = parseFloat(attribute[2]) || undefined;
		} else if (attribute[1] === 'height') {
			height = parseFloat(attribute[2]) || undefined;
		} else {
			viewBox = attribute[2].split(/[\s,]+/).map(Number);
		}
	}
	if (viewBox?.length === 4) {
		const aspect = viewBox[2] / viewBox[3];
		if (!width) {
			width = height ? height * aspect : 150 * aspect;
		}
		if (!height) {
			height = width / aspect;
		}
	}
	return { width: width ?? 0, height: height ?? 0 };
}

/** An SVG's markup and natural size. */
export class SvgData {
	native: { source: string; width: number; height: number } | null = null;

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

	/** Premultiplied RGBA pixels at the natural size. */
	get data(): ArrayBuffer | null {
		const width = Math.ceil(this.width);
		const height = Math.ceil(this.height);
		if (!(width > 0 && height > 0)) {
			return null;
		}
		Helpers.initialize();
		const document = SVGModule.createSVGDocument(this.native.source);
		if (!document) {
			return null;
		}
		const pixels = new Uint8Array(width * height * 4);
		document.setContainerSize(this.width, this.height);
		document.renderToBuffer(pixels, width, height, 1);
		return pixels.buffer;
	}
}

function svgData(source: string): SvgData | null {
	const { width, height } = parseSVGDimensions(source);
	return width > 0 && height > 0 ? SvgData.fromNative({ source, width, height }) : null;
}

export class Svg extends SVGBase {
	private _container: any;
	private _image: any;
	private _bitmap: any = null;
	private _pixels: Uint8Array | null = null;

	constructor() {
		super();
		const { Controls } = Microsoft.UI.Xaml;
		this._container = new Controls.Grid();
		this._image = new Controls.Image();
		this._image.Stretch = Microsoft.UI.Xaml.Media.Stretch.Fill;
		this._image.HorizontalAlignment = Microsoft.UI.Xaml.HorizontalAlignment.Left;
		this._image.VerticalAlignment = Microsoft.UI.Xaml.VerticalAlignment.Top;
		this._container.Children.Append(this._image);
		this.on('layoutChanged', () => {
			// A percentage root size is relative to the view, so a new layout can change it.
			this.__rootSizeValid = false;
			this.__redraw();
		});
	}

	createNativeView() {
		return this._container;
	}

	get native() {
		return this._image;
	}

	setSvgData(value: SvgData) {
		if (value?.native) {
			this.__loadSource(value.native.source);
		}
	}

	[srcProperty.setNative](value: string) {
		this.__loadSrc(value);
	}

	/** Rasterized on the CPU for now. */
	get activeBackend(): SvgBackend {
		return 'auto';
	}

	__redraw() {
		if (!this._attachedToDom) {
			return;
		}
		this.__resolveRootSize();
		const width = this.__rootWidth;
		const height = this.__rootHeight;
		const scale = Screen.mainScreen.scale * this.__fitScale;
		const pixelWidth = Math.round(width * scale);
		const pixelHeight = Math.round(height * scale);
		if (!(pixelWidth > 0 && pixelHeight > 0)) {
			return;
		}
		if (!this._bitmap || this._bitmap.PixelWidth !== pixelWidth || this._bitmap.PixelHeight !== pixelHeight) {
			this._bitmap = new Microsoft.UI.Xaml.Media.Imaging.WriteableBitmap(pixelWidth, pixelHeight);
			// The bitmap's own memory: rendered into with no copy.
			this._pixels = new Uint8Array(NSWinRT.interop.arrayBufferFromBuffer(this._bitmap.PixelBuffer));
			this._image.Source = this._bitmap;
		}
		this._image.Width = width * this.__fitScale;
		this._image.Height = height * this.__fitScale;
		this.__document.setContainerSize(width, height);
		this.__document.renderToBuffer(this._pixels, pixelWidth, pixelHeight, scale, true);
		this._bitmap.Invalidate();
	}

	onLoaded() {
		super.onLoaded();
		this._attachedToDom = true;
		this.__startAnimations();
		this.__redraw();
	}

	onUnloaded() {
		this._attachedToDom = false;
		this.__stopAnimations();
		super.onUnloaded();
	}

	disposeNativeView() {
		this._bitmap = null;
		this._pixels = null;
		super.disposeNativeView();
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
		// Markup only: reading a file or URL synchronously is not possible here.
		return typeof value === 'string' && value.indexOf('<svg') > -1 ? svgData(value) : null;
	}

	static fromSrc(value: string): Promise<SvgData> {
		return new Promise((resolve, reject) => {
			if (typeof value !== 'string' || value.length === 0) {
				reject(new Error('Source is not valid'));
				return;
			}
			readSrc(
				value,
				(source) => {
					const data = svgData(source);
					if (data) {
						resolve(data);
					} else {
						reject(new Error('Failed to parse SVG'));
					}
				},
				reject,
			);
		});
	}
}
