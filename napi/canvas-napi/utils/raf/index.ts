import { queueMacrotask } from '../macrotask-scheduler.ts';
import { FPSCallback } from './fps-meter.ts';

/**
 * @returns {function(): number} The current time in milliseconds.
 */
const time = (<any>('global' in globalThis ? global : globalThis)).__time || Date.now;

export function getTimeInFrameBase() {
	return time();
}

export interface FrameRequestCallback {
	(time: number): void;
}

type AnimationFrameCallbacks = { [key: string]: FrameRequestCallback };

let animationId = 0;
let currentFrameAnimationCallbacks: AnimationFrameCallbacks = {}; // requests that were scheduled in this frame and must be called ASAP
let currentFrameScheduled = false;

let nextFrameAnimationCallbacks: AnimationFrameCallbacks = {}; // requests there were scheduled in another request and must be called in the next frame
let shouldStop = true;
let inAnimationFrame = false;
let fpsCallback: FPSCallback;
let lastFrameTime = 0;

function getNewId() {
	return animationId++;
}

function ensureNative() {
	if (fpsCallback) {
		return;
	}
	fpsCallback = new FPSCallback(doFrame);
}

function callAnimationCallbacks(thisFrameCbs: AnimationFrameCallbacks, frameTime: number) {
	inAnimationFrame = true;
	for (const animationId in thisFrameCbs) {
		if (thisFrameCbs[animationId]) {
			try {
				thisFrameCbs[animationId](frameTime);
			} catch (err) {
				//	const msg = err ? err.stack || err : err;
			}
		}
	}
	inAnimationFrame = false;
}

function doCurrentFrame() {
	// if we're not getting accurate frame times
	// set last frame time as the current time
	if (!fpsCallback || !fpsCallback.running) {
		lastFrameTime = getTimeInFrameBase();
	}
	currentFrameScheduled = false;
	const thisFrameCbs = currentFrameAnimationCallbacks;
	currentFrameAnimationCallbacks = {};
	callAnimationCallbacks(thisFrameCbs, lastFrameTime);
}

function doFrame(currentTimeMillis: number) {
	lastFrameTime = currentTimeMillis;
	shouldStop = true;
	const thisFrameCbs = nextFrameAnimationCallbacks;
	nextFrameAnimationCallbacks = {};
	callAnimationCallbacks(thisFrameCbs, lastFrameTime);
	if (shouldStop) {
		fpsCallback.stop(); // TODO: check performance without stopping to allow consistent frame times
	}
}

function ensureCurrentFrameScheduled() {
	if (!currentFrameScheduled) {
		currentFrameScheduled = true;
		queueMacrotask(doCurrentFrame);
	}
}

/**
 *
 * @param {Function} callback
 * @returns {Function}
 */
const zonedCallback = function (callback: any) {
	if ((global as any).zone) {
		// Zone v0.5.* style callback wrapping
		return (global as any).zone.bind(callback);
	}
	if ((global as any).Zone) {
		// Zone v0.6.* style callback wrapping
		return (global as any).Zone.current.wrap(callback);
	} else {
		return callback;
	}
};

export function requestAnimationFrame(cb: FrameRequestCallback): number {
	const animId = getNewId();
	if (!inAnimationFrame) {
		ensureCurrentFrameScheduled();
		currentFrameAnimationCallbacks[animId] = zonedCallback(cb);
		return animId;
	}
	ensureNative();
	nextFrameAnimationCallbacks[animId] = zonedCallback(cb);
	shouldStop = false;
	fpsCallback.start();

	return animId;
}

export function cancelAnimationFrame(id: number) {
	delete currentFrameAnimationCallbacks[id];
	delete nextFrameAnimationCallbacks[id];
}
