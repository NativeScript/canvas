import { CSSType, PercentLength, View, Screen, Utils, Application, Property, booleanConverter } from '@nativescript/core';
import { CanvasRenderingContext } from '../common';

export interface ICanvasBase {
	on(eventName: 'ready', callback: (data: any) => void, thisArg?: any): void;
}

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
	depth: true,
	failIfMajorPerformanceCaveat: false,
	powerPreference: 'default',
	premultipliedAlpha: true,
	preserveDrawingBuffer: false,
	stencil: false,
	desynchronized: false,
	xrCompatible: false,
};

interface EventOptions {
	bubbles?: boolean;
	cancelable?: boolean;
	composed?: boolean;
}

export class DOMRectReadOnly {
	readonly bottom: number;
	readonly height: number;
	readonly left: number;
	readonly right: number;
	readonly top: number;
	readonly width: number;
	readonly x: number;
	readonly y: number;
	constructor(x: number, y: number, width: number, height: number, top?: number, right?: number, bottom?: number, left?: number) {
		this.x = x;
		this.y = y;
		this.width = width;
		this.height = height;
		this.left = left ?? x;
		this.top = top ?? y;
		this.right = right ?? x + width;
		this.bottom = bottom ?? y + height;
	}
}

export class DOMRect extends DOMRectReadOnly {
	constructor(x: number, y: number, width: number, height: number, top?: number, right?: number, bottom?: number, left?: number) {
		super(x, y, width, height, top, right, bottom, left);
	}
}

export class Event {
	readonly type: string;
	readonly bubbles: boolean = false;
	readonly cancelable: boolean = false;
	readonly composed: boolean = false;
	readonly timeStamp: number;
	constructor(type: string, options: EventOptions) {
		this.type = type;
		this.bubbles = options?.bubbles ?? false;
		this.cancelable = options?.cancelable ?? false;
		this.cancelable = options?.cancelable ?? false;
		this.composed = options?.composed ?? false;
	}

	preventDefault() {}
	stopPropagation() {}
}

interface UIEventOptions extends EventOptions {
	detail?: number;
	view?: any;
	sourceCapabilities?: any;
}

export class UIEvent extends Event {
	readonly detail: number;
	readonly view: any;
	readonly sourceCapabilities?: any;
	constructor(type: 'load' | 'unload' | 'abort' | 'error' | 'select', options?: UIEventOptions) {
		super(type, options);
		this.detail = options?.detail ?? 0;
		this.view = options?.view ?? null;
		this.sourceCapabilities = options.sourceCapabilities ?? {};
	}
}

interface MouseEventOptions extends UIEventOptions {
	screenX?: number;
	screenY?: number;
	clientX?: number;
	clientY?: number;
	ctrlKey?: boolean;
	shiftKey?: boolean;
	altKey?: boolean;
	metaKey?: boolean;
	button?: number;
	buttons?: number;
	relatedTarget?: any;
	region?: any;
	movementX?: number;
	movementY?: number;
	pageX?: number;
	pageY?: number;
}

export class MouseEvent extends UIEvent {
	readonly screenX: number;
	readonly screenY: number;
	readonly clientX: number;
	readonly clientY: number;
	readonly ctrlKey: boolean;
	readonly shiftKey: boolean;
	readonly altKey: boolean;
	readonly metaKey: boolean;
	readonly button: number;
	readonly buttons: number;
	readonly relatedTarget: any;
	readonly region: any;
	readonly movementX: number;
	readonly movementY: number;
	readonly pageX: number;
	readonly pageY: number;
	constructor(type: 'dblclick' | 'mousedown' | 'mouseenter' | 'mouseleave' | 'mousemove' | 'mouseout' | 'mouseover' | 'mouseup', options?: MouseEventOptions) {
		super(type as any, options);
		this.screenX = options?.screenX ?? 0;
		this.screenY = options?.screenY ?? 0;
		this.clientX = options?.clientX ?? 0;
		this.clientY = options?.clientY ?? 0;
		this.ctrlKey = options?.ctrlKey ?? false;
		this.shiftKey = options?.ctrlKey ?? false;
		this.altKey = options?.ctrlKey ?? false;
		this.metaKey = options?.ctrlKey ?? false;
		this.button = options?.button ?? 0;
		this.buttons = options?.buttons ?? 0;
		this.relatedTarget = options?.relatedTarget ?? null;
		this.region = options?.region ?? null;
		this.movementX = options?.movementX ?? 0;
		this.movementY = options?.movementY ?? 0;
		this.pageX = options?.pageX ?? 0;
		this.pageY = options?.pageY ?? 0;
	}
}

