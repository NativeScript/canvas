declare class CanvasSVGHelper extends NSObject {
	static alloc(): CanvasSVGHelper; // inherited from NSObject

	static drawFromPathSizeWidthHeightPath(data: string | interop.Pointer | interop.Reference<any>, size: number, width: number, height: number, path: string): void;

	static drawFromStringSizeWidthHeightSvg(data: string | interop.Pointer | interop.Reference<any>, size: number, width: number, height: number, svg: string): void;

	static new(): CanvasSVGHelper; // inherited from NSObject
}

declare var CanvasSVGVersionNumber: number;

declare var CanvasSVGVersionString: interop.Reference<number>;

declare class NSCSVG extends UIView {
	static alloc(): NSCSVG; // inherited from NSObject

	static appearance(): NSCSVG; // inherited from UIAppearance

	/**
	 * @since 8.0
	 */
	static appearanceForTraitCollection(trait: UITraitCollection): NSCSVG; // inherited from UIAppearance

	/**
	 * @since 8.0
	 * @deprecated 9.0
	 */
	static appearanceForTraitCollectionWhenContainedIn(trait: UITraitCollection, ContainerClass: typeof NSObject): NSCSVG; // inherited from UIAppearance

	/**
	 * @since 9.0
	 */
	static appearanceForTraitCollectionWhenContainedInInstancesOfClasses(trait: UITraitCollection, containerTypes: NSArray<typeof NSObject> | (typeof NSObject)[]): NSCSVG; // inherited from UIAppearance

	/**
	 * @since 5.0
	 * @deprecated 9.0
	 */
	static appearanceWhenContainedIn(ContainerClass: typeof NSObject): NSCSVG; // inherited from UIAppearance

	/**
	 * @since 9.0
	 */
	static appearanceWhenContainedInInstancesOfClasses(containerTypes: NSArray<typeof NSObject> | (typeof NSObject)[]): NSCSVG; // inherited from UIAppearance

	static fromPath(path: string, callback: (p1: NSCSVGData) => void): void;

	static fromPathSync(path: string): NSCSVGData;

	static fromRemote(path: string, callback: (p1: NSCSVGData) => void): void;

	static fromRemoteSync(path: string): NSCSVGData;

	static fromString(source: string, callback: (p1: NSCSVGData) => void): void;

	static fromStringSync(source: string): NSCSVGData;

	static new(): NSCSVG; // inherited from NSObject

	autoScale: boolean;

	src: string;

	srcPath: string;

	toData(): NSData;

	toImage(): UIImage;

	update(): void;
}

declare class NSCSVGData extends NSObject {
	static alloc(): NSCSVGData; // inherited from NSObject

	static new(): NSCSVGData; // inherited from NSObject

	readonly data: NSMutableData;

	readonly height: number;

	readonly rawData: interop.Pointer | interop.Reference<any>;

	readonly width: number;

	getImage(): UIImage;
}

declare function canvas_native_svg_draw_from_bytes(data: string | interop.Pointer | interop.Reference<any>, size: number, width: number, height: number, scale: number, svg_data: string | interop.Pointer | interop.Reference<any>, svg_size: number): void;

declare function canvas_native_svg_draw_from_path(data: string | interop.Pointer | interop.Reference<any>, size: number, width: number, height: number, scale: number, path: string | interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_svg_draw_from_string(data: string | interop.Pointer | interop.Reference<any>, size: number, width: number, height: number, scale: number, svg: string | interop.Pointer | interop.Reference<any>): void;
