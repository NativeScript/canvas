/** The Node-API module (crates/audio-context-napi) on Node-API hosts. */
export declare const Native: any;

/** What a Worker requires to load the module. */
export declare const ADDON: string;

/** A path for the native module from a decodeAudioData path or file URL. */
export declare function nativePath(source: string): string;
