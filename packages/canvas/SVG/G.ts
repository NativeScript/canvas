import { AddChildFromBuilder, View } from "@nativescript/core";
import { Canvas } from "../Canvas";
import { Svg } from "./SVG";
import { SVGItem } from "./SVGItem";

export class G extends SVGItem  {
    transform: string;
    x: any;
    y: any;
}
