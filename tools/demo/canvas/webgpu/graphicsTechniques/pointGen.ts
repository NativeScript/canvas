// Constants for calculations
const SQRT_3 = Math.sqrt(3);
const A = 1.0 / Math.SQRT2;
const B = SQRT_3 * A;
const S45 = Math.SQRT1_2;
const C45 = S45;

// Represents the attributes of a terrain vertex
export class TerrainVertexAttributes {
	position: Float32Array;
	normal: Float32Array;
	colour: Uint8Array;
	constructor(position: Float32Array, normal: Float32Array, colour: Uint8Array) {
		this.position = position; // Array of 3 floats
		this.normal = normal; // Array of 3 floats
		this.colour = colour; // Array of 4 bytes (uint8)
	}
}

// Represents the attributes of a water vertex
export class WaterVertexAttributes {
	position: Int16Array;
	offsets: Int8Array;
	constructor(position, offsets) {
		this.position = position; // Array of 2 int16
		this.offsets = offsets; // Array of 4 int8
	}
}

// Represents the center of a single hexagon
export class TerrainVertex {
	position: Float32Array;
	colour: Uint8Array;
	constructor(position: Float32Array, colour: Uint8Array) {
		this.position = position; // Assume this is a vector (like in Rust)
		this.colour = colour; // Array of 4 bytes (uint8)
	}
}

// Function to get the surrounding hexagonal points from a point
function surroundingHexagonalPoints(x: number, y: number): [number, number][] {
	return [
		[x, y - 1],
		[x + 1, y - 1],
		[x + 1, y],
		[x, y + 1],
		[x - 1, y + 1],
		[x - 1, y],
	];
}

function add(dst: Float32Array, source: Float32Array): Float32Array {
	if (dst.length !== source.length) {
		throw new Error('Arrays must be of the same length');
	}

	for (let i = 0; i < dst.length; i++) {
		dst[i] += source[i];
	}

	return dst;
}

function div(dst: Float32Array, scalar: number): Float32Array {
	if (scalar === 0) {
		throw new Error('Division by zero is not allowed');
	}

	for (let i = 0; i < dst.length; i++) {
		dst[i] /= scalar;
	}

	return dst;
}

function sub(dst: Float32Array, source: Float32Array): Float32Array {
	if (dst.length !== source.length) {
		throw new Error('Arrays must be of the same length');
	}

	for (let i = 0; i < dst.length; i++) {
		dst[i] -= source[i];
	}

	return dst;
}

function normalize(dst: Float32Array): Float32Array {
	const len = Math.hypot(...Array.from(dst));
	const ret = new Float32Array(dst.length);
	if (len > 0) {
		for (let i = 0; i < ret.length; i++) {
			ret[i] -= ret[i] / len;
		}
	}
	return ret;
}

function cross3(dst: Float32Array, other: Float32Array): Float32Array {
	return new Float32Array([dst[1] * other[2] - dst[2] * other[1], dst[2] * other[0] - dst[0] * other[2], dst[0] * other[1] - dst[1] * other[0]]);
}

// Function to iterate over surrounding point values
function surroundingPointValuesIter<T>(hashmap: Map<[number, number], T>, x: number, y: number, forEach: (value: [T, T]) => void) {
	let points = surroundingHexagonalPoints(x, y);
	points.push(points[0]); // to make the array cyclic

	for (let i = 0; i < points.length - 1; i++) {
		let a = hashmap.get(points[i]);
		let b = hashmap.get(points[i + 1]);
		if (a && b) {
			forEach([a, b]);
		}
	}
}

// Function to calculate the normal vector given three points
function calculateNormal(a: Float32Array, b: Float32Array, c: Float32Array) {
	return normalize(cross3(normalize(sub(b, a)), normalize(sub(c, a))));
}

// Given the radius, how large of a square do we need to make a unit hexagon grid?
function qGivenR(radius) {
	return Math.floor(Math.floor((4.0 * radius) / SQRT_3 + 1.0) / 2.0) * 2 + 1;
}

// Represents terrain, containing the vertices only once
export class HexTerrainMesh {
	vertices: Map<[number, number], TerrainVertex>;
	halfSize: number;
	constructor(vertices, halfSize) {
		this.vertices = vertices; // Map of (isize, isize) -> TerrainVertex
		this.halfSize = halfSize; // isize
	}

