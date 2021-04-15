package org.nativescript.canvas

/**
 * Created by triniwiz on 2019-07-13
 */
enum class TNSCompositeOperationType(private var type: String, private var value: Int) {
	SourceOver("source-over", 0), SourceIn("source-in", 1), SourceOut(
		"source-out",
		2
	),
	SourceAtop("source-atop", 3), DestinationOver(
		"destination-over", 4
	),
	DestinationIn("destination-in", 5), DestinationOut(
		"destination-out",
		6
	),
	DestinationAtop("destination-atop", 7), Lighter(
		"lighter", 8
	),
	Copy("copy", 9), Xor("xor", 10), Multiply("multiply", 11), Screen(
		"screen",
		12
	),
	Overlay("overlay", 13), Darken("darken", 14), Lighten(
		"lighten", 15
	),
	ColorDodge("color-dodge", 16), ColorBurn("color-burn", 17), HardLight(
		"hard-light",
		18
	),
	SoftLight("soft-light", 19), Difference(
		"difference", 20
	),
	Exclusion("exclusion", 21), Hue("hue", 22), Saturation("saturation", 23), Color(
		"color",
		24
	),
	Luminosity("luminosity", 25);

	override fun toString(): String {
		return type
	}

	fun toNative(): Int {
		return value
	}

	internal var isError = false

	companion object {
		fun fromNative(value: Int): TNSCompositeOperationType {
			return when (value) {
				0 -> SourceOver
				1 -> SourceIn
				2 -> SourceOut
				3 -> SourceAtop
				4 -> DestinationOver
				5 -> DestinationIn
				6 -> DestinationOut
				7 -> DestinationAtop
				8 -> Lighter
				9 -> Copy
				10 -> Multiply
				11 -> Xor
				12 -> Screen
				13 -> Overlay
				14 -> Darken
				15 -> Lighten
				16 -> ColorDodge
				17 -> ColorBurn
				18 -> HardLight
				19 -> SoftLight
				20 -> Difference
				21 -> Exclusion
				22 -> Hue
				23 -> Saturation
				24 -> Luminosity
				else -> SourceOver.apply { isError = true }
			}
		}
	}

}
