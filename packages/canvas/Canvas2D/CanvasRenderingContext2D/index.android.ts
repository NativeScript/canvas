import { CanvasRenderingContext2DBase } from './common';
import { CanvasGradient } from '../CanvasGradient';
import { Path2D } from '../Path2D';
import { ImageData } from '../ImageData';
import { TextMetrics } from '../TextMetrics';
import { ImageSource, Screen } from '@nativescript/core';
import { ImageAsset } from '../../ImageAsset';
import { CanvasPattern } from '../CanvasPattern';
import { Canvas } from '../../Canvas';

import { Helpers } from '../../helpers';

let ctor;

export class CanvasRenderingContext2D extends CanvasRenderingContext2DBase {
	public static isDebug = false;
	static colorCache = {};
	private context;

	static {
		Helpers.initialize();
		ctor = global.CanvasJSIModule.create2DContext;
	}

	constructor(context: any, contextOptions) {
		super();
		this.context = context;

		const width = context.getDrawingBufferWidth(); // can use  getMeasuredWidth/ getMeasuredHeight
		const height = context.getDrawingBufferHeight();
		const ctx = BigInt(context.getNativeContext().toString());

		let direction = 0;
		if (androidx.core.text.TextUtilsCompat.getLayoutDirectionFromLocale(java.util.Locale.getDefault()) === androidx.core.view.ViewCompat.LAYOUT_DIRECTION_RTL) {
			direction = 1;
		}

		this.context = ctor(width, height, Screen.mainScreen.scale, ctx, 1, contextOptions.alpha, 0, 1, direction);
	}

	get native() {
		return this.context;
	}

	get canvas() {
		return this._canvas;
	}

	get shadowColor() {
		this.log('shadowColor');
		return this.context.shadowColor;
	}

