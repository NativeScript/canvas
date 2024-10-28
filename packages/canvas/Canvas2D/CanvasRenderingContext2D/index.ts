import { CanvasGradient } from '../CanvasGradient';
import { Path2D } from '../Path2D';
import { ImageData } from '../ImageData';
import { TextMetrics } from '../TextMetrics';
import { ImageSource, Screen, Color } from '@nativescript/core';
import { ImageAsset } from '../../ImageAsset';
import { CanvasPattern } from '../CanvasPattern';
import { Canvas } from '../../Canvas';

import { Helpers } from '../../helpers';
import { DOMMatrix } from '../DOMMatrix';
declare const NSCCanvasRenderingContext2D;

function ruleToEnum(rule: string): number {
	switch (rule) {
		case 'nonzero':
			return 0;
		case 'evenodd':
			return 1;
		default:
			return -1; // will be ignored
	}
}

function toBaseLine(value: string): number {
	switch (value) {
		case 'top':
			return 0;
		case 'hanging':
			return 1;
		case 'middle':
			return 2;
		case 'alphabetic':
			return 3;
		case 'ideographic':
			return 4;
		case 'bottom':
			return 5;
		default:
			return -1;
	}
}

function fromBaseLine(value: number): string {
	switch (value) {
		case 0:
			return 'top';
		case 1:
			return 'hanging';
		case 2:
			return 'middle';
		case 3:
			return 'alphabetic';
		case 4:
			return 'ideographic';
		case 5:
			return 'bottom';
		default:
			return null;
	}
}

function toBlendMode(value: string): number {
	switch (value) {
		case 'source-over':
			return 0;
		case 'source-in':
			return 1;
		case 'source-out':
			return 2;
		case 'source-atop':
			return 3;
		case 'destination-over':
			return 4;
		case 'destination-in':
			return 5;
		case 'destination-out':
			return 6;
		case 'destination-atop':
			return 7;
		case 'lighter':
			return 8;
		case 'copy':
			return 9;
		case 'xor':
			return 10;
		case 'multiply':
			return 11;
		case 'screen':
			return 12;
		case 'overlay':
			return 13;
		case 'darken':
			return 14;
		case 'lighten':
			return 15;
		case 'color-dodge':
			return 16;
		case 'color-burn':
			return 17;
		case 'hard-light':
			return 18;
		case 'soft-light':
			return 19;
		case 'difference':
			return 20;
		case 'exclusion':
			return 21;
		case 'hue':
			return 22;
		case 'saturation':
			return 23;
		case 'color':
			return 24;
		case 'luminosity':
			return 25;
		default:
			null;
	}
}

function drawNativeImage(args: any[], image, context: any) {
	let dirty = false;
	const ptr = context.context.__getPointer();
	if (args.length === 3) {
		if (__ANDROID__) {
			dirty = (org as any).nativescript.canvas.NSCCanvasRenderingContext2D.drawImage(long(ptr), image, args[1], args[2]);
		}

		if (__IOS__) {
			dirty = NSCCanvasRenderingContext2D.drawImage(ptr, image, args[1], args[2]);
		}
	} else if (args.length === 5) {
		if (__ANDROID__) {
			dirty = (org as any).nativescript.canvas.NSCCanvasRenderingContext2D.drawImage(long(ptr), image, args[1], args[2], args[3], args[4]);
		}

		if (__IOS__) {
			dirty = NSCCanvasRenderingContext2D.drawImage(ptr, image, args[1], args[2], args[3], args[4]);
		}
	} else if (args.length === 9) {
		if (__ANDROID__) {
			dirty = (org as any).nativescript.canvas.NSCCanvasRenderingContext2D.drawImage(long(ptr), image, args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8]);
		}

		if (__IOS__) {
			dirty = NSCCanvasRenderingContext2D.drawImage(ptr, image, args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8]);
		}
	}

	if (dirty) {
		context.context.__makeDirty();
	}
}

export class CanvasRenderingContext2D {
	public static isDebug = true;
	private context;
	private contextPtr: any;
	_type: string;
	static {
		Helpers.initialize();
	}

