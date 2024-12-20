import { CSSType, View, Property, booleanConverter } from '@nativescript/core';
import type { CanvasRenderingContext } from '../common';
import { removeItemFromArray } from './utils';

export interface ICanvasBase {
	on(eventName: 'ready', callback: (data: any) => void, thisArg?: any): void;
}

interface EventOptions {
	bubbles?: boolean;
	cancelable?: boolean;
	composed?: boolean;
	target?: any;
	preventDefault?: () => void;
	stopPropagation?: () => void;
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
	readonly target: any;

	private _preventDefault?: () => void;
	private _stopPropagation?: () => void;

	constructor(type: string, options: EventOptions) {
		this.type = type;
		this.bubbles = options?.bubbles ?? false;
		this.cancelable = options?.cancelable ?? false;
		this.composed = options?.composed ?? false;
		this.target = options?.target ?? null;
		this._preventDefault = options?.preventDefault;
		this._stopPropagation = options?.stopPropagation;
	}

	preventDefault() {
		this._preventDefault?.();
	}

	stopPropagation() {
		this._stopPropagation?.();
	}
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

export class TouchList extends Array {
	static fromList(list: Touch[]): TouchList {
		const ret = new TouchList();
		ret.splice(0, 0, ...list);
		return ret;
	}

	static empty() {
		return new TouchList();
	}

	item(index: number): Touch {
		return this[index];
	}

	get length(): number {
		return this.length;
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

	constructor(top: number, left: number, width: number, height: number) {
		this.top = top;
		this.left = left;
		// check name of fields!
		this.width = width;
		this.height = height;
	}
}

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
	ignoreTouchEvents: boolean;
	_isCustom: boolean = false;
	_classList: Set<any>;

	_pointerMoveCallbacks = [];
	_pointerUpCallbacks = [];
	_pointerOutCallbacks = [];
	_pointerLeaveCallbacks = [];
	_pointerDownCallbacks = [];
	_pointerCancelCallbacks = [];

	// For mouse compat because wtf
	_mouseMoveCallbacks = [];
	_mouseUpCallbacks = [];
	_mouseDownCallbacks = [];
	_mouseCancelCallbacks = [];
	_mouseWheelCallbacks = [];

	_touchStartCallbacks = new Array<(arg0: TouchEvent) => void>();
	_touchEndCallbacks = new Array<(arg0: TouchEvent) => void>();
	_touchMoveCallbacks = new Array<(arg0: TouchEvent) => void>();
	_touchCancelCallbacks = new Array<(arg0: TouchEvent) => void>();

	_touches: Touch[] = [];
	_touchesById: Touch[] = [];

	_lastPointerEventById: Map<number, { pointerId: number; x: number; y: number }> = new Map();

	protected constructor() {
		super();
		this._classList = new Set();
		this.style.width = { unit: '%', value: 1 };
		this.style.height = 'auto';
	}

	get isConnected() {
		return this.parent !== null && this.parent !== undefined;
	}

	get ownerDocument() {
		//return window?.document ?? doc;
		return this;
	}

	emit() {}

	dispatchEvent(event) {
		return true;
	}

	__target = null;

