import {CanvasRenderingContext2DBase} from './common';
import {CanvasGradient} from '../CanvasGradient';
import {Path2D} from '../Path2D';
import {ImageData} from '../ImageData';
import {TextMetrics} from '../TextMetrics';
import {ImageSource} from '@nativescript/core';
import {ImageAsset} from '../../ImageAsset';
import {CanvasPattern} from '../CanvasPattern';
import {Canvas} from '../../Canvas';

declare let TNSImageAsset, TNSCanvas, TNSFillRule;

export class CanvasRenderingContext2D extends CanvasRenderingContext2DBase {
	public static isDebug = false;
	private context;

	constructor(context: any) {
		super();
		this.context = context;
	}

	_fillRuleFromString(string: string) {
		if (string === 'evenodd') {
			return TNSFillRule.EvenOdd;
		} else if (string === 'nonzero') {
			return TNSFillRule.NonZero;
		}
		return null;
	}

	get native() {
		return this.context;
	}

	get canvas() {
		return this._canvas;
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

	get shadowColor() {
		this.log('get shadowColor');
		return this.context.shadowColor;
	}

	set shadowColor(color: any) {
		this.log('set shadowColor', color);
		this._ensureLayoutBeforeDraw();
		if (this.context && typeof color === 'string') {
			this.context.shadowColor = color;
		}
	}

	get globalAlpha(): number {
		this.log('get globalAlpha');
		return this.context.globalAlpha;
	}

	set globalAlpha(alpha: number) {
		this.log('set globalAlpha', alpha);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			this.context.globalAlpha = alpha;
		}
	}

	get imageSmoothingEnabled() {
		this.log('get imageSmoothingEnabled');
		return this.context.imageSmoothingEnabled;
	}

