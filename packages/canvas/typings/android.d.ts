/// <reference path="android-declarations.d.ts"/>

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class AnimationFrame {
					public static class: java.lang.Class<com.github.triniwiz.canvas.AnimationFrame>;
					public doFrame(param0: number): void;
					public constructor();
					public static requestAnimationFrame(param0: com.github.triniwiz.canvas.AnimationFrame.Callback): number;
					public static cancelAnimationFrame(param0: number): void;
				}
				export module AnimationFrame {
					export class Callback {
						public static class: java.lang.Class<com.github.triniwiz.canvas.AnimationFrame.Callback>;
						/**
						 * Constructs a new instance of the com.github.triniwiz.canvas.AnimationFrame$Callback interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							onFrame(param0: number): void;
						});
						public constructor();
						public onFrame(param0: number): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class BuildConfig {
					public static class: java.lang.Class<com.github.triniwiz.canvas.BuildConfig>;
					public static DEBUG: boolean;
					public static LIBRARY_PACKAGE_NAME: string;
					public static BUILD_TYPE: string;
					public static VERSION_CODE: number;
					public static VERSION_NAME: string;
					public constructor();
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class CPUView {
					public static class: java.lang.Class<com.github.triniwiz.canvas.CPUView>;
					public constructor(param0: globalAndroid.content.Context);
					public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet);
					public onDraw(param0: globalAndroid.graphics.Canvas): void;
					public flush(): void;
					public onSizeChanged(param0: number, param1: number, param2: number, param3: number): void;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class CanvasColorStyleType {
					public static class: java.lang.Class<com.github.triniwiz.canvas.CanvasColorStyleType>;
					public static Color: com.github.triniwiz.canvas.CanvasColorStyleType;
					public static Gradient: com.github.triniwiz.canvas.CanvasColorStyleType;
					public static Pattern: com.github.triniwiz.canvas.CanvasColorStyleType;
					public static values(): native.Array<com.github.triniwiz.canvas.CanvasColorStyleType>;
					public static valueOf(param0: string): com.github.triniwiz.canvas.CanvasColorStyleType;
					public toString(): string;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class CanvasCompositeOperationType {
					public static class: java.lang.Class<com.github.triniwiz.canvas.CanvasCompositeOperationType>;
					public static SourceOver: com.github.triniwiz.canvas.CanvasCompositeOperationType;
					public static SourceIn: com.github.triniwiz.canvas.CanvasCompositeOperationType;
					public static SourceoOut: com.github.triniwiz.canvas.CanvasCompositeOperationType;
					public static SourceAtop: com.github.triniwiz.canvas.CanvasCompositeOperationType;
					public static DestinationOver: com.github.triniwiz.canvas.CanvasCompositeOperationType;
					public static DestinationIn: com.github.triniwiz.canvas.CanvasCompositeOperationType;
					public static DestinationOut: com.github.triniwiz.canvas.CanvasCompositeOperationType;
					public static DestinationAtop: com.github.triniwiz.canvas.CanvasCompositeOperationType;
					public static Lighter: com.github.triniwiz.canvas.CanvasCompositeOperationType;
					public static Copy: com.github.triniwiz.canvas.CanvasCompositeOperationType;
					public static Xor: com.github.triniwiz.canvas.CanvasCompositeOperationType;
					public static Multiply: com.github.triniwiz.canvas.CanvasCompositeOperationType;
					public static Screen: com.github.triniwiz.canvas.CanvasCompositeOperationType;
					public static Overlay: com.github.triniwiz.canvas.CanvasCompositeOperationType;
					public static Darken: com.github.triniwiz.canvas.CanvasCompositeOperationType;
					public static Lighten: com.github.triniwiz.canvas.CanvasCompositeOperationType;
					public static ColorDodge: com.github.triniwiz.canvas.CanvasCompositeOperationType;
					public static ColorBurn: com.github.triniwiz.canvas.CanvasCompositeOperationType;
					public static HardLight: com.github.triniwiz.canvas.CanvasCompositeOperationType;
					public static SoftLight: com.github.triniwiz.canvas.CanvasCompositeOperationType;
					public static Difference: com.github.triniwiz.canvas.CanvasCompositeOperationType;
					public static Exclusion: com.github.triniwiz.canvas.CanvasCompositeOperationType;
					public static Hue: com.github.triniwiz.canvas.CanvasCompositeOperationType;
					public static Saturation: com.github.triniwiz.canvas.CanvasCompositeOperationType;
					public static Color: com.github.triniwiz.canvas.CanvasCompositeOperationType;
					public static Luminosity: com.github.triniwiz.canvas.CanvasCompositeOperationType;
					public toString(): string;
					public static valueOf(param0: string): com.github.triniwiz.canvas.CanvasCompositeOperationType;
					public static values(): native.Array<com.github.triniwiz.canvas.CanvasCompositeOperationType>;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class CanvasDOMMatrix {
					public static class: java.lang.Class<com.github.triniwiz.canvas.CanvasDOMMatrix>;
					public setB(param0: number): void;
					public getE(): number;
					public setC(param0: number): void;
					public setA(param0: number): void;
					public finalize(): void;
					public setD(param0: number): void;
					public getB(): number;
					public setE(param0: number): void;
					public setF(param0: number): void;
					public getF(): number;
					public getD(): number;
					public constructor();
					public getA(): number;
					public getC(): number;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class CanvasPath2D {
					public static class: java.lang.Class<com.github.triniwiz.canvas.CanvasPath2D>;
					public moveTo(param0: number, param1: number): void;
					public rect(param0: number, param1: number, param2: number, param3: number): void;
					public constructor(param0: string);
					public bezierCurveTo(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number): void;
					public finalize(): void;
					public arc(param0: number, param1: number, param2: number, param3: number, param4: number, param5: boolean): void;
					public ellipse(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: boolean): void;
					public constructor();
					public arcTo(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public quadraticCurveTo(param0: number, param1: number, param2: number, param3: number): void;
					public lineTo(param0: number, param1: number): void;
					public constructor(param0: com.github.triniwiz.canvas.CanvasPath2D);
					public addPath(param0: com.github.triniwiz.canvas.CanvasPath2D, param1: com.github.triniwiz.canvas.CanvasDOMMatrix): void;
					public closePath(): void;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class CanvasRenderingContext {
					public static class: java.lang.Class<com.github.triniwiz.canvas.CanvasRenderingContext>;
					/**
					 * Constructs a new instance of the com.github.triniwiz.canvas.CanvasRenderingContext interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
					 */
					public constructor(implementation: {
					});
					public constructor();
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class CanvasRenderingContext2D extends com.github.triniwiz.canvas.CanvasRenderingContext {
					public static class: java.lang.Class<com.github.triniwiz.canvas.CanvasRenderingContext2D>;
					public static isDebug: boolean;
					public block: boolean;
					public beginPath(): void;
					public drawImage(param0: com.github.triniwiz.canvas.CanvasView, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number): void;
					public isPointInPath(param0: number, param1: number, param2: string): boolean;
					public fillText(param0: string, param1: number, param2: number): void;
					public strokeRect(param0: number, param1: number, param2: number, param3: number): void;
					public strokeText(param0: string, param1: number, param2: number, param3: number): void;
					public bezierCurveTo(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number): void;
					public getDirection(): string;
					public fillRect(param0: number, param1: number, param2: number, param3: number): void;
					public translate(param0: number, param1: number): void;
					public getImageData(param0: number, param1: number, param2: number, param3: number): com.github.triniwiz.canvas.ImageData;
					public arc(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public drawImage(param0: globalAndroid.graphics.Bitmap, param1: number, param2: number, param3: number, param4: number): void;
					public getLineJoin(): com.github.triniwiz.canvas.CanvasRenderingContext2D.LineJoin;
					public isPointInStroke(param0: com.github.triniwiz.canvas.CanvasPath2D, param1: number, param2: number): boolean;
					public getGlobalCompositeOperation(): com.github.triniwiz.canvas.CanvasCompositeOperationType;
					public scale(param0: number, param1: number): void;
					public getGlobalAlpha(): number;
					public quadraticCurveTo(param0: number, param1: number, param2: number, param3: number): void;
					public transform(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number): void;
					public getShadowOffsetY(): number;
					public getShadowBlur(): number;
					public setGlobalCompositeOperation(param0: com.github.triniwiz.canvas.CanvasCompositeOperationType): void;
					public getFillStyle(): any;
					public setTextAlign(param0: com.github.triniwiz.canvas.CanvasTextAlignment): void;
					public setDirection(param0: string): void;
					public createPattern(param0: globalAndroid.graphics.Bitmap, param1: com.github.triniwiz.canvas.Pattern.PatternRepetition): com.github.triniwiz.canvas.Pattern;
					public fill(param0: com.github.triniwiz.canvas.CanvasPath2D, param1: string): void;
					public getImageSmoothingEnabled(): boolean;
					public getCurrentTransform(): com.github.triniwiz.canvas.CanvasDOMMatrix;
					public createLinearGradient(param0: number, param1: number, param2: number, param3: number): com.github.triniwiz.canvas.LinearGradient;
					public getFont(): string;
					public getLineDash(): native.Array<number>;
					public setShadowColor(param0: number): void;
					public getTextAlign(): com.github.triniwiz.canvas.CanvasTextAlignment;
					public save(): void;
					public drawImage(param0: com.github.triniwiz.canvas.ImageAsset, param1: number, param2: number, param3: number, param4: number): void;
					public setStrokeStyle(param0: number): void;
					public isPointInPath(param0: number, param1: number): boolean;
					public setFillStyle(param0: any): void;
					public getLineWidth(): number;
					public fill(param0: com.github.triniwiz.canvas.CanvasPath2D): void;
					public measureText(param0: string): com.github.triniwiz.canvas.CanvasTextMetrics;
					public setLineCap(param0: com.github.triniwiz.canvas.CanvasRenderingContext2D.LineCap): void;
					public createPattern(param0: com.github.triniwiz.canvas.CanvasView, param1: com.github.triniwiz.canvas.Pattern.PatternRepetition): com.github.triniwiz.canvas.Pattern;
					public createImageData(param0: number, param1: number): com.github.triniwiz.canvas.ImageData;
					public setCurrentTransform(param0: com.github.triniwiz.canvas.CanvasDOMMatrix): void;
					public getLineCap(): com.github.triniwiz.canvas.CanvasRenderingContext2D.LineCap;
					public setFillStyle(param0: number): void;
					public setLineWidth(param0: number): void;
					public getMiterLimit(): number;
					public setFont(param0: string): void;
					public setLineDashOffset(param0: number): void;
					public clip(param0: com.github.triniwiz.canvas.CanvasPath2D, param1: string): void;
					public stroke(): void;
					public drawImage(param0: com.github.triniwiz.canvas.CanvasView, param1: number, param2: number, param3: number, param4: number): void;
					public getShadowOffsetX(): number;
					public createRadialGradient(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number): com.github.triniwiz.canvas.RadialGradient;
					public clip(param0: string): void;
					public createPattern(param0: com.github.triniwiz.canvas.ImageAsset, param1: com.github.triniwiz.canvas.Pattern.PatternRepetition): com.github.triniwiz.canvas.Pattern;
					public getImageSmoothingQuality(): com.github.triniwiz.canvas.CanvasRenderingContext2D.ImageSmoothingQuality;
					public stroke(param0: com.github.triniwiz.canvas.CanvasPath2D): void;
					public lineTo(param0: number, param1: number): void;
					public restore(): void;
					public closePath(): void;
					public clearRect(param0: number, param1: number, param2: number, param3: number): void;
					public clip(param0: com.github.triniwiz.canvas.CanvasPath2D): void;
					public rect(param0: number, param1: number, param2: number, param3: number): void;
					public fill(): void;
					public ellipse(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number): void;
					public getCanvas(): com.github.triniwiz.canvas.CanvasView;
					public drawImage(param0: com.github.triniwiz.canvas.ImageAsset, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number): void;
					public fillText(param0: string, param1: number, param2: number, param3: number): void;
					public getStrokeStyle(): any;
					public arcTo(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public putImageData(param0: com.github.triniwiz.canvas.ImageData, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number): void;
					public setStrokeStyle(param0: any): void;
					public isPointInStroke(param0: number, param1: number): boolean;
					public drawImage(param0: globalAndroid.graphics.Bitmap, param1: number, param2: number): void;
					public getLineDashOffset(): number;
					public setGlobalAlpha(param0: number): void;
					public setLineJoin(param0: com.github.triniwiz.canvas.CanvasRenderingContext2D.LineJoin): void;
					public getShadowColor(): number;
					public clip(): void;
					public setTransform(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number): void;
					public createImageData(param0: com.github.triniwiz.canvas.ImageData): com.github.triniwiz.canvas.ImageData;
					public arc(param0: number, param1: number, param2: number, param3: number, param4: number, param5: boolean): void;
					public ellipse(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: boolean): void;
					public rotate(param0: number): void;
					public setShadowOffsetY(param0: number): void;
					public moveTo(param0: number, param1: number): void;
					public setImageSmoothingQuality(param0: com.github.triniwiz.canvas.CanvasRenderingContext2D.ImageSmoothingQuality): void;
					public strokeText(param0: string, param1: number, param2: number): void;
					public setImageSmoothingEnabled(param0: boolean): void;
					public drawImage(param0: com.github.triniwiz.canvas.ImageAsset, param1: number, param2: number): void;
					public putImageData(param0: com.github.triniwiz.canvas.ImageData, param1: number, param2: number): void;
					public resetTransform(): void;
					public setMiterLimit(param0: number): void;
					public drawImage(param0: com.github.triniwiz.canvas.CanvasView, param1: number, param2: number): void;
					public putImageData(param0: com.github.triniwiz.canvas.ImageData): void;
					public setShadowBlur(param0: number): void;
					public fill(param0: string): void;
					public setLineDash(param0: native.Array<number>): void;
					public isPointInPath(param0: com.github.triniwiz.canvas.CanvasPath2D, param1: number, param2: number, param3: string): boolean;
					public drawImage(param0: globalAndroid.graphics.Bitmap, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number): void;
					public setShadowOffsetX(param0: number): void;
				}
				export module CanvasRenderingContext2D {
					export class ImageSmoothingQuality {
						public static class: java.lang.Class<com.github.triniwiz.canvas.CanvasRenderingContext2D.ImageSmoothingQuality>;
						public static Low: com.github.triniwiz.canvas.CanvasRenderingContext2D.ImageSmoothingQuality;
						public static Medium: com.github.triniwiz.canvas.CanvasRenderingContext2D.ImageSmoothingQuality;
						public static High: com.github.triniwiz.canvas.CanvasRenderingContext2D.ImageSmoothingQuality;
						public static values(): native.Array<com.github.triniwiz.canvas.CanvasRenderingContext2D.ImageSmoothingQuality>;
						public static valueOf(param0: string): com.github.triniwiz.canvas.CanvasRenderingContext2D.ImageSmoothingQuality;
						public toString(): string;
					}
					export class LineCap {
						public static class: java.lang.Class<com.github.triniwiz.canvas.CanvasRenderingContext2D.LineCap>;
						public static Butt: com.github.triniwiz.canvas.CanvasRenderingContext2D.LineCap;
						public static Round: com.github.triniwiz.canvas.CanvasRenderingContext2D.LineCap;
						public static Square: com.github.triniwiz.canvas.CanvasRenderingContext2D.LineCap;
						public static values(): native.Array<com.github.triniwiz.canvas.CanvasRenderingContext2D.LineCap>;
						public static valueOf(param0: string): com.github.triniwiz.canvas.CanvasRenderingContext2D.LineCap;
						public toString(): string;
					}
					export class LineJoin {
						public static class: java.lang.Class<com.github.triniwiz.canvas.CanvasRenderingContext2D.LineJoin>;
						public static Bevel: com.github.triniwiz.canvas.CanvasRenderingContext2D.LineJoin;
						public static Round: com.github.triniwiz.canvas.CanvasRenderingContext2D.LineJoin;
						public static Miter: com.github.triniwiz.canvas.CanvasRenderingContext2D.LineJoin;
						public static values(): native.Array<com.github.triniwiz.canvas.CanvasRenderingContext2D.LineJoin>;
						public toString(): string;
						public static valueOf(param0: string): com.github.triniwiz.canvas.CanvasRenderingContext2D.LineJoin;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class CanvasTextAlignment {
					public static class: java.lang.Class<com.github.triniwiz.canvas.CanvasTextAlignment>;
					public static Left: com.github.triniwiz.canvas.CanvasTextAlignment;
					public static Right: com.github.triniwiz.canvas.CanvasTextAlignment;
					public static Center: com.github.triniwiz.canvas.CanvasTextAlignment;
					public static Start: com.github.triniwiz.canvas.CanvasTextAlignment;
					public static End: com.github.triniwiz.canvas.CanvasTextAlignment;
					public toString(): string;
					public static valueOf(param0: string): com.github.triniwiz.canvas.CanvasTextAlignment;
					public static values(): native.Array<com.github.triniwiz.canvas.CanvasTextAlignment>;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class CanvasTextBaseline {
					public static class: java.lang.Class<com.github.triniwiz.canvas.CanvasTextBaseline>;
					public static Top: com.github.triniwiz.canvas.CanvasTextBaseline;
					public static Hanging: com.github.triniwiz.canvas.CanvasTextBaseline;
					public static Middle: com.github.triniwiz.canvas.CanvasTextBaseline;
					public static Alphabetic: com.github.triniwiz.canvas.CanvasTextBaseline;
					public static Ideographic: com.github.triniwiz.canvas.CanvasTextBaseline;
					public static Bottom: com.github.triniwiz.canvas.CanvasTextBaseline;
					public static valueOf(param0: string): com.github.triniwiz.canvas.CanvasTextBaseline;
					public toString(): string;
					public static values(): native.Array<com.github.triniwiz.canvas.CanvasTextBaseline>;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class CanvasTextMetrics {
					public static class: java.lang.Class<com.github.triniwiz.canvas.CanvasTextMetrics>;
					public getWidth(): number;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class CanvasView {
					public static class: java.lang.Class<com.github.triniwiz.canvas.CanvasView>;
					public onActivityStarted(param0: globalAndroid.app.Activity): void;
					public onActivityCreated(param0: globalAndroid.app.Activity, param1: globalAndroid.os.Bundle): void;
					public doFrame(param0: number): void;
					public finalize(): void;
					public queueEvent(param0: java.lang.Runnable): void;
					public setupActivityHandler(param0: globalAndroid.app.Application): void;
					public setHandleInvalidationManually(param0: boolean): void;
					public static getViews(): java.util.concurrent.ConcurrentHashMap;
					public onAttachedToWindow(): void;
					public getSurface(): com.github.triniwiz.canvas.GLView;
					public toDataURLAsync(param0: string, param1: number, param2: com.github.triniwiz.canvas.CanvasView.DataURLListener): void;
					public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet);
					public onActivitySaveInstanceState(param0: globalAndroid.app.Activity, param1: globalAndroid.os.Bundle): void;
					public toDataURLAsync(param0: com.github.triniwiz.canvas.CanvasView.DataURLListener): void;
					public destroy(): void;
					public resizeViewPort(): void;
					public toDataURL(): string;
					public onActivityResumed(param0: globalAndroid.app.Activity): void;
					public onActivityPaused(param0: globalAndroid.app.Activity): void;
					public onActivityDestroyed(param0: globalAndroid.app.Activity): void;
					public onDetachedFromWindow(): void;
					public setListener(param0: com.github.triniwiz.canvas.CanvasView.Listener): void;
					public flush(): void;
					public toDataURLAsync(param0: string, param1: com.github.triniwiz.canvas.CanvasView.DataURLListener): void;
					public onSizeChanged(param0: number, param1: number, param2: number, param3: number): void;
					public onResume(): void;
					public constructor(param0: globalAndroid.content.Context);
					public onActivityStopped(param0: globalAndroid.app.Activity): void;
					public getContext(param0: string, param1: java.util.Map<string,any>): com.github.triniwiz.canvas.CanvasRenderingContext;
					public constructor(param0: globalAndroid.content.Context, param1: boolean);
					public toDataURL(param0: string): string;
					public isHandleInvalidationManually(): boolean;
					public toDataURL(param0: string, param1: number): string;
					public onPause(): void;
					public toData(): native.Array<number>;
					public getContext(param0: string): com.github.triniwiz.canvas.CanvasRenderingContext;
					public static createSVGMatrix(): com.github.triniwiz.canvas.CanvasDOMMatrix;
				}
				export module CanvasView {
					export class ContextType {
						public static class: java.lang.Class<com.github.triniwiz.canvas.CanvasView.ContextType>;
						public static NONE: com.github.triniwiz.canvas.CanvasView.ContextType;
						public static CANVAS: com.github.triniwiz.canvas.CanvasView.ContextType;
						public static WEBGL: com.github.triniwiz.canvas.CanvasView.ContextType;
						public static valueOf(param0: string): com.github.triniwiz.canvas.CanvasView.ContextType;
						public static values(): native.Array<com.github.triniwiz.canvas.CanvasView.ContextType>;
					}
					export class DataURLListener {
						public static class: java.lang.Class<com.github.triniwiz.canvas.CanvasView.DataURLListener>;
						/**
						 * Constructs a new instance of the com.github.triniwiz.canvas.CanvasView$DataURLListener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							onResult(param0: string): void;
						});
						public constructor();
						public onResult(param0: string): void;
					}
					export class Listener {
						public static class: java.lang.Class<com.github.triniwiz.canvas.CanvasView.Listener>;
						/**
						 * Constructs a new instance of the com.github.triniwiz.canvas.CanvasView$Listener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							contextReady(): void;
						});
						public constructor();
						public contextReady(): void;
					}
					export class Size {
						public static class: java.lang.Class<com.github.triniwiz.canvas.CanvasView.Size>;
						public getWidth(): number;
						public getHeight(): number;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class Color extends com.github.triniwiz.canvas.ICanvasColorStyle {
					public static class: java.lang.Class<com.github.triniwiz.canvas.Color>;
					public getStyleType(): com.github.triniwiz.canvas.CanvasColorStyleType;
					public constructor(param0: string);
					public constructor(param0: number);
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class Colors {
					public static class: java.lang.Class<com.github.triniwiz.canvas.Colors>;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class Constants {
					public static class: java.lang.Class<com.github.triniwiz.canvas.Constants>;
					public static GL_RGBA16F_EXT: number;
					public static GL_RGB16F_EXT: number;
					public static GL_RG16F_EXT: number;
					public static GL_R16F_EXT: number;
					public static GL_R32F_EXT: number;
					public static GL_RG32F_EXT: number;
					public static GL_RGBA32F_EXT: number;
					public static GL_RGB32F_EXT: number;
					public static GL_MIN_EXT: number;
					public static GL_MAX_EXT: number;
					public static GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT: number;
					public static GL_UNSIGNED_NORMALIZED_EXT: number;
					public static GL_SRGB_EXT: number;
					public static GL_SRGB_ALPHA_EXT: number;
					public static GL_SRGB8_ALPHA8_EXT: number;
					public static GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT: number;
					public static GL_TEXTURE_MAX_ANISOTROPY_EXT: number;
					public static GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT: number;
					public static GL_QUERY_COUNTER_BITS_EXT: number;
					public static GL_CURRENT_QUERY_EXT: number;
					public static GL_QUERY_RESULT_EXT: number;
					public static GL_QUERY_RESULT_AVAILABLE_EXT: number;
					public static GL_TIME_ELAPSED_EXT: number;
					public static GL_TIMESTAMP_EXT: number;
					public static GL_GPU_DISJOINT_EXT: number;
					public static GL_FRAGMENT_SHADER_DERIVATIVE_HINT_OES: number;
					public static GL_HALF_FLOAT_OES: number;
					public static GL_VERTEX_ARRAY_BINDING_OES: number;
					public static COMPRESSED_RGB_ATC_WEBGL: number;
					public static COMPRESSED_RGBA_ATC_EXPLICIT_ALPHA_WEBGL: number;
					public static COMPRESSED_RGBA_ATC_INTERPOLATED_ALPHA_WEBGL: number;
					public static GL_COMPRESSED_RGB_PVRTC_4BPPV1_IMG: number;
					public static GL_COMPRESSED_RGB_PVRTC_2BPPV1_IMG: number;
					public static GL_COMPRESSED_RGBA_PVRTC_4BPPV1_IMG: number;
					public static GL_COMPRESSED_RGBA_PVRTC_2BPPV1_IMG: number;
					public static GL_COMPRESSED_RGB_S3TC_DXT1_EXT: number;
					public static GL_COMPRESSED_RGBA_S3TC_DXT1_EXT: number;
					public static GL_COMPRESSED_RGBA_S3TC_DXT3_EXT: number;
					public static GL_COMPRESSED_RGBA_S3TC_DXT5_EXT: number;
					public static GL_COMPRESSED_SRGB_S3TC_DXT1_EXT: number;
					public static GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT: number;
					public static GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT: number;
					public static GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT: number;
					public static GL_MAX_COLOR_ATTACHMENTS_EXT: number;
					public static GL_MAX_DRAW_BUFFERS_EXT: number;
					public static GL_DRAW_BUFFER0_EXT: number;
					public static GL_DRAW_BUFFER1_EXT: number;
					public static GL_DRAW_BUFFER2_EXT: number;
					public static GL_DRAW_BUFFER3_EXT: number;
					public static GL_DRAW_BUFFER4_EXT: number;
					public static GL_DRAW_BUFFER5_EXT: number;
					public static GL_DRAW_BUFFER6_EXT: number;
					public static GL_DRAW_BUFFER7_EXT: number;
					public static GL_DRAW_BUFFER8_EXT: number;
					public static GL_DRAW_BUFFER9_EXT: number;
					public static GL_DRAW_BUFFER10_EXT: number;
					public static GL_DRAW_BUFFER11_EXT: number;
					public static GL_DRAW_BUFFER12_EXT: number;
					public static GL_DRAW_BUFFER13_EXT: number;
					public static GL_DRAW_BUFFER14_EXT: number;
					public static GL_DRAW_BUFFER15_EXT: number;
					public static GL_COLOR_ATTACHMENT0_EXT: number;
					public static GL_COLOR_ATTACHMENT1_EXT: number;
					public static GL_COLOR_ATTACHMENT2_EXT: number;
					public static GL_COLOR_ATTACHMENT3_EXT: number;
					public static GL_COLOR_ATTACHMENT4_EXT: number;
					public static GL_COLOR_ATTACHMENT5_EXT: number;
					public static GL_COLOR_ATTACHMENT6_EXT: number;
					public static GL_COLOR_ATTACHMENT7_EXT: number;
					public static GL_COLOR_ATTACHMENT8_EXT: number;
					public static GL_COLOR_ATTACHMENT9_EXT: number;
					public static GL_COLOR_ATTACHMENT10_EXT: number;
					public static GL_COLOR_ATTACHMENT11_EXT: number;
					public static GL_COLOR_ATTACHMENT12_EXT: number;
					public static GL_COLOR_ATTACHMENT13_EXT: number;
					public static GL_COLOR_ATTACHMENT14_EXT: number;
					public static GL_COLOR_ATTACHMENT15_EXT: number;
					public static MAX_CLIENT_WAIT_TIMEOUT_WEBGL: number;
					public constructor();
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class FileReader {
					public static class: java.lang.Class<com.github.triniwiz.canvas.FileReader>;
					public static read(param0: string): native.Array<number>;
					public constructor();
					public static read(param0: java.io.File): native.Array<number>;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class FramebufferAttachmentParameter {
					public static class: java.lang.Class<com.github.triniwiz.canvas.FramebufferAttachmentParameter>;
					public constructor();
					public getValue(): number;
					public isRenderbuffer(): boolean;
					public isTexture(): boolean;
					public constructor(param0: boolean, param1: boolean, param2: number);
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class GLContext {
					public static class: java.lang.Class<com.github.triniwiz.canvas.GLContext>;
					public glView: java.lang.ref.WeakReference<com.github.triniwiz.canvas.GLView>;
					public doFrame(param0: number): void;
					public queueEvent(param0: java.lang.Runnable): void;
					public swapBuffers(param0: javax.microedition.khronos.egl.EGLSurface): boolean;
					public init(param0: any): void;
					public isHeadless(): boolean;
					public flush(): void;
					public createSurface(param0: javax.microedition.khronos.egl.EGLConfig, param1: any): javax.microedition.khronos.egl.EGLSurface;
					public onResume(): void;
					public makeCurrent(param0: javax.microedition.khronos.egl.EGLSurface): boolean;
					public destroy(): void;
					public isGLThreadStarted(): boolean;
					public constructor();
					public destroySurface(param0: javax.microedition.khronos.egl.EGLSurface): boolean;
					public onPause(): void;
				}
				export module GLContext {
					export class GLThread {
						public static class: java.lang.Class<com.github.triniwiz.canvas.GLContext.GLThread>;
						public start(): void;
						public run(): void;
						public constructor(param0: com.github.triniwiz.canvas.GLContext, param1: any);
						public interrupt(): void;
						public setPaused(param0: boolean): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class GLTextureView {
					public static class: java.lang.Class<com.github.triniwiz.canvas.GLTextureView>;
					public static RENDERMODE_WHEN_DIRTY: number;
					public static RENDERMODE_CONTINUOUSLY: number;
					public static DEBUG_CHECK_GL_ERROR: number;
					public static DEBUG_LOG_GL_CALLS: number;
					public setPreserveEGLContextOnPause(param0: boolean): void;
					public setGLWrapper(param0: com.github.triniwiz.canvas.GLTextureView.GLWrapper): void;
					public setEGLContextFactory(param0: com.github.triniwiz.canvas.GLTextureView.EGLContextFactory): void;
					public finalize(): void;
					public surfaceCreated(param0: globalAndroid.graphics.SurfaceTexture): void;
					public queueEvent(param0: java.lang.Runnable): void;
					public setEGLConfigChooser(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number): void;
					public getDebugFlags(): number;
					public onAttachedToWindow(): void;
					public setDebugFlags(param0: number): void;
					public setEGLConfigChooser(param0: com.github.triniwiz.canvas.GLTextureView.EGLConfigChooser): void;
					public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet);
					public setEGLWindowSurfaceFactory(param0: com.github.triniwiz.canvas.GLTextureView.EGLWindowSurfaceFactory): void;
					public onSurfaceTextureAvailable(param0: globalAndroid.graphics.SurfaceTexture, param1: number, param2: number): void;
					public setDrawListener(param0: com.github.triniwiz.canvas.GLTextureView.Listener): void;
					public onLayoutChange(param0: globalAndroid.view.View, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number): void;
					public setEGLContextClientVersion(param0: number): void;
					public getPreserveEGLContextOnPause(): boolean;
					public onDetachedFromWindow(): void;
					public surfaceChanged(param0: globalAndroid.graphics.SurfaceTexture, param1: number, param2: number, param3: number): void;
					public surfaceDestroyed(param0: globalAndroid.graphics.SurfaceTexture): void;
					public setEGLConfigChooser(param0: boolean): void;
					public onSurfaceTextureDestroyed(param0: globalAndroid.graphics.SurfaceTexture): boolean;
					public onSurfaceTextureUpdated(param0: globalAndroid.graphics.SurfaceTexture): void;
					public requestRender(): void;
					public setRenderer(param0: com.github.triniwiz.canvas.GLTextureView.Renderer): void;
					public onResume(): void;
					public constructor(param0: globalAndroid.content.Context);
					public addSurfaceTextureListener(param0: globalAndroid.view.TextureView.SurfaceTextureListener): void;
					public getRenderMode(): number;
					public onPause(): void;
					public onSurfaceTextureSizeChanged(param0: globalAndroid.graphics.SurfaceTexture, param1: number, param2: number): void;
					public setRenderMode(param0: number): void;
				}
				export module GLTextureView {
					export abstract class BaseConfigChooser extends com.github.triniwiz.canvas.GLTextureView.EGLConfigChooser {
						public static class: java.lang.Class<com.github.triniwiz.canvas.GLTextureView.BaseConfigChooser>;
						public mConfigSpec: native.Array<number>;
						public constructor(param0: com.github.triniwiz.canvas.GLTextureView, param1: native.Array<number>);
						public chooseConfig(param0: javax.microedition.khronos.egl.EGL10, param1: javax.microedition.khronos.egl.EGLDisplay): javax.microedition.khronos.egl.EGLConfig;
					}
					export class ComponentSizeChooser extends com.github.triniwiz.canvas.GLTextureView.BaseConfigChooser {
						public static class: java.lang.Class<com.github.triniwiz.canvas.GLTextureView.ComponentSizeChooser>;
						public redSize: number;
						public greenSize: number;
						public blueSize: number;
						public alphaSize: number;
						public depthSize: number;
						public stencilSize: number;
						public chooseConfig(param0: javax.microedition.khronos.egl.EGL10, param1: javax.microedition.khronos.egl.EGLDisplay, param2: native.Array<javax.microedition.khronos.egl.EGLConfig>): javax.microedition.khronos.egl.EGLConfig;
						public constructor(param0: com.github.triniwiz.canvas.GLTextureView, param1: native.Array<number>);
						public constructor(param0: com.github.triniwiz.canvas.GLTextureView, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number);
						public chooseConfig(param0: javax.microedition.khronos.egl.EGL10, param1: javax.microedition.khronos.egl.EGLDisplay): javax.microedition.khronos.egl.EGLConfig;
					}
					export class DefaultContextFactory extends com.github.triniwiz.canvas.GLTextureView.EGLContextFactory {
						public static class: java.lang.Class<com.github.triniwiz.canvas.GLTextureView.DefaultContextFactory>;
						public createContext(param0: javax.microedition.khronos.egl.EGL10, param1: javax.microedition.khronos.egl.EGLDisplay, param2: javax.microedition.khronos.egl.EGLConfig): javax.microedition.khronos.egl.EGLContext;
						public destroyContext(param0: javax.microedition.khronos.egl.EGL10, param1: javax.microedition.khronos.egl.EGLDisplay, param2: javax.microedition.khronos.egl.EGLContext): void;
					}
					export class DefaultWindowSurfaceFactory extends com.github.triniwiz.canvas.GLTextureView.EGLWindowSurfaceFactory {
						public static class: java.lang.Class<com.github.triniwiz.canvas.GLTextureView.DefaultWindowSurfaceFactory>;
						public createWindowSurface(param0: javax.microedition.khronos.egl.EGL10, param1: javax.microedition.khronos.egl.EGLDisplay, param2: javax.microedition.khronos.egl.EGLConfig, param3: any): javax.microedition.khronos.egl.EGLSurface;
						public destroySurface(param0: javax.microedition.khronos.egl.EGL10, param1: javax.microedition.khronos.egl.EGLDisplay, param2: javax.microedition.khronos.egl.EGLSurface): void;
					}
					export class EGLConfigChooser {
						public static class: java.lang.Class<com.github.triniwiz.canvas.GLTextureView.EGLConfigChooser>;
						/**
						 * Constructs a new instance of the com.github.triniwiz.canvas.GLTextureView$EGLConfigChooser interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							chooseConfig(param0: javax.microedition.khronos.egl.EGL10, param1: javax.microedition.khronos.egl.EGLDisplay): javax.microedition.khronos.egl.EGLConfig;
						});
						public constructor();
						public chooseConfig(param0: javax.microedition.khronos.egl.EGL10, param1: javax.microedition.khronos.egl.EGLDisplay): javax.microedition.khronos.egl.EGLConfig;
					}
					export class EGLContextFactory {
						public static class: java.lang.Class<com.github.triniwiz.canvas.GLTextureView.EGLContextFactory>;
						/**
						 * Constructs a new instance of the com.github.triniwiz.canvas.GLTextureView$EGLContextFactory interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							createContext(param0: javax.microedition.khronos.egl.EGL10, param1: javax.microedition.khronos.egl.EGLDisplay, param2: javax.microedition.khronos.egl.EGLConfig): javax.microedition.khronos.egl.EGLContext;
							destroyContext(param0: javax.microedition.khronos.egl.EGL10, param1: javax.microedition.khronos.egl.EGLDisplay, param2: javax.microedition.khronos.egl.EGLContext): void;
						});
						public constructor();
						public createContext(param0: javax.microedition.khronos.egl.EGL10, param1: javax.microedition.khronos.egl.EGLDisplay, param2: javax.microedition.khronos.egl.EGLConfig): javax.microedition.khronos.egl.EGLContext;
						public destroyContext(param0: javax.microedition.khronos.egl.EGL10, param1: javax.microedition.khronos.egl.EGLDisplay, param2: javax.microedition.khronos.egl.EGLContext): void;
					}
					export class EGLWindowSurfaceFactory {
						public static class: java.lang.Class<com.github.triniwiz.canvas.GLTextureView.EGLWindowSurfaceFactory>;
						/**
						 * Constructs a new instance of the com.github.triniwiz.canvas.GLTextureView$EGLWindowSurfaceFactory interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							createWindowSurface(param0: javax.microedition.khronos.egl.EGL10, param1: javax.microedition.khronos.egl.EGLDisplay, param2: javax.microedition.khronos.egl.EGLConfig, param3: any): javax.microedition.khronos.egl.EGLSurface;
							destroySurface(param0: javax.microedition.khronos.egl.EGL10, param1: javax.microedition.khronos.egl.EGLDisplay, param2: javax.microedition.khronos.egl.EGLSurface): void;
						});
						public constructor();
						public createWindowSurface(param0: javax.microedition.khronos.egl.EGL10, param1: javax.microedition.khronos.egl.EGLDisplay, param2: javax.microedition.khronos.egl.EGLConfig, param3: any): javax.microedition.khronos.egl.EGLSurface;
						public destroySurface(param0: javax.microedition.khronos.egl.EGL10, param1: javax.microedition.khronos.egl.EGLDisplay, param2: javax.microedition.khronos.egl.EGLSurface): void;
					}
					export class EglHelper {
						public static class: java.lang.Class<com.github.triniwiz.canvas.GLTextureView.EglHelper>;
						public static logEglErrorAsWarning(param0: string, param1: string, param2: number): void;
						public static formatEglError(param0: string, param1: number): string;
						public constructor(param0: java.lang.ref.WeakReference<com.github.triniwiz.canvas.GLTextureView>);
						public start(): void;
						public swap(): number;
						public finish(): void;
						public static throwEglException(param0: string, param1: number): void;
						public makeContextCurrent(): boolean;
						public destroySurface(): void;
						public createSurface(): boolean;
					}
					export class GLThread {
						public static class: java.lang.Class<com.github.triniwiz.canvas.GLTextureView.GLThread>;
						public queueEvent(param0: java.lang.Runnable): void;
						public requestReleaseEglContextLocked(): void;
						public onResume(): void;
						public ableToDraw(): boolean;
						public surfaceDestroyed(): void;
						public onPause(): void;
						public requestRender(): void;
						public surfaceCreated(): void;
						public run(): void;
						public onWindowResize(param0: number, param1: number): void;
						public getRenderMode(): number;
						public requestExitAndWait(): void;
						public setRenderMode(param0: number): void;
					}
					export class GLThreadManager {
						public static class: java.lang.Class<com.github.triniwiz.canvas.GLTextureView.GLThreadManager>;
						public threadExiting(param0: com.github.triniwiz.canvas.GLTextureView.GLThread): void;
						public tryAcquireEglContextLocked(param0: com.github.triniwiz.canvas.GLTextureView.GLThread): boolean;
						public releaseEglContextLocked(param0: com.github.triniwiz.canvas.GLTextureView.GLThread): void;
						public checkGLDriver(param0: javax.microedition.khronos.opengles.GL10): void;
						public shouldTerminateEGLWhenPausing(): boolean;
						public shouldReleaseEGLContextWhenPausing(): boolean;
					}
					export class GLWrapper {
						public static class: java.lang.Class<com.github.triniwiz.canvas.GLTextureView.GLWrapper>;
						/**
						 * Constructs a new instance of the com.github.triniwiz.canvas.GLTextureView$GLWrapper interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							wrap(param0: javax.microedition.khronos.opengles.GL): javax.microedition.khronos.opengles.GL;
						});
						public constructor();
						public wrap(param0: javax.microedition.khronos.opengles.GL): javax.microedition.khronos.opengles.GL;
					}
					export class Listener {
						public static class: java.lang.Class<com.github.triniwiz.canvas.GLTextureView.Listener>;
						/**
						 * Constructs a new instance of the com.github.triniwiz.canvas.GLTextureView$Listener interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							didDraw(): void;
						});
						public constructor();
						public didDraw(): void;
					}
					export class LogWriter {
						public static class: java.lang.Class<com.github.triniwiz.canvas.GLTextureView.LogWriter>;
						public close(): void;
						public flush(): void;
						public write(param0: native.Array<string>, param1: number, param2: number): void;
					}
					export class Renderer {
						public static class: java.lang.Class<com.github.triniwiz.canvas.GLTextureView.Renderer>;
						/**
						 * Constructs a new instance of the com.github.triniwiz.canvas.GLTextureView$Renderer interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							onSurfaceCreated(param0: javax.microedition.khronos.opengles.GL10, param1: javax.microedition.khronos.egl.EGLConfig): void;
							onSurfaceChanged(param0: javax.microedition.khronos.opengles.GL10, param1: number, param2: number): void;
							onDrawFrame(param0: javax.microedition.khronos.opengles.GL10): void;
						});
						public constructor();
						public onSurfaceChanged(param0: javax.microedition.khronos.opengles.GL10, param1: number, param2: number): void;
						public onSurfaceCreated(param0: javax.microedition.khronos.opengles.GL10, param1: javax.microedition.khronos.egl.EGLConfig): void;
						public onDrawFrame(param0: javax.microedition.khronos.opengles.GL10): void;
					}
					export class SimpleEGLConfigChooser extends com.github.triniwiz.canvas.GLTextureView.ComponentSizeChooser {
						public static class: java.lang.Class<com.github.triniwiz.canvas.GLTextureView.SimpleEGLConfigChooser>;
						public chooseConfig(param0: javax.microedition.khronos.egl.EGL10, param1: javax.microedition.khronos.egl.EGLDisplay, param2: native.Array<javax.microedition.khronos.egl.EGLConfig>): javax.microedition.khronos.egl.EGLConfig;
						public constructor(param0: com.github.triniwiz.canvas.GLTextureView, param1: native.Array<number>);
						public constructor(param0: com.github.triniwiz.canvas.GLTextureView, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number);
						public chooseConfig(param0: javax.microedition.khronos.egl.EGL10, param1: javax.microedition.khronos.egl.EGLDisplay): javax.microedition.khronos.egl.EGLConfig;
						public constructor(param0: com.github.triniwiz.canvas.GLTextureView, param1: boolean);
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class GLView {
					public static class: java.lang.Class<com.github.triniwiz.canvas.GLView>;
					public constructor(param0: globalAndroid.content.Context);
					public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet);
					public onSurfaceTextureAvailable(param0: globalAndroid.graphics.SurfaceTexture, param1: number, param2: number): void;
					public setListener(param0: com.github.triniwiz.canvas.CanvasView.Listener): void;
					public queueEvent(param0: java.lang.Runnable): void;
					public onSurfaceTextureDestroyed(param0: globalAndroid.graphics.SurfaceTexture): boolean;
					public onSurfaceTextureUpdated(param0: globalAndroid.graphics.SurfaceTexture): void;
					public flush(): void;
					public getGLContext(): com.github.triniwiz.canvas.GLContext;
					public onSurfaceTextureSizeChanged(param0: globalAndroid.graphics.SurfaceTexture, param1: number, param2: number): void;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class GLViewSV {
					public static class: java.lang.Class<com.github.triniwiz.canvas.GLViewSV>;
					public constructor(param0: globalAndroid.content.Context);
					public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet);
					public setListener(param0: com.github.triniwiz.canvas.CanvasView.Listener): void;
					public surfaceDestroyed(param0: globalAndroid.view.SurfaceHolder): void;
					public queueEvent(param0: java.lang.Runnable): void;
					public surfaceCreated(param0: globalAndroid.view.SurfaceHolder): void;
					public flush(): void;
					public getGLContext(): com.github.triniwiz.canvas.GLContext;
					public surfaceChanged(param0: globalAndroid.view.SurfaceHolder, param1: number, param2: number, param3: number): void;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class Gradient extends com.github.triniwiz.canvas.ICanvasColorStyle {
					public static class: java.lang.Class<com.github.triniwiz.canvas.Gradient>;
					public addColorStop(param0: number, param1: number): void;
					public getStyleType(): com.github.triniwiz.canvas.CanvasColorStyleType;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class ICanvasColorStyle {
					public static class: java.lang.Class<com.github.triniwiz.canvas.ICanvasColorStyle>;
					/**
					 * Constructs a new instance of the com.github.triniwiz.canvas.ICanvasColorStyle interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
					 */
					public constructor(implementation: {
						getStyleType(): com.github.triniwiz.canvas.CanvasColorStyleType;
					});
					public constructor();
					public getStyleType(): com.github.triniwiz.canvas.CanvasColorStyleType;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class ImageAsset {
					public static class: java.lang.Class<com.github.triniwiz.canvas.ImageAsset>;
					public loadImageFromPathAsync(param0: string, param1: com.github.triniwiz.canvas.ImageAsset.Callback): void;
					public getBuffer(): java.nio.ByteBuffer;
					public loadImageFromBuffer(param0: java.nio.ByteBuffer): boolean;
					public flipX(): void;
					public getHeight(): number;
					public finalize(): void;
					public getWidth(): number;
					public loadImageFromPath(param0: string): boolean;
					public loadImageFromImage(param0: globalAndroid.graphics.Bitmap): boolean;
					public save(param0: string, param1: com.github.triniwiz.canvas.ImageAssetFormat): boolean;
					public getBytes(): native.Array<number>;
					public getError(): string;
					public loadImageFromImageAsync(param0: globalAndroid.graphics.Bitmap, param1: com.github.triniwiz.canvas.ImageAsset.Callback): void;
					public flipY(): void;
					public loadImageFromBuferAsync(param0: java.nio.ByteBuffer, param1: com.github.triniwiz.canvas.ImageAsset.Callback): void;
					public scale(param0: number, param1: number): void;
					public constructor();
					public loadImageFromBytesAsync(param0: native.Array<number>, param1: com.github.triniwiz.canvas.ImageAsset.Callback): void;
					public saveAsync(param0: string, param1: com.github.triniwiz.canvas.ImageAssetFormat, param2: com.github.triniwiz.canvas.ImageAsset.Callback): void;
					public loadImageFromBytes(param0: native.Array<number>): boolean;
				}
				export module ImageAsset {
					export class Callback {
						public static class: java.lang.Class<com.github.triniwiz.canvas.ImageAsset.Callback>;
						/**
						 * Constructs a new instance of the com.github.triniwiz.canvas.ImageAsset$Callback interface with the provided implementation. An empty constructor exists calling super() when extending the interface class.
						 */
						public constructor(implementation: {
							onSuccess(param0: any): void;
							onError(param0: string): void;
						});
						public constructor();
						public onSuccess(param0: any): void;
						public onError(param0: string): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class ImageAssetFormat {
					public static class: java.lang.Class<com.github.triniwiz.canvas.ImageAssetFormat>;
					public static JPG: com.github.triniwiz.canvas.ImageAssetFormat;
					public static PNG: com.github.triniwiz.canvas.ImageAssetFormat;
					public static ICO: com.github.triniwiz.canvas.ImageAssetFormat;
					public static BMP: com.github.triniwiz.canvas.ImageAssetFormat;
					public static TIFF: com.github.triniwiz.canvas.ImageAssetFormat;
					public getFormat(): number;
					public static valueOf(param0: string): com.github.triniwiz.canvas.ImageAssetFormat;
					public static values(): native.Array<com.github.triniwiz.canvas.ImageAssetFormat>;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class ImageData {
					public static class: java.lang.Class<com.github.triniwiz.canvas.ImageData>;
					public getHeight(): number;
					public getWidth(): number;
					public getData(): java.nio.ByteBuffer;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class IndexedParameter {
					public static class: java.lang.Class<com.github.triniwiz.canvas.IndexedParameter>;
					public getIsBuffer(): boolean;
					public constructor();
					public getValue(): number;
					public getBufferValue(): number;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class LinearGradient extends com.github.triniwiz.canvas.Gradient {
					public static class: java.lang.Class<com.github.triniwiz.canvas.LinearGradient>;
					public constructor(param0: number, param1: number, param2: number, param3: number);
					public getStyleType(): com.github.triniwiz.canvas.CanvasColorStyleType;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class Pattern extends com.github.triniwiz.canvas.ICanvasColorStyle {
					public static class: java.lang.Class<com.github.triniwiz.canvas.Pattern>;
					public getStyleType(): com.github.triniwiz.canvas.CanvasColorStyleType;
					public constructor(param0: com.github.triniwiz.canvas.CanvasView, param1: com.github.triniwiz.canvas.ImageAsset, param2: com.github.triniwiz.canvas.Pattern.PatternRepetition);
					public finalize(): void;
					public setTransform(param0: com.github.triniwiz.canvas.CanvasDOMMatrix): void;
					public constructor(param0: com.github.triniwiz.canvas.CanvasView, param1: com.github.triniwiz.canvas.CanvasView, param2: com.github.triniwiz.canvas.Pattern.PatternRepetition);
					public constructor(param0: com.github.triniwiz.canvas.CanvasView, param1: globalAndroid.graphics.Bitmap, param2: com.github.triniwiz.canvas.Pattern.PatternRepetition);
				}
				export module Pattern {
					export class PatternRepetition {
						public static class: java.lang.Class<com.github.triniwiz.canvas.Pattern.PatternRepetition>;
						public static Repeat: com.github.triniwiz.canvas.Pattern.PatternRepetition;
						public static RepeatX: com.github.triniwiz.canvas.Pattern.PatternRepetition;
						public static RepeatY: com.github.triniwiz.canvas.Pattern.PatternRepetition;
						public static NoRepeat: com.github.triniwiz.canvas.Pattern.PatternRepetition;
						public static values(): native.Array<com.github.triniwiz.canvas.Pattern.PatternRepetition>;
						public static valueOf(param0: string): com.github.triniwiz.canvas.Pattern.PatternRepetition;
						public toString(): string;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class RadialGradient extends com.github.triniwiz.canvas.Gradient {
					public static class: java.lang.Class<com.github.triniwiz.canvas.RadialGradient>;
					public getStyleType(): com.github.triniwiz.canvas.CanvasColorStyleType;
					public constructor(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number);
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class SVGView {
					public static class: java.lang.Class<com.github.triniwiz.canvas.SVGView>;
					public bitmap: globalAndroid.graphics.Bitmap;
					public svgCanvas: number;
					public constructor(param0: globalAndroid.content.Context);
					public constructor(param0: globalAndroid.content.Context, param1: globalAndroid.util.AttributeSet);
					public setSrc(param0: string): void;
					public onDraw(param0: globalAndroid.graphics.Canvas): void;
					public onSizeChanged(param0: number, param1: number, param2: number, param3: number): void;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TextDecoder {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TextDecoder>;
					public constructor(param0: string);
					public constructor();
					public getEncoding(): string;
					public decode(param0: native.Array<number>): string;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TextEncoder {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TextEncoder>;
					public constructor(param0: string);
					public constructor();
					public getEncoding(): string;
					public encode(param0: string): native.Array<number>;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class TextMetrics {
					public static class: java.lang.Class<com.github.triniwiz.canvas.TextMetrics>;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class Utils {
					public static class: java.lang.Class<com.github.triniwiz.canvas.Utils>;
					public constructor();
					public static isEmulator(): boolean;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class WebGL2RenderingContext extends com.github.triniwiz.canvas.WebGLRenderingContext {
					public static class: java.lang.Class<com.github.triniwiz.canvas.WebGL2RenderingContext>;
					public READ_BUFFER: number;
					public UNPACK_ROW_LENGTH: number;
					public UNPACK_SKIP_ROWS: number;
					public UNPACK_SKIP_PIXELS: number;
					public PACK_ROW_LENGTH: number;
					public PACK_SKIP_ROWS: number;
					public PACK_SKIP_PIXELS: number;
					public TEXTURE_BINDING_3D: number;
					public UNPACK_SKIP_IMAGES: number;
					public UNPACK_IMAGE_HEIGHT: number;
					public MAX_3D_TEXTURE_SIZE: number;
					public MAX_ELEMENTS_VERTICES: number;
					public MAX_ELEMENTS_INDICES: number;
					public MAX_TEXTURE_LOD_BIAS: number;
					public MAX_FRAGMENT_UNIFORM_COMPONENTS: number;
					public MAX_VERTEX_UNIFORM_COMPONENTS: number;
					public MAX_ARRAY_TEXTURE_LAYERS: number;
					public MIN_PROGRAM_TEXEL_OFFSET: number;
					public MAX_PROGRAM_TEXEL_OFFSET: number;
					public MAX_VARYING_COMPONENTS: number;
					public FRAGMENT_SHADER_DERIVATIVE_HINT: number;
					public RASTERIZER_DISCARD: number;
					public VERTEX_ARRAY_BINDING: number;
					public MAX_VERTEX_OUTPUT_COMPONENTS: number;
					public MAX_FRAGMENT_INPUT_COMPONENTS: number;
					public MAX_SERVER_WAIT_TIMEOUT: number;
					public MAX_ELEMENT_INDEX: number;
					public RED: number;
					public RGB8: number;
					public RGBA8: number;
					public RGB10_A2: number;
					public TEXTURE_3D: number;
					public TEXTURE_WRAP_R: number;
					public TEXTURE_MIN_LOD: number;
					public TEXTURE_MAX_LOD: number;
					public TEXTURE_BASE_LEVEL: number;
					public TEXTURE_MAX_LEVEL: number;
					public TEXTURE_COMPARE_MODE: number;
					public TEXTURE_COMPARE_FUNC: number;
					public SRGB: number;
					public SRGB8: number;
					public SRGB8_ALPHA8: number;
					public COMPARE_REF_TO_TEXTURE: number;
					public RGBA32F: number;
					public RGB32F: number;
					public RGBA16F: number;
					public RGB16F: number;
					public TEXTURE_2D_ARRAY: number;
					public TEXTURE_BINDING_2D_ARRAY: number;
					public R11F_G11F_B10F: number;
					public RGB9_E5: number;
					public RGBA32UI: number;
					public RGB32UI: number;
					public RGBA16UI: number;
					public RGB16UI: number;
					public RGBA8UI: number;
					public RGB8UI: number;
					public RGBA32I: number;
					public RGB32I: number;
					public RGBA16I: number;
					public RGB16I: number;
					public RGBA8I: number;
					public RGB8I: number;
					public RED_INTEGER: number;
					public RGB_INTEGER: number;
					public RGBA_INTEGER: number;
					public R8: number;
					public RG8: number;
					public R16F: number;
					public R32F: number;
					public RG16F: number;
					public RG32F: number;
					public R8I: number;
					public R8UI: number;
					public R16I: number;
					public R16UI: number;
					public R32I: number;
					public R32UI: number;
					public RG8I: number;
					public RG8UI: number;
					public RG16I: number;
					public RG16UI: number;
					public RG32I: number;
					public RG32UI: number;
					public R8_SNORM: number;
					public RG8_SNORM: number;
					public RGB8_SNORM: number;
					public RGBA8_SNORM: number;
					public RGB10_A2UI: number;
					public TEXTURE_IMMUTABLE_FORMAT: number;
					public TEXTURE_IMMUTABLE_LEVELS: number;
					public UNSIGNED_INT_2_10_10_10_REV: number;
					public UNSIGNED_INT_10F_11F_11F_REV: number;
					public UNSIGNED_INT_5_9_9_9_REV: number;
					public FLOAT_32_UNSIGNED_INT_24_8_REV: number;
					public UNSIGNED_INT_24_8: number;
					public HALF_FLOAT: number;
					public RG: number;
					public RG_INTEGER: number;
					public INT_2_10_10_10_REV: number;
					public QUERY_RESULT_AVAILABLE: number;
					public QUERY_RESULT: number;
					public CURRENT_QUERY: number;
					public ANY_SAMPLES_PASSED: number;
					public ANY_SAMPLES_PASSED_CONSERVATIVE: number;
					public MAX_DRAW_BUFFERS: number;
					public DRAW_BUFFER0: number;
					public DRAW_BUFFER1: number;
					public DRAW_BUFFER2: number;
					public DRAW_BUFFER3: number;
					public DRAW_BUFFER4: number;
					public DRAW_BUFFER5: number;
					public DRAW_BUFFER6: number;
					public DRAW_BUFFER7: number;
					public DRAW_BUFFER8: number;
					public DRAW_BUFFER9: number;
					public DRAW_BUFFER10: number;
					public DRAW_BUFFER11: number;
					public DRAW_BUFFER12: number;
					public DRAW_BUFFER13: number;
					public DRAW_BUFFER14: number;
					public DRAW_BUFFER15: number;
					public MAX_COLOR_ATTACHMENTS: number;
					public COLOR_ATTACHMENT1: number;
					public COLOR_ATTACHMENT2: number;
					public COLOR_ATTACHMENT3: number;
					public COLOR_ATTACHMENT4: number;
					public COLOR_ATTACHMENT5: number;
					public COLOR_ATTACHMENT6: number;
					public COLOR_ATTACHMENT7: number;
					public COLOR_ATTACHMENT8: number;
					public COLOR_ATTACHMENT9: number;
					public COLOR_ATTACHMENT10: number;
					public COLOR_ATTACHMENT11: number;
					public COLOR_ATTACHMENT12: number;
					public COLOR_ATTACHMENT13: number;
					public COLOR_ATTACHMENT14: number;
					public COLOR_ATTACHMENT15: number;
					public SAMPLER_3D: number;
					public SAMPLER_2D_SHADOW: number;
					public SAMPLER_2D_ARRAY: number;
					public SAMPLER_2D_ARRAY_SHADOW: number;
					public SAMPLER_CUBE_SHADOW: number;
					public INT_SAMPLER_2D: number;
					public INT_SAMPLER_3D: number;
					public INT_SAMPLER_CUBE: number;
					public INT_SAMPLER_2D_ARRAY: number;
					public UNSIGNED_INT_SAMPLER_2D: number;
					public UNSIGNED_INT_SAMPLER_3D: number;
					public UNSIGNED_INT_SAMPLER_CUBE: number;
					public UNSIGNED_INT_SAMPLER_2D_ARRAY: number;
					public MAX_SAMPLES: number;
					public SAMPLER_BINDING: number;
					public PIXEL_PACK_BUFFER: number;
					public PIXEL_UNPACK_BUFFER: number;
					public PIXEL_PACK_BUFFER_BINDING: number;
					public PIXEL_UNPACK_BUFFER_BINDING: number;
					public COPY_READ_BUFFER: number;
					public COPY_WRITE_BUFFER: number;
					public COPY_READ_BUFFER_BINDING: number;
					public COPY_WRITE_BUFFER_BINDING: number;
					public FLOAT_MAT2x3: number;
					public FLOAT_MAT2x4: number;
					public FLOAT_MAT3x2: number;
					public FLOAT_MAT3x4: number;
					public FLOAT_MAT4x2: number;
					public FLOAT_MAT4x3: number;
					public UNSIGNED_INT_VEC2: number;
					public UNSIGNED_INT_VEC3: number;
					public UNSIGNED_INT_VEC4: number;
					public UNSIGNED_NORMALIZED: number;
					public SIGNED_NORMALIZED: number;
					public VERTEX_ATTRIB_ARRAY_INTEGER: number;
					public VERTEX_ATTRIB_ARRAY_DIVISOR: number;
					public TRANSFORM_FEEDBACK_BUFFER_MODE: number;
					public MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: number;
					public TRANSFORM_FEEDBACK_VARYINGS: number;
					public TRANSFORM_FEEDBACK_BUFFER_START: number;
					public TRANSFORM_FEEDBACK_BUFFER_SIZE: number;
					public TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: number;
					public MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: number;
					public MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: number;
					public INTERLEAVED_ATTRIBS: number;
					public SEPARATE_ATTRIBS: number;
					public TRANSFORM_FEEDBACK_BUFFER: number;
					public TRANSFORM_FEEDBACK_BUFFER_BINDING: number;
					public TRANSFORM_FEEDBACK: number;
					public TRANSFORM_FEEDBACK_PAUSED: number;
					public TRANSFORM_FEEDBACK_ACTIVE: number;
					public TRANSFORM_FEEDBACK_BINDING: number;
					public FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: number;
					public FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: number;
					public FRAMEBUFFER_ATTACHMENT_RED_SIZE: number;
					public FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: number;
					public FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: number;
					public FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: number;
					public FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: number;
					public FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: number;
					public FRAMEBUFFER_DEFAULT: number;
					public DEPTH_STENCIL_ATTACHMENT: number;
					public DEPTH_STENCIL: number;
					public DEPTH24_STENCIL8: number;
					public DRAW_FRAMEBUFFER_BINDING: number;
					public READ_FRAMEBUFFER: number;
					public DRAW_FRAMEBUFFER: number;
					public READ_FRAMEBUFFER_BINDING: number;
					public RENDERBUFFER_SAMPLES: number;
					public FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: number;
					public FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: number;
					public UNIFORM_BUFFER: number;
					public UNIFORM_BUFFER_BINDING: number;
					public UNIFORM_BUFFER_START: number;
					public UNIFORM_BUFFER_SIZE: number;
					public MAX_VERTEX_UNIFORM_BLOCKS: number;
					public MAX_FRAGMENT_UNIFORM_BLOCKS: number;
					public MAX_COMBINED_UNIFORM_BLOCKS: number;
					public MAX_UNIFORM_BUFFER_BINDINGS: number;
					public MAX_UNIFORM_BLOCK_SIZE: number;
					public MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: number;
					public MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: number;
					public UNIFORM_BUFFER_OFFSET_ALIGNMENT: number;
					public ACTIVE_UNIFORM_BLOCKS: number;
					public UNIFORM_TYPE: number;
					public UNIFORM_SIZE: number;
					public UNIFORM_BLOCK_INDEX: number;
					public UNIFORM_OFFSET: number;
					public UNIFORM_ARRAY_STRIDE: number;
					public UNIFORM_MATRIX_STRIDE: number;
					public UNIFORM_IS_ROW_MAJOR: number;
					public UNIFORM_BLOCK_BINDING: number;
					public UNIFORM_BLOCK_DATA_SIZE: number;
					public UNIFORM_BLOCK_ACTIVE_UNIFORMS: number;
					public UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: number;
					public UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: number;
					public UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: number;
					public OBJECT_TYPE: number;
					public SYNC_CONDITION: number;
					public SYNC_STATUS: number;
					public SYNC_FLAGS: number;
					public SYNC_FENCE: number;
					public SYNC_GPU_COMMANDS_COMPLETE: number;
					public UNSIGNALED: number;
					public SIGNALED: number;
					public ALREADY_SIGNALED: number;
					public TIMEOUT_EXPIRED: number;
					public CONDITION_SATISFIED: number;
					public WAIT_FAILED: number;
					public SYNC_FLUSH_COMMANDS_BIT: number;
					public COLOR: number;
					public DEPTH: number;
					public STENCIL: number;
					public MIN: number;
					public MAX: number;
					public DEPTH_COMPONENT24: number;
					public STREAM_READ: number;
					public STREAM_COPY: number;
					public STATIC_READ: number;
					public STATIC_COPY: number;
					public DYNAMIC_READ: number;
					public DYNAMIC_COPY: number;
					public DEPTH_COMPONENT32F: number;
					public DEPTH32F_STENCIL8: number;
					public INVALID_INDEX: number;
					public TIMEOUT_IGNORED: number;
					public MAX_CLIENT_WAIT_TIMEOUT_WEBGL: number;
					public beginQuery(param0: number, param1: number): void;
					public endTransformFeedback(): void;
					public getUniformBlockIndex(param0: number, param1: string): number;
					public uniformMatrix2x3fv(param0: number, param1: boolean, param2: native.Array<number>): void;
					public bindVertexArray(param0: number): void;
					public createVertexArray(): number;
					public vertexAttribI4uiv(param0: number, param1: native.Array<number>): void;
					public getSyncParameter(param0: number, param1: number): any;
					public isVertexArray(param0: number): boolean;
					public getIndexedParameter(param0: number, param1: number): any;
					public getActiveUniformBlockParameter(param0: number, param1: number, param2: number): any;
					public uniform2uiv(param0: number, param1: native.Array<number>): void;
					public uniformMatrix3x4fv(param0: number, param1: boolean, param2: native.Array<number>): void;
					public constructor(param0: com.github.triniwiz.canvas.CanvasView, param1: java.util.Map<string,any>);
					public deleteSync(param0: number): void;
					public deleteVertexArray(param0: number): void;
					public copyBufferSubData(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public uniform1uiv(param0: number, param1: native.Array<number>): void;
					public bindSampler(param0: number, param1: number): void;
					public blitFramebuffer(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number): void;
					public getFragDataLocation(param0: number, param1: string): number;
					public texSubImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: number): void;
					public bindTransformFeedback(param0: number, param1: number): void;
					public transformFeedbackVaryings(param0: number, param1: native.Array<string>, param2: number): void;
					public deleteTransformFeedback(param0: number): void;
					public texImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number): void;
					public invalidateFramebuffer(param0: number, param1: native.Array<number>): void;
					public readBuffer(param0: number): void;
					public uniformMatrix3x2fv(param0: number, param1: boolean, param2: native.Array<number>): void;
					public constructor(param0: com.github.triniwiz.canvas.CanvasView);
					public getQueryParameter(param0: number, param1: number): any;
					public texImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: native.Array<number>): void;
					public texStorage2D(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public drawBuffers(param0: native.Array<number>): void;
					public vertexAttribI4ui(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public drawRangeElements(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number): void;
					public isQuery(param0: number): boolean;
					public uniformMatrix4x3fv(param0: number, param1: boolean, param2: native.Array<number>): void;
					public getUniformIndices(param0: number, param1: native.Array<string>): native.Array<number>;
					public getBufferSubData(param0: number, param1: number, param2: native.Array<number>, param3: number, param4: number): void;
					public texSubImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: globalAndroid.graphics.Bitmap): void;
					public deleteSampler(param0: number): void;
					public uniformMatrix4x2fv(param0: number, param1: boolean, param2: native.Array<number>): void;
					public pauseTransformFeedback(): void;
					public texSubImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: com.github.triniwiz.canvas.ImageAsset): void;
					public getActiveUniformBlockName(param0: number, param1: number): string;
					public clearBufferiv(param0: number, param1: number, param2: native.Array<number>): void;
					public uniform2ui(param0: number, param1: number, param2: number): void;
					public getSamplerParameter(param0: number, param1: number): any;
					public resumeTransformFeedback(): void;
					public drawArraysInstanced(param0: number, param1: number, param2: number, param3: number): void;
					public texImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: com.github.triniwiz.canvas.ImageAsset): void;
					public vertexAttribI4iv(param0: number, param1: native.Array<number>): void;
					public clearBufferuiv(param0: number, param1: number, param2: native.Array<number>): void;
					public framebufferTextureLayer(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public texSubImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: native.Array<number>, param11: number): void;
					public texImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: globalAndroid.graphics.Bitmap): void;
					public compressedTexSubImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: native.Array<number>, param10: number, param11: number): void;
					public endQuery(param0: number): void;
					public texSubImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: native.Array<number>): void;
					public samplerParameteri(param0: number, param1: number, param2: number): void;
					public uniformBlockBinding(param0: number, param1: number, param2: number): void;
					public isSync(param0: number): boolean;
					public getActiveUniforms(param0: number, param1: native.Array<number>, param2: number): any;
					public isSampler(param0: number): boolean;
					public uniform4uiv(param0: number, param1: native.Array<number>): void;
					public createSampler(): number;
					public createTransformFeedback(): number;
					public texStorage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number): void;
					public samplerParameterf(param0: number, param1: number, param2: number): void;
					public vertexAttribDivisor(param0: number, param1: number): void;
					public clearBufferfi(param0: number, param1: number, param2: number, param3: number): void;
					public compressedTexSubImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number, param9: number, param10: number): void;
					public getInternalformatParameter(param0: number, param1: number, param2: number): any;
					public invalidateSubFramebuffer(param0: number, param1: native.Array<number>, param2: number, param3: number, param4: number, param5: number): void;
					public renderbufferStorageMultisample(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public uniform1ui(param0: number, param1: number): void;
					public deleteQuery(param0: number): void;
					public getTransformFeedbackVarying(param0: number, param1: number): any;
					public vertexAttribI4i(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public copyTexSubImage3D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: number): void;
					public uniform4ui(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public bindBufferRange(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public isTransformFeedback(param0: number): boolean;
					public beginTransformFeedback(param0: number): void;
					public createQuery(): number;
					public fenceSync(param0: number, param1: number): void;
					public uniform3uiv(param0: number, param1: native.Array<number>): void;
					public getQuery(param0: number, param1: number): any;
					public bindBufferBase(param0: number, param1: number, param2: number): void;
					public clientWaitSync(param0: number, param1: number, param2: number): number;
					public drawElementsInstanced(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public clearBufferfv(param0: number, param1: number, param2: native.Array<number>): void;
					public uniform3ui(param0: number, param1: number, param2: number, param3: number): void;
					public uniformMatrix2x4fv(param0: number, param1: boolean, param2: native.Array<number>): void;
				}
				export module WebGL2RenderingContext {
					export class ReturnType {
						public static class: java.lang.Class<com.github.triniwiz.canvas.WebGL2RenderingContext.ReturnType>;
						public static EnumType: com.github.triniwiz.canvas.WebGL2RenderingContext.ReturnType;
						public static UnsignedIntType: com.github.triniwiz.canvas.WebGL2RenderingContext.ReturnType;
						public static IntType: com.github.triniwiz.canvas.WebGL2RenderingContext.ReturnType;
						public static BoolType: com.github.triniwiz.canvas.WebGL2RenderingContext.ReturnType;
						public static valueOf(param0: string): com.github.triniwiz.canvas.WebGL2RenderingContext.ReturnType;
						public static values(): native.Array<com.github.triniwiz.canvas.WebGL2RenderingContext.ReturnType>;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class WebGLActiveInfo {
					public static class: java.lang.Class<com.github.triniwiz.canvas.WebGLActiveInfo>;
					public getType(): number;
					public constructor();
					public getName(): string;
					public constructor(param0: string, param1: number, param2: number);
					public getSize(): number;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class WebGLRenderingContext extends com.github.triniwiz.canvas.CanvasRenderingContext {
					public static class: java.lang.Class<com.github.triniwiz.canvas.WebGLRenderingContext>;
					public DEPTH_BUFFER_BIT: number;
					public COLOR_BUFFER_BIT: number;
					public STENCIL_BUFFER_BIT: number;
					public POINTS: number;
					public LINES: number;
					public LINE_LOOP: number;
					public LINE_STRIP: number;
					public TRIANGLES: number;
					public TRIANGLE_STRIP: number;
					public TRIANGLE_FAN: number;
					public ONE: number;
					public ZERO: number;
					public SRC_COLOR: number;
					public ONE_MINUS_SRC_COLOR: number;
					public SRC_ALPHA: number;
					public ONE_MINUS_SRC_ALPHA: number;
					public DST_ALPHA: number;
					public ONE_MINUS_DST_ALPHA: number;
					public DST_COLOR: number;
					public ONE_MINUS_DST_COLOR: number;
					public SRC_ALPHA_SATURATE: number;
					public CONSTANT_COLOR: number;
					public ONE_MINUS_CONSTANT_COLOR: number;
					public CONSTANT_ALPHA: number;
					public ONE_MINUS_CONSTANT_ALPHA: number;
					public FUNC_ADD: number;
					public FUNC_SUBTRACT: number;
					public FUNC_REVERSE_SUBTRACT: number;
					public BLEND_EQUATION: number;
					public BLEND_EQUATION_RGB: number;
					public BLEND_EQUATION_ALPHA: number;
					public BLEND_DST_RGB: number;
					public BLEND_SRC_RGB: number;
					public BLEND_DST_ALPHA: number;
					public BLEND_SRC_ALPHA: number;
					public BLEND_COLOR: number;
					public ARRAY_BUFFER_BINDING: number;
					public ELEMENT_ARRAY_BUFFER_BINDING: number;
					public LINE_WIDTH: number;
					public ALIASED_POINT_SIZE_RANGE: number;
					public ALIASED_LINE_WIDTH_RANGE: number;
					public CULL_FACE_MODE: number;
					public FRONT_FACE: number;
					public DEPTH_RANGE: number;
					public DEPTH_WRITEMASK: number;
					public DEPTH_CLEAR_VALUE: number;
					public DEPTH_FUNC: number;
					public STENCIL_CLEAR_VALUE: number;
					public STENCIL_FUNC: number;
					public STENCIL_FAIL: number;
					public STENCIL_PASS_DEPTH_FAIL: number;
					public STENCIL_PASS_DEPTH_PASS: number;
					public STENCIL_REF: number;
					public STENCIL_VALUE_MASK: number;
					public STENCIL_WRITEMASK: number;
					public STENCIL_BACK_FUNC: number;
					public STENCIL_BACK_FAIL: number;
					public STENCIL_BACK_PASS_DEPTH_FAIL: number;
					public STENCIL_BACK_PASS_DEPTH_PASS: number;
					public STENCIL_BACK_REF: number;
					public STENCIL_BACK_VALUE_MASK: number;
					public STENCIL_BACK_WRITEMASK: number;
					public VIEWPORT: number;
					public SCISSOR_BOX: number;
					public COLOR_CLEAR_VALUE: number;
					public COLOR_WRITEMASK: number;
					public UNPACK_ALIGNMENT: number;
					public PACK_ALIGNMENT: number;
					public MAX_TEXTURE_SIZE: number;
					public MAX_VIEWPORT_DIMS: number;
					public SUBPIXEL_BITS: number;
					public RED_BITS: number;
					public GREEN_BITS: number;
					public BLUE_BITS: number;
					public ALPHA_BITS: number;
					public DEPTH_BITS: number;
					public STENCIL_BITS: number;
					public POLYGON_OFFSET_UNITS: number;
					public POLYGON_OFFSET_FACTOR: number;
					public TEXTURE_BINDING_2D: number;
					public SAMPLE_BUFFERS: number;
					public SAMPLES: number;
					public SAMPLE_COVERAGE_VALUE: number;
					public SAMPLE_COVERAGE_INVERT: number;
					public COMPRESSED_TEXTURE_FORMATS: number;
					public VENDOR: number;
					public RENDERER: number;
					public VERSION: number;
					public IMPLEMENTATION_COLOR_READ_TYPE: number;
					public IMPLEMENTATION_COLOR_READ_FORMAT: number;
					public BROWSER_DEFAULT_WEBGL: number;
					public STATIC_DRAW: number;
					public STREAM_DRAW: number;
					public DYNAMIC_DRAW: number;
					public ARRAY_BUFFER: number;
					public ELEMENT_ARRAY_BUFFER: number;
					public BUFFER_SIZE: number;
					public BUFFER_USAGE: number;
					public CURRENT_VERTEX_ATTRIB: number;
					public VERTEX_ATTRIB_ARRAY_ENABLED: number;
					public VERTEX_ATTRIB_ARRAY_SIZE: number;
					public VERTEX_ATTRIB_ARRAY_STRIDE: number;
					public VERTEX_ATTRIB_ARRAY_TYPE: number;
					public VERTEX_ATTRIB_ARRAY_NORMALIZED: number;
					public VERTEX_ATTRIB_ARRAY_POINTER: number;
					public VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: number;
					public CULL_FACE: number;
					public FRONT: number;
					public BACK: number;
					public FRONT_AND_BACK: number;
					public BLEND: number;
					public DEPTH_TEST: number;
					public DITHER: number;
					public POLYGON_OFFSET_FILL: number;
					public SAMPLE_ALPHA_TO_COVERAGE: number;
					public SAMPLE_COVERAGE: number;
					public SCISSOR_TEST: number;
					public STENCIL_TEST: number;
					public NO_ERROR: number;
					public INVALID_ENUM: number;
					public INVALID_VALUE: number;
					public INVALID_OPERATION: number;
					public INVALID_FRAMEBUFFER_OPERATION: number;
					public OUT_OF_MEMORY: number;
					public CONTEXT_LOST_WEBGL: number;
					public CW: number;
					public CCW: number;
					public DONT_CARE: number;
					public FASTEST: number;
					public NICEST: number;
					public GENERATE_MIPMAP_HINT: number;
					public BYTE: number;
					public UNSIGNED_BYTE: number;
					public UNSIGNED_SHORT: number;
					public SHORT: number;
					public UNSIGNED_INT: number;
					public INT: number;
					public FLOAT: number;
					public DEPTH_COMPONENT: number;
					public ALPHA: number;
					public RGB: number;
					public RGBA: number;
					public LUMINANCE: number;
					public LUMINANCE_ALPHA: number;
					public UNSIGNED_SHORT_4_4_4_4: number;
					public UNSIGNED_SHORT_5_5_5_1: number;
					public UNSIGNED_SHORT_5_6_5: number;
					public FRAGMENT_SHADER: number;
					public VERTEX_SHADER: number;
					public COMPILE_STATUS: number;
					public DELETE_STATUS: number;
					public LINK_STATUS: number;
					public VALIDATE_STATUS: number;
					public ATTACHED_SHADERS: number;
					public ACTIVE_ATTRIBUTES: number;
					public ACTIVE_UNIFORMS: number;
					public MAX_VERTEX_UNIFORM_VECTORS: number;
					public MAX_VARYING_VECTORS: number;
					public MAX_COMBINED_TEXTURE_IMAGE_UNITS: number;
					public MAX_VERTEX_TEXTURE_IMAGE_UNITS: number;
					public MAX_TEXTURE_IMAGE_UNITS: number;
					public MAX_VERTEX_ATTRIBS: number;
					public MAX_FRAGMENT_UNIFORM_VECTORS: number;
					public SHADER_TYPE: number;
					public SHADING_LANGUAGE_VERSION: number;
					public CURRENT_PROGRAM: number;
					public NEVER: number;
					public LESS: number;
					public EQUAL: number;
					public LEQUAL: number;
					public GREATER: number;
					public NOTEQUAL: number;
					public GEQUAL: number;
					public ALWAYS: number;
					public KEEP: number;
					public REPLACE: number;
					public INCR: number;
					public DECR: number;
					public INVERT: number;
					public INCR_WRAP: number;
					public DECR_WRAP: number;
					public NEAREST: number;
					public LINEAR: number;
					public NEAREST_MIPMAP_NEAREST: number;
					public LINEAR_MIPMAP_NEAREST: number;
					public NEAREST_MIPMAP_LINEAR: number;
					public LINEAR_MIPMAP_LINEAR: number;
					public TEXTURE_MAG_FILTER: number;
					public TEXTURE_MIN_FILTER: number;
					public TEXTURE_WRAP_S: number;
					public TEXTURE_WRAP_T: number;
					public TEXTURE_2D: number;
					public TEXTURE: number;
					public TEXTURE_CUBE_MAP: number;
					public TEXTURE_BINDING_CUBE_MAP: number;
					public TEXTURE_CUBE_MAP_POSITIVE_X: number;
					public TEXTURE_CUBE_MAP_NEGATIVE_X: number;
					public TEXTURE_CUBE_MAP_POSITIVE_Y: number;
					public TEXTURE_CUBE_MAP_NEGATIVE_Y: number;
					public TEXTURE_CUBE_MAP_POSITIVE_Z: number;
					public TEXTURE_CUBE_MAP_NEGATIVE_Z: number;
					public MAX_CUBE_MAP_TEXTURE_SIZE: number;
					public TEXTURE0: number;
					public TEXTURE1: number;
					public TEXTURE2: number;
					public TEXTURE3: number;
					public TEXTURE4: number;
					public TEXTURE5: number;
					public TEXTURE6: number;
					public TEXTURE7: number;
					public TEXTURE8: number;
					public TEXTURE9: number;
					public TEXTURE10: number;
					public TEXTURE11: number;
					public TEXTURE12: number;
					public TEXTURE13: number;
					public TEXTURE14: number;
					public TEXTURE15: number;
					public TEXTURE16: number;
					public TEXTURE17: number;
					public TEXTURE18: number;
					public TEXTURE19: number;
					public TEXTURE20: number;
					public TEXTURE21: number;
					public TEXTURE22: number;
					public TEXTURE23: number;
					public TEXTURE24: number;
					public TEXTURE25: number;
					public TEXTURE26: number;
					public TEXTURE27: number;
					public TEXTURE28: number;
					public TEXTURE29: number;
					public TEXTURE30: number;
					public TEXTURE31: number;
					public ACTIVE_TEXTURE: number;
					public REPEAT: number;
					public CLAMP_TO_EDGE: number;
					public MIRRORED_REPEAT: number;
					public FLOAT_VEC2: number;
					public FLOAT_VEC3: number;
					public FLOAT_VEC4: number;
					public INT_VEC2: number;
					public INT_VEC3: number;
					public INT_VEC4: number;
					public BOOL: number;
					public BOOL_VEC2: number;
					public BOOL_VEC3: number;
					public BOOL_VEC4: number;
					public FLOAT_MAT2: number;
					public FLOAT_MAT3: number;
					public FLOAT_MAT4: number;
					public SAMPLER_2D: number;
					public SAMPLER_CUBE: number;
					public LOW_FLOAT: number;
					public MEDIUM_FLOAT: number;
					public HIGH_FLOAT: number;
					public LOW_INT: number;
					public MEDIUM_INT: number;
					public HIGH_INT: number;
					public FRAMEBUFFER: number;
					public RENDERBUFFER: number;
					public RGBA4: number;
					public RGB565: number;
					public RGB5_A1: number;
					public DEPTH_COMPONENT16: number;
					public STENCIL_INDEX8: number;
					public DEPTH_STENCIL: number;
					public RENDERBUFFER_WIDTH: number;
					public RENDERBUFFER_HEIGHT: number;
					public RENDERBUFFER_INTERNAL_FORMAT: number;
					public RENDERBUFFER_RED_SIZE: number;
					public RENDERBUFFER_GREEN_SIZE: number;
					public RENDERBUFFER_BLUE_SIZE: number;
					public RENDERBUFFER_ALPHA_SIZE: number;
					public RENDERBUFFER_DEPTH_SIZE: number;
					public RENDERBUFFER_STENCIL_SIZE: number;
					public FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: number;
					public FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: number;
					public FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: number;
					public FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: number;
					public COLOR_ATTACHMENT0: number;
					public DEPTH_ATTACHMENT: number;
					public STENCIL_ATTACHMENT: number;
					public DEPTH_STENCIL_ATTACHMENT: number;
					public NONE: number;
					public FRAMEBUFFER_COMPLETE: number;
					public FRAMEBUFFER_INCOMPLETE_ATTACHMENT: number;
					public FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: number;
					public FRAMEBUFFER_INCOMPLETE_DIMENSIONS: number;
					public FRAMEBUFFER_UNSUPPORTED: number;
					public FRAMEBUFFER_BINDING: number;
					public RENDERBUFFER_BINDING: number;
					public MAX_RENDERBUFFER_SIZE: number;
					public UNPACK_COLORSPACE_CONVERSION_WEBGL: number;
					public UNPACK_FLIP_Y_WEBGL: number;
					public UNPACK_PREMULTIPLY_ALPHA_WEBGL: number;
					public framebufferTexture2D(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public uniform1iv(param0: number, param1: native.Array<number>): void;
					public uniform4iv(param0: number, param1: native.Array<number>): void;
					public bindFramebuffer(param0: number, param1: number): void;
					public isFramebuffer(param0: number): boolean;
					public bindAttribLocation(param0: number, param1: number, param2: string): void;
					public uniform2f(param0: number, param1: number, param2: number): void;
					public scissor(param0: number, param1: number, param2: number, param3: number): void;
					public vertexAttrib3fv(param0: number, param1: native.Array<number>): void;
					public getProgramParameter(param0: number, param1: number): any;
					public blendEquationSeparate(param0: number, param1: number): void;
					public stencilOp(param0: number, param1: number, param2: number): void;
					public uniform2i(param0: number, param1: number, param2: number): void;
					public deleteProgram(param0: number): void;
					public readPixels(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: native.Array<number>): void;
					public stencilOpSeparate(param0: number, param1: number, param2: number, param3: number): void;
					public uniform3f(param0: number, param1: number, param2: number, param3: number): void;
					public constructor(param0: com.github.triniwiz.canvas.CanvasView);
					public clearStencil(param0: number): void;
					public commit(): void;
					public depthMask(param0: boolean): void;
					public getActiveUniform(param0: number, param1: number): com.github.triniwiz.canvas.WebGLActiveInfo;
					public createTexture(): number;
					public colorMask(param0: boolean, param1: boolean, param2: boolean, param3: boolean): void;
					public texParameterf(param0: number, param1: number, param2: number): void;
					public checkFramebufferStatus(param0: number): number;
					public flush(): void;
					public isProgram(param0: number): boolean;
					public createShader(param0: number): number;
					public getActiveAttrib(param0: number, param1: number): com.github.triniwiz.canvas.WebGLActiveInfo;
					public getDrawingBufferHeight(): number;
					public getUniform(param0: number, param1: number): any;
					public texSubImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: com.github.triniwiz.canvas.ImageAsset): void;
					public compileShader(param0: number): void;
					public isEnabled(param0: number): boolean;
					public uniform4f(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public viewport(param0: number, param1: number, param2: number, param3: number): void;
					public texSubImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: com.github.triniwiz.canvas.CanvasView): void;
					public blendFuncSeparate(param0: number, param1: number, param2: number, param3: number): void;
					public isShader(param0: number): boolean;
					public getShaderSource(param0: number): string;
					public uniform3i(param0: number, param1: number, param2: number, param3: number): void;
					public bindRenderbuffer(param0: number, param1: number): void;
					public copyTexSubImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number): void;
					public getBufferParameter(param0: number, param1: number): number;
					public getVertexAttribOffset(param0: number, param1: number): number;
					public depthFunc(param0: number): void;
					public copyTexImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number): void;
					public createBuffer(): number;
					public vertexAttrib1fv(param0: number, param1: native.Array<number>): void;
					public vertexAttrib4fv(param0: number, param1: native.Array<number>): void;
					public blendEquation(param0: number): void;
					public bindBuffer(param0: number, param1: any): void;
					public depthRange(param0: number, param1: number): void;
					public texSubImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: native.Array<number>): void;
					public drawArrays(param0: number, param1: number, param2: number): void;
					public pixelStorei(param0: number, param1: any): void;
					public getProgramInfoLog(param0: number): string;
					public attachShader(param0: number, param1: number): void;
					public detachShader(param0: number, param1: number): void;
					public enable(param0: number): void;
					public texImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: com.github.triniwiz.canvas.CanvasView): void;
					public stencilFuncSeparate(param0: number, param1: number, param2: number, param3: number): void;
					public texParameteri(param0: number, param1: number, param2: number): void;
					public vertexAttrib4f(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public disableVertexAttribArray(param0: number): void;
					public useProgram(param0: number): void;
					public getExtension(param0: string): any;
					public vertexAttrib2fv(param0: number, param1: native.Array<number>): void;
					public createFramebuffer(): number;
					public hint(param0: number, param1: number): void;
					public isRenderbuffer(param0: number): boolean;
					public deleteFramebuffer(param0: number): void;
					public activeTexture(param0: number): void;
					public vertexAttribPointer(param0: number, param1: number, param2: number, param3: boolean, param4: number, param5: number): void;
					public sampleCoverage(param0: number, param1: boolean): void;
					public isContextLost(): boolean;
					public vertexAttrib1f(param0: number, param1: number): void;
					public getFramebufferAttachmentParameter(param0: number, param1: number, param2: number): com.github.triniwiz.canvas.FramebufferAttachmentParameter;
					public getShaderInfoLog(param0: number): string;
					public isTexture(param0: number): boolean;
					public getSupportedExtensions(): native.Array<string>;
					public uniform3fv(param0: number, param1: native.Array<number>): void;
					public frontFace(param0: number): void;
					public getRenderbufferParameter(param0: number, param1: number): number;
					public constructor(param0: com.github.triniwiz.canvas.CanvasView, param1: java.util.Map<string,any>);
					public polygonOffset(param0: number, param1: number): void;
					public getAttribLocation(param0: number, param1: string): number;
					public generateMipmap(param0: number): void;
					public createProgram(): number;
					public blendColor(param0: number, param1: number, param2: number, param3: number): void;
					public cullFace(param0: number): void;
					public disable(param0: number): void;
					public getVertexAttrib(param0: number, param1: number): any;
					public blendFunc(param0: number, param1: number): void;
					public stencilMask(param0: number): void;
					public getCanvas(): com.github.triniwiz.canvas.CanvasView;
					public linkProgram(param0: number): void;
					public bufferData(param0: number, param1: native.Array<number>, param2: number): void;
					public getShaderPrecisionFormat(param0: number, param1: number): com.github.triniwiz.canvas.WebGLShaderPrecisionFormat;
					public stencilFunc(param0: number, param1: number, param2: number): void;
					public framebufferRenderbuffer(param0: number, param1: number, param2: number, param3: number): void;
					public uniform3iv(param0: number, param1: native.Array<number>): void;
					public compressedTexImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: native.Array<number>): void;
					public bindTexture(param0: number, param1: number): void;
					public uniformMatrix4fv(param0: number, param1: boolean, param2: native.Array<number>): void;
					public uniform2iv(param0: number, param1: native.Array<number>): void;
					public getDrawingBufferWidth(): number;
					public lineWidth(param0: number): void;
					public bufferSubData(param0: number, param1: number, param2: native.Array<number>): void;
					public deleteRenderbuffer(param0: number): void;
					public getParameter(param0: number): any;
					public shaderSource(param0: number, param1: string): void;
					public renderbufferStorage(param0: number, param1: number, param2: number, param3: number): void;
					public vertexAttrib3f(param0: number, param1: number, param2: number, param3: number): void;
					public deleteShader(param0: number): void;
					public uniformMatrix3fv(param0: number, param1: boolean, param2: native.Array<number>): void;
					public clearColor(param0: number, param1: number, param2: number, param3: number): void;
					public validateProgram(param0: number): void;
					public getAttachedShaders(param0: number): native.Array<number>;
					public isBuffer(param0: number): boolean;
					public uniform1fv(param0: number, param1: native.Array<number>): void;
					public bufferData(param0: number, param1: number, param2: number): void;
					public compressedTexSubImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: native.Array<number>): void;
					public drawElements(param0: number, param1: number, param2: number, param3: number): void;
					public getUniformLocation(param0: number, param1: string): number;
					public uniform4i(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					public uniformMatrix2fv(param0: number, param1: boolean, param2: native.Array<number>): void;
					public texImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: number, param7: number, param8: native.Array<number>): void;
					public vertexAttrib2f(param0: number, param1: number, param2: number): void;
					public getError(): number;
					public clear(param0: number): void;
					public uniform4fv(param0: number, param1: native.Array<number>): void;
					public stencilMaskSeparate(param0: number, param1: number): void;
					public getContextAttributes(): java.util.Map<string,any>;
					public uniform2fv(param0: number, param1: native.Array<number>): void;
					public texSubImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: number, param6: globalAndroid.graphics.Bitmap): void;
					public finish(): void;
					public texImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: globalAndroid.graphics.Bitmap): void;
					public uniform1f(param0: number, param1: number): void;
					public clearDepth(param0: number): void;
					public deleteTexture(param0: number): void;
					public bufferData(param0: number, param1: any, param2: number): void;
					public texImage2D(param0: number, param1: number, param2: number, param3: number, param4: number, param5: com.github.triniwiz.canvas.ImageAsset): void;
					public deleteBuffer(param0: number): void;
					public getShaderParameter(param0: number, param1: number): any;
					public uniform1i(param0: number, param1: number): void;
					public getTexParameter(param0: number, param1: number): number;
					public enableVertexAttribArray(param0: number): void;
					public bindBuffer(param0: number, param1: number): void;
					public createRenderbuffer(): number;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export class WebGLShaderPrecisionFormat {
					public static class: java.lang.Class<com.github.triniwiz.canvas.WebGLShaderPrecisionFormat>;
					public getRangeMin(): number;
					public constructor();
					public constructor(param0: number, param1: number, param2: number);
					public getRangeMax(): number;
					public getPrecision(): number;
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class ANGLE_instanced_arrays {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.ANGLE_instanced_arrays>;
						public VERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE: number;
						public constructor();
						public vertexAttribDivisorANGLE(param0: number, param1: number): void;
						public drawArraysInstancedANGLE(param0: number, param1: number, param2: number, param3: number): void;
						public drawElementsInstancedANGLE(param0: number, param1: number, param2: number, param3: number, param4: number): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class EXT_blend_minmax {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.EXT_blend_minmax>;
						public MIN_EXT: number;
						public MAX_EXT: number;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class EXT_color_buffer_float {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.EXT_color_buffer_float>;
						public R16F: number;
						public RG16F: number;
						public RGB16F: number;
						public R32F: number;
						public RG32F: number;
						public RGBA32F: number;
						public R11F_G11F_B10F: number;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class EXT_color_buffer_half_float {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.EXT_color_buffer_half_float>;
						public RGBA16F_EXT: number;
						public RGB16F_EXT: number;
						public FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT: number;
						public UNSIGNED_NORMALIZED_EXT: number;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class EXT_disjoint_timer_query {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.EXT_disjoint_timer_query>;
						public static QUERY_COUNTER_BITS_EXT: number;
						public static CURRENT_QUERY_EXT: number;
						public static QUERY_RESULT_EXT: number;
						public static QUERY_RESULT_AVAILABLE_EXT: number;
						public static TIME_ELAPSED_EXT: number;
						public static TIMESTAMP_EXT: number;
						public static GPU_DISJOINT_EXT: number;
						public deleteQueryEXT(param0: number): void;
						public queryCounterEXT(param0: number, param1: number): void;
						public getQueryEXT(param0: number, param1: number): number;
						public endQueryEXT(param0: number): void;
						public createQueryEXT(): number;
						public getQueryObjectEXT(param0: number, param1: number): any;
						public beginQueryEXT(param0: number, param1: number): void;
						public constructor(param0: com.github.triniwiz.canvas.CanvasView);
						public isQueryEXT(param0: number): boolean;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class EXT_sRGB {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.EXT_sRGB>;
						public SRGB_EXT: number;
						public SRGB_ALPHA_EXT: number;
						public SRGB8_ALPHA8_EXT: number;
						public FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT: number;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class EXT_shader_texture_lod {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.EXT_shader_texture_lod>;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class EXT_texture_filter_anisotropic {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.EXT_texture_filter_anisotropic>;
						public MAX_TEXTURE_MAX_ANISOTROPY_EXT: number;
						public TEXTURE_MAX_ANISOTROPY_EXT: number;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class OES_element_index_uint {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.OES_element_index_uint>;
						public UNSIGNED_INT: number;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class OES_fbo_render_mipmap {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.OES_fbo_render_mipmap>;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class OES_standard_derivatives {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.OES_standard_derivatives>;
						public FRAGMENT_SHADER_DERIVATIVE_HINT_OES: number;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class OES_texture_float {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.OES_texture_float>;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class OES_texture_float_linear {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.OES_texture_float_linear>;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class OES_texture_half_float {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.OES_texture_half_float>;
						public HALF_FLOAT_OES: number;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class OES_texture_half_float_linear {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.OES_texture_half_float_linear>;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class OES_vertex_array_object {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.OES_vertex_array_object>;
						public VERTEX_ARRAY_BINDING_OES: number;
						public createVertexArrayOES(): number;
						public deleteVertexArrayOES(param0: number): void;
						public bindVertexArrayOES(param0: number): void;
						public constructor(param0: com.github.triniwiz.canvas.CanvasView);
						public isVertexArrayOES(param0: number): boolean;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class WEBGL_color_buffer_float {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.WEBGL_color_buffer_float>;
						public RGBA32F_EXT: number;
						public RGB32F_EXT: number;
						public FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT: number;
						public UNSIGNED_NORMALIZED_EXT: number;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class WEBGL_compressed_texture_atc {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.WEBGL_compressed_texture_atc>;
						public COMPRESSED_RGB_ATC_WEBGL: number;
						public COMPRESSED_RGBA_ATC_EXPLICIT_ALPHA_WEBGL: number;
						public COMPRESSED_RGBA_ATC_INTERPOLATED_ALPHA_WEBGL: number;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class WEBGL_compressed_texture_etc {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.WEBGL_compressed_texture_etc>;
						public COMPRESSED_R11_EAC: number;
						public COMPRESSED_SIGNED_R11_EAC: number;
						public COMPRESSED_RG11_EAC: number;
						public COMPRESSED_SIGNED_RG11_EAC: number;
						public COMPRESSED_RGB8_ETC2: number;
						public COMPRESSED_RGBA8_ETC2_EAC: number;
						public COMPRESSED_SRGB8_ETC2: number;
						public COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: number;
						public COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: number;
						public COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: number;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class WEBGL_compressed_texture_etc1 {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.WEBGL_compressed_texture_etc1>;
						public COMPRESSED_RGB_ETC1_WEBGL: number;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class WEBGL_compressed_texture_pvrtc {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.WEBGL_compressed_texture_pvrtc>;
						public COMPRESSED_RGB_PVRTC_4BPPV1_IMG: number;
						public COMPRESSED_RGBA_PVRTC_4BPPV1_IMG: number;
						public COMPRESSED_RGB_PVRTC_2BPPV1_IMG: number;
						public COMPRESSED_RGBA_PVRTC_2BPPV1_IMG: number;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class WEBGL_compressed_texture_s3tc {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.WEBGL_compressed_texture_s3tc>;
						public COMPRESSED_RGB_S3TC_DXT1_EXT: number;
						public COMPRESSED_RGBA_S3TC_DXT1_EXT: number;
						public COMPRESSED_RGBA_S3TC_DXT3_EXT: number;
						public COMPRESSED_RGBA_S3TC_DXT5_EXT: number;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class WEBGL_compressed_texture_s3tc_srgb {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.WEBGL_compressed_texture_s3tc_srgb>;
						public COMPRESSED_SRGB_S3TC_DXT1_EXT: number;
						public COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT: number;
						public COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT: number;
						public COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT: number;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class WEBGL_depth_texture {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.WEBGL_depth_texture>;
						public UNSIGNED_INT_24_8_WEBGL: number;
						public constructor();
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class WEBGL_draw_buffers {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.WEBGL_draw_buffers>;
						public COLOR_ATTACHMENT0_WEBGL: number;
						public COLOR_ATTACHMENT1_WEBGL: number;
						public COLOR_ATTACHMENT2_WEBGL: number;
						public COLOR_ATTACHMENT3_WEBGL: number;
						public COLOR_ATTACHMENT4_WEBGL: number;
						public COLOR_ATTACHMENT5_WEBGL: number;
						public COLOR_ATTACHMENT6_WEBGL: number;
						public COLOR_ATTACHMENT7_WEBGL: number;
						public COLOR_ATTACHMENT8_WEBGL: number;
						public COLOR_ATTACHMENT9_WEBGL: number;
						public COLOR_ATTACHMENT10_WEBGL: number;
						public COLOR_ATTACHMENT11_WEBGL: number;
						public COLOR_ATTACHMENT12_WEBGL: number;
						public COLOR_ATTACHMENT13_WEBGL: number;
						public COLOR_ATTACHMENT14_WEBGL: number;
						public COLOR_ATTACHMENT15_WEBGL: number;
						public DRAW_BUFFER0_WEBGL: number;
						public DRAW_BUFFER1_WEBGL: number;
						public DRAW_BUFFER2_WEBGL: number;
						public DRAW_BUFFER3_WEBGL: number;
						public DRAW_BUFFER4_WEBGL: number;
						public DRAW_BUFFER5_WEBGL: number;
						public DRAW_BUFFER6_WEBGL: number;
						public DRAW_BUFFER7_WEBGL: number;
						public DRAW_BUFFER8_WEBGL: number;
						public DRAW_BUFFER9_WEBGL: number;
						public DRAW_BUFFER10_WEBGL: number;
						public DRAW_BUFFER11_WEBGL: number;
						public DRAW_BUFFER12_WEBGL: number;
						public DRAW_BUFFER13_WEBGL: number;
						public DRAW_BUFFER14_WEBGL: number;
						public DRAW_BUFFER15_WEBGL: number;
						public MAX_COLOR_ATTACHMENTS_WEBGL: number;
						public MAX_DRAW_BUFFERS_WEBGL: number;
						public constructor();
						public drawBuffersWEBGL(param0: native.Array<number>): void;
					}
				}
			}
		}
	}
}

declare module com {
	export module github {
		export module triniwiz {
			export module canvas {
				export module extensions {
					export class WEBGL_lose_context {
						public static class: java.lang.Class<com.github.triniwiz.canvas.extensions.WEBGL_lose_context>;
						public restoreContext(): void;
						public loseContext(): void;
						public constructor(param0: com.github.triniwiz.canvas.CanvasView);
					}
				}
			}
		}
	}
}

//Generics information:

