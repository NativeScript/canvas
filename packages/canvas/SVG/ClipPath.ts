import {SVGItem} from "./SVGItem";
import { DOMParser } from 'xmldom';
export class ClipPath extends SVGItem {
    constructor(){
		super();
		this._dom = new DOMParser().parseFromString('<clip-path></clip-path>');
	}
}