import {G} from "./G";
import {Svg} from "./SVG";
import {SVGItem} from "./SVGItem";
import {Symbol} from './Symbol';
import {Circle} from "./Circle";
import {Rect} from './Rect';
import { DOMParser } from 'xmldom';
export class Use extends SVGItem {
    constructor(){
		super();
		this._dom = new DOMParser().parseFromString('<use></use>');
	}
}
