//
//  NSCRender.swift
//  CanvasNative
//
//  Created by Osei Fortune on 10/04/2021.
//

// visionOS has no OpenGL ES; this entire OpenGLES-based renderer is excluded there.
#if !os(visionOS)
import Foundation
import OpenGLES
import CoreVideo
import UIKit
import AVFoundation
import Metal
import Accelerate


@objcMembers
@objc(NSCRender)
public class NSCRender: NSObject {
	private var glCache: CVOpenGLESTextureCache?
	private var mtlCache: CVMetalTextureCache?
	private var texture: GLuint = 0
	private var fbo: [GLuint] = [0,0]
	private var isGL2 = false
	private var program: GLuint = 0
	private var quadVBO: GLuint = 0
	private var context: EAGLContext?
	private var width: GLuint = 0
	private var height: GLuint = 0
	
	public override init() {
		super.init()
		context = EAGLContext.current()
		guard let context = context else {return}
#if !targetEnvironment(simulator)
		CVOpenGLESTextureCacheCreate(kCFAllocatorDefault, nil, context, nil, &glCache)
#endif
		
		 isGL2 = context.api == .openGLES3
		
#if targetEnvironment(simulator)
		glGenFramebuffers(1, &fbo)
		program = glCreateProgram()
		setupShaders()
		setupQuad()
#else
		if(isGL2){
			glGenFramebuffers(2, &fbo)
		}	else {
			glGenFramebuffers(1, &fbo)
			program = glCreateProgram()
			setupShaders()
			setupQuad()
		}
#endif
		
#if targetEnvironment(simulator)
		glGenTextures(1, &texture)
#endif
	}
	
	public init(device: MTLDevice) {
		CVMetalTextureCacheCreate(kCFAllocatorDefault, nil, device, nil, &mtlCache)
	}
	
	
	func getGLTexture(_ cache: CVOpenGLESTextureCache, _ buffer: CVPixelBuffer,_ width: Int, _ height: Int) -> (name: GLuint, texRef: CVOpenGLESTexture)? {
		// video px format is set to bgra32
		var texRef: CVOpenGLESTexture?
		let w = GLsizei(width), h = GLsizei(height)
		
		let status = CVOpenGLESTextureCacheCreateTextureFromImage(
			kCFAllocatorDefault, cache, buffer, nil,
			GLenum(GL_TEXTURE_2D), GL_RGBA, w, h,
			GLenum(GL_RGBA), GLenum(GL_UNSIGNED_BYTE),
			0, &texRef
		)
		
		guard status == kCVReturnSuccess, let t = texRef else { return nil }
		return (CVOpenGLESTextureGetName(t), t)
	}
	
	
	func getMetalTexture(_ buffer: CVPixelBuffer,_ width: Int, _ height: Int) -> (texture: MTLTexture, cvTex: CVMetalTexture)? {
		guard let cache = mtlCache else {return nil}
		var cvTex: CVMetalTexture?
		let status = CVMetalTextureCacheCreateTextureFromImage(
			kCFAllocatorDefault, cache, buffer, nil,
			.bgra8Unorm, width, height, 0, &cvTex
		)
		guard status == kCVReturnSuccess, let c = cvTex,
					let tex = CVMetalTextureGetTexture(c) else { return nil }
		return (tex, c)
	}
	