interface PointerEventOptions extends MouseEventOptions {
	pointerId?: number;
	width?: number;
	height?: number;
	pressure?: number;
	tangentialPressure?: number;
	pointerType?: 'mouse' | 'touch' | 'pen';
	tiltX?: number;
	tiltY?: number;
	twist?: number;
	isPrimary?: boolean;
}

export class PointerEvent extends MouseEvent {
	readonly type: string;
	readonly pointerType: string;
	readonly pointerId: number;
	readonly width: number;
	readonly height: number;
	readonly pressure: number;
	readonly tangentialPressure: number;
	readonly tiltX?: number;
	readonly tiltY?: number;
	readonly twist?: number;
	readonly isPrimary?: boolean;
	constructor(type: 'pointerover' | 'pointerenter' | 'pointerdown' | 'pointermove' | 'pointerrawupdate' | 'pointerup' | 'pointercancel' | 'pointerout' | 'pointerleave' | 'gotpointercapture' | 'lostpointercapture', options?: PointerEventOptions) {
		super(type as any, options);
		this.pointerType = options?.pointerType ?? '';
		this.type = type;
		this.pointerId = options?.pointerId ?? 0;
		this.width = options?.width ?? 1;
		this.height = options?.height ?? 1;
		this.pressure = options?.pressure ?? 0;
		this.tangentialPressure = options?.tangentialPressure ?? 0;
		this.tiltX = options?.tiltX ?? 0;
		this.tiltY = options?.tiltY ?? 0;
		this.twist = options?.twist ?? 0;
		this.isPrimary = options?.isPrimary ?? false;
	}
	preventDefault() {}
	stopPropagation() {}
}

interface TouchOptions {
	identifier: number;
	target: any;
	clientX?: number;
	clientY?: number;
	screenX?: number;
	screenY?: number;
	pageX?: number;
	pageY?: number;
	radiusX?: number;
	radiusY?: number;
	rotationAngle?: number;
	force?: number;
}

export class Touch {
	readonly identifier: number;
	readonly target: any;
	readonly clientX: number;
	readonly clientY: number;
	readonly screenX: number;
	readonly screenY: number;
	readonly pageX: number;
	readonly pageY: number;
	readonly radiusX: number;
	readonly radiusY: number;
	readonly rotationAngle: number;
	readonly force: number;
	constructor(options: TouchOptions) {
		this.identifier = options.identifier;
		this.target = options.target;
		this.clientX = options?.clientX ?? 0;
		this.clientY = options?.clientY ?? 0;
		this.screenX = options?.screenX ?? 0;
		this.screenY = options?.screenY ?? 0;
		this.pageX = options?.pageX ?? 0;
		this.pageY = options?.pageY ?? 0;
		this.radiusX = options?.radiusX ?? 0;
		this.radiusY = options?.radiusY ?? 0;
		this.rotationAngle = options?.rotationAngle ?? 0;
		this.force = options?.force ?? 0;
	}
	preventDefault() {}
	stopPropagation() {}
}

export class TouchList {
	private _list: Touch[];

	static fromList(list: Touch[]): TouchList {
		const ret = new TouchList();
		ret._list = list;
		return ret;
	}

