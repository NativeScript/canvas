package org.nativescript.canvas

enum class CanvasFit(val value: Int) {
	None(0),
	Fill(1),
	FitX(2),
	FitY(3),
	ScaleDown(4);

	companion object {
		fun fromNative(value: Int): CanvasFit? {
			return when (value) {
				0 -> None
				1 -> Fill
				2 -> FitX
				3 -> FitY
				4 -> ScaleDown
				else -> null
			}
		}
	}
}

