import {CanvasGradientBase} from './common';
import {Color} from '@nativescript/core';

export class CanvasGradient extends CanvasGradientBase {
    readonly nativeInstance;

    protected constructor(nativeInstance: any) {
        super();
        this.nativeInstance = nativeInstance;
    }

    get native() {
        return this.nativeInstance;
    }

    static fromNative(nativeInstance) {
        return new CanvasGradient(nativeInstance);
    }

    public addColorStop(offset: number, color: any): void {
        this.nativeInstance.addColorStop(offset, new Color(color).android);
    }
}