	static empty() {
		const ret = new TouchList();
		ret._list = [];
		return ret;
	}

	item(index: number): Touch {
		return this._list[index];
	}

	get length(): number {
		return this._list.length;
	}
}

interface TouchEventOptions extends UIEventOptions {
	touches?: TouchList;
	targetTouches?: TouchList;
	changedTouches?: Touch[];
	ctrlKey?: boolean;
	shiftKey?: boolean;
	altKey?: boolean;
	metaKey?: boolean;
}

export class TouchEvent extends UIEvent {
	readonly touches: TouchList;
	readonly targetTouches: TouchList;
	readonly changedTouches: Touch[];
	readonly ctrlKey: boolean;
	readonly shiftKey: boolean;
	readonly altKey: boolean;
	readonly metaKey: boolean;
	constructor(type: 'touchstart' | 'touchend' | 'touchmove' | 'touchcancel', options?: TouchEventOptions) {
		super(type as any, options);
		this.touches = options?.touches ?? TouchList.empty();
		this.targetTouches = options?.touches ?? TouchList.empty();
		this.changedTouches = options?.changedTouches ?? [];
		this.altKey = options?.altKey ?? false;
		this.metaKey = options?.metaKey ?? false;
		this.ctrlKey = options?.ctrlKey ?? false;
		this.shiftKey = options?.shiftKey ?? false;
	}
	preventDefault() {}
	stopPropagation() {}
}

interface WheelEventOptions extends UIEventOptions {
	deltaX?: number;
	deltaY?: number;
	deltaZ?: number;
	deltaMode?: number;
}

export class WheelEvent extends UIEvent {
	readonly deltaX?: number;
	readonly deltaY?: number;
	readonly deltaZ?: number;
	readonly deltaMode?: number;
	constructor(type: 'wheel', options?: WheelEventOptions) {
		super('wheel' as any, options);
		this.deltaX = options?.deltaX ?? 0;
		this.deltaY = options?.deltaY ?? 0;
		this.deltaZ = options?.deltaZ ?? 0;
		this.deltaMode = options?.deltaMode ?? 0;
	}
}

class Rectangle {
	top: number;
	left: number;
	width: number;
	height: number;
	constructor(top, left, width, height) {
		this.top = top;
		this.left = left;
		// check name of fields!
		this.width = width;
		this.height = height;
	}
}

export const ignorePixelScalingProperty = new Property<CanvasBase, boolean>({
	name: 'ignorePixelScaling',
	defaultValue: false,
	valueConverter: booleanConverter,
});

export const upscaleProperty = new Property<CanvasBase, boolean>({
	name: 'upscale',
	defaultValue: false,
	valueConverter: booleanConverter,
});

export const ignoreTouchEventsProperty = new Property<CanvasBase, boolean>({
	name: 'ignoreTouchEvents',
	defaultValue: false,
	valueConverter: booleanConverter,
});

export const doc = {
	defaultView: {
		getComputedStyle: function () {
			return null;
		},
	},
};

@CSSType('Canvas')
export abstract class CanvasBase extends View implements ICanvasBase {
	public static readyEvent = 'ready';
	ignorePixelScaling: boolean;
	upscaleProperty: boolean;
	ignoreTouchEvents: boolean;
	_isCustom: boolean = false;
	_classList: Set<any>;

	_pointerMoveCallbacks = [];
	_pointerUpCallbacks = [];
	_pointerDownCallbacks = [];
	_pointerCancelCallbacks = [];

	// For mouse compat because wtf
	_mouseMoveCallbacks = [];
	_mouseUpCallbacks = [];
	_mouseDownCallbacks = [];
	_mouseCancelCallbacks = [];
	_mouseWheelCallbacks = [];

	_touchStartCallbacks = new Array<(TouchEvent) => void>();
	_touchEndCallbacks = new Array<(TouchEvent) => void>();
	_touchMoveCallbacks = new Array<(TouchEvent) => void>();
	_touchCancelCallbacks = new Array<(TouchEvent) => void>();

