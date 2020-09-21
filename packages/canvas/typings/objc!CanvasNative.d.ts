
interface CanvasArray {
	array: interop.Pointer | interop.Reference<any>;
	length: number;
}
declare var CanvasArray: interop.StructType<CanvasArray>;

declare const enum CanvasColorStyleType {

	Color = 0,

	Gradient = 1,

	Pattern = 2
}

interface CanvasDevice {
	device: interop.Pointer | interop.Reference<any>;
	queue: interop.Pointer | interop.Reference<any>;
	drawable: interop.Pointer | interop.Reference<any>;
}
declare var CanvasDevice: interop.StructType<CanvasDevice>;

declare class CanvasGLKView extends GLKView {

	static alloc(): CanvasGLKView; // inherited from NSObject

	static appearance(): CanvasGLKView; // inherited from UIAppearance

	static appearanceForTraitCollection(trait: UITraitCollection): CanvasGLKView; // inherited from UIAppearance

	static appearanceForTraitCollectionWhenContainedIn(trait: UITraitCollection, ContainerClass: typeof NSObject): CanvasGLKView; // inherited from UIAppearance

	static appearanceForTraitCollectionWhenContainedInInstancesOfClasses(trait: UITraitCollection, containerTypes: NSArray<typeof NSObject> | typeof NSObject[]): CanvasGLKView; // inherited from UIAppearance

	static appearanceWhenContainedIn(ContainerClass: typeof NSObject): CanvasGLKView; // inherited from UIAppearance

	static appearanceWhenContainedInInstancesOfClasses(containerTypes: NSArray<typeof NSObject> | typeof NSObject[]): CanvasGLKView; // inherited from UIAppearance

	static new(): CanvasGLKView; // inherited from NSObject
}

declare var CanvasNativeVersionNumber: number;

declare var CanvasNativeVersionString: interop.Reference<number>;

interface CanvasTextMetrics {
	width: number;
}
declare var CanvasTextMetrics: interop.StructType<CanvasTextMetrics>;

declare class Color extends NSObject implements ICanvasColorStyle {

	static alloc(): Color; // inherited from NSObject

	static new(): Color; // inherited from NSObject

	constructor(o: { color: UIColor; });

	getStyleType(): CanvasColorStyleType;

	initWithColor(color: UIColor): this;
}

declare class GLRenderer extends NSObject implements GLKViewDelegate {

	static alloc(): GLRenderer; // inherited from NSObject

	static new(): GLRenderer; // inherited from NSObject

	readonly debugDescription: string; // inherited from NSObjectProtocol

	readonly description: string; // inherited from NSObjectProtocol

	readonly hash: number; // inherited from NSObjectProtocol

	readonly isProxy: boolean; // inherited from NSObjectProtocol

	readonly superclass: typeof NSObject; // inherited from NSObjectProtocol

	readonly  // inherited from NSObjectProtocol

	class(): typeof NSObject;

	conformsToProtocol(aProtocol: any /* Protocol */): boolean;

	glkViewDrawInRect(view: GLKView, rect: CGRect): void;

	isEqual(object: any): boolean;

	isKindOfClass(aClass: typeof NSObject): boolean;

	isMemberOfClass(aClass: typeof NSObject): boolean;

	performSelector(aSelector: string): any;

	performSelectorWithObject(aSelector: string, object: any): any;

	performSelectorWithObjectWithObject(aSelector: string, object1: any, object2: any): any;

	respondsToSelector(aSelector: string): boolean;

	retainCount(): number;

	self(): this;
}

declare class Gradient extends NSObject implements ICanvasColorStyle {

	static alloc(): Gradient; // inherited from NSObject

	static new(): Gradient; // inherited from NSObject

	addColorStopWithOffsetColor(offset: number, color: number): void;

	addColorStopWithOffsetUIColor(offset: number, color: UIColor): void;

	getStyleType(): CanvasColorStyleType;
}

interface ICanvasColorStyle {

	getStyleType(): CanvasColorStyleType;
}
declare var ICanvasColorStyle: {

	prototype: ICanvasColorStyle;
};

declare class MetalRenderer extends NSObject implements MTKViewDelegate {

	static alloc(): MetalRenderer; // inherited from NSObject

	static new(): MetalRenderer; // inherited from NSObject

	readonly debugDescription: string; // inherited from NSObjectProtocol

	readonly description: string; // inherited from NSObjectProtocol

	readonly hash: number; // inherited from NSObjectProtocol

	readonly isProxy: boolean; // inherited from NSObjectProtocol

	readonly superclass: typeof NSObject; // inherited from NSObjectProtocol

	readonly  // inherited from NSObjectProtocol

	class(): typeof NSObject;

	conformsToProtocol(aProtocol: any /* Protocol */): boolean;

	drawInMTKView(view: MTKView): void;

	isEqual(object: any): boolean;

	isKindOfClass(aClass: typeof NSObject): boolean;

	isMemberOfClass(aClass: typeof NSObject): boolean;

	mtkViewDrawableSizeWillChange(view: MTKView, size: CGSize): void;

	performSelector(aSelector: string): any;

	performSelectorWithObject(aSelector: string, object: any): any;

	performSelectorWithObjectWithObject(aSelector: string, object1: any, object2: any): any;

	respondsToSelector(aSelector: string): boolean;

	retainCount(): number;

	self(): this;
}

interface NativeByteArray {
	array: string;
	length: number;
}
declare var NativeByteArray: interop.StructType<NativeByteArray>;

declare class TNSAnimationFrame extends NSObject {

	static alloc(): TNSAnimationFrame; // inherited from NSObject

	static cancelAnimationFrameWithId(id: string): void;

	static new(): TNSAnimationFrame; // inherited from NSObject

	static requestAnimationFrameToLoop(toLoop: (p1: number) => void): void;
}

declare class TNSCanvas extends UIView {

	static alloc(): TNSCanvas; // inherited from NSObject

	static appearance(): TNSCanvas; // inherited from UIAppearance

	static appearanceForTraitCollection(trait: UITraitCollection): TNSCanvas; // inherited from UIAppearance

	static appearanceForTraitCollectionWhenContainedIn(trait: UITraitCollection, ContainerClass: typeof NSObject): TNSCanvas; // inherited from UIAppearance

	static appearanceForTraitCollectionWhenContainedInInstancesOfClasses(trait: UITraitCollection, containerTypes: NSArray<typeof NSObject> | typeof NSObject[]): TNSCanvas; // inherited from UIAppearance

	static appearanceWhenContainedIn(ContainerClass: typeof NSObject): TNSCanvas; // inherited from UIAppearance

	static appearanceWhenContainedInInstancesOfClasses(containerTypes: NSArray<typeof NSObject> | typeof NSObject[]): TNSCanvas; // inherited from UIAppearance

	static createSVGMatrix(): TNSDOMMatrix;

	static getViews(): NSMapTable<string, TNSCanvas>;

	static new(): TNSCanvas; // inherited from NSObject

	canvas: number;

	canvasState: NSArray<number>;

	readonly fps: number;

	handleInvalidationManually: boolean;

	readonly height: number;

	readonly isGL: boolean;

	readonly width: number;

	constructor(o: { frame: CGRect; useGL: boolean; });

	didDraw(): void;

	doDraw(): void;

	flush(): void;

	getContext(type: string): TNSCanvasRenderingContext;

	getContextContextAttributes(type: string, contextAttributes: NSDictionary<any, any>): TNSCanvasRenderingContext;

	getId(): number;

	getViewPtr(): interop.Pointer | interop.Reference<any>;

	handleMoveOffMain(): void;

	handleMoveToMain(): void;

	initWithFrameUseGL(frame: CGRect, useGL: boolean): this;

	moveOffMain(): void;

	moveToMain(): void;

	pause(): void;

	resume(): void;

	setListener(listener: TNSCanvasListener): void;

	snapshot(): NSArray<number>;

	toDataURL(): string;

	toDataURLAsync(callback: (p1: string) => void): void;

	updateDirection(direction: string): void;
}

interface TNSCanvasListener {

	contextReady(): void;
}
declare var TNSCanvasListener: {

	prototype: TNSCanvasListener;
};

declare class TNSCanvasRenderingContext extends NSObject {

	static alloc(): TNSCanvasRenderingContext; // inherited from NSObject

	static new(): TNSCanvasRenderingContext; // inherited from NSObject
}

declare class TNSCanvasRenderingContext2D extends TNSCanvasRenderingContext {

	static alloc(): TNSCanvasRenderingContext2D; // inherited from NSObject

	static new(): TNSCanvasRenderingContext2D; // inherited from NSObject

	currentTransform: TNSDOMMatrix;

	fillStyle: any;

	font: string;

	globalAlpha: number;

	globalCompositeOperation: TNSCompositeOperationType;

	imageSmoothingEnabled: boolean;

	imageSmoothingQuality: TNSImageSmoothingQuality;

	lineCap: TNSLineCap;

	lineDashOffset: number;

	lineJoin: TNSLineJoin;

	lineWidth: number;

	miterLimit: number;

	shadowBlur: number;

	shadowColor: any;

	shadowOffsetX: number;

	shadowOffsetY: number;

	strokeStyle: any;

	textAlign: TNSTextAlignment;

	constructor();

	arc(x: number, y: number, radius: number, startAngle: number, endAngle: number): void;

	arcTo(x1: number, y1: number, x2: number, y2: number, radius: number): void;

	beginPath(): void;

	bezierCurveTo(cp1x: number, cp1y: number, cp2x: number, cp2y: number, x: number, y: number): void;

	clearRect(x: number, y: number, width: number, height: number): void;

	clip(): void;

	closePath(): void;

	createImageData(imageData: TNSImageData): TNSImageData;

	createLinearGradient(x0: number, y0: number, x1: number, y1: number): TNSLinearGradient;

	createPattern(value: any, repetition: TNSPatternRepetition): any;

	createRadialGradient(x0: number, y0: number, r0: number, x1: number, y1: number, r1: number): TNSRadialGradient;

	drawImage(image: any, sx: number, sy: number, sWidth: number, sHeight: number, dx: number, dy: number, dWidth: number, dHeight: number): void;

	ellipse(x: number, y: number, radiusX: number, radiusY: number, rotation: number, startAngle: number, endAngle: number): void;

	fill(): void;

	fillRect(x: number, y: number, width: number, height: number): void;

	fillText(text: string, x: number, y: number): void;

	fillWithValue(value: any): void;

	getCanvas(): TNSCanvas;

	getImageData(sx: number, sy: number, sw: number, sh: number): TNSImageData;

	getLineDash(): NSArray<number>;

	// @ts-ignore
	init(canvas: TNSCanvas): this;

	isPointInPath(x: number, y: number): boolean;

	isPointInStroke(x: number, y: number): boolean;

	lineTo(x: number, y: number): void;

	measureText(text: string): TNSTextMetrics;

	moveTo(x: number, y: number): void;

	putImageData(imageData: TNSImageData, dx: number, dy: number): void;

	quadraticCurveTo(cpx: number, cpy: number, x: number, y: number): void;

	rect(x: number, y: number, width: number, height: number): void;

	resetTransform(): void;

	restore(): void;

	rotate(angle: number): void;

	save(): void;

	scale(x: number, y: number): void;

	setLineDash(segments: NSArray<number> | number[]): void;

	setTransform(a: number, b: number, c: number, d: number, e: number, f: number): void;

	stroke(path: TNSPath2D): void;

	strokeRect(x: number, y: number, width: number, height: number): void;

	strokeText(text: string, x: number, y: number, width: number): void;

	transform(a: number, b: number, c: number, d: number, e: number, f: number): void;

	translate(x: number, y: number): void;
}

declare class TNSColorStyle extends NSObject {

	static alloc(): TNSColorStyle; // inherited from NSObject

	static new(): TNSColorStyle; // inherited from NSObject
}

declare const enum TNSCompositeOperationType {

	SourceOver = 0,

	SourceIn = 1,

	SourceOut = 2,

	SourceAtop = 3,

	DestinationOver = 4,

	DestinationIn = 5,

	DestinationOut = 6,

	DestinationAtop = 7,

	Lighter = 8,

	Copy = 9,

	Xor = 10,

	Multiply = 11,

	Screen = 12,

	Overlay = 13,

	Darken = 14,

	Lighten = 15,

	ColorDodge = 16,

	ColorBurn = 17,

	HardLight = 18,

	SoftLight = 19,

	Difference = 20,

	Exclusion = 21,

	Hue = 22,

	Saturation = 23,

	Color = 24,

	Luminosity = 25
}

