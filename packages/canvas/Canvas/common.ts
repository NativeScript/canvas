import { CSSType, PercentLength, View, Screen, GestureStateTypes, Utils, Application, Property, booleanConverter } from '@nativescript/core';
import { CanvasRenderingContext, TouchList } from '../common';
import { GestureTypes, GestureEventData, Pointer } from '@nativescript/core/ui/gestures';

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


import { observe as gestureObserve } from '@nativescript-community/gesturehandler/gestures_override';
import { GestureHandlerTouchEvent, GestureTouchEventData, HandlerType, Manager, PanGestureHandler, PinchGestureHandler, TapGestureHandler, GestureStateEventData, GestureHandlerStateEvent, GestureState } from '@nativescript-community/gesturehandler';


export const TOUCH_GESTURE_TAG = 1;
export const PAN_GESTURE_TAG = 2;
export const PINCH_GESTURE_TAG = 3;
let pointerId = 0;

@CSSType('Canvas')
export abstract class CanvasBase extends View implements ICanvasBase {
	public static readyEvent = 'ready';
	ignorePixelScaling: boolean;
	_isCustom: boolean = false;

	protected constructor() {
		super();
		this._classList = new Set();
	}

	__touchStart;
	__touchLast;
	_isPinching = false;
	_isPanning = false;
	_previousX = 0;
	_previousY = 0;
	_previousPinchDistance = 0;

	_panGestureHandler: PanGestureHandler;
	_touchGestureHandler: TapGestureHandler;
	_pinchGestureHandler: PinchGestureHandler;
	_gesturesRegistered = false;

	_pointers: { id: number, coords: { x: number, y: number } }[] = [];

	_findPointer(extraData) {
		return this._pointers.findIndex(item => {
			if (item.coords.x === extraData.x && item.coords.y === extraData.y) {
				return true;
			}
			const oldX = extraData.x + (extraData?.translationX ?? 0)
			const oldY = extraData.y + (extraData?.translationY ?? 0)
			if (oldX === item.coords.x && oldY === item.coords.y) {
				return true
			}
			console.log(
				'_findPointer', item, extraData
			);
			return undefined;
		})
	}

	__ensureGestures() {
		if (!this._gesturesRegistered) {
			this._gesturesRegistered = true;
			//this.on('touch, pan, pinch', this._touchEvent, this);

			const manager = Manager.getInstance();
			this._touchGestureHandler = manager.createGestureHandler(
				HandlerType.TAP, TOUCH_GESTURE_TAG, {
				simultaneousHandlers: [PAN_GESTURE_TAG],
				shouldCancelWhenOutside: false,
			}
			);

			this._panGestureHandler = manager.createGestureHandler(
				HandlerType.PAN, PAN_GESTURE_TAG, {
				simultaneousHandlers: [
					TOUCH_GESTURE_TAG
				]
			}
			);

			this._pinchGestureHandler = manager.createGestureHandler(
				HandlerType.PINCH, PINCH_GESTURE_TAG, {
				simultaneousHandlers: [
					PAN_GESTURE_TAG
				],
				shouldCancelWhenOutside: false,
				minPointers: 2,
			}
			);

			if (this.isLoaded) {
				this._listenToGestures();
			}

		}
	}

