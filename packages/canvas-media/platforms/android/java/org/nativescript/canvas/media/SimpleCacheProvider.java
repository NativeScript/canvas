package org.nativescript.canvas.media;

import android.util.Log;
import java.io.File;

import androidx.media3.datasource.cache.SimpleCache;
import androidx.media3.datasource.cache.LeastRecentlyUsedCacheEvictor;
import androidx.media3.database.DatabaseProvider;

public class SimpleCacheProvider {
    private static SimpleCache instance = null;

    public synchronized static SimpleCache getInstance(File cacheDir, LeastRecentlyUsedCacheEvictor evictor, DatabaseProvider dbProvider) {
        if (instance == null) {
            try {
                instance = new SimpleCache(cacheDir, evictor, dbProvider);
            } catch (IllegalStateException e) {
                Log.w("SimpleCacheProvider", "SimpleCache already exists for dir: " + cacheDir + " - " + e.getMessage());
                instance = null;
            } catch (Exception e) {
                Log.w("SimpleCacheProvider", "Failed to create SimpleCache: " + e.getMessage());
                instance = null;
            }
        }
        return instance;
    }
}
