/**
 * WebGPU samples runnable by name, e.g. from the perf page's `webgpu:<name>` suite:
 *
 *   echo {"demo":"canvas-perf","suite":"webgpu:rotatingCube"} > ...\LocalState\launch-args.json
 *
 * Loaded on demand so the samples' shaders and assets cost nothing until one runs.
 */
export const WEBGPU_SAMPLES: Record<string, () => { run(canvas: any): any }> = {
	rotatingCube: () => require('./rotatingCube'),
	renderBundles: () => require('./renderBundles'),
	occlusionQuery: () => require('./occlusionQuery'),
	particles: () => require('./particles'),
	texturedCube: () => require('./basicGraphics/texturedCube'),
	twoCubes: () => require('./basicGraphics/twoCubes'),
	fractalCube: () => require('./basicGraphics/fractalCube'),
	imageBlur: () => require('./imageBlur'),
	cubeMap: () => require('./cubeMap'),
	instancedCube: () => require('./instancedCube'),
	computeBoids: () => require('./gpgpu/computeBoids'),
	wireframe: () => require('./graphicsTechniques/wireframe'),
	pristineGrid: () => require('./pristine-grid'),
};

export function runWebGPUSample(name: string, canvas: any): boolean {
	const load = WEBGPU_SAMPLES[name];
	if (!load) {
		return false;
	}
	Promise.resolve(load().run(canvas)).catch((e) => console.log(`WEBGPU|error|${name}|${e?.message ?? e}`));
	return true;
}
