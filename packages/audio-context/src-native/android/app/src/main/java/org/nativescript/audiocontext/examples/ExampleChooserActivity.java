package org.nativescript.audiocontext.examples;

import android.content.Intent;
import android.os.Bundle;
import android.widget.Button;

import androidx.appcompat.app.AppCompatActivity;


public class ExampleChooserActivity extends AppCompatActivity {
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        int layoutId = getResources().getIdentifier("activity_example_chooser", "layout", getPackageName());
        if (layoutId != 0) setContentView(layoutId);

        Button btnPanner = findViewById(getResources().getIdentifier("btn_open_panner", "id", getPackageName()));
        Button btnVisualizer = findViewById(getResources().getIdentifier("btn_open_visualizer", "id", getPackageName()));

        btnPanner.setOnClickListener(v -> {
            Intent it = new Intent(ExampleChooserActivity.this, PannerDemoActivity.class);
            startActivity(it);
        });

        btnVisualizer.setOnClickListener(v -> {
            Intent it = new Intent(ExampleChooserActivity.this, VisualizerActivity.class);
            startActivity(it);
        });
    }
}
