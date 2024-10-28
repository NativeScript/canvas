import { initialSVG, SVGBase, srcProperty, syncProperty } from './common';
import { Application, Http, knownFolders, path, Screen, Utils } from '@nativescript/core';
import { SVGItem } from './Elements/SVGItem';
export * from './Elements';

declare const org;

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
			const split = match.split(' ');

			for (const part of split) {
				const parts = part.split('=');

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
		return this.native?.getWidth?.() ?? 0;
	}

	get height(): number {
		return this.native?.getHeight?.() ?? 0;
	}

	get data(): ArrayBuffer | null {
		const data = this.native?.getData?.();
		if (data) {
			return (<any>ArrayBuffer).from(data);
		}
		return null;
	}
}

export class Svg extends SVGBase {
	_svg;

	constructor() {
		super();
		const context = Application.android.foregroundActivity || Application.android.startActivity || Utils.android.getApplicationContext();
		this._svg = new org.nativescript.canvas.svg.NSCSVG(context);
		this.on('layoutChanged', (args) => {
			this.__redraw();
		});
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
				this._svg.setSrc(value);
			} else {
				if (value.startsWith('~')) {
					this._svg.setSrcPath(path.join(knownFolders.currentApp().path, value.replace('~', '')));
				} else if (value.startsWith('/')) {
					this._svg.setSrcPath(value);
				} else if (value.startsWith('http')) {
					Http.getFile(value)
						.then((res) => {
							this._svg.setSrcPath(res.path);
						})
						.catch((e) => {
							console.log(e);
						});
				}
			}
		}
	}

	[syncProperty.setNative](value: boolean) {
		this._svg.setSync(value);
	}

	__redraw() {
		if (this._attachedToDom) {
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
			const context = Utils.android.getApplicationContext();
			if (value.indexOf('<svg') > -1) {
				const { width, height } = parseSVGDimensions(value);
				if (width > 0 && height > 0) {
					const svg = org.nativescript.canvas.svg.NSCSVG.fromStringSync(context, width, height, value);
					return SvgData.fromNative(svg);
				}
			} else {
				let nativeSvg;
				try {
					if (value.startsWith('~')) {
						nativeSvg = org.nativescript.canvas.svg.NSCSVG.fromPathSync(context, path.join(knownFolders.currentApp().path, value.replace('~', '')));
					} else if (value.startsWith('/')) {
						nativeSvg = org.nativescript.canvas.svg.NSCSVG.fromPathSync(context, value);
					} else if (value.startsWith('http')) {
						nativeSvg = org.nativescript.canvas.svg.NSCSVG.fromRemoteSync(context, value);
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
				const context = Utils.android.getApplicationContext();
				if (value.indexOf('<svg') > -1) {
					const { width, height } = parseSVGDimensions(value);
					if (width > 0 && height > 0) {
						org.nativescript.canvas.svg.NSCSVG.fromString(
							context,
							width,
							height,
							value,
							new org.nativescript.canvas.svg.NSCSVG.Callback({
								onSuccess(svg) {
									const ret = SvgData.fromNative(svg);
									resolve(ret);
								},
							})
						);
						return;
					}
				} else {
					const cb = new org.nativescript.canvas.svg.NSCSVG.Callback({
						onSuccess(nativeSvg) {
							if (nativeSvg) {
								const ret = SvgData.fromNative(nativeSvg);
								resolve(ret);
							} else {
								reject(new Error('Failed to parse SVG'));
							}
						},
					});
					if (value.startsWith('~')) {
						org.nativescript.canvas.svg.NSCSVG.fromPath(context, path.join(knownFolders.currentApp().path, value.replace('~', '')), cb);
						return;
					} else if (value.startsWith('/')) {
						org.nativescript.canvas.svg.NSCSVG.fromPath(context, value, cb);
						return;
					} else if (value.startsWith('http')) {
						org.nativescript.canvas.svg.NSCSVG.fromRemote(context, value, cb);
						return;
					}
				}
			}
			reject(new Error('Source is not valid'));
		});
	}
}