declare class TNSDOMMatrix extends NSObject {

	static alloc(): TNSDOMMatrix; // inherited from NSObject

	static new(): TNSDOMMatrix; // inherited from NSObject

	a: number;

	b: number;

	c: number;

	d: number;

	e: number;

	f: number;
}

declare class TNSFramebufferAttachmentParameter extends NSObject {

	static alloc(): TNSFramebufferAttachmentParameter; // inherited from NSObject

	static new(): TNSFramebufferAttachmentParameter; // inherited from NSObject

	readonly isRenderbuffer: boolean;

	readonly isTexture: boolean;

	readonly value: number;

	constructor(o: { isTexture: boolean; isRenderbuffer: boolean; value: number; });

	initWithIsTextureIsRenderbufferValue(isTexture: boolean, isRenderbuffer: boolean, value: number): this;
}

declare class TNSImageAsset extends NSObject {

	static alloc(): TNSImageAsset; // inherited from NSObject

	static new(): TNSImageAsset; // inherited from NSObject

	static set_queue(value: NSObject): void;

	readonly error: string;

	readonly height: number;

	readonly width: number;

	static _queue: NSObject;

	flipX(): void;

	flipY(): void;

	getRawBytes(): string;

	loadImageFromBytesAsyncWithArrayCallback(array: NSArray<number> | number[], callback: (p1: string) => void): void;

	loadImageFromBytesWithArray(array: NSArray<number> | number[]): boolean;

	loadImageFromImageAsyncWithImageCallback(image: UIImage, callback: (p1: string) => void): void;

	loadImageFromImageWithImage(image: UIImage): boolean;

	loadImageFromPathAsyncWithPathCallback(path: string, callback: (p1: string) => void): void;

	loadImageFromPathWithPath(path: string): boolean;

	saveAsyncWithPathFormatCallback(path: string, format: TNSImageAssetFormat, callback: (p1: boolean) => void): void;

	saveWithPathFormat(path: string, format: TNSImageAssetFormat): boolean;

	scaleWithXY(x: number, y: number): void;
}

declare const enum TNSImageAssetFormat {

	JPG = 0,

	PNG = 1,

	ICO = 2,

	BMP = 3,

	TIFF = 4
}

declare class TNSImageData extends NSObject {

	static alloc(): TNSImageData; // inherited from NSObject

	static new(): TNSImageData; // inherited from NSObject

	readonly data: NSData;

	readonly height: number;

	readonly width: number;

	constructor(o: { width: number; height: number; });

	initWithWidthHeight(width: number, height: number): this;
}

declare const enum TNSImageSmoothingQuality {

	Low = 0,

	Medium = 1,

	High = 2
}

declare class TNSIndexedParameter extends NSObject {

	static alloc(): TNSIndexedParameter; // inherited from NSObject

	static new(): TNSIndexedParameter; // inherited from NSObject
}

declare const enum TNSLineCap {

	Butt = 0,

	Round = 1,

	Square = 2
}

declare const enum TNSLineJoin {

	Bevel = 0,

	Round = 1,

	Miter = 2
}

declare class TNSLinearGradient extends Gradient {

	static alloc(): TNSLinearGradient; // inherited from NSObject

	static new(): TNSLinearGradient; // inherited from NSObject

	constructor(o: { x0: number; y0: number; x1: number; y1: number; });

	initWithX0Y0X1Y1(x0: number, y0: number, x1: number, y1: number): this;
}

declare class TNSPath2D extends NSObject {

	static alloc(): TNSPath2D; // inherited from NSObject

	static new(): TNSPath2D; // inherited from NSObject

	constructor(o: { data: string; });

	constructor(o: { path: TNSPath2D; });

	addPath(path: TNSPath2D): void;

	arc(x: number, y: number, radius: number, startAngle: number, endAngle: number): void;

	arcTo(x1: number, y1: number, x2: number, y2: number, radius: number): void;

	bezierCurveTo(cp1x: number, cp1y: number, cp2x: number, cp2y: number, x: number, y: number): void;

	closePath(): void;

	ellipse(x: number, y: number, radiusX: number, radiusY: number, rotation: number, startAngle: number, endAngle: number): void;

	initWithData(data: string): this;

	initWithPath(path: TNSPath2D): this;

	lineTo(x: number, y: number): void;

	moveTo(x: number, y: number): void;

	quadraticCurveTo(cpx: number, cpy: number, x: number, y: number): void;

	rect(x: number, y: number, width: number, height: number): void;
}

declare class TNSPattern extends NSObject implements ICanvasColorStyle {

	static alloc(): TNSPattern; // inherited from NSObject

	static new(): TNSPattern; // inherited from NSObject

	constructor(o: { asset: TNSImageAsset; pattern: TNSPatternRepetition; });

	constructor(o: { canvas: TNSCanvas; pattern: TNSPatternRepetition; });

	constructor(o: { src: UIImage; pattern: TNSPatternRepetition; });

	getStyleType(): CanvasColorStyleType;

	initWithAssetPattern(src: TNSImageAsset, pattern: TNSPatternRepetition): this;

	initWithCanvasPattern(canvas: TNSCanvas, pattern: TNSPatternRepetition): this;

	initWithSrcPattern(src: UIImage, pattern: TNSPatternRepetition): this;

	setTransformWithMatrix(matrix: TNSDOMMatrix): void;
}

declare const enum TNSPatternRepetition {

	Repeat = 0,

	RepeatX = 1,

	RepeatY = 2,

	NoRepeat = 3
}

declare class TNSRadialGradient extends Gradient {

	static alloc(): TNSRadialGradient; // inherited from NSObject

	static new(): TNSRadialGradient; // inherited from NSObject

	constructor(o: { x0: number; y0: number; r0: number; x1: number; y1: number; r1: number; });

	initWithX0Y0R0X1Y1R1(x0: number, y0: number, r0: number, x1: number, y1: number, r1: number): this;
}

declare const enum TNSTextAlignment {

	Left = 0,

	Start = 1,

	Center = 2,

	End = 3,

	Right = 4
}

declare class TNSTextDecoder extends NSObject {

	static alloc(): TNSTextDecoder; // inherited from NSObject

	static new(): TNSTextDecoder; // inherited from NSObject

	readonly encoding: string;

	constructor(o: { encoding: string; });

	decodeWithBuffer(buffer: NSData): string;

	decodeWithBytes(bytes: NSArray<number> | number[]): string;

	decodeWithI16(bytes: NSArray<number> | number[]): string;

	decodeWithI32(bytes: NSArray<number> | number[]): string;

	decodeWithI8(bytes: NSArray<number> | number[]): string;

	decodeWithU16(bytes: NSArray<number> | number[]): string;

	initWithEncoding(encoding: string): this;
}

declare class TNSTextEncoder extends NSObject {

	static alloc(): TNSTextEncoder; // inherited from NSObject

	static new(): TNSTextEncoder; // inherited from NSObject

	readonly encoding: string;

	constructor(o: { encoding: string; });

	encodeWithPointer(text: interop.Pointer | interop.Reference<number>): NSData;

	encodeWithText(text: string): NSData;

	initWithEncoding(encoding: string): this;
}

declare class TNSTextMetrics extends NSObject {

	static alloc(): TNSTextMetrics; // inherited from NSObject

	static new(): TNSTextMetrics; // inherited from NSObject

	readonly width: number;
}

declare class TNSWebGL2RenderingContext extends TNSWebGLRenderingContext {

	static alloc(): TNSWebGL2RenderingContext; // inherited from NSObject

	static new(): TNSWebGL2RenderingContext; // inherited from NSObject

	readonly ACTIVE_UNIFORM_BLOCKS: number;

	readonly ALREADY_SIGNALED: number;

	readonly ANY_SAMPLES_PASSED: number;

	readonly ANY_SAMPLES_PASSED_CONSERVATIVE: number;

	readonly COLOR: number;

	readonly COLOR_ATTACHMENT1: number;

	readonly COLOR_ATTACHMENT10: number;

	readonly COLOR_ATTACHMENT11: number;

	readonly COLOR_ATTACHMENT12: number;

	readonly COLOR_ATTACHMENT13: number;

	readonly COLOR_ATTACHMENT14: number;

	readonly COLOR_ATTACHMENT15: number;

	readonly COLOR_ATTACHMENT2: number;

	readonly COLOR_ATTACHMENT3: number;

	readonly COLOR_ATTACHMENT4: number;

	readonly COLOR_ATTACHMENT5: number;

	readonly COLOR_ATTACHMENT6: number;

	readonly COLOR_ATTACHMENT7: number;

	readonly COLOR_ATTACHMENT8: number;

	readonly COLOR_ATTACHMENT9: number;

	readonly COMPARE_REF_TO_TEXTURE: number;

	readonly CONDITION_SATISFIED: number;

	readonly COPY_READ_BUFFER: number;

	readonly COPY_READ_BUFFER_BINDING: number;

	readonly COPY_WRITE_BUFFER: number;

	readonly COPY_WRITE_BUFFER_BINDING: number;

	readonly CURRENT_QUERY: number;

	readonly DEPTH: number;

	readonly DEPTH24_STENCIL8: number;

	readonly DEPTH32F_STENCIL8: number;

	readonly DEPTH_COMPONENT24: number;

	readonly DEPTH_COMPONENT32F: number;

	readonly DRAW_BUFFER0: number;

	readonly DRAW_BUFFER1: number;

	readonly DRAW_BUFFER10: number;

	readonly DRAW_BUFFER11: number;

	readonly DRAW_BUFFER12: number;

	readonly DRAW_BUFFER13: number;

	readonly DRAW_BUFFER14: number;

	readonly DRAW_BUFFER15: number;

	readonly DRAW_BUFFER2: number;

	readonly DRAW_BUFFER3: number;

	readonly DRAW_BUFFER4: number;

	readonly DRAW_BUFFER5: number;

	readonly DRAW_BUFFER6: number;

	readonly DRAW_BUFFER7: number;

	readonly DRAW_BUFFER8: number;

	readonly DRAW_BUFFER9: number;

	readonly DRAW_FRAMEBUFFER: number;

	readonly DRAW_FRAMEBUFFER_BINDING: number;

	readonly DYNAMIC_COPY: number;

	readonly DYNAMIC_READ: number;

	readonly FLOAT_32_UNSIGNED_INT_24_8_REV: number;

	readonly FLOAT_MAT2x3: number;

	readonly FLOAT_MAT2x4: number;

	readonly FLOAT_MAT3x2: number;

	readonly FLOAT_MAT3x4: number;

	readonly FLOAT_MAT4x2: number;

	readonly FLOAT_MAT4x3: number;

	readonly FRAGMENT_SHADER_DERIVATIVE_HINT: number;

	readonly FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: number;

	readonly FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: number;

	readonly FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: number;

	readonly FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: number;

	readonly FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: number;

	readonly FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: number;

	readonly FRAMEBUFFER_ATTACHMENT_RED_SIZE: number;

	readonly FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: number;

	readonly FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: number;

	readonly FRAMEBUFFER_DEFAULT: number;

	readonly FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: number;

	readonly HALF_FLOAT: number;

	readonly INTERLEAVED_ATTRIBS: number;

	readonly INT_2_10_10_10_REV: number;

	readonly INT_SAMPLER_2D: number;

	readonly INT_SAMPLER_2D_ARRAY: number;

	readonly INT_SAMPLER_3D: number;

	readonly INT_SAMPLER_CUBE: number;

	readonly INVALID_INDEX: number;

	readonly MAX: number;

	readonly MAX_3D_TEXTURE_SIZE: number;

	readonly MAX_ARRAY_TEXTURE_LAYERS: number;

	readonly MAX_CLIENT_WAIT_TIMEOUT_WEBGL: number;

	readonly MAX_COLOR_ATTACHMENTS: number;

	readonly MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: number;

	readonly MAX_COMBINED_UNIFORM_BLOCKS: number;

	readonly MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: number;

	readonly MAX_DRAW_BUFFERS: number;

	readonly MAX_ELEMENTS_INDICES: number;

	readonly MAX_ELEMENTS_VERTICES: number;

	readonly MAX_ELEMENT_INDEX: number;

	readonly MAX_FRAGMENT_INPUT_COMPONENTS: number;

	readonly MAX_FRAGMENT_UNIFORM_BLOCKS: number;

	readonly MAX_FRAGMENT_UNIFORM_COMPONENTS: number;

	readonly MAX_PROGRAM_TEXEL_OFFSET: number;

	readonly MAX_SAMPLES: number;

