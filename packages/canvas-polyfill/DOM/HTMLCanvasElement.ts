import { Canvas } from '@nativescript/canvas';
import { HTMLElement } from './HTMLElement';
import setValue from 'set-value';

export class HTMLCanvasElement extends HTMLElement {
	constructor() {
		super('canvas');
		let canvas = undefined;
		if (arguments.length > 0) {
			canvas = arguments[0];
		}

		if (canvas instanceof Canvas) {
			this.nativeElement = canvas;
		} else {
			this.nativeElement = (Canvas as any).createCustomView();
		}
	}

	get _canvas() {
		return this.nativeElement;
	}

	set width(value) {
		setValue(this.nativeElement, 'width', value);
	}

	get width() {
		return this.nativeElement['width'];
	}

	set height(value) {
		setValue(this.nativeElement, 'height', value);
	}

	get height() {
		return this.nativeElement['height'];
	}

	toDataURL(type: string, encoderOptions: number = 0.92) {
		const nativeElement = this.nativeElement as never as {
			toDataURL: (type: string, encoderOptions: number) => string
		};

		if (nativeElement) {
			return nativeElement.toDataURL?.(type, encoderOptions);
		}
		return 'data:,';
	}

	getContext(contextType: string, contextOptions) {
		const nativeElement = this.nativeElement as never as {
			getContext: (contextType: string, contextOptions) => any
		};
		if (nativeElement) {
			return nativeElement.getContext(contextType, contextOptions);
		}
		return null;
	}

	setPointerCapture(id: string) {
	}

	releasePointerCapture(id: string) {
	}

}