	constructor(context: any, contextOptions) {
		this.contextPtr = context;
		const ctxPtr = BigInt(context.toString());
		this.context = global.CanvasModule.create2DContextWithPointer(ctxPtr);
		this._type = '2d';
	}

	get type() {
		return this._type;
	}

	get native() {
		return this.context;
	}

	_canvas: any;
	get canvas() {
		return this._canvas;
	}

	get shadowColor() {
		return this.context.shadowColor;
	}

	set shadowColor(color: string) {
		this.context.shadowColor = color;
	}

	get font(): string {
		return this.context.font;
	}

	set font(value: string) {
		this.context.font = value;
	}

	get direction(): string {
		return this.context.direction;
	}

	set direction(value: string) {
		this.context.direction = value;
	}

	get globalAlpha(): number {
		return this.context.globalAlpha;
	}

	set globalAlpha(alpha: number) {
		this.context.globalAlpha = alpha;
	}

	get imageSmoothingEnabled() {
		return this.context.imageSmoothingEnabled;
	}

	set imageSmoothingEnabled(enabled: boolean) {
		this.context.imageSmoothingEnabled = enabled;
	}

	get imageSmoothingQuality() {
		return this.context.imageSmoothingQuality;
	}

	set imageSmoothingQuality(quality: string) {
		this.context.imageSmoothingQuality = quality;
	}

	get lineDashOffset() {
		return this.context.lineDashOffset;
	}

	set lineDashOffset(offset: number) {
		this.context.lineDashOffset = offset;
	}

	get letterSpacing() {
		return this.context.letterSpacing;
	}

	set letterSpacing(spacing: string) {
		this.context.letterSpacing = spacing;
	}

	get lineJoin() {
		return this.context.lineJoin;
	}

	set lineJoin(join: string) {
		this.context.lineJoin = join;
	}

	get lineCap() {
		return this.context.lineCap;
	}

	set lineCap(cap: string) {
		this.context.lineCap = cap;
	}

	get miterLimit() {
		return this.context.miterLimit;
	}

	set miterLimit(limit: number) {
		this.context.miterLimit = limit;
	}

	get shadowBlur() {
		return this.context.shadowBlur;
	}

	set shadowBlur(blur: number) {
		this.context.shadowBlur = blur;
	}

	get shadowOffsetX() {
		return this.context.shadowOffsetX;
	}

	set shadowOffsetX(x: number) {
		this.context.shadowOffsetX = x;
	}

	get shadowOffsetY() {
		return this.context.shadowOffsetY;
	}

	set shadowOffsetY(y: number) {
		this.context.shadowOffsetY = y;
	}

	get textAlign() {
		return this.context.textAlign;
	}

	set textAlign(alignment: string) {
		this.context.textAlign = alignment;
	}

	get textBaseline() {
		return fromBaseLine(this.context.textBaseline);
	}

	set textBaseline(baseline: string) {
		const value = toBaseLine(baseline);
		if (value === -1) {
			return;
		}
		this.context.textBaseline = value;
	}

	get globalCompositeOperation() {
		return this.context.globalCompositeOperation;
	}

	set globalCompositeOperation(composite: string) {
		this.context.globalCompositeOperation = composite;
	}

	get fillStyle() {
		return this.context.fillStyle;
	}

	set fillStyle(color: CanvasGradient | CanvasPattern | string) {
		if (color === undefined || color === null) {
			return;
		}
		this.context.fillStyle = typeof color === 'object' ? color.native : color;
	}

	get filter(): string {
		return this.context.filter;
	}

	set filter(value: string) {
		this.context.filter = value;
	}

	get strokeStyle() {
		return this.context.strokeStyle;
	}

	set strokeStyle(color: string | CanvasGradient | CanvasPattern) {
		if (color === undefined || color === null) {
			return;
		}
		this.context.strokeStyle = typeof color === 'object' ? color.native : color;
	}

	get lineWidth() {
		return this.context.lineWidth;
	}

