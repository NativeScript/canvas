package org.nativescript.audiocontext;

public class StereoPannerNode extends AudioPannerNode {
    StereoPannerNode(String id) {
        super(id);
    }

    StereoPannerNode(AudioContextInstance context) {
        super(AudioContext.getInstance().createPanner(context.getId()).getId());
    }
}
