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

	const regex = /\b(?:width|height|viewBox)\s*=\s*"([^"]*(?:"[^"]*")*[^"]*)"/g;

	const matches = svgAttributes.match(regex);
	let width, height, viewBox;

	if (matches) {
		matches.forEach((match) => {
			const parts = match.split('=');
			const attributeName = parts[0].trim() as string;
			const attributeValue = parts[1].trim().replace(/"/g, '') as string;

			if (attributeName === 'width') {
				if (width === undefined) {
					width = parseFloat(attributeValue) ?? 0;
				}
			} else if (attributeName === 'height') {
				if (height === undefined) {
					height = parseFloat(attributeValue) ?? 0;
				}
			} else if (attributeName === 'viewBox') {
				if (viewBox === undefined) {
					viewBox = attributeValue.split(' ').map(parseFloat).splice(0, 4);
				}
			}
		});
	}

	if (!width && viewBox && viewBox.length === 4) {
		const viewBoxWidth = viewBox[2];
		const viewBoxHeight = viewBox[3];
		const aspectRatio = viewBoxWidth / viewBoxHeight;
		width = (height ?? Screen.mainScreen.widthDIPs) * aspectRatio;
	}

	if (!height && viewBox && viewBox.length === 4) {
		const viewBoxWidth = viewBox[2];
		const viewBoxHeight = viewBox[3];
		const aspectRatio = viewBoxWidth / viewBoxHeight;
		height = (width ?? Screen.mainScreen.heightDIPs) / aspectRatio;
	}

	return { width: width ?? 0, height: height ?? 0 };
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
		if (this._attachedToDom) {
			const domCopy = this.__domElement.valueOf() as Element;
			const width = domCopy.getAttribute('width');
			const height = domCopy.getAttribute('height');
			const viewBox = domCopy.getAttribute('viewBox');
			if (!width) {
				domCopy.setAttribute('width', `${this.getMeasuredWidth() / Screen.mainScreen.scale}`);
			}
			if (!height) {
				domCopy.setAttribute('height', `${this.getMeasuredHeight() / Screen.mainScreen.scale}`);
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

	static fromSrcSync(value: string): Svg | null {
		if (typeof value === 'string') {
			if (value.indexOf('<svg') > -1) {
				const { width, height } = parseSVGDimensions(value);
				if (width > 0 && height > 0) {
					const svg = NSCSVG.fromStringSync(value);
					svg.backgroundColor = UIColor.redColor;
					const view = new Svg();
					view._svg = svg;
					view.width = svg.frame.size.width;
					view.height = svg.frame.size.height;
					return view;
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
					const view = new Svg();
					view._svg = nativeSvg;
					const frame = nativeSvg.frame.size;
					view.width = frame.width;
					view.height = frame.height;
					return view;
				}
			}
		}
		return null;
	}

	static fromSrc(value: string): Promise<Svg> {
		return new Promise((resolve, reject) => {
			if (typeof value === 'string') {
				if (value.indexOf('<svg') > -1) {
					const { width, height } = parseSVGDimensions(value);
					if (width > 0 && height > 0) {
						NSCSVG.fromString(value, (svg) => {
							const view = new Svg();
							view._svg = svg;
							view.width = width;
							view.height = height;
							resolve(view);
						});
						return;
					}
				} else {
					const cb = (nativeSvg) => {
						if (nativeSvg) {
							const view = new Svg();
							const lp = nativeSvg.frame.size;
							view._svg = nativeSvg;
							view.width = lp.width;
							view.height = lp.height;
							resolve(view);
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