	_touches: Touch[] = [];
	_touchesById: Touch[] = [];

	_lastPointerEventById: { pointerId: number; x: number; y: number }[] = new Array();

	protected constructor() {
		super();
		this._classList = new Set();
	}

	get ownerDocument() {
		return window?.document ?? doc;
	}

	public addEventListener(arg: string, callback: any, thisArg?: any) {
		super.addEventListener(arg, callback, thisArg);
		if (typeof arg !== 'string') {
			return;
		}
		const eventtype = arg.toLowerCase();

		switch (eventtype) {
			case 'mousemove':
				this._mouseMoveCallbacks.push(callback);
				break;
			case 'pointermove':
				this._pointerMoveCallbacks.push(callback);
				break;
			case 'mousedown':
				this._mouseDownCallbacks.push(callback);
				break;
			case 'pointerdown':
				this._pointerDownCallbacks.push(callback);
				break;
			case 'mouseup':
				this._mouseUpCallbacks.push(callback);
				break;
			case 'pointerup':
				this._pointerUpCallbacks.push(callback);
				break;
			case 'moveout':
			case 'mousecancel':
				this._mouseCancelCallbacks.push(callback);
				break;
			case 'pointercancel':
				this._pointerCancelCallbacks.push(callback);
				break;
			case 'touchstart':
				this._touchStartCallbacks.push(callback);
				break;
			case 'touchend':
				this._touchEndCallbacks.push(callback);
				break;
			case 'touchmove':
				this._touchMoveCallbacks.push(callback);
				break;
			case 'touchcancel':
				this._touchCancelCallbacks.push(callback);
				break;
			case 'wheel':
			case 'mousewheel':
			case 'dommousescroll':
				this._mouseWheelCallbacks.push(callback);
				break;
		}
	}

	private _removeItemFromArray(array: any[], item) {
		const index = array.indexOf(item);
		if (index !== -1) {
			array.splice(index, 1);
		}
	}

	public removeEventListener(arg: string, callback: any, thisArg?: any) {
		super.removeEventListener(arg, callback, thisArg);

		const eventtype = arg.toLowerCase();

		switch (eventtype) {
			case 'mousemove':
				this._removeItemFromArray(this._mouseMoveCallbacks, callback);
				break;
			case 'pointermove':
				this._removeItemFromArray(this._pointerMoveCallbacks, callback);
				break;
			case 'mousedown':
				this._removeItemFromArray(this._mouseDownCallbacks, callback);
				break;
			case 'pointerdown':
				this._removeItemFromArray(this._pointerDownCallbacks, callback);
				break;
			case 'mouseup':
				this._removeItemFromArray(this._mouseUpCallbacks, callback);
				break;
			case 'pointerup':
				this._removeItemFromArray(this._pointerUpCallbacks, callback);
				break;
			case 'moveout':
			case 'mousecancel':
				this._removeItemFromArray(this._mouseCancelCallbacks, callback);
				break;
			case 'pointercancel':
				this._removeItemFromArray(this._pointerCancelCallbacks, callback);
				break;
			case 'touchstart':
				this._removeItemFromArray(this._touchStartCallbacks, callback);
				break;
			case 'touchend':
				this._removeItemFromArray(this._touchEndCallbacks, callback);
				break;
			case 'touchmove':
				this._removeItemFromArray(this._touchMoveCallbacks, callback);
				break;
			case 'touchcancel':
				this._removeItemFromArray(this._touchCancelCallbacks, callback);
				break;
			case 'wheel':
			case 'mousewheel':
			case 'dommousescroll':
				this._removeItemFromArray(this._mouseWheelCallbacks, callback);
				break;
		}
	}

