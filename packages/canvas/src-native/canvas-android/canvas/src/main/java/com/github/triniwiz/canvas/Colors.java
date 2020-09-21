package com.github.triniwiz.canvas;

import android.graphics.Color;

import java.util.HashMap;

/**
 * Created by triniwiz on 3/27/20
 */
class Colors {
    static HashMap<String, String> hexColorMap = new HashMap<>();

    static {
        String allColors = "aliceblue:#F0F8FFFF," +
                "antiquewhite:#FAEBD7FF," +
                "aqua:#00FFFFFF," +
                "aquamarine:#7FFFD4FF," +
                "azure:#F0FFFFFF," +
                "beige:#F5F5DCFF," +
                "bisque:#FFE4C4FF," +
                "black:#000000FF," +
                "blanchedalmond:#FFEBCDFF," +
                "blue:#0000FFFF," +
                "blueviolet:#8A2BE2FF," +
                "brown:#A52A2AFF," +
                "burlywood:#DEB887FF," +
                "cadetblue:#5F9EA0FF," +
                "chartreuse:#7FFF00FF," +
                "chocolate:#D2691EFF," +
                "coral:#FF7F50FF," +
                "cornflowerblue:#6495EDFF," +
                "cornsilk:#FFF8DCFF," +
                "crimson:#DC143CFF," +
                "cyan:#00FFFFFF," +
                "darkblue:#00008BFF," +
                "darkcyan:#008B8BFF," +
                "darkgoldenrod:#B8860BFF," +
                "darkgray:#A9A9A9FF," +
                "darkgrey:#A9A9A9FF," +
                "darkgreen:#006400FF," +
                "darkkhaki:#BDB76BFF," +
                "darkmagenta:#8B008BFF," +
                "darkolivegreen:#556B2FFF," +
                "darkorange:#FF8C00FF," +
                "darkorchid:#9932CCFF," +
                "darkred:#8B0000FF," +
                "darksalmon:#E9967AFF," +
                "darkseagreen:#8FBC8FFF," +
                "darkslateblue:#483D8BFF," +
                "darkslategray:#2F4F4FFF," +
                "darkslategrey:#2F4F4FFF," +
                "darkturquoise:#00CED1FF," +
                "darkviolet:#9400D3FF," +
                "deeppink:#FF1493FF," +
                "deepskyblue:#00BFFFFF," +
                "dimgray:#696969FF," +
                "dimgrey:#696969FF," +
                "dodgerblue:#1E90FFFF," +
                "firebrick:#B22222FF," +
                "floralwhite:#FFFAF0FF," +
                "forestgreen:#228B22FF," +
                "fuchsia:#FF00FFFF," +
                "gainsboro:#DCDCDCFF," +
                "ghostwhite:#F8F8FFFF," +
                "gold:#FFD700FF," +
                "goldenrod:#DAA520FF," +
                "gray:#808080FF," +
                "grey:#808080FF," +
                "green:#008000FF," +
                "greenyellow:#ADFF2FFF," +
                "honeydew:#F0FFF0FF," +
                "hotpink:#FF69B4FF," +
                "indianred:#CD5C5CFF," +
                "indigo:#4B0082FF," +
                "ivory:#FFFFF0FF," +
                "khaki:#F0E68CFF," +
                "lavender:#E6E6FAFF," +
                "lavenderblush:#FFF0F5FF," +
                "lawngreen:#7CFC00FF," +
                "lemonchiffon:#FFFACDFF," +
                "lightblue:#ADD8E6FF," +
                "lightcoral:#F08080FF," +
                "lightcyan:#E0FFFFFF," +
                "lightgoldenrodyellow:#FAFAD2FF," +
                "lightgray:#D3D3D3FF," +
                "lightgrey:#D3D3D3FF," +
                "lightgreen:#90EE90FF," +
                "lightpink:#FFB6C1FF," +
                "lightsalmon:#FFA07AFF," +
                "lightseagreen:#20B2AAFF," +
                "lightskyblue:#87CEFAFF," +
                "lightslategray:#778899FF," +
                "lightslategrey:#778899FF," +
                "lightsteelblue:#B0C4DEFF," +
                "lightyellow:#FFFFE0FF," +
                "lime:#00FF00FF," +
                "limegreen:#32CD32FF," +
                "linen:#FAF0E6FF," +
                "magenta:#FF00FFFF," +
                "maroon:#800000FF," +
                "mediumaquamarine:#66CDAAFF," +
                "mediumblue:#0000CDFF," +
                "mediumorchid:#BA55D3FF," +
                "mediumpurple:#9370D8FF," +
                "mediumseagreen:#3CB371FF," +
                "mediumslateblue:#7B68EEFF," +
                "mediumspringgreen:#00FA9AFF," +
                "mediumturquoise:#48D1CCFF," +
                "mediumvioletred:#C71585FF," +
                "midnightblue:#191970FF," +
                "mintcream:#F5FFFAFF," +
                "mistyrose:#FFE4E1FF," +
                "moccasin:#FFE4B5FF," +
                "navajowhite:#FFDEADFF," +
                "navy:#000080FF," +
                "oldlace:#FDF5E6FF," +
                "olive:#808000FF," +
                "olivedrab:#6B8E23FF," +
                "orange:#FFA500FF," +
                "orangered:#FF4500FF," +
                "orchid:#DA70D6FF," +
                "palegoldenrod:#EEE8AAFF," +
                "palegreen:#98FB98FF," +
                "paleturquoise:#AFEEEEFF," +
                "palevioletred:#D87093FF," +
                "papayawhip:#FFEFD5FF," +
                "peachpuff:#FFDAB9FF," +
                "peru:#CD853FFF," +
                "pink:#FFC0CBFF," +
                "plum:#DDA0DDFF," +
                "powderblue:#B0E0E6FF," +
                "purple:#800080FF," +
                "rebeccapurple:#663399FF," +
                "red:#FF0000FF," +
                "rosybrown:#BC8F8FFF," +
                "royalblue:#4169E1FF," +
                "saddlebrown:#8B4513FF," +
                "salmon:#FA8072FF," +
                "sandybrown:#F4A460FF," +
                "seagreen:#2E8B57FF," +
                "seashell:#FFF5EEFF," +
                "sienna:#A0522DFF," +
                "silver:#C0C0C0FF," +
                "skyblue:#87CEEBFF," +
                "slateblue:#6A5ACDFF," +
                "slategray:#708090FF," +
                "slategrey:#708090FF," +
                "snow:#FFFAFAFF," +
                "springgreen:#00FF7FFF," +
                "steelblue:#4682B4FF," +
                "tan:#D2B48CFF," +
                "teal:#008080FF," +
                "thistle:#D8BFD8FF," +
                "tomato:#FF6347FF," +
                "turquoise:#40E0D0FF," +
                "violet:#EE82EEFF," +
                "wheat:#F5DEB3FF," +
                "white:#FFFFFFFF," +
                "whitesmoke:#F5F5F5FF," +
                "yellow:#FFFF00FF," +
                "yellowgreen:#9ACD32FF";
        String[] colors = allColors.split(",");
        for (String color : colors) {
            String[] keyValue = color.split(":");
            hexColorMap.put(keyValue[0], keyValue[1]);
        }
    }

