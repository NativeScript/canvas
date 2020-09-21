package com.github.triniwiz.canvas;

import java.util.ArrayList;
import java.util.Collection;
import java.util.HashMap;
import java.util.Map;
import java.util.Set;

/**
 * Created by triniwiz on 5/30/20
 */
public class Gradient implements ICanvasColorStyle {
    Map<Float, Integer> gradientMap;
    ArrayList<Integer> colors;
    ArrayList<Float> keys;

    Gradient() {
        this.gradientMap = new HashMap<>();
        this.colors = new ArrayList<>();
        this.keys = new ArrayList<>();
    }

    @Override
    public CanvasColorStyleType getStyleType() {
        return CanvasColorStyleType.Gradient;
    }

    public void addColorStop(float offset, int color) throws Exception {
        if (offset < 0) {
            throw new Exception("INDEX_SIZE_ERR");
        }

        if (offset > 1) {
            throw new Exception("INDEX_SIZE_ERR");
        }
        this.gradientMap.put(offset, color);
        this.colors.add(color);
        this.keys.add(offset);
    }

    int[] getColors() {
        Collection<Integer> valuesCollection = gradientMap.values();
        int[] rawValues = new int[valuesCollection.size()];
        int i = 0;
        for (Integer value : valuesCollection) {
            rawValues[i] = value;
            i++;
        }
        return rawValues;
    }

    float[] getPositions() {
        Set<Float> keysSet = gradientMap.keySet();
        float[] rawKeys = new float[keysSet.size()];

        int i = 0;
        for (Float key : keysSet) {
            rawKeys[i] = key;
            i++;
        }
        return rawKeys;
    }

}
