import {CSSType, PercentLength, View, Screen, GestureStateTypes, Utils, Application} from '@nativescript/core';
import {CanvasRenderingContext, TouchList} from '../common';
import {GestureTypes, GestureEventData, PinchGestureEventData, Pointer} from '@nativescript/core/ui/gestures';
import {observe as gestureObserve} from '@nativescript-community/gesturehandler/gestures_override';

export interface ICanvasBase {
	on(eventName: 'ready', callback: (data: any) => void, thisArg?: any): void;
}

const WEB_GESTURE_EVENTS = ['touchmove', 'touchstart', 'touchcancel', 'touchend']

@CSSType('Canvas')
export abstract class CanvasBase extends View implements ICanvasBase {
	public static readyEvent = 'ready';
	_isCustom: boolean = false;

	protected constructor() {
		super();
		this._classList = new Set();
	}

	_gesturesRegistered = false;
	__ensureGestures() {
		if (!this._gesturesRegistered) {
			this._gesturesRegistered = true;
			this.on('touch, pan, pinch', this._touchEvent, this);
		}
	}
	__unregisterGestures() {
		if (this._gesturesRegistered) {
			this._gesturesRegistered = false;
			this.off('touch, pan, pinch', this._touchEvent, this);
		}
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
			if (!WEB_GESTURE_EVENTS.some(e=>this.hasListeners(e))){
				this.__unregisterGestures();
			}
		}
	}

	_classList: Set<any>;

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
				return {alpha: true};
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

	__touchStart?: Pointer;

	_isPinching = false;
	_touchEvent(event: any) {
		if (event.eventName === 'touch') {
			switch (event.action) {
				case 'down':
					// ensure we dont have multiple touchstart
					// on the web seems to be called only on first touch
					if (! this.__touchStart) {
						this.__touchStart = event.getActivePointers()[0];
						this._emitEvent('touchstart', event);
					}
					break;
				case 'up':
					if (event.getPointerCount() === 1 && this.__touchStart) {
						this._emitEvent('touchend', event);
						this._isPinching = false;
						this.__touchStart = undefined;
					}
					break;
				case 'cancel':
					this._emitEvent('touchcancel', event);
					this.__touchStart = undefined;
					break;
				case 'move':
					// NOOP
					break;
				default:
					break;
			}
		} else if (event.eventName === 'pinch') {
			if (!this._isPinching && event.getPointerCount() >= 2 && (event.state === GestureStateTypes.began || event.state === GestureStateTypes.changed)) {
				this._previousPinchDistance = 0;
				this._isPinching = true;
			}
			if (this._isPinching) {
				this._emitEvent('touchmove:pinch', event);
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
				this._emitEvent('touchmove', event);
			}
		}
	}

	_previousX = 0;
	_previousY = 0;
	_previousPinchDistance = 0;

	_getTouchEvent(name, event, target): TouchEvent {
		const pointers = new TouchList();
		let scale = 1;
		let activePointer: {};

		if (name === 'touchmove') {
			name = 'touchmove';

			activePointer = {
				clientX: event.getX() * scale,
				clientY: event.getY() * scale,
				force: 0.0,
				identifier: 0,
				pageX: event.getX() * scale,
				pageY: event.getY() * scale,
				radiusX: 0,
				radiusY: 0,
				rotationAngle: 0,
				screenX: event.getX() * scale,
				screenY: event.getY() * scale,
				target
			};

			const count = event.getAllPointers().length;
			for (let i = 0; i < count; i++) {
				const point = event.getAllPointers()[i];
				pointers.push({
					clientX: point.getX(),
					clientY: point.getY(),
					force: 0.0,
					identifier: i,
					pageX: point.getX(),
					pageY: point.getY(),
					radiusX: 0,
					radiusY: 0,
					rotationAngle: 0,
					screenX: point.getX(),
					screenY: point.getY(),
					target,
				});
			}

			/* mouse */
			// pointers.push({
			// 	// * SCALE ??
			// 	clientX: x,
			// 	clientY: y,
			// 	force: 0.0,
			// 	identifier: 0,
			// 	pageX: x,
			// 	pageY: y,
			// 	radiusX: 0,
			// 	radiusY: 0,
			// 	rotationAngle: 0,
			// 	screenX: x,
			// 	screenY: y,
			// 	target,
			// });
			this._previousX = event.deltaX;
			this._previousY = event.deltaY;
		} else if (name === 'touchmove:pinch') {
			const dx = event.extraData.positions[2] - event.extraData.positions[0];
			const dy = event.extraData.positions[3] - event.extraData.positions[1];
			let delta =0 ;

			var distance = Math.sqrt(dx * dx + dy * dy);
			if (this._previousPinchDistance) {
				delta  = this._previousPinchDistance - distance;
			}
			this._previousPinchDistance= distance;
			name = 'wheel';
			const x = event.getFocusX();
			const y = event.getFocusY();
			const scale = event.scale;
			// mouse event
			let data = {
				deltaMode:0,
				clientX: x,
				clientY: y,
				screenX: x ,
				screenY: x,
				deltaX:0,
				deltaY:delta,
				deltaZ:0,
			};
			return Object.assign(new TouchEvent(name), { eventName: name, object: null, defaultPrevented: false, cancelable: false, altKey: false, ctrlKey: true, metaKey: false, shiftKey: false, target }, data);
		} else {
			const count = event.getAllPointers().length;
			const point = event.getActivePointers()[0];

			// mouse event
			activePointer = {
				clientX: point.getX() * scale,
				clientY: point.getY() * scale,
				force: 0.0,
				identifier: 0,
				pageX: point.getX() * scale,
				pageY: point.getY() * scale,
				radiusX: 0,
				radiusY: 0,
				rotationAngle: 0,
				screenX: point.getX() * scale,
				screenY: point.getY() * scale,
			};

			for (let i = 0; i < count; i++) {
				const point = event.getAllPointers()[i];
				pointers.push({
					clientX: point.getX(),
					clientY: point.getY(),
					force: 0.0,
					identifier: i,
					pageX: point.getX(),
					pageY: point.getY(),
					radiusX: 0,
					radiusY: 0,
					rotationAngle: 0,
					screenX: point.getX(),
					screenY: point.getY(),
					target,
				});
			}
		}

		return Object.assign(new TouchEvent(name),{
			eventName: name,
			defaultPrevented: false,
			cancelable: false,
			altKey: false,
			changedTouches: pointers,
			ctrlKey: false,
			metaKey: false,
			shiftKey: false,
			targetTouches: pointers,
			touches: pointers,
			target,
			...activePointer,
		});
	}

	_emitEvent(name, event) {
		this.notify(this._getTouchEvent(name, event, this) as any );
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
}

(CanvasBase.prototype as any)._observe = function (type: GestureTypes, callback: (args: GestureEventData) => void, thisArg?: any): void {
	if (!this._gestureObservers[type]) {
		this._gestureObservers[type] = [];
	}

	this._gestureObservers[type].push(gestureObserve(this, type, callback, thisArg));
	if (global.isAndroid) {
		if (this.isLoaded && !this.touchListenerIsSet) {
			this.setOnTouchListener();
		}
	}
};