	private _moveCallback(pointers: { ptrId: number; x: number; y: number; isPrimary: boolean }[]) {
		const hasPointerCallbacks = this._pointerMoveCallbacks.length > 0;
		const hasMouseCallbacks = this._mouseMoveCallbacks.length > 0;
		if (hasPointerCallbacks || hasMouseCallbacks) {
			for (const pointer of pointers) {
				const pointerId = pointer.ptrId;
				const index = this._lastPointerEventById.findIndex((item) => {
					return item?.pointerId === pointerId;
				});
				let previousEvent;
				if (index > -1) {
					previousEvent = this._lastPointerEventById[index];
				} else {
					previousEvent = { pointerId, x: 0, y: 0 };
				}

				if (hasPointerCallbacks) {
					const event = new PointerEvent('pointermove', {
						pointerType: 'touch',
						pointerId,
						clientX: pointer.x,
						clientY: pointer.y,
						screenX: pointer.x,
						screenY: pointer.y,
						pageX: pointer.x,
						pageY: pointer.y,
						movementX: pointer.x - previousEvent.x,
						movementY: pointer.y - previousEvent.y,
						isPrimary: pointer.isPrimary,
						button: -1,
					});

					for (const callback of this._pointerMoveCallbacks) {
						callback(event);
					}
				}

				if (hasMouseCallbacks) {
					const event = new MouseEvent('mousemove', {
						clientX: pointer.x,
						clientY: pointer.y,
						screenX: pointer.x,
						screenY: pointer.y,
						pageX: pointer.x,
						pageY: pointer.y,
						movementX: pointer.x - previousEvent.x,
						movementY: pointer.y - previousEvent.y,
						button: -1,
					});

					for (const callback of this._mouseMoveCallbacks) {
						callback(event);
					}
				}

				if (index > -1) {
					this._lastPointerEventById[index] = { pointerId, x: pointer.x, y: pointer.y };
				}
			}
		}

		if (this._touchMoveCallbacks.length > 0) {
			const changedTouches = [];

			for (const pointer of pointers) {
				changedTouches.push(
					new Touch({
						identifier: pointer.ptrId,
						target: null,
						clientX: pointer.x,
						clientY: pointer.y,
						screenX: pointer.x,
						screenY: pointer.y,
						pageX: pointer.x,
						pageY: pointer.y,
					})
				);
			}

			const touches = TouchList.fromList(this._touches);

			const event = new TouchEvent('touchmove', {
				touches,
				targetTouches: touches,
				changedTouches,
			});

			for (const callback of this._touchMoveCallbacks) {
				callback(event);
			}
		}
	}

	private _upCallback(ptrId, x, y, isPrimary) {
		const hasPointerCallbacks = this._pointerUpCallbacks.length > 0;
		const hasMouseCallbacks = this._mouseUpCallbacks.length > 0;

		if (hasPointerCallbacks || hasMouseCallbacks) {
			const pointerId = ptrId;
			if (hasPointerCallbacks) {
				const event = new PointerEvent('pointerup', {
					pointerType: 'touch',
					pointerId,
					clientX: x,
					clientY: y,
					screenX: x,
					screenY: y,
					isPrimary,
					pageX: x,
					pageY: y,
				});

				for (const callback of this._pointerUpCallbacks) {
					callback(event);
				}
			}

			if (hasMouseCallbacks) {
				const event = new MouseEvent('mouseup', {
					clientX: x,
					clientY: y,
					screenX: x,
					screenY: y,
					pageX: x,
					pageY: y,
				});

				for (const callback of this._mouseUpCallbacks) {
					callback(event);
				}
			}

			const index = this._lastPointerEventById.findIndex((item) => {
				return item?.pointerId === pointerId;
			});
			if (index > -1) {
				this._lastPointerEventById.splice(index, 1);
			}
		}

		const length = this._touches.length;
		for (let i = 0; i < length; i++) {
			if (this._touches[i].identifier == ptrId) {
				this._touches.splice(i, 1);
				break;
			}
		}

		if (this._touchEndCallbacks.length > 0) {
			const touches = TouchList.fromList(this._touches);

			const changedTouches = [
				new Touch({
					identifier: ptrId,
					target: null,
					clientX: x,
					clientY: y,
					screenX: x,
					screenY: y,
					pageX: x,
					pageY: y,
				}),
			];

			const event = new TouchEvent('touchend', {
				touches,
				targetTouches: touches,
				changedTouches,
			});

			for (const callback of this._touchEndCallbacks) {
				callback(event);
			}
		}
	}

