package org.nativescript.audiocontext;

import java.lang.ref.PhantomReference;
import java.lang.ref.ReferenceQueue;
import java.util.concurrent.ConcurrentHashMap;

public class NativeFinalizer {
	public enum ResourceKind {BUFFER, GAIN, BIQUAD, PANNER, IIR, PERIODICWAVE}

	private static final ReferenceQueue<Object> queue = new ReferenceQueue<>();
	private static final ConcurrentHashMap<PhantomReference<Object>, ResourceDescriptor> refs = new ConcurrentHashMap<>();

	static {
		Thread t = new Thread(() -> {
			while (true) {
				try {
					@SuppressWarnings("unchecked")
					PhantomReference<Object> pr = (PhantomReference<Object>) queue.remove();
					ResourceDescriptor d = refs.remove(pr);
					if (d != null) {
						try {
							switch (d.kind) {
								case BUFFER:
									AudioContext.getInstance().releaseBuffer(d.id);
									break;
								case GAIN:
									AudioContext.getInstance().releaseGain(d.id);
									break;
								case BIQUAD:
									AudioContext.getInstance().releaseBiquad(d.id);
									break;
								case PANNER:
									AudioContext.getInstance().releasePanner(d.id);
									break;
								case IIR:
									AudioContext.getInstance().releaseIIR(d.id);
									break;
								case PERIODICWAVE:
									AudioContext.getInstance().releasePeriodicWave(d.id);
									break;
							}
						} catch (Throwable ignored) {
						}
					}
				} catch (InterruptedException ex) {
					break;
				}
			}
		}, "NativeFinalizer");
		t.setDaemon(true);
		t.start();
	}

	public static void register(Object wrapper, ResourceKind kind, String id) {
		if (wrapper == null || id == null) return;
		PhantomReference<Object> pr = new PhantomReference<>(wrapper, queue);
		refs.put(pr, new ResourceDescriptor(kind, id));
	}

	private static class ResourceDescriptor {
		final ResourceKind kind;
		final String id;

		ResourceDescriptor(ResourceKind k, String id) {
			this.kind = k;
			this.id = id;
		}
	}
}
