export class MutationObserver {
	private callback: any;
	private element: any;
	private interval: any;
	private oldHtml: any;
	constructor(callback: () => void) {
		this.callback = callback;
	}

	observe(element, options) {
		this.element = element;
		return (this.interval = setInterval(
			(function (_this) {
				return function () {
					var html;
					html = _this.element.innerHTML;
					if (html !== _this.oldHtml) {
						_this.oldHtml = html;
						return _this.callback(null);
					}
				};
			})(this),
			200
		));
	}

	disconnect() {
		return clearInterval(this.interval);
	}
}