	set lineWidth(width: number) {
		this.context.lineWidth = width;
	}

	get wordSpacing() {
		return this.context.wordSpacing;
	}

	set wordSpacing(spacing: string) {
		this.context.wordSpacing = spacing;
	}

	__toDataURL(type: string = 'image/jpg', quality: number = 92) {
		return this.context.__toDataURL(type, quality);
	}

	addHitRegion(region: any): void {}

	arc(x: number, y: number, radius: number, startAngle: number, endAngle: number, anticlockwise: boolean = false): void {
		this.context.arc(x, y, radius, startAngle, endAngle, anticlockwise ?? false);
	}

	arcTo(x1: number, y1: number, x2: number, y2: number, radius: number): void {
		this.context.arcTo(x1, y1, x2, y2, radius);
	}

	beginPath(): void {
		this.context.beginPath();
	}

	bezierCurveTo(cp1x: number, cp1y: number, cp2x: number, cp2y: number, x: number, y: number): void {
		this.context.bezierCurveTo(cp1x, cp1y, cp2x, cp2y, x, y);
	}

	clearHitRegions(): void {}

	clearRect(x: number, y: number, width: number, height: number): void {
		this.context.clearRect(x, y, width, height);
	}

	clip(): void;

	clip(fillRule: string): void;

	clip(path: Path2D, fillRule: string): void;

	clip(...args: any): void {
		if (typeof args[0] === 'string') {
			this.context.clip(ruleToEnum(args[0]));
		} else if (args[0] instanceof Path2D && typeof args[1] === 'string') {
			this.context.clip(args[0].native, ruleToEnum(args[1]));
		} else if (args[0] instanceof Path2D) {
			this.context.clip(args[0].native);
		} else {
			this.context.clip();
		}
	}

	closePath(): void {
		this.context.closePath();
	}

	createImageData(width: number, height: number): ImageData;

	createImageData(data: ImageData): ImageData;

	createImageData(width: number | ImageData, height?: number): ImageData {
		if (width instanceof ImageData) {
			return new ImageData(width.width, width.height);
		} else {
			return new ImageData(width, height);
		}
	}

	createLinearGradient(x0: number, y0: number, x1: number, y1: number) {
		return CanvasGradient.fromNative(this.context.createLinearGradient(x0, y0, x1, y1));
	}