	private _downCallback(ptrId, x, y, isPrimary = false) {
		const hasPointerCallbacks = this._pointerDownCallbacks.length > 0;
		const hasMouseCallbacks = this._mouseDownCallbacks.length > 0;

		if (hasPointerCallbacks || hasMouseCallbacks) {
			const pointerId = ptrId;
			if (hasPointerCallbacks) {
				const event = new PointerEvent('pointerdown', {
					pointerType: 'touch',
					pointerId,
					clientX: x,
					clientY: y,
					screenX: x,
					screenY: y,
					isPrimary,
					pageX: x,
					pageY: y,
				});

				for (const callback of this._pointerDownCallbacks) {
					callback(event);
				}
			}

			if (hasMouseCallbacks) {
				const event = new MouseEvent('mousedown', {
					clientX: x,
					clientY: y,
					screenX: x,
					screenY: y,
					pageX: x,
					pageY: y,
				});

				for (const callback of this._mouseDownCallbacks) {
					callback(event);
				}
			}

			this._lastPointerEventById.push({ pointerId, x, y });
		}

		if (this._touchStartCallbacks.length > 0) {
			const touch = new Touch({
				identifier: ptrId,
				target: this,
				clientX: x,
				clientY: y,
				screenX: x,
				screenY: y,
				pageX: x,
				pageY: y,
			});
			this._touches.push(touch);
			this._touchesById[ptrId] = touch;

			const touches = TouchList.fromList(this._touches);
			const touchEvent = new TouchEvent('touchstart', {
				touches,
				targetTouches: touches,
				changedTouches: this._touches,
			});

			for (const callback of this._touchStartCallbacks) {
				callback(touchEvent);
			}
		}
	}

	private _cancelCallback(ptrid, x, y, isPrimary = false) {
		const hasPointerCallbacks = this._pointerCancelCallbacks.length > 0;
		const hasMouseCallbacks = this._mouseCancelCallbacks.length > 0;
		if (hasPointerCallbacks || hasMouseCallbacks) {
			const pointerId = ptrid;
			if (hasPointerCallbacks) {
				const event = new PointerEvent('pointercancel', {
					pointerType: 'touch',
					pointerId,
					clientX: x,
					clientY: y,
					screenX: x,
					screenY: y,
					pageX: x,
					pageY: y,
					isPrimary,
				});

				for (const callback of this._pointerCancelCallbacks) {
					callback(event);
				}
			}

			if (hasMouseCallbacks) {
				const event = new MouseEvent('mouseout', {
					clientX: x,
					clientY: y,
					screenX: x,
					screenY: y,
					pageX: x,
					pageY: y,
				});

				for (const callback of this._mouseCancelCallbacks) {
					callback(event);
				}
			}
		}

		if (this._touchCancelCallbacks.length > 0) {
			const touch = new Touch({
				identifier: ptrid,
				target: this,
				clientX: x,
				clientY: y,
				screenX: x,
				screenY: y,
				pageX: x,
				pageY: y,
			});
			const touchesList = [touch];
			const touchesById = [];
			touchesById[ptrid] = touch;
			const touches = TouchList.fromList(touchesList);
			const touchEvent = new TouchEvent('touchcancel', {
				touches,
				targetTouches: touches,
				changedTouches: touchesList,
			});

			for (const callback of this._touchCancelCallbacks) {
				callback(touchEvent);
			}
		}
	}

