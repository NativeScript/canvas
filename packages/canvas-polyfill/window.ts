import { Document } from './DOM/Document';
import './performance';
import { HTMLImageElement } from './DOM/HTMLImageElement';
import { HTMLCanvasElement } from './DOM/HTMLCanvasElement';
import { HTMLVideoElement } from './DOM/HTMLVideoElement';
import { XMLDocument } from './DOM/XMLDocument';
import { DOMPointReadOnly, DOMPoint } from './DOM/DOMPointReadOnly';
import { Device, fromObject, View } from '@nativescript/core';
import { CanvasRenderingContext2D, WebGLRenderingContext, WebGL2RenderingContext, ImageData, ImageBitmap } from '@nativescript/canvas';
import { HTMLCollection } from './DOM/HTMLCollection';
import { HTMLUnknownElement } from './DOM/HTMLUnknownElement';
import { Navigator } from './navigator';
(global as any).CANVAS_RENDERER = 'true';
(global as any).WEBGL_RENDERER = 'true';
(global as any).window = (global as any).window || {
	console: console,
	WEBGL_RENDERER: 'true',
	CANVAS_RENDERER: 'true',
};
(global as any).window.self = (global as any).self = (global as any).self || window;
(global as any).window.HTMLImageElement = (global as any).HTMLImageElement = (global as any).HTMLImageElement || HTMLImageElement;
(global as any).window.Image = (global as any).Image = (global as any).Image || HTMLImageElement;
(global as any).window.ImageBitmap = (global as any).ImageBitmap = (global as any).ImageBitmap || ImageBitmap;
(global as any).window.HTMLVideoElement = (global as any).HTMLVideoElement = (global as any).HTMLVideoElement || HTMLVideoElement;
(global as any).window.Video = (global as any).Video = (global as any).Video || HTMLVideoElement;
(global as any).window.HTMLCanvasElement = (global as any).HTMLCanvasElement = (global as any).HTMLCanvasElement || HTMLCanvasElement;
(global as any).window.Canvas = (global as any).Canvas = (global as any).Canvas || HTMLCanvasElement;
(global as any).window.CanvasRenderingContext2D = (global as any).CanvasRenderingContext2D = (global as any).CanvasRenderingContext2D || CanvasRenderingContext2D;
(global as any).window.WebGLRenderingContext = (global as any).WebGLRenderingContext = (global as any).WebGLRenderingContext || WebGLRenderingContext;
(global as any).window.WebGL2RenderingContext = (global as any).WebGL2RenderingContext = (global as any).WebGL2RenderingContext || WebGL2RenderingContext;
(global as any).window.ImageData = (global as any).ImageData = (global as any).ImageData || ImageData;
(global as any).window.DOMPoint = (global as any).DOMPoint = (global as any).DOMPoint || DOMPoint;
(global as any).window.DOMPointReadOnly = (global as any).DOMPointReadOnly = (global as any).DOMPointReadOnly || DOMPointReadOnly;
(global as any).window.HTMLCollection = (global as any).HTMLCollection = (global as any).HTMLCollection || HTMLCollection;
(global as any).window.HTMLUnknownElement = (global as any).HTMLUnknownElement = (global as any).HTMLUnknownElement || HTMLUnknownElement;

// svg
import { SVGMarkerElement, SVGAnimatedTransformList, SVGUseElement, SVGStopElement, SVGRadialGradientElement, SVGLinearGradientElement, SVGGradientElement, SVGTextElement, SVGPolygonElement, SVGEllipseElement, SVGImageElement, SVGAnimatedRect, SVGPointList, SVGTransformList, SVGTransform, SVGRect, SVGNumber, SVGMatrix, SVGPoint, SVGAngle, SVGCircleElement, SVGElement, SVGSVGElement, SVGGraphicsElement, SVGMaskElement, SVGLineElement, SVGLength, SVGAnimatedLength, SVGPolylineElement, SVGGElement, SVGPathElement, SVGRectElement, SVGAnimatedString } from './DOM/svg';

