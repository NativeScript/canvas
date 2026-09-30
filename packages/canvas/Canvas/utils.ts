const defaultGLOptions = {
	alpha: true,
	antialias: true,
	depth: true,
	failIfMajorPerformanceCaveat: false,
	powerPreference: 'default',
	premultipliedAlpha: true,
	preserveDrawingBuffer: false,
	stencil: false,
	desynchronized: false,
	xrCompatible: false,
};

const default2DOptions = {
	alpha: true,
	antialias: true,
	depth: false,
	failIfMajorPerformanceCaveat: false,
	powerPreference: 'default',
	premultipliedAlpha: true,
	preserveDrawingBuffer: false,
	stencil: false,
	desynchronized: false,
	xrCompatible: false,
	willReadFrequently: false,
};

export type CanvasContextType = '2d' | 'bitmaprenderer' | 'webgl' | 'webgl2' | 'experimental-webgl' | 'experimental-webgl2';

export function parsePowerPreference(powerPreference: string) {
	switch (powerPreference) {
		case 'default':
			return 0;
		case 'high-performance':
			return 1;
		case 'low-power':
			return 2;
		default:
			return -1;
	}
}

export function handleContextOptions(type: CanvasContextType, contextAttributes) {
	// A bitmaprenderer is backed by the same 2D surface and takes the same
	// options; the only one the spec gives it is `alpha`.
	if (type === 'bitmaprenderer') {
		type = '2d';
	}
	if (!contextAttributes) {
		if (type === '2d') {
			return { ...default2DOptions, powerPreference: 0 };
		}
		if (type.indexOf('webgl') > -1) {
			return { ...defaultGLOptions, powerPreference: 0 };
		}
	}
	if (type === '2d') {
		if (contextAttributes.alpha !== undefined && typeof contextAttributes.alpha === 'boolean') {
			return { ...contextAttributes, powerPreference: 0 };
		} else {
			return { ...contextAttributes, alpha: true, powerPreference: 0 };
		}
	}
	const glOptions = { ...defaultGLOptions };
	const setIfDefined = (prop: string, value: unknown) => {
		const property = glOptions[prop];
		if (property !== undefined && prop === 'powerPreference') {
			// converts to int
			glOptions[prop] = value as never;
		}
		if (property !== undefined && typeof value === typeof property) {
			glOptions[prop] = value;
		}
	};
	if (type.indexOf('webgl') > -1) {
		setIfDefined('alpha', contextAttributes.alpha);
		setIfDefined('antialias', contextAttributes.antialias);
		setIfDefined('depth', contextAttributes.depth);
		setIfDefined('failIfMajorPerformanceCaveat', contextAttributes.failIfMajorPerformanceCaveat);
		setIfDefined('powerPreference', parsePowerPreference(contextAttributes.powerPreference ?? 'default'));
		setIfDefined('premultipliedAlpha', contextAttributes.premultipliedAlpha);
		setIfDefined('preserveDrawingBuffer', contextAttributes.preserveDrawingBuffer);
		setIfDefined('stencil', contextAttributes.stencil);
		setIfDefined('desynchronized', contextAttributes.desynchronized);
		return glOptions;
	}
	return null;
}

export function removeItemFromArray(array: any[], item) {
	const index = array.indexOf(item);
	if (index !== -1) {
		array.splice(index, 1);
	}
}

export const microtask: (cb: () => void) => void = typeof queueMicrotask === 'function' ? queueMicrotask : (cb) => Promise.resolve().then(cb);

export function isPercentLength(value: any) {
	return (typeof value === 'object' && value?.unit === '%') || (typeof value === 'string' && value.trim().endsWith('%'));
}

/** A % length as a 0-1 fraction. */
export function percentFraction(value: any): number {
	return typeof value === 'string' ? parseFloat(value) / 100 : value.value;
}

const percentAxes_ = Symbol('percentAxes');

/**
 * Hands a % width/height to a parent that lays out its children itself (MasonKit), so it resolves
 * against the containing block rather than the canvas being sized to its surface. A size that
 * stops being a % clears it again.
 */
export function setParentPercentSize(view: any, horizontal: boolean, value: any) {
	const axis = horizontal ? 1 : 2;
	const axes = view[percentAxes_] ?? 0;
	if (isPercentLength(value)) {
		if (view.parent?._setChildPercentSize?.(view, horizontal, percentFraction(value))) {
			view[percentAxes_] = axes | axis;
		}
	} else if (axes & axis) {
		view.parent?._setChildPercentSize?.(view, horizontal, null);
		view[percentAxes_] = axes & ~axis;
	}
}
