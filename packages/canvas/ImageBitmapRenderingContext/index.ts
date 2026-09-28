import type { Canvas } from '../Canvas';
import { ImageBitmap } from '../ImageBitmap';

/**
 * `canvas.getContext('bitmaprenderer')`.
 *
 * The spec transfers the bitmap; a native surface cannot be re-pointed at
 * foreign pixels, so the transfer is a blit into the canvas's own surface.
 * Everything observable still matches: the canvas resizes, the bitmap detaches,
 * a detached bitmap throws.
 */
export class ImageBitmapRenderingContext {
	_type = 'bitmaprenderer';

	private _owner: Canvas;
	private _context: any;
	private _alpha: boolean;

	private _mode: 'blank' | 'valid' = 'blank';

	/** Held until the next transfer: the blit may not have rasterised yet. */
	private _retained: any = null;

	constructor(owner: Canvas, context: any, options?: { alpha?: boolean }) {
		this._owner = owner;
		this._context = context;
		this._alpha = options?.alpha !== false;
		this._paintBlank();
	}

	get canvas(): Canvas {
		return this._owner;
	}

	/** How toDataURL, drawImage(canvas) and createPattern(canvas) reach the surface. */
	get native() {
		return this._context?.native ?? this._context;
	}

	getContextAttributes() {
		return { alpha: this._alpha };
	}

	transferFromImageBitmap(bitmap: ImageBitmap | null | undefined): void {
		if (bitmap === null || bitmap === undefined) {
			this._mode = 'blank';
			this._retained = null;
			this._paintBlank();
			return;
		}

		if (!(bitmap instanceof ImageBitmap)) {
			throw new TypeError("Failed to execute 'transferFromImageBitmap' on 'ImageBitmapRenderingContext': parameter 1 is not of type 'ImageBitmap'.");
		}

		if ((bitmap as any).__detached) {
			const error: any = new Error("Failed to execute 'transferFromImageBitmap' on 'ImageBitmapRenderingContext': The input ImageBitmap has been detached.");
			error.name = 'InvalidStateError';
			throw error;
		}

		const width = bitmap.width;
		const height = bitmap.height;

		// Set back to back so the pair coalesces into one resize.
		const canvas = this._owner as any;
		canvas.width = width;
		canvas.height = height;

		const ctx = this._context;
		ctx.save();
		ctx.resetTransform();
		ctx.globalAlpha = 1;
		ctx.globalCompositeOperation = 'copy';
		ctx.filter = 'none';
		ctx.shadowColor = 'rgba(0, 0, 0, 0)';
		ctx.imageSmoothingEnabled = false;
		if (!this._alpha) {
			ctx.fillStyle = '#000000';
			ctx.fillRect(0, 0, width, height);
			ctx.globalCompositeOperation = 'source-over';
		}
		ctx.drawImage(bitmap as any, 0, 0);
		ctx.restore();

		this._retained = (bitmap as any).native;
		this._mode = 'valid';
		(bitmap as any).__detach();
	}

	/** Legacy name for transferFromImageBitmap. */
	transferImageBitmap(bitmap: ImageBitmap | null): void {
		this.transferFromImageBitmap(bitmap);
	}

	private _paintBlank() {
		const ctx = this._context;
		if (!ctx) {
			return;
		}
		const width = (this._owner as any).width;
		const height = (this._owner as any).height;
		ctx.save();
		ctx.resetTransform();
		ctx.globalAlpha = 1;
		ctx.globalCompositeOperation = 'copy';
		ctx.filter = 'none';
		if (this._alpha) {
			ctx.clearRect(0, 0, width, height);
		} else {
			ctx.fillStyle = '#000000';
			ctx.fillRect(0, 0, width, height);
		}
		ctx.restore();
	}
}