	public func drawFrame(_ player: AVPlayer, _ output: AVPlayerItemVideoOutput,_ videoSize: CGSize, _ internalFormat: Int32,_ format: Int32,_ flipYWebGL: Bool){
		let currentTime = player.currentTime()
		
		if(!output.hasNewPixelBuffer(forItemTime: currentTime)) {return}
		
		var presentationTime = CMTime.zero
		
		let buffer = output.copyPixelBuffer(forItemTime: currentTime, itemTimeForDisplay: &presentationTime)
		
		guard let pixel_buffer = buffer else {return}
		
		
		let width = CVPixelBufferGetWidth(pixel_buffer)
		let height = CVPixelBufferGetHeight(pixel_buffer)
		
		drawFrame(buffer: pixel_buffer, width: Int(width), height: Int(height), internalFormat: internalFormat, format: format, flipYWebGL: flipYWebGL)
	}
	
	
	func drawBuffer(buffer: CVPixelBuffer,
									width: Int,
									height: Int,
									internalFormat: Int32,
									format: Int32,
									flipYWebGL: Bool) {
		
		CVPixelBufferLockBaseAddress(buffer, .readOnly)
		defer { CVPixelBufferUnlockBaseAddress(buffer, .readOnly) }
		
		guard let baseAddress = CVPixelBufferGetBaseAddress(buffer) else { return }
		
		
		var prevFBO: GLint = 0
		glGetIntegerv(GLenum(GL_FRAMEBUFFER_BINDING), &prevFBO)
		
		var prevTexture: GLint = 0
		glGetIntegerv(GLenum(GL_TEXTURE_BINDING_2D), &prevTexture)
		
		var prevProgram: GLint = 0
		glGetIntegerv(GLenum(GL_CURRENT_PROGRAM), &prevProgram)
		
		var prevVBO: GLuint = 0
		glGetIntegerv(GLenum(GL_ARRAY_BUFFER_BINDING), &prevVBO)
		
		var prevActiveTexture: GLint = 0
		glGetIntegerv(GLenum(GL_ACTIVE_TEXTURE), &prevActiveTexture)
		
		var previousVertexArray: GLint = 0
		glGetIntegerv(GLenum(GL_VERTEX_ARRAY_BINDING), &previousVertexArray)
		
		var previousViewPort: [GLint] = [0,0,0,0]
		glGetIntegerv(GLenum(GL_VIEWPORT), &previousViewPort)
		
		
		var prevAlignment: GLint = 0
		var prevRowLength: GLint = 0
		
		glGetIntegerv(GLenum(GL_UNPACK_ALIGNMENT), &prevAlignment)
		
		if(isGL2){
			glGetIntegerv(GLenum(GL_UNPACK_ROW_LENGTH), &prevRowLength)
		}
		
		let bytesPerRow = CVPixelBufferGetBytesPerRow(buffer)
		let tightlyPacked = bytesPerRow == width * 4
		let pixelCount = width * height
		var uploadPointer: UnsafeMutableRawPointer
		
		
		if(isGL2){
			uploadPointer = baseAddress
		}else {
			if tightlyPacked {
					uploadPointer = baseAddress
			} else {

					let tempBuffer = UnsafeMutableRawPointer.allocate(byteCount: pixelCount * 4, alignment: 1)
					defer { tempBuffer.deallocate() }
					
					for y in 0..<height {
							let src = baseAddress.advanced(by: y * bytesPerRow)
							let dst = tempBuffer.advanced(by: y * width * 4)
							memcpy(dst, src, width * 4)
					}
					
					uploadPointer = tempBuffer
			}
		}
		
		
		glPixelStorei(GLenum(GL_UNPACK_ALIGNMENT), 1)
		
		if(isGL2){
			glPixelStorei(GLenum(GL_UNPACK_ROW_LENGTH), GLint(bytesPerRow / 4))
		}
		
		
		glBindFramebuffer(GLenum(GL_FRAMEBUFFER), fbo[0])
		glFramebufferTexture2D(GLenum(GL_FRAMEBUFFER), GLenum(GL_COLOR_ATTACHMENT0), GLenum(GL_TEXTURE_2D), GLuint(prevTexture), 0)
		
		glUseProgram(program)
		glBindBuffer(GLenum(GL_ARRAY_BUFFER), quadVBO)
		
		let posLoc = GLuint(glGetAttribLocation(program, "aPosition"))
		let texLoc = GLuint(glGetAttribLocation(program, "aTexCoord"))
		
		glEnableVertexAttribArray(posLoc)
		glEnableVertexAttribArray(texLoc)
		
		let stride = GLsizei(4 * MemoryLayout<GLfloat>.size)
		glVertexAttribPointer(posLoc, 2, GLenum(GL_FLOAT), GLboolean(GL_FALSE), stride, UnsafeRawPointer(bitPattern: 0))
		glVertexAttribPointer(texLoc, 2, GLenum(GL_FLOAT), GLboolean(GL_FALSE), stride, UnsafeRawPointer(bitPattern: 2 * MemoryLayout<GLfloat>.size))
		
		
		if(self.width != GLuint(width) || self.height != GLuint(height)){
			glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_MIN_FILTER), GL_LINEAR)
			glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_MAG_FILTER), GL_LINEAR)
			glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_WRAP_S), GL_CLAMP_TO_EDGE)
			glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_WRAP_T), GL_CLAMP_TO_EDGE)
			glTexImage2D(GLenum(GL_TEXTURE_2D),
									 0,
									 GL_RGBA,
									 GLsizei(width),
									 GLsizei(height),
									 0,
									 GLenum(GL_BGRA),
									 GLenum(GL_UNSIGNED_BYTE),
									 nil)
		}
		
		
		glActiveTexture(GLenum(GL_TEXTURE0))
		glBindTexture(GLenum(GL_TEXTURE_2D), texture)
		
		
		if(self.width != GLuint(width) || self.height != GLuint(height)){
			glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_MIN_FILTER), GL_LINEAR)
			glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_MAG_FILTER), GL_LINEAR)
			glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_WRAP_S), GL_CLAMP_TO_EDGE)
			glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_WRAP_T), GL_CLAMP_TO_EDGE)
			
			glTexImage2D(GLenum(GL_TEXTURE_2D),
									 0,
									 GL_RGBA,
									 GLsizei(width),
									 GLsizei(height),
									 0,
									 GLenum(GL_BGRA),
									 GLenum(GL_UNSIGNED_BYTE),
									 uploadPointer)
			
			self.width = GLuint(width)
			self.height =  GLuint(height)
		}else {
			glTexSubImage2D(GLenum(GL_TEXTURE_2D),
											0,
											0, 0,
											GLsizei(width),
											GLsizei(height),
											GLenum(GL_BGRA),
											GLenum(GL_UNSIGNED_BYTE),
											uploadPointer)
		}
		
		
		let texUniform = glGetUniformLocation(program, "uTexture")
		glUniform1i(texUniform, 0)
		
		let flipLoc = glGetUniformLocation(program, "uFlipY")
		glUniform1i(flipLoc, flipYWebGL ? 1 : 0)
		
		
		glViewport(0, 0, GLsizei(width), GLsizei(height))
		
		glDrawArrays(GLenum(GL_TRIANGLES), 0, 6)
		
		glViewport(
			previousViewPort[0],
			previousViewPort[1],
			previousViewPort[2],
			previousViewPort[3]
		)
		
		glBindFramebuffer(GLenum(GL_FRAMEBUFFER), GLuint(prevFBO))
		glBindTexture(GLenum(GL_TEXTURE_2D), GLuint(prevTexture))
		glUseProgram(GLuint(prevProgram))
		glBindBuffer(GLenum(GL_ARRAY_BUFFER), prevVBO)
		glActiveTexture(GLenum(prevActiveTexture))
		glPixelStorei(GLenum(GL_UNPACK_ALIGNMENT), prevAlignment)
		
		if(isGL2){
			glPixelStorei(GLenum(GL_UNPACK_ROW_LENGTH), GLint(prevRowLength))
		}
		
		if(previousVertexArray != 0){
			glBindVertexArray(GLuint(previousVertexArray))
		}
		
	}
	
	func drawTexture(surface: (name: GLuint, texRef: CVOpenGLESTexture),
									width: Int,
									height: Int,
									internalFormat: Int32,
									format: Int32,
									flipYWebGL: Bool) {
		
		var prevFBO: GLint = 0
		glGetIntegerv(GLenum(GL_FRAMEBUFFER_BINDING), &prevFBO)
		
		var prevTexture: GLint = 0
		glGetIntegerv(GLenum(GL_TEXTURE_BINDING_2D), &prevTexture)
		
		var prevProgram: GLint = 0
		glGetIntegerv(GLenum(GL_CURRENT_PROGRAM), &prevProgram)
		
		var prevVBO: GLuint = 0
		glGetIntegerv(GLenum(GL_ARRAY_BUFFER_BINDING), &prevVBO)
		
		var prevActiveTexture: GLint = 0
		glGetIntegerv(GLenum(GL_ACTIVE_TEXTURE), &prevActiveTexture)
		
		var previousVertexArray: GLint = 0
		glGetIntegerv(GLenum(GL_VERTEX_ARRAY_BINDING), &previousVertexArray)
		
		var previousViewPort: [GLint] = [0,0,0,0]
		glGetIntegerv(GLenum(GL_VIEWPORT), &previousViewPort)

				
		glBindFramebuffer(GLenum(GL_FRAMEBUFFER), fbo[0])
		glFramebufferTexture2D(GLenum(GL_FRAMEBUFFER), GLenum(GL_COLOR_ATTACHMENT0), GLenum(GL_TEXTURE_2D), GLuint(prevTexture), 0)
		
	
		glUseProgram(program)
		glBindBuffer(GLenum(GL_ARRAY_BUFFER), quadVBO)
		
		let posLoc = GLuint(glGetAttribLocation(program, "aPosition"))
		let texLoc = GLuint(glGetAttribLocation(program, "aTexCoord"))
		
		glEnableVertexAttribArray(posLoc)
		glEnableVertexAttribArray(texLoc)
		
		
		let stride = GLsizei(4 * MemoryLayout<GLfloat>.size)
		glVertexAttribPointer(posLoc, 2, GLenum(GL_FLOAT), GLboolean(GL_FALSE), stride, UnsafeRawPointer(bitPattern: 0))
	
		
		glVertexAttribPointer(texLoc, 2, GLenum(GL_FLOAT), GLboolean(GL_FALSE), stride, UnsafeRawPointer(bitPattern: 2 * MemoryLayout<GLfloat>.size))
		
		if(self.width != GLuint(width) || self.height != GLuint(height)){
			glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_MIN_FILTER), GL_LINEAR)
			glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_MAG_FILTER), GL_LINEAR)
			glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_WRAP_S), GL_CLAMP_TO_EDGE)
			glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_WRAP_T), GL_CLAMP_TO_EDGE)
			
			glTexImage2D(GLenum(GL_TEXTURE_2D),
									 0,
									 GL_RGBA,
									 GLsizei(width),
									 GLsizei(height),
									 0,
									 GLenum(GL_BGRA),
									 GLenum(GL_UNSIGNED_BYTE),
									 nil)
			
			self.width = GLuint(width)
			self.height = GLuint(height)
		}
		
		

		glBindTexture(GLenum(GL_TEXTURE_2D), surface.name)
		glActiveTexture(GLenum(GL_TEXTURE0))

		
		glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_MIN_FILTER), GL_LINEAR)
		glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_MAG_FILTER), GL_LINEAR)
		glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_WRAP_S), GL_CLAMP_TO_EDGE)
		glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_WRAP_T), GL_CLAMP_TO_EDGE)
		
		let texUniform = glGetUniformLocation(program, "uTexture")
		glUniform1i(texUniform, 0)
		
		let flipLoc = glGetUniformLocation(program, "uFlipY")
		glUniform1i(flipLoc, flipYWebGL ? 1 : 0)
		
		
		glViewport(0, 0, GLsizei(width), GLsizei(height))
		
		glDrawArrays(GLenum(GL_TRIANGLES), 0, 6)
		
		
		glViewport(
			previousViewPort[0],
			previousViewPort[1],
			previousViewPort[2],
			previousViewPort[3]
		)
		
		glBindFramebuffer(GLenum(GL_FRAMEBUFFER), GLuint(prevFBO))
		glBindTexture(GLenum(GL_TEXTURE_2D), GLuint(prevTexture))
		glUseProgram(GLuint(prevProgram))
		glBindBuffer(GLenum(GL_ARRAY_BUFFER), prevVBO)
		glActiveTexture(GLenum(prevActiveTexture))
		
		if(previousVertexArray != 0){
			glBindVertexArray(GLuint(previousVertexArray))
		}
		
	}
	
	
	public func drawFrame(buffer:CVPixelBuffer, width: Int, height: Int, internalFormat: Int32,
												format: Int32,
												flipYWebGL: Bool){
#if targetEnvironment(simulator)
		drawBuffer(buffer: buffer, width: width, height: height, internalFormat: internalFormat, format: format, flipYWebGL: flipYWebGL)
#else
		var prevTexture: GLint = 0
		glGetIntegerv(GLenum(GL_TEXTURE_BINDING_2D), &prevTexture)
		if let glCache = glCache {
			let srcTexture = getGLTexture(glCache, buffer, width, height)
			guard let srcTexture = srcTexture else {return}
			if(isGL2){
				var prevReadFBO: GLint = 0
				var prevDrawFBO: GLint = 0
				
				var prevActiveTexture: GLint = 0
				glGetIntegerv(GLenum(GL_ACTIVE_TEXTURE), &prevActiveTexture)
				
				
				if(self.width != GLuint(width) || self.height != GLuint(height)){
					glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_MIN_FILTER), GL_LINEAR)
					glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_MAG_FILTER), GL_LINEAR)
					glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_WRAP_S), GL_CLAMP_TO_EDGE)
					glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_WRAP_T), GL_CLAMP_TO_EDGE)
					
					glTexImage2D(GLenum(GL_TEXTURE_2D),
											 0,
											 GL_RGBA,
											 GLsizei(width),
											 GLsizei(height),
											 0,
											 GLenum(GL_BGRA),
											 GLenum(GL_UNSIGNED_BYTE),
											 nil)
					
					self.width = GLuint(width)
					self.height = GLuint(height)
				}
				
				
				glActiveTexture(GLenum(GL_TEXTURE0))
				glBindTexture(GLenum(GL_TEXTURE_2D), 0)
				
				glGetIntegerv(GLenum(GL_READ_FRAMEBUFFER_BINDING), &prevReadFBO)
				glGetIntegerv(GLenum(GL_DRAW_FRAMEBUFFER_BINDING), &prevDrawFBO)
				
				
				glBindFramebuffer(GLenum(GL_READ_FRAMEBUFFER), fbo[0])
				glFramebufferTexture2D(GLenum(GL_READ_FRAMEBUFFER),
															 GLenum(GL_COLOR_ATTACHMENT0),
															 GLenum(GL_TEXTURE_2D),
															 srcTexture.name, 0)
				
				
				glBindFramebuffer(GLenum(GL_DRAW_FRAMEBUFFER), fbo[1])
				glFramebufferTexture2D(GLenum(GL_DRAW_FRAMEBUFFER),
															 GLenum(GL_COLOR_ATTACHMENT0),
															 GLenum(GL_TEXTURE_2D),
															 GLuint(prevTexture), 0)
				
				
				
				let dstY0: GLint = flipYWebGL ? GLint(height) : 0
				let dstY1: GLint = flipYWebGL ? 0 : GLint(height)
				
				glBlitFramebuffer(0, 0, GLint(width), GLint(height),
													0, dstY0, GLint(width), dstY1,
													GLbitfield(GL_COLOR_BUFFER_BIT),
													GLenum(GL_LINEAR))
				
				
				glBindFramebuffer(GLenum(GL_READ_FRAMEBUFFER), GLuint(prevReadFBO))
				glBindFramebuffer(GLenum(GL_DRAW_FRAMEBUFFER), GLuint(prevDrawFBO))
				glBindTexture(GLenum(GL_TEXTURE_2D), GLuint(prevTexture))
				glActiveTexture(GLenum(prevActiveTexture))
				
			}else {
				drawTexture(surface: srcTexture, width: width, height: height, internalFormat: internalFormat, format: format, flipYWebGL: flipYWebGL)
			}
		}else {
			drawBuffer(buffer: buffer, width: width, height: height, internalFormat: internalFormat, format: format, flipYWebGL: flipYWebGL)
		}
		
