package org.nativescript.canvas

/**
 * Created by triniwiz on 2019-08-11
 */
class TNSPath2D {
	internal var path: Long

	constructor() {
		path = nativeInit()
	}

	constructor(path2D: TNSPath2D) {
		path = nativeCreateWithPath(path2D.path)
	}

	constructor(data: String) {
		path = nativeCreateWithString(data)
	}

	fun addPath(path2D: TNSPath2D) {
		path = nativeAddPath(path, path2D.path)
	}

	fun addPath(path2D: TNSPath2D, matrix: TNSDOMMatrix) {
		nativeAddPathWithMatrix(path, path2D.path, matrix.matrix)
	}

	fun closePath() {
		nativeClosePath(path)
	}

	fun moveTo(x: Float, y: Float) {
		nativeMoveTo(path, x, y)
	}

	fun rect(x: Float, y: Float, width: Float, height: Float) {
		nativeRect(path, x, y, width, height)
	}

	fun lineTo(x: Float, y: Float) {
		nativeLineTo(path, x, y)
	}

	fun arc(
		x: Float,
		y: Float,
		radius: Float,
		startAngle: Float,
		endAngle: Float,
		anticlockwise: Boolean
	) {
		nativeArc(path, x, y, radius, startAngle, endAngle, anticlockwise)
	}

	fun arcTo(x1: Float, y1: Float, x2: Float, y2: Float, radius: Float) {
		nativeArcTo(path, x1, y1, x2, y2, radius)
	}

	fun bezierCurveTo(cp1x: Float, cp1y: Float, cp2x: Float, cp2y: Float, x: Float, y: Float) {
		nativeBezierCurveTo(path, cp1x, cp1y, cp2x, cp2y, x, y)
	}

	fun ellipse(
		x: Float,
		y: Float,
		radiusX: Float,
		radiusY: Float,
		rotation: Float,
		startAngle: Float,
		endAngle: Float,
		anticlockwise: Boolean
	) {
		nativeEllipse(path, x, y, radiusX, radiusY, rotation, startAngle, endAngle, anticlockwise)
	}

	fun quadraticCurveTo(cpx: Float, cpy: Float, x: Float, y: Float) {
		nativeQuadraticCurveTo(path, cpx, cpy, x, y)
	}

	@Throws(Throwable::class)
	protected fun finalize() {
		nativeDestroy(path)
		path = 0
	}

	companion object {
		@JvmStatic
		private external fun nativeInit(): Long

		@JvmStatic
		private external fun nativeCreateWithPath(path: Long): Long

		@JvmStatic
		private external fun nativeCreateWithString(data: String): Long

		@JvmStatic
		private external fun nativeAddPath(path: Long, pathSrc: Long): Long

		@JvmStatic
		private external fun nativeAddPathWithMatrix(path: Long, pathSrc: Long, matrix: Long)

		@JvmStatic
		private external fun nativeClosePath(path: Long)

		@JvmStatic
		private external fun nativeRect(
			path: Long,
			x: Float,
			y: Float,
			width: Float,
			height: Float
		)

		@JvmStatic
		private external fun nativeMoveTo(path: Long, x: Float, y: Float)

		@JvmStatic
		private external fun nativeLineTo(path: Long, x: Float, y: Float)

		@JvmStatic
		private external fun nativeArc(
			path: Long,
			x: Float,
			y: Float,
			radius: Float,
			startAngle: Float,
			endAngle: Float,
			anticlockwise: Boolean
		)

		@JvmStatic
		private external fun nativeArcTo(
			path: Long,
			x1: Float,
			y1: Float,
			x2: Float,
			y2: Float,
			radius: Float
		)

		@JvmStatic
		private external fun nativeBezierCurveTo(
			path: Long,
			cp1x: Float,
			cp1y: Float,
			cp2x: Float,
			cp2y: Float,
			x: Float,
			y: Float
		)

		@JvmStatic
		private external fun nativeEllipse(
			path: Long,
			x: Float,
			y: Float,
			radiusX: Float,
			radiusY: Float,
			rotation: Float,
			startAngle: Float,
			endAngle: Float,
			anticlockwise: Boolean
		)

		@JvmStatic
		private external fun nativeQuadraticCurveTo(
			path: Long,
			cpx: Float,
			cpy: Float,
			x: Float,
			y: Float
		)

		@JvmStatic
		private external fun nativeDestroy(path: Long)
	}
}