	// Generates the vertices (or the centers of the hexagons)
	static generate(radius, genVertex) {
		let width = qGivenR(radius);
		let halfWidth = Math.floor(width / 2);
		let map = new Map();
		let max = -Infinity;

		for (let i = -halfWidth; i <= halfWidth; i++) {
			let x_o = i;
			for (let j = -halfWidth; j <= halfWidth; j++) {
				let y_o = j;
				let x = A * (x_o * C45 - y_o * S45);
				let z = B * (x_o * S45 + y_o * C45);

				if (Math.hypot(x, z) < radius) {
					let vertex = genVertex([x, z]);
					if (vertex.position.y > max) {
						max = vertex.position.y;
					}
					map.set([i, j], vertex);
				}
			}
		}

		return new HexTerrainMesh(map, halfWidth);
	}

	// Creates the points required to render the mesh
	makeBufferData() {
		let vertices: TerrainVertexAttributes[] = [];

		const middle = (p1: TerrainVertex, p2: TerrainVertex, p: TerrainVertex) => {
			return div(add(add(p1.position, p2.position), p.position), 3);
		};
		const half = (p1: TerrainVertex, p2: TerrainVertex) => {
			return div(add(p1.position, p2.position), 2);
		};

		const pushTriangle = (p1: TerrainVertex, p2: TerrainVertex, p: TerrainVertex, c: Uint8Array) => {
			let m = middle(p1, p2, p);
			let ap = half(p1, p);
			let bp = half(p2, p);
			let n1 = calculateNormal(ap, m, p.position);
			let n2 = calculateNormal(m, bp, p.position);

			vertices.push(
				...[ap, m, p.position, m, bp, p.position].map(
					(pos, i) =>
						new TerrainVertexAttributes(
							pos,
							i < 3 ? n1 : n2, // Select normal
							c // Colour
						)
				)
			);
		};

		for (let i = -this.halfSize; i <= this.halfSize; i++) {
			for (let j = -this.halfSize; j <= this.halfSize; j++) {
				let p = this.vertices.get([i, j]);
				if (p) {
					surroundingPointValuesIter<TerrainVertex>(this.vertices, i, j, ([a, b]) => {
						pushTriangle(a, b, p, p.colour);
					});
				}
			}
		}

		return vertices;
	}
}

// Represents water mesh which contains vertex data for the water mesh
export class HexWaterMesh {
	vertices: Map<[number, number], Int16Array>;
	halfSize: number;
	constructor(vertices, halfSize) {
		this.vertices = vertices; // Map of (isize, isize) -> [i16; 2]
		this.halfSize = halfSize; // isize
	}

	static generate(radius) {
		let width = qGivenR(radius);
		let halfWidth = Math.floor(width / 2);
		let map = new Map();

		for (let i = -halfWidth; i <= halfWidth; i++) {
			let x_o = i;
			for (let j = -halfWidth; j <= halfWidth; j++) {
				let y_o = j;
				let x = A * (x_o * C45 - y_o * S45);
				let z = B * (x_o * S45 + y_o * C45);

				if (Math.hypot(x, z) < radius) {
					x = Math.round(x * 2.0);
					z = Math.round((z / B) * Math.SQRT2);
					map.set([i, j], [x, z]);
				}
			}
		}

		return new HexWaterMesh(map, halfWidth);
	}

	generatePoints() {
		let vertices: WaterVertexAttributes[] = [];

		const calculateDifferences = (a, b, c) => {
			return new Uint8Array([b[0] - a[0], b[1] - a[1], c[0] - a[0], c[1] - a[1]]);
		};

		const pushTriangle = (a, b, c) => {
			let bc = calculateDifferences(a, b, c);
			let ca = calculateDifferences(b, c, a);
			let ab = calculateDifferences(c, a, b);

			vertices.push(...[a, b, c].map((position, i) => new WaterVertexAttributes(position, i === 0 ? bc : i === 1 ? ca : ab)));
		};

		for (let i = -this.halfSize; i <= this.halfSize; i++) {
			for (let j = -this.halfSize; j <= this.halfSize; j++) {
				if ((i - j) % 3 === 0) {
					let p = this.vertices.get([i, j]);
					if (p) {
						surroundingPointValuesIter(this.vertices, i, j, ([a, b]) => {
							pushTriangle(a, b, p);
						});
					}
				}
			}
		}

		return vertices;
	}
}
