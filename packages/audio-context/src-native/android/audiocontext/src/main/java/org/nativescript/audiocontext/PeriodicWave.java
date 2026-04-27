package org.nativescript.audiocontext;

public class PeriodicWave implements NativeObject {
    private final String id;

    PeriodicWave(String id) {
        this.id = id;
        NativeFinalizer.register(this, NativeFinalizer.ResourceKind.PERIODICWAVE, id);
    }

    public String getId() {
        return id;
    }

    @Override
    public void release() {
        AudioContext.getInstance().releasePeriodicWave(id);
    }
}
