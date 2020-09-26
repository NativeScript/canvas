package com.github.triniwiz.asyncdemo;

import androidx.annotation.Nullable;
import androidx.appcompat.app.AppCompatActivity;

import android.os.Bundle;
import android.util.Log;

import com.github.triniwiz.async.Async;

import org.json.JSONException;
import org.json.JSONObject;

import java.io.File;
import java.io.UnsupportedEncodingException;

public class MainActivity extends AppCompatActivity {

	@Override
	protected void onCreate(Bundle savedInstanceState) {
		super.onCreate(savedInstanceState);
		setContentView(R.layout.activity_main);
		File font = new File(getFilesDir(), "helvetiker_regular.typeface.json");
		Async.FileManager.readFile(font.getAbsolutePath(), null, new Async.FileManager.Callback() {
			@Override
			public void onError(String error, Exception e) {
				Log.d("com.test", "readFile error: " + error);
			}

			@Override
			public void onComplete(@Nullable Object object) {
				Log.d("com.test", "readFile success: ");
				byte[] data = (byte[]) object;
				try {
					String string = new String(data, java.nio.charset.Charset.forName("UTF-8")).toString();
					JSONObject json = new JSONObject(string);
					Log.d("com.test", "json: " + json.toString().contains("»"));
				}catch (JSONException e) {
					e.printStackTrace();
				}
			}
		});

	}
}