	public addEventListener(arg: string, callback: any, thisArg?: any) {
		if (typeof thisArg === 'boolean') {
			thisArg = {
				capture: thisArg,
			};
		}
		super.addEventListener(arg, callback, thisArg);
		if (typeof arg !== 'string') {
			return;
		}

		switch (arg) {
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
			case 'pointerout':
				this._pointerOutCallbacks.push(callback);
				break;
			case 'pointerleave':
				this._pointerLeaveCallbacks.push(callback);
				break;
			case 'mouseout':
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

	public removeEventListener(arg: string, callback: any, thisArg?: any) {
		if (typeof thisArg === 'boolean') {
			thisArg = {
				capture: thisArg,
			};
		}

		super.removeEventListener(arg, callback, thisArg);

		switch (arg) {
			case 'mousemove':
				removeItemFromArray(this._mouseMoveCallbacks, callback);
				break;
			case 'pointermove':
				removeItemFromArray(this._pointerMoveCallbacks, callback);
				break;
			case 'mousedown':
				removeItemFromArray(this._mouseDownCallbacks, callback);
				break;
			case 'pointerdown':
				removeItemFromArray(this._pointerDownCallbacks, callback);
				break;
			case 'mouseup':
				removeItemFromArray(this._mouseUpCallbacks, callback);
				break;
			case 'pointerup':
				removeItemFromArray(this._pointerUpCallbacks, callback);
				break;
			case 'pointerout':
				removeItemFromArray(this._pointerOutCallbacks, callback);
				break;
			case 'pointerleave':
				removeItemFromArray(this._pointerLeaveCallbacks, callback);
				break;
			case 'mouseout':
			case 'mousecancel':
				removeItemFromArray(this._mouseCancelCallbacks, callback);
				break;
			case 'pointercancel':
				removeItemFromArray(this._pointerCancelCallbacks, callback);
				break;
			case 'touchstart':
				removeItemFromArray(this._touchStartCallbacks, callback);
				break;
			case 'touchend':
				removeItemFromArray(this._touchEndCallbacks, callback);
				break;
			case 'touchmove':
				removeItemFromArray(this._touchMoveCallbacks, callback);
				break;
			case 'touchcancel':
				removeItemFromArray(this._touchCancelCallbacks, callback);
				break;
			case 'wheel':
			case 'mousewheel':
			case 'dommousescroll':
				removeItemFromArray(this._mouseWheelCallbacks, callback);
				break;
		}
	}

	private _moveCallback(pointers: { ptrId: number; x: number; y: number; isPrimary: boolean }[]) {
		const hasPointerCallbacks = this._pointerMoveCallbacks.length > 0;
		const hasMouseCallbacks = this._mouseMoveCallbacks.length > 0;
		const hasTouchCallbacks = this._touchMoveCallbacks.length > 0;
		if (hasPointerCallbacks || hasMouseCallbacks || hasTouchCallbacks) {
			let preventDefault = false;
			const changedTouches = hasTouchCallbacks ? TouchList.empty() : null;
			for (const pointer of pointers) {
				const pointerId = pointer.ptrId;
				const previousEvent = this._lastPointerEventById.get(pointerId) ?? { pointerId, x: 0, y: 0 };

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
						target: this.__target ?? this,
						cancelable: true,
						preventDefault: () => {
							preventDefault = true;
						},
					});

					for (const callback of this._pointerMoveCallbacks) {
						callback(event);
					}
				}

				if (!preventDefault && hasTouchCallbacks) {
					changedTouches.push(
						new Touch({
							identifier: pointer.ptrId,
							target: this.__target ?? this,
							clientX: pointer.x,
							clientY: pointer.y,
							screenX: pointer.x,
							screenY: pointer.y,
							pageX: pointer.x,
							pageY: pointer.y,
						})
					);
				}

				this._lastPointerEventById.set(pointerId, { pointerId, x: pointer.x, y: pointer.y });
			}

			if ((changedTouches?.length ?? 0) > 0) {
				const touches = TouchList.fromList(this._touches);

				const event = new TouchEvent('touchmove', {
					touches,
					targetTouches: touches,
					changedTouches,
					target: this.__target ?? this,
					cancelable: true,
					preventDefault: () => {
						preventDefault = true;
					},
				});

				for (const callback of this._touchMoveCallbacks) {
					callback(event);
				}
			}

			if (!preventDefault && hasMouseCallbacks) {
				for (const pointer of pointers) {
					const pointerId = pointer.ptrId;
					const previousEvent = this._lastPointerEventById.get(pointerId) ?? { pointerId, x: 0, y: 0 };
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
						target: this.__target ?? this,
						preventDefault: () => {
							preventDefault = true;
						},
					});
					for (const callback of this._mouseMoveCallbacks) {
						callback(event);
					}
				}
			}
		}
	}

	private _upCallback(ptrId: number, x: number, y: number, isPrimary: boolean = false) {
		const hasPointerCallbacks = this._pointerUpCallbacks.length > 0 || this._pointerOutCallbacks.length > 0 || this._pointerLeaveCallbacks.length > 0;
		const hasMouseCallbacks = this._mouseUpCallbacks.length > 0;
		const hasTouchCallbacks = this._touchEndCallbacks.length > 0;
		const pointerId = ptrId;
		if (hasPointerCallbacks || hasMouseCallbacks || hasTouchCallbacks) {
			let preventDefault = false;
			if (hasPointerCallbacks) {
				const up = new PointerEvent('pointerup', {
					pointerType: 'touch',
					pointerId,
					clientX: x,
					clientY: y,
					screenX: x,
					screenY: y,
					isPrimary,
					pageX: x,
					pageY: y,
					target: this.__target ?? this,
					cancelable: true,
					preventDefault: () => {
						preventDefault = true;
					},
				});
				const out = new PointerEvent('pointerout', {
					pointerType: 'touch',
					pointerId,
					clientX: x,
					clientY: y,
					screenX: x,
					screenY: y,
					isPrimary,
					pageX: x,
					pageY: y,
					target: this.__target ?? this,
				});
				const leave = new PointerEvent('pointerleave', {
					pointerType: 'touch',
					pointerId,
					clientX: x,
					clientY: y,
					screenX: x,
					screenY: y,
					isPrimary,
					pageX: x,
					pageY: y,
					target: this.__target ?? this,
				});

				for (const callback of this._pointerUpCallbacks) {
					callback(up);
				}

				for (const callback of this._pointerOutCallbacks) {
					callback(out);
				}

				for (const callback of this._pointerLeaveCallbacks) {
					callback(leave);
				}
			}

			if (hasTouchCallbacks && !preventDefault) {
				const touches = TouchList.fromList(this._touches);

				const changedTouches = TouchList.fromList([
					new Touch({
						identifier: ptrId,
						target: this.__target ?? this,
						clientX: x,
						clientY: y,
						screenX: x,
						screenY: y,
						pageX: x,
						pageY: y,
					}),
				]);

				const event = new TouchEvent('touchend', {
					touches,
					targetTouches: touches,
					changedTouches,
					target: this.__target ?? this,
					cancelable: true,
					preventDefault: () => {
						preventDefault = true;
					},
				});

				for (const callback of this._touchEndCallbacks) {
					callback(event);
				}
			}

			if (hasMouseCallbacks && !preventDefault) {
				const event = new MouseEvent('mouseup', {
					clientX: x,
					clientY: y,
					screenX: x,
					screenY: y,
					pageX: x,
					pageY: y,
					target: this.__target ?? this,
					preventDefault: () => {
						preventDefault = true;
					},
				});

				for (const callback of this._mouseUpCallbacks) {
					callback(event);
				}
			}
		}

		const length = this._touches.length;
		for (let i = 0; i < length; i++) {
			if (this._touches[i].identifier == ptrId) {
				this._touches.splice(i, 1);
				break;
			}
		}

		this._lastPointerEventById.delete(pointerId);
	}

	private _downCallback(ptrId: number, x: number, y: number, isPrimary = false) {
		const hasPointerCallbacks = this._pointerDownCallbacks.length > 0;
		const hasMouseCallbacks = this._mouseDownCallbacks.length > 0;
		const hasTouchCallbacks = this._touchStartCallbacks.length > 0;
		if (hasPointerCallbacks || hasMouseCallbacks || hasTouchCallbacks) {
			let preventDefault = false;
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
					target: this.__target ?? this,
					cancelable: true,
					preventDefault: () => {
						preventDefault = true;
					},
				});

				for (const callback of this._pointerDownCallbacks) {
					callback(event);
				}
			}

			if (hasTouchCallbacks && !preventDefault) {
				const touch = new Touch({
					identifier: ptrId,
					target: this.__target ?? this,
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
					target: this.__target ?? this,
					cancelable: true,
					preventDefault: () => {
						preventDefault = true;
					},
				});

				for (const callback of this._touchStartCallbacks) {
					callback(touchEvent);
				}
			}

			if (hasMouseCallbacks && !preventDefault) {
				const event = new MouseEvent('mousedown', {
					clientX: x,
					clientY: y,
					screenX: x,
					screenY: y,
					pageX: x,
					pageY: y,
					target: this.__target ?? this,
					preventDefault: () => {
						preventDefault = true;
					},
				});

				for (const callback of this._mouseDownCallbacks) {
					callback(event);
				}
			}

			this._lastPointerEventById.set(pointerId, { pointerId, x, y });
		}
	}

	private _cancelCallback(ptrId: number, x: number, y: number, isPrimary = false) {
		const hasPointerCallbacks = this._pointerCancelCallbacks.length > 0;
		const hasMouseCallbacks = this._mouseCancelCallbacks.length > 0;
		const hasTouchCallbacks = this._touchCancelCallbacks.length > 0;

		if (hasPointerCallbacks || hasMouseCallbacks || hasTouchCallbacks) {
			let preventDefault = false;
			const pointerId = ptrId;
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
					target: this.__target ?? this,
					cancelable: true,
					preventDefault: () => {
						preventDefault = true;
					},
				});

				for (const callback of this._pointerCancelCallbacks) {
					callback(event);
				}
			}

			if (hasTouchCallbacks && !preventDefault) {
				const touch = new Touch({
					identifier: ptrId,
					target: this.__target ?? this,
					clientX: x,
					clientY: y,
					screenX: x,
					screenY: y,
					pageX: x,
					pageY: y,
				});
				const touchesList = [touch];
				const touchesById = [];
				touchesById[ptrId] = touch;
				const touches = TouchList.fromList(touchesList);
				const touchEvent = new TouchEvent('touchcancel', {
					touches,
					targetTouches: touches,
					changedTouches: touchesList,
					target: this.__target ?? this,
					cancelable: true,
					preventDefault: () => {
						preventDefault = true;
					},
				});

				for (const callback of this._touchCancelCallbacks) {
					callback(touchEvent);
				}
			}

			if (hasMouseCallbacks && !preventDefault) {
				const event = new MouseEvent('mouseout', {
					clientX: x,
					clientY: y,
					screenX: x,
					screenY: y,
					pageX: x,
					pageY: y,
					target: this.__target ?? this,
					preventDefault: () => {
						preventDefault = true;
					},
				});

				for (const callback of this._mouseCancelCallbacks) {
					callback(event);
				}
			}

			this._lastPointerEventById.set(pointerId, { pointerId, x, y });
		}
	}

	private _pinchCallback(data: { event: string; deltaX: number; deltaY: number; deltaMode: number; pointers: { ptrId: number; x: number; y: number }[]; isInProgress: boolean }) {
		// move callback

		const hasPointerCallbacks = this._pointerMoveCallbacks.length > 0;
		const hasMouseCallbacks = this._mouseMoveCallbacks.length > 0;
		const hasMouseWheel = this._mouseWheelCallbacks.length > 0;
		const hasTouchCallbacks = this._touchMoveCallbacks.length > 0;

		if (hasPointerCallbacks || hasMouseCallbacks || hasMouseWheel || hasTouchCallbacks) {
			const changedTouches = hasTouchCallbacks ? TouchList.empty() : null;
			let preventDefault = false;
			for (const pointer of data.pointers) {
				const pointerId = pointer.ptrId;

				const previousEvent = this._lastPointerEventById.get(pointerId) ?? { pointerId, x: 0, y: 0 };

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
						target: this.__target ?? this,
						cancelable: true,
						preventDefault: () => {
							preventDefault = true;
						},
					});

					for (const callback of this._pointerMoveCallbacks) {
						callback(event);
					}
				}

				if (hasTouchCallbacks && !preventDefault) {
					changedTouches.push(
						new Touch({
							identifier: pointer.ptrId,
							target: this.__target ?? this,
							clientX: pointer.x,
							clientY: pointer.y,
							screenX: pointer.x,
							screenY: pointer.y,
							pageX: pointer.x,
							pageY: pointer.y,
						})
					);
				}

				this._lastPointerEventById.set(pointerId, { pointerId, x: pointer.x, y: pointer.y });
			}

			if ((changedTouches?.length ?? 0) > 0) {
				const touches = TouchList.fromList(this._touches);

				const event = new TouchEvent('touchmove', {
					touches,
					targetTouches: touches,
					changedTouches,
					target: this.__target ?? this,
					cancelable: true,
					preventDefault: () => {
						preventDefault = true;
					},
				});

				for (const callback of this._touchMoveCallbacks) {
					callback(event);
				}
			}

			if (!preventDefault && hasMouseCallbacks) {
				for (const pointer of data.pointers) {
					const pointerId = pointer.ptrId;

					const previousEvent = this._lastPointerEventById.get(pointerId) ?? { pointerId, x: 0, y: 0 };

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
						target: this.__target ?? this,
						preventDefault: () => {
							preventDefault = true;
						},
					});

					for (const callback of this._mouseMoveCallbacks) {
						callback(event);
					}
				}
			}

			if (hasMouseWheel) {
				const event = new WheelEvent('wheel', {
					deltaX: data.deltaX,
					deltaY: data.deltaY,
					deltaZ: 0,
					deltaMode: data.deltaMode,
					target: this.__target ?? this,
				});

				for (const callback of this._mouseWheelCallbacks) {
					callback(event);
				}
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

	_readyEvent() {
		this.notify({
			eventName: 'ready',
			object: this,
		});
	}

	getAttribute(attrib) {
		return this[attrib];
	}

	setAttribute(attrib) {
		this[attrib] = attrib;
	}

	public abstract getContext(type: string, contextAttributes?: any): CanvasRenderingContext | null;

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

	setPointerCapture(id) {}

	releasePointerCapture(id) {}

	getRootNode() {
		return this;
	}
}
