import { AddChildFromBuilder, View } from "@nativescript/core";
import { Canvas } from "../Canvas";
import { Svg } from "./SVG";
import { SVGItem } from "./SVGItem";

export class G extends SVGItem implements AddChildFromBuilder {
    _views: any[];
    _children: Map<string, View>;
    #viewPostion: Map<number, string>;
    transform: string;
    x: any;
    y: any;

    constructor() {
        super();
        this.#viewPostion = new Map();
        this._views = [];
        this._children = new Map<string, View>();
    }

    handleValues(canvas) {
        this._canvas = canvas;
        const ctx = this._canvas.getContext('2d') as any;
        ctx.save();
        if (typeof this.transform === 'string') {
            // TODO use regex
            const matrix = this.transform.replace('matrix(', '')
                .replace('(', '')
                .split(',')
                .map(value => {
                    const val = parseFloat(value);
                    if (isNaN(val)) {
                        return 0;
                    }
                    return val;
                });
            ctx.transform(matrix[0], matrix[1], matrix[2], matrix[3], matrix[4], matrix[5]);
        }

        const updateColors = () => {
            if (this._doStroke()) {
                ctx.strokeStyle = this._realStroke;
            }
            if (this._doFill()) {
                if (this.fill === undefined) {
                    // ctx.fillStyle = 'black';
                } else {
                    ctx.fillStyle = this._realFill;
                }
            }
        };
        this._views.forEach(view => {
            if (typeof view.handleValues === 'function') {
                updateColors();
                ctx.globalAlpha = this.opacity;
                view.handleValues(this._canvas);
            } else if (view instanceof G) {
                view._views.forEach((view) => {
                    updateColors();
                    view.handleValues();
                });
            }
        });
        ctx.restore();
    }

    _refresh() {
        if (this.parent && this.parent instanceof Svg) {
            this._views.forEach((view) => {
                if (typeof view.handleValues === 'function') {
                    view.handleValues(this._canvas);
                }
            })
        }
    }
    _forceRedraw() {
        if (this.parent && this.parent instanceof Svg) {
            const ctx = this._canvas.getContext('2d');
            this._views.forEach((view) => {
                if (typeof view.handleValues === 'function') {
                    view.handleValues(this._canvas);
                }
            });
        }
    }

    eachChildView(callback: (child: View) => boolean) {
        this._children.forEach((view, key) => {
            callback(view);
        });
    }


    public _addChildFromBuilder(name: string, value: any) {
        if (value instanceof View) {
            this.addChild(value);
        }
    }

    getChildrenCount(): number {
        return this._children.size;
    }

    // overrides the base property.
    get _childrenCount(): number {
        return this._children.size;
    }

    getChildAt(index: number): View {
        return this._views[index];
    }

    getChildIndex(child: View): number {
        return this._views.indexOf(child);
    }

    public getChildById(id: string) {
        return this._getViewById(id);
    }

    public _registerLayoutChild(child: View) {
        //Overridden
    }

    public _unregisterLayoutChild(child: View) {
        //Overridden
    }

    public addChildren(...children: View[]) {
		children.forEach(child => {
			this._views.push(child);
			this._children.set(`${child.id || child._domId}`, child);
			this._addView(child);
			this._registerLayoutChild(child);
		});
		this._refresh();
	}

    public addChild(child: View): void {
        this._views.push(child);
        this._children.set(`${child.id || child._domId}`, child);
        this._addView(child);
        this._registerLayoutChild(child);
        this._refresh();
    }

    public insertChild(child: View, atIndex: number): void {
        this._views.splice(atIndex, 0, child);
        this._children.set(`${child.id || child._domId}`, child);
        this._addView(child, atIndex);
        this._registerLayoutChild(child);
        this._refresh();
    }

    public removeChild(child: View): void {
        this._removeView(child);
        // TODO: consider caching the index on the child.
        const index = this._views.indexOf(child);
        this._views.splice(index, 1);
        this._children.delete(`${child.id || child._domId}`)
        this._unregisterLayoutChild(child);
        if (!this.#isRemoving) {
            this._refresh();
        }
    }

    #isRemoving: boolean = false;
    public removeChildren(): void {
        this.#isRemoving = true;
        while (this.getChildrenCount() !== 0) {
            this.removeChild(this._views[this.getChildrenCount() - 1]);
        }
        if (this.#isRemoving) {
            this._refresh();
        }
        this.#isRemoving = false;
    }
}
