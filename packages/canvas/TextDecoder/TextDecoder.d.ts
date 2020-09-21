import {TextDecoderBase} from './common';

export declare class TextDecoder extends TextDecoderBase {
    readonly native: any;
    readonly encoding: string;

    constructor(encoding?: string);

    decode(buffer?: ArrayBuffer | ArrayBufferView, options?: any): string;
}
