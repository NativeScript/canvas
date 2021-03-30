import { Property } from "@nativescript/core";
import { Path2D, CanvasGradient } from "../Canvas2D";
import { SVGItem } from "./SVGItem";

export const dProperty = new Property<Path, string>({
	name: 'd'
});

export class Path extends SVGItem {
	public d: string;
}

dProperty.register(Path);
