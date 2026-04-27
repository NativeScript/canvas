package org.nativescript.audiocontextdemo

import android.os.Bundle
import android.widget.Button
import android.widget.SeekBar
import androidx.activity.enableEdgeToEdge
import androidx.appcompat.app.AppCompatActivity
import androidx.core.view.ViewCompat
import androidx.core.view.WindowInsetsCompat
import java.io.ByteArrayOutputStream
import org.nativescript.audiocontext.AudioContext
import org.nativescript.audiocontext.AudioContextInstance

class MainActivity : AppCompatActivity() {
    private lateinit var ac: AudioContext
    private lateinit var ctx: AudioContextInstance
    private var gainNode: org.nativescript.audiocontext.GainNode? = null
    private var analyser: org.nativescript.audiocontext.AnalyserNode? = null
    private var currentSource: org.nativescript.audiocontext.AudioBufferSourceNode? = null

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContentView(R.layout.activity_main)

        ViewCompat.setOnApplyWindowInsetsListener(findViewById(R.id.main)) { v, insets ->
            val systemBars = insets.getInsets(WindowInsetsCompat.Type.systemBars())
            v.setPadding(systemBars.left, systemBars.top, systemBars.right, systemBars.bottom)
            insets
        }

        ac = AudioContext.getInstance()
        ctx = AudioContextInstance()
        gainNode = ac.createGain(ctx)
        analyser = ac.createAnalyser(ctx)
        // route: gain -> analyser -> destination
        gainNode?.connect(analyser)
        analyser?.connect(ac.getDestination(ctx))

        val visualizer = findViewById<VisualizerView>(R.id.visualizer)
        visualizer.setAnalyser(analyser)

        val playButton = findViewById<Button>(R.id.play_button)
        val stopButton = findViewById<Button>(R.id.stop_button)
        val volumeSeek = findViewById<SeekBar>(R.id.volume_seekbar)


        volumeSeek.progress = 50
        volumeSeek.setOnSeekBarChangeListener(object : SeekBar.OnSeekBarChangeListener {
            override fun onProgressChanged(seekBar: SeekBar?, progress: Int, fromUser: Boolean) {
                val v = progress / 100.0
                gainNode?.gain?.setValueAtTime(v, 0.0)
            }

            override fun onStartTrackingTouch(seekBar: SeekBar?) {}
            override fun onStopTrackingTouch(seekBar: SeekBar?) {}
        })

        stopButton.setOnClickListener {
            try {
                currentSource?.stop()
            } catch (_: Throwable) {}
            currentSource = null
        }

        playButton.setOnClickListener {
            playButton.isEnabled = false
            Thread {
                try {
                    val ins = resources.openRawResource(R.raw.gs_16b_1c_44100hz)
                    val baos = ByteArrayOutputStream()
                    val buf = ByteArray(4096)
                    var r = ins.read(buf)
                    while (r > 0) {
                        baos.write(buf, 0, r)
                        r = ins.read(buf)
                    }
                    ins.close()
                    val bytes = baos.toByteArray()

                    runOnUiThread {
                        ac.decodeAudioDataFromByteArrayAsync(bytes, ctx) { buffer ->
													if (buffer != null) {
														try {
															val src = ac.createBufferSource(ctx, buffer)
															gainNode?.let { src.connect(it) }
															src.loop = false
															src.start()
															currentSource = src
														} catch (e: Throwable) {
															e.printStackTrace()
														}
													}
													playButton.isEnabled = true
												}
										}
                } catch (e: Exception) {
                    e.printStackTrace()
                    runOnUiThread { playButton.isEnabled = true }
                }
            }.start()
        }
    }
}
