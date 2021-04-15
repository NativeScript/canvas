import { ImageAssetBase, ImageAssetSaveFormat } from './common';
import { knownFolders, path as filePath } from '@nativescript/core';

export class ImageAsset extends ImageAssetBase {
    constructor() {
        super(new org.nativescript.canvas.TNSImageAsset());
    }

    get width() {
        return this.native.getWidth();
    }

    get height() {
        return this.native.getHeight();
    }

    get error(): string {
        return this.native.getError();
    }

    static toPrimitive(value): any {
        if (value instanceof java.lang.Integer) {
            return value.intValue();
        } else if (value instanceof java.lang.Long) {
            return value.longValue();
        } else if (value instanceof java.lang.Short) {
            return value.shortValue();
        } else if (value instanceof java.lang.Byte) {
            return value.byteValue();
        } else if (value instanceof java.lang.Boolean) {
            return value.booleanValue();
        } else if (value instanceof java.lang.String) {
            return value.toString();
        } else if (value instanceof java.lang.Float) {
            return value.floatValue();
        } else if (value instanceof java.lang.Double) {
            return value.doubleValue();
        }
        return value;
    }

    loadFromUrl(url: string): boolean {
        return this.native.loadImageFromUrl(url);
    }

    loadFromUrlAsync(path: string) {
        return new Promise((resolve, reject) => {
            this.native.loadImageFromUrlAsync(
                path,
                new org.nativescript.canvas.TNSImageAsset.Callback({
                    onError(error) {
                        reject(error);
                    },
                    onSuccess(success) {
                        resolve(ImageAsset.toPrimitive(success));
                    },
                })
            );
        });
    }


    loadFile(path: string): boolean {
        let realPath = path;
        if (typeof realPath === 'string') {
            if (realPath.startsWith('~/')) {
                realPath = filePath.join(
                    knownFolders.currentApp().path,
                    realPath.replace('~/', '')
                );
            }
        }
        return this.native.loadImageFromPath(realPath);
    }

    loadFileAsync(path: string) {
        return new Promise((resolve, reject) => {
            if (typeof path === 'string') {
                if (path.startsWith('~/')) {
                    path = filePath.join(
                        knownFolders.currentApp().path,
                        path.replace('~/', '')
                    );
                }
            }
            this.native.loadImageFromPathAsync(
                path,
                new org.nativescript.canvas.TNSImageAsset.Callback({
                    onError(error) {
                        reject(error);
                    },
                    onSuccess(success) {
                        resolve(ImageAsset.toPrimitive(success));
                    },
                })
            );
        });
    }

    loadFromNative(image: any): boolean {
        return this.native.loadImageFromImage(image);
    }

    loadFromNativeAsync(image: any) {
        return new Promise((resolve, reject) => {
            this.native.loadImageFromImageAsync(
                image,
                new org.nativescript.canvas.TNSImageAsset.Callback({
                    onError(error) {
                        reject(error);
                    },
                    onSuccess(success) {
                        resolve(ImageAsset.toPrimitive(success));
                    },
                })
            );
        });
    }

    loadFromBytes(bytes: Uint8Array | Uint8ClampedArray) {
        if (bytes instanceof Uint8Array || bytes instanceof Uint8ClampedArray) {
            return this.native.loadImageFromBytes(Array.from(bytes));
        }
        return this.native.loadImageFromBytes(bytes as any);
    }

    loadFromBytesAsync(bytes: Uint8Array | Uint8ClampedArray) {
        return new Promise((resolve, reject) => {
            const callback = new org.nativescript.canvas.TNSImageAsset.Callback({
                onError(error) {
                    reject(error);
                },
                onSuccess(success) {
                    resolve(ImageAsset.toPrimitive(success));
                },
            });
            if (bytes instanceof Uint8Array || bytes instanceof Uint8ClampedArray) {
                this.native.loadImageFromBytesAsync(Array.from(bytes), callback);
            } else {
                this.native.loadImageFromBytesAsync(bytes as any, callback);
            }
        });
    }

    scale(x: number, y: number) {
        this.native.scale(x, y);
    }

    save(path: string, format: ImageAssetSaveFormat): boolean {
        let realFormat;
        switch (format) {
            case ImageAssetSaveFormat.PNG:
                realFormat = org.nativescript.canvas.TNSImageAssetFormat.PNG;
                break;
            case ImageAssetSaveFormat.ICO:
                realFormat = org.nativescript.canvas.TNSImageAssetFormat.ICO;
                break;
            case ImageAssetSaveFormat.BMP:
                realFormat = org.nativescript.canvas.TNSImageAssetFormat.BMP;
                break;
            case ImageAssetSaveFormat.TIFF:
                realFormat = org.nativescript.canvas.TNSImageAssetFormat.TIFF;
                break;
            default:
                realFormat = org.nativescript.canvas.TNSImageAssetFormat.JPG;
                break;
        }
        return this.native.save(path, realFormat);
    }

    saveAsync(path: string, format: ImageAssetSaveFormat): Promise<boolean> {
        return new Promise((resolve, reject) => {
            let realFormat;
            switch (format) {
                case ImageAssetSaveFormat.PNG:
                    realFormat = org.nativescript.canvas.TNSImageAssetFormat.PNG;
                    break;
                case ImageAssetSaveFormat.ICO:
                    realFormat = org.nativescript.canvas.TNSImageAssetFormat.ICO;
                    break;
                case ImageAssetSaveFormat.BMP:
                    realFormat = org.nativescript.canvas.TNSImageAssetFormat.BMP;
                    break;
                case ImageAssetSaveFormat.TIFF:
                    realFormat = org.nativescript.canvas.TNSImageAssetFormat.TIFF;
                    break;
                default:
                    realFormat = org.nativescript.canvas.TNSImageAssetFormat.JPG;
                    break;
            }
            (this.native as org.nativescript.canvas.TNSImageAsset).saveAsync(
                path,
                realFormat,
                new org.nativescript.canvas.TNSImageAsset.Callback({
                    onError(error) {
                        reject(error);
                    },
                    onSuccess(success) {
                        resolve(ImageAsset.toPrimitive(success));
                    },
                })
            );
        });
    }

    flipX() {
        this.native.flipX();
    }

    flipY() {
        this.native.flipY();
    }
}
