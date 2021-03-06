package org.nativescript.canvas

enum class HowToClear {
	// Skip clearing the backbuffer.
	Skipped,  // Clear the backbuffer.
	JustClear,  // Combine webgl.clear() API with the backbuffer clear, so webgl.clear()

	// doesn't have to call glClear() again.
	CombinedClear
}