	readonly MAX_SERVER_WAIT_TIMEOUT: number;

	readonly MAX_TEXTURE_LOD_BIAS: number;

	readonly MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: number;

	readonly MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: number;

	readonly MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: number;

	readonly MAX_UNIFORM_BLOCK_SIZE: number;

	readonly MAX_UNIFORM_BUFFER_BINDINGS: number;

	readonly MAX_VARYING_COMPONENTS: number;

	readonly MAX_VERTEX_OUTPUT_COMPONENTS: number;

	readonly MAX_VERTEX_UNIFORM_BLOCKS: number;

	readonly MAX_VERTEX_UNIFORM_COMPONENTS: number;

	readonly MIN: number;

	readonly MIN_PROGRAM_TEXEL_OFFSET: number;

	readonly OBJECT_TYPE: number;

	readonly PACK_ROW_LENGTH: number;

	readonly PACK_SKIP_PIXELS: number;

	readonly PACK_SKIP_ROWS: number;

	readonly PIXEL_PACK_BUFFER: number;

	readonly PIXEL_PACK_BUFFER_BINDING: number;

	readonly PIXEL_UNPACK_BUFFER: number;

	readonly PIXEL_UNPACK_BUFFER_BINDING: number;

	readonly QUERY_RESULT: number;

	readonly QUERY_RESULT_AVAILABLE: number;

	readonly R11F_G11F_B10F: number;

	readonly R16F: number;

	readonly R16I: number;

	readonly R16UI: number;

	readonly R32F: number;

	readonly R32I: number;

	readonly R32UI: number;

	readonly R8: number;

	readonly R8I: number;

	readonly R8UI: number;

	readonly R8_SNORM: number;

	readonly RASTERIZER_DISCARD: number;

	readonly READ_BUFFER: number;

	readonly READ_FRAMEBUFFER: number;

	readonly READ_FRAMEBUFFER_BINDING: number;

	readonly RED: number;

	readonly RED_INTEGER: number;

	readonly RENDERBUFFER_SAMPLES: number;

	readonly RG: number;

	readonly RG16F: number;

	readonly RG16I: number;

	readonly RG16UI: number;

	readonly RG32F: number;

	readonly RG32I: number;

	readonly RG32UI: number;

	readonly RG8: number;

	readonly RG8I: number;

	readonly RG8UI: number;

	readonly RG8_SNORM: number;

	readonly RGB10_A2: number;

	readonly RGB10_A2UI: number;

	readonly RGB16F: number;

	readonly RGB16I: number;

	readonly RGB16UI: number;

	readonly RGB32F: number;

	readonly RGB32I: number;

	readonly RGB32UI: number;

	readonly RGB8: number;

	readonly RGB8I: number;

	readonly RGB8UI: number;

	readonly RGB8_SNORM: number;

	readonly RGB9_E5: number;

	readonly RGBA16F: number;

	readonly RGBA16I: number;

	readonly RGBA16UI: number;

	readonly RGBA32F: number;

	readonly RGBA32I: number;

	readonly RGBA32UI: number;

	readonly RGBA8: number;

	readonly RGBA8I: number;

	readonly RGBA8UI: number;

	readonly RGBA8_SNORM: number;

	readonly RGBA_INTEGER: number;

	readonly RGB_INTEGER: number;

	readonly RG_INTEGER: number;

	readonly SAMPLER_2D_ARRAY: number;

	readonly SAMPLER_2D_ARRAY_SHADOW: number;

	readonly SAMPLER_2D_SHADOW: number;

	readonly SAMPLER_3D: number;

	readonly SAMPLER_BINDING: number;

	readonly SAMPLER_CUBE_SHADOW: number;

	readonly SEPARATE_ATTRIBS: number;

	readonly SIGNALED: number;

	readonly SIGNED_NORMALIZED: number;

	readonly SRGB: number;

	readonly SRGB8: number;

	readonly SRGB8_ALPHA8: number;

	readonly STATIC_COPY: number;

	readonly STATIC_READ: number;

	readonly STENCIL: number;

	readonly STREAM_COPY: number;

	readonly STREAM_READ: number;

	readonly SYNC_CONDITION: number;

	readonly SYNC_FENCE: number;

	readonly SYNC_FLAGS: number;

	readonly SYNC_FLUSH_COMMANDS_BIT: number;

	readonly SYNC_GPU_COMMANDS_COMPLETE: number;

	readonly SYNC_STATUS: number;

	readonly TEXTURE_2D_ARRAY: number;

	readonly TEXTURE_3D: number;

	readonly TEXTURE_BASE_LEVEL: number;

	readonly TEXTURE_BINDING_2D_ARRAY: number;

	readonly TEXTURE_BINDING_3D: number;

	readonly TEXTURE_COMPARE_FUNC: number;

	readonly TEXTURE_COMPARE_MODE: number;

	readonly TEXTURE_IMMUTABLE_FORMAT: number;

	readonly TEXTURE_IMMUTABLE_LEVELS: number;

	readonly TEXTURE_MAX_LEVEL: number;

	readonly TEXTURE_MAX_LOD: number;

	readonly TEXTURE_MIN_LOD: number;

	readonly TEXTURE_WRAP_R: number;

	readonly TIMEOUT_EXPIRED: number;

	readonly TIMEOUT_IGNORED: number;

	readonly TRANSFORM_FEEDBACK: number;

	readonly TRANSFORM_FEEDBACK_ACTIVE: number;

	readonly TRANSFORM_FEEDBACK_BINDING: number;

	readonly TRANSFORM_FEEDBACK_BUFFER: number;

	readonly TRANSFORM_FEEDBACK_BUFFER_BINDING: number;

	readonly TRANSFORM_FEEDBACK_BUFFER_MODE: number;

	readonly TRANSFORM_FEEDBACK_BUFFER_SIZE: number;

	readonly TRANSFORM_FEEDBACK_BUFFER_START: number;

	readonly TRANSFORM_FEEDBACK_PAUSED: number;

	readonly TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: number;

	readonly TRANSFORM_FEEDBACK_VARYINGS: number;

	readonly UNIFORM_ARRAY_STRIDE: number;

	readonly UNIFORM_BLOCK_ACTIVE_UNIFORMS: number;

	readonly UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: number;

	readonly UNIFORM_BLOCK_BINDING: number;

	readonly UNIFORM_BLOCK_DATA_SIZE: number;

	readonly UNIFORM_BLOCK_INDEX: number;

	readonly UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: number;

	readonly UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: number;

	readonly UNIFORM_BUFFER: number;

	readonly UNIFORM_BUFFER_BINDING: number;

	readonly UNIFORM_BUFFER_OFFSET_ALIGNMENT: number;

	readonly UNIFORM_BUFFER_SIZE: number;

	readonly UNIFORM_BUFFER_START: number;

	readonly UNIFORM_IS_ROW_MAJOR: number;

	readonly UNIFORM_MATRIX_STRIDE: number;

	readonly UNIFORM_OFFSET: number;

	readonly UNIFORM_SIZE: number;

	readonly UNIFORM_TYPE: number;

	readonly UNPACK_IMAGE_HEIGHT: number;

	readonly UNPACK_ROW_LENGTH: number;

	readonly UNPACK_SKIP_IMAGES: number;

	readonly UNPACK_SKIP_PIXELS: number;

	readonly UNPACK_SKIP_ROWS: number;

	readonly UNSIGNALED: number;

	readonly UNSIGNED_INT_10F_11F_11F_REV: number;

	readonly UNSIGNED_INT_24_8: number;

	readonly UNSIGNED_INT_2_10_10_10_REV: number;

	readonly UNSIGNED_INT_5_9_9_9_REV: number;

	readonly UNSIGNED_INT_SAMPLER_2D: number;

	readonly UNSIGNED_INT_SAMPLER_2D_ARRAY: number;

	readonly UNSIGNED_INT_SAMPLER_3D: number;

	readonly UNSIGNED_INT_SAMPLER_CUBE: number;

	readonly UNSIGNED_INT_VEC2: number;

	readonly UNSIGNED_INT_VEC3: number;

	readonly UNSIGNED_INT_VEC4: number;

	readonly UNSIGNED_NORMALIZED: number;

	readonly VERTEX_ARRAY_BINDING: number;

	readonly VERTEX_ATTRIB_ARRAY_DIVISOR: number;

	readonly VERTEX_ATTRIB_ARRAY_INTEGER: number;

	readonly WAIT_FAILED: number;

	beginQuery(target: number, query: number): void;

	beginTransformFeedback(primitiveMode: number): void;

	bindBufferBase(target: number, index: number, buffer: number): void;

	bindBufferRange(target: number, index: number, buffer: number, offset: number, size: number): void;

	bindSampler(unit: number, sampler: number): void;

	bindTransformFeedback(target: number, transformFeedback: number): void;

	bindVertexArray(vertexArray: number): void;

	blitFramebuffer(srcX0: number, srcY0: number, srcX1: number, srcY1: number, dstX0: number, dstY0: number, dstX1: number, dstY1: number, mask: number, filter: number): void;

	clearBufferfi(buffer: number, drawbuffer: number, depth: number, stencil: number): void;

	clearBufferfv(buffer: number, drawbuffer: number, values: NSArray<number> | number[]): void;

	clearBufferiv(buffer: number, drawbuffer: number, values: NSArray<number> | number[]): void;

	clearBufferuiv(buffer: number, drawbuffer: number, values: NSArray<number> | number[]): void;

	clientWaitSync(sync: interop.Pointer | interop.Reference<any>, flags: number, timeout: number): number;

	compressedTexSubImage3D(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, imageSize: number, offset: number): void;

	compressedTexSubImage3DF32(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, srcData: NSArray<number> | number[], srcOffset: number, srcLengthOverride: number): void;

	compressedTexSubImage3DF64(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, srcData: NSArray<number> | number[], srcOffset: number, srcLengthOverride: number): void;

	compressedTexSubImage3DI16(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, srcData: NSArray<number> | number[], srcOffset: number, srcLengthOverride: number): void;

	compressedTexSubImage3DI32(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, srcData: NSArray<number> | number[], srcOffset: number, srcLengthOverride: number): void;

	compressedTexSubImage3DI8(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, srcData: NSArray<number> | number[], srcOffset: number, srcLengthOverride: number): void;

	compressedTexSubImage3DU16(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, srcData: NSArray<number> | number[], srcOffset: number, srcLengthOverride: number): void;

	compressedTexSubImage3DU32(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, srcData: NSArray<number> | number[], srcOffset: number, srcLengthOverride: number): void;

	compressedTexSubImage3DU8(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, srcData: NSArray<number> | number[], srcOffset: number, srcLengthOverride: number): void;

	copyBufferSubData(readTarget: number, writeTarget: number, readOffset: number, writeOffset: number, size: number): void;

	copyTexSubImage3D(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, x: number, y: number, width: number, height: number): void;

	createQuery(): number;

	createSampler(): number;

	createTransformFeedback(): number;

	createVertexArray(): number;

	deleteQuery(query: number): void;

	deleteSampler(sampler: number): void;

	deleteSync(sync: interop.Pointer | interop.Reference<any>): void;

	deleteTransformFeedback(transformFeedback: number): void;

	deleteVertexArray(vertexArray: number): void;

	drawArraysInstanced(mode: number, first: number, count: number, instanceCount: number): void;

	drawBuffers(buffers: NSArray<number> | number[]): void;

	drawElementsInstanced(mode: number, count: number, type: number, offset: number, instanceCount: number): void;

	drawRangeElements(mode: number, start: number, end: number, count: number, type: number, offset: number): void;

	endQuery(target: number): void;

	endTransformFeedback(): void;

	fenceSync(condition: number, flags: number): void;

	framebufferTextureLayer(target: number, attachment: number, texture: number, level: number, layer: number): void;

	getActiveUniformBlockName(program: number, uniformBlockIndex: number): string;

	getActiveUniformBlockParameter(program: number, uniformBlockIndex: number, pname: number): any;

	getActiveUniforms(program: number, uniformIndices: NSArray<number> | number[], pname: number): any;

	getBufferSubData(target: number, srcByteOffset: number, dstData: NSData, dstOffset: number, length: number): void;

	getFragDataLocation(program: number, name: string): number;

	getIndexedParameter(target: number, index: number): any;

	getInternalformatParameter(target: number, internalformat: number, pname: number): any;

	getQuery(target: number, pname: number): any;

	getQueryParameter(query: number, pname: number): any;

	getSamplerParameter(sampler: number, pname: number): any;

	getSyncParameter(sync: interop.Pointer | interop.Reference<any>, pname: number): any;

