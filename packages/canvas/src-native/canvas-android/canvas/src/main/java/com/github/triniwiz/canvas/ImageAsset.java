package com.github.triniwiz.canvas;

import android.graphics.Bitmap;

import androidx.annotation.Nullable;

import java.io.ByteArrayOutputStream;
import java.nio.Buffer;
import java.nio.ByteBuffer;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

/**
 * Created by triniwiz on 5/4/20
 */
public class ImageAsset {
    private static native long nativeInit();

    private static native int nativeGetWidth(long asset);

    private static native int nativeGetHeight(long asset);

    private static native int nativeFlipX(long asset);

    private static native int nativeFlipY(long asset);

    private static native byte[] nativeGetBytes(long asset);

    private static native ByteBuffer nativeGetBuffer(long asset);

    private static native long nativeScale(long asset, int x, int y);

    private static native String nativeGetError(long asset);

    private static native boolean nativeHasError(long asset);

    private static native boolean nativeSave(long asset, String path, int format);

    private static native boolean nativeLoadAssetPath(long asset, String path);

    private static native boolean nativeLoadAssetBytes(long asset, byte[] buffer);

    private static native boolean nativeLoadAssetBuffer(long asset, Buffer buffer);

    private static native void nativeRelease(long asset);

    long nativeImageAsset;

    private static ExecutorService executorService = Executors.newCachedThreadPool();

    public interface Callback {
        public void onSuccess(Object value);

        public void onError(String error);
    }

    public ImageAsset() {
        this.nativeImageAsset = nativeInit();
    }

    public int getWidth() {
        if (nativeImageAsset == 0) {
            return 0;
        }
        return nativeGetWidth(nativeImageAsset);
    }

    public int getHeight() {
        if (nativeImageAsset == 0) {
            return 0;
        }
        return nativeGetHeight(nativeImageAsset);
    }

    public byte[] getBytes() {
        return nativeGetBytes(nativeImageAsset);
    }


    public ByteBuffer getBuffer() {
        return nativeGetBuffer(nativeImageAsset);
    }

    public @Nullable
    String getError() {
        if (nativeImageAsset == 0) {
            return null;
        }
        if (!nativeHasError(nativeImageAsset)) {
            return null;
        }
        return nativeGetError(nativeImageAsset);
    }

    public void scale(int x, int y) {
        if (nativeImageAsset == 0) {
            return;
        }
        nativeImageAsset = nativeScale(nativeImageAsset, x, y);
    }

    public void flipX() {
        if (nativeImageAsset == 0) {
            return;
        }
        nativeImageAsset = nativeFlipX(nativeImageAsset);
    }

    public void flipY() {
        if (nativeImageAsset == 0) {
            return;
        }
        nativeImageAsset = nativeFlipY(nativeImageAsset);
    }

    public boolean save(String path, ImageAssetFormat format) {
        if (nativeImageAsset == 0) {
            return false;
        }
        return nativeSave(nativeImageAsset, path, format.getFormat());
    }

    public void saveAsync(final String path, final ImageAssetFormat format, final Callback callback) {
        executorService.submit(new Runnable() {
            @Override
            public void run() {
                if (save(path, format)) {
                    callback.onSuccess(true);
                } else {
                    callback.onError(getError());
                }
            }
        });
    }

    public boolean loadImageFromPath(String path) {
        if (nativeImageAsset == 0) {
            return false;
        }
        return nativeLoadAssetPath(nativeImageAsset, path);
    }

    public void loadImageFromPathAsync(final String path, final Callback callback) {
        executorService.submit(new Runnable() {
            @Override
            public void run() {
                if (nativeLoadAssetPath(nativeImageAsset, path)) {
                    callback.onSuccess(true);
                } else {
                    callback.onError(getError());
                }
            }
        });
    }

    public boolean loadImageFromBytes(byte[] buffer) {
        if (nativeImageAsset == 0) {
            return false;
        }
        return nativeLoadAssetBytes(nativeImageAsset, buffer);
    }

    public boolean loadImageFromBuffer(ByteBuffer buffer) {
        if (nativeImageAsset == 0) {
            return false;
        }
        return nativeLoadAssetBuffer(nativeImageAsset, buffer);
    }

    public void loadImageFromBytesAsync(final byte[] buffer, final Callback callback) {
        executorService.submit(new Runnable() {
            @Override
            public void run() {
                if (nativeLoadAssetBytes(nativeImageAsset, buffer)) {
                    callback.onSuccess(true);
                } else {
                    callback.onError(getError());
                }
            }
        });
    }


    public void loadImageFromBuferAsync(final ByteBuffer buffer, final Callback callback) {
        executorService.submit(new Runnable() {
            @Override
            public void run() {
                if (nativeLoadAssetBuffer(nativeImageAsset, buffer)) {
                    callback.onSuccess(true);
                } else {
                    callback.onError(getError());
                }
            }
        });
    }


    public boolean loadImageFromImage(Bitmap bitmap) {
        if (nativeImageAsset == 0) {
            return false;
        }
        ByteArrayOutputStream os = new ByteArrayOutputStream();
        bitmap.compress(Bitmap.CompressFormat.PNG, 100, os);
        return loadImageFromBytes(os.toByteArray());
    }

    public void loadImageFromImageAsync(final Bitmap bitmap, final Callback callback) {
        executorService.submit(new Runnable() {
            @Override
            public void run() {
                ByteArrayOutputStream os = new ByteArrayOutputStream();
                bitmap.compress(Bitmap.CompressFormat.PNG, 100, os);
                if (loadImageFromBytes(os.toByteArray())) {
                    callback.onSuccess(true);
                } else {
                    callback.onError(getError());
                }
            }
        });
    }

    @Override
    protected void finalize() throws Throwable {
        nativeRelease(nativeImageAsset);
        nativeImageAsset = 0;
        super.finalize();
    }
}
