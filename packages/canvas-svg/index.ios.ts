import { SVGBase, srcProperty, syncProperty, initialSVG } from './common';
import { Http, knownFolders, path, Screen, Utils } from '@nativescript/core';
import { SVGItem } from './Elements/SVGItem';

export * from './Elements';

declare const NSCSVG, CanvasSVGHelper;

function parseSVGDimensions(svgString) {
	const svgRegex = /<svg([^>]*)>/i;
	const match = svgString.match(svgRegex);
	if (!match) {
		return { width: 0, height: 0 };
	}
	const svgAttributes = match[1];
	const regex = /\b(width|height|viewBox)\s*=\s*"([^"]+)"/g;

	let width, height, viewBox;

	let attributeMatch;
	while ((attributeMatch = regex.exec(svgAttributes)) !== null) {
		const attributeName = attributeMatch[1];
		const attributeValue = attributeMatch[2];

		if (attributeName === 'width') {
			width = parseFloat(attributeValue) || undefined;
		} else if (attributeName === 'height') {
			height = parseFloat(attributeValue) || undefined;
		} else if (attributeName === 'viewBox') {
			viewBox = attributeValue.split(' ').map(Number);
		}
	}

	if (!width && viewBox && viewBox.length === 4) {
		const viewBoxWidth = viewBox[2];
		const viewBoxHeight = viewBox[3];
		const aspectRatio = viewBoxWidth / viewBoxHeight;
		width = height ? height * aspectRatio : 150 * aspectRatio;
	}
	if (!height && viewBox && viewBox.length === 4) {
		const viewBoxWidth = viewBox[2];
		const viewBoxHeight = viewBox[3];
		const aspectRatio = viewBoxWidth / viewBoxHeight;
		height = width ? width / aspectRatio : 300 / aspectRatio;
	}

	return { width: width ?? 0, height: height ?? 0 };
}
export class SvgData {
	native: any;

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

	get data(): ArrayBuffer | null {
		const data = this.native?.data;
		if (data) {
			return interop.bufferFromData(data);
		}
		return null;
	}
}

export class Svg extends SVGBase {
	_svg;

	constructor() {
		super();
		this._svg = NSCSVG.alloc().initWithFrame(CGRectZero);
		this._svg.backgroundColor = UIColor.clearColor;
	}

	createNativeView() {
		return this._svg;
	}

	get native() {
		return this._svg;
	}

	[srcProperty.setNative](value: string) {
		if (typeof value === 'string') {
			if (value.indexOf('<svg') > -1) {
				this._svg.src = value;
			} else {
				if (value.startsWith('~')) {
					this._svg.srcPath = path.join(knownFolders.currentApp().path, value.replace('~', ''));
				} else if (value.startsWith('/')) {
					this._svg.srcPath = value;
				} else if (value.startsWith('http')) {
					Http.getFile(value)
						.then((res) => {
							this._svg.srcPath = res.path;
						})
						.catch((e) => {
							console.error(e);
						});
				}
			}
		}
	}

	[syncProperty.setNative](value: boolean) {
		this._svg.sync = value;
	}

	public onLayout(left: number, top: number, right: number, bottom: number): void {
		super.onLayout(left, top, right, bottom);
		this.__redraw();
	}

	public onMeasure(widthMeasureSpec: number, heightMeasureSpec: number) {
		const nativeView = this.nativeView;

		if (nativeView) {
			const width = Utils.layout.getMeasureSpecSize(widthMeasureSpec);
			const height = Utils.layout.getMeasureSpecSize(heightMeasureSpec);
			this.setMeasuredDimension(width, height);
		}
	}

	__redraw() {
		if (this._attachedToDom && !this.src) {
			const domCopy = this.__domElement.valueOf() as Element;
			const width = domCopy.getAttribute('width');
			const height = domCopy.getAttribute('height');
			const viewBox = domCopy.getAttribute('viewBox');
			if (!width) {
				domCopy.setAttribute('width', `300`);
			}
			if (!height) {
				domCopy.setAttribute('height', `150`);
			}

			if (!viewBox) {
				domCopy.setAttribute('viewBox', '0 0 100 100');
			}

			const serialized = this._serializer.serializeToString(domCopy);
			if (serialized !== initialSVG) {
				this.src = serialized;
			}
		}
	}

	onLoaded() {
		super.onLoaded();
		this._attachedToDom = true;
	}

	onUnloaded() {
		this._attachedToDom = false;
		super.onUnloaded();
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
		if (typeof value === 'string') {
			if (value.indexOf('<svg') > -1) {
				const { width, height } = parseSVGDimensions(value);
				if (width > 0 && height > 0) {
					const native = NSCSVG.fromSVGStringSync(value);
					return SvgData.fromNative(native);
				}
			} else {
				let nativeSvg;
				try {
					if (value.startsWith('~')) {
						nativeSvg = NSCSVG.fromPathSync(path.join(knownFolders.currentApp().path, value.replace('~', '')));
					} else if (value.startsWith('/')) {
						nativeSvg = NSCSVG.fromPathSync(value);
					} else if (value.startsWith('http')) {
						nativeSvg = NSCSVG.fromRemoteSync(value);
					}
				} catch (error) {}
				if (nativeSvg) {
					return SvgData.fromNative(nativeSvg);
				}
			}
		}
		return null;
	}

	static fromSrc(value: string): Promise<SvgData> {
		return new Promise((resolve, reject) => {
			if (typeof value === 'string') {
				if (value.indexOf('<svg') > -1) {
					const { width, height } = parseSVGDimensions(value);
					if (width > 0 && height > 0) {
						NSCSVG.fromString(value, (svg) => {
							const ret = SvgData.fromNative(svg);
							resolve(ret);
						});
						return;
					}
				} else {
					const cb = (nativeSvg) => {
						if (nativeSvg) {
							const ret = SvgData.fromNative(nativeSvg);
							resolve(ret);
						} else {
							reject(new Error('Failed to parse SVG'));
						}
					};
					if (value.startsWith('~')) {
						NSCSVG.fromPath(path.join(knownFolders.currentApp().path, value.replace('~', '')), cb);
						return;
					} else if (value.startsWith('/')) {
						NSCSVG.fromPath(value, cb);
						return;
					} else if (value.startsWith('http')) {
						NSCSVG.fromRemote(value, cb);
						return;
					}
				}
			}
			reject(new Error('Source is not valid'));
		});
	}
}
