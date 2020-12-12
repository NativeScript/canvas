package com.github.triniwiz.asyncopdemo

import android.os.Bundle
import android.util.Log
import androidx.appcompat.app.AppCompatActivity
import com.github.triniwiz.async.Async2
import org.json.JSONException
import org.json.JSONObject
import java.io.File
import java.nio.charset.Charset

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        val font = File(filesDir, "helvetiker_regular.typeface.json")

        Async2.FileManager.readFile(font.absolutePath, null, object : Async2.FileManager.Callback {
            override fun onError(error: String?, e: Exception?) {
                Log.d("com.test", "readFile error: $error")
            }

            override fun onComplete(file: Any) {
                Log.d("com.test", "readFile success: ")
                val data = file as ByteArray
                try {
                    val string = String(data, Charset.forName("UTF-8"))
                    println(string)
                    val json = JSONObject(string)
                    Log.d("com.test", "json: " + json.toString().contains("»"))
                } catch (e: JSONException) {
                    e.printStackTrace()
                }
            }
        })

        Async2.Base64.base64ToFile("iVBORw0KGgoAAAANSUhEUgAAAAQAAAABAQMAAADD8p2OAAAAA1BMVEX/AP804Oa6AAAACklEQVQI12NgAAAAAgAB4iG8MwAAAABJRU5ErkJggg==", filesDir.absolutePath,object: Async2.Base64.Callback {
            override fun success(response: Any?) {
                Log.d("com.test", "base64ToFile success: $response")
            }

            override fun error(error: Any?, message: String?) {
                Log.d("com.test", "base64ToFile error: $error")
            }
        })
    }
}