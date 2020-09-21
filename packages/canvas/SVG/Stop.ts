import {SVGItem} from "./SVGItem";

export class Stop extends SVGItem {
    offset: any;

    get stopColor(): any {
        return (this.style as any).stopColor;
    }

    set stopColor(value) {
        (this.style as any).stopColor = value;
    }

    _realOffset() {
        if (typeof this.offset === 'string') {
            if (this.offset.indexOf('%') > -1) {
                const offset = parseInt(this.offset.replace('%', ''));
                return !isNaN(offset) ? offset / 100 : 0;
            } else {
                return parseFloat(this.offset);
            }
        }
        return this.offset;
    }
}