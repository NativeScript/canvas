// Linux: documents come from the Node-API SVG module (canvassvg.node, global.SVGModule), are
// rasterized on the CPU straight into a new cairo image surface's memory (BGRA, premultiplied) and
// painted by a GtkDrawingArea at the content's size in DIPs, one pixel per device pixel. A new
// surface per frame: GTK may still hold the last one.
import { SVGBase, readSrc, srcProperty, type SvgBackend } from './common';
import './canvas-image';
import { Screen } from '@nativescript/core';
import { SVGItem } from './Elements/SVGItem';
import { Helpers } from './helpers';

export * from './Elements';

declare const SVGModule: any, interop: any;
declare const gtk_drawing_area_new: any, gtk_drawing_area_set_draw_func: any, gtk_drawing_area_set_content_width: any, gtk_drawing_area_set_content_height: any;
declare const gtk_widget_queue_draw: any, gtk_widget_get_scale_factor: any;
declare const cairo_image_surface_create: any, cairo_image_surface_get_data: any, cairo_surface_destroy: any, cairo_surface_flush: any, cairo_surface_mark_dirty: any;
declare const cairo_save: any, cairo_restore: any, cairo_scale: any, cairo_set_source_surface: any, cairo_paint: any;

const CAIRO_FORMAT_ARGB32 = 0;

const views = new Map<number, WeakRef<Svg>>();
let nextView = 1;
let drawFunc: any = null;

/** One C draw callback for every view; `user_data` carries the view id. */
function sharedDrawFunc() {
	drawFunc ??= interop.callback(
		(_area: any, cr: any, _width: number, _height: number, data: any) => {
			views
				.get(data?.toNumber?.() ?? 0)
				?.deref()
				?._paint(cr);
		},
		{ returns: interop.types.void, args: [interop.types.pointer, interop.types.pointer, interop.types.int32, interop.types.int32, interop.types.pointer] },
	);
	return drawFunc;
}

/** A cairo image surface the SVG renders into, through a view of its memory. */
interface Raster {
	pixels: Uint8Array;
	surface: any;
}

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
	private _area: any = null;
	private _id = nextView++;
	private _raster: Raster | null = null;
	private _scale = 1;

	constructor() {
		super();
		this.on('layoutChanged', () => {
			// A percentage root size is relative to the view, so a new layout can change it.
			this.__rootSizeValid = false;
			this.__redraw();
		});
	}

	createNativeView() {
		// a new widget each time: disposing the native view frees the previous one
		const area = gtk_drawing_area_new();
		views.set(this._id, new WeakRef(this));
		gtk_drawing_area_set_draw_func(area, sharedDrawFunc(), new interop.Pointer(this._id), null);
		this._area = area;
		return area;
	}

	get native() {
		return this._area;
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

	/** @internal The draw callback: the raster at the content's size, top-left. */
	_paint(cr: any) {
		const raster = this._raster;
		if (!raster) {
			return;
		}
		cairo_save(cr);
		cairo_scale(cr, 1 / this._scale, 1 / this._scale);
		cairo_set_source_surface(cr, raster.surface, 0, 0);
		cairo_paint(cr);
		cairo_restore(cr);
	}

	private _newRaster(width: number, height: number): Raster {
		const surface = cairo_image_surface_create(CAIRO_FORMAT_ARGB32, width, height);
		cairo_surface_flush(surface);
		// ARGB32 rows are width * 4 bytes, as the SVG module writes them.
		return { surface, pixels: new Uint8Array(interop.bufferFromData(cairo_image_surface_get_data(surface), width * height * 4)) };
	}

	private _freeRaster() {
		const raster = this._raster;
		this._raster = null;
		if (raster) {
			cairo_surface_destroy(raster.surface);
		}
	}

	__redraw() {
		if (!this._attachedToDom) {
			return;
		}
		this.__resolveRootSize();
		const width = this.__rootWidth;
		const height = this.__rootHeight;
		const scale = (gtk_widget_get_scale_factor(this._area) || Screen.mainScreen.scale || 1) * this.__fitScale;
		const pixelWidth = Math.round(width * scale);
		const pixelHeight = Math.round(height * scale);
		if (!(pixelWidth > 0 && pixelHeight > 0)) {
			return;
		}
		const raster = this._newRaster(pixelWidth, pixelHeight);
		this._scale = scale / this.__fitScale;
		this.__document.setContainerSize(width, height);
		// cairo's ARGB32 is B, G, R, A in memory here.
		this.__document.renderToBuffer(raster.pixels, pixelWidth, pixelHeight, scale, true);
		cairo_surface_mark_dirty(raster.surface);
		this._freeRaster();
		this._raster = raster;
		gtk_drawing_area_set_content_width(this._area, Math.ceil(width * this.__fitScale));
		gtk_drawing_area_set_content_height(this._area, Math.ceil(height * this.__fitScale));
		gtk_widget_queue_draw(this._area);
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
		views.delete(this._id);
		this._freeRaster();
		this._area = null;
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
