import {SVGItem} from "./SVGItem";
import {Canvas} from "../Canvas";
import {AddChildFromBuilder, PercentLength} from "@nativescript/core";
import {Svg} from "./SVG";
import { DOMParser } from 'xmldom';
export class Pattern extends SVGItem {
	constructor(){
		super();
		this._dom = new DOMParser().parseFromString('<pattern></pattern>');
	}
}
