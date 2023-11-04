package org.nativescript.canvas.polyfill;

import android.content.Context;
import android.content.SharedPreferences;
import android.webkit.MimeTypeMap;

import java.io.IOException;
import java.nio.ByteBuffer;
import java.io.File;
import java.io.FileOutputStream;
import java.nio.channels.Channels;
import java.nio.channels.WritableByteChannel;
import java.util.UUID;

public class Utils {

    static final String BLOB_PATH = "blob:nativescript/";
    static final String BLOB_DIR = "ns_blobs";

    static final String BLOB_KEYS = "org.nativescript.canvas.blob.keys";

    public static void writeBytes(ByteBuffer buffer, int position, String path) throws IOException {
        buffer.position(position);
        File file = new File(path);
        FileOutputStream fos = new FileOutputStream(file);
        WritableByteChannel channel = Channels.newChannel(fos);
        channel.write(buffer);
        channel.close();
        buffer.position(position);
    }

    public static String createObjectURL(Context context, ByteBuffer buffer, int position, String mime, String extension) throws IOException {
        String id = UUID.randomUUID().toString();
        createObjectURLWithURL(context, BLOB_PATH + id, buffer, position, mime, extension);
        return BLOB_PATH + id;
    }

    public static void createObjectURLWithURL(Context context, String url, ByteBuffer buffer, int position, String mime, String extension) throws IOException {
        String id = url.replace(BLOB_PATH, "");
        File blob_root = new File(context.getFilesDir(), BLOB_DIR);
        if (!blob_root.exists()) {
            blob_root.mkdirs();
        }

        String fileName = id;
        if (extension != null) {
            fileName = id + "." + extension;
        } else {
            // todo get type from magic bytes
            String ext = MimeTypeMap.getSingleton().getExtensionFromMimeType(mime);
            if (ext != null) {
                fileName = id + "." + ext;
            }
        }

        org.nativescript.canvas.polyfill.Utils.writeBytes(buffer, position, new File(blob_root, fileName).getAbsolutePath());

        putItem(context, id, fileName);
    }

    public static String getItemOrCreateAndReturn(Context context, String url, ByteBuffer buffer, int position, String mime, String extension) throws IOException {
        createObjectURLWithURL(context, url, buffer, position, mime, extension);
        return getPath(context, url);
    }

    public static void revokeObjectURL(Context context, String url) {
        if (url != null) {
            String id = url.replace(BLOB_PATH, "");
            String realPath = getItem(context, id);
            if (realPath == null || realPath.isEmpty()) {
                return;
            }
            File file = new File(realPath);
            if (file.exists()) {
                file.delete();
                deleteItem(context, id);
            }
        }
    }

    public static String getPath(Context context, String url) {
        if (url != null) {
            String id = url.replace(BLOB_PATH, "");
            return getItem(context, id);
        }
        return "";
    }


    public static void putItem(Context context, String key, String value) {
        SharedPreferences sharedPreferences = context.getSharedPreferences(BLOB_KEYS, Context.MODE_PRIVATE);
        sharedPreferences.edit().putString(key, value).commit();
    }

    public static String getItem(Context context, String key) {
        File blob_root = new File(context.getFilesDir(), BLOB_DIR);
        SharedPreferences sharedPreferences = context.getSharedPreferences(BLOB_KEYS, Context.MODE_PRIVATE);

        String fileName = sharedPreferences.getString(key, null);

        if (fileName != null) {
            return new File(blob_root, fileName).getAbsolutePath();
        }
        return null;
    }

    public static void deleteItem(Context context, String key) {
        if (key != null) {
            File blob_root = new File(context.getFilesDir(), BLOB_DIR);
            SharedPreferences sharedPreferences = context.getSharedPreferences(BLOB_KEYS, Context.MODE_PRIVATE);

            String fileName = sharedPreferences.getString(key, null);

            if (fileName != null) {
                File file = new File(blob_root, fileName);
                try {
                    file.delete();
                } catch (Exception ignored) {
                }
            }
            putItem(context, key, null);
        }
    }

}