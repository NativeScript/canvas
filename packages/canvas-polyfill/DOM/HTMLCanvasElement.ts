import { Canvas } from '@nativescript/canvas';
import { HTMLElement } from './HTMLElement';
import setValue from 'set-value';
import { DOMParser } from '@xmldom/xmldom';

export class HTMLCanvasElement extends HTMLElement {
	private _htmlCanvas = true;
	constructor(canvas?: Canvas) {
		super('canvas');
		if (arguments.length > 0) {
			canvas = arguments[0];
		}

		if (canvas instanceof Canvas) {
			this.nativeElement = canvas;
		} else {
			this.nativeElement = Canvas.createCustomView();
		}

		if (!this.nativeElement.__domElement) {
			this.nativeElement.__domElement = new DOMParser().parseFromString('<canvas></canvas>', 'text/html').documentElement as never;
		}
	}

	static [Symbol.hasInstance](obj) {
		return obj?._htmlCanvas || obj instanceof Canvas;
	}

	get _canvas() {
		return this.nativeElement;
	}

	get innerWidth() {
		return this.clientWidth;
	}

	get innerHeight() {
		return this.clientHeight;
	}

	get clientWidth() {
		return this.nativeElement['clientWidth'] as never;
	}

	get clientHeight() {
		return this.nativeElement['clientHeight'] as never;
	}

	set width(value) {
		setValue(this.nativeElement, 'width', value);
	}

	get width() {
		return this.nativeElement['width'] as never;
	}

	set height(value) {
		setValue(this.nativeElement, 'height', value);
	}

	get height() {
		return this.nativeElement['height'] as never;
	}

	toDataURL(type: string, encoderOptions: number = 0.92) {
		const nativeElement = this.nativeElement as never as {
			toDataURL: (type: string, encoderOptions: number) => string;
		};

		if (nativeElement) {
			return nativeElement.toDataURL?.(type, encoderOptions);
		}
		return 'data:,';
	}

	getContext(contextType: string, contextOptions) {
		const nativeElement = this.nativeElement as never as {
			getContext: (contextType: string, contextOptions) => any;
		};
		if (nativeElement) {
			return nativeElement.getContext(contextType, contextOptions);
		}
		return null;
	}

	setPointerCapture(id: string) {}

	releasePointerCapture(id: string) {}

	getRootNode() {
		return this;
	}
}

(<any>Canvas.prototype).toHTMLCanvas = function () {
	return new HTMLCanvasElement(this);
};
