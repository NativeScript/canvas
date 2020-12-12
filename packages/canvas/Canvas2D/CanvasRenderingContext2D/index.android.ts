import {CanvasRenderingContext2DBase} from './common';
import {CanvasGradient} from '../CanvasGradient';
import {Path2D} from '../Path2D';
import {ImageData} from '../ImageData';
import {TextMetrics} from '../TextMetrics';
import {Color, ImageSource} from '@nativescript/core';
import {ImageAsset} from '../../ImageAsset';
import {CanvasPattern} from '../CanvasPattern';
import {Canvas} from '../../Canvas';

declare var com;

export class CanvasRenderingContext2D extends CanvasRenderingContext2DBase {
	public static isDebug = false;
	static colorCache = {};
	private context;

	constructor(context: any) {
		super();
		this.context = context;
	}

	_fillRuleFromString(string: string) {
		if (string === 'evenodd') {
			return com.github.triniwiz.canvas.TNSFillRule.EvenOdd;
		} else if (string === 'nonzero') {
			return com.github.triniwiz.canvas.TNSFillRule.NonZero;
		}
		return null;
	}

	get native() {
		return this.context;
	}

	get canvas() {
		return this._canvas;
	}

	private _shadowColor: string = 'transparent';

	get shadowColor() {
		this.log('shadowColor');
		return this.context.getShadowColor();
	}