	getTransformFeedbackVarying(program: number, index: number): any;

	getUniformBlockIndex(program: number, uniformBlockName: string): number;

	getUniformIndices(program: number, uniformNames: NSArray<string> | string[]): NSArray<number>;

	invalidateFramebuffer(target: number, attachments: NSArray<number> | number[]): void;

	invalidateSubFramebuffer(target: number, attachments: NSArray<number> | number[], x: number, y: number, width: number, height: number): void;

	isQuery(query: number): boolean;

	isSampler(sampler: number): boolean;

	isSync(sync: interop.Pointer | interop.Reference<any>): boolean;

	isTransformFeedback(transformFeedback: number): boolean;

	isVertexArray(vertexArray: number): boolean;

	pauseTransformFeedback(): void;

	readBuffer(src: number): void;

	renderbufferStorageMultisample(target: number, samples: number, internalFormat: number, width: number, height: number): void;

	resumeTransformFeedback(): void;

	samplerParameterf(sampler: number, pname: number, param: number): void;

	samplerParameteri(sampler: number, pname: number, param: number): void;

	texImage3DAsset(target: number, level: number, internalformat: number, width: number, height: number, depth: number, border: number, format: number, type: number, asset: TNSImageAsset): void;

	texImage3DCanvas(target: number, level: number, internalformat: number, width: number, height: number, depth: number, border: number, format: number, type: number, canvas: TNSCanvas): void;

	texImage3DF32(target: number, level: number, internalformat: number, width: number, height: number, depth: number, border: number, format: number, type: number, source: NSArray<number> | number[]): void;

	texImage3DF64(target: number, level: number, internalformat: number, width: number, height: number, depth: number, border: number, format: number, type: number, source: NSArray<number> | number[]): void;

	texImage3DI16(target: number, level: number, internalformat: number, width: number, height: number, depth: number, border: number, format: number, type: number, source: NSArray<number> | number[]): void;

	texImage3DI32(target: number, level: number, internalformat: number, width: number, height: number, depth: number, border: number, format: number, type: number, source: NSArray<number> | number[]): void;

	texImage3DI8(target: number, level: number, internalformat: number, width: number, height: number, depth: number, border: number, format: number, type: number, source: NSArray<number> | number[]): void;

	texImage3DOffset(target: number, level: number, internalformat: number, width: number, height: number, depth: number, border: number, format: number, type: number, offset: number): void;

	texImage3DSource(target: number, level: number, internalformat: number, width: number, height: number, depth: number, border: number, format: number, type: number, source: UIImage): void;

	texImage3DU16(target: number, level: number, internalformat: number, width: number, height: number, depth: number, border: number, format: number, type: number, source: NSArray<number> | number[]): void;

	texImage3DU32(target: number, level: number, internalformat: number, width: number, height: number, depth: number, border: number, format: number, type: number, source: NSArray<number> | number[]): void;

	texImage3DU8(target: number, level: number, internalformat: number, width: number, height: number, depth: number, border: number, format: number, type: number, source: NSArray<number> | number[]): void;

	texStorage2D(target: number, levels: number, internalformat: number, width: number, height: number): void;

	texStorage3D(target: number, levels: number, internalformat: number, width: number, height: number, depth: number): void;

	texSubImage3DAsset(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type: number, asset: TNSImageAsset): void;

	texSubImage3DCanvas(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type: number, canvas: TNSCanvas): void;

	texSubImage3DF32(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type: number, srcData: NSArray<number> | number[], srcOffset: number): void;

	texSubImage3DF64(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type: number, srcData: NSArray<number> | number[]): void;

	texSubImage3DI16(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type: number, srcData: NSArray<number> | number[]): void;

	texSubImage3DI32(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type: number, srcData: NSArray<number> | number[]): void;

	texSubImage3DI8(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type: number, srcData: NSArray<number> | number[], srcOffset: number): void;

	texSubImage3DOffset(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type: number, offset: number): void;

	texSubImage3DSrcData(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type: number, srcData: UIImage): void;

	texSubImage3DU16(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type: number, srcData: NSArray<number> | number[]): void;

	texSubImage3DU32(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type: number, srcData: NSArray<number> | number[]): void;

	texSubImage3DU8(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, type: number, srcData: NSArray<number> | number[], srcOffset: number): void;

	transformFeedbackVaryings(program: number, varyings: NSArray<string> | string[], bufferMode: number): void;

	uniform1ui(location: number, v0: number): void;

	uniform1uiv(location: number, data: NSData): void;

	uniform2ui(location: number, v0: number, v1: number): void;

	uniform2uiv(location: number, data: NSData): void;

	uniform3ui(location: number, v0: number, v1: number, v2: number): void;

	uniform3uiv(location: number, data: NSData): void;

	uniform4ui(location: number, v0: number, v1: number, v2: number, v3: number): void;

	uniform4uiv(location: number, data: NSData): void;

	uniformBlockBinding(program: number, uniformBlockIndex: number, uniformBlockBinding: number): void;

	uniformMatrix2x3fv(location: number, transpose: boolean, data: NSData): void;

	uniformMatrix2x4fv(location: number, transpose: boolean, data: NSData): void;

	uniformMatrix3x2fv(location: number, transpose: boolean, data: NSData): void;

	uniformMatrix3x4fv(location: number, transpose: boolean, data: NSData): void;

	uniformMatrix4x2fv(location: number, transpose: boolean, data: NSData): void;

	uniformMatrix4x3fv(location: number, transpose: boolean, data: NSData): void;

	vertexAttribDivisor(index: number, divisor: number): void;

	vertexAttribI4i(index: number, v0: number, v1: number, v2: number, v3: number): void;

	vertexAttribI4iv(index: number, value: NSData): void;

	vertexAttribI4ui(index: number, v0: number, v1: number, v2: number, v3: number): void;

	vertexAttribI4uiv(index: number, value: NSData): void;
}

declare class TNSWebGLActiveInfo extends NSObject {

	static alloc(): TNSWebGLActiveInfo; // inherited from NSObject

	static new(): TNSWebGLActiveInfo; // inherited from NSObject

	readonly name: string;

	readonly size: number;

	readonly type: number;

	constructor(o: { name: string; size: number; type: number; });

	initWithNameSizeType(name: string, size: number, type: number): this;
}

declare class TNSWebGLRenderingContext extends TNSCanvasRenderingContext {

	static alloc(): TNSWebGLRenderingContext; // inherited from NSObject

	static new(): TNSWebGLRenderingContext; // inherited from NSObject

	readonly ACTIVE_ATTRIBUTES: number;

	readonly ACTIVE_TEXTURE: number;

	readonly ACTIVE_UNIFORMS: number;

	readonly ALIASED_LINE_WIDTH_RANGE: number;

	readonly ALIASED_POINT_SIZE_RANGE: number;

	readonly ALPHA: number;

	readonly ALPHA_BITS: number;

	readonly ALWAYS: number;

	readonly ARRAY_BUFFER: number;

	readonly ARRAY_BUFFER_BINDING: number;

	readonly ATTACHED_SHADERS: number;

	readonly BACK: number;

	readonly BLEND: number;

	readonly BLEND_COLOR: number;

	readonly BLEND_DST_ALPHA: number;

	readonly BLEND_DST_RGB: number;

	readonly BLEND_EQUATION: number;

	readonly BLEND_EQUATION_ALPHA: number;

	readonly BLEND_EQUATION_RGB: number;

	readonly BLEND_SRC_ALPHA: number;

	readonly BLEND_SRC_RGB: number;

	readonly BLUE_BITS: number;

	readonly BOOL: number;

	readonly BOOL_VEC2: number;

	readonly BOOL_VEC3: number;

	readonly BOOL_VEC4: number;

	readonly BROWSER_DEFAULT_WEBGL: number;

	readonly BUFFER_SIZE: number;

	readonly BUFFER_USAGE: number;

	readonly BYTE: number;

	readonly CCW: number;

	readonly CLAMP_TO_EDGE: number;

	readonly COLOR_ATTACHMENT0: number;

	readonly COLOR_BUFFER_BIT: number;

	readonly COLOR_CLEAR_VALUE: number;

	readonly COLOR_WRITEMASK: number;

	readonly COMPILE_STATUS: number;

	readonly COMPRESSED_TEXTURE_FORMATS: number;

	readonly CONSTANT_ALPHA: number;

	readonly CONSTANT_COLOR: number;

	readonly CONTEXT_LOST_WEBGL: number;

	readonly CULL_FACE: number;

	readonly CULL_FACE_MODE: number;

	readonly CURRENT_PROGRAM: number;

	readonly CURRENT_VERTEX_ATTRIB: number;

	readonly CW: number;

	readonly DECR: number;

	readonly DECR_WRAP: number;

	readonly DELETE_STATUS: number;

	readonly DEPTH_ATTACHMENT: number;

	readonly DEPTH_BITS: number;

	readonly DEPTH_BUFFER_BIT: number;

	readonly DEPTH_CLEAR_VALUE: number;

	readonly DEPTH_COMPONENT: number;

	readonly DEPTH_COMPONENT16: number;

	readonly DEPTH_FUNC: number;

	readonly DEPTH_RANGE: number;

	readonly DEPTH_STENCIL: number;

	readonly DEPTH_STENCIL_ATTACHMENT: number;

	readonly DEPTH_TEST: number;

	readonly DEPTH_WRITEMASK: number;

	readonly DITHER: number;

	readonly DONT_CARE: number;

	readonly DST_ALPHA: number;

	readonly DST_COLOR: number;

	readonly DYNAMIC_DRAW: number;

	readonly ELEMENT_ARRAY_BUFFER: number;

	readonly ELEMENT_ARRAY_BUFFER_BINDING: number;

	readonly EQUAL: number;

	readonly FASTEST: number;

	readonly FLOAT: number;

	readonly FLOAT_MAT2: number;

	readonly FLOAT_MAT3: number;

	readonly FLOAT_MAT4: number;

	readonly FLOAT_VEC2: number;

	readonly FLOAT_VEC3: number;

	readonly FLOAT_VEC4: number;

	readonly FRAGMENT_SHADER: number;

	readonly FRAMEBUFFER: number;

	readonly FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: number;

	readonly FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: number;

	readonly FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: number;

	readonly FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: number;

	readonly FRAMEBUFFER_BINDING: number;

	readonly FRAMEBUFFER_COMPLETE: number;

	readonly FRAMEBUFFER_INCOMPLETE_ATTACHMENT: number;

	readonly FRAMEBUFFER_INCOMPLETE_DIMENSIONS: number;

	readonly FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: number;

	readonly FRAMEBUFFER_UNSUPPORTED: number;

	readonly FRONT: number;

	readonly FRONT_AND_BACK: number;

	readonly FRONT_FACE: number;

	readonly FUNC_ADD: number;

	readonly FUNC_REVERSE_SUBTRACT: number;

	readonly FUNC_SUBTRACT: number;

	readonly GENERATE_MIPMAP_HINT: number;

	readonly GEQUAL: number;

	readonly GREATER: number;

	readonly GREEN_BITS: number;

	readonly HIGH_FLOAT: number;

	readonly HIGH_INT: number;

	readonly IMPLEMENTATION_COLOR_READ_FORMAT: number;

	readonly IMPLEMENTATION_COLOR_READ_TYPE: number;

	readonly INCR: number;

	readonly INCR_WRAP: number;

	readonly INT: number;

	readonly INT_VEC2: number;

	readonly INT_VEC3: number;

	readonly INT_VEC4: number;

	readonly INVALID_ENUM: number;

	readonly INVALID_FRAMEBUFFER_OPERATION: number;

	readonly INVALID_OPERATION: number;

	readonly INVALID_VALUE: number;

	readonly INVERT: number;

	readonly KEEP: number;

	readonly LEQUAL: number;

	readonly LESS: number;

	readonly LINEAR: number;

	readonly LINEAR_MIPMAP_LINEAR: number;

	readonly LINEAR_MIPMAP_NEAREST: number;

	readonly LINES: number;

	readonly LINE_LOOP: number;

	readonly LINE_STRIP: number;

	readonly LINE_WIDTH: number;

	readonly LINK_STATUS: number;

	readonly LOW_FLOAT: number;

	readonly LOW_INT: number;

	readonly LUMINANCE: number;

	readonly LUMINANCE_ALPHA: number;

	readonly MAX_COMBINED_TEXTURE_IMAGE_UNITS: number;

	readonly MAX_CUBE_MAP_TEXTURE_SIZE: number;