(global as any).window.SVGCircleElement = (global as any).SVGCircleElement = (global as any).SVGCircleElement || SVGCircleElement;
(global as any).window.SVGSVGElement = (global as any).SVGSVGElement = (global as any).SVGSVGElement || SVGSVGElement;
(global as any).window.SVGElement = (global as any).SVGElement = (global as any).SVGElement || SVGElement;
(global as any).window.SVGGraphicsElement = (global as any).SVGGraphicsElement = (global as any).SVGGraphicsElement || SVGGraphicsElement;
(global as any).window.SVGMaskElement = (global as any).SVGMaskElement = (global as any).SVGMaskElement || SVGMaskElement;
(global as any).window.SVGLineElement = (global as any).SVGLineElement = (global as any).SVGLineElement || SVGLineElement;
(global as any).window.SVGLength = (global as any).SVGLength = (global as any).SVGLength || SVGLength;
(global as any).window.SVGAnimatedLength = (global as any).SVGAnimatedLength = (global as any).SVGAnimatedLength || SVGAnimatedLength;
(global as any).window.SVGPolylineElement = (global as any).SVGPolylineElement = (global as any).SVGPolylineElement || SVGPolylineElement;
(global as any).window.SVGPolygonElement = (global as any).SVGPolygonElement = (global as any).SVGPolygonElement || SVGPolygonElement;
(global as any).window.SVGGElement = (global as any).SVGGElement = (global as any).SVGGElement || SVGGElement;
(global as any).window.SVGPathElement = (global as any).SVGPathElement = (global as any).SVGPathElement || SVGPathElement;
(global as any).window.SVGRectElement = (global as any).SVGRectElement = (global as any).SVGRectElement || SVGRectElement;
(global as any).window.SVGAnimatedString = (global as any).SVGAnimatedString = (global as any).SVGAnimatedString || SVGAnimatedString;

(global as any).window.SVGPoint = (global as any).SVGPoint = (global as any).SVGPoint || SVGPoint;
(global as any).window.SVGAngle = (global as any).SVGAngle = (global as any).SVGAngle || SVGAngle;
(global as any).window.SVGMatrix = (global as any).SVGMatrix = (global as any).SVGMatrix || SVGMatrix;
(global as any).window.SVGNumber = (global as any).SVGNumber = (global as any).SVGNumber || SVGNumber;
(global as any).window.SVGRect = (global as any).SVGRect = (global as any).SVGRect || SVGRect;
(global as any).window.SVGTransform = (global as any).SVGTransform = (global as any).SVGTransform || SVGTransform;
(global as any).window.SVGPointList = (global as any).SVGPointList = (global as any).SVGPointList || SVGPointList;
(global as any).window.SVGTransformList = (global as any).SVGTransformList = (global as any).SVGTransformList || SVGTransformList;
(global as any).window.SVGAnimatedRect = (global as any).SVGAnimatedRect = (global as any).SVGAnimatedRect || SVGAnimatedRect;

(global as any).window.SVGImageElement = (global as any).SVGImageElement = (global as any).SVGImageElement || SVGImageElement;
(global as any).window.SVGEllipseElement = (global as any).SVGEllipseElement = (global as any).SVGEllipseElement || SVGEllipseElement;
(global as any).window.SVGTextElement = (global as any).SVGTextElement = (global as any).SVGTextElement || SVGTextElement;

(global as any).window.SVGGradientElement = (global as any).SVGGradientElement = (global as any).SVGGradientElement || SVGGradientElement;
(global as any).window.SVGRadialGradientElement = (global as any).SVGRadialGradientElement = (global as any).SVGRadialGradientElement || SVGRadialGradientElement;
(global as any).window.SVGLinearGradientElement = (global as any).SVGLinearGradientElement = (global as any).SVGLinearGradientElement || SVGLinearGradientElement;
(global as any).window.SVGStopElement = (global as any).SVGStopElement = (global as any).SVGStopElement || SVGStopElement;
(global as any).window.SVGUseElement = (global as any).SVGUseElement = (global as any).SVGUseElement || SVGUseElement;
(global as any).window.SVGAnimatedTransformList = (global as any).SVGAnimatedTransformList = (global as any).SVGAnimatedTransformList || SVGAnimatedTransformList;
(global as any).window.SVGMarkerElement = (global as any).SVGMarkerElement = (global as any).SVGMarkerElement || SVGMarkerElement;