	set shadowColor(color: string) {
		this.log('shadowColor value:', color);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			this.context.setShadowColor(color);
		}
	}

	get font(): string {
		this.log('get font');
		return this.context.getFont();
	}

	set font(value: string) {
		this.log('set font value:', value);
		if (this.context) {
			this.context.setFont(value);
		}
	}

	get direction(): string {
		this.log('direction');
		return this.context.getDirection() === com.github.triniwiz.canvas.TNSTextDirection.Ltr ? "ltr" : "rtl";
	}

	set direction(value: string) {
		this.log('direction value:', value);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			this.context.setDirection(
				value === "rtl" ? com.github.triniwiz.canvas.TNSTextDirection.Rtl : com.github.triniwiz.canvas.TNSTextDirection.Ltr
			);
		}
	}

	get globalAlpha(): number {
		this.log('globalAlpha');
		return this.context.getGlobalAlpha();
	}

	set globalAlpha(alpha: number) {
		this.log('globalAlpha value:', alpha);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			this.context.setGlobalAlpha(alpha);
		}
	}

	get imageSmoothingEnabled() {
		this.log('imageSmoothingEnabled');
		return this.context.getImageSmoothingEnabled();
	}

	set imageSmoothingEnabled(enabled: boolean) {
		this.log('imageSmoothingEnabled value:', enabled);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			this.context.setImageSmoothingEnabled(enabled);
		}
	}

	get imageSmoothingQuality() {
		this.log('imageSmoothingQuality');
		return this.context.getImageSmoothingQuality().toString();
	}

	set imageSmoothingQuality(quality: string) {
		this.log('imageSmoothingQuality value:', quality);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			switch (quality) {
				case 'high':
					this.context.setImageSmoothingQuality(com.github.triniwiz.canvas.TNSImageSmoothingQuality.High);
					break;
				case 'medium':
					this.context.setImageSmoothingQuality(com.github.triniwiz.canvas.TNSImageSmoothingQuality.Medium);
					break;
				default:
					this.context.setImageSmoothingQuality(com.github.triniwiz.canvas.TNSImageSmoothingQuality.Low);
					break;
			}
		}
	}

	get lineDashOffset() {
		this.log('lineDashOffset');
		return this.context.getLineDashOffset();
	}

	set lineDashOffset(offset: number) {
		this.log('lineDashOffset value:', offset);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			this.context.setLineDashOffset(offset);
		}
	}

	get lineJoin() {
		this.log('lineJoin');
		return this.context.getLineJoin().toString();
	}

	set lineJoin(join: string) {
		this.log('lineJoin value:', join);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			switch (join) {
				case 'bevel':
					this.context.setLineJoin(com.github.triniwiz.canvas.TNSLineJoin.Bevel);
					break;
				case 'round':
					this.context.setLineJoin(com.github.triniwiz.canvas.TNSLineJoin.Round);
					break;
				default:
					this.context.setLineJoin(com.github.triniwiz.canvas.TNSLineJoin.Miter);
					break;
			}
		}
	}

	get lineCap() {
		this.log('lineCap');
		return this.context.getLineCap().toString();
	}

	set lineCap(cap: string) {
		this.log('lineCap value:', cap);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			switch (cap) {
				case 'round':
					this.context.setLineCap(com.github.triniwiz.canvas.TNSLineCap.Round);
					break;
				case 'square':
					this.context.setLineCap(com.github.triniwiz.canvas.TNSLineCap.Square);
					break;
				default:
					this.context.setLineCap(com.github.triniwiz.canvas.TNSLineCap.Butt);
			}
		}
	}

	get miterLimit() {
		this.log('miterLimit value:');
		return this.context.getMiterLimit();
	}

	set miterLimit(limit: number) {
		this.log('miterLimit value:', limit);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			this.context.setMiterLimit(limit);
		}
	}

	get shadowBlur() {
		this.log('shadowBlur');
		return this.context.getShadowBlur();
	}

	set shadowBlur(blur: number) {
		this.log('shadowBlur value:', blur);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			this.context.setShadowBlur(blur);
		}
	}

	get shadowOffsetX() {
		this.log('shadowOffsetX');
		return this.context.getShadowOffsetX();
	}

	set shadowOffsetX(x: number) {
		this.log('shadowOffsetX value:', x);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			this.context.setShadowOffsetX(x);
		}
	}

	get shadowOffsetY() {
		this.log('shadowOffsetY');
		return this.context.getShadowOffsetY();
	}

	set shadowOffsetY(y: number) {
		this.log('shadowOffsetY value:', y);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			this.context.setShadowOffsetY(y);
		}
	}

	get textAlign() {
		this.log('textAlign');
		return this.context.getTextAlign().toString();
	}

	set textAlign(alignment: string) {
		this.log('textAlign value:', alignment);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			switch (alignment) {
				case 'start':
					this.context.setTextAlign(com.github.triniwiz.canvas.TNSTextAlignment.Start);
					break;
				case 'center':
					this.context.setTextAlign(com.github.triniwiz.canvas.TNSTextAlignment.Center);
					break;
				case 'end':
					this.context.setTextAlign(com.github.triniwiz.canvas.TNSTextAlignment.End);
					break;
				case 'right':
					this.context.setTextAlign(com.github.triniwiz.canvas.TNSTextAlignment.Right);
					break;
				default:
					this.context.setTextAlign(com.github.triniwiz.canvas.TNSTextAlignment.Left);
					break;
			}
		}
	}

	get globalCompositeOperation() {
		this.log('globalCompositeOperation');
		return this.context.getGlobalCompositeOperation().toString();
	}

	set globalCompositeOperation(composite: string) {
		this.log('globalCompositeOperation value:', composite);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			let operation;
			switch (composite.toLowerCase()) {
				case 'source-over':
					operation = com.github.triniwiz.canvas.TNSCompositeOperationType.SourceOver;
					break;
				case 'source-in':
					operation = com.github.triniwiz.canvas.TNSCompositeOperationType.SourceIn;
					break;
				case 'source-out':
					operation = com.github.triniwiz.canvas.TNSCompositeOperationType.SourceOut;
					break;
				case 'source-atop':
					operation = com.github.triniwiz.canvas.TNSCompositeOperationType.SourceAtop;
					break;
				case 'destination-over':
					operation = com.github.triniwiz.canvas.TNSCompositeOperationType.DestinationOver;
					break;
				case 'destination-in':
					operation = com.github.triniwiz.canvas.TNSCompositeOperationType.DestinationIn;
					break;
				case 'destination-out':
					operation = com.github.triniwiz.canvas.TNSCompositeOperationType.DestinationOut;
					break;
				case 'destination-atop':
					operation = com.github.triniwiz.canvas.TNSCompositeOperationType.DestinationAtop;
					break;
				case 'lighter':
					operation = com.github.triniwiz.canvas.TNSCompositeOperationType.Lighter;
					break;
				case 'copy':
					operation = com.github.triniwiz.canvas.TNSCompositeOperationType.Copy;
					break;
				case 'xor':
					operation = com.github.triniwiz.canvas.TNSCompositeOperationType.Xor;
					break;
				case 'multiply':
					operation = com.github.triniwiz.canvas.TNSCompositeOperationType.Multiply;
					break;
				case 'screen':
					operation = com.github.triniwiz.canvas.TNSCompositeOperationType.Screen;
					break;
				case 'overlay':
					operation = com.github.triniwiz.canvas.TNSCompositeOperationType.Overlay;
					break;
				case 'darken':
					operation = com.github.triniwiz.canvas.TNSCompositeOperationType.Darken;
					break;
				case 'lighten':
					operation = com.github.triniwiz.canvas.TNSCompositeOperationType.Lighten;
					break;
				case 'color-dodge':
					operation = com.github.triniwiz.canvas.TNSCompositeOperationType.ColorDodge;
					break;
				case 'color-burn':
					operation = com.github.triniwiz.canvas.TNSCompositeOperationType.ColorBurn;
					break;
				case 'hard-light':
					operation = com.github.triniwiz.canvas.TNSCompositeOperationType.HardLight;
					break;
				case 'soft-light':
					operation = com.github.triniwiz.canvas.TNSCompositeOperationType.SoftLight;
					break;
				case 'difference':
					operation = com.github.triniwiz.canvas.TNSCompositeOperationType.Difference;
					break;
				case 'exclusion':
					operation = com.github.triniwiz.canvas.TNSCompositeOperationType.Exclusion;
					break;
				case 'hue':
					operation = com.github.triniwiz.canvas.TNSCompositeOperationType.Hue;
					break;
				case 'saturation':
					operation = com.github.triniwiz.canvas.TNSCompositeOperationType.Saturation;
					break;
				case 'color':
					operation = com.github.triniwiz.canvas.TNSCompositeOperationType.Color;
					break;
				case 'luminosity':
					operation = com.github.triniwiz.canvas.TNSCompositeOperationType.Luminosity;
					break;
			}
			this.context.setGlobalCompositeOperation(operation);
		}
	}


	get fillStyle() {
		this.log('fillStyle');
		switch (this.context.getFillStyle().getStyleType()) {
			case CanvasColorStyleType.Color:
				const color = this.context.getFillStyle() as com.github.triniwiz.canvas.TNSColor;
				return color.getColor();
			case CanvasColorStyleType.Gradient:
				return CanvasGradient.fromNative(this.context.getFillStyle());
			case CanvasColorStyleType.Pattern:
				return new CanvasPattern(this.context.getFillStyle());
		}
	}

	set fillStyle(color: string | CanvasGradient | CanvasPattern) {
		this.log('fillStyle:', color);
		if (color === undefined || color === null) {
			return;
		}
		this._ensureLayoutBeforeDraw();
		if (typeof color === 'string') {
			this.context.setFillStyle(new com.github.triniwiz.canvas.TNSColor(color));
		} else if (color instanceof CanvasGradient) {
			this.context.setFillStyle(color.native);
		} else if (color instanceof CanvasPattern) {
			this.context.setFillStyle(color.native);
		}
	}

	get strokeStyle() {
		this.log('strokeStyle');
		switch (this.context.getStrokeStyle().getStyleType()) {
			case CanvasColorStyleType.Color:
				const color = this.context.getStrokeStyle() as com.github.triniwiz.canvas.TNSColor;
				return color.getColor();
			case CanvasColorStyleType.Gradient:
				return CanvasGradient.fromNative(this.context.getStrokeStyle());
			case CanvasColorStyleType.Pattern:
				return new CanvasPattern(this.context.getStrokeStyle());
		}
	}

	set strokeStyle(color: string | CanvasGradient | CanvasPattern) {
		this.log('strokeStyle:', color);
		if (color === undefined || color === null) {
			return;
		}
		this._ensureLayoutBeforeDraw();
		if (typeof color === 'string') {
			this.context.setStrokeStyle(new com.github.triniwiz.canvas.TNSColor(color));
		} else if (color instanceof CanvasGradient) {
			this.context.setStrokeStyle(color.native);
		} else if (color instanceof CanvasPattern) {
			this.context.setStrokeStyle(color.native);
		}
	}

	get lineWidth() {
		this.log('lineWidth');
		return this.context.getLineWidth();
	}

	set lineWidth(width: number) {
		this.log('lineWidth value:', width);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			this.context.setLineWidth(width);
		}
	}

	addHitRegion(region: any): void {
	}

	arc(
		x: number,
		y: number,
		radius: number,
		startAngle: number,
		endAngle: number,
		anticlockwise: boolean = false
	): void {
		this.log('arc value:', x, y, radius, startAngle, endAngle, anticlockwise);
		this._ensureLayoutBeforeDraw();
		this.context.arc(x, y, radius, startAngle, endAngle, anticlockwise);
	}

	arcTo(x1: number, y1: number, x2: number, y2: number, radius: number): void {
		this.log('arcTo value:', x1, y1, x2, y2, radius);
		this._ensureLayoutBeforeDraw();
		this.context.arcTo(x1, y1, x2, y2, radius);
	}

	beginPath(): void {
		this.log('beginPath');
		this._ensureLayoutBeforeDraw();
		this.context.beginPath();
	}

	bezierCurveTo(
		cp1x: number,
		cp1y: number,
		cp2x: number,
		cp2y: number,
		x: number,
		y: number
	): void {
		this.log('bezierCurveTo value:', cp1x, cp1y, cp2x, cp2y, x, y);
		this._ensureLayoutBeforeDraw();
		this.context.bezierCurveTo(cp1x, cp1y, cp2x, cp2y, x, y);
	}

	clearHitRegions(): void {
	}

	clearRect(x: number, y: number, width: number, height: number): void {
		this.log('clearRect value:', x, y, width, height);
		this._ensureLayoutBeforeDraw();
		this.context.clearRect(x, y, width, height);
	}

	clip(): void;

	clip(fillRule: string): void;

	clip(path: Path2D, fillRule: string): void;

	clip(...args: any): void {
		this.log('clip value:', ...args);
		this._ensureLayoutBeforeDraw();
		if (typeof args[0] === 'string') {
			this.context.clip(args[0]);
		} else if (args[0] instanceof Path2D && typeof args[1] === 'string') {
			this.context.clip(args[0].native, args[1]);
		} else if (args[0] instanceof Path2D) {
			this.context.clip(args[0].native);
		} else {
			this.context.clip();
		}
	}

	closePath(): void {
		this.log('closePath');
		this._ensureLayoutBeforeDraw();
		this.context.closePath();
	}

	createImageData(width: number, height: number): ImageData;

	createImageData(data: ImageData): ImageData;

	createImageData(width: number | ImageData, height?: number): ImageData {
		this.log('createImageData value:', arguments);
		if (width instanceof ImageData) {
			return ImageData.from(width);
		} else {
			return ImageData.fromNative(
				this.context.createImageData(width as any, height)
			);
		}
	}

	createLinearGradient(x0: number, y0: number, x1: number, y1: number) {
		this.log('createLinearGradient value:', x0, y0, x1, y1);
		this._ensureLayoutBeforeDraw();
		return CanvasGradient.fromNative(
			this.context.createLinearGradient(x0, y0, x1, y1)
		);
	}

	createPattern(image: any, repetition: string): CanvasPattern | null {
		this.log('createPattern value:', image, repetition);
		this._ensureLayoutBeforeDraw();
		if (repetition === undefined || typeof repetition !== 'string') {
			const e = new Error('The string did not match the expected pattern.');
			e.name = 'SyntaxError';
			throw e;
		}
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
		} else if (
			image &&
			typeof image.tagName === 'string' &&
			image.tagName === 'CANVAS' && image._canvas instanceof Canvas
		) {
			img = image._canvas.android;
		}
		if (!img) {
			return null;
		}

		let rep;
		switch (repetition) {
			case 'no-repeat':
				rep = com.github.triniwiz.canvas.TNSPatternRepetition.NoRepeat;
				break;
			case 'repeat-x':
				rep = com.github.triniwiz.canvas.TNSPatternRepetition.RepeatX;
				break;
			case 'repeat-y':
				rep = com.github.triniwiz.canvas.TNSPatternRepetition.RepeatY;
				break;
			default:
				rep = com.github.triniwiz.canvas.TNSPatternRepetition.Repeat;
				break;
		}
		return new CanvasPattern(this.context.createPattern(img, rep));
	}

	createRadialGradient(
		x0: number,
		y0: number,
		r0: number,
		x1: number,
		y1: number,
		r1: number
	) {
		this.log('createRadialGradient value:', x0, y0, r0, x1, y1, r1);
		this._ensureLayoutBeforeDraw();
		return CanvasGradient.fromNative(
			this.context.createRadialGradient(x0, y0, r0, x1, y1, r1)
		);
	}

	drawFocusIfNeeded(element): void;

	drawFocusIfNeeded(path, element): void;

	drawFocusIfNeeded(...args: any): void {
	}

	drawImage(image: any, dx: number, dy: number): void;

	drawImage(
		image: any,
		dx: number,
		dy: number,
		dWidth: number,
		dHeight: number
	): void;

	drawImage(
		image: any,
		sx: number,
		sy: number,
		sWidth: number,
		sHeight: number,
		dx: number,
		dy: number,
		dWidth: number,
		dHeight: number
	): void;

	drawImage(...args): void {
		this.log('drawImage value:', ...args);
		this._ensureLayoutBeforeDraw();
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
		} else if (
			image &&
			typeof image.tagName === 'string' &&
			image.tagName === 'CANVAS' && image._canvas instanceof Canvas
		) {
			image = image._canvas.android;
		}

		if (args.length === 3) {
			this.context.drawImage(image, args[1], args[2]);
		} else if (args.length === 5) {
			this.context.drawImage(image, args[1], args[2], args[3], args[4]);
		} else if (args.length === 9) {
			this.context.drawImage(
				image,
				args[1],
				args[2],
				args[3],
				args[4],
				args[5],
				args[6],
				args[7],
				args[8]
			);
		}
	}

	ellipse(
		x: number,
		y: number,
		radiusX: number,
		radiusY: number,
		rotation: number,
		startAngle: number,
		endAngle: number,
		anticlockwise: boolean = false
	): void {
		this.log('ellipse value:', x,
			y,
			radiusX,
			radiusY,
			rotation,
			startAngle,
			endAngle,
			anticlockwise);
		this._ensureLayoutBeforeDraw();
		this.context.ellipse(
			x,
			y,
			radiusX,
			radiusY,
			rotation,
			startAngle,
			endAngle,
			anticlockwise
		);
	}

	fill(): void;

	fill(fillRule: string): void;

	fill(path: Path2D, fillRule: string): void;

	fill(...args: any): void {
		this.log('fill value:', ...args);
		this._ensureLayoutBeforeDraw();
		if (typeof args[0] === 'string') {
			this.context.fill(args[0]);
		} else if (args[0] instanceof Path2D && typeof args[1] === 'string') {
			this.context.fill(args[0].native, args[1]);
		} else if (args[0] instanceof Path2D) {
			this.context.fill(args[0].native);
		} else {
			this.context.fill();
		}
	}

	fillRect(x: number, y: number, width: number, height: number): void {
		this.log('fillRect value:', x, y, width, height);
		this._ensureLayoutBeforeDraw();
		this.context.fillRect(x, y, width, height);
	}

	fillText(text: string, x: number, y: number, maxWidth: number): void {
		this.log('fillText value:', text, x, y, maxWidth);
		this._ensureLayoutBeforeDraw();
		if (typeof maxWidth === 'number') {
			this.context.fillText(text, x, y, maxWidth);
		} else {
			this.context.fillText(text, x, y);
		}
	}

	getImageData(sx: number, sy: number, sw: number, sh: number): ImageData {
		this.log('getImageData value:', sx, sy, sw, sh);
		this._ensureLayoutBeforeDraw();
		return ImageData.fromNative(this.context.getImageData(sx, sy, sw, sh));
	}

	getLineDash(): number[] {
		this.log('getLineDash');
		this._ensureLayoutBeforeDraw();
		return this.context.getLineDash() as any;
	}

	isPointInPath(x: number, y: number, fillRule: string): boolean;

	isPointInPath(
		path: Path2D,
		x: number,
		y: number,
		fillRule: string
	): boolean;

	isPointInPath(...args): boolean {
		this.log('isPointInPath value:', ...args);
		this._ensureLayoutBeforeDraw();
		if (args.length === 2) {
			return this.context.isPointInPath(args[0], args[1]);
		} else if (args.length === 3) {
			return this.context.isPointInPath(args[0], args[1], args[2]);
		} else if (args.length === 4 && args[0] instanceof Path2D) {
			return this.context.isPointInPath(args[0].native, args[1], args[2], args[3]);
		}
		return false;
	}

	isPointInStroke(x: number, y: number): boolean;

	isPointInStroke(path: Path2D, x: number, y: number): boolean;

	isPointInStroke(...args): boolean {
		this.log('isPointInStroke value:', ...args);
		this._ensureLayoutBeforeDraw();
		if (args.length === 2) {
			return this.context.isPointInStroke(args[0], args[1]);
		} else if (args.length === 3 && args[0] instanceof Path2D) {
			return this.context.isPointInStroke(args[0].native, args[1], args[2]);
		}
		return false;
	}

	lineTo(x: number, y: number): void {
		this.log('lineTo value:', x, y);
		this._ensureLayoutBeforeDraw();
		this.context.lineTo(x, y);
	}

	measureText(text: string): TextMetrics {
		this.log('measureText value:', text);
		this._ensureLayoutBeforeDraw();
		return new TextMetrics(this.context.measureText(text));
	}

	moveTo(x: number, y: number): void {
		this.log('moveTo value:', x, y);
		this._ensureLayoutBeforeDraw();
		this.context.moveTo(x, y);
	}

	putImageData(imageData: ImageData, dx: number, dy: number): void;

	putImageData(
		imageData: ImageData,
		dx: number,
		dy: number,
		dirtyX: number,
		dirtyY: number,
		dirtyWidth: number,
		dirtyHeight: number
	): void;

	putImageData(
		imageData: ImageData,
		dx: number,
		dy: number,
		dirtyX?: number,
		dirtyY?: number,
		dirtyWidth?: number,
		dirtyHeight?: number
	): void;

	putImageData(...args): void {
		this.log('putImageData value:', ...args);
		this._ensureLayoutBeforeDraw();
		if (!args) {
			return;
		}
		let data = args[0] as any;
		// Would be nice not to update the native backing array each time ... but since buffers don't use pointers ... we are here 😅
		if (args.length === 3) {
			//data.native.updateData(Array.from(data.data));
			this.context.putImageData(data.native, args[1], args[2]);
		} else if (args.length === 7) {
			// data.native.updateData(Array.from(data.data));
			this.context.putImageData(data.native, args[1], args[2], args[3], args[4], args[5], args[6]);
		}
	}

	quadraticCurveTo(cpx: number, cpy: number, x: number, y: number) {
		this.log('quadraticCurveTo value:', cpx, cpy, x, y);
		this._ensureLayoutBeforeDraw();
		this.context.quadraticCurveTo(cpx, cpy, x, y);
	}

	rect(x: number, y: number, width: number, height: number): void {
		this.log('rect value:', x, y, width, height);
		this._ensureLayoutBeforeDraw();
		this.context.rect(x, y, width, height);
	}

	removeHitRegion(id: string): void {
	}

	resetTransform(): void {
		this.log('resetTransform value:');
		this._ensureLayoutBeforeDraw();
		this.context.resetTransform();
	}

	restore(): void {
		this.log('restore');
		this._ensureLayoutBeforeDraw();
		this.context.restore();
	}

	rotate(angle: number): void {
		this.log('rotate value:', angle);
		this._ensureLayoutBeforeDraw();
		this.context.rotate(angle);
	}

	save(): void {
		this.log('save');
		this._ensureLayoutBeforeDraw();
		this.context.save();
	}

	scale(x: number, y: number): void {
		this.log('scale value:', x, y);
		this._ensureLayoutBeforeDraw();
		this.context.scale(x, y);
	}

	scrollPathIntoView(): void;

	scrollPathIntoView(path: Path2D): void;

	scrollPathIntoView(path?: Path2D): void {
	}

	setLineDash(segments: number[]): void {
		this.log('setLineDash value:', segments);
		this._ensureLayoutBeforeDraw();
		this.context.setLineDash(segments);
	}

	setTransform(
		a: number,
		b: number,
		c: number,
		d: number,
		e: number,
		f: number
	): void {
		this.log('setTransform value:', a, b, c, d, e, f);
		this._ensureLayoutBeforeDraw();
		this.context.setTransform(a, b, c, d, e, f);
	}

	stroke(): void;

	stroke(path?: Path2D): void {
		this.log('stroke value:', path);
		this._ensureLayoutBeforeDraw();
		if (path) {
			this.context.stroke(path.native);
		} else {
			this.context.stroke();
		}
	}

	strokeRect(x: number, y: number, width: number, height: number): void {
		this.log('strokeRect value:', x, y, width, height);
		this._ensureLayoutBeforeDraw();
		this.context.strokeRect(x, y, width, height);
	}

	strokeText(text: string, x: number, y: number, maxWidth?: number): void {
		this.log('strokeText value:', text, x, y, maxWidth);
		this._ensureLayoutBeforeDraw();
		if (typeof maxWidth === 'number') {
			this.context.strokeText(text, x, y, maxWidth);
		} else {
			this.context.strokeText(text, x, y);
		}
	}

	transform(
		a: number,
		b: number,
		c: number,
		d: number,
		e: number,
		f: number
	): void {
		this.log('transform value:', a, b, c, d, e, f);
		this._ensureLayoutBeforeDraw();
		this.context.transform(a, b, c, d, e, f);
	}

	translate(x: number, y: number): void {
		this.log('translate value:', x, y);
		this._ensureLayoutBeforeDraw();
		this.context.translate(x, y);
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