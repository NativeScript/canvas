package org.nativescript.canvas_media;
import android.content.Context;
import android.net.Uri;

import com.google.android.exoplayer2.database.ExoDatabaseProvider;
import com.google.android.exoplayer2.upstream.DataSource;
import com.google.android.exoplayer2.upstream.DataSpec;
import com.google.android.exoplayer2.upstream.DefaultDataSourceFactory;
import com.google.android.exoplayer2.upstream.cache.CacheDataSource;
import com.google.android.exoplayer2.upstream.cache.CacheWriter;
import com.google.android.exoplayer2.upstream.cache.LeastRecentlyUsedCacheEvictor;
import com.google.android.exoplayer2.upstream.cache.SimpleCache;
import com.google.android.exoplayer2.util.Util;

import java.io.File;
import java.io.IOException;
import java.util.concurrent.Executor;
import java.util.concurrent.Executors;


public class Utils {
	private static Executor executor = Executors.newCachedThreadPool();
	private static SimpleCache cache;
	private static LeastRecentlyUsedCacheEvictor leastRecentlyUsedCacheEvictor;
	private static ExoDatabaseProvider exoDatabaseProvider;
	private static long exoPlayerCacheSize = 100 * 1024 * 1024;
	private static java.io.File cacheDir;
	private static boolean didInit;
	private static String packageName;

	public static void init(Context context, String cacheDir) {
		init(context, cacheDir, exoPlayerCacheSize);
	}

	public static void init(Context context, String cacheDir, long exoPlayerCacheSize) {
		if (!didInit) {
			Utils.packageName = context.getPackageName();
			Utils.cacheDir = new File(cacheDir);
			Utils.exoPlayerCacheSize = exoPlayerCacheSize;
			if (!Utils.cacheDir.exists()) {
				Utils.cacheDir.mkdirs();
			}

			leastRecentlyUsedCacheEvictor = new com.google.android.exoplayer2.upstream.cache.LeastRecentlyUsedCacheEvictor(exoPlayerCacheSize);

			exoDatabaseProvider = new com.google.android.exoplayer2.database.ExoDatabaseProvider(context);
			cache = new com.google.android.exoplayer2.upstream.cache.SimpleCache(Utils.cacheDir, leastRecentlyUsedCacheEvictor, exoDatabaseProvider);
			didInit = true;
		}
	}

	public static void cacheUrl(final Context context, final String url) {
		executor.execute(new Runnable() {
			@Override
			public void run() {
				Uri videoUri = Uri.parse(url);
				DataSpec dataSpec = new DataSpec(videoUri);

				DataSource dataSource =
					new DefaultDataSourceFactory(
						context,
						Util.getUserAgent(context, packageName)).createDataSource();

				try {
					new CacheWriter(
						new CacheDataSource(cache, dataSource), dataSpec, true, null, null
					).cache();
				} catch (IOException ignored) {
				}
			}
		});
	}
}