	readonly MAX_FRAGMENT_UNIFORM_VECTORS: number;

	readonly MAX_RENDERBUFFER_SIZE: number;

	readonly MAX_TEXTURE_IMAGE_UNITS: number;

	readonly MAX_TEXTURE_SIZE: number;

	readonly MAX_VARYING_VECTORS: number;

	readonly MAX_VERTEX_ATTRIBS: number;

	readonly MAX_VERTEX_TEXTURE_IMAGE_UNITS: number;

	readonly MAX_VERTEX_UNIFORM_VECTORS: number;

	readonly MAX_VIEWPORT_DIMS: number;

	readonly MEDIUM_FLOAT: number;

	readonly MEDIUM_INT: number;

	readonly MIRRORED_REPEAT: number;

	readonly NEAREST: number;

	readonly NEAREST_MIPMAP_LINEAR: number;

	readonly NEAREST_MIPMAP_NEAREST: number;

	readonly NEVER: number;

	readonly NICEST: number;

	readonly NONE: number;

	readonly NOTEQUAL: number;

	readonly NO_ERROR: number;

	readonly ONE: number;

	readonly ONE_MINUS_CONSTANT_ALPHA: number;

	readonly ONE_MINUS_CONSTANT_COLOR: number;

	readonly ONE_MINUS_DST_ALPHA: number;

	readonly ONE_MINUS_DST_COLOR: number;

	readonly ONE_MINUS_SRC_ALPHA: number;

	readonly ONE_MINUS_SRC_COLOR: number;

	readonly OUT_OF_MEMORY: number;

	readonly PACK_ALIGNMENT: number;

	readonly POINTS: number;

	readonly POLYGON_OFFSET_FACTOR: number;

	readonly POLYGON_OFFSET_FILL: number;

	readonly POLYGON_OFFSET_UNITS: number;

	readonly RED_BITS: number;

	readonly RENDERBUFFER: number;

	readonly RENDERBUFFER_ALPHA_SIZE: number;

	readonly RENDERBUFFER_BINDING: number;

	readonly RENDERBUFFER_BLUE_SIZE: number;

	readonly RENDERBUFFER_DEPTH_SIZE: number;

	readonly RENDERBUFFER_GREEN_SIZE: number;

	readonly RENDERBUFFER_HEIGHT: number;

	readonly RENDERBUFFER_INTERNAL_FORMAT: number;

	readonly RENDERBUFFER_RED_SIZE: number;

	readonly RENDERBUFFER_STENCIL_SIZE: number;

	readonly RENDERBUFFER_WIDTH: number;

	readonly RENDERER: number;

	readonly REPEAT: number;

	readonly REPLACE: number;

	readonly RGB: number;

	readonly RGB565: number;

	readonly RGB5_A1: number;

	readonly RGBA: number;

	readonly RGBA4: number;

	readonly SAMPLER_2D: number;

	readonly SAMPLER_CUBE: number;

	readonly SAMPLES: number;

	readonly SAMPLE_ALPHA_TO_COVERAGE: number;

	readonly SAMPLE_BUFFERS: number;

	readonly SAMPLE_COVERAGE: number;

	readonly SAMPLE_COVERAGE_INVERT: number;

	readonly SAMPLE_COVERAGE_VALUE: number;

	readonly SCISSOR_BOX: number;

	readonly SCISSOR_TEST: number;

	readonly SHADER_TYPE: number;

	readonly SHADING_LANGUAGE_VERSION: number;

	readonly SHORT: number;

	readonly SRC_ALPHA: number;

	readonly SRC_ALPHA_SATURATE: number;

	readonly SRC_COLOR: number;

	readonly STATIC_DRAW: number;

	readonly STENCIL_ATTACHMENT: number;

	readonly STENCIL_BACK_FAIL: number;

	readonly STENCIL_BACK_FUNC: number;

	readonly STENCIL_BACK_PASS_DEPTH_FAIL: number;

	readonly STENCIL_BACK_PASS_DEPTH_PASS: number;

	readonly STENCIL_BACK_REF: number;

	readonly STENCIL_BACK_VALUE_MASK: number;

	readonly STENCIL_BACK_WRITEMASK: number;

	readonly STENCIL_BITS: number;

	readonly STENCIL_BUFFER_BIT: number;

	readonly STENCIL_CLEAR_VALUE: number;

	readonly STENCIL_FAIL: number;

	readonly STENCIL_FUNC: number;

	readonly STENCIL_INDEX8: number;

	readonly STENCIL_PASS_DEPTH_FAIL: number;

	readonly STENCIL_PASS_DEPTH_PASS: number;

	readonly STENCIL_REF: number;

	readonly STENCIL_TEST: number;

	readonly STENCIL_VALUE_MASK: number;

	readonly STENCIL_WRITEMASK: number;

	readonly STREAM_DRAW: number;

	readonly SUBPIXEL_BITS: number;

	readonly TEXTURE: number;

	readonly TEXTURE0: number;

	readonly TEXTURE1: number;

	readonly TEXTURE10: number;

	readonly TEXTURE11: number;

	readonly TEXTURE12: number;

	readonly TEXTURE13: number;

	readonly TEXTURE14: number;

	readonly TEXTURE15: number;

	readonly TEXTURE16: number;

	readonly TEXTURE17: number;

	readonly TEXTURE18: number;

	readonly TEXTURE19: number;

	readonly TEXTURE2: number;

	readonly TEXTURE20: number;

	readonly TEXTURE21: number;

	readonly TEXTURE22: number;

	readonly TEXTURE23: number;

	readonly TEXTURE24: number;

	readonly TEXTURE25: number;

	readonly TEXTURE26: number;

	readonly TEXTURE27: number;

	readonly TEXTURE28: number;

	readonly TEXTURE29: number;

	readonly TEXTURE3: number;

	readonly TEXTURE30: number;

	readonly TEXTURE31: number;

	readonly TEXTURE4: number;

	readonly TEXTURE5: number;

	readonly TEXTURE6: number;

	readonly TEXTURE7: number;

	readonly TEXTURE8: number;

	readonly TEXTURE9: number;

	readonly TEXTURE_2D: number;

	readonly TEXTURE_BINDING_2D: number;

	readonly TEXTURE_BINDING_CUBE_MAP: number;

	readonly TEXTURE_CUBE_MAP: number;

	readonly TEXTURE_CUBE_MAP_NEGATIVE_X: number;

	readonly TEXTURE_CUBE_MAP_NEGATIVE_Y: number;

	readonly TEXTURE_CUBE_MAP_NEGATIVE_Z: number;

	readonly TEXTURE_CUBE_MAP_POSITIVE_X: number;

	readonly TEXTURE_CUBE_MAP_POSITIVE_Y: number;

	readonly TEXTURE_CUBE_MAP_POSITIVE_Z: number;

	readonly TEXTURE_MAG_FILTER: number;

	readonly TEXTURE_MIN_FILTER: number;

	readonly TEXTURE_WRAP_S: number;

	readonly TEXTURE_WRAP_T: number;

	readonly TRIANGLES: number;

	readonly TRIANGLE_FAN: number;

	readonly TRIANGLE_STRIP: number;

	readonly UNPACK_ALIGNMENT: number;

	readonly UNPACK_COLORSPACE_CONVERSION_WEBGL: number;

	readonly UNPACK_FLIP_Y_WEBGL: number;

	readonly UNPACK_PREMULTIPLY_ALPHA_WEBGL: number;

	readonly UNSIGNED_BYTE: number;

	readonly UNSIGNED_INT: number;

	readonly UNSIGNED_SHORT: number;

	readonly UNSIGNED_SHORT_4_4_4_4: number;

	readonly UNSIGNED_SHORT_5_5_5_1: number;

	readonly UNSIGNED_SHORT_5_6_5: number;

	readonly VALIDATE_STATUS: number;

	readonly VENDOR: number;

	readonly VERSION: number;

	readonly VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: number;

	readonly VERTEX_ATTRIB_ARRAY_ENABLED: number;

	readonly VERTEX_ATTRIB_ARRAY_NORMALIZED: number;

	readonly VERTEX_ATTRIB_ARRAY_POINTER: number;

	readonly VERTEX_ATTRIB_ARRAY_SIZE: number;

	readonly VERTEX_ATTRIB_ARRAY_STRIDE: number;

	readonly VERTEX_ATTRIB_ARRAY_TYPE: number;

	readonly VERTEX_SHADER: number;

	readonly VIEWPORT: number;

	readonly ZERO: number;

	readonly drawingBufferHeight: number;

	readonly drawingBufferWidth: number;

	constructor();

	activeTexture(texture: number): void;

	attachShader(program: number, shader: number): void;

	bindAttribLocation(program: number, index: number, name: string): void;

	bindBuffer(target: number, buffer: number): void;

	bindFramebuffer(target: number, framebuffer: number): void;

	bindRenderbuffer(target: number, renderbuffer: number): void;

	bindTexture(target: number, texture: number): void;

	blendColor(red: number, green: number, blue: number, alpha: number): void;

	blendEquation(mode: number): void;

	blendEquationSeparate(modeRGB: number, modeAlpha: number): void;

	blendFunc(sfactor: number, dfactor: number): void;

	blendFuncSeparate(srcRGB: number, dstRGB: number, srcAlpha: number, dstAlpha: number): void;

	bufferDataF32(target: number, srcData: NSArray<number> | number[], usage: number): void;

	bufferDataF64(target: number, srcData: NSArray<number> | number[], usage: number): void;

	bufferDataI16(target: number, srcData: NSArray<number> | number[], usage: number): void;

	bufferDataI32(target: number, srcData: NSArray<number> | number[], usage: number): void;

	bufferDataI8(target: number, srcData: NSArray<number> | number[], usage: number): void;

	bufferDataSize(target: number, size: number, usage: number): void;

	bufferDataSrcData(target: number, srcData: NSNull, usage: number): void;

	bufferDataU16(target: number, srcData: NSArray<number> | number[], usage: number): void;

	bufferDataU32(target: number, srcData: NSArray<number> | number[], usage: number): void;

	bufferDataU8(target: number, srcData: NSArray<number> | number[], usage: number): void;

	bufferSubData(target: number, offset: number, srcData: NSNull): void;

	bufferSubDataF32(target: number, offset: number, srcData: NSArray<number> | number[]): void;

	bufferSubDataF64(target: number, offset: number, srcData: NSArray<number> | number[]): void;

	bufferSubDataI16(target: number, offset: number, srcData: NSArray<number> | number[]): void;

	bufferSubDataI32(target: number, offset: number, srcData: NSArray<number> | number[]): void;

	bufferSubDataI8(target: number, offset: number, srcData: NSArray<number> | number[]): void;

	bufferSubDataU16(target: number, offset: number, srcData: NSArray<number> | number[]): void;

	bufferSubDataU32(target: number, offset: number, srcData: NSArray<number> | number[]): void;

	bufferSubDataU8(target: number, offset: number, srcData: NSArray<number> | number[]): void;

	checkFramebufferStatus(target: number): number;

	clear(mask: number): void;

	clearColor(red: number, green: number, blue: number, alpha: number): void;

	clearDepth(depth: number): void;

	clearStencil(stencil: number): void;

	colorMask(red: boolean, green: boolean, blue: boolean, alpha: boolean): void;

	commit(): void;

	compileShader(shader: number): void;

	compressedTexImage2D(target: number, level: number, internalformat: number, width: number, height: number, border: number, pixels: NSData): void;

	compressedTexSubImage2D(target: number, level: number, xoffset: number, yoffset: number, width: number, height: number, format: number, pixels: NSData): void;

	copyTexImage2D(target: number, level: number, internalformat: number, x: number, y: number, width: number, height: number, border: number): void;

	copyTexSubImage2D(target: number, level: number, xoffset: number, yoffset: number, x: number, y: number, width: number, height: number): void;

	createBuffer(): number;

	createFramebuffer(): number;

	createProgram(): number;

	createRenderbuffer(): number;

	createShader(type: number): number;

	createTexture(): number;

	cullFace(mode: number): void;

	deleteBuffer(buffer: number): void;

	deleteFramebuffer(frameBuffer: number): void;

	deleteProgram(program: number): void;

	deleteRenderbuffer(renderbuffer: number): void;

	deleteShader(shader: number): void;

	deleteTexture(texture: number): void;

	depthFunc(fn: number): void;

	depthMask(flag: boolean): void;

	depthRange(zNear: number, zFar: number): void;

	detachShader(program: number, shader: number): void;

	disable(cap: number): void;

	disableVertexAttribArray(index: number): void;