function checkEmitter() {
	if (!(global as any).emitter || !((global as any).emitter.on || (global as any).emitter.addEventListener || (global as any).emitter.addListener)) {
		(global as any).window.emitter = (global as any).emitter = fromObject({});
	}
}

(global as any).window.scrollTo = (global as any).scrollTo = (global as any).scrollTo || (() => ({}));

(global as any).window.addEventListener = (global as any).addEventListener = (eventName, listener) => {
	checkEmitter();
	const addListener = () => {
		if ((global as any).emitter.on) {
			(global as any).emitter.on(eventName, listener);
		} else if ((global as any).emitter.addEventListener) {
			(global as any).emitter.addEventListener(eventName, listener);
		} else if ((global as any).emitter.addListener) {
			(global as any).emitter.addListener(eventName, listener);
		}
	};

	if (typeof listener !== 'function') {
		return;
	}

	addListener();

	if (eventName.toLowerCase() === 'load') {
		if ((global as any).emitter && (global as any).emitter.emit) {
			setTimeout(() => {
				(global as any).emitter.emit('load');
			}, 1);
		}
	}
};

(global as any).window.removeEventListener = (global as any).removeEventListener = (eventName, listener) => {
	checkEmitter();
	if ((global as any).emitter.off) {
		(global as any).emitter.off(eventName, listener);
	} else if ((global as any).emitter.removeEventListener) {
		(global as any).emitter.removeEventListener(eventName, listener);
	} else if ((global as any).emitter.removeListener) {
		(global as any).emitter.removeListener(eventName, listener);
	}
};

import { DOMParser as Parser } from '@xmldom/xmldom';

type DOMParserSupportedType = 'application/xhtml+xml' | 'application/xml' | 'image/svg+xml' | 'text/html' | 'text/xml';

export class DOMParser {
	parseFromString(xmlsource: string, mimeType?: DOMParserSupportedType) {
		const instance = new Parser().parseFromString(xmlsource, mimeType);
		if (mimeType === 'image/svg+xml' || mimeType === 'application/xhtml+xml' || mimeType === 'text/xml' || mimeType === 'application/xml') {
			return XMLDocument.fromParser(instance);
		} else {
			if (instance) {
				const doc = new Document();
				doc.__instance = instance as never;
				return doc;
			}
		}

		return null;
	}
}

(global as any).window.DOMParser = (global as any).DOMParser = (global as any).DOMParser || DOMParser;

(global as any).window.XMLDocument = (global as any).XMLDocument = XMLDocument;

(global as any).window.navigator = (global as any).navigator = (global as any).navigator || new Navigator();

(global as any).window['chrome'] = (global as any)['chrome'] = (global as any)['chrome'] || {
	extension: {},
};
/// https://www.w3schools.com/js/js_window_location.asp
(global as any).window.location = (global as any).location = (global as any).location || {
	href: '', //  window.location.href returns the href (URL) of the current page
	hostname: '', // window.location.hostname returns the domain name of the web host
	pathname: '', // window.location.pathname returns the path and filename of the current page
	protocol: 'https', // window.location.protocol returns the web protocol used (http: or https:)
	assign: null, // window.location.assign loads a new document
};

if ((global as any).document) {
	(global as any).document.readyState = 'complete';
}

if (!global.ontouchstart) {
	global.ontouchstart = () => {};
}

(global as any).window.setTimeout = setTimeout;
(global as any).window.setInterval = setInterval;
(global as any).window.requestAnimationFrame = requestAnimationFrame;
(global as any).window.cancelAnimationFrame = cancelAnimationFrame;
(global as any).window.getComputedStyle = function (element, pseudoEltOptional) {
	const obj: any = {};
	obj.getPropertyValue = function (prop) {
		if (element instanceof View) {
			let val = element.style.get(prop);
			if (val !== undefined && typeof val.value && typeof val.unit) {
				if (typeof val.value && typeof val.unit) {
					return `${val.value}${val.unit}`;
				}
			}
			return val;
		}
		return null;
	};
	return obj;
};