	set shadowColor(color: string) {
		this.log('shadowColor value:', color);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			this.context.shadowColor = color;
		}
	}

	get font(): string {
		this.log('get font');
		return this.context.font;
	}

	set font(value: string) {
		this.log('set font value:', value);
		if (this.context) {
			this.context.font = value;
		}
	}

	get direction(): string {
		this.log('direction');
		return this.context.direction;
	}

	set direction(value: string) {
		this.log('direction value:', value);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			this.context.direction = value;
		}
	}

	get globalAlpha(): number {
		this.log('globalAlpha');
		return this.context.globalAlpha;
	}

	set globalAlpha(alpha: number) {
		this.log('globalAlpha value:', alpha);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			this.context.globalAlpha = alpha;
		}
	}

	get imageSmoothingEnabled() {
		this.log('imageSmoothingEnabled');
		return this.context.imageSmoothingEnabled;
	}

	set imageSmoothingEnabled(enabled: boolean) {
		this.log('imageSmoothingEnabled value:', enabled);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			this.context.imageSmoothingEnabled = enabled;
		}
	}

	get imageSmoothingQuality() {
		this.log('imageSmoothingQuality');
		return this.context.imageSmoothingQuality;
	}

	set imageSmoothingQuality(quality: string) {
		this.log('imageSmoothingQuality value:', quality);
		this._ensureLayoutBeforeDraw();
		this.context.imageSmoothingQuality = quality;
	}

	get lineDashOffset() {
		this.log('lineDashOffset');
		return this.context.lineDashOffset;
	}

	set lineDashOffset(offset: number) {
		this.log('lineDashOffset value:', offset);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			this.context.lineDashOffset = offset;
		}
	}

	get lineJoin() {
		this.log('lineJoin');
		return this.context.lineJoin;
	}

	set lineJoin(join: string) {
		this.log('lineJoin value:', join);
		this._ensureLayoutBeforeDraw();
		this.context.lineJoin = join;
	}

	get lineCap() {
		this.log('lineCap');
		return this.context.lineCap;
	}

	set lineCap(cap: string) {
		this.log('lineCap value:', cap);
		this._ensureLayoutBeforeDraw();
		this.context.lineCap = cap;
	}

	get miterLimit() {
		this.log('miterLimit value:');
		return this.context.miterLimit;
	}

	set miterLimit(limit: number) {
		this.log('miterLimit value:', limit);
		this._ensureLayoutBeforeDraw();
		this.context.miterLimit = limit;
	}

	get shadowBlur() {
		this.log('shadowBlur');
		return this.context.shadowBlur;
	}

	set shadowBlur(blur: number) {
		this.log('shadowBlur value:', blur);
		this._ensureLayoutBeforeDraw();
		this.context.shadowBlur = blur;
	}

	get shadowOffsetX() {
		this.log('shadowOffsetX');
		return this.context.shadowOffsetX;
	}

	set shadowOffsetX(x: number) {
		this.log('shadowOffsetX value:', x);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			this.context.shadowOffsetX = x;
		}
	}

	get shadowOffsetY() {
		this.log('shadowOffsetY');
		return this.context.shadowOffsetY;
	}

	set shadowOffsetY(y: number) {
		this.log('shadowOffsetY value:', y);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			this.context.shadowOffsetY = y;
		}
	}

	get textAlign() {
		this.log('textAlign');
		return this.context.textAlign;
	}

	set textAlign(alignment: string) {
		this.log('textAlign value:', alignment);
		this._ensureLayoutBeforeDraw();
		this.context.setTextAlign = alignment;
	}

	get globalCompositeOperation() {
		this.log('globalCompositeOperation');
		return this.context.globalCompositeOperation;
	}

	set globalCompositeOperation(composite: string) {
		this.log('globalCompositeOperation value:', composite);
		this._ensureLayoutBeforeDraw();
		this.context.globalCompositeOperation = composite;
	}

	get fillStyle() {
		this.log('fillStyle');
		return this.context.fillStyle;
	}

	set fillStyle(color: string | CanvasGradient | CanvasPattern) {
		this.log('fillStyle:', color);
		if (color === undefined || color === null) {
			return;
		}
		this._ensureLayoutBeforeDraw();

		if (color instanceof CanvasGradient || color instanceof CanvasPattern) {
			this.context.fillStyle = color.native;
		} else {
			this.context.fillStyle = color;
		}
	}

	get filter(): string {
		this.log('get filter');
		return this.context.filter;
	}

	set filter(value: string) {
		this.log('set filter', value);
		this.context.filter = value;
	}

	get strokeStyle() {
		this.log('strokeStyle');
		return this.context.strokeStyle;
	}

	set strokeStyle(color: string | CanvasGradient | CanvasPattern) {
		this.log('strokeStyle:', color);
		if (color === undefined || color === null) {
			return;
		}
		this._ensureLayoutBeforeDraw();
		if (color instanceof CanvasGradient || color instanceof CanvasPattern) {
			this.context.strokeStyle = color.native;
		} else {
			this.context.strokeStyle = color;
		}
	}

	get lineWidth() {
		this.log('lineWidth');
		return this.context.lineWidth;
	}

	set lineWidth(width: number) {
		this.log('lineWidth value:', width);
		this._ensureLayoutBeforeDraw();
		this.context.lineWidth = width;
	}

	_methodCache = new Map();

	_getMethod(name: string) {
		const cached = this._methodCache.get(name);
		if (cached === undefined) {
			const ret = this.context[name];
			this._methodCache.set(name, ret);
			return ret;
		}

		return cached;
	}

	addHitRegion(region: any): void {}

	arc(x: number, y: number, radius: number, startAngle: number, endAngle: number, anticlockwise: boolean = false): void {
		this.log('arc value:', x, y, radius, startAngle, endAngle, anticlockwise);
		this._ensureLayoutBeforeDraw();
		const arc = this._getMethod('arc');
		arc(x, y, radius, startAngle, endAngle, anticlockwise);
	}

	arcTo(x1: number, y1: number, x2: number, y2: number, radius: number): void {
		this.log('arcTo value:', x1, y1, x2, y2, radius);
		this._ensureLayoutBeforeDraw();
		const arcTo = this._getMethod('arcTo');
		arcTo(x1, y1, x2, y2, radius);
	}

	beginPath(): void {
		this.log('beginPath');
		this._ensureLayoutBeforeDraw();
		const beginPath = this._getMethod('beginPath');
		beginPath();
	}

	bezierCurveTo(cp1x: number, cp1y: number, cp2x: number, cp2y: number, x: number, y: number): void {
		this.log('bezierCurveTo value:', cp1x, cp1y, cp2x, cp2y, x, y);
		this._ensureLayoutBeforeDraw();
		const bezierCurveTo = this._getMethod('bezierCurveTo');
		bezierCurveTo(cp1x, cp1y, cp2x, cp2y, x, y);
	}

	clearHitRegions(): void {}

	clearRect(x: number, y: number, width: number, height: number): void {
		this.log('clearRect value:', x, y, width, height);
		this._ensureLayoutBeforeDraw();
		const clearRect = this._getMethod('clearRect');
		clearRect(x, y, width, height);
	}

	clip(): void;

	clip(fillRule: string): void;

	clip(path: Path2D, fillRule: string): void;

	clip(...args: any): void {
		this.log('clip value:', ...args);
		this._ensureLayoutBeforeDraw();
		const clip = this._getMethod('clip');
		if (typeof args[0] === 'string') {
			clip(args[0]);
		} else if (args[0] instanceof Path2D && typeof args[1] === 'string') {
			clip(args[0].native, args[1]);
		} else if (args[0] instanceof Path2D) {
			clip(args[0].native);
		} else {
			clip();
		}
	}

	closePath(): void {
		this.log('closePath');
		this._ensureLayoutBeforeDraw();
		const closePath = this._getMethod('closePath');
		closePath();
	}

	createImageData(width: number, height: number): ImageData;

	createImageData(data: ImageData): ImageData;

	createImageData(width: number | ImageData, height?: number): ImageData {
		this.log('createImageData value:', arguments);
		if (width instanceof ImageData) {
			return new ImageData(width.width, width.height);
		} else {
			return new ImageData(width, height);
		}
	}

	createLinearGradient(x0: number, y0: number, x1: number, y1: number) {
		this.log('createLinearGradient value:', x0, y0, x1, y1);
		this._ensureLayoutBeforeDraw();
		const createLinearGradient = this._getMethod('createLinearGradient');
		return CanvasGradient.fromNative(createLinearGradient(x0, y0, x1, y1));
	}

	createPattern(image: any, repetition: string): CanvasPattern | null {
		this.log('createPattern value:', image, repetition);
		this._ensureLayoutBeforeDraw();
		if (repetition === undefined || typeof repetition !== 'string') {
			const e = new Error('The string did not match the expected pattern.');
			e.name = 'SyntaxError';
			throw e;
		}

		const createPattern = this._getMethod('createPattern');

		let img;
		if (image instanceof ImageSource) {
			img = image.android;
		} else if (image instanceof android.graphics.Bitmap) {
			img = image;
		} else if (image instanceof ImageAsset) {
			img = image.native;
		} else if (image instanceof Canvas) {
			img = image.android;
		} else if (image && typeof image.tagName === 'string' && (image.tagName === 'IMG' || image.tagName === 'IMAGE')) {
			if (image._imageSource instanceof ImageSource) {
				img = image._imageSource.android;
			} else if (image._image instanceof android.graphics.Bitmap) {
				img = image._image;
			} else if (image._asset instanceof ImageAsset) {
				img = image._asset.native;
			} else if (typeof image.src === 'string') {
				img = ImageSource.fromFileSync(image.src).android;
			}
		} else if (image && typeof image.tagName === 'string' && image.tagName === 'CANVAS' && image._canvas instanceof Canvas) {
			img = image._canvas.android;
		} else if (image instanceof ImageBitmap || image?.nativeInstance instanceof org.nativescript.canvas.TNSImageBitmap) {
			img = image.native;
		}
		if (!img) {
			return null;
		}

		return new CanvasPattern(createPattern(img, repetition));
	}

	createRadialGradient(x0: number, y0: number, r0: number, x1: number, y1: number, r1: number) {
		this.log('createRadialGradient value:', x0, y0, r0, x1, y1, r1);
		this._ensureLayoutBeforeDraw();
		const createRadialGradient = this._getMethod('createRadialGradient');
		return CanvasGradient.fromNative(createRadialGradient(x0, y0, r0, x1, y1, r1));
	}

	drawFocusIfNeeded(element): void;

	drawFocusIfNeeded(path, element): void;

	drawFocusIfNeeded(...args: any): void {}

	drawImage(image: any, dx: number, dy: number): void;

	drawImage(image: any, dx: number, dy: number, dWidth: number, dHeight: number): void;

	drawImage(image: any, sx: number, sy: number, sWidth: number, sHeight: number, dx: number, dy: number, dWidth: number, dHeight: number): void;

	drawImage(...args): void {
		this.log('drawImage value:', ...args);
		this._ensureLayoutBeforeDraw();
		const drawImage = this._getMethod('drawImage');
		let image = args[0];
		if (image instanceof ImageAsset) {
			image = image.native;
		} else if (image instanceof ImageSource) {
			image = image.android;
		} else if (image instanceof android.graphics.Bitmap) {
			image = image;
		} else if (image instanceof Canvas) {
			image = image.android;
		} else if (image && typeof image.tagName === 'string' && (image.tagName === 'IMG' || image.tagName === 'IMAGE')) {
			if (image._imageSource instanceof ImageSource) {
				image = image._imageSource.android;
			} else if (image._image instanceof android.graphics.Bitmap) {
				image = image._image;
			} else if (image._asset instanceof ImageAsset) {
				image = image._asset.native;
			} else if (typeof image.src === 'string') {
				image = ImageSource.fromFileSync(image.src).android;
			}
		} else if (image && typeof image.tagName === 'string' && image.tagName === 'CANVAS' && image._canvas instanceof Canvas) {
			image = image._canvas.android;
		} else if (image instanceof ImageBitmap || image?.nativeInstance instanceof org.nativescript.canvas.TNSImageBitmap) {
			image = image.native;
		}

		if (args.length === 3) {
			drawImage(image, args[1], args[2]);
		} else if (args.length === 5) {
			drawImage(image, args[1], args[2], args[3], args[4]);
		} else if (args.length === 9) {
			drawImage(image, args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8]);
		}
	}

	ellipse(x: number, y: number, radiusX: number, radiusY: number, rotation: number, startAngle: number, endAngle: number, anticlockwise: boolean = false): void {
		this.log('ellipse value:', x, y, radiusX, radiusY, rotation, startAngle, endAngle, anticlockwise);
		this._ensureLayoutBeforeDraw();
		const ellipse = this._getMethod('ellipse');
		ellipse(x, y, radiusX, radiusY, rotation, startAngle, endAngle, anticlockwise);
	}

	fill(): void;

	fill(fillRule: string): void;

	fill(path: Path2D, fillRule: string): void;

	fill(...args: any): void {
		this.log('fill value:', ...args);
		this._ensureLayoutBeforeDraw();
		const fill = this._getMethod('fill');
		if (typeof args[0] === 'string') {
			fill(args[0]);
		} else if (args[0] instanceof Path2D && typeof args[1] === 'string') {
			fill(args[0].native, args[1]);
		} else if (args[0] instanceof Path2D) {
			fill(args[0].native);
		} else {
			fill();
		}
	}

	fillRect(x: number, y: number, width: number, height: number): void {
		this.log('fillRect value:', x, y, width, height);
		this._ensureLayoutBeforeDraw();
		const fillRect = this._getMethod('fillRect');
		fillRect(x, y, width, height);
	}

	fillText(text: string, x: number, y: number, maxWidth: number): void {
		this.log('fillText value:', text, x, y, maxWidth);
		this._ensureLayoutBeforeDraw();
		const fillText = this._getMethod('fillText');
		if (typeof maxWidth === 'number') {
			fillText(text + '', x, y, maxWidth);
		} else {
			fillText(text + '', x, y);
		}
	}

	getImageData(sx: number, sy: number, sw: number, sh: number): ImageData {
		this.log('getImageData value:', sx, sy, sw, sh);
		this._ensureLayoutBeforeDraw();
		const getImageData = this._getMethod('getImageData');
		return ImageData.fromNative(getImageData(sx, sy, sw, sh));
	}

	getLineDash(): number[] {
		this.log('getLineDash');
		this._ensureLayoutBeforeDraw();
		const getLineDash = this._getMethod('getLineDash');
		return getLineDash();
	}

	isPointInPath(x: number, y: number, fillRule: string): boolean;

	isPointInPath(path: Path2D, x: number, y: number, fillRule: string): boolean;

	isPointInPath(...args): boolean {
		this.log('isPointInPath value:', ...args);
		this._ensureLayoutBeforeDraw();
		const isPointInPath = this._getMethod('isPointInPath');
		if (args.length === 2) {
			return isPointInPath(args[0], args[1]);
		} else if (args.length === 3) {
			return isPointInPath(args[0], args[1], args[2]);
		} else if (args.length === 4 && args[0] instanceof Path2D) {
			return isPointInPath(args[0].native, args[1], args[2], args[3]);
		}
		return false;
	}

	isPointInStroke(x: number, y: number): boolean;

	isPointInStroke(path: Path2D, x: number, y: number): boolean;

	isPointInStroke(...args): boolean {
		this.log('isPointInStroke value:', ...args);
		this._ensureLayoutBeforeDraw();
		const isPointInStroke = this._getMethod('isPointInStroke');
		if (args.length === 2) {
			return isPointInStroke(args[0], args[1]);
		} else if (args.length === 3 && args[0] instanceof Path2D) {
			return isPointInStroke(args[0].native, args[1], args[2]);
		}
		return false;
	}

	lineTo(x: number, y: number): void {
		this.log('lineTo value:', x, y);
		this._ensureLayoutBeforeDraw();
		const lineTo = this._getMethod('lineTo');
		lineTo(x, y);
	}

	measureText(text: string): TextMetrics {
		this.log('measureText value:', text);
		this._ensureLayoutBeforeDraw();
		const measureText = this._getMethod('measureText');
		return new TextMetrics(measureText(text));
	}

	moveTo(x: number, y: number): void {
		this.log('moveTo value:', x, y);
		this._ensureLayoutBeforeDraw();
		const moveTo = this._getMethod('moveTo');
		moveTo(x, y);
	}

	putImageData(imageData: ImageData, dx: number, dy: number): void;

	putImageData(imageData: ImageData, dx: number, dy: number, dirtyX: number, dirtyY: number, dirtyWidth: number, dirtyHeight: number): void;

	putImageData(imageData: ImageData, dx: number, dy: number, dirtyX?: number, dirtyY?: number, dirtyWidth?: number, dirtyHeight?: number): void;

	putImageData(...args): void {
		this.log('putImageData value:', ...args);
		this._ensureLayoutBeforeDraw();
		if (!args) {
			return;
		}

		const putImageData = this._getMethod('putImageData');
		let data = args[0] as any;
		if (args.length === 3) {
			putImageData(data.native, args[1], args[2]);
		} else if (args.length === 7) {
			putImageData(data.native, args[1], args[2], args[3], args[4], args[5], args[6]);
		}
	}

	quadraticCurveTo(cpx: number, cpy: number, x: number, y: number) {
		this.log('quadraticCurveTo value:', cpx, cpy, x, y);
		this._ensureLayoutBeforeDraw();
		const quadraticCurveTo = this._getMethod('quadraticCurveTo');
		quadraticCurveTo(cpx, cpy, x, y);
	}

	rect(x: number, y: number, width: number, height: number): void {
		this.log('rect value:', x, y, width, height);
		this._ensureLayoutBeforeDraw();
		const rect = this._getMethod('rect');
		rect(x, y, width, height);
	}

	removeHitRegion(id: string): void {}

	resetTransform(): void {
		this.log('resetTransform value:');
		this._ensureLayoutBeforeDraw();
		const resetTransform = this._getMethod('resetTransform');
		resetTransform();
	}

	restore(): void {
		this.log('restore');
		this._ensureLayoutBeforeDraw();
		const restore = this._getMethod('restore');
		restore();
	}

	rotate(angle: number): void {
		this.log('rotate value:', angle);
		this._ensureLayoutBeforeDraw();
		const rotate = this._getMethod('rotate');
		rotate(angle);
	}

	save(): void {
		this.log('save');
		this._ensureLayoutBeforeDraw();
		const save = this._getMethod('save');
		save();
	}

	scale(x: number, y: number): void {
		this.log('scale value:', x, y);
		this._ensureLayoutBeforeDraw();
		const scale = this._getMethod('scale');
		scale(x, y);
	}

	scrollPathIntoView(): void;

	scrollPathIntoView(path: Path2D): void;

	scrollPathIntoView(path?: Path2D): void {}

	setLineDash(segments: number[]): void {
		this.log('setLineDash value:', segments);
		this._ensureLayoutBeforeDraw();
		const setLineDash = this._getMethod('setLineDash');
		setLineDash(segments);
	}

	setTransform(a: number, b: number, c: number, d: number, e: number, f: number): void {
		this.log('setTransform value:', a, b, c, d, e, f);
		this._ensureLayoutBeforeDraw();
		const setTransform = this._getMethod('setTransform');
		setTransform(a, b, c, d, e, f);
	}

	stroke(): void;

	stroke(path?: Path2D): void {
		this.log('stroke value:', path);
		this._ensureLayoutBeforeDraw();
		const stroke = this._getMethod('stroke');
		if (path) {
			stroke(path.native);
		} else {
			stroke();
		}
	}

	strokeRect(x: number, y: number, width: number, height: number): void {
		this.log('strokeRect value:', x, y, width, height);
		this._ensureLayoutBeforeDraw();
		const strokeRect = this._getMethod('strokeRect');
		strokeRect(x, y, width, height);
	}

	strokeText(text: string, x: number, y: number, maxWidth?: number): void {
		this.log('strokeText value:', text, x, y, maxWidth);
		this._ensureLayoutBeforeDraw();
		const strokeText = this._getMethod('strokeText');
		if (typeof maxWidth === 'number') {
			strokeText(text + '', x, y, maxWidth);
		} else {
			strokeText(text + '', x, y);
		}
	}

	transform(a: number, b: number, c: number, d: number, e: number, f: number): void {
		this.log('transform value:', a, b, c, d, e, f);
		this._ensureLayoutBeforeDraw();
		const transform = this._getMethod('transform');
		transform(a, b, c, d, e, f);
	}

	translate(x: number, y: number): void {
		this.log('translate value:', x, y);
		this._ensureLayoutBeforeDraw();
		const translate = this._getMethod('translate');
		translate(x, y);
	}

	private _ensureLayoutBeforeDraw() {
		if (this.canvas) {
			this.canvas._layoutNative();
		}
	}

	private log(message, ...args) {
		if (!CanvasRenderingContext2D.isDebug) {
			return;
		}
		console.log(message, args);
	}
}
