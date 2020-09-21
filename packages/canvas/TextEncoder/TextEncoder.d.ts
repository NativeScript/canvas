import {TextEncoderBase} from './common';

export declare class TextEncoder extends TextEncoderBase {
	readonly native;
	readonly encoding: string;

	constructor(encoding?: string);

	encode(text: string): Uint8Array;
}
