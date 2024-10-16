import { Quaternion } from './Quaternion.js';

/**
 * @see {@link https://github.com/mrdoob/three.js/blob/master/src/math/MathUtils.js|src/math/MathUtils.js}
 */

export const DEG2RAD: number;

export const RAD2DEG: number;

export function generateUUID(): string;

/**
 * Clamps the x to be between a and b.
 *
 * @param value Value to be clamped.
 * @param min Minimum value.
 * @param max Maximum value.
 */
export function clamp(value: number, min: number, max: number): number;

export function euclideanModulo(n: number, m: number): number;

/**
 * Linear mapping of x from range [a1, a2] to range [b1, b2].
 *
 * @param x Value to be mapped.
 * @param a1 Minimum value for range A.
 * @param a2 Maximum value for range A.
 * @param b1 Minimum value for range B.
 * @param b2 Maximum value for range B.
 */
export function mapLinear(x: number, a1: number, a2: number, b1: number, b2: number): number;

export function inverseLerp(x: number, y: number, t: number): number;

/**
 * Returns a value linearly interpolated from two known points based
 * on the given interval - t = 0 will return x and t = 1 will return y.
 *
 * @param x Start point.
 * @param y End point.
 * @param t interpolation factor in the closed interval [0, 1]
 */
export function lerp(x: number, y: number, t: number): number;

/**
 * Smoothly interpolate a number from x toward y in a spring-like
 * manner using the dt to maintain frame rate independent movement.
 *
 * @param x Current point.
 * @param y Target point.
 * @param lambda A higher lambda value will make the movement more sudden, and a lower value will make the movement more gradual.
 * @param dt Delta time in seconds.
 */
export function damp(x: number, y: number, lambda: number, dt: number): number;

/**
 * Returns a value that alternates between 0 and length.
 *
 * @param x The value to pingpong.
 * @param length The positive value the export function will pingpong to. Default is 1.
 */
export function pingpong(x: number, length?: number): number;

export function smoothstep(x: number, min: number, max: number): number;

export function smootherstep(x: number, min: number, max: number): number;

/**
 * Random integer from low to high interval.
 */
export function randInt(low: number, high: number): number;

/**
 * Random float from low to high interval.
 */
export function randFloat(low: number, high: number): number;

/**
 * Random float from - range / 2 to range / 2 interval.
 */
export function randFloatSpread(range: number): number;

/**
 * Deterministic pseudo-random float in the interval [ 0, 1 ].
 */
export function seededRandom(seed?: number): number;

export function degToRad(degrees: number): number;

export function radToDeg(radians: number): number;

export function isPowerOfTwo(value: number): boolean;

export function ceilPowerOfTwo(value: number): number;

export function floorPowerOfTwo(value: number): number;

export function setQuaternionFromProperEuler(q: Quaternion, a: number, b: number, c: number, order: string): void;

export function denormalize(value: number, array: Float32Array | Uint32Array | Uint16Array | Uint8Array | Int32Array | Int16Array | Int8Array): number;

export function normalize(value: number, array: Float32Array | Uint32Array | Uint16Array | Uint8Array | Int32Array | Int16Array | Int8Array): number;

export const MathUtils: {
	DEG2RAD: typeof DEG2RAD;
	RAD2DEG: typeof RAD2DEG;
	generateUUID: typeof generateUUID;
	clamp: typeof clamp;
	euclideanModulo: typeof euclideanModulo;
	mapLinear: typeof mapLinear;
	inverseLerp: typeof inverseLerp;
	lerp: typeof lerp;
	damp: typeof damp;
	pingpong: typeof pingpong;
	smoothstep: typeof smoothstep;
	smootherstep: typeof smootherstep;
	randInt: typeof randInt;
	randFloat: typeof randFloat;
	randFloatSpread: typeof randFloatSpread;
	seededRandom: typeof seededRandom;
	degToRad: typeof degToRad;
	radToDeg: typeof radToDeg;
	isPowerOfTwo: typeof isPowerOfTwo;
	ceilPowerOfTwo: typeof ceilPowerOfTwo;
	floorPowerOfTwo: typeof floorPowerOfTwo;
	setQuaternionFromProperEuler: typeof setQuaternionFromProperEuler;
	normalize: typeof normalize;
	denormalize: typeof denormalize;
};