	drawArrays(mode: number, first: number, count: number): void;

	drawElements(mode: number, count: number, type: number, offset: number): void;

	enable(cap: number): void;

	enableVertexAttribArray(index: number): void;

	finish(): void;

	flush(): void;

	framebufferRenderbuffer(target: number, attachment: number, renderbuffertarget: number, renderbuffer: number): void;

	framebufferTexture2D(target: number, attachment: number, textarget: number, texture: number, level: number): void;

	frontFace(mode: number): void;

	generateMipmap(target: number): void;

	getActiveAttrib(program: number, index: number): TNSWebGLActiveInfo;

	getActiveUniform(program: number, index: number): TNSWebGLActiveInfo;

	getAttachedShaders(program: number): NSArray<number>;

	getAttribLocation(program: number, name: string): number;

	getBufferParameter(target: number, pname: number): number;

	getCanvas(): TNSCanvas;

	getContextAttributes(): any;

	getError(): number;

	getExtension(name: string): any;

	getFramebufferAttachmentParameter(target: number, attachment: number, pname: number): TNSFramebufferAttachmentParameter;

	getParameter(pname: number): any;

	getProgramInfoLog(program: number): string;

	getProgramParameter(program: number, pname: number): any;

	getRenderbufferParameter(target: number, pname: number): number;

	getShaderInfoLog(shader: number): string;

	getShaderParameter(shader: number, pname: number): any;

	getShaderPrecisionFormat(shaderType: number, precisionType: number): TNSWebGLShaderPrecisionFormat;

	getShaderSource(shader: number): string;

	getSupportedExtensions(): NSArray<string>;

	getTexParameter(target: number, pname: number): number;

	getUniform(program: number, location: number): any;

	getUniformLocation(program: number, name: string): number;

	getVertexAttrib(index: number, pname: number): any;

	getVertexAttribOffsetWithIndexPname(index: number, pname: number): number;

	hint(target: number, mode: number): void;

	// @ts-ignore
	init(canvas: TNSCanvas): this;

	isBuffer(buffer: number): boolean;

	isContextLost(): boolean;

	isEnabled(cap: number): boolean;

	isFramebuffer(framebuffer: number): boolean;

	isProgram(program: number): boolean;

	isRenderbuffer(renderbuffer: number): boolean;

	isShader(shader: number): boolean;

	isTexture(texture: number): boolean;

	lineWidth(width: number): void;

	linkProgram(program: number): void;

	pixelStorei(pname: number, param: number): void;

	polygonOffset(factor: number, units: number): void;

	readPixels(x: number, y: number, width: number, height: number, format: number, type: number, pixels: NSData): void;

	renderbufferStorage(target: number, internalFormat: number, width: number, height: number): void;

	sampleCoverage(value: number, invert: boolean): void;

	scissor(x: number, y: number, width: number, height: number): void;

	shaderSource(shader: number, source: string): void;

	stencilFunc(fn: number, ref: number, mask: number): void;

	stencilFuncSeparate(face: number, fn: number, ref: number, mask: number): void;

	stencilMask(mask: number): void;

	stencilMaskSeparate(face: number, mask: number): void;

	stencilOp(fail: number, zfail: number, zpass: number): void;

	stencilOpSeparate(face: number, fail: number, zfail: number, zpass: number): void;

	texImage2D(target: number, level: number, internalformat: number, width: number, height: number, border: number, format: number, type: number, pixels: NSNull): void;

	texImage2DAsset(target: number, level: number, internalformat: number, format: number, type: number, asset: TNSImageAsset): void;

	texImage2DCanvas(target: number, level: number, internalformat: number, format: number, type: number, canvas: TNSCanvas): void;

	texImage2DF32(target: number, level: number, internalformat: number, width: number, height: number, border: number, format: number, type: number, pixels: NSArray<number> | number[]): void;

	texImage2DPixels(target: number, level: number, internalformat: number, format: number, type: number, pixels: UIImage): void;

	texImage2DU16(target: number, level: number, internalformat: number, width: number, height: number, border: number, format: number, type: number, pixels: NSArray<number> | number[]): void;

	texImage2DU32(target: number, level: number, internalformat: number, width: number, height: number, border: number, format: number, type: number, pixels: NSArray<number> | number[]): void;

	texImage2DU8(target: number, level: number, internalformat: number, width: number, height: number, border: number, format: number, type: number, pixels: NSArray<number> | number[]): void;

	texParameterf(target: number, pname: number, param: number): void;

	texParameteri(target: number, pname: number, param: number): void;

	texSubImage2D(target: number, level: number, xoffset: number, yoffset: number, format: number, type: number, pixels: NSNull): void;

	texSubImage2DAsset(target: number, level: number, xoffset: number, yoffset: number, format: number, type: number, asset: TNSImageAsset): void;

	texSubImage2DCanvas(target: number, level: number, xoffset: number, yoffset: number, format: number, type: number, canvas: TNSCanvas): void;

	texSubImage2DF32(target: number, level: number, xoffset: number, yoffset: number, width: number, height: number, format: number, type: number, pixels: NSArray<number> | number[]): void;

	texSubImage2DPixels(target: number, level: number, xoffset: number, yoffset: number, format: number, type: number, pixels: UIImage): void;

	texSubImage2DU16(target: number, level: number, xoffset: number, yoffset: number, width: number, height: number, format: number, type: number, pixels: NSArray<number> | number[]): void;

	texSubImage2DU8(target: number, level: number, xoffset: number, yoffset: number, width: number, height: number, format: number, type: number, pixels: NSArray<number> | number[]): void;

	uniform1f(location: number, v0: number): void;

	uniform1fv(location: number, value: NSArray<number> | number[]): void;

	uniform1i(location: number, v0: number): void;

	uniform1iv(location: number, value: NSArray<number> | number[]): void;

	uniform2f(location: number, v0: number, v1: number): void;

	uniform2fv(location: number, value: NSArray<number> | number[]): void;

	uniform2i(location: number, v0: number, v1: number): void;

	uniform2iv(location: number, value: NSArray<number> | number[]): void;

	uniform3f(location: number, v0: number, v1: number, v2: number): void;

	uniform3fv(location: number, value: NSArray<number> | number[]): void;

	uniform3i(location: number, v0: number, v1: number, v2: number): void;

	uniform3iv(location: number, value: NSArray<number> | number[]): void;

	uniform4f(location: number, v0: number, v1: number, v2: number, v3: number): void;

	uniform4fv(location: number, value: NSArray<number> | number[]): void;

	uniform4i(location: number, v0: number, v1: number, v2: number, v3: number): void;

	uniform4iv(location: number, value: NSArray<number> | number[]): void;

	uniformMatrix2fv(location: number, transpose: boolean, value: NSArray<number> | number[]): void;

	uniformMatrix3fv(location: number, transpose: boolean, value: NSArray<number> | number[]): void;

	uniformMatrix4fv(location: number, transpose: boolean, value: NSArray<number> | number[]): void;

	useProgram(program: number): void;

	validateProgram(program: number): void;

	vertexAttrib1f(index: number, v0: number): void;

	vertexAttrib1fv(index: number, value: NSArray<number> | number[]): void;

	vertexAttrib2f(index: number, v0: number, v1: number): void;

	vertexAttrib2fv(index: number, value: NSArray<number> | number[]): void;

	vertexAttrib3f(index: number, v0: number, v1: number, v2: number): void;

	vertexAttrib3fv(index: number, value: NSArray<number> | number[]): void;

	vertexAttrib4f(index: number, v0: number, v1: number, v2: number, v3: number): void;

	vertexAttrib4fv(index: number, value: NSArray<number> | number[]): void;

	vertexAttribPointer(index: number, size: number, type: number, normalized: boolean, stride: number, offset: number): void;

	viewport(x: number, y: number, width: number, height: number): void;
}

declare class TNSWebGLShaderPrecisionFormat extends NSObject {

	static alloc(): TNSWebGLShaderPrecisionFormat; // inherited from NSObject

	static new(): TNSWebGLShaderPrecisionFormat; // inherited from NSObject

	readonly precision: number;

	readonly rangeMax: number;

	readonly rangeMin: number;

	constructor(o: { rangeMin: number; rangeMax: number; precision: number; });

	initWithRangeMinRangeMaxPrecision(rangeMin: number, rangeMax: number, precision: number): this;
}

declare class TNS_ANGLE_instanced_arrays extends NSObject {

	static alloc(): TNS_ANGLE_instanced_arrays; // inherited from NSObject

	static new(): TNS_ANGLE_instanced_arrays; // inherited from NSObject

	readonly VERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE: number;

	constructor(o: { context: TNSWebGLRenderingContext; });

	drawArraysInstancedANGLEWithModeFirstCountPrimcount(mode: number, first: number, count: number, primcount: number): void;

	drawElementsInstancedANGLEWithModeCountTypeOffsetPrimcount(mode: number, count: number, type: number, offset: number, primcount: number): void;

	initWithContext(context: TNSWebGLRenderingContext): this;

	vertexAttribDivisorANGLEWithIndexDivisor(index: number, divisor: number): void;
}

declare class TNS_EXT_blend_minmax extends NSObject {

	static alloc(): TNS_EXT_blend_minmax; // inherited from NSObject

	static new(): TNS_EXT_blend_minmax; // inherited from NSObject

	readonly MAX_EXT: number;

	readonly MIN_EXT: number;
}

declare class TNS_EXT_color_buffer_float extends NSObject {

	static alloc(): TNS_EXT_color_buffer_float; // inherited from NSObject

	static new(): TNS_EXT_color_buffer_float; // inherited from NSObject

	readonly R11F_G11F_B10F: number;

	readonly R16F: number;

	readonly R32F: number;

	readonly RG16F: number;

	readonly RG32F: number;

	readonly RGB16F: number;

	readonly RGBA32F: number;
}

declare class TNS_EXT_color_buffer_half_float extends NSObject {

	static alloc(): TNS_EXT_color_buffer_half_float; // inherited from NSObject

	static new(): TNS_EXT_color_buffer_half_float; // inherited from NSObject

	readonly FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT: number;

	readonly RGB16F_EXT: number;

	readonly RGBA16F_EXT: number;

	readonly UNSIGNED_NORMALIZED_EXT: number;
}

declare class TNS_EXT_sRGB extends NSObject {

	static alloc(): TNS_EXT_sRGB; // inherited from NSObject

	static new(): TNS_EXT_sRGB; // inherited from NSObject

	readonly FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT: number;

	readonly SRGB8_ALPHA8_EXT: number;

	readonly SRGB_ALPHA_EXT: number;

	readonly SRGB_EXT: number;
}

declare class TNS_EXT_shader_texture_lod extends NSObject {

	static alloc(): TNS_EXT_shader_texture_lod; // inherited from NSObject

	static new(): TNS_EXT_shader_texture_lod; // inherited from NSObject
}

declare class TNS_EXT_texture_filter_anisotropic extends NSObject {

	static alloc(): TNS_EXT_texture_filter_anisotropic; // inherited from NSObject

	static new(): TNS_EXT_texture_filter_anisotropic; // inherited from NSObject

	readonly MAX_TEXTURE_MAX_ANISOTROPY_EXT: number;

	readonly TEXTURE_MAX_ANISOTROPY_EXT: number;
}

declare class TNS_OES_element_index_uint extends NSObject {

	static alloc(): TNS_OES_element_index_uint; // inherited from NSObject

	static new(): TNS_OES_element_index_uint; // inherited from NSObject

	readonly UNSIGNED_INT: number;
}

declare class TNS_OES_fbo_render_mipmap extends NSObject {

	static alloc(): TNS_OES_fbo_render_mipmap; // inherited from NSObject

	static new(): TNS_OES_fbo_render_mipmap; // inherited from NSObject
}

declare class TNS_OES_standard_derivatives extends NSObject {

	static alloc(): TNS_OES_standard_derivatives; // inherited from NSObject

	static new(): TNS_OES_standard_derivatives; // inherited from NSObject
}

declare class TNS_OES_texture_float extends NSObject {

	static alloc(): TNS_OES_texture_float; // inherited from NSObject

	static new(): TNS_OES_texture_float; // inherited from NSObject
}

declare class TNS_OES_texture_float_linear extends NSObject {

	static alloc(): TNS_OES_texture_float_linear; // inherited from NSObject

	static new(): TNS_OES_texture_float_linear; // inherited from NSObject
}

declare class TNS_OES_texture_half_float extends NSObject {

	static alloc(): TNS_OES_texture_half_float; // inherited from NSObject

	static new(): TNS_OES_texture_half_float; // inherited from NSObject

