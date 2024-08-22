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

	static fromPath(path: string, callback: (p1: NSCSVG) => void): void;

	static fromPathSync(path: string): NSCSVG;

	static fromRemote(path: string, callback: (p1: NSCSVG) => void): void;

	static fromRemoteSync(path: string): NSCSVG;

	static fromString(source: string, callback: (p1: NSCSVG) => void): void;

	static fromStringSync(source: string): NSCSVG;

	static new(): NSCSVG; // inherited from NSObject

	autoScale: boolean;

	readonly buf_size: number;

	readonly data: interop.Pointer | interop.Reference<any>;

	readonly data_size: CGSize;

	src: string;

	srcPath: string;

	toData(): NSData;

	toImage(): UIImage;
}

declare function canvas_native_svg_draw_from_path(data: string | interop.Pointer | interop.Reference<any>, size: number, width: number, height: number, scale: number, path: string | interop.Pointer | interop.Reference<any>): void;

declare function canvas_native_svg_draw_from_string(data: string | interop.Pointer | interop.Reference<any>, size: number, width: number, height: number, scale: number, svg: string | interop.Pointer | interop.Reference<any>): void;
