import * as THREE from 'three/webgpu';
import type { Canvas } from '@nativescript/canvas';

/**
 * The same yard of parts drawn three ways on WebGPURenderer, to compare draw-call cost:
 * - `meshes`: one Mesh per part (one draw, bind group and uniform upload each);
 * - `batched`: one BatchedMesh with per-instance colours (one draw per visible part, no state
 *   changes between them);
 * - `instanced`: one InstancedMesh per geometry (one draw per geometry).
 */
export type DrawMode = 'meshes' | 'batched' | 'instanced';

export interface BatchedMeshStats {
	mode: DrawMode;
	count: number;
	fps: number;
	draws: number;
	triangles: number;
	/** Median renderer.render() time over the last second, in ms. */
	renderMs: number;
}

export interface BatchedMeshOptions {
	/** Parts in the yard. */
	count?: number;
	mode?: DrawMode;
	/** Once a second. */
	onStats?: (stats: BatchedMeshStats) => void;
}

export interface BatchedMeshDemo {
	setMode(mode: DrawMode): void;
	setCount(count: number): void;
	dispose(): void;
}

/** BatchedMesh needs every geometry indexed: the polyhedra are not. */
function indexed(geometry: THREE.BufferGeometry) {
	if (!geometry.index) {
		const count = geometry.attributes.position.count;
		const index = new Uint32Array(count);
		for (let i = 0; i < count; i++) index[i] = i;
		geometry.setIndex(new THREE.BufferAttribute(index, 1));
	}
	return geometry;
}

const GEOMETRIES = () => [new THREE.BoxGeometry(1, 1, 1), new THREE.SphereGeometry(0.6, 10, 8), new THREE.TorusKnotGeometry(0.4, 0.15, 32, 6), new THREE.CylinderGeometry(0.4, 0.5, 1, 10), new THREE.ConeGeometry(0.5, 1, 10), new THREE.IcosahedronGeometry(0.6, 0), new THREE.TorusGeometry(0.5, 0.2, 8, 16), new THREE.DodecahedronGeometry(0.6), new THREE.OctahedronGeometry(0.6), new THREE.CapsuleGeometry(0.3, 0.6, 2, 8), new THREE.TetrahedronGeometry(0.7), new THREE.RingGeometry(0.2, 0.6, 12)].map(indexed);

const PALETTE = [0xf97316, 0x22d3ee, 0xa3e635, 0xe879f9, 0xfacc15, 0x60a5fa, 0xf87171, 0x34d399];

/** Where part `i` sits: a spiral that spreads outwards as the yard grows. */
function placement(i: number, out: THREE.Object3D) {
	const ring = Math.sqrt(i);
	const angle = i * 2.399;
	out.position.set(Math.cos(angle) * ring * 1.6, ((i % 7) - 3) * 2, Math.sin(angle) * ring * 1.6 - 10);
	out.rotation.set(0, 0, 0);
	out.scale.setScalar(0.8 + (i % 5) * 0.1);
}