	readonly HALF_FLOAT_OES: number;
}

declare class TNS_OES_texture_half_float_linear extends NSObject {

	static alloc(): TNS_OES_texture_half_float_linear; // inherited from NSObject

	static new(): TNS_OES_texture_half_float_linear; // inherited from NSObject
}

declare class TNS_OES_vertex_array_object extends NSObject {

	static alloc(): TNS_OES_vertex_array_object; // inherited from NSObject

	static new(): TNS_OES_vertex_array_object; // inherited from NSObject

	readonly VERTEX_ARRAY_BINDING_OES: number;

	bindVertexArrayOESWithArrayObject(arrayObject: number): void;

	createVertexArrayOES(): number;

	deleteVertexArrayOESWithArrayObject(arrayObject: number): void;

	isVertexArrayOESWithArrayObject(arrayObject: number): boolean;
}

declare class TNS_WEBGL_color_buffer_float extends NSObject {

	static alloc(): TNS_WEBGL_color_buffer_float; // inherited from NSObject

	static new(): TNS_WEBGL_color_buffer_float; // inherited from NSObject

	readonly FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT: number;

	readonly RGB32F_EXT: number;

	readonly RGBA32F_EXT: number;

	readonly UNSIGNED_NORMALIZED_EXT: number;
}

declare class TNS_WEBGL_compressed_texture_etc extends NSObject {

	static alloc(): TNS_WEBGL_compressed_texture_etc; // inherited from NSObject

	static new(): TNS_WEBGL_compressed_texture_etc; // inherited from NSObject

	readonly COMPRESSED_R11_EAC: number;

	readonly COMPRESSED_RG11_EAC: number;

	readonly COMPRESSED_RGB8_ETC2: number;

	readonly COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: number;

	readonly COMPRESSED_RGBA8_ETC2_EAC: number;

	readonly COMPRESSED_SIGNED_R11_EAC: number;

	readonly COMPRESSED_SIGNED_RG11_EAC: number;

	readonly COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: number;

	readonly COMPRESSED_SRGB8_ETC2: number;

	readonly COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: number;
}

declare class TNS_WEBGL_compressed_texture_etc1 extends NSObject {

	static alloc(): TNS_WEBGL_compressed_texture_etc1; // inherited from NSObject

	static new(): TNS_WEBGL_compressed_texture_etc1; // inherited from NSObject

	readonly COMPRESSED_RGB_ETC1_WEBGL: number;
}

declare class TNS_WEBGL_compressed_texture_pvrtc extends NSObject {

	static alloc(): TNS_WEBGL_compressed_texture_pvrtc; // inherited from NSObject

	static new(): TNS_WEBGL_compressed_texture_pvrtc; // inherited from NSObject

	readonly COMPRESSED_RGBA_PVRTC_2BPPV1_IMG: number;

	readonly COMPRESSED_RGBA_PVRTC_4BPPV1_IMG: number;

	readonly COMPRESSED_RGB_PVRTC_2BPPV1_IMG: number;

	readonly COMPRESSED_RGB_PVRTC_4BPPV1_IMG: number;
}

declare class TNS_WEBGL_depth_texture extends NSObject {

	static alloc(): TNS_WEBGL_depth_texture; // inherited from NSObject

	static new(): TNS_WEBGL_depth_texture; // inherited from NSObject

	readonly UNSIGNED_INT_24_8_WEBGL: number;
}

declare class TNS_WEBGL_draw_buffers extends NSObject {

	static alloc(): TNS_WEBGL_draw_buffers; // inherited from NSObject

	static new(): TNS_WEBGL_draw_buffers; // inherited from NSObject

	readonly COLOR_ATTACHMENT0_WEBGL: number;

	readonly COLOR_ATTACHMENT10_WEBGL: number;

	readonly COLOR_ATTACHMENT11_WEBGL: number;

	readonly COLOR_ATTACHMENT12_WEBGL: number;

	readonly COLOR_ATTACHMENT13_WEBGL: number;

	readonly COLOR_ATTACHMENT14_WEBGL: number;

	readonly COLOR_ATTACHMENT15_WEBGL: number;

	readonly COLOR_ATTACHMENT1_WEBGL: number;

	readonly COLOR_ATTACHMENT2_WEBGL: number;

	readonly COLOR_ATTACHMENT3_WEBGL: number;

	readonly COLOR_ATTACHMENT4_WEBGL: number;

	readonly COLOR_ATTACHMENT5_WEBGL: number;

	readonly COLOR_ATTACHMENT6_WEBGL: number;

	readonly COLOR_ATTACHMENT7_WEBGL: number;

	readonly COLOR_ATTACHMENT8_WEBGL: number;

	readonly COLOR_ATTACHMENT9_WEBGL: number;

	readonly DRAW_BUFFER0_WEBGL: number;

	readonly DRAW_BUFFER10_WEBGL: number;

	readonly DRAW_BUFFER11_WEBGL: number;

	readonly DRAW_BUFFER12_WEBGL: number;

	readonly DRAW_BUFFER13_WEBGL: number;

	readonly DRAW_BUFFER14_WEBGL: number;

	readonly DRAW_BUFFER15_WEBGL: number;

	readonly DRAW_BUFFER1_WEBGL: number;

	readonly DRAW_BUFFER2_WEBGL: number;

	readonly DRAW_BUFFER3_WEBGL: number;

	readonly DRAW_BUFFER4_WEBGL: number;

	readonly DRAW_BUFFER5_WEBGL: number;

	readonly DRAW_BUFFER6_WEBGL: number;

	readonly DRAW_BUFFER7_WEBGL: number;

	readonly DRAW_BUFFER8_WEBGL: number;

	readonly DRAW_BUFFER9_WEBGL: number;

	readonly MAX_COLOR_ATTACHMENTS_WEBGL: number;

	readonly MAX_DRAW_BUFFERS_WEBGL: number;

	drawBuffersWEBGLWithBuffers(buffers: NSArray<number> | number[]): void;
}

declare class TNS_WEBGL_lose_context extends NSObject {

	static alloc(): TNS_WEBGL_lose_context; // inherited from NSObject

	static new(): TNS_WEBGL_lose_context; // inherited from NSObject

	constructor(o: { canvas: TNSCanvas; });

	initWithCanvas(canvas: TNSCanvas): this;

	loseContext(): void;

	restoreContext(): void;
}

declare function native_arc(canvas_native_ptr: number, x: number, y: number, radius: number, start_angle: number, end_angle: number, anticlockwise: boolean): number;

declare function native_arc_to(canvas_native_ptr: number, x1: number, y1: number, x2: number, y2: number, radius: number): number;

declare function native_begin_path(canvas_native_ptr: number): number;

declare function native_bezier_curve_to(canvas_native_ptr: number, cp1x: number, cp1y: number, cp2x: number, cp2y: number, x: number, y: number): number;

declare function native_clear_canvas(canvas_native_ptr: number, view: interop.Pointer | interop.Reference<any>): number;

declare function native_clear_rect(canvas_native_ptr: number, x: number, y: number, width: number, height: number, view: interop.Pointer | interop.Reference<any>): number;

declare function native_clip(canvas_native_ptr: number, view: interop.Pointer | interop.Reference<any>): number;

declare function native_clip_path_rule(canvas_native_ptr: number, path: number, fill_rule: string | interop.Pointer | interop.Reference<any>, view: interop.Pointer | interop.Reference<any>): number;

declare function native_clip_rule(canvas_native_ptr: number, fill_rule: string | interop.Pointer | interop.Reference<any>, view: interop.Pointer | interop.Reference<any>): number;

declare function native_close_path(canvas_native_ptr: number): number;

declare function native_create_image_asset(): number;

declare function native_create_image_data(width: number, height: number): interop.Pointer | interop.Reference<CanvasArray>;

declare function native_create_matrix(): number;

declare function native_create_path_2d(): number;

declare function native_create_path_2d_from_path_data(data: string | interop.Pointer | interop.Reference<any>): number;

declare function native_create_path_from_path(path: number): number;

declare function native_create_pattern(image_array: string | interop.Pointer | interop.Reference<any>, image_size: number, original_width: number, original_height: number, repetition: string | interop.Pointer | interop.Reference<any>): number;

declare function native_create_pattern_encoded(image_array: string | interop.Pointer | interop.Reference<any>, image_size: number, repetition: string | interop.Pointer | interop.Reference<any>): number;

declare function native_create_text_decoder(decoding: string | interop.Pointer | interop.Reference<any>): number;

declare function native_create_text_encoder(encoding: string | interop.Pointer | interop.Reference<any>): number;

declare function native_destroy(canvas_ptr: number): void;

declare function native_draw_image(canvas_native_ptr: number, image_array: string | interop.Pointer | interop.Reference<any>, image_size: number, original_width: number, original_height: number, dx: number, dy: number, view: interop.Pointer | interop.Reference<any>): number;

declare function native_draw_image_dw(canvas_native_ptr: number, image_array: string | interop.Pointer | interop.Reference<any>, image_size: number, original_width: number, original_height: number, dx: number, dy: number, d_width: number, d_height: number, view: interop.Pointer | interop.Reference<any>): number;

declare function native_draw_image_dw_raw(canvas_native_ptr: number, image_array: string | interop.Pointer | interop.Reference<any>, image_size: number, original_width: number, original_height: number, dx: number, dy: number, d_width: number, d_height: number, view: interop.Pointer | interop.Reference<any>): number;

declare function native_draw_image_raw(canvas_native_ptr: number, image_array: string | interop.Pointer | interop.Reference<any>, image_size: number, original_width: number, original_height: number, dx: number, dy: number, view: interop.Pointer | interop.Reference<any>): number;

declare function native_draw_image_sw(canvas_native_ptr: number, image_array: string | interop.Pointer | interop.Reference<any>, image_size: number, original_width: number, original_height: number, sx: number, sy: number, s_width: number, s_height: number, dx: number, dy: number, d_width: number, d_height: number, view: interop.Pointer | interop.Reference<any>): number;

declare function native_draw_image_sw_raw(canvas_native_ptr: number, image_array: string | interop.Pointer | interop.Reference<any>, image_size: number, original_width: number, original_height: number, sx: number, sy: number, s_width: number, s_height: number, dx: number, dy: number, d_width: number, d_height: number, view: interop.Pointer | interop.Reference<any>): number;

declare function native_drop_image_data(data: interop.Pointer | interop.Reference<CanvasArray>): void;

declare function native_drop_text_metrics(data: interop.Pointer | interop.Reference<CanvasTextMetrics>): void;

declare function native_ellipse(canvas_native_ptr: number, x: number, y: number, radius_x: number, radius_y: number, rotation: number, start_angle: number, end_angle: number, anticlockwise: boolean): number;

declare function native_fill(canvas_native_ptr: number, view: interop.Pointer | interop.Reference<any>): number;

declare function native_fill_path_rule(canvas_native_ptr: number, path_ptr: number, rule: string | interop.Pointer | interop.Reference<any>, view: interop.Pointer | interop.Reference<any>): number;

declare function native_fill_rect(canvas_native_ptr: number, x: number, y: number, width: number, height: number, view: interop.Pointer | interop.Reference<any>): number;

declare function native_fill_rule(canvas_native_ptr: number, rule: string | interop.Pointer | interop.Reference<any>, view: interop.Pointer | interop.Reference<any>): number;

declare function native_fill_text(canvas_native_ptr: number, text: string | interop.Pointer | interop.Reference<any>, x: number, y: number, width: number, view: interop.Pointer | interop.Reference<any>): number;

declare function native_flip_y_in_place(data: string | interop.Pointer | interop.Reference<any>, length: number, bytes_per_row: number, height: number): void;

declare function native_flip_y_in_place_3d(data: string | interop.Pointer | interop.Reference<any>, length: number, bytes_per_row: number, height: number, depth: number): void;

declare function native_flush(canvas_ptr: number): number;

declare function native_free_byte_array(array: interop.Pointer | interop.Reference<NativeByteArray>): void;

declare function native_free_char(text: string | interop.Pointer | interop.Reference<any>): void;

declare function native_free_matrix_data(data: interop.Pointer | interop.Reference<CanvasArray>): void;

declare function native_free_path_2d(path: number): void;

declare function native_free_pattern(pattern: number): void;

declare function native_get_current_transform(canvas_native_ptr: number): number;

declare function native_get_direction(canvas_native_ptr: number): string;

declare function native_get_image_data(canvas_native_ptr: number, sx: number, sy: number, sw: number, sh: number): interop.Pointer | interop.Reference<CanvasArray>;