	createPattern(image: any, repetition: string): CanvasPattern | null {
		if (repetition === undefined || typeof repetition !== 'string') {
			const e = new Error('The string did not match the expected pattern.');
			e.name = 'SyntaxError';
			throw e;
		}

		//const createPattern = this._getMethod('createPattern');
		//const createPattern = this.context.createPattern;
		let img;

		if (image?._type === '2d' || image?._type?.indexOf('webgl') > -1) {
			img = (image as any).native;
		} else if (image instanceof ImageSource) {
			//const ptr = this.context.__getPointer; //this._getMethod('__getPointer');
			//const createPattern = this.context.__createPatternWithNative; //this._getMethod('__createPatternWithNative');
			if (__ANDROID__) {
				img = image.android;
				const pattern = (org as any).nativescript.canvas.NSCCanvasRenderingContext2D.createPattern(long(this.context.__getPointer()), img, repetition);
				return new CanvasPattern(this.context.__createPatternWithNative(pattern));
			}

			if (__IOS__) {
				img = image.ios;
				const pattern = NSCCanvasRenderingContext2D.createPattern(this.context.__getPointer(), img, repetition);
				return new CanvasPattern(this.context.__createPatternWithNative(pattern));
			}

			return null;
		} else if (__ANDROID__ && image instanceof android.graphics.Bitmap) {
			//const ptr = this.context.__getPointer; //this._getMethod('__getPointer');
			const createPattern = this.context.__createPatternWithNative; //this._getMethod('__createPatternWithNative');
			if (__ANDROID__) {
				const pattern = (org as any).nativescript.canvas.NSCCanvasRenderingContext2D.createPattern(long(this.context.__getPointer()), image, repetition);
				return new CanvasPattern(this.context.__createPatternWithNative(pattern));
			}

			if (__IOS__) {
				const pattern = NSCCanvasRenderingContext2D.createPattern(this.context.__getPointer(), image, repetition);
				return new CanvasPattern(this.context.__createPatternWithNative(pattern));
			}

			return null;
		} else if (image instanceof ImageAsset) {
			img = image.native;
		} else if (image instanceof Canvas) {
			img = (image as any).native;
		} else if (image && typeof image.tagName === 'string' && (image.tagName === 'IMG' || image.tagName === 'IMAGE')) {
			if (__ANDROID__ && image._image instanceof android.graphics.Bitmap) {
				//todo ios
				img = image._image;
			} else if (image._asset instanceof ImageAsset) {
				img = image._asset.native;
			} else if (typeof image.src === 'string') {
				//const ptr = this.context.__getPointer; //this._getMethod('__getPointer');
				//const createPattern = this.context.__createPatternWithNative; //this._getMethod('__createPatternWithNative');
				if (__ANDROID__) {
					img = ImageSource.fromFileSync(image.src).android;
					const pattern = (org as any).nativescript.canvas.NSCCanvasRenderingContext2D.createPattern(long(this.context.__getPointer()), img, repetition);
					return new CanvasPattern(this.context.__createPatternWithNative(pattern));
				}

				if (__IOS__) {
					img = ImageSource.fromFileSync(image.src).android;
					const pattern = NSCCanvasRenderingContext2D.createPattern(this.context.__getPointer(), img, repetition);
					return new CanvasPattern(this.context.__createPatternWithNative(pattern));
				}

				return null;
			}
		} else if (image && typeof image.tagName === 'string' && image.tagName === 'CANVAS' && image._canvas instanceof Canvas) {
			img = image._canvas.native;
		} else if (image instanceof ImageBitmap) {
			img = (image as any).native;
		}
		if (!img) {
			return null;
		}

		return new CanvasPattern(this.context.createPattern(img, repetition));
	}

	createRadialGradient(x0: number, y0: number, r0: number, x1: number, y1: number, r1: number) {
		return CanvasGradient.fromNative(this.context.createRadialGradient(x0, y0, r0, x1, y1, r1));
	}

	createConicGradient(startAngle: number, x: number, y: number) {
		return CanvasGradient.fromNative(this.context.createConicGradient(startAngle, x, y));
	}

	drawPaint(color: string) {
		this.context.drawPaint(color);
	}

	drawPoint(x: number, y: number) {
		this.context.drawPoint(x, y);
	}

	drawPoints(mode: 'points' | 'lines' | 'polygon', points: { x: number; y: number }[]) {
		let modeValue = -1;
		switch (mode) {
			case 'points':
				modeValue = 0;
				break;
			case 'lines':
				modeValue = 1;
				break;
			case 'polygon':
				modeValue = 2;
				break;
		}
		this.context.drawPoints(modeValue, points);
	}

	drawFocusIfNeeded(element): void;

	drawFocusIfNeeded(path, element): void;

	drawFocusIfNeeded(...args: any): void {}

	drawImage(image: any, dx: number, dy: number): void;

	drawImage(image: any, dx: number, dy: number, dWidth: number, dHeight: number): void;

	drawImage(image: any, sx: number, sy: number, sWidth: number, sHeight: number, dx: number, dy: number, dWidth: number, dHeight: number): void;