    static int getColor(String color) {
        if (color.startsWith("#")) {
            String hex = color.replace("#", "");
            if (hex.length() == 3) {
                hex = "" + hex.charAt(0) + hex.charAt(0) + hex.charAt(1) + hex.charAt(1) + hex.charAt(2) + hex.charAt(2);
            } else if (hex.length() == 4) {
                hex = "" + hex.charAt(0) + hex.charAt(0) + hex.charAt(1) + hex.charAt(1) + hex.charAt(2) + hex.charAt(2) + hex.charAt(3) + hex.charAt(3);
            }

            if (hex.length() == 8) {
                // move r to front .... html uses rgba android uses argb
                String gb = hex.substring(2, 6);
                hex = hex.substring(6, 8) + hex.substring(0, 2) + gb;
            }
            hex = "#" + hex;
            return Color.parseColor(hex);
        } else if (color.startsWith("rgba")) {
            String[] rgba = color.replace("rgba(", "")
                    .replace(")", "")
                    .split(",");
            int A = (int) (Float.parseFloat(rgba[3]) * 255);
            int R = Integer.parseInt(rgba[0].trim());
            int G = Integer.parseInt(rgba[1].trim());
            int B = Integer.parseInt(rgba[2].trim());
            return (A & 0xff) << 24 | (R & 0xff) << 16 | (G & 0xff) << 8 | (B & 0xff);
        } else if (color.startsWith("rgb")) {
            String[] rgb = color.replace("rgb(", "")
                    .replace(")", "")
                    .split(",");
            int A = 255;
            int R = Integer.parseInt(rgb[0].trim());
            int G = Integer.parseInt(rgb[1].trim());
            int B = Integer.parseInt(rgb[2].trim());
            return (A & 0xff) << 24 | (R & 0xff) << 16 | (G & 0xff) << 8 | (B & 0xff);
        } else {
            String hex = Colors.hexColorMap.get(color.toLowerCase());
            if (hex == null) {
                hex = "#000000FF";
            }
            return Colors.getColor(hex);
        }
    }
}
