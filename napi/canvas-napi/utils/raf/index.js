const { queueMacrotask } = require('../macrotask-scheduler');
const { FPSCallback } = require('./fps-meter');

/**
 * @returns {function(): number} The current time in milliseconds.
 */
const time = global.__time || Date.now;

function getTimeInFrameBase() {
	return time();
}

let animationId = 0;
/**
 * @type { [key: string]: function(time: number): void }
 */
let currentFrameAnimationCallbacks = {}; // requests that were scheduled in this frame and must be called ASAP
let currentFrameScheduled = false;

/**
 * @type { [key: string]: function(time: number): void }
 */
let nextFrameAnimationCallbacks = {}; // requests there were scheduled in another request and must be called in the next frame
let shouldStop = true;
let inAnimationFrame = false;
/**
 * @type {FPSCallback}
 */
let fpsCallback;
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

/**
 *
 * @param { [key: string]: function(time: number): void } thisFrameCbs
 * @param {number} frameTime
 *
 */
function callAnimationCallbacks(thisFrameCbs, frameTime) {
	inAnimationFrame = true;
	for (const animationId in thisFrameCbs) {
		if (thisFrameCbs[animationId]) {
			try {
				thisFrameCbs[animationId](frameTime);
			} catch (err) {
				const msg = err ? err.stack || err : err;
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

/**
 *
 * @param  {number} currentTimeMillis
 */
function doFrame(currentTimeMillis) {
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
const zonedCallback = function (callback) {
	if (global.zone) {
		// Zone v0.5.* style callback wrapping
		return global.zone.bind(callback);
	}
	if (global.Zone) {
		// Zone v0.6.* style callback wrapping
		return global.Zone.current.wrap(callback);
	} else {
		return callback;
	}
};

/**
 *
 * @param {function(time: number): void} cb
 * @returns {number}
 */
function requestAnimationFrame(cb) {
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

/**
 *
 * @param {number} id
 */
function cancelAnimationFrame(id) {
	delete currentFrameAnimationCallbacks[id];
	delete nextFrameAnimationCallbacks[id];
}

module.exports.requestAnimationFrame = requestAnimationFrame;
module.exports.cancelAnimationFrame = cancelAnimationFrame;
module.exports.time = time;
