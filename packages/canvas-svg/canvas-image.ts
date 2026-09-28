import { File, knownFolders, path } from '@nativescript/core';
import { SVGBase, readSrc } from './common';
import { SvgDocumentWrapper } from './NativeNode';

// Lets @nativescript/canvas use SVGs as image sources without importing this package.

interface ImageAssetLike {
	loadFromBytesSync(width: number, height: number, bytes: Uint8Array, premultiplied?: boolean): boolean;
}

interface SvgLoadOptions {
	width?: number;
	height?: number;
	scale?: number;
	time?: number;
}

// Caps the longest raster side.
const MAX_PIXELS = 8192;

/** Last raster per asset, so an unchanged frame is not re-rendered. */
interface Snapshot {
	document: SvgDocumentWrapper;
	revision: number;
	width: number;
	height: number;
	buffer: Uint8Array;
}

const snapshots = new WeakMap<object, Snapshot>();

function resolve(value: any): SVGBase | null {
	if (value instanceof SVGBase) {
		return value;
	}
	// Polyfill `<svg>` holds the view as `nativeElement`, `_fromSvg` images as `_svg`.
	if (value?.nativeElement instanceof SVGBase) {
		return value.nativeElement;
	}
	if (value?._svg instanceof SVGBase) {
		return value._svg;
	}
	return null;
}

function naturalSize(view: SVGBase) {
	view.__resolveRootSize();
	return { width: view.__rootWidth * view.__fitScale, height: view.__rootHeight * view.__fitScale };
}

function clampScale(width: number, height: number, scale: number) {
	return Math.min(scale, MAX_PIXELS / Math.max(width, height));
}

function rasterize(view: SVGBase, scale: number, asset: ImageAssetLike): boolean {
	const size = naturalSize(view);
	if (!(size.width > 0 && size.height > 0 && scale > 0)) {
		return false;
	}
	scale = clampScale(size.width, size.height, scale);
	const width = Math.max(1, Math.round(size.width * scale));
	const height = Math.max(1, Math.round(size.height * scale));
	const document = view.__document;

	const last = snapshots.get(asset);
	if (last && last.document === document && last.revision === document.revision && last.width === width && last.height === height) {
		return true;
	}

	// Reused so an animated view does not allocate every frame.
	const buffer = last?.buffer.length === width * height * 4 ? last.buffer : new Uint8Array(width * height * 4);
	document.setContainerSize(view.__rootWidth, view.__rootHeight);
	document.renderToBuffer(buffer, width, height, scale * view.__fitScale);
	if (!asset.loadFromBytesSync(width, height, buffer, true)) {
		snapshots.delete(asset);
		return false;
	}
	snapshots.set(asset, { document, revision: document.revision, width, height, buffer });
	return true;
}

/** width/height, else viewBox, else 300x150, in CSS pixels. */
function intrinsicSize(document: SvgDocumentWrapper) {
	const root = document.rootElement;
	const length = (name: string) => {
		const value = root.getAttribute(name);
		return !value || value.trim().endsWith('%') ? NaN : parseFloat(value);
	};
	let width = length('width');
	let height = length('height');
	const box = (root.getAttribute('viewBox') ?? '')
		.trim()
		.split(/[\s,]+/)
		.map(Number);
	const hasBox = box.length === 4 && box[2] > 0 && box[3] > 0;
	if (hasBox) {
		if (!(width > 0) && !(height > 0)) {
			width = box[2];
			height = box[3];
		} else if (!(width > 0)) {
			width = (height * box[2]) / box[3];
		} else if (!(height > 0)) {
			height = (width * box[3]) / box[2];
		}
	}
	return { width: width > 0 ? width : 300, height: height > 0 ? height : 150 };
}

/** Keeps the aspect ratio; the tighter side wins. */
function fitFactor(natural: { width: number; height: number }, options?: SvgLoadOptions) {
	const kx = options?.width > 0 ? options.width / natural.width : NaN;
	const ky = options?.height > 0 ? options.height / natural.height : NaN;
	if (kx > 0 && ky > 0) {
		return Math.min(kx, ky);
	}
	return kx > 0 ? kx : ky > 0 ? ky : 1;
}

function loadMarkup(markup: string, asset: ImageAssetLike, options?: SvgLoadOptions): boolean {
	let document: SvgDocumentWrapper;
	try {
		document = new SvgDocumentWrapper(markup);
	} catch (error) {
		throw new Error('Could not parse SVG');
	}
	const natural = intrinsicSize(document);
	let scale = fitFactor(natural, options) * (options?.scale > 0 ? options.scale : 1);
	scale = clampScale(natural.width, natural.height, scale);
	const width = Math.max(1, Math.round(natural.width * scale));
	const height = Math.max(1, Math.round(natural.height * scale));

	if (document.hasAnimations) {
		document.setCurrentTime(options?.time ?? 0);
	}
	const buffer = new Uint8Array(width * height * 4);
	document.setContainerSize(natural.width, natural.height);
	document.renderToBuffer(buffer, width, height, scale);
	snapshots.delete(asset);
	return asset.loadFromBytesSync(width, height, buffer, true);
}

function loadView(view: SVGBase, asset: ImageAssetLike, options?: SvgLoadOptions): boolean {
	const natural = naturalSize(view);
	return rasterize(view, fitFactor(natural, options) * (options?.scale > 0 ? options.scale : 1), asset);
}

function readSrcSync(value: string): string {
	if (value.indexOf('<svg') > -1) {
		return value;
	}
	if (value.startsWith('http')) {
		throw new Error('loadSvgSync cannot fetch a URL; use loadSvg');
	}
	const file = value.startsWith('~') ? path.join(knownFolders.currentApp().path, value.replace('~', '')) : value;
	// File.fromPath creates a missing file.
	if (!File.exists(file)) {
		throw new Error(`No SVG at ${file}`);
	}
	return File.fromPath(file).readTextSync();
}

function invalidSource() {
	return new TypeError('Expected SVG markup, a path, a URL or an Svg view');
}

const provider = {
	resolve,
	naturalSize,
	rasterize,
	loadSync(source: string | object, asset: ImageAssetLike, options?: SvgLoadOptions): boolean {
		const view = resolve(source);
		if (view) {
			return loadView(view, asset, options);
		}
		if (typeof source !== 'string') {
			throw invalidSource();
		}
		return loadMarkup(readSrcSync(source), asset, options);
	},
	load(source: string | object, asset: ImageAssetLike, options?: SvgLoadOptions): Promise<boolean> {
		const view = resolve(source);
		if (view) {
			return Promise.resolve(loadView(view, asset, options));
		}
		if (typeof source !== 'string') {
			return Promise.reject(invalidSource());
		}
		return new Promise((resolve, reject) => {
			readSrc(
				source,
				(markup) => {
					try {
						resolve(loadMarkup(markup, asset, options));
					} catch (error) {
						reject(error);
					}
				},
				reject,
			);
		});
	},
};

(global as any).__canvasSvgImageProvider = provider;
