import { Style } from './HTMLElement';

export class CSSStyleDeclaration {
	private _style: Style;
	getPropertyValue(property: string): string {
		return this._style[property] ?? '';
	}

	item(index: number): string {
		return this._style.__item(index);
	}

	setProperty(property: string, value: string): void {
		this._style[property] = value;
	}

	removeProperty(property: string): string {
		const value = this._style[property];
		delete this._style[property];
		return value;
	}

	getPropertyPriority(property) {
		return '';
	}
}
