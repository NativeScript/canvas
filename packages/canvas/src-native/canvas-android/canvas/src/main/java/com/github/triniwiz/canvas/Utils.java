package com.github.triniwiz.canvas;

import android.os.Build;
import android.os.Environment;
import java.io.File;
public class Utils {
  private static int rating = -1;

  public static boolean isEmulator() {
    int newRating = 0;
    if (rating < 0) {
      if (Build.PRODUCT.contains("sdk") ||
        Build.PRODUCT.contains("Andy") ||
        Build.PRODUCT.contains("ttVM_Hdragon") ||
        Build.PRODUCT.contains("google_sdk") ||
        Build.PRODUCT.contains("Droid4X") ||
        Build.PRODUCT.contains("nox") ||
        Build.PRODUCT.contains("sdk_x86") ||
        Build.PRODUCT.contains("sdk_gphone_x86_arm") ||
        Build.PRODUCT.contains("sdk_google") ||
        Build.PRODUCT.contains("vbox86p")) {
        newRating++;
      }

      if (Build.MANUFACTURER.equals("unknown") ||
        Build.MANUFACTURER.equals("Genymotion") ||
        Build.MANUFACTURER.contains("Andy") ||
        Build.MANUFACTURER.contains("MIT") ||
        Build.MANUFACTURER.contains("nox") ||
        Build.MANUFACTURER.contains("TiantianVM") ||
        Build.MANUFACTURER.contains("vmos")) {
        newRating++;
      }

      if (Build.BRAND.equals("generic") ||
        Build.BRAND.equals("generic_x86") ||
        Build.BRAND.equals("generic_x86_arm") ||
        Build.BRAND.equals("TTVM") ||
        Build.BRAND.contains("Andy")) {
        newRating++;
      }

      if (Build.DEVICE.contains("generic") ||
        Build.DEVICE.contains("generic_x86") ||
        Build.DEVICE.equals("generic_x86_arm") ||
        Build.DEVICE.contains("Andy") ||
        Build.DEVICE.contains("ttVM_Hdragon") ||
        Build.DEVICE.contains("Droid4X") ||
        Build.DEVICE.contains("nox") ||
        Build.DEVICE.contains("generic_x86_64") ||
        Build.DEVICE.contains("sdk_gphone_x86_arm") ||
        Build.DEVICE.contains("vbox86p")) {
        newRating++;
      }

      if (Build.MODEL.equals("sdk") ||
        Build.MODEL.equals("google_sdk") ||
        Build.MODEL.equals("sdk_gphone_x86_arm") ||
        Build.MODEL.contains("Droid4X") ||
        Build.MODEL.contains("TiantianVM") ||
        Build.MODEL.contains("Andy") ||
        Build.MODEL.equals("Android SDK built for x86_64") ||
        Build.MODEL.equals("Android SDK built for x86") ||
        Build.MODEL.equals("vmos")) {
        newRating++;
      }

      if (Build.HARDWARE.equals("goldfish") ||
        Build.HARDWARE.equals("vbox86") ||
        Build.HARDWARE.contains("nox") ||
        Build.HARDWARE.contains("ttVM_x86")) {
        newRating++;
      }

      if (Build.FINGERPRINT.contains("generic/sdk/generic") ||
        Build.FINGERPRINT.contains("generic_x86/sdk_x86/generic_x86") ||
        Build.FINGERPRINT.contains("sdk_gphone_x86_arm") ||
        Build.FINGERPRINT.contains("Andy") ||
        Build.FINGERPRINT.contains("ttVM_Hdragon") ||
        Build.FINGERPRINT.contains("generic_x86_64") ||
        Build.FINGERPRINT.contains("generic/google_sdk/generic") ||
        Build.FINGERPRINT.contains("vbox86p") ||
        Build.FINGERPRINT.contains("generic/vbox86p/vbox86p") ||
        Build.FINGERPRINT.contains("test-keys")) {
        newRating++;
      }

      try {
        String opengl = android.opengl.GLES20.glGetString(android.opengl.GLES20.GL_RENDERER);
        if (opengl != null) {
          if (opengl.contains("Bluestacks") ||
            opengl.contains("Translator")
          )
            newRating += 10;
        }
      } catch (Exception e) {
        e.printStackTrace();
      }

      try {
        File sharedFolder = new File(Environment
          .getExternalStorageDirectory().toString()
          + File.separatorChar
          + "windows"
          + File.separatorChar
          + "BstSharedFolder");

        if (sharedFolder.exists()) {
          newRating += 10;
        }
      } catch (Exception e) {
        e.printStackTrace();
      }
      rating = newRating;
    }
    return rating > 3;
  }
}
