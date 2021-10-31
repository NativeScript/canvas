import { CSSType, PercentLength, View, Screen, GestureStateTypes, Utils, Application, Property, booleanConverter } from '@nativescript/core';
import { CanvasRenderingContext } from '../common';
import { Pointer, TouchGestureEventData } from '@nativescript/core/ui/gestures';

export interface ICanvasBase {
	on(eventName: 'ready', callback: (data: any) => void, thisArg?: any): void;
}


export class TouchEvent {
	readonly type: string;
	constructor(name, init?: { [key: string]: any }) {
		this.type = name;
		if (init && typeof init === 'object') {
			Object.keys(init).forEach(key => {
				this[key] = init[key];
			});
		}
	}
	preventDefault() { }
	stopPropagation() { }
}

export class PointerEvent {
	readonly type: string;
	constructor(name, init?: { [key: string]: any }) {
		this.type = name;
		if (init && typeof init === 'object') {
			Object.keys(init).forEach(key => {
				this[key] = init[key];
			});
		}
	}
	preventDefault() { }
	stopPropagation() { }
}

const WEB_GESTURE_EVENTS = ['touchmove', 'touchstart', 'touchcancel', 'touchend', 'change', 'pointerup', 'pointerdown', 'pointermove', 'pointercancel'];


export const ignorePixelScalingProperty = new Property<CanvasBase, boolean>({
	name: 'ignorePixelScaling',
	defaultValue: false,
	valueConverter: booleanConverter
});

let pointerId = 0;
@CSSType('Canvas')
export abstract class CanvasBase extends View implements ICanvasBase {
	public static readyEvent = 'ready';
	ignorePixelScaling: boolean;
	_isCustom: boolean = false;

	_classList: Set<any>;

	private _gesturesRegistered = false;
	private __touchStart?: Pointer;
	private _isPinching = false;

	private _previousX = 0;
	private _previousY = 0;
	private _previousPinchDistance = 0;
	private _previousPointerCount = 0;

	private _pointers: { id: number, coords: { x: number, y: number } }[] = [];

	protected constructor() {
		super();
		this._classList = new Set();
	}


	public addEventListener(arg: string, callback: any, thisArg?: any) {
		super.addEventListener(arg, callback, thisArg);
		if (WEB_GESTURE_EVENTS.indexOf(arg) !== -1) {
			this.__ensureGestures();
		}
	}