	private _pinchCallback(data: { event: string; deltaX: number; deltaY: number; deltaMode: number; pointers: { ptrId: number; x: number; y: number }[]; isInProgress: boolean }) {
		// move callback

		const hasPointerCallbacks = this._pointerMoveCallbacks.length > 0;
		const hasMouseCallbacks = this._mouseMoveCallbacks.length > 0;
		const hasMouseWheel = this._mouseWheelCallbacks.length > 0;

		if (hasPointerCallbacks || hasMouseCallbacks || hasMouseWheel) {
			for (const pointer of data.pointers) {
				const pointerId = pointer.ptrId;

				const index = this._lastPointerEventById.findIndex((item) => {
					return item?.pointerId === pointerId;
				});
				let previousEvent;
				if (index > -1) {
					previousEvent = this._lastPointerEventById[index];
				} else {
					previousEvent = { pointerId, x: 0, y: 0 };
				}

				if (hasPointerCallbacks) {
					const event = new PointerEvent('pointermove', {
						pointerType: 'touch',
						pointerId,
						clientX: pointer.x,
						clientY: pointer.y,
						screenX: pointer.x,
						screenY: pointer.y,
						pageX: pointer.x,
						pageY: pointer.y,
						movementX: pointer.x - previousEvent.x,
						movementY: pointer.y - previousEvent.y,
						button: -1,
					});

					for (const callback of this._pointerMoveCallbacks) {
						callback(event);
					}
				}

				if (hasMouseCallbacks) {
					const event = new MouseEvent('mousemove', {
						clientX: pointer.x,
						clientY: pointer.y,
						screenX: pointer.x,
						screenY: pointer.y,
						pageX: pointer.x,
						pageY: pointer.y,
						movementX: pointer.x - previousEvent.x,
						movementY: pointer.y - previousEvent.y,
						button: -1,
					});

					for (const callback of this._mouseMoveCallbacks) {
						callback(event);
					}
				}

				if (hasMouseWheel) {
					const event = new WheelEvent('wheel', {
						deltaX: data.deltaX,
						deltaY: data.deltaY,
						deltaZ: 0,
						deltaMode: data.deltaMode,
					});

					for (const callback of this._mouseWheelCallbacks) {
						callback(event);
					}
				}

				if (index > -1) {
					this._lastPointerEventById[index] = { pointerId, x: pointer.x, y: pointer.y };
				}
			}
		}

		if (this._touchMoveCallbacks.length > 0) {
			const changedTouches = [];

			for (const pointer of data.pointers) {
				changedTouches.push(
					new Touch({
						identifier: pointer.ptrId,
						target: this,
						clientX: pointer.x,
						clientY: pointer.y,
						screenX: pointer.x,
						screenY: pointer.y,
						pageX: pointer.x,
						pageY: pointer.y,
					})
				);
			}

			const touches = TouchList.fromList(this._touches);

			const event = new TouchEvent('touchmove', {
				touches,
				targetTouches: touches,
				changedTouches,
			});

			for (const callback of this._touchMoveCallbacks) {
				callback(event);
			}
		}
	}

	_handleEvents(event) {
		try {
			const data = JSON.parse(event);
			switch (data.event) {
				case 'down':
					this._downCallback(data.ptrId, data.x, data.y, data.isPrimary);
					break;
				case 'move':
					this._moveCallback(data.pointers);
					break;
				case 'up':
					this._upCallback(data.ptrId, data.x, data.y, data.isPrimary);
					break;
				case 'scale':
					this._pinchCallback(data);
					break;
				case 'cancel':
					this._cancelCallback(data.ptrId, data.x, data.y, data.isPrimary);
					break;
				default:
					break;
			}
		} catch (error) {}
	}

	get classList() {
		return this._classList;
	}

	get _realSize(): { width: number; height: number } {
		return {
			width: this.getSize(this.style.width, this.getMeasuredWidth(), 'width'),
			height: this.getSize(this.style.height, this.getMeasuredHeight(), 'height'),
		};
	}

	_handleContextOptions(type, contextOpts?) {
		if (!contextOpts) {
			if (type === '2d') {
				return { ...default2DOptions };
			}
			if (type.indexOf('webgl') > -1) {
				return { ...defaultGLOptions };
			}
		}
		if (type === '2d') {
			if (contextOpts.alpha !== undefined && typeof contextOpts.alpha === 'boolean') {
				return contextOpts;
			} else {
				return { alpha: true };
			}
		}
		const glOptions = { ...defaultGLOptions };
		const setIfDefined = (prop, value) => {
			const property = glOptions[prop];
			if (property !== undefined && typeof value === typeof property) {
				glOptions[prop] = value;
			}
		};
		if (type.indexOf('webgl') > -1) {
			setIfDefined('alpha', contextOpts.alpha);
			setIfDefined('antialias', contextOpts.antialias);
			setIfDefined('depth', contextOpts.depth);
			setIfDefined('failIfMajorPerformanceCaveat', contextOpts.failIfMajorPerformanceCaveat);
			setIfDefined('powerPreference', contextOpts.powerPreference);
			setIfDefined('premultipliedAlpha', contextOpts.premultipliedAlpha);
			setIfDefined('preserveDrawingBuffer', contextOpts.preserveDrawingBuffer);
			setIfDefined('stencil', contextOpts.stencil);
			setIfDefined('desynchronized', contextOpts.desynchronized);
			return glOptions;
		}
		return null;
	}

	_readyEvent() {
		this.notify({
			eventName: 'ready',
			object: this,
		});
	}

	getAttribute(attrib) {
		if (attrib === 'width') {
			return this.width;
		}
		if (attrib === 'height') {
			return this.height;
		}

		if (attrib === 'tabindex') {
			return (this['tabindex'] = arguments[1]);
		}
		return this[attrib];
	}

	setAttribute(attrib) {
		if (attrib === 'width') {
			if (!isNaN(parseInt(arguments[1]))) {
				this.width = parseInt(arguments[1]);
			}
		}
		if (attrib === 'height') {
			if (!isNaN(parseInt(arguments[1]))) {
				this.height = parseInt(arguments[1]);
			}
		}
		if (attrib === 'tabindex') {
			this['tabindex'] = arguments[1];
		}
	}

	public abstract getContext(type: string, options?: any): CanvasRenderingContext | null;

	public abstract getBoundingClientRect(): {
		x: number;
		y: number;
		width: number;
		height: number;
		top: number;
		right: number;
		bottom: number;
		left: number;
	};

	private getSize(value, measuredSize, type): number {
		if (value === 'auto') {
			return Utils.layout.toDeviceIndependentPixels(Utils.layout.getMeasureSpecSize(Utils.layout.makeMeasureSpec(measuredSize, Utils.layout.UNSPECIFIED)));
		}
		if (typeof value === 'string') {
			value = PercentLength.parse(value);
		}
		if (typeof value === 'number') {
			if (global.isAndroid) {
				return Utils.layout.toDevicePixels(value) || 0;
			}
			return value || 0;
		} else if ((value !== null || true) && typeof value === 'object' && typeof value.value && typeof value.unit) {
			if (value.unit === 'px') {
				if (global.isIOS) {
					return Utils.layout.toDeviceIndependentPixels(value.value || 0);
				}
				return value.value || 0;
			} else if (value.unit === 'dip') {
				if (global.isAndroid) {
					return Utils.layout.toDevicePixels(value.value) || 0;
				}
				return value.value || 0;
			} else if (value.unit === '%') {
				return Utils.layout.toDeviceIndependentPixels(measuredSize * value.value ?? 0);
			}
		}

		return 0;
	}

	setPointerCapture(id) {}

	releasePointerCapture(id) {}
}

ignorePixelScalingProperty.register(CanvasBase);

upscaleProperty.register(CanvasBase);
