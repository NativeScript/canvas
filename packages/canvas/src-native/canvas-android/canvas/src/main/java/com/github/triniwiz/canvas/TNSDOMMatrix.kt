package com.github.triniwiz.canvas

/**
 * Created by triniwiz on 3/27/20
 */
class TNSDOMMatrix(internal var matrix: Long) {
	constructor() : this(nativeInit())

	var a: Float
		set(value) {
			nativeSetA(matrix, value)
		}
		get() {
			return nativeA(matrix)
		}

	var b: Float
		set(value) {
			nativeSetB(matrix, value)
		}
		get() {
			return nativeB(matrix)
		}

	var c: Float
		set(value) {
			nativeSetC(matrix, value)
		}
		get() {
			return nativeC(matrix)
		}

	var d: Float
		set(value) {
			nativeSetD(matrix, value)
		}
		get() {
			return nativeD(matrix)
		}

	var e: Float
		set(value) {
			nativeSetE(matrix, value)
		}
		get() {
			return nativeE(matrix)
		}

	var f: Float
		set(value) {
			nativeSetF(matrix, value)
		}
		get() {
			return nativeF(matrix)
		}

	var m11: Float
		set(value) {
			nativeSetM11(matrix, value)
		}
		get() {
			return nativeM11(matrix)
		}

	var m12: Float
		set(value) {
			nativeSetM12(matrix, value)
		}
		get() {
			return nativeM12(matrix)
		}

	var m13: Float
		set(value) {
			nativeSetM13(matrix, value)
		}
		get() {
			return nativeM13(matrix)
		}

	var m14: Float
		set(value) {
			nativeSetM14(matrix, value)
		}
		get() {
			return nativeM14(matrix)
		}


	var m21: Float
		set(value) {
			nativeSetM21(matrix, value)
		}
		get() {
			return nativeM21(matrix)
		}

	var m22: Float
		set(value) {
			nativeSetM22(matrix, value)
		}
		get() {
			return nativeM22(matrix)
		}

	var m23: Float
		set(value) {
			nativeSetM23(matrix, value)
		}
		get() {
			return nativeM23(matrix)
		}

	var m24: Float
		set(value) {
			nativeSetM24(matrix, value)
		}
		get() {
			return nativeM24(matrix)
		}


	var m31: Float
		set(value) {
			nativeSetM31(matrix, value)
		}
		get() {
			return nativeM31(matrix)
		}

	var m32: Float
		set(value) {
			nativeSetM32(matrix, value)
		}
		get() {
			return nativeM32(matrix)
		}

	var m33: Float
		set(value) {
			nativeSetM33(matrix, value)
		}
		get() {
			return nativeM33(matrix)
		}

	var m34: Float
		set(value) {
			nativeSetM34(matrix, value)
		}
		get() {
			return nativeM34(matrix)
		}


	var m41: Float
		set(value) {
			nativeSetM41(matrix, value)
		}
		get() {
			return nativeM41(matrix)
		}

	var m42: Float
		set(value) {
			nativeSetM42(matrix, value)
		}
		get() {
			return nativeM42(matrix)
		}

	var m43: Float
		set(value) {
			nativeSetM43(matrix, value)
		}
		get() {
			return nativeM43(matrix)
		}

	var m44: Float
		set(value) {
			nativeSetM44(matrix, value)
		}
		get() {
			return nativeM44(matrix)
		}

	@Synchronized
	@Throws(Throwable::class)
	protected fun finalize() {
		nativeDestroy(matrix)
	}

	companion object {
		@JvmStatic
		private external fun nativeInit(): Long

		@JvmStatic
		private external fun nativeDestroy(matrix: Long)

		@JvmStatic
		private external fun nativeUpdate(matrix: Long, data: FloatArray)

		@JvmStatic
		private external fun nativeA(matrix: Long): Float

		@JvmStatic
		private external fun nativeSetA(matrix: Long, a: Float)

		@JvmStatic
		private external fun nativeB(matrix: Long): Float

		@JvmStatic
		private external fun nativeSetB(matrix: Long, b: Float)

		@JvmStatic
		private external fun nativeC(matrix: Long): Float

		@JvmStatic
		private external fun nativeSetC(matrix: Long, c: Float)

		@JvmStatic
		private external fun nativeD(matrix: Long): Float

		@JvmStatic
		private external fun nativeSetD(matrix: Long, d: Float)

		@JvmStatic
		private external fun nativeE(matrix: Long): Float

		@JvmStatic
		private external fun nativeSetE(matrix: Long, e: Float)

		@JvmStatic
		private external fun nativeF(matrix: Long): Float

		@JvmStatic
		private external fun nativeSetF(matrix: Long, F: Float)

		@JvmStatic
		private external fun nativeM11(matrix: Long): Float

		@JvmStatic
		private external fun nativeSetM11(matrix: Long, a: Float)

		@JvmStatic
		private external fun nativeM12(matrix: Long): Float

		@JvmStatic
		private external fun nativeSetM12(matrix: Long, b: Float)

		@JvmStatic
		private external fun nativeM13(matrix: Long): Float

		@JvmStatic
		private external fun nativeSetM13(matrix: Long, c: Float)

		@JvmStatic
		private external fun nativeM14(matrix: Long): Float

		@JvmStatic
		private external fun nativeSetM14(matrix: Long, d: Float)

		@JvmStatic
		private external fun nativeM21(matrix: Long): Float

		@JvmStatic
		private external fun nativeSetM21(matrix: Long, a: Float)

		@JvmStatic
		private external fun nativeM22(matrix: Long): Float

		@JvmStatic
		private external fun nativeSetM22(matrix: Long, b: Float)

		@JvmStatic
		private external fun nativeM23(matrix: Long): Float

		@JvmStatic
		private external fun nativeSetM23(matrix: Long, c: Float)

		@JvmStatic
		private external fun nativeM24(matrix: Long): Float

		@JvmStatic
		private external fun nativeSetM24(matrix: Long, d: Float)

		@JvmStatic
		private external fun nativeM31(matrix: Long): Float

		@JvmStatic
		private external fun nativeSetM31(matrix: Long, a: Float)

		@JvmStatic
		private external fun nativeM32(matrix: Long): Float

		@JvmStatic
		private external fun nativeSetM32(matrix: Long, b: Float)

		@JvmStatic
		private external fun nativeM33(matrix: Long): Float

		@JvmStatic
		private external fun nativeSetM33(matrix: Long, c: Float)

		@JvmStatic
		private external fun nativeM34(matrix: Long): Float

		@JvmStatic
		private external fun nativeSetM34(matrix: Long, d: Float)

		@JvmStatic
		private external fun nativeM41(matrix: Long): Float

		@JvmStatic
		private external fun nativeSetM41(matrix: Long, a: Float)

		@JvmStatic
		private external fun nativeM42(matrix: Long): Float

		@JvmStatic
		private external fun nativeSetM42(matrix: Long, b: Float)

		@JvmStatic
		private external fun nativeM43(matrix: Long): Float

		@JvmStatic
		private external fun nativeSetM43(matrix: Long, c: Float)

		@JvmStatic
		private external fun nativeM44(matrix: Long): Float

		@JvmStatic
		private external fun nativeSetM44(matrix: Long, d: Float)

	}
}