	public removeEventListener(arg: string, callback: any, thisArg?: any) {
		super.removeEventListener(arg, callback, thisArg);
		if (WEB_GESTURE_EVENTS.indexOf(arg) !== -1) {
			// if we dont have any other web gestures we can unregister gestures
			if (!WEB_GESTURE_EVENTS.some(e => this.hasListeners(e))) {
				this.__unregisterGestures();
			}
		}
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
		if (!contextOpts) {
			if (type === '2d') {
				return {
					alpha: true,
				};
			}
			if (type.indexOf('webgl') > -1) {
				return defaultGLOptions;
			}
		}
		if (type === '2d') {
			if (contextOpts.alpha !== undefined && typeof contextOpts.alpha === 'boolean') {
				return contextOpts;
			} else {
				return { alpha: true };
			}
		}
		const setIfDefined = (prop, value) => {
			const property = defaultGLOptions[prop];
			if (property !== undefined && typeof value === typeof property) {
				defaultGLOptions[prop] = value;
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
			return defaultGLOptions;
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
		if (typeof value === 'string') {
			value = PercentLength.parse(value);
		}
		if (typeof value === 'number') {
			// treat as px
			return value || 0;
		} else if ((value !== null || true) && typeof value === 'object' && typeof value.value && typeof value.unit) {
			if (value.unit === 'px') {
				return value.value || 0;
			} else if (value.unit === 'dip') {
				return Utils.layout.toDevicePixels(value.value) || 0;
			} else if (value.unit === '%') {
				if (Application.orientation() === 'portrait') {
					return type === 'width' ? Screen.mainScreen.widthPixels : Screen.mainScreen.heightPixels * value.value || 0;
				} else if (Application.orientation() === 'landscape') {
					return type === 'width' ? Screen.mainScreen.widthPixels : Screen.mainScreen.heightPixels * value.value || 0;
				} else {
					return 0;
				}
			} else {
				return 0;
			}
		} else {
			return 0;
		}
	}


	setPointerCapture() { }

	releasePointerCapture() { }

	__ensureGestures() {
		if (!this._gesturesRegistered) {
			this._gesturesRegistered = true;
			this.on(global.isAndroid ? 'touch, pan' : 'touch, pan, pinch', this._touchEvent, this);
		}
	}

	__unregisterGestures() {
		if (this._gesturesRegistered) {
			this._gesturesRegistered = false;
			this.off(global.isAndroid ? 'touch, pan' : 'touch, pan, pinch', this._touchEvent, this);
		}
	}

	_pointerCountFromEvent(event) {
		let count = 0;
		if (global.isIOS) {
			count = event.getPointerCount();
		} else if (global.isAndroid) {
			count = (event.android.current || event.android.initial || event.android).getPointerCount?.() ?? 0;
		}
		return count;
	}

	_positionsFromEvent(event) {
		const positions = [];

		if (global.isiOS) {
			const pointers = event.getAllPointers();
			const count = pointers.length;

			for (let i = 0; i < count; i++) {
				const point = pointers[i];
				positions.push(point.getX(), point.getY(),
				);
			}
		} else if (global.isAndroid) {
			const motionEvent = event.android?.current ?? event.android;
			const count = motionEvent.getPointerCount();
			const coords = new android.view.MotionEvent.PointerCoords();

			for (let i = 0; i < count; i++) {
				motionEvent.getPointerCoords(i, coords);
				positions.push(coords.x / Screen.mainScreen.scale, coords.y / Screen.mainScreen.scale);
				coords.clear();
			}
		}

		return positions;
	}

	_touchEvent(event: any) {
		let extraData;

		let hasPointerDown = false;
		let hasTouchStart = false;

		let hasPointerMove = false;
		let hasTouchMove = false;

		let hasPointerUp = false;
		let hasTouchEnd = false;

		let hasPointerCancel = false;
		let hasTouchCancel = false;

		if (event.eventName === 'touch') {
			switch (event.action) {
				case 'down':
					hasPointerDown = this.hasListeners('pointerdown');
					hasTouchStart = this.hasListeners('touchstart');
					if (hasPointerDown || hasTouchStart) {
						const numberOfPointers = (<TouchGestureEventData>event).getPointerCount();
						const positions = this._positionsFromEvent(event);
						const x = (<TouchGestureEventData>event).getX();
						const y = (<TouchGestureEventData>event).getY();
						extraData = {
							numberOfPointers,
							positions,
							x,
							y
						}
					}

					if (hasPointerDown && this._previousPointerCount === extraData.numberOfPointers) {
						this._previousPointerCount = extraData.numberOfPointers;
						pointerId++;

						this._pointers[this._previousPointerCount - 1] = {
							id: pointerId,
							coords: {
								x: extraData.x,
								y: extraData.y
							}
						}


						this.notify({
							...this._createPointerEvent('pointerdown', extraData),
							pointerId: pointerId
						});
					}

					if (hasTouchStart) {
						this.notify(
							this._createTouchEvent('touchstart', extraData)
						);
					}

					break;
				case 'up':
					hasPointerUp = this.hasListeners('pointerup');
					hasTouchEnd = this.hasListeners('touchend');

					if (hasPointerUp || hasTouchEnd) {
						const numberOfPointers = (<TouchGestureEventData>event).getPointerCount();
						const positions = this._positionsFromEvent(event);
						const x = (<TouchGestureEventData>event).getX();
						const y = (<TouchGestureEventData>event).getY();
						extraData = {
							numberOfPointers,
							positions,
							x,
							y
						}
					}
					if (hasPointerUp) {
						if (global.isAndroid) {
							this._previousPointerCount -= extraData.numberOfPointers;
						} else if (global.isIOS) {
							this._previousPointerCount -= 1;
						}
						for (let i = 0; i < extraData.numberOfPointers; i++) {
							const x = extraData.positions[i];
							const y = extraData.positions[i + 1];
							this.notify({
								...this._createPointerEvent('pointerup', { ...extraData, x, y }),
								pointerId: this._pointers[i].id
							});
						}
					}

					if (hasTouchEnd) {
						this.notify(
							this._createTouchEvent('touchend', extraData)
						);
					}
					break;
				case 'cancel':
					hasPointerCancel = this.hasListeners('pointercancel');
					hasTouchCancel = this.hasListeners('touchcancel');
					if (hasPointerCancel || hasTouchCancel) {
						const numberOfPointers = (<TouchGestureEventData>event).getPointerCount();
						const positions = this._positionsFromEvent(event);
						const x = (<TouchGestureEventData>event).getX();
						const y = (<TouchGestureEventData>event).getY();
						extraData = {
							numberOfPointers,
							positions,
							x,
							y
						}
					}
					if (hasPointerCancel) {
						if (global.isAndroid) {
							this._previousPointerCount -= extraData.numberOfPointers;
						} else if (global.isIOS) {
							this._previousPointerCount -= 1;
						}
						for (let i = 0; i < extraData.numberOfPointers; i++) {
							const x = extraData.positions[i];
							const y = extraData.positions[i + 1];
							this.notify({
								...this._createPointerEvent('pointercancel', { ...extraData, x, y }),
								pointerId: this._pointers[i].id
							});
						}
					}

					if (hasTouchCancel) {
						this.notify(
							this._createTouchEvent('touchcancel', extraData)
						);
					}
					break;
				case 'move':
					// NOOP
					break;
				default:
					break;
			}
		} else if (event.eventName === 'pinch') {
			const numberOfPointers = this._pointerCountFromEvent(event);
			if (!this._isPinching && numberOfPointers >= 2 && (event.state === GestureStateTypes.began || event.state === GestureStateTypes.changed)) {
				this._previousPinchDistance = 0;
				this._isPinching = true;
			}
			if (this._isPinching) {

				hasPointerMove = this.hasListeners('pointermove');
				hasTouchMove = this.hasListeners('touchmove');

				if (hasPointerMove || hasTouchMove) {
					const positions = this._positionsFromEvent(event);
					const x = global.isIOS ? (<TouchGestureEventData>event).getX() : positions[0];
					const y = global.isIOS ? (<TouchGestureEventData>event).getY() : positions[1];
					extraData = {
						numberOfPointers,
						positions,
						x,
						y
					}
				}


				const dx = extraData.positions[2] - extraData.positions[0];
				const dy = extraData.positions[3] - extraData.positions[1];
				let delta = 0;

				var distance = Math.sqrt(dx * dx + dy * dy);
				if (this._previousPinchDistance) {
					delta = this._previousPinchDistance - distance;
				}
				this._previousPinchDistance = distance;

				const x = event.getFocusX();
				const y = event.getFocusY();
				const scale = event.scale;

				let data = {
					deltaMode: 0,
					clientX: x,
					clientY: y,
					screenX: x,
					screenY: y,
					deltaX: 0,
					deltaY: delta,
					deltaZ: 0,
				};



				if (hasPointerMove) {
					const count = extraData.numberOfPointers;
					const positions = extraData.positions;
					for (let i = 0; i < count; i++) {
						let x = positions[i] - extraData.x;
						let y = positions[i + 1] - extraData.y;
						this.notify({
							...this._createPointerEvent('pointermove', { ...extraData, x, y }),
							pointerId: this._pointers[i].id
						});
					}
				}

				if (hasTouchMove) {
					this.notify(
						this._createTouchEvent('touchmove', { ...extraData, data })
					);
				}


			}
			if (event.state === GestureStateTypes.ended || event.state === GestureStateTypes.cancelled) {
				this._isPinching = false;
			}
		}
		else if (event.eventName === 'pan') {
			if (this._isPinching) {
				return;
			}
			if (event.state === GestureStateTypes.began || event.state === GestureStateTypes.changed) {
				hasPointerMove = this.hasListeners('pointermove');
				hasTouchMove = this.hasListeners('touchmove');
				if (hasPointerMove || hasTouchMove) {
					const numberOfPointers = this._pointerCountFromEvent(event);
					const positions = this._positionsFromEvent(event);
					const x = global.isIOS ? (<TouchGestureEventData>event).getX() : positions[0];
					const y = global.isIOS ? (<TouchGestureEventData>event).getY() : positions[1];
					extraData = {
						numberOfPointers,
						positions,
						x,
						y
					}
				}

				if (hasPointerMove) {
					const positions = extraData.positions;
					for (let i = 0; i < extraData.numberOfPointers; i++) {
						const x = positions[i];
						const y = positions[i + 1];
						this.notify({
							...this._createPointerEvent('pointermove', { numberOfPointers: extraData.numberOfPointers, positions, x, y }),
							pointerId: this._pointers[i].id
						});
					}
				}

				if (hasTouchMove) {
					this.notify(this._createTouchEvent('touchmove', extraData));
				}

			}
		}
	}

	_createTouchEvent(name, extraData): any {
		const count = extraData.numberOfPointers;
		const point = extraData.positions;
		if (!point) {
			return null;
		}
		const x = extraData.x;
		const y = extraData.y;
		const activePointer = {
			clientX: x,
			clientY: y,
			force: 0.0,
			identifier: 0,
			pageX: x,
			pageY: y,
			radiusX: 0,
			radiusY: 0,
			rotationAngle: 0,
			screenX: x,
			screenY: y,
		};

		const pointers = [];
		for (let i = 0; i < count; i++) {
			const x = extraData.positions[i];
			const y = extraData.positions[i + 1];
			pointers.push({
				clientX: x,
				clientY: y,
				force: 0.0,
				identifier: i,
				pageX: x,
				pageY: y,
				radiusX: 0,
				radiusY: 0,
				rotationAngle: 0,
				screenX: x,
				screenY: y,
				target: this,
			});
		}


		return Object.assign(new TouchEvent(name), {
			eventName: name,
			defaultPrevented: false,
			cancelable: false,
			altKey: false,
			ctrlKey: false,
			metaKey: false,
			shiftKey: false,
			target: this,
			changedTouches: pointers,
			targetTouches: pointers,
			touches: pointers,
			...activePointer
		});
	}

	_createPointerEvent(name, extraData): any {
		const x = extraData.x;
		const y = extraData.y;
		const activePointer = {
			clientX: x,
			clientY: y,
			force: 0.0,
			identifier: 0,
			pageX: x,
			pageY: y,
			radiusX: 0,
			radiusY: 0,
			rotationAngle: 0,
			screenX: x,
			screenY: y,
			pointerId: 0,
			pointerType: 'touch',
			x,
			y,
			width: 23.4375,
			height: 23.4375,
			isPrimary: true
		};

		return Object.assign(new PointerEvent(name), {
			eventName: name,
			defaultPrevented: false,
			cancelable: false,
			altKey: false,
			ctrlKey: false,
			metaKey: false,
			shiftKey: false,
			target: this,
			...activePointer
		});
	}
}

ignorePixelScalingProperty.register(CanvasBase);