	_listenToGestures(remove = false) {
		if (!this._gesturesRegistered) {
			return;
		}
		if (remove) {
			this._touchGestureHandler.off(GestureHandlerTouchEvent, this._onTouchGesture);
			this._touchGestureHandler.off(GestureHandlerStateEvent, this._onTouchGestureState);

			this._touchGestureHandler.detachFromView(this);


			this._panGestureHandler.off(GestureHandlerTouchEvent, this._onPanGesture);
			this._panGestureHandler.off(GestureHandlerStateEvent, this._onPanGestureState);


			this._panGestureHandler.detachFromView(this);

			this._pinchGestureHandler.off(GestureHandlerTouchEvent, this._onPinchGesture);
			this._pinchGestureHandler.off(GestureHandlerStateEvent, this._onPinchGestureState);


			this._pinchGestureHandler.detachFromView(this);

		} else {
			this._touchGestureHandler.on(GestureHandlerTouchEvent, this._onTouchGesture, this);
			this._touchGestureHandler.on(GestureHandlerStateEvent, this._onTouchGestureState, this);

			this._touchGestureHandler.attachToView(this);

			this._panGestureHandler.on(GestureHandlerTouchEvent, this._onPanGesture, this);
			this._panGestureHandler.on(GestureHandlerStateEvent, this._onPanGestureState, this);

			this._panGestureHandler.attachToView(this);

			this._pinchGestureHandler.on(GestureHandlerTouchEvent, this._onPinchGesture, this);
			this._pinchGestureHandler.on(GestureHandlerStateEvent, this._onPinchGestureState, this);

			this._pinchGestureHandler.attachToView(this);
		}

	}

	__unregisterGestures() {
		if (this._gesturesRegistered) {
			this._gesturesRegistered = false;
			this._listenToGestures(true);
			this._panGestureHandler = null;
			this._touchGestureHandler = null;
			this._pinchGestureHandler = null;
			//this.off('touch, pan, pinch', this._touchEvent, this);
		}
	}

	_createTouchEvent(name, extraData, pointData = {}): any {
		const count = extraData.numberOfPointers;
		const point = extraData.positions;
		if (!point) {
			return null;
		}
		const x = extraData.x;
		const y = extraData.y;
		const pointers = [];
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

		let result;
		if (name.indexOf('pointer') !== -1) {
			pointerId++;
			activePointer['pointerId'] = pointerId;
			activePointer['pointerType'] = 'touch';
			activePointer['width'] = 23.4375;
			activePointer['height'] = 23.4375;
			activePointer['isPrimary'] = true;
			activePointer['x'] = x;
			activePointer['y'] = y;
			result = new PointerEvent(name);
		} else {
			result = new TouchEvent(name);
		}

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

		return Object.assign(result, {
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
			target: this,
			...activePointer,
			...pointData
		});
	}

	_updatePointerId(extraData, event, remove = false) {
		const oldPointIndex = this._findPointer(extraData);
		let newPointer = null;
		if (oldPointIndex !== -1) {
			const oldPoint = this._pointers[oldPointIndex];
			console.log('_updatePointerId', oldPoint);
			pointerId--;
			newPointer = { ...event, ...{ pointerId: oldPoint.id } }
			if (remove) {
				console.log(this._pointers);
				console.log('removing', oldPointIndex);
				this._pointers.splice(oldPointIndex, 1);
				console.log(this._pointers)
			}
		}

		return newPointer;
	}

	_onTouchGesture(args: GestureTouchEventData) {
		const { state, extraData } = args.data;
	}

	_onTouchGestureState(args: GestureStateEventData) {
		const { state, extraData } = args.data;
		if (state === GestureState.BEGAN) {
			this.__touchStart = args.data.extraData;
			if (this.hasListeners('pointerdown')) {
				console.log('_onTouchGestureState', 'points', this._pointers);
				const event = this._createTouchEvent('pointerdown', extraData);
				this._pointers.push({
					id: event.pointerId,
					coords: {
						x: event.x,
						y: event.y
					}
				});
				this.notify(event);
			}

			if (this.hasListeners('touchstart')) {
				this.notify(
					this._createTouchEvent('touchstart', extraData)
				);
			}
		}


		if (state === GestureState.END) {
			if (this.hasListeners('pointerup')) {
				const event = this._updatePointerId(extraData, this._createTouchEvent('pointerup', extraData), true);
				if (event) {
					this.notify(event);
				}
			}

			if (this.hasListeners('touchend')) {
				this.notify(
					this._createTouchEvent('touchend', extraData)
				);
			}

			this.__touchStart = undefined;
		}

		if (state === GestureState.CANCELLED || state === GestureState.FAILED) {

			if (this.hasListeners('pointercancel')) {
				const event = this._updatePointerId(extraData, this._createTouchEvent('pointercancel', extraData), true);
				if (event) {
					this.notify(event);
				}
			}

			if (this.hasListeners('touchcancel')) {
				this.notify(
					this._createTouchEvent('touchcancel', extraData)
				);
			}

			this.__touchStart = undefined;
		}
	}


	_onPanGesture(args: GestureTouchEventData) {
		const { state, extraData } = args.data;
		if (state === GestureState.ACTIVE) {
			console.log('_onPanGesture', args.data.state, this._isPinching, this._isPanning, extraData.numberOfPointers);
			const emit = (name) => {
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
					target: this
				};

				let result;
				if (name.indexOf('pointer') !== -1) {
					pointerId++;
					console.log('_onPanGesture', 'pointerId', pointerId);
					activePointer['pointerId'] = pointerId;
					activePointer['pointerType'] = 'touch';
					activePointer['width'] = 100;
					activePointer['height'] = 100;
					activePointer['isPrimary'] = true;
					activePointer['x'] = x;
					activePointer['y'] = y;
					result = new PointerEvent(name);
				} else {
					result = new TouchEvent(name);
				}

				const pointers = [];

				const count = extraData.numberOfPointers;
				for (let i = 0; i < count; i++) {
					const point = extraData.positions;
					const x = point[i];
					const y = point[i + 1]
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

				this._previousX = extraData.translationX;
				this._previousY = extraData.translationY;
				return Object.assign(result, {
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
					target: this,
					...activePointer,
				});
			}

			if (this.hasListeners('pointermove')) {
				const event = this._updatePointerId(extraData, emit('pointermove'));
				if (event) {
					this.notify(event);
				}
			}

			if (this.hasListeners('touchmove')) {
				this.notify(emit('touchmove'));
			}
		}
	}

	_onPanGestureState(args: GestureStateEventData) {
		const { state, extraData } = args.data;

		if (state === GestureState.END) {
			this._isPanning = false;

			console.log('_onPanGestureState', 'pointerup', this.hasListeners('pointerup'), this._pointers);
			if (this.hasListeners('pointerup')) {
				const event = this._updatePointerId(extraData, this._createTouchEvent('pointerup', extraData), true);
				if (event) {
					this.notify(event);
				}
			}
			console.log('after', '_onPanGestureState', 'pointerup', this.hasListeners('pointerup'), this._pointers);

			if (this.hasListeners('touchend')) {
				this.notify(
					this._createTouchEvent('touchend', extraData)
				);
			}

			this.__touchStart = undefined;
			this.__touchLast = undefined;
		} else if (state === GestureState.CANCELLED || state === GestureState.FAILED) {
			this._isPanning = false;

			if (this.hasListeners('pointercancel')) {
				const event = this._updatePointerId(extraData, this._createTouchEvent('pointercancel', extraData), true)
				if (event) {
					this.notify(event);
				}
			}

			if (this.hasListeners('touchcancel')) {
				this.notify(
					this._createTouchEvent('touchcancel', extraData)
				);
			}

			// this.__touchStart = undefined;
			// this.__touchLast = undefined;
		}
	}

	_onPinchGesture(args: GestureTouchEventData) {
		const { state, extraData } = args.data;
		if (state !== GestureState.ACTIVE) {
			return;
		}
		console.log('_onPinchGesture', args.data.state, this._isPinching, this._isPanning, extraData.numberOfPointers);
		this._isPanning = false;
		this._isPinching = true;
		const dx = extraData.positions[2] - extraData.positions[0];
		const dy = extraData.positions[3] - extraData.positions[1];

		let delta = 0;

		var distance = Math.sqrt(dx * dx + dy * dy);
		if (this._previousPinchDistance) {
			delta = this._previousPinchDistance - distance;
		}
		this._previousPinchDistance = distance;
		const x = extraData.focalX;
		const y = extraData.focalY;
		const data = {
			deltaMode: 0,
			clientX: x,
			clientY: y,
			screenX: x,
			screenY: y,
			deltaX: 0,
			deltaY: delta,
			deltaZ: 0,
		};

		if (this.hasListeners('pointermove')) {
			this.notify(
				this._createTouchEvent('pointermove', extraData, data)
			);
		}

		if (this.hasListeners('touchmove')) {
			this.notify(
				this._createTouchEvent('touchmove', extraData, data)
			);
		}

	}

	_onPinchGestureState(args: GestureStateEventData) {
		const { state, extraData } = args.data;
		if (state === GestureState.END) {
			this._isPinching = false;

			if (this.hasListeners('pointerup')) {
				console.log('start', '_onPinchGestureState');
				const event = this._updatePointerId(extraData, this._createTouchEvent('pointerup', extraData), true);
				console.log('ebd', '_onPinchGestureState');
				if (event) {
					this.notify(event);
				}
			}

			if (this.hasListeners('touchend')) {
				this.notify(
					this._createTouchEvent('touchend', extraData)
				);
			}

			this.__touchStart = undefined;
			this.__touchLast = undefined;
		} else if (state === GestureState.CANCELLED || state === GestureState.FAILED) {
			this._isPinching = false;

			if (this.hasListeners('pointercancel')) {
				console.log('pointercancel', args.data.prevState, args.data.state)
				const event = this._updatePointerId(extraData, this._createTouchEvent('pointercancel', extraData), true)
				if (event) {
					this.notify(event);
				}
			}

			if (this.hasListeners('touchcancel')) {
				this.notify(
					this._createTouchEvent('touchcancel', extraData)
				);
			}

			// this.__touchStart = undefined;
			// this.__touchLast = undefined;

		}
	}


	_observe(type: GestureTypes, callback: (args: GestureEventData) => void, thisArg?: any): void {
		if (!this._gestureObservers[type]) {
			this._gestureObservers[type] = [];
		}

		this._gestureObservers[type].push(gestureObserve(this, type, callback, thisArg));
		if (global.isAndroid) {
			if (this.isLoaded && !(<any>this).touchListenerIsSet) {
				this.setOnTouchListener();
			}
		}
	}


	onLoaded() {
		super.onLoaded();
		this._listenToGestures();
	}

	setPointerCapture() {

	}

	releasePointerCapture() {

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

	_touchEvent(event: any) {
		if (event.eventName === 'touch') {
			switch (event.action) {
				case 'down':
					// ensure we dont have multiple touchstart
					// on the web seems to be called only on first touch
					if (!this.__touchStart) {
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
			} else if (event.state === GestureStateTypes.ended) {
				this._emitEvent('touchend', event);
			} else if (event.state === GestureStateTypes.cancelled) {
				this._emitEvent('touchcancel', event);
			}


		}
	}

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
			this._previousX = event.deltaX;
			this._previousY = event.deltaY;
		} else if (name === 'touchmove:pinch') {
			const dx = event.extraData.positions[2] - event.extraData.positions[0];
			const dy = event.extraData.positions[3] - event.extraData.positions[1];
			let delta = 0;

			var distance = Math.sqrt(dx * dx + dy * dy);
			if (this._previousPinchDistance) {
				delta = this._previousPinchDistance - distance;
			}
			this._previousPinchDistance = distance;
			name = 'wheel';
			const x = event.getFocusX();
			const y = event.getFocusY();
			const scale = event.scale;
			// mouse event
			let data = {
				deltaMode: 0,
				clientX: x,
				clientY: y,
				screenX: x,
				screenY: x,
				deltaX: 0,
				deltaY: delta,
				deltaZ: 0,
			};
			return Object.assign(new TouchEvent(name), { eventName: name, object: null, defaultPrevented: false, cancelable: false, altKey: false, ctrlKey: true, metaKey: false, shiftKey: false, target }, data);
		} else {
			const count = event.getAllPointers().length;
			const point = event.getActivePointers()[0];
			if (!point) {
				return null;
			}
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

		return Object.assign(new TouchEvent(name), {
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
		this.notify(this._getTouchEvent(name, event, this) as any);
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

ignorePixelScalingProperty.register(CanvasBase);