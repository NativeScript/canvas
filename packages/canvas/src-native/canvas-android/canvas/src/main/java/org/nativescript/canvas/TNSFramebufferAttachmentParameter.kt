package org.nativescript.canvas

/**
 * Created by triniwiz on 4/21/20
 */
class TNSFramebufferAttachmentParameter {
	var isTexture = false
	var isRenderbuffer = false
	var value = 0

	constructor()
	constructor(isTexture: Boolean, isRenderbuffer: Boolean, value: Int) {
		this.isTexture = isTexture
		this.isRenderbuffer = isRenderbuffer
		this.value = value
	}
}