export async function webgpu_batched_mesh(canvas: Canvas, options: BatchedMeshOptions = {}): Promise<BatchedMeshDemo> {
	let count = options.count ?? 2048;
	let mode: DrawMode = options.mode ?? 'batched';

	const scale = window.devicePixelRatio;
	const width = canvas.clientWidth;
	const height = canvas.clientHeight;
	canvas.width = width * scale;
	canvas.height = height * scale;

	const renderer = new THREE.WebGPURenderer({ canvas: canvas as never, antialias: true });
	renderer.setPixelRatio(scale);
	renderer.setSize(width, height, false);
	await renderer.init();

	const scene = new THREE.Scene();
	scene.background = new THREE.Color(0x101820);
	const camera = new THREE.PerspectiveCamera(60, width / height, 0.1, 500);
	camera.position.set(0, 14, 44);
	camera.lookAt(0, 0, 0);
	scene.add(new THREE.HemisphereLight(0xffffff, 0x404060, 1.2));
	const sun = new THREE.DirectionalLight(0xffffff, 1.5);
	sun.position.set(10, 20, 10);
	scene.add(sun);

	const geometries = GEOMETRIES();
	const materials = PALETTE.map((color) => new THREE.MeshStandardMaterial({ color, metalness: 0.3, roughness: 0.6 }));
	// One material for the batched and instanced parts: their colour is per instance.
	const shared = new THREE.MeshStandardMaterial({ color: 0xffffff, metalness: 0.3, roughness: 0.6 });
	const colors = PALETTE.map((color) => new THREE.Color(color));

	const dummy = new THREE.Object3D();
	const matrix = new THREE.Matrix4();
	let group: THREE.Group | null = null;
	let batched: THREE.BatchedMesh | null = null;
	let instanced: THREE.InstancedMesh[] = [];
	/** `instanced` only: the index of part `i` inside its geometry's mesh. */
	let slot: Uint32Array = new Uint32Array(0);

	function clear() {
		if (group) {
			scene.remove(group);
			group = null;
		}
		if (batched) {
			scene.remove(batched);
			batched.dispose();
			batched = null;
		}
		for (const mesh of instanced) {
			scene.remove(mesh);
			mesh.dispose();
		}
		instanced = [];
	}

	function build() {
		clear();
		if (mode === 'meshes') {
			group = new THREE.Group();
			for (let i = 0; i < count; i++) {
				const mesh = new THREE.Mesh(geometries[i % geometries.length], materials[i % materials.length]);
				placement(i, mesh);
				group.add(mesh);
			}
			scene.add(group);
		} else if (mode === 'batched') {
			let vertices = 0;
			let indices = 0;
			for (const geometry of geometries) {
				vertices += geometry.attributes.position.count;
				indices += geometry.index ? geometry.index.count : 0;
			}
			batched = new THREE.BatchedMesh(count, vertices, indices, shared);
			// Sorting per frame costs JS time per part; the parts are opaque, so skip it.
			batched.sortObjects = false;
			const ids = geometries.map((geometry) => batched!.addGeometry(geometry));
			for (let i = 0; i < count; i++) {
				const id = batched.addInstance(ids[i % ids.length]);
				placement(i, dummy);
				dummy.updateMatrix();
				batched.setMatrixAt(id, dummy.matrix);
				batched.setColorAt(id, colors[i % colors.length]);
			}
			scene.add(batched);
		} else {
			const per = geometries.map(() => 0);
			slot = new Uint32Array(count);
			for (let i = 0; i < count; i++) {
				slot[i] = per[i % geometries.length]++;
			}
			instanced = geometries.map((geometry, g) => {
				const mesh = new THREE.InstancedMesh(geometry, shared, Math.max(1, per[g]));
				mesh.count = per[g];
				scene.add(mesh);
				return mesh;
			});
			for (let i = 0; i < count; i++) {
				const mesh = instanced[i % geometries.length];
				placement(i, dummy);
				dummy.updateMatrix();
				mesh.setMatrixAt(slot[i], dummy.matrix);
				mesh.setColorAt(slot[i], colors[i % colors.length]);
			}
			for (const mesh of instanced) {
				mesh.instanceMatrix.needsUpdate = true;
				if (mesh.instanceColor) mesh.instanceColor.needsUpdate = true;
			}
		}
	}

	/** Spins every part, the same work in each mode. */
	function animate(time: number) {
		const t = time / 1000;
		if (group) {
			const children = group.children;
			for (let i = 0; i < children.length; i++) {
				children[i].rotation.set(t * 0.5, t + i, 0);
			}
		} else if (batched) {
			for (let i = 0; i < count; i++) {
				placement(i, dummy);
				dummy.rotation.set(t * 0.5, t + i, 0);
				dummy.updateMatrix();
				batched.setMatrixAt(i, dummy.matrix);
			}
		} else {
			for (let i = 0; i < count; i++) {
				placement(i, dummy);
				dummy.rotation.set(t * 0.5, t + i, 0);
				dummy.updateMatrix();
				instanced[i % geometries.length].setMatrixAt(slot[i], dummy.matrix);
			}
			for (const mesh of instanced) {
				mesh.instanceMatrix.needsUpdate = true;
			}
		}
	}

	let frames = 0;
	let renders: number[] = [];
	let lastStats = 0;
	let disposed = false;

	build();
	renderer.setAnimationLoop((time: number) => {
		if (disposed) return;
		animate(time);
		const r0 = performance.now();
		renderer.render(scene, camera);
		renders.push(performance.now() - r0);
		frames++;
		if (lastStats === 0) lastStats = time;
		if (time - lastStats >= 1000) {
			const sorted = renders.sort((a, b) => a - b);
			options.onStats?.({
				mode,
				count,
				fps: (frames * 1000) / (time - lastStats),
				draws: renderer.info.render.drawCalls,
				triangles: renderer.info.render.triangles,
				renderMs: sorted[sorted.length >> 1] ?? 0,
			});
			frames = 0;
			renders = [];
			lastStats = time;
		}
	});

	return {
		setMode(next) {
			if (next !== mode) {
				mode = next;
				build();
			}
		},
		setCount(next) {
			if (next !== count) {
				count = next;
				build();
			}
		},
		dispose() {
			disposed = true;
			renderer.setAnimationLoop(null);
			clear();
			renderer.dispose();
		},
	};
}
