declare class NSCFontDescriptors extends NSObject {
	static alloc(): NSCFontDescriptors; // inherited from NSObject

	static new(): NSCFontDescriptors; // inherited from NSObject

	static parseFontFaceRules(css: string): NSArray<NSDictionary<any, any>>;

	ascentOverride: string;

	descentOverride: string;

	display: NSCFontDisplay;

	family: string;

	featureSettings: string;

	kerning: string;

	lineGapOverride: string;

	stretch: string;

	style: NSCFontStyle;

	unicodeRange: string;

	variant: string;

	variantLigatures: string;

	variationSettings: string;

	weight: NSCFontWeight;

	constructor(o: { family: string });

	initWithFamily(family: string): this;

	setFontDisplayFromString(value: string): void;

	setFontStyleFromString(value: string): void;

	setFontWeightFromString(value: string): void;

	update(value: string): void;
}

declare const enum NSCFontDisplay {
	Auto = 0,

	Block = 1,

	Fallback = 2,

	Optional = 3,

	Swap = 4,
}

declare class NSCFontFace extends NSObject {
	static alloc(): NSCFontFace; // inherited from NSObject

	static clearFontCache(): void;

	static importFromRemoteLoadCompletion(url: string, load: boolean, completion: (p1: NSArray<NSCFontFace>, p2: string) => void): void;

	static new(): NSCFontFace; // inherited from NSObject

	ascentOverride: string;

	descentOverride: string;

	display: NSCFontDisplay;

	readonly family: string;

	featureSettings: string;

	font: any;

	readonly fontData: NSData;

	readonly fontDescriptors: NSCFontDescriptors;

	readonly fontPath: string;

	lineGapOverride: string;

	onReloadListeners: NSMutableArray<(p1: NSCFontFace, p2: string) => void>;

	status: NSCFontFaceStatus;

	stretch: string;

	style: NSCFontStyle;

	readonly uiFont: UIFont;

	unicodeRange: string;

	variant: string;

	variationSettings: string;

	weight: NSCFontWeight;

	constructor(o: { family: string });

	constructor(o: { family: string; data: NSData });

	constructor(o: { family: string; source: string });

	constructor(o: { fontDescriptor: NSCFontDescriptors });

	constructor(o: { fontDescriptor: NSCFontDescriptors; data: NSData });

	constructor(o: { fontDescriptor: NSCFontDescriptors; source: string });

	addReloadListener(listener: (p1: NSCFontFace, p2: string) => void): void;

	initWithFamily(family: string): this;

	initWithFamilyData(family: string, data: NSData): this;

	initWithFamilySource(family: string, source: string): this;

	initWithFontDescriptor(fontDescriptor: NSCFontDescriptors): this;

	initWithFontDescriptorData(fontDescriptor: NSCFontDescriptors, data: NSData): this;

	initWithFontDescriptorSource(fontDescriptor: NSCFontDescriptors, source: string): this;

	load(callback: (p1: string) => void): void;

	loadSync(outError: interop.Pointer | interop.Reference<string>): void;

	rawData(): NSData;

	removeAllReloadListeners(): void;

	removeOnReloadListener(listener: (p1: NSCFontFace, p2: string) => void): void;

	setFontAscentOverride(value: string): void;

	setFontDescentOverride(value: string): void;

	setFontDisplay(value: string): void;

	setFontFeatureSettings(value: string): void;

	setFontLineGapOverride(value: string): void;

	setFontStretch(value: string): void;

	setFontStyleAngle(value: string, angle: string): void;

	setFontUnicodeRange(value: string): void;

	setFontVariant(value: string): void;

	setFontVariationSettings(value: string): void;

	setFontWeight(value: string): void;

	uiFontWithSize(size: number): UIFont;

	updateDescriptor(value: string): void;

	updateDescriptorWithValue(value: string): void;
}

declare var NSCFontFacePattern: string;

declare class NSCFontFaceSet extends NSObject {
	static alloc(): NSCFontFaceSet; // inherited from NSObject

	static instance(): NSCFontFaceSet;

	static new(): NSCFontFaceSet; // inherited from NSObject

	status: NSCFontFaceSetStatus;

	add(font: NSCFontFace): void;

	addOnLoadingDoneListener(listener: (p1: NSCFontFace) => void): void;

