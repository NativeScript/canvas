export class MutationObserver {
	private callback: any;
	private element: any;
	private _rafHandle: any;
	private _active: boolean = false;
	private oldHtml: any;

	constructor(callback: () => void) {
		this.callback = callback;
	}

	observe(element, _options?) {
		this.element = element;
		this._active = true;
		this.oldHtml = this.element.innerHTML;
		this._poll();
	}

	private _poll() {
		if (!this._active) {
			return;
		}
		const html = this.element?.innerHTML;
		if (html !== this.oldHtml) {
			this.oldHtml = html;
			this.callback(null);
		}
		this._rafHandle = requestAnimationFrame(() => this._poll());
	}

	disconnect() {
		this._active = false;
		if (this._rafHandle !== undefined) {
			cancelAnimationFrame(this._rafHandle);
			this._rafHandle = undefined;
		}
		this.element = null;
	}
}