	set imageSmoothingEnabled(enabled: boolean) {
		this.log('set imageSmoothingEnabled', enabled);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			this.context.imageSmoothingEnabled = enabled;
		}
	}

	get imageSmoothingQuality() {
		this.log('get imageSmoothingQuality');
		switch (this.context.imageSmoothingQuality) {
			case TNSImageSmoothingQuality.High:
				return 'high';
			case TNSImageSmoothingQuality.Medium:
				return 'medium';
			default:
				return 'low';
		}
	}

	set imageSmoothingQuality(quality: string) {
		this.log('set imageSmoothingQuality', quality);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			switch (quality) {
				case 'high':
					this.context.imageSmoothingQuality = TNSImageSmoothingQuality.High;
					break;
				case 'medium':
					this.context.imageSmoothingQuality = TNSImageSmoothingQuality.Medium;
					break;
				case 'low':
					this.context.imageSmoothingQuality = TNSImageSmoothingQuality.Low;
					break;
			}
		}
	}

	get lineDashOffset() {
		this.log('get lineDashOffset');
		return this.context.lineDashOffset;
	}

	set lineDashOffset(offset: number) {
		this.log('set lineDashOffset', offset);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			this.context.lineDashOffset = offset;
		}
	}

	get lineJoin() {
		this.log('get lineJoin');
		switch (this.context.lineJoin) {
			case TNSLineJoin.Bevel:
				return 'bevel';
			case TNSLineJoin.Round:
				return 'round';
			default:
				return 'miter';
		}
	}

	set lineJoin(join: string) {
		this.log('set lineJoin', join);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			switch (join) {
				case 'bevel':
					this.context.lineJoin = TNSLineJoin.Bevel;
					break;
				case 'round':
					this.context.lineJoin = TNSLineJoin.Round;
					break;
				case 'miter':
					this.context.lineJoin = TNSLineJoin.Miter;
					break;
			}
		}
	}

	get lineCap() {
		this.log('get lineCap');
		switch (this.context.lineCap) {
			case TNSLineCap.Round:
				return 'round';
			case TNSLineCap.Square:
				return 'square';
			default:
				return 'butt';
		}
	}

	set lineCap(cap: string) {
		this.log('set lineCap', cap);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			switch (cap) {
				case 'round':
					this.context.lineCap = TNSLineCap.Round;
					break;
				case 'square':
					this.context.lineCap = TNSLineCap.Square;
					break;
				case 'butt':
					this.context.lineCap = TNSLineCap.Butt;
			}
		}
	}

	get miterLimit() {
		this.log('get miterLimit');
		return this.context.miterLimit;
	}

	set miterLimit(limit: number) {
		this.log('set miterLimit', limit);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			this.context.miterLimit = limit;
		}
	}

	get shadowBlur() {
		this.log('get shadowBlur');
		return this.context.shadowBlur;
	}

	set shadowBlur(blur: number) {
		this.log('set shadowBlur', blur);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			this.context.shadowBlur = blur;
		}
	}

	get shadowOffsetX() {
		this.log('get shadowOffsetX');
		return this.context.shadowOffsetX;
	}

	set shadowOffsetX(x: number) {
		this.log('set shadowOffsetX', x);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			this.context.shadowOffsetX = x;
		}
	}

	get shadowOffsetY() {
		this.log('get shadowOffsetY');
		return this.context.shadowOffsetY;
	}

	set shadowOffsetY(y: number) {
		this.log('set shadowOffsetY', y);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			this.context.shadowOffsetY = y;
		}
	}

	get textAlign() {
		this.log('get textAlign');
		switch (this.context.textAlign) {
			case TNSTextAlignment.Start:
				return 'start';
			case TNSTextAlignment.Center:
				return 'center';
			case TNSTextAlignment.End:
				return 'end';
			case TNSTextAlignment.Right:
				return 'right';
			default:
				return 'left';
		}
	}

	set textAlign(alignment: string) {
		this.log('set textAlign', alignment);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			switch (alignment) {
				case 'start':
					this.context.textAlign = TNSTextAlignment.Start;
					break;
				case 'center':
					this.context.textAlign = TNSTextAlignment.Center;
					break;
				case 'end':
					this.context.textAlign = TNSTextAlignment.End;
					break;
				case 'right':
					this.context.textAlign = TNSTextAlignment.Right;
					break;
				case 'left':
					this.context.textAlign = TNSTextAlignment.Left;
					break;
			}
		}
	}

	get globalCompositeOperation() {
		this.log('get globalCompositeOperation');
		switch (this.context.globalCompositeOperation) {
			case TNSCompositeOperationType.SourceOver:
				return 'source-over';
			case TNSCompositeOperationType.SourceIn:
				return 'source-in';
			case TNSCompositeOperationType.SourceOut:
				return 'source-out';
			case TNSCompositeOperationType.SourceAtop:
				return 'source-atop';
			case TNSCompositeOperationType.DestinationOver:
				return 'destination-over';
			case TNSCompositeOperationType.DestinationIn:
				return 'destination-in';
			case TNSCompositeOperationType.DestinationOut:
				return 'destination-out';
			case TNSCompositeOperationType.DestinationAtop:
				return 'destination-atop';
			case TNSCompositeOperationType.Lighter:
				return 'lighter';
			case TNSCompositeOperationType.Copy:
				return 'copy';
			case TNSCompositeOperationType.Xor:
				return 'xor';
			case TNSCompositeOperationType.Multiply:
				return 'multiply';
			case TNSCompositeOperationType.Screen:
				return 'screen';
			case TNSCompositeOperationType.Overlay:
				return 'overlay';
			case TNSCompositeOperationType.Darken:
				return 'darken';
			case TNSCompositeOperationType.Lighten:
				return 'lighten';
			case TNSCompositeOperationType.ColorDodge:
				return 'color-dodge';
			case TNSCompositeOperationType.ColorBurn:
				return 'color-burn';
			case TNSCompositeOperationType.HardLight:
				return 'hard-light';
			case TNSCompositeOperationType.SoftLight:
				return 'soft-light';
			case TNSCompositeOperationType.Difference:
				return 'difference';
			case TNSCompositeOperationType.Exclusion:
				return 'exclusion';
			case TNSCompositeOperationType.Hue:
				return 'hue';
			case TNSCompositeOperationType.Saturation:
				return 'saturation';
			case TNSCompositeOperationType.Color:
				return 'color';
			case TNSCompositeOperationType.Luminosity:
				return 'luminosity';
		}
	}

	set globalCompositeOperation(composite: string) {
		this.log('set globalCompositeOperation', composite);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			switch (composite.toLowerCase()) {
				case 'source-over':
					this.context.globalCompositeOperation = TNSCompositeOperationType.SourceOver;
					break;
				case 'source-in':
					this.context.globalCompositeOperation = TNSCompositeOperationType.SourceIn;
					break;
				case 'source-out':
					this.context.globalCompositeOperation = TNSCompositeOperationType.SourceOut;
					break;
				case 'source-atop':
					this.context.globalCompositeOperation = TNSCompositeOperationType.SourceAtop;
					break;
				case 'destination-over':
					this.context.globalCompositeOperation = TNSCompositeOperationType.DestinationOver;
					break;
				case 'destination-in':
					this.context.globalCompositeOperation = TNSCompositeOperationType.DestinationIn;
					break;
				case 'destination-out':
					this.context.globalCompositeOperation = TNSCompositeOperationType.DestinationOut;
					break;
				case 'destination-atop':
					this.context.globalCompositeOperation = TNSCompositeOperationType.DestinationAtop;
					break;
				case 'lighter':
					this.context.globalCompositeOperation = TNSCompositeOperationType.Lighter;
					break;
				case 'copy':
					this.context.globalCompositeOperation = TNSCompositeOperationType.Copy;
					break;
				case 'xor':
					this.context.globalCompositeOperation = TNSCompositeOperationType.Xor;
					break;
				case 'multiply':
					this.context.globalCompositeOperation = TNSCompositeOperationType.Multiply;
					break;
				case 'screen':
					this.context.globalCompositeOperation = TNSCompositeOperationType.Screen;
					break;
				case 'overlay':
					this.context.globalCompositeOperation = TNSCompositeOperationType.Overlay;
					break;
				case 'darken':
					this.context.globalCompositeOperation = TNSCompositeOperationType.Darken;
					break;
				case 'lighten':
					this.context.globalCompositeOperation = TNSCompositeOperationType.Lighten;
					break;
				case 'color-dodge':
					this.context.globalCompositeOperation = TNSCompositeOperationType.ColorDodge;
					break;
				case 'color-burn':
					this.context.globalCompositeOperation = TNSCompositeOperationType.ColorBurn;
					break;
				case 'hard-light':
					this.context.globalCompositeOperation = TNSCompositeOperationType.HardLight;
					break;
				case 'soft-light':
					this.context.globalCompositeOperation = TNSCompositeOperationType.SoftLight;
					break;
				case 'difference':
					this.context.globalCompositeOperation = TNSCompositeOperationType.Difference;
					break;
				case 'exclusion':
					this.context.globalCompositeOperation = TNSCompositeOperationType.Exclusion;
					break;
				case 'hue':
					this.context.globalCompositeOperation = TNSCompositeOperationType.Hue;
					break;
				case 'saturation':
					this.context.globalCompositeOperation = TNSCompositeOperationType.Saturation;
					break;
				case 'color':
					this.context.globalCompositeOperation = TNSCompositeOperationType.Color;
					break;
				case 'luminosity':
					this.context.globalCompositeOperation = TNSCompositeOperationType.Luminosity;
					break;
			}
		}
	}

	get filter(): string {
		this.log('get filter');
		return this.context.filter;
	}

	set filter(value: string) {
		this.log('set filter', value);
		if (this.context) {
			this.context.filter = value;
		}
	}


	get fillStyle() {
		this.log('get fillStyle');
		switch (this.context.fillStyle.getStyleType()) {
			case CanvasColorStyleType.Color:
				const color = this.context.fillStyle as TNSColor;
				return color.color;
			case CanvasColorStyleType.Gradient:
				return CanvasGradient.fromNative(this.context.fillStyle);
			case CanvasColorStyleType.Pattern:
				return new CanvasPattern(this.context.fillStyle);
		}
	}

	set fillStyle(color: string | CanvasGradient | CanvasPattern) {
		this.log('set fillStyle', color);
		if (color === undefined || color === null) {
			return;
		}
		this._ensureLayoutBeforeDraw();
		if (typeof color === 'string') {
			this.context.fillStyle = TNSColor.alloc().init(color);
		} else if (color instanceof CanvasGradient) {
			this.context.fillStyle = color.native;
		} else if (color instanceof CanvasPattern) {
			this.context.fillStyle = color.native;
		}
	}


	get strokeStyle() {
		this.log('get strokeStyle');
		switch (this.context.strokeStyle.getStyleType()) {
			case CanvasColorStyleType.Color:
				const color = this.context.strokeStyle as TNSColor;
				return color.color;
			case CanvasColorStyleType.Gradient:
				return CanvasGradient.fromNative(this.context.strokeStyle);
			case CanvasColorStyleType.Pattern:
				return new CanvasPattern(this.context.strokeStyle);
		}
	}

	set strokeStyle(color: string | CanvasGradient | CanvasPattern) {
		this.log('set strokeStyle', color);
		if (color === undefined || color === null) {
			return;
		}
		this._ensureLayoutBeforeDraw();
		if (typeof color === 'string') {
			this.context.strokeStyle = TNSColor.alloc().init(color);
		} else if (color instanceof CanvasGradient) {
			this.context.strokeStyle = color.native;
		} else if (color instanceof CanvasPattern) {
			this.context.strokeStyle = color.native;
		}
	}

	get lineWidth() {
		this.log('get lineWidth');
		return this.context.lineWidth;
	}

	set lineWidth(width: number) {
		this.log('set lineWidth', width);
		this._ensureLayoutBeforeDraw();
		if (this.context) {
			this.context.lineWidth = width;
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
		this.log('arc', x, y, radius, startAngle, anticlockwise);
		this._ensureLayoutBeforeDraw();
		this.context.arc(
			x,
			y,
			radius,
			startAngle,
			endAngle,
			anticlockwise
		);
	}

	arcTo(x1: number, y1: number, x2: number, y2: number, radius: number): void {
		this.log('arcTo', x1, y1, x2, y2, radius);
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
		this.log('bezierCurveTo', cp1x, cp1y, cp2x, cp2y, x, y);
		this._ensureLayoutBeforeDraw();
		this.context.bezierCurveTo(cp1x, cp1y, cp2x, cp2y, x, y);
	}

	clearHitRegions(): void {
	}

	clearRect(x: number, y: number, width: number, height: number): void {
		this.log('clearRect', x, y, width, height);
		this._ensureLayoutBeforeDraw();
		this.context.clearRect(x, y, width, height);
	}

	clip(): void;

	clip(fillRule: string): void;

	clip(path: any, fillRule: string): void;

	clip(...args): void {
		this.log('clip', ...args);
		this._ensureLayoutBeforeDraw();
		if (typeof args[0] === 'string') {
			// browser throws for invalid enum
			const rule = this._fillRuleFromString(args[0]);
			console.log(rule);
			this.context.clip(rule);
		} else if (args[0] instanceof Path2D && typeof args[1] === 'string') {
			const rule = this._fillRuleFromString(args[1]);
			this.context.clip(args[0].native, rule);
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
		this.log('createImageData', width, height);
		if (width instanceof ImageData) {
			return ImageData.fromNative(
				this.context.createImageData(width.native)
			);
		} else {
			return ImageData.fromNative(
				this.context.createImageData(width, height)
			);
		}
	}

	createLinearGradient(x0: number, y0: number, x1: number, y1: number) {
		this.log('createLinearGradient', x0, y0, x1, y1);
		this._ensureLayoutBeforeDraw();
		return CanvasGradient.fromNative(
			this.context.createLinearGradient(x0, y0, x1, y1)
		);
	}

	createPattern(image: any, repetition: string): CanvasPattern | null {
		this.log('createPattern', image, repetition);
		this._ensureLayoutBeforeDraw();
		if (repetition === undefined || typeof repetition !== 'string') {
			const e = new Error('The string did not match the expected pattern.');
			e.name = 'SyntaxError';
			throw e;
		}
		let img;
		if (image instanceof ImageSource) {
			img = image.ios;
		} else if (image instanceof UIImage) {
			img = image;
		} else if (image instanceof ImageAsset) {
			img = image.native;
		} else if (image instanceof Canvas) {
			img = image.ios;
		} else if (image && typeof image.tagName === 'string' && (image.tagName === 'IMG' || image.tagName === 'IMAGE')) {
			if (image._imageSource instanceof ImageSource) {
				img = image._imageSource.android;
			} else if (image._image instanceof UIImage) {
				img = image._image;
			} else if (image._asset instanceof ImageAsset) {
				img = image._asset.native;
			} else if (typeof image.src === 'string') {
				img = ImageSource.fromFileSync(image.src).ios;
			}
		} else if (
			image &&
			typeof image.tagName === 'string' &&
			image.tagName === 'CANVAS'
		) {
			if (image._canvas instanceof Canvas) {
				img = image._canvas.ios;
			}
		}

		if (!img) {
			return null;
		}

		let rep;
		switch (repetition) {
			case 'no-repeat':
				rep = TNSPatternRepetition.NoRepeat;
				break;
			case 'repeat-x':
				rep = TNSPatternRepetition.RepeatX;
				break;
			case 'repeat-y':
				rep = TNSPatternRepetition.RepeatY;
				break;
			default:
				rep = TNSPatternRepetition.Repeat;
				break;
		}

		if (img instanceof TNSImageAsset) {
			return new CanvasPattern(this.context.createPattern(img, rep));
		} else if (img instanceof UIImage) {
			return new CanvasPattern(this.context.createPattern(img, rep));
		} else if (img instanceof TNSCanvas) {
			return new CanvasPattern(this.context.createPattern(img, rep));
		}
		return null;
	}

	createRadialGradient(
		x0: number,
		y0: number,
		r0: number,
		x1: number,
		y1: number,
		r1: number
	) {
		this.log('createRadialGradient', x0, y0, r0, x1, y1, r1);
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
		this.log('drawImage', ...args);
		this._ensureLayoutBeforeDraw();
		if (!args) {
			return;
		}
		let image = args[0];
		let asset = null;
		let width = 0;
		let height = 0;
		if (image instanceof ImageSource) {
			image = image.ios;
		} else if (image instanceof ImageAsset || image instanceof UIImage) {
			// NOOP
		} else if (
			image &&
			typeof image.tagName === 'string' &&
			(image.tagName === 'IMG' || image.tagName === 'IMAGE')
		) {
			width = image.width;
			height = image.height;
			if (image._imageSource instanceof ImageSource) {
				image = image._imageSource.ios;
			} else if (image._image instanceof UIImage) {
				image = image._image;
			} else if (image._asset instanceof ImageAsset) {
				image = image._asset;
			} else if (typeof image.src === 'string') {
				image = ImageSource.fromFileSync(image.src).ios;
			}
		} else if (
			image &&
			typeof image.tagName === 'string' &&
			image.tagName === 'CANVAS' && image._canvas instanceof Canvas
		) {
			image = image._canvas;
		}

		if (args.length === 3) {
			if (image instanceof ImageAsset) {
				this.context.drawImage(image.native, args[1], args[2]);
			} else if (image instanceof Canvas) {
				this.context.drawImage(image.ios, args[1], args[2]);
			} else {
				this.context.drawImage(image, args[1], args[2]);
			}

		} else if (args.length === 5) {
			if (image instanceof ImageAsset) {
				this.context.drawImage(
					image.native,
					args[1],
					args[2],
					args[3],
					args[4]
				);
			} else if (image instanceof Canvas) {
				this.context.drawImage(image.ios, args[1],
					args[2],
					args[3],
					args[4]);
			} else {
				this.context.drawImage(
					image,
					args[1],
					args[2],
					args[3],
					args[4]
				);
			}
		} else if (args.length === 9) {
			if (image instanceof ImageAsset) {
				this.context.drawImage(
					image.native,
					args[1],
					args[2],
					args[3],
					args[4],
					args[5],
					args[6],
					args[7],
					args[8]
				);
			} else if (image instanceof Canvas) {
				this.context.drawImage(image.ios, args[1],
					args[2],
					args[3],
					args[4],
					args[5],
					args[6],
					args[7],
					args[8]);
			} else {
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
		this.log('ellipse', x,
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
		this.log('fill', ...args);
		this._ensureLayoutBeforeDraw();
		if (typeof args[0] === 'string') {
			const rule = this._fillRuleFromString(args[0]);
			this.context.fillWithValue(rule);
		} else if (args.length === 2 && args[0] instanceof Path2D && typeof args[1] === 'string') {
			const rule = this._fillRuleFromString(args[1]);
			this.context.fill(args[0].native, rule);
		} else if (args[0] instanceof Path2D) {
			this.context.fillWithValue(args[0].native);
		} else {
			this.context.fill();
		}
	}

	fillRect(x: number, y: number, width: number, height: number): void {
		this.log('fillRect', x, y, width, height);
		this._ensureLayoutBeforeDraw();
		this.context.fillRect(x, y, width, height);
	}

	fillText(text: string, x: number, y: number, maxWidth?: number): void {
		this.log('fillText', text, x, y, maxWidth);
		this._ensureLayoutBeforeDraw();
		if (typeof maxWidth === 'number') {
			this.context.fillText(text, x, y, maxWidth);
		} else {
			this.context.fillText(text, x, y);
		}
	}

	getImageData(sx: number, sy: number, sw: number, sh: number): ImageData {
		this.log('getImageData', sx, sy, sw, sh);
		this._ensureLayoutBeforeDraw();
		return ImageData.fromNative(this.context.getImageData(sx, sy, sw, sh));
	}

	getLineDash() {
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
		this.log('isPointInPath', ...args);
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
		this.log('isPointInStroke', ...args);
		this._ensureLayoutBeforeDraw();
		if (args.length === 2) {
			return this.context.isPointInStroke(args[0], args[1]);
		} else if (args.length === 3 && args[0] instanceof Path2D) {
			return this.context.isPointInStroke(args[0].native, args[1], args[2]);
		}
		return false;
	}

	lineTo(x: number, y: number): void {
		this.log('lineTo', x, y);
		this._ensureLayoutBeforeDraw();
		this.context.lineTo(x, y);
	}

	measureText(text: string): TextMetrics {
		this.log('measureText', text);
		this._ensureLayoutBeforeDraw();
		return new TextMetrics(this.context.measureText(text));
	}

	moveTo(x: number, y: number): void {
		this.log('moveTo', x, y);
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
		this.log('putImageData', ...args);
		this._ensureLayoutBeforeDraw();
		if (!args) {
			return;
		}
		let data = args[0] as any;
		if (args.length === 3) {
			this.context.putImageData(data.native, args[1], args[2]);
		} else if (args.length === 7) {
			this.context.putImageData(
				data.native,
				args[1],
				args[2],
				args[3],
				args[4],
				args[5],
				args[6]
			);
		}
	}

	quadraticCurveTo(cpx: number, cpy: number, x: number, y: number) {
		this.log('quadraticCurveTo', cpx, cpy, x, y);
		this._ensureLayoutBeforeDraw();
		this.context.quadraticCurveTo(cpx, cpy, x, y);
	}

	rect(x: number, y: number, width: number, height: number): void {
		this.log('rect', x, y, width, height);
		this._ensureLayoutBeforeDraw();
		this.context.rect(x, y, width, height);
	}

	removeHitRegion(id: string): void {
	}

	resetTransform(): void {
		this.log('resetTransform');
		this._ensureLayoutBeforeDraw();
		this.context.resetTransform();
	}

	restore(): void {
		this.log('restore');
		this._ensureLayoutBeforeDraw();
		this.context.restore();
	}

	rotate(angle: number): void {
		this.log('rotate', angle);
		this._ensureLayoutBeforeDraw();
		this.context.rotate(angle);
	}

	save(): void {
		this.log('save');
		this._ensureLayoutBeforeDraw();
		this.context.save();
	}

	scale(x: number, y: number): void {
		this.log('scale', x, y);
		this._ensureLayoutBeforeDraw();
		if (typeof x === 'number' && typeof y === 'number') {
			this.context.scale(x, y);
		}
	}

	scrollPathIntoView(): void;

	scrollPathIntoView(path: Path2D): void;

	scrollPathIntoView(path?: Path2D): void {
	}

	setLineDash(segments: number[]): void {
		this.log('setLineDash', segments);
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
		this.log('setTransform', a, b, c, d, e, f);
		this._ensureLayoutBeforeDraw();
		this.context.setTransform(a, b, c, d, e, f);
	}

	stroke(): void;

	stroke(path?: Path2D): void {
		this.log('stroke', path);
		this._ensureLayoutBeforeDraw();
		if (path) {
			this.context.stroke(path.native);
		} else {
			this.context.stroke();
		}
	}

	strokeRect(x: number, y: number, width: number, height: number): void {
		this.log('strokeRect', x, y, width, height);
		this._ensureLayoutBeforeDraw();
		this.context.strokeRect(x, y, width, height);
	}

	strokeText(text: string, x: number, y: number, maxWidth?: number): void {
		this.log('strokeText', text, x, y, maxWidth);
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
		this.log('transform', a, b, c, d, e, f);
		this._ensureLayoutBeforeDraw();
		this.context.transform(a, b, c, d, e, f);
	}

	translate(x: number, y: number): void {
		this.log('translate', x, y);
		this._ensureLayoutBeforeDraw();
		this.context.translate(x, y);
	}

	private log(message, ...args) {
		if (!CanvasRenderingContext2D.isDebug) {
			return;
		}
		console.log(message, args);
	}

	private _ensureLayoutBeforeDraw() {
		if (this.canvas) {
			this.canvas._layoutNative();
		}
	}
}