	addOnLoadingErrorListener(listener: (p1: NSCFontFace, p2: string) => void): void;

	addOnLoadingListener(listener: (p1: NSCFontFace) => void): void;

	addOnStatusListener(listener: (p1: NSCFontFaceSetStatus) => void): void;

	array(): NSArray<NSCFontFace>;

	checkText(font: string, text: string): boolean;

	clear(): void;

	delete(font: NSCFontFace): void;

	forEach(block: (p1: NSCFontFace) => void): void;

	has(font: NSCFontFace): boolean;

	iter(): NSEnumerator<any>;

	loadTextCallback(font: string, text: string, callback: (p1: NSArray<NSCFontFace>, p2: string) => void): void;

	ready(callback: (p1: NSCFontFaceSet) => void): void;

	removeOnLoadingDoneListener(listener: (p1: NSCFontFace) => void): void;

	removeOnLoadingErrorListener(listener: (p1: NSCFontFace, p2: string) => void): void;

	removeOnLoadingListener(listener: (p1: NSCFontFace) => void): void;

	removeOnStatusListener(listener: (p1: NSCFontFaceSetStatus) => void): void;

	size(): number;
}

declare const enum NSCFontFaceSetEventType {
	Add = 0,

	Remove = 1,

	Clear = 2,
}

interface NSCFontFaceSetListener extends NSObjectProtocol {
	fontFaceSetDidEmitEventFaceFamily?(event: NSCFontFaceSetEventType, face: NSCFontFace, family: string): void;
}
declare var NSCFontFaceSetListener: {
	prototype: NSCFontFaceSetListener;
};

declare const enum NSCFontFaceSetStatus {
	Loading = 0,

	Loaded = 1,
}

declare const enum NSCFontFaceStatus {
	Unloaded = 0,

	Loading = 1,

	Loaded = 2,

	Error = 3,
}

declare class NSCFontParseResult extends NSObject {
	static alloc(): NSCFontParseResult; // inherited from NSObject

	static new(): NSCFontParseResult; // inherited from NSObject

	families: NSArray<string>;

	lineHeight: number;

	sizePx: number;

	style: NSCFontStyle;

	weight: number;
}

declare class NSCFontParser extends NSObject {
	static alloc(): NSCFontParser; // inherited from NSObject

	static new(): NSCFontParser; // inherited from NSObject

	static parse(input: string): NSCFontParseResult;
}

declare class NSCFontResolver extends NSObject {
	static alloc(): NSCFontResolver; // inherited from NSObject

	static new(): NSCFontResolver; // inherited from NSObject

	static shared(): NSCFontResolver;

	importFromRemoteWithURLLoadCompletion(url: string, load: boolean, completion: (p1: NSArray<NSCFontFace>, p2: NSError) => void): void;

	loadFontDataFromURLError(src: string): NSData;

	registerFontFromDataError(data: NSData): any;

	resolveFontWithFamilySrcCompletion(family: string, src: string, completion: (p1: any, p2: NSData, p3: NSError) => void): void;

	resolveGenericFamily(family: string): string;
}

declare class NSCFontResolverResult extends NSObject {
	static alloc(): NSCFontResolverResult; // inherited from NSObject

	static new(): NSCFontResolverResult; // inherited from NSObject

	data: NSData;

	font: any;
}

declare class NSCFontStyle extends NSObject implements NSCopying {
	static alloc(): NSCFontStyle; // inherited from NSObject

	static fromString(value: string): NSCFontStyle;

	static italic(): NSCFontStyle;

	static new(): NSCFontStyle; // inherited from NSObject

	static normal(): NSCFontStyle;

	static obliqueWithAngle(angle: number): NSCFontStyle;

	readonly obliqueAngle: number;

	readonly type: NSCFontStyleType;

	copyWithZone(zone: interop.Pointer | interop.Reference<any>): any;

	toString(): string;
}

declare const enum NSCFontStyleType {
	Normal = 0,

	Italic = 1,

	Oblique = 2,
}

declare const enum NSCFontWeight {
	Thin = 100,

	ExtraLight = 200,

	Light = 300,

	Normal = 400,

	Medium = 500,

	SemiBold = 600,

	Bold = 700,

	ExtraBold = 800,

	Black = 900,
}

declare var NSCPropertyPattern: string;

declare function NSCUIFontWeight(weight: NSCFontWeight): number;
