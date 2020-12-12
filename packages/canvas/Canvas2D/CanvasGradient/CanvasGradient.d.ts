import {CanvasGradientBase} from './common';

export declare class CanvasGradient extends CanvasGradientBase {
    readonly nativeInstance: any;
    readonly native: any;

    protected constructor(nativeInstance: any);

    static fromNative(nativeInstance: any): CanvasGradient;

    addColorStop(offset: number, color: string): void;
}
