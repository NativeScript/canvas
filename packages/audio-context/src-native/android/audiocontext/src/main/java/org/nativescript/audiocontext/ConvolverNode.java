package org.nativescript.audiocontext;

import androidx.annotation.Nullable;

public class ConvolverNode implements NativeObject, AudioNode {
    private final String id;
    private final String contextId;
    @Nullable
    private AudioBuffer buffer;
    private boolean normalize = true;

    ConvolverNode(String id) {
        this(id, null);
    }

    ConvolverNode(String id, @Nullable String contextId) {
        this.id = id;
        this.contextId = contextId;
        NativeFinalizer.register(this, NativeFinalizer.ResourceKind.GAIN, id);
    }

    ConvolverNode(AudioContextInstance context) {
        this(AudioContext.getInstance().createConvolver(context.getId()), context.getId());
    }

    @Override
    public String getId() {
        return id;
    }

    @Nullable
    public AudioBuffer getBuffer() {
        return buffer;
    }

    public void setBuffer(@Nullable AudioBuffer buffer) {
        this.buffer = buffer;
        if (buffer != null && contextId != null) {
            AudioContext.getInstance().assignBufferToContext(buffer, contextId);
        }
    }

    public boolean getNormalize() { return normalize; }
    public void setNormalize(boolean v) { this.normalize = v; }

    public void setValue(double v) {
        AudioContext.getInstance().setGain(id, v);
    }

    @Override
    public void connect(AudioNode node) {
        connect(node, 0, 0);
    }

    @Override
    public void connect(AudioNode node, int output) {
        connect(node, output, 0);
    }

    @Override
    public void connect(AudioNode node, int output, int input) {
        if (node instanceof NativeObject) {
            AudioContext context = AudioContext.getInstance();
            NativeObject object = (NativeObject) node;
            String destId = object.getId();
            if (destId != null) {
                if (context.isDestinationNode(destId)) {
                    return;
                }

                java.util.Set<String> sources = context.getNodeSourcesSnapshot(id);
                if (sources.isEmpty()) {
                    context.attachGainToVoice(destId, id, output, input);
                    context.registerNodeSource(destId, id);
                    return;
                }

                for (String sourceId : sources) {
                    if (node instanceof AudioPannerNode) {
                        context.attachPannerToVoice(sourceId, destId, output, input);
                    } else if (node instanceof AudioBiquadNode) {
                        context.attachBiquadToVoice(sourceId, destId, output, input);
                    } else {
                        context.attachGainToVoice(sourceId, destId, output, input);
                    }
                }
                context.registerNodeSources(destId, sources);
            }
        }
    }

    @Override
    public void disconnect(AudioNode node) {
        if (node instanceof NativeObject) {
            AudioContext context = AudioContext.getInstance();
            NativeObject object = (NativeObject) node;
            String destId = object.getId();
            if (destId != null) {
                java.util.Set<String> sources = context.getNodeSourcesSnapshot(id);
                for (String sourceId : sources) {
                    if (node instanceof AudioPannerNode) {
                        context.attachPannerToVoice(sourceId, "", 0, 0);
                    } else if (node instanceof AudioBiquadNode) {
                        context.attachBiquadToVoice(sourceId, "", 0, 0);
                    } else if (!context.isDestinationNode(destId)) {
                        context.attachGainToVoice(sourceId, "", 0, 0);
                    }
                    context.unregisterNodeSource(destId, sourceId);
                }
                if (sources.isEmpty() && !context.isDestinationNode(destId)) {
                    context.attachGainToVoice(destId, "", 0, 0);
                    context.unregisterNodeSource(destId, id);
                }
                return;
            }
        }
        disconnect();
    }

    @Override
    public void disconnect() {
        AudioContext context = AudioContext.getInstance();
        context.detachGain(id);
        context.clearNodeSources(id);
    }

    @Override
    public void disconnect(int output) {
        disconnect();
    }

    @Override
    public void disconnect(AudioNode node, int output) {
        disconnect(node);
    }

    @Override
    public void disconnect(AudioNode node, int output, int input) {
        disconnect(node);
    }

    @Override
    public void release() {
        AudioContext.getInstance().releaseGain(id);
    }
}