declare function native_get_ios_device(canvas_native_ptr: number): interop.Pointer | interop.Reference<CanvasDevice>;

declare function native_get_matrix(matrix: number): interop.Pointer | interop.Reference<CanvasArray>;

declare function native_gl_tex_image_2D_asset(target: number, level: number, internalformat: number, width: number, height: number, border: number, format: number, image_type: number, asset: number, flip_y: number): void;

declare function native_gl_tex_sub_image_2D_asset(target: number, level: number, xoffset: number, yoffset: number, width: number, height: number, format: number, image_type: number, asset: number, flip_y: number): void;

declare function native_image_asset_flip_x(asset: number): number;

declare function native_image_asset_flip_x_in_place_owned(width: number, height: number, buf: string | interop.Pointer | interop.Reference<any>, length: number): void;

declare function native_image_asset_flip_y(asset: number): number;

declare function native_image_asset_flip_y_in_place_owned(width: number, height: number, buf: string | interop.Pointer | interop.Reference<any>, length: number): void;

declare function native_image_asset_free_bytes(data: NativeByteArray): void;

declare function native_image_asset_get_bytes(asset: number): NativeByteArray;

declare function native_image_asset_get_error(asset: number): string;

declare function native_image_asset_get_height(asset: number): number;

declare function native_image_asset_get_width(asset: number): number;

declare function native_image_asset_has_error(asset: number): number;

declare function native_image_asset_load_from_path(asset: number, path: string | interop.Pointer | interop.Reference<any>): number;

declare function native_image_asset_load_from_raw(asset: number, array: string | interop.Pointer | interop.Reference<any>, size: number): number;

declare function native_image_asset_release(asset: number): void;

declare function native_image_asset_scale(asset: number, x: number, y: number): number;

declare function native_image_smoothing_enabled(canvas_native_ptr: number, enabled: boolean): number;

declare function native_image_smoothing_quality(canvas_native_ptr: number, quality: string | interop.Pointer | interop.Reference<any>): number;

declare function native_init(device: interop.Pointer | interop.Reference<any>, queue: interop.Pointer | interop.Reference<any>, view: interop.Pointer | interop.Reference<any>, scale: number, direction: string | interop.Pointer | interop.Reference<any>): number;

declare function native_init_legacy(width: number, height: number, buffer_id: number, scale: number, stencil: number, samples: number, alpha: number, direction: string | interop.Pointer | interop.Reference<any>): number;

declare function native_is_point_in_path(canvas_ptr: number, x: number, y: number): number;

declare function native_is_point_in_path_with_path_rule(canvas_ptr: number, path: number, x: number, y: number, fill_rule: string | interop.Pointer | interop.Reference<any>): number;

declare function native_is_point_in_path_with_rule(canvas_ptr: number, x: number, y: number, fill_rule: string | interop.Pointer | interop.Reference<any>): number;

declare function native_is_point_in_stroke(canvas_ptr: number, x: number, y: number): number;

declare function native_is_point_in_stroke_with_path(canvas_ptr: number, path: number, x: number, y: number): number;

declare function native_line_dash_offset(canvas_native_ptr: number, offset: number): number;

declare function native_line_join(canvas_native_ptr: number, line_join: string | interop.Pointer | interop.Reference<any>): number;

declare function native_line_to(canvas_native_ptr: number, x: number, y: number): number;

declare function native_measure_text(canvas_native_ptr: number, text: string | interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<CanvasTextMetrics>;

declare function native_miter_limit(canvas_native_ptr: number, limit: number): number;

declare function native_move_to(canvas_native_ptr: number, x: number, y: number): number;

declare function native_native_image_asset_save_path(asset: number, path: string | interop.Pointer | interop.Reference<any>, format: number): number;

declare function native_path_2d_add_path(path: number, path_to_add: number, matrix: number): number;

declare function native_path_2d_arc(path: number, x: number, y: number, radius: number, start_angle: number, end_angle: number, anticlockwise: boolean): number;

declare function native_path_2d_arc_to(path: number, x1: number, y1: number, x2: number, y2: number, radius: number): number;

declare function native_path_2d_bezier_curve_to(path: number, cp1x: number, cp1y: number, cp2x: number, cp2y: number, x: number, y: number): number;

declare function native_path_2d_close_path(path: number): number;

declare function native_path_2d_ellipse(path: number, x: number, y: number, radius_x: number, radius_y: number, rotation: number, start_angle: number, end_angle: number, anticlockwise: boolean): number;

declare function native_path_2d_line_to(path: number, x: number, y: number): number;

declare function native_path_2d_move_to(path: number, x: number, y: number): number;

declare function native_path_2d_quadratic_curve_to(path: number, cpx: number, cpy: number, x: number, y: number): number;

declare function native_path_2d_rect(path: number, x: number, y: number, width: number, height: number): number;

declare function native_put_image_data(canvas_native_ptr: number, width: number, height: number, array: string | interop.Pointer | interop.Reference<any>, array_size: number, x: number, y: number, dirty_x: number, dirty_y: number, dirty_width: number, dirty_height: number): number;

declare function native_quadratic_curve_to(canvas_native_ptr: number, cpx: number, cpy: number, x: number, y: number): number;

declare function native_rect(canvas_native_ptr: number, x: number, y: number, width: number, height: number): number;

declare function native_reset_transform(canvas_native_ptr: number): number;

declare function native_restore(canvas_native_ptr: number): number;

declare function native_rotate(canvas_native_ptr: number, angle: number, view: interop.Pointer | interop.Reference<any>): number;

declare function native_save(canvas_native_ptr: number): number;

declare function native_scale(canvas_native_ptr: number, x: number, y: number, view: interop.Pointer | interop.Reference<any>): number;

declare function native_set_current_transform(canvas_native_ptr: number, matrix: number): number;

declare function native_set_direction(canvas_native_ptr: number, direction: string | interop.Pointer | interop.Reference<any>): number;

declare function native_set_fill_color(canvas_native_ptr: number, color: number): number;

declare function native_set_fill_color_rgba(canvas_native_ptr: number, red: number, green: number, blue: number, alpha: number): number;

declare function native_set_fill_gradient_linear(canvas_native_ptr: number, x0: number, y0: number, x1: number, y1: number, colors_size: number, colors_array: interop.Pointer | interop.Reference<number>, positions_size: number, positions_array: interop.Pointer | interop.Reference<number>): number;

declare function native_set_fill_gradient_radial(canvas_native_ptr: number, x0: number, y0: number, radius_0: number, x1: number, y1: number, radius_1: number, colors_size: number, colors_array: interop.Pointer | interop.Reference<number>, positions_size: number, positions_array: interop.Pointer | interop.Reference<number>): number;

declare function native_set_fill_pattern(canvas_native_ptr: number, pattern: number): number;

declare function native_set_font(canvas_native_ptr: number, font: string | interop.Pointer | interop.Reference<any>): number;

declare function native_set_global_alpha(canvas_native_ptr: number, alpha: number): number;

declare function native_set_global_composite_operation(canvas_native_ptr: number, composite: string | interop.Pointer | interop.Reference<any>): number;

declare function native_set_line_cap(canvas_native_ptr: number, line_cap: string | interop.Pointer | interop.Reference<any>): number;

declare function native_set_line_dash(canvas_native_ptr: number, size: number, array: interop.Pointer | interop.Reference<number>): number;

declare function native_set_line_width(canvas_native_ptr: number, line_width: number): number;

declare function native_set_matrix(matrix: number, array: interop.Pointer | interop.Reference<any>, length: number): number;

declare function native_set_pattern_transform(pattern: number, matrix: number): number;

declare function native_set_stroke_color(canvas_native_ptr: number, color: number): number;

declare function native_set_stroke_color_rgba(canvas_native_ptr: number, red: number, green: number, blue: number, alpha: number): number;

declare function native_set_stroke_gradient_linear(canvas_native_ptr: number, x0: number, y0: number, x1: number, y1: number, colors_size: number, colors_array: interop.Pointer | interop.Reference<number>, positions_size: number, positions_array: interop.Pointer | interop.Reference<number>): number;

declare function native_set_stroke_gradient_radial(canvas_native_ptr: number, x0: number, y0: number, radius_0: number, x1: number, y1: number, radius_1: number, colors_size: number, colors_array: interop.Pointer | interop.Reference<number>, positions_size: number, positions_array: interop.Pointer | interop.Reference<number>): number;

declare function native_set_stroke_pattern(canvas_native_ptr: number, pattern: number): number;

declare function native_set_transform(canvas_native_ptr: number, a: number, b: number, c: number, d: number, e: number, f: number, view: interop.Pointer | interop.Reference<any>): number;

declare function native_shadow_blur(canvas_native_ptr: number, limit: number): number;

declare function native_shadow_color(canvas_native_ptr: number, color: number): number;

declare function native_shadow_offset_x(canvas_native_ptr: number, x: number): number;

declare function native_shadow_offset_y(canvas_native_ptr: number, y: number): number;

declare function native_snapshot_canvas(canvas_native_ptr: number): interop.Pointer | interop.Reference<NativeByteArray>;

declare function native_stroke(canvas_native_ptr: number, view: interop.Pointer | interop.Reference<any>): number;

declare function native_stroke_path(canvas_native_ptr: number, path: number, view: interop.Pointer | interop.Reference<any>): number;

declare function native_stroke_rect(canvas_native_ptr: number, x: number, y: number, width: number, height: number, view: interop.Pointer | interop.Reference<any>): number;

declare function native_stroke_text(canvas_native_ptr: number, text: string | interop.Pointer | interop.Reference<any>, x: number, y: number, width: number, view: interop.Pointer | interop.Reference<any>): number;

declare function native_surface_resized(_width: number, _height: number, _device: interop.Pointer | interop.Reference<any>, _queue: interop.Pointer | interop.Reference<any>, _scale: number, current_canvas: number): number;

declare function native_surface_resized_legacy(width: number, height: number, buffer_id: number, _scale: number, stencil: number, samples: number, alpha: number, canvas_native_ptr: number): number;

declare function native_tex_image_3D_asset(target: number, level: number, internalformat: number, width: number, height: number, depth: number, border: number, format: number, image_type: number, asset: number, flip_y: number): void;

declare function native_tex_sub_image_3D_asset(target: number, level: number, xoffset: number, yoffset: number, zoffset: number, width: number, height: number, depth: number, format: number, image_type: number, asset: number, flip_y: number): void;

declare function native_text_align(canvas_native_ptr: number, alignment: string | interop.Pointer | interop.Reference<any>): number;

declare function native_text_decoder_decode(decoder: number, data: string | interop.Pointer | interop.Reference<any>, len: number): string;

declare function native_text_decoder_decode_i16(decoder: number, data: interop.Pointer | interop.Reference<number>, len: number): string;

declare function native_text_decoder_decode_i32(decoder: number, data: interop.Pointer | interop.Reference<number>, len: number): string;

declare function native_text_decoder_decode_u16(decoder: number, data: interop.Pointer | interop.Reference<number>, len: number): string;

declare function native_text_decoder_free(decoder: number): void;

declare function native_text_decoder_get_encoding(decoder: number): string;

declare function native_text_encoder_encode(encoder: number, text: string | interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<NativeByteArray>;

declare function native_text_encoder_free(encoder: number): void;

declare function native_text_encoder_get_encoding(encoder: number): string;

declare function native_to_data_url(canvas_ptr: number, format: string | interop.Pointer | interop.Reference<any>, quality: number): string;

declare function native_transform(canvas_native_ptr: number, a: number, b: number, c: number, d: number, e: number, f: number, _view: interop.Pointer | interop.Reference<any>): number;

declare function native_translate(canvas_native_ptr: number, x: number, y: number, view: interop.Pointer | interop.Reference<any>): number;

declare function offsetF32By(data: interop.Pointer | interop.Reference<number>, offset: number): void;

declare function offsetF64By(data: interop.Pointer | interop.Reference<number>, offset: number): void;

declare function offsetI16By(data: interop.Pointer | interop.Reference<number>, offset: number): void;

declare function offsetI32By(data: interop.Pointer | interop.Reference<number>, offset: number): void;

declare function offsetI8By(data: interop.Pointer | interop.Reference<number>, offset: number): void;

declare function offsetU16By(data: interop.Pointer | interop.Reference<number>, offset: number): void;

declare function offsetU32By(data: interop.Pointer | interop.Reference<number>, offset: number): void;

declare function offsetU8By(data: string | interop.Pointer | interop.Reference<any>, offset: number): void;
