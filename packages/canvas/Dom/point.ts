export interface Point {
	x: number;
	y: number;
}

/** `{x, y}`, `"x,y"` / `"x y"`, or one number for both, as vec() takes it. */
export function parsePoint(value: any): Point | null {
	if (value === undefined || value === null) return null;
	if (typeof value === 'object' && 'x' in value) {
		const x = Number(value.x) || 0;
		return { x, y: 'y' in value ? Number(value.y) || 0 : x };
	}
	if (typeof value === 'number') {
		return Number.isNaN(value) ? null : { x: value, y: value };
	}
	if (typeof value === 'string') {
		const parts = value.split(/[\s,]+/).filter(Boolean).map(parseFloat);
		if (parts.length === 0 || parts.some(Number.isNaN)) return null;
		return { x: parts[0], y: parts[1] ?? parts[0] };
	}
	return null;
}

/** An array of points, JSON, or `"x,y x,y ..."` as in SVG's points attribute. */
export function parsePoints(value: any): Point[] {
	if (typeof value === 'string') {
		const text = value.trim();
		if (text.startsWith('[')) {
			try {
				return parsePoints(JSON.parse(text));
			} catch (e) {
				return [];
			}
		}
		const numbers = text.split(/[\s,]+/).filter(Boolean).map(parseFloat);
		const points: Point[] = [];
		for (let i = 0; i + 1 < numbers.length; i += 2) {
			if (!Number.isNaN(numbers[i]) && !Number.isNaN(numbers[i + 1])) {
				points.push({ x: numbers[i], y: numbers[i + 1] });
			}
		}
		return points;
	}
	if (Array.isArray(value)) {
		return value.map(parsePoint).filter(Boolean);
	}
	return [];
}