	drawImage(...args): void {
		//const drawImage = this.context.drawImage; //this._getMethod('drawImage');

		let image = args[0];
		if (image?._type === '2d' || image?._type?.indexOf('webgl') > -1) {
			image = (image as any).native;
		} else if (image instanceof ImageAsset) {
			image = image.native;
		} else if (image instanceof ImageSource) {
			if (__ANDROID__) {
				drawNativeImage(args, image.android, this);
			}
			if (__IOS__) {
				drawNativeImage(args, image.ios, this);
			}
			return;
		} else if (__ANDROID__ && image instanceof android.graphics.Bitmap) {
			drawNativeImage(args, image, this);
			return;
		} else if (__IOS__ && image instanceof UIImage) {
			drawNativeImage(args, image, this);
			return;
		} else if (image instanceof Canvas) {
			image = (image as any).native;
		} else if (image && typeof image.tagName === 'string' && (image.tagName === 'IMG' || image.tagName === 'IMAGE')) {
			if (image._imageSource instanceof ImageSource) {
				if (__ANDROID__) {
					drawNativeImage(args, image._imageSource.android, this);
				}
				if (__IOS__) {
					drawNativeImage(args, image._imageSource.ios, this);
				}
				return;
			} else if (__ANDROID__ && image._image instanceof android.graphics.Bitmap) {
				drawNativeImage(args, image._image, this);
				return;
			} else if (__ANDROID__ && image._svg) {
				const bitmap = image._svg?._svg?.getBitmap?.();
				if (bitmap) {
					drawNativeImage(args, bitmap, this);
				}
				return;
			} else if (__IOS__ && image._image instanceof UIImage) {
				drawNativeImage(args, image._image, this);
				return;
			} else if (image._asset instanceof ImageAsset) {
				image = image._asset.native;
			} else if (typeof image.src === 'string') {
				image = ImageSource.fromFileSync(image.src).android;
				if (__ANDROID__) {
					drawNativeImage(args, image.android, this);
				}
				if (__IOS__) {
					drawNativeImage(args, image.ios, this);
				}
				return;
			}
		} else if (image && typeof image.tagName === 'string' && image.tagName === 'CANVAS' && image._canvas instanceof Canvas) {
			image = image._canvas.native;
		} else if (image instanceof ImageBitmap) {
			image = (image as any).native;
		}

		if (args.length === 3) {
			this.context.drawImage(image, args[1], args[2]);
		} else if (args.length === 5) {
			this.context.drawImage(image, args[1], args[2], args[3], args[4]);
		} else if (args.length === 9) {
			this.context.drawImage(image, args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8]);
		}
	}

	drawAtlas(
		image: any,
		xform: {
			scos: number;
			ssin: number;
			tx: number;
			ty: number;
		}[],
		tex: { x: number; y: number; width: number; height: number }[],
		colors?: string[],
		blendMode: GlobalCompositeOperation = 'destination-over'
	): void {
		let isNativeSource = false;
		if (image?._type === '2d' || image?._type?.indexOf('webgl') > -1) {
			image = (image as any).native;
		} else if (image instanceof ImageAsset) {
			image = image.native;
		} else if (image instanceof ImageSource) {
			if (__ANDROID__) {
				isNativeSource = true;
				image = image.android;
			}
			if (__IOS__) {
				isNativeSource = true;
				image = image.ios;
			}
		} else if (__ANDROID__ && image instanceof android.graphics.Bitmap) {
			isNativeSource = true;
		} else if (__IOS__ && image instanceof UIImage) {
			isNativeSource = true;
		} else if (image instanceof Canvas) {
			image = (image as any).native;
		} else if (image && typeof image.tagName === 'string' && (image.tagName === 'IMG' || image.tagName === 'IMAGE')) {
			if (image._imageSource instanceof ImageSource) {
				if (__ANDROID__) {
					isNativeSource = true;
					image = image._imageSource.android;
				}
				if (__IOS__) {
					isNativeSource = true;
					image = image._imageSource.ios;
				}
				return;
			} else if (__ANDROID__ && image._image instanceof android.graphics.Bitmap) {
				image = image._image;
				isNativeSource = true;
			} else if (__IOS__ && image._image instanceof UIImage) {
				image = image._image;
				isNativeSource = true;
			} else if (image._asset instanceof ImageAsset) {
				image = image._asset.native;
			} else if (typeof image.src === 'string') {
				image = new ImageAsset();
				image.fromFileSync(image.src);
			}
		} else if (image && typeof image.tagName === 'string' && image.tagName === 'CANVAS' && image._canvas instanceof Canvas) {
			image = image._canvas.native;
		} else if (image instanceof ImageBitmap) {
			image = (image as any).native;
		}

		if (isNativeSource) {
			if (__IOS__) {
				return;
			}

			if (__ANDROID__) {
				const x = Array.create('float', xform.length * 4);
				let next = 0;
				for (let i = 0; i < xform.length; i++) {
					const item = xform[i];
					x[next] = item.scos;
					x[next + 1] = item.ssin;
					x[next + 1] = item.tx;
					x[next + 1] = item.ty;
					next += 3;
				}

				const t = Array.create('float', tex.length * 4);
				let tnext = 0;
				for (let i = 0; i < tex.length; i++) {
					const item = tex[i];
					t[tnext] = item.x;
					t[tnext + 1] = item.y;
					t[tnext + 1] = item.width;
					t[tnext + 1] = item.height;
					tnext += 3;
				}

				const c = Array.create('int', colors?.length ?? 0);

				if (colors?.length) {
					for (let i = 0; i < colors.length; i++) {
						const item = colors[i];
						c[i] = new Color(item).argb;
					}
				}

				(org as any).nativescript.canvas.NSCCanvasRenderingContext2D.drawAtlas(this.contextPtr, image, x, t, c, toBlendMode(blendMode ?? 'destination-over') ?? 4);
			}
		} else {
			if (image) {
				const x = xform.reduce((accumulator: number[], currentValue) => {
					accumulator.push(currentValue.scos, currentValue.ssin, currentValue.tx, currentValue.ty);
					return accumulator;
				}, []);

				const t = tex.reduce((accumulator: number[], currentValue) => {
					accumulator.push(currentValue.x, currentValue.y, currentValue.width, currentValue.height);
					return accumulator;
				}, []);

				this.context.drawAtlas(image, x, t, colors ?? null, toBlendMode(blendMode ?? 'destination-over') ?? 4);
			}
		}
	}

	ellipse(x: number, y: number, radiusX: number, radiusY: number, rotation: number, startAngle: number, endAngle: number, anticlockwise: boolean = false): void {
		this.context.ellipse(x, y, radiusX, radiusY, rotation, startAngle, endAngle, anticlockwise ?? false);
	}

	fill(): void;

	fill(fillRule: string): void;

	fill(path: Path2D, fillRule: string): void;

	fill(...args: any): void {
		if (typeof args[0] === 'string') {
			this.context.fill(ruleToEnum(args[0]));
		} else if (args[0] instanceof Path2D && typeof args[1] === 'string') {
			this.context.fill(args[0].native, ruleToEnum(args[1]));
		} else if (args[0] instanceof Path2D) {
			this.context.fill(args[0].native);
		} else {
			this.context.fill();
		}
	}

	fillRect(x: number, y: number, width: number, height: number): void {
		this.context.fillRect(x, y, width, height);
	}

	fillText(text: string, x: number, y: number, maxWidth: number): void {
		if (typeof maxWidth === 'number') {
			this.context.fillText(text + '', x, y, maxWidth);
		} else {
			this.context.fillText(text + '', x, y);
		}
	}

	fillOval(x: number, y: number, width: number, height: number): void {
		this.context.fillOval(x, y, width, height);
	}

	getImageData(sx: number, sy: number, sw: number, sh: number): ImageData {
		return ImageData.fromNative(this.context.getImageData(sx, sy, sw, sh));
	}

	getLineDash(): number[] {
		return this.context.getLineDash();
	}

	getTransform(): DOMMatrix {
		return new DOMMatrix(this.context.getTransform());
	}

	isPointInPath(x: number, y: number, fillRule: string): boolean;

	isPointInPath(path: Path2D, x: number, y: number, fillRule: string): boolean;

	isPointInPath(...args): boolean {
		const length = args.length;
		if (length === 2) {
			return this.context.isPointInPath(args[0], args[1]);
		} else if (length === 3) {
			return this.context.isPointInPath(args[0], args[1], ruleToEnum(args[2]));
		} else if (length === 4 && args[0] instanceof Path2D) {
			return this.context.isPointInPath(args[0].native, args[1], args[2], ruleToEnum(args[3]));
		}
		return false;
	}

	isPointInStroke(x: number, y: number): boolean;

	isPointInStroke(path: Path2D, x: number, y: number): boolean;

	isPointInStroke(...args): boolean {
		const length = args.length;
		if (length === 2) {
			return this.context.isPointInStroke(args[0], args[1]);
		} else if (length === 3 && args[0] instanceof Path2D) {
			return this.context.isPointInStroke(args[0].native, args[1], args[2]);
		}
		return false;
	}

	lineTo(x: number, y: number): void {
		this.context.lineTo(x, y);
	}

	measureText(text: string): TextMetrics {
		return new TextMetrics(this.context.measureText(text));
	}

	moveTo(x: number, y: number): void {
		this.context.moveTo(x, y);
	}

	putImageData(imageData: ImageData, dx: number, dy: number): void;

	putImageData(imageData: ImageData, dx: number, dy: number, dirtyX: number, dirtyY: number, dirtyWidth: number, dirtyHeight: number): void;

	putImageData(imageData: ImageData, dx: number, dy: number, dirtyX?: number, dirtyY?: number, dirtyWidth?: number, dirtyHeight?: number): void;

	putImageData(...args): void {
		if (!args) {
			return;
		}

		const data = args[0] as any;
		const length = args.length;
		if (length === 3) {
			this.context.putImageData(data.native, args[1], args[2]);
		} else if (length === 7) {
			this.context.putImageData(data.native, args[1], args[2], args[3], args[4], args[5], args[6]);
		}
	}

	quadraticCurveTo(cpx: number, cpy: number, x: number, y: number) {
		this.context.quadraticCurveTo(cpx, cpy, x, y);
	}

	rect(x: number, y: number, width: number, height: number): void {
		this.context.rect(x, y, width, height);
	}

	public roundRect(x: number, y: number, width: number, height: number, radii: number): void;
	public roundRect(x: number, y: number, width: number, height: number, radii: number[]): void;
	public roundRect(x: unknown, y: unknown, width: unknown, height: unknown, radii: unknown): void {
		this.context.roundRect(x, y, width, height, radii);
	}

	removeHitRegion(id: string): void {}

	resetTransform(): void {
		this.context.resetTransform();
	}

	restore(): void {
		this.context.restore();
	}

	rotate(angle: number): void {
		this.context.rotate(angle);
	}

	save(): void {
		this.context.save();
	}

	scale(x: number, y: number): void {
		this.context.scale(x, y);
	}

	scrollPathIntoView(): void;

	scrollPathIntoView(path: Path2D): void;

	scrollPathIntoView(path?: Path2D): void {}

	setLineDash(segments: number[]): void {
		this.context.setLineDash(segments);
	}

	setTransform(matrix: DOMMatrix): void;
	setTransform(a: number, b: number, c: number, d: number, e: number, f: number): void;
	setTransform(a: number | DOMMatrix, b?: number, c?: number, d?: number, e?: number, f?: number): void {
		if (typeof a === 'object') {
			// @ts-ignore
			this.context.setTransform(a.native);
		} else {
			this.context.setTransform(a, b, c, d, e, f);
		}
	}

	stroke(): void;

	stroke(path?: Path2D): void {
		if (path) {
			this.context.stroke(path.native);
		} else {
			this.context.stroke();
		}
	}

	strokeRect(x: number, y: number, width: number, height: number): void {
		this.context.strokeRect(x, y, width, height);
	}

	strokeText(text: string, x: number, y: number, maxWidth?: number): void {
		if (typeof maxWidth === 'number') {
			this.context.strokeText(text + '', x, y, maxWidth);
		} else {
			this.context.strokeText(text + '', x, y);
		}
	}

	strokeOval(x: number, y: number, width: number, height: number): void {
		this.context.strokeOval(x, y, width, height);
	}

	transform(a: number, b: number, c: number, d: number, e: number, f: number): void {
		this.context.transform(a, b, c, d, e, f);
	}

	translate(x: number, y: number): void {
		this.context.translate(x, y);
	}
}