#endif
	}
	
	
	
	private func setupShaders() {
		let vertexShaderSource: NSString = """
 attribute vec2 aPosition;
 attribute vec2 aTexCoord;
 varying vec2 vTexCoord;
 uniform bool uFlipY;
 
 void main() {
 gl_Position = vec4(aPosition, 0.0, 1.0);
 if (uFlipY) {
 vTexCoord = vec2(aTexCoord.x, 1.0 - aTexCoord.y);
 } else {
 vTexCoord = aTexCoord;
 }
 }
 """
		
		let fragmentShaderSource: NSString = """
 precision mediump float;
 varying vec2 vTexCoord;
 uniform sampler2D uTexture;
 
 void main() {
 gl_FragColor = texture2D(uTexture, vTexCoord);
 }
 """
		
		let vertexShader = glCreateShader(GLenum(GL_VERTEX_SHADER))
		var vSource = vertexShaderSource.utf8String
		glShaderSource(vertexShader, 1, &vSource, nil)
		glCompileShader(vertexShader)
		
		let fragmentShader = glCreateShader(GLenum(GL_FRAGMENT_SHADER))
		var fSource = fragmentShaderSource.utf8String
		glShaderSource(fragmentShader, 1, &fSource, nil)
		glCompileShader(fragmentShader)
		
		program = glCreateProgram()
		glAttachShader(program, vertexShader)
		glAttachShader(program, fragmentShader)
		glLinkProgram(program)
	}
	
	private func setupQuad() {
		let quadData: [GLfloat] = [
			-1, -1, 0, 0,
			 1, -1, 1, 0,
			 -1,  1, 0, 1,
			 -1,  1, 0, 1,
			 1, -1, 1, 0,
			 1,  1, 1, 1
		]
		glGenBuffers(1, &quadVBO)
		glBindBuffer(GLenum(GL_ARRAY_BUFFER), quadVBO)
		glBufferData(GLenum(GL_ARRAY_BUFFER), quadData.count * MemoryLayout<GLfloat>.size, quadData, GLenum(GL_STATIC_DRAW))
	}
	
	
	static func getPixelFormat(_ cgImage: CGImage) -> CGBitmapInfo? {
		return cgImage.bitmapInfo.intersection(.byteOrderMask)
	}
	
	public func drawFrameTexImage3D(_ player: AVPlayer, _ output: AVPlayerItemVideoOutput, _ videoSize: CGSize,
	                                 _ target: Int32, _ level: Int32, _ internalFormat: Int32,
	                                 _ width: Int32, _ height: Int32, _ depth: Int32, _ border: Int32,
	                                 _ format: Int32, _ type: Int32, _ flipYWebGL: Bool) {
		let currentTime = player.currentTime()
		guard output.hasNewPixelBuffer(forItemTime: currentTime) else { return }
		var presentationTime = CMTime.zero
		guard let buffer = output.copyPixelBuffer(forItemTime: currentTime, itemTimeForDisplay: &presentationTime) else { return }
		uploadPixelBufferTexImage3D(buffer: buffer, target: target, level: level,
		                            internalFormat: internalFormat, width: width, height: height,
		                            depth: depth, border: border, flipYWebGL: flipYWebGL)
	}

	public func drawFrameTexSubImage3D(_ player: AVPlayer, _ output: AVPlayerItemVideoOutput, _ videoSize: CGSize,
	                                    _ target: Int32, _ level: Int32,
	                                    _ xoffset: Int32, _ yoffset: Int32, _ zoffset: Int32,
	                                    _ width: Int32, _ height: Int32, _ depth: Int32,
	                                    _ format: Int32, _ type: Int32, _ flipYWebGL: Bool) {
		let currentTime = player.currentTime()
		guard output.hasNewPixelBuffer(forItemTime: currentTime) else { return }
		var presentationTime = CMTime.zero
		guard let buffer = output.copyPixelBuffer(forItemTime: currentTime, itemTimeForDisplay: &presentationTime) else { return }
		uploadPixelBufferTexSubImage3D(buffer: buffer, target: target, level: level,
		                               xoffset: xoffset, yoffset: yoffset, zoffset: zoffset,
		                               width: width, height: height, depth: depth, flipYWebGL: flipYWebGL)
	}


	private func decodeBGRAtoRGBA(baseAddress: UnsafeMutableRawPointer,
	                               pixelWidth: Int, pixelHeight: Int,
	                               srcBytesPerRow: Int) -> UnsafeMutableRawPointer {
		let tightBytesPerRow = pixelWidth * 4
		let totalBytes = pixelWidth * pixelHeight * 4

		let bgraBuf = UnsafeMutableRawPointer.allocate(byteCount: totalBytes, alignment: 1)
		if srcBytesPerRow == tightBytesPerRow {
			memcpy(bgraBuf, baseAddress, totalBytes)
		} else {
			for y in 0..<pixelHeight {
				memcpy(bgraBuf.advanced(by: y * tightBytesPerRow),
				       baseAddress.advanced(by: y * srcBytesPerRow),
				       tightBytesPerRow)
			}
		}

		let rgbaBuf = UnsafeMutableRawPointer.allocate(byteCount: totalBytes, alignment: 1)
		var srcVImg = vImage_Buffer(data: bgraBuf, height: vImagePixelCount(pixelHeight),
		                            width: vImagePixelCount(pixelWidth), rowBytes: tightBytesPerRow)
		var dstVImg = vImage_Buffer(data: rgbaBuf, height: vImagePixelCount(pixelHeight),
		                            width: vImagePixelCount(pixelWidth), rowBytes: tightBytesPerRow)
		// BGRA[0,1,2,3] → RGBA: channel 2→0, 1→1, 0→2, 3→3
		vImagePermuteChannels_ARGB8888(&srcVImg, &dstVImg, [2, 1, 0, 3], 0)
		bgraBuf.deallocate()
		return rgbaBuf
	}


	private func scaleRGBA(_ src: UnsafeMutableRawPointer,
	                        srcWidth: Int, srcHeight: Int,
	                        destWidth: Int, destHeight: Int) -> UnsafeMutableRawPointer {
		let srcRowBytes  = srcWidth  * 4
		let destRowBytes = destWidth * 4
		let destBytes    = destWidth * destHeight * 4
		let destBuf = UnsafeMutableRawPointer.allocate(byteCount: destBytes, alignment: 1)
		var srcVImg  = vImage_Buffer(data: src, height: vImagePixelCount(srcHeight),
		                             width: vImagePixelCount(srcWidth), rowBytes: srcRowBytes)
		var destVImg = vImage_Buffer(data: destBuf, height: vImagePixelCount(destHeight),
		                             width: vImagePixelCount(destWidth), rowBytes: destRowBytes)
		vImageScale_ARGB8888(&srcVImg, &destVImg, nil, vImage_Flags(kvImageHighQualityResampling))
		return destBuf
	}


	private func flipRGBAVertically(_ src: UnsafeMutableRawPointer,
	                                 width: Int, height: Int) -> UnsafeMutableRawPointer {
		let rowBytes  = width * 4
		let totalBytes = width * height * 4
		let flipped = UnsafeMutableRawPointer.allocate(byteCount: totalBytes, alignment: 1)
		let fp = flipped.assumingMemoryBound(to: UInt8.self)
		let rp = src.assumingMemoryBound(to: UInt8.self)
		for y in 0..<height {
			memcpy(fp.advanced(by: (height - 1 - y) * rowBytes),
			       rp.advanced(by: y * rowBytes),
			       rowBytes)
		}
		return flipped
	}

	private func uploadPixelBufferTexImage3D(buffer: CVPixelBuffer,
	                                          target: Int32, level: Int32, internalFormat: Int32,
	                                          width: Int32, height: Int32, depth: Int32, border: Int32,
	                                          flipYWebGL: Bool) {
		CVPixelBufferLockBaseAddress(buffer, .readOnly)
		defer { CVPixelBufferUnlockBaseAddress(buffer, .readOnly) }
		guard let baseAddress = CVPixelBufferGetBaseAddress(buffer) else { return }

		let pixelWidth  = CVPixelBufferGetWidth(buffer)
		let pixelHeight = CVPixelBufferGetHeight(buffer)
		let srcBytesPerRow = CVPixelBufferGetBytesPerRow(buffer)
		let destWidth  = Int(width)
		let destHeight = Int(height)

		var rgbaBuf = decodeBGRAtoRGBA(baseAddress: baseAddress, pixelWidth: pixelWidth,
		                                pixelHeight: pixelHeight, srcBytesPerRow: srcBytesPerRow)
		defer { rgbaBuf.deallocate() }

		var scaledBuf: UnsafeMutableRawPointer? = nil
		var uploadBuf = rgbaBuf
		if pixelWidth != destWidth || pixelHeight != destHeight {
			scaledBuf = scaleRGBA(rgbaBuf, srcWidth: pixelWidth, srcHeight: pixelHeight,
			                      destWidth: destWidth, destHeight: destHeight)
			uploadBuf = scaledBuf!
		}
		defer { scaledBuf?.deallocate() }

		var flippedBuf: UnsafeMutableRawPointer? = nil
		if flipYWebGL {
			flippedBuf = flipRGBAVertically(uploadBuf, width: destWidth, height: destHeight)
			uploadBuf = flippedBuf!
		}
		defer { flippedBuf?.deallocate() }

		var prevAlignment: GLint = 0
		glGetIntegerv(GLenum(GL_UNPACK_ALIGNMENT), &prevAlignment)
		glPixelStorei(GLenum(GL_UNPACK_ALIGNMENT), 1)
		defer { glPixelStorei(GLenum(GL_UNPACK_ALIGNMENT), prevAlignment) }

		glTexImage3D(GLenum(target), level, internalFormat,
		             GLsizei(destWidth), GLsizei(destHeight), GLsizei(depth),
		             border, GLenum(GL_RGBA), GLenum(GL_UNSIGNED_BYTE), uploadBuf)
	}

	private func uploadPixelBufferTexSubImage3D(buffer: CVPixelBuffer,
	                                             target: Int32, level: Int32,
	                                             xoffset: Int32, yoffset: Int32, zoffset: Int32,
	                                             width: Int32, height: Int32, depth: Int32,
	                                             flipYWebGL: Bool) {
		CVPixelBufferLockBaseAddress(buffer, .readOnly)
		defer { CVPixelBufferUnlockBaseAddress(buffer, .readOnly) }
		guard let baseAddress = CVPixelBufferGetBaseAddress(buffer) else { return }

		let pixelWidth  = CVPixelBufferGetWidth(buffer)
		let pixelHeight = CVPixelBufferGetHeight(buffer)
		let srcBytesPerRow = CVPixelBufferGetBytesPerRow(buffer)
		let destWidth  = Int(width)
		let destHeight = Int(height)

		var rgbaBuf = decodeBGRAtoRGBA(baseAddress: baseAddress, pixelWidth: pixelWidth,
		                                pixelHeight: pixelHeight, srcBytesPerRow: srcBytesPerRow)
		defer { rgbaBuf.deallocate() }

		var scaledBuf: UnsafeMutableRawPointer? = nil
		var uploadBuf = rgbaBuf
		if pixelWidth != destWidth || pixelHeight != destHeight {
			scaledBuf = scaleRGBA(rgbaBuf, srcWidth: pixelWidth, srcHeight: pixelHeight,
			                      destWidth: destWidth, destHeight: destHeight)
			uploadBuf = scaledBuf!
		}
		defer { scaledBuf?.deallocate() }

		var flippedBuf: UnsafeMutableRawPointer? = nil
		if flipYWebGL {
			flippedBuf = flipRGBAVertically(uploadBuf, width: destWidth, height: destHeight)
			uploadBuf = flippedBuf!
		}
		defer { flippedBuf?.deallocate() }

		var prevAlignment: GLint = 0
		glGetIntegerv(GLenum(GL_UNPACK_ALIGNMENT), &prevAlignment)
		glPixelStorei(GLenum(GL_UNPACK_ALIGNMENT), 1)
		defer { glPixelStorei(GLenum(GL_UNPACK_ALIGNMENT), prevAlignment) }

		glTexSubImage3D(GLenum(target), level, xoffset, yoffset, zoffset,
		                GLsizei(destWidth), GLsizei(destHeight), GLsizei(depth),
		                GLenum(GL_RGBA), GLenum(GL_UNSIGNED_BYTE), uploadBuf)
	}

	/// Shared Metal texture cache for static drawVideoFrame methods
	private static var sharedMtlCache: CVMetalTextureCache? = {
		guard let device = MTLCreateSystemDefaultDevice() else { return nil }
		var cache: CVMetalTextureCache?
		CVMetalTextureCacheCreate(kCFAllocatorDefault, nil, device, nil, &cache)
		return cache
	}()
	
	private static func copyCurrentPixelBuffer(
		_ player: AVPlayer,
		_ output: AVPlayerItemVideoOutput
	) -> CVPixelBuffer? {
		let currentTime = player.currentTime()
		guard output.hasNewPixelBuffer(forItemTime: currentTime) else { return nil }
		var presentationTime = CMTime.zero
		return output.copyPixelBuffer(forItemTime: currentTime, itemTimeForDisplay: &presentationTime)
	}
	
	private static func drawWithMetalTexture(
		_ buffer: CVPixelBuffer,
		_ context: Int64,
		_ sx: Float, _ sy: Float, _ sw: Float, _ sh: Float,
		_ dx: Float, _ dy: Float, _ dw: Float, _ dh: Float
	) -> Bool {
		guard let cache = sharedMtlCache else { return false }
		let width = CVPixelBufferGetWidth(buffer)
		let height = CVPixelBufferGetHeight(buffer)
		
		var cvTex: CVMetalTexture?
		let status = CVMetalTextureCacheCreateTextureFromImage(
			kCFAllocatorDefault, cache, buffer, nil,
			.bgra8Unorm, width, height, 0, &cvTex
		)
		guard status == kCVReturnSuccess,
					let cvMetalTex = cvTex,
					let mtlTexture = CVMetalTextureGetTexture(cvMetalTex) else { return false }
		
		let texPtr = Unmanaged.passUnretained(mtlTexture).toOpaque()
		return canvas_native_ios_context_draw_image_with_metal_texture(
			context, texPtr,
			Float(width), Float(height),
			sx, sy, sw, sh,
			dx, dy, dw, dh
		)
	}
	
	private static func drawWithBGRABytes(
		_ buffer: CVPixelBuffer,
		_ context: Int64,
		_ sx: Float, _ sy: Float, _ sw: Float, _ sh: Float,
		_ dx: Float, _ dy: Float, _ dw: Float, _ dh: Float
	) -> Bool {
		CVPixelBufferLockBaseAddress(buffer, .readOnly)
		defer { CVPixelBufferUnlockBaseAddress(buffer, .readOnly) }
		
		guard let baseAddress = CVPixelBufferGetBaseAddress(buffer) else { return false }
		let width = CVPixelBufferGetWidth(buffer)
		let height = CVPixelBufferGetHeight(buffer)
		let bytesPerRow = CVPixelBufferGetBytesPerRow(buffer)
		let tightSize = width * 4
		if bytesPerRow == tightSize {
			return canvas_native_ios_context_draw_image_with_bgra_bytes(
				context,
				baseAddress.assumingMemoryBound(to: UInt8.self),
				UInt(width * height * 4),
				Float(width), Float(height),
				sx, sy, sw, sh,
				dx, dy, dw, dh
			)
		} else {
			let totalBytes = width * height * 4
			let compact = UnsafeMutableRawPointer.allocate(byteCount: totalBytes, alignment: 1)
			defer { compact.deallocate() }
			for y in 0..<height {
				memcpy(compact.advanced(by: y * tightSize),
							 baseAddress.advanced(by: y * bytesPerRow),
							 tightSize)
			}
			return canvas_native_ios_context_draw_image_with_bgra_bytes(
				context,
				compact.assumingMemoryBound(to: UInt8.self),
				UInt(totalBytes),
				Float(width), Float(height),
				sx, sy, sw, sh,
				dx, dy, dw, dh
			)
		}
	}
	
	@objc public static func drawVideoFrame(_ player: AVPlayer, _ output: AVPlayerItemVideoOutput, _ videoSize: CGSize, _ context: Int64, _ dx: Float, _ dy: Float) -> Bool {
		guard let buffer = copyCurrentPixelBuffer(player, output) else { return false }
		let w = Float(CVPixelBufferGetWidth(buffer))
		let h = Float(CVPixelBufferGetHeight(buffer))
		if drawWithMetalTexture(buffer, context, 0, 0, w, h, dx, dy, w, h) { return true }
		return drawWithBGRABytes(buffer, context, 0, 0, w, h, dx, dy, w, h)
	}
	
	@objc public static func drawVideoFrame(_ player: AVPlayer, _ output: AVPlayerItemVideoOutput, _ videoSize: CGSize, _ context: Int64, _ dx: Float, _ dy: Float, _ dw: Float, _ dh: Float) -> Bool {
		guard let buffer = copyCurrentPixelBuffer(player, output) else { return false }
		let w = Float(CVPixelBufferGetWidth(buffer))
		let h = Float(CVPixelBufferGetHeight(buffer))
		if drawWithMetalTexture(buffer, context, 0, 0, w, h, dx, dy, dw, dh) { return true }
		return drawWithBGRABytes(buffer, context, 0, 0, w, h, dx, dy, dw, dh)
	}
	
	@objc public static func drawVideoFrame(_ player: AVPlayer, _ output: AVPlayerItemVideoOutput, _ videoSize: CGSize, _ context: Int64, _ sx: Float, _ sy: Float, _ sw: Float, _ sh: Float, _ dx: Float, _ dy: Float, _ dw: Float, _ dh: Float) -> Bool {
		guard let buffer = copyCurrentPixelBuffer(player, output) else { return false }
		if drawWithMetalTexture(buffer, context, sx, sy, sw, sh, dx, dy, dw, dh) { return true }
		return drawWithBGRABytes(buffer, context, sx, sy, sw, sh, dx, dy, dw, dh)
	}
	
	/// Captures the current video frame and returns it as tightly-packed **RGBA8888** bytes.
	///
	/// The BGRA→RGBA channel swap is performed in Swift using
	/// `vImagePermuteChannels_ARGB8888` from the Accelerate framework, which exploits
	/// SIMD instructions and is substantially faster than a JS-side byte-swap loop.
	///
	/// - Returns: `NSDictionary` with keys `"data"` (NSData, RGBA), `"width"`, `"height"`,
	///   or `nil` when no frame is available.
	@objc public static func getVideoFrameData(_ player: AVPlayer, _ output: AVPlayerItemVideoOutput, _ videoSize: CGSize) -> NSDictionary? {
		guard let buffer = copyCurrentPixelBuffer(player, output) else { return nil }
		CVPixelBufferLockBaseAddress(buffer, .readOnly)
		defer { CVPixelBufferUnlockBaseAddress(buffer, .readOnly) }

		guard let baseAddress = CVPixelBufferGetBaseAddress(buffer) else { return nil }
		let width       = CVPixelBufferGetWidth(buffer)
		let height      = CVPixelBufferGetHeight(buffer)
		let srcRowBytes = CVPixelBufferGetBytesPerRow(buffer)
		let dstRowBytes = width * 4
		let totalBytes  = height * dstRowBytes

		// NSMutableData owns the RGBA output buffer.
		guard let outData = NSMutableData(length: totalBytes) else { return nil }
		let dstPtr = outData.mutableBytes

		// Permute map for BGRA → RGBA:
		//   dst[0] = src[2] (R), dst[1] = src[1] (G),
		//   dst[2] = src[0] (B), dst[3] = src[3] (A)
		let permuteMap: [UInt8] = [2, 1, 0, 3]

		if srcRowBytes == dstRowBytes {
			// No row padding — single-pass SIMD conversion
			var src = vImage_Buffer(data: baseAddress,
			                        height: vImagePixelCount(height),
			                        width:  vImagePixelCount(width),
			                        rowBytes: srcRowBytes)
			var dst = vImage_Buffer(data: dstPtr,
			                        height: vImagePixelCount(height),
			                        width:  vImagePixelCount(width),
			                        rowBytes: dstRowBytes)
			vImagePermuteChannels_ARGB8888(&src, &dst, permuteMap, 0)
		} else {
			// Padded rows — compact and convert in one row-by-row pass
			for y in 0..<height {
				var src = vImage_Buffer(data: baseAddress.advanced(by: y * srcRowBytes),
				                        height: 1,
				                        width:  vImagePixelCount(width),
				                        rowBytes: srcRowBytes)
				var dst = vImage_Buffer(data: dstPtr.advanced(by: y * dstRowBytes),
				                        height: 1,
				                        width:  vImagePixelCount(width),
				                        rowBytes: dstRowBytes)
				vImagePermuteChannels_ARGB8888(&src, &dst, permuteMap, 0)
			}
		}

		return [
			"data":   outData,
			"width":  NSNumber(value: width),
			"height": NSNumber(value: height),
		]
	}
}
#endif
