import type { DrawableController } from './common';

export * from './common';

/** Node-API hosts have no native child views to draw yet. */
export function createDrawableController(canvas: any): DrawableController | null {
	return null;
}
