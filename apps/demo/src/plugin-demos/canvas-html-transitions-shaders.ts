/*
 * Shaders for canvas-html-transitions, in WGSL (WebGPU) and GLSL (WebGL). Both take the same packed
 * uniforms. Quads: proj, model, dst, uv, color, clip (vec4s), then size, radius, alpha, mode, soft,
 * shade, clipRadius. Effects: res, toScale, origin, p, time, tintA, tintB, shift (the
 * offset that puts the page's photo under the card; the effects slide it from there to its place).
 * Positions are canvas pixels with y down. `from` is the list scene, `to` the detail page's snapshot.
 */

const WGSL_COMMON = `
const PI = 3.14159265;

fn hash(p: vec2f) -> f32 { return fract(sin(dot(p, vec2f(127.1, 311.7))) * 43758.5453); }
fn noise(p: vec2f) -> f32 {
	let i = floor(p);
	let f = fract(p);
	let u = f * f * (3.0 - 2.0 * f);
	return mix(mix(hash(i), hash(i + vec2f(1.0, 0.0)), u.x), mix(hash(i + vec2f(0.0, 1.0)), hash(i + vec2f(1.0, 1.0)), u.x), u.y);
}
fn fbm(p0: vec2f) -> f32 {
	var p = p0;
	var v = 0.0;
	var a = 0.5;
	for (var i = 0; i < 4; i++) {
		v += a * noise(p);
		p = p * 2.03 + 17.0;
		a *= 0.5;
	}
	return v;
}
fn sq(x: f32) -> f32 { return x * x; }
fn sdRound(p: vec2f, h: vec2f, r: f32) -> f32 {
	let d = abs(p) - h + r;
	return length(max(d, vec2f(0.0))) + min(max(d.x, d.y), 0.0) - r;
}
`;

export const WGSL_QUAD =
	WGSL_COMMON +
	`
struct Quad {
	proj: mat4x4f,
	model: mat4x4f,
	dst: vec4f,
	uv: vec4f,
	color: vec4f,
	clip: vec4f,
	size: vec2f,
	radius: f32,
	alpha: f32,
	mode: f32,
	soft: f32,
	shade: f32,
	clipRadius: f32,
};

@group(0) @binding(0) var<uniform> q: Quad;
@group(0) @binding(1) var samp: sampler;
@group(0) @binding(2) var tex: texture_2d<f32>;

struct QuadOut {
	@builtin(position) pos: vec4f,
	@location(0) local: vec2f,
};

@vertex fn quadVertex(@builtin(vertex_index) i: u32) -> QuadOut {
	let corner = vec2f(f32(i & 1u), f32(i >> 1u));
	var o: QuadOut;
	o.local = corner;
	o.pos = q.proj * q.model * vec4f(q.dst.xy + corner * q.dst.zw, 0.0, 1.0);
	return o;
}

@fragment fn quadFragment(v: QuadOut) -> @location(0) vec4f {
	let sampled = textureSample(tex, samp, mix(q.uv.xy, q.uv.zw, v.local));
	let p = (v.local - 0.5) * q.size;
	let d = sdRound(p, 0.5 * q.size, min(q.radius, 0.5 * min(q.size.x, q.size.y)));
	let soft = max(q.soft, 0.001);
	var mask = select(clamp(0.5 - d, 0.0, 1.0), 1.0 - smoothstep(-soft, soft, d), q.mode > 1.5);
	if (q.clip.z > 0.0) {
		let r = min(q.clipRadius, 0.5 * min(q.clip.z, q.clip.w));
		mask *= clamp(0.5 - sdRound(v.pos.xy - q.clip.xy - 0.5 * q.clip.zw, 0.5 * q.clip.zw, r), 0.0, 1.0);
	}
	let c = select(sampled, q.color, q.mode > 0.5);
	return vec4f(c.rgb * q.shade, c.a) * (mask * q.alpha);
}
`;

export const WGSL_FX =
	WGSL_COMMON +
	`
struct Fx {
	res: vec2f,
	toScale: vec2f,
	origin: vec2f,
	p: f32,
	time: f32,
	tintA: vec4f,
	tintB: vec4f,
	shift: vec2f,
};

@group(0) @binding(0) var<uniform> fx: Fx;
@group(0) @binding(1) var samp: sampler;
@group(0) @binding(2) var fromTex: texture_2d<f32>;
@group(0) @binding(3) var toTex: texture_2d<f32>;

@vertex fn fxVertex(@builtin(vertex_index) i: u32) -> @builtin(position) vec4f {
	let x = f32(i & 1u) * 4.0 - 1.0;
	let y = f32(i >> 1u) * 4.0 - 1.0;
	return vec4f(x, y, 0.0, 1.0);
}

fn fromAt(px: vec2f) -> vec4f { return textureSampleLevel(fromTex, samp, px / fx.res, 0.0); }
fn toAt(px: vec2f) -> vec4f { return textureSampleLevel(toTex, samp, clamp(px / fx.res, vec2f(0.0), vec2f(1.0)) * fx.toScale, 0.0); }

// Water: a wavefront spreads from the card, refracting the list and leaving the detail page behind it.
@fragment fn rippleFragment(@builtin(position) pos: vec4f) -> @location(0) vec4f {
	let px = pos.xy;
	let s = fx.res.y;
	let v = px - fx.origin;
	let d = length(v);
	let dir = v / max(d, 0.0001);
	let corner = max(fx.origin, fx.res - fx.origin);
	let front = d - fx.p * length(corner) * 1.1;
	let energy = sin(PI * fx.p);
	let wobble = (noise(vec2f(atan2(v.y, v.x) * 2.5, fx.time * 0.7)) - 0.5) * 0.04 * s * energy;
	let band = exp(-abs(front) / s * 9.0);
	let wave = sin(d / s * 80.0 - fx.time * 10.0) * band * energy;
	let edge = front + wobble + wave * 0.006 * s;
	let off = dir * wave * 0.012 * s;
	let inside = 1.0 - smoothstep(-0.003 * s, 0.003 * s, edge);
	// A little dispersion where the water bends the light.
	let list = vec3f(fromAt(px + off).r, fromAt(px + off * 1.08).g, fromAt(px + off * 1.16).b);
	let page = toAt(px + off * 0.4 + fx.shift * (1.0 - smoothstep(0.05, 0.55, fx.p))).rgb;
	var col = mix(list, page, inside);
	let crest = exp(-sq(edge / (0.006 * s)));
	let trough = exp(-sq((edge + 0.025 * s) / (0.02 * s)));
	let caustic = smoothstep(0.55, 1.0, noise(px / s * 28.0 + vec2f(fx.time * 1.3, -fx.time))) * band;
	col *= 1.0 - 0.18 * trough * energy;
	let light = mix(vec3f(1.0), fx.tintA.rgb, 0.35);
	col += (light * (crest * 0.45 + caustic * 0.35) + max(wave, 0.0) * 0.06) * energy;
	return vec4f(col, 1.0);
}

fn streaks(r: f32, ang: f32, cells: f32, seed: f32, speed: f32, travel: f32) -> f32 {
	let a = (ang / (2.0 * PI) + 0.5) * cells;
	let h = hash(vec2f(floor(a), seed));
	let z = fract(h * 7.31 + travel * (0.6 + h));
	let at = 0.04 + z * z * 1.4;
	let len = 0.004 + 0.45 * speed * z * z;
	let along = smoothstep(at - len, at, r) * (1.0 - smoothstep(at, at + 0.004, r));
	let across = abs(fract(a) - 0.5) * 2.0 * PI * r / cells * fx.res.y;
	return along * exp(-across * across / (0.8 + 3.0 * z)) * (0.5 + h) * step(0.3, h) * smoothstep(0.0, 0.3, z);
}

fn listAt(v: vec2f, zoom: f32, blur: f32, ca: f32, jitter: f32) -> vec3f {
	var acc = vec3f(0.0);
	for (var i = 0; i < 6; i++) {
		acc += fromAt(fx.origin + v / (zoom * (1.0 - blur * (f32(i) + jitter) / 6.0))).rgb;
	}
	acc /= 6.0;
	return vec3f(mix(acc.r, fromAt(fx.origin + v / (zoom * (1.0 + ca))).r, 0.5), acc.g, mix(acc.b, fromAt(fx.origin + v / (zoom * (1.0 - ca))).b, 0.5));
}

fn pageAt(v: vec2f, zoom: f32, blur: f32, ca: f32, jitter: f32, shift: vec2f) -> vec3f {
	var acc = vec3f(0.0);
	let o = fx.origin + shift;
	for (var i = 0; i < 6; i++) {
		acc += toAt(o + v / (zoom * (1.0 + blur * (f32(i) + jitter) / 6.0))).rgb;
	}
	acc /= 6.0;
	return vec3f(mix(acc.r, toAt(o + v / (zoom * (1.0 + ca))).r, 0.5), acc.g, mix(acc.b, toAt(o + v / (zoom * (1.0 - ca))).b, 0.5));
}

// Hyperspace: the camera dives into the card, rides a tunnel in the place's colours, and drops out at the page.
// Each stage only runs while it can be seen (p is uniform, so the branches are free).
@fragment fn warpFragment(@builtin(position) pos: vec4f) -> @location(0) vec4f {
	let px = pos.xy;
	let s = fx.res.y;
	let p = fx.p;
	let v = px - fx.origin;
	let r = length(v) / s;
	let jitter = hash(px + fract(fx.time) * 61.0);
	var col = vec3f(0.005, 0.004, 0.02);

	// The tunnel: walls of nebula rushing past, a light at the far end.
	if (p > 0.2 && p < 0.72) {
		let ang = atan2(v.y, v.x);
		let speed = smoothstep(0.12, 0.42, p) * (1.0 - smoothstep(0.6, 0.82, p));
		let travel = p * 5.0 + fx.time * 0.25;
		let a = ang / (2.0 * PI) + 0.5;
		let depth = 0.16 / max(r, 0.002);
		let spiral = fract(a + depth * 0.04);
		let tw = vec2f(spiral * 9.0, depth + travel * 1.6);
		let n = mix(fbm(tw), fbm(tw - vec2f(9.0, 0.0)), spiral);
		let lines = pow(mix(noise(vec2f(a * 140.0, depth * 0.3 + travel * 4.0)), noise(vec2f((a - 1.0) * 140.0, depth * 0.3 + travel * 4.0)), a), 5.0);
		let wall = mix(fx.tintA.rgb, fx.tintB.rgb, smoothstep(0.35, 0.65, n)) * pow(smoothstep(0.3, 0.75, n), 2.5) * 2.4 + vec3f(0.7, 0.85, 1.0) * lines * 1.4 * speed;
		col += wall * smoothstep(0.0, 0.35, r) * smoothstep(0.15, 0.45, speed + 0.1);
		let core = exp(-r * r / 0.0015) + 0.25 * exp(-r * r / 0.02);
		col += mix(vec3f(1.0), fx.tintA.rgb, 0.25) * core * (0.4 + speed);
		let stars = streaks(r, ang, 120.0, 3.7, speed, travel) + streaks(r, ang + 0.011, 200.0, 9.1, speed, travel * 0.8);
		col += vec3f(0.8, 0.9, 1.0) * stars * 2.4;
		col *= 1.0 - 0.45 * smoothstep(0.5, 1.3, r);
	}

	// Leaving: the list magnifies into the card, smeared radially, the channels slipping apart.
	if (p < 0.45) {
		let leave = smoothstep(0.0, 0.45, p);
		let listColor = listAt(v, exp2(leave * leave * 3.3), 0.4 * leave * leave, 0.05 * leave * leave, jitter);
		col = mix(col, listColor, 1.0 - smoothstep(0.22, 0.45, p));
	}

	// The light at the end of the tunnel opens up to fill the screen, and the page settles out of it.
	if (p > 0.64) {
		let exit = smoothstep(0.64, 1.0, p);
		let settle = 1.0 - pow(1.0 - exit, 4.0);
		let pageColor = pageAt(v, mix(1.35, 1.0, settle), 0.12 * (1.0 - settle), 0.03 * (1.0 - settle), jitter, fx.shift * (1.0 - settle));
		col = mix(col, pageColor, smoothstep(0.64, 0.7, p));
	}
	let open = smoothstep(0.46, 0.66, p);
	let glow = exp(-r * r / (0.0015 + 1.2 * open * open)) * (1.0 - smoothstep(0.66, 0.77, p));
	col = mix(col, mix(vec3f(1.0), fx.tintA.rgb, 0.18), clamp(glow * (0.6 + 0.4 * open), 0.0, 1.0));
	return vec4f(col, 1.0);
}

// Atmosphere: a bank of sunlit cloud, tinted with the place, rolls out from the card and clears to the page.
@fragment fn hazeFragment(@builtin(position) pos: vec4f) -> @location(0) vec4f {
	let px = pos.xy;
	let s = fx.res.y;
	let q = px / s;
	let toward = normalize(fx.origin - px + vec2f(0.0001));
	let dist = length(px - fx.origin) / s;
	let drift = fx.time * 0.05;
	let n = fbm(q * 2.6 + vec2f(drift, drift * 0.6));
	let detail = fbm(q * 7.0 - vec2f(drift * 2.0, 0.0));
	let field = n * 0.75 + detail * 0.25 + dist * 0.65;
	let threshold = mix(-0.1, 1.85, fx.p);
	let reveal = 1.0 - smoothstep(threshold - 0.06, threshold + 0.02, field);
	let energy = sin(PI * fx.p);
	let thickness = exp(-sq((field - threshold) / 0.24)) * energy;

	// Heat shimmer under the cloud.
	let g = q * 6.0 + drift * 3.0;
	let e = 0.08;
	let grad = vec2f(noise(g + vec2f(e, 0.0)) - noise(g - vec2f(e, 0.0)), noise(g + vec2f(0.0, e)) - noise(g - vec2f(0.0, e)));
	let off = grad * thickness * 0.04 * s;
	// A slow push in: the list grows past the camera, the page settles from just beyond it.
	let push = 1.0 + 0.18 * smoothstep(0.0, 1.0, fx.p);
	let arrive = mix(1.1, 1.0, smoothstep(0.2, 1.0, fx.p));
	let listPx = fx.origin + (px + off - fx.origin) / push;
	let pagePx = fx.origin + (px + off * 0.5 - fx.origin) / arrive + fx.shift * (1.0 - smoothstep(0.05, 0.55, fx.p));
	var col = mix(fromAt(listPx).rgb, toAt(pagePx).rgb, reveal);
	col *= 1.0 - 0.15 * thickness;

	// Cloud lit from the card: brighter where the density falls away toward the light.
	let density = smoothstep(0.15, 0.6, thickness) * (0.7 + 0.3 * detail);
	let lit = clamp((fbm(q * 2.6 + vec2f(drift, drift * 0.6) - toward * 0.06) - n) * 7.0 + 0.6, 0.0, 1.0);
	let shadowTone = mix(fx.tintB.rgb, vec3f(0.92, 0.93, 0.97), 0.62) * 0.9;
	let sunTone = mix(fx.tintA.rgb, vec3f(1.0, 0.99, 0.96), 0.82);
	col = mix(col, mix(shadowTone, sunTone, lit), density * 0.96);

	var rays = 0.0;
	let span = fx.origin - px;
	for (var i = 0; i < 10; i++) {
		let sp = (px + span * (f32(i) / 10.0) * 0.45) / s;
		rays += smoothstep(0.5, 0.85, noise(sp * 5.0 + drift));
	}
	col += mix(fx.tintA.rgb, vec3f(1.0, 0.95, 0.85), 0.6) * (rays / 10.0) * 0.45 * energy * (1.0 - 0.7 * reveal);
	return vec4f(col, 1.0);
}

fn rot(v: vec2f, a: f32) -> vec2f {
	let c = cos(a);
	let s = sin(a);
	return vec2f(c * v.x - s * v.y, s * v.x + c * v.y);
}

// Cosmos: a black hole opens on the card, swallows the list in a spiral and spins the page back out.
@fragment fn vortexFragment(@builtin(position) pos: vec4f) -> @location(0) vec4f {
	let px = pos.xy;
	let s = fx.res.y;
	let p = fx.p;
	let v = px - fx.origin;
	let r = length(v) / s;
	let energy = sin(PI * p);
	let horizon = 0.1 * energy;
	let fall = smoothstep(0.0, 0.55, p);
	let rise = 1.0 - smoothstep(0.45, 1.0, p);
	// Nearer the hole, space twists further and is pulled in harder; light bends around the horizon.
	let bend = exp(-r * 3.0);
	let lens = 1.0 + horizon * horizon * 0.3 / max(r * r, 0.0004);
	let listV = rot(v, fall * fall * 10.0 * bend) * (1.0 + fall * 2.5 * bend) * lens;
	let pageV = rot(v, -rise * rise * 10.0 * bend) * (1.0 + rise * 2.5 * bend) * lens;
	var col = mix(fromAt(fx.origin + listV).rgb, toAt(fx.origin + pageV + fx.shift * rise).rgb, smoothstep(0.42, 0.58, p));
	col *= 1.0 - 0.7 * energy * exp(-r * 7.0);
	let d = r - horizon;
	let disc = exp(-sq(d / (0.015 + 0.035 * energy))) * energy;
	let swirl = noise(rot(v / s, fx.time * 2.5 + 0.6 / (r + 0.05)) * 36.0);
	col += mix(fx.tintA.rgb, vec3f(1.0, 0.92, 0.8), 0.45) * disc * (0.5 + 1.1 * swirl) * 1.5;
	col *= smoothstep(-0.003, 0.003, d);
	return vec4f(col, 1.0);
}

// Liquid: ink blooms from the card through the paper; the page is what is left behind it.
@fragment fn inkFragment(@builtin(position) pos: vec4f) -> @location(0) vec4f {
	let px = pos.xy;
	let s = fx.res.y;
	let p = fx.p;
	let q = px / s;
	let flow = fx.time * 0.06;
	let warped = length(px - fx.origin) / s + (fbm(q * 2.5 + vec2f(flow, -flow)) - 0.5) * 0.5 + (fbm(q * 9.0 - flow) - 0.5) * 0.1;
	let front = p * 1.8 - 0.18;
	let inside = 1.0 - smoothstep(front - 0.005, front + 0.005, warped);
	let energy = sin(PI * p);
	let page = toAt(px + fx.shift * (1.0 - smoothstep(0.05, 0.55, p))).rgb;
	var col = mix(fromAt(px).rgb, page, inside);
	// Pigment pools at the wet edge and thins out behind it.
	let pool = smoothstep(front - 0.12, front - 0.004, warped) * inside;
	let ink = mix(fx.tintA.rgb, fx.tintB.rgb * 0.35, smoothstep(front - 0.12, front, warped));
	col = mix(col, ink, pool * 0.92 * min(1.0, energy * 2.5));
	col = mix(col, fx.tintB.rgb * 0.25, exp(-sq((warped - front) / 0.005)) * 0.8 * min(1.0, energy * 2.5));
	col += vec3f(1.0) * exp(-sq((warped - front + 0.025) / 0.008)) * 0.22 * energy;
	return vec4f(col, 1.0);
}

// Fire: the list burns away from the card, scorching ahead of the flame, embers lifting off the edge.
@fragment fn burnFragment(@builtin(position) pos: vec4f) -> @location(0) vec4f {
	let px = pos.xy;
	let s = fx.res.y;
	let p = fx.p;
	let q = px / s;
	let field = length(px - fx.origin) / s + (fbm(q * 4.0) - 0.5) * 0.35 + (noise(q * 18.0) - 0.5) * 0.05;
	let front = p * 1.8 - 0.15;
	let gone = 1.0 - smoothstep(front - 0.004, front + 0.004, field);
	let live = smoothstep(0.0, 0.04, p) * (1.0 - smoothstep(0.94, 1.0, p));
	let heat = exp(-sq((field - front) / 0.05)) * live;
	let wobble = vec2f(noise(q * 20.0 + vec2f(0.0, fx.time * 3.0)), noise(q * 20.0 + vec2f(5.0, fx.time * 3.0))) - 0.5;
	let page = toAt(px + wobble * heat * 0.012 * s + fx.shift * (1.0 - smoothstep(0.05, 0.55, p))).rgb;
	var list = fromAt(px).rgb;
	list = mix(list, list * vec3f(0.3, 0.18, 0.1), smoothstep(front + 0.09, front, field) * (1.0 - gone) * live);
	var col = mix(list, page, gone);
	let edge = field - front;
	col += (vec3f(1.0, 0.45, 0.08) * exp(-sq(edge / 0.007)) * 1.8 + vec3f(1.0, 0.9, 0.5) * exp(-sq(edge / 0.0025)) * 1.4) * live;
	let grid = vec2f(px.x, px.y + fx.time * 0.15 * s) / (0.018 * s);
	let local = fract(grid) - 0.5;
	let spark = step(0.92, hash(floor(grid))) * exp(-dot(local, local) * 50.0) * exp(-sq((edge - 0.04) / 0.05));
	col += vec3f(1.0, 0.6, 0.2) * spark * 2.2 * live;
	return vec4f(col, 1.0);
}

// Digital: the list breaks into blocks that flip to the page outward from the card, rows glitching as they go.
@fragment fn pixelsFragment(@builtin(position) pos: vec4f) -> @location(0) vec4f {
	let px = pos.xy;
	let s = fx.res.y;
	let p = fx.p;
	let energy = sin(PI * p);
	let size = max(1.0, floor(s * 0.045 * pow(energy, 1.5)));
	let center = (floor(px / size) + 0.5) * size;
	let at = 0.2 + 0.45 * length(center - fx.origin) / length(max(fx.origin, fx.res - fx.origin)) + 0.15 * hash(floor(px / (s * 0.045)));
	let take = step(at, p);
	let g = hash(vec2f(floor(px.y / (s * 0.012)), floor(fx.time * 14.0)));
	let shove = vec2f(select(0.0, (g - 0.9) * 0.6 * s, g > 0.9) * energy, 0.0);
	let spot = select(center, px, size <= 1.0) + shove;
	var col = mix(fromAt(spot).rgb, toAt(spot + fx.shift * (1.0 - smoothstep(0.05, 0.55, p))).rgb, take);
	let f = fract(px / size);
	col *= mix(1.0, 0.86 + 0.14 * step(0.07, f.x) * step(0.07, f.y), step(2.0, size));
	col = mix(col, fx.tintA.rgb, exp(-sq((p - at) / 0.03)) * 0.55 * step(2.0, size));
	return vec4f(col, 1.0);
}
`;

export const GLSL_QUAD_VERTEX = `
attribute vec2 a_corner;
uniform mat4 u_proj;
uniform mat4 u_model;
uniform vec4 u_dst;
varying vec2 v_local;
void main() {
	v_local = a_corner;
	gl_Position = u_proj * u_model * vec4(u_dst.xy + a_corner * u_dst.zw, 0.0, 1.0);
}`;

export const GLSL_QUAD = `
precision highp float;
varying vec2 v_local;
uniform sampler2D u_tex;
uniform vec4 u_uv;
uniform vec4 u_color;
uniform vec4 u_clip;
uniform vec2 u_size;
uniform float u_radius;
uniform float u_alpha;
uniform float u_mode;
uniform float u_soft;
uniform float u_shade;
uniform float u_clipRadius;
uniform float u_height;

float sdRound(vec2 p, vec2 h, float r) {
	vec2 d = abs(p) - h + r;
	return length(max(d, 0.0)) + min(max(d.x, d.y), 0.0) - r;
}

void main() {
	vec2 p = (v_local - 0.5) * u_size;
	float d = sdRound(p, 0.5 * u_size, min(u_radius, 0.5 * min(u_size.x, u_size.y)));
	float soft = max(u_soft, 0.001);
	float mask = u_mode > 1.5 ? 1.0 - smoothstep(-soft, soft, d) : clamp(0.5 - d, 0.0, 1.0);
	if (u_clip.z > 0.0) {
		vec2 frag = vec2(gl_FragCoord.x, u_height - gl_FragCoord.y);
		float r = min(u_clipRadius, 0.5 * min(u_clip.z, u_clip.w));
		mask *= clamp(0.5 - sdRound(frag - u_clip.xy - 0.5 * u_clip.zw, 0.5 * u_clip.zw, r), 0.0, 1.0);
	}
	vec4 c = u_mode > 0.5 ? u_color : texture2D(u_tex, mix(u_uv.xy, u_uv.zw, v_local));
	gl_FragColor = vec4(c.rgb * u_shade, c.a) * (mask * u_alpha);
}`;

export const GLSL_FX_VERTEX = `
attribute vec2 a_corner;
void main() {
	gl_Position = vec4(a_corner * 2.0 - 1.0, 0.0, 1.0);
}`;

// The list scene was rendered into a GL framebuffer, so it is stored bottom-up.
const GLSL_FX_HEADER = `
precision highp float;
uniform sampler2D u_from;
uniform sampler2D u_to;
uniform vec2 u_res;
uniform vec2 u_toScale;
uniform vec2 u_origin;
uniform float u_p;
uniform float u_time;
uniform vec4 u_tintA;
uniform vec4 u_tintB;
uniform vec2 u_shift;

const float PI = 3.14159265;

vec2 fragPx() { return vec2(gl_FragCoord.x, u_res.y - gl_FragCoord.y); }
vec4 fromAt(vec2 px) { return texture2D(u_from, vec2(px.x, u_res.y - px.y) / u_res); }
vec4 toAt(vec2 px) { return texture2D(u_to, clamp(px / u_res, 0.0, 1.0) * u_toScale); }
float sq(float x) { return x * x; }
float hash(vec2 p) { return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453); }
float noise(vec2 p) {
	vec2 i = floor(p);
	vec2 f = fract(p);
	vec2 u = f * f * (3.0 - 2.0 * f);
	return mix(mix(hash(i), hash(i + vec2(1.0, 0.0)), u.x), mix(hash(i + vec2(0.0, 1.0)), hash(i + vec2(1.0, 1.0)), u.x), u.y);
}
float fbm(vec2 p) {
	float v = 0.0;
	float a = 0.5;
	for (int i = 0; i < 4; i++) {
		v += a * noise(p);
		p = p * 2.03 + 17.0;
		a *= 0.5;
	}
	return v;
}
`;

export const GLSL_RIPPLE =
	GLSL_FX_HEADER +
	`
void main() {
	vec2 px = fragPx();
	float s = u_res.y;
	vec2 v = px - u_origin;
	float d = length(v);
	vec2 dir = v / max(d, 0.0001);
	vec2 corner = max(u_origin, u_res - u_origin);
	float front = d - u_p * length(corner) * 1.1;
	float energy = sin(PI * u_p);
	float wobble = (noise(vec2(atan(v.y, v.x) * 2.5, u_time * 0.7)) - 0.5) * 0.04 * s * energy;
	float band = exp(-abs(front) / s * 9.0);
	float wave = sin(d / s * 80.0 - u_time * 10.0) * band * energy;
	float edge = front + wobble + wave * 0.006 * s;
	vec2 off = dir * wave * 0.012 * s;
	float inside = 1.0 - smoothstep(-0.003 * s, 0.003 * s, edge);
	vec3 list = vec3(fromAt(px + off).r, fromAt(px + off * 1.08).g, fromAt(px + off * 1.16).b);
	vec3 page = toAt(px + off * 0.4 + u_shift * (1.0 - smoothstep(0.05, 0.55, u_p))).rgb;
	vec3 col = mix(list, page, inside);
	float crest = exp(-sq(edge / (0.006 * s)));
	float trough = exp(-sq((edge + 0.025 * s) / (0.02 * s)));
	float caustic = smoothstep(0.55, 1.0, noise(px / s * 28.0 + vec2(u_time * 1.3, -u_time))) * band;
	col *= 1.0 - 0.18 * trough * energy;
	vec3 light = mix(vec3(1.0), u_tintA.rgb, 0.35);
	col += (light * (crest * 0.45 + caustic * 0.35) + max(wave, 0.0) * 0.06) * energy;
	gl_FragColor = vec4(col, 1.0);
}`;

export const GLSL_WARP =
	GLSL_FX_HEADER +
	`
float streaks(float r, float ang, float cells, float seed, float speed, float travel) {
	float a = (ang / (2.0 * PI) + 0.5) * cells;
	float h = hash(vec2(floor(a), seed));
	float z = fract(h * 7.31 + travel * (0.6 + h));
	float at = 0.04 + z * z * 1.4;
	float len = 0.004 + 0.45 * speed * z * z;
	float along = smoothstep(at - len, at, r) * (1.0 - smoothstep(at, at + 0.004, r));
	float across = abs(fract(a) - 0.5) * 2.0 * PI * r / cells * u_res.y;
	return along * exp(-across * across / (0.8 + 3.0 * z)) * (0.5 + h) * step(0.3, h) * smoothstep(0.0, 0.3, z);
}

vec3 listAt(vec2 v, float zoom, float blur, float ca, float jitter) {
	vec3 acc = vec3(0.0);
	for (int i = 0; i < 6; i++) {
		acc += fromAt(u_origin + v / (zoom * (1.0 - blur * (float(i) + jitter) / 6.0))).rgb;
	}
	acc /= 6.0;
	return vec3(mix(acc.r, fromAt(u_origin + v / (zoom * (1.0 + ca))).r, 0.5), acc.g, mix(acc.b, fromAt(u_origin + v / (zoom * (1.0 - ca))).b, 0.5));
}

vec3 pageAt(vec2 v, float zoom, float blur, float ca, float jitter, vec2 shift) {
	vec3 acc = vec3(0.0);
	vec2 o = u_origin + shift;
	for (int i = 0; i < 6; i++) {
		acc += toAt(o + v / (zoom * (1.0 + blur * (float(i) + jitter) / 6.0))).rgb;
	}
	acc /= 6.0;
	return vec3(mix(acc.r, toAt(o + v / (zoom * (1.0 + ca))).r, 0.5), acc.g, mix(acc.b, toAt(o + v / (zoom * (1.0 - ca))).b, 0.5));
}

void main() {
	vec2 px = fragPx();
	float s = u_res.y;
	float p = u_p;
	vec2 v = px - u_origin;
	float r = length(v) / s;
	float jitter = hash(px + fract(u_time) * 61.0);
	vec3 col = vec3(0.005, 0.004, 0.02);

	if (p > 0.2 && p < 0.72) {
		float ang = atan(v.y, v.x);
		float speed = smoothstep(0.12, 0.42, p) * (1.0 - smoothstep(0.6, 0.82, p));
		float travel = p * 5.0 + u_time * 0.25;
		float a = ang / (2.0 * PI) + 0.5;
		float depth = 0.16 / max(r, 0.002);
		float spiral = fract(a + depth * 0.04);
		vec2 tw = vec2(spiral * 9.0, depth + travel * 1.6);
		float n = mix(fbm(tw), fbm(tw - vec2(9.0, 0.0)), spiral);
		float lines = pow(mix(noise(vec2(a * 140.0, depth * 0.3 + travel * 4.0)), noise(vec2((a - 1.0) * 140.0, depth * 0.3 + travel * 4.0)), a), 5.0);
		vec3 wall = mix(u_tintA.rgb, u_tintB.rgb, smoothstep(0.35, 0.65, n)) * pow(smoothstep(0.3, 0.75, n), 2.5) * 2.4 + vec3(0.7, 0.85, 1.0) * lines * 1.4 * speed;
		col += wall * smoothstep(0.0, 0.35, r) * smoothstep(0.15, 0.45, speed + 0.1);
		float core = exp(-r * r / 0.0015) + 0.25 * exp(-r * r / 0.02);
		col += mix(vec3(1.0), u_tintA.rgb, 0.25) * core * (0.4 + speed);
		float stars = streaks(r, ang, 120.0, 3.7, speed, travel) + streaks(r, ang + 0.011, 200.0, 9.1, speed, travel * 0.8);
		col += vec3(0.8, 0.9, 1.0) * stars * 2.4;
		col *= 1.0 - 0.45 * smoothstep(0.5, 1.3, r);
	}

	if (p < 0.45) {
		float leave = smoothstep(0.0, 0.45, p);
		vec3 listColor = listAt(v, exp2(leave * leave * 3.3), 0.4 * leave * leave, 0.05 * leave * leave, jitter);
		col = mix(col, listColor, 1.0 - smoothstep(0.22, 0.45, p));
	}

	if (p > 0.64) {
		float exit = smoothstep(0.64, 1.0, p);
		float settle = 1.0 - pow(1.0 - exit, 4.0);
		vec3 pageColor = pageAt(v, mix(1.35, 1.0, settle), 0.12 * (1.0 - settle), 0.03 * (1.0 - settle), jitter, u_shift * (1.0 - settle));
		col = mix(col, pageColor, smoothstep(0.64, 0.7, p));
	}
	float open = smoothstep(0.46, 0.66, p);
	float glow = exp(-r * r / (0.0015 + 1.2 * open * open)) * (1.0 - smoothstep(0.66, 0.77, p));
	col = mix(col, mix(vec3(1.0), u_tintA.rgb, 0.18), clamp(glow * (0.6 + 0.4 * open), 0.0, 1.0));
	gl_FragColor = vec4(col, 1.0);
}`;

export const GLSL_HAZE =
	GLSL_FX_HEADER +
	`
void main() {
	vec2 px = fragPx();
	float s = u_res.y;
	vec2 q = px / s;
	vec2 toward = normalize(u_origin - px + vec2(0.0001));
	float dist = length(px - u_origin) / s;
	float drift = u_time * 0.05;
	float n = fbm(q * 2.6 + vec2(drift, drift * 0.6));
	float detail = fbm(q * 7.0 - vec2(drift * 2.0, 0.0));
	float field = n * 0.75 + detail * 0.25 + dist * 0.65;
	float threshold = mix(-0.1, 1.85, u_p);
	float reveal = 1.0 - smoothstep(threshold - 0.06, threshold + 0.02, field);
	float energy = sin(PI * u_p);
	float thickness = exp(-sq((field - threshold) / 0.24)) * energy;

	vec2 g = q * 6.0 + drift * 3.0;
	float e = 0.08;
	vec2 grad = vec2(noise(g + vec2(e, 0.0)) - noise(g - vec2(e, 0.0)), noise(g + vec2(0.0, e)) - noise(g - vec2(0.0, e)));
	vec2 off = grad * thickness * 0.04 * s;
	float push = 1.0 + 0.18 * smoothstep(0.0, 1.0, u_p);
	float arrive = mix(1.1, 1.0, smoothstep(0.2, 1.0, u_p));
	vec2 listPx = u_origin + (px + off - u_origin) / push;
	vec2 pagePx = u_origin + (px + off * 0.5 - u_origin) / arrive + u_shift * (1.0 - smoothstep(0.05, 0.55, u_p));
	vec3 col = mix(fromAt(listPx).rgb, toAt(pagePx).rgb, reveal);
	col *= 1.0 - 0.15 * thickness;

	float density = smoothstep(0.15, 0.6, thickness) * (0.7 + 0.3 * detail);
	float lit = clamp((fbm(q * 2.6 + vec2(drift, drift * 0.6) - toward * 0.06) - n) * 7.0 + 0.6, 0.0, 1.0);
	vec3 shadowTone = mix(u_tintB.rgb, vec3(0.92, 0.93, 0.97), 0.62) * 0.9;
	vec3 sunTone = mix(u_tintA.rgb, vec3(1.0, 0.99, 0.96), 0.82);
	col = mix(col, mix(shadowTone, sunTone, lit), density * 0.96);

	float rays = 0.0;
	vec2 span = u_origin - px;
	for (int i = 0; i < 10; i++) {
		vec2 sp = (px + span * (float(i) / 10.0) * 0.45) / s;
		rays += smoothstep(0.5, 0.85, noise(sp * 5.0 + drift));
	}
	col += mix(u_tintA.rgb, vec3(1.0, 0.95, 0.85), 0.6) * (rays / 10.0) * 0.45 * energy * (1.0 - 0.7 * reveal);
	gl_FragColor = vec4(col, 1.0);
}`;

const GLSL_ROT = `
vec2 rot(vec2 v, float a) {
	float c = cos(a);
	float s = sin(a);
	return vec2(c * v.x - s * v.y, s * v.x + c * v.y);
}
`;

export const GLSL_VORTEX =
	GLSL_FX_HEADER +
	GLSL_ROT +
	`
void main() {
	vec2 px = fragPx();
	float s = u_res.y;
	float p = u_p;
	vec2 v = px - u_origin;
	float r = length(v) / s;
	float energy = sin(PI * p);
	float horizon = 0.1 * energy;
	float fall = smoothstep(0.0, 0.55, p);
	float rise = 1.0 - smoothstep(0.45, 1.0, p);
	float bend = exp(-r * 3.0);
	float lens = 1.0 + horizon * horizon * 0.3 / max(r * r, 0.0004);
	vec2 listV = rot(v, fall * fall * 10.0 * bend) * (1.0 + fall * 2.5 * bend) * lens;
	vec2 pageV = rot(v, -rise * rise * 10.0 * bend) * (1.0 + rise * 2.5 * bend) * lens;
	vec3 col = mix(fromAt(u_origin + listV).rgb, toAt(u_origin + pageV + u_shift * rise).rgb, smoothstep(0.42, 0.58, p));
	col *= 1.0 - 0.7 * energy * exp(-r * 7.0);
	float d = r - horizon;
	float disc = exp(-sq(d / (0.015 + 0.035 * energy))) * energy;
	float swirl = noise(rot(v / s, u_time * 2.5 + 0.6 / (r + 0.05)) * 36.0);
	col += mix(u_tintA.rgb, vec3(1.0, 0.92, 0.8), 0.45) * disc * (0.5 + 1.1 * swirl) * 1.5;
	col *= smoothstep(-0.003, 0.003, d);
	gl_FragColor = vec4(col, 1.0);
}`;

export const GLSL_INK =
	GLSL_FX_HEADER +
	`
void main() {
	vec2 px = fragPx();
	float s = u_res.y;
	float p = u_p;
	vec2 q = px / s;
	float flow = u_time * 0.06;
	float warped = length(px - u_origin) / s + (fbm(q * 2.5 + vec2(flow, -flow)) - 0.5) * 0.5 + (fbm(q * 9.0 - flow) - 0.5) * 0.1;
	float front = p * 1.8 - 0.18;
	float inside = 1.0 - smoothstep(front - 0.005, front + 0.005, warped);
	float energy = sin(PI * p);
	vec3 page = toAt(px + u_shift * (1.0 - smoothstep(0.05, 0.55, p))).rgb;
	vec3 col = mix(fromAt(px).rgb, page, inside);
	float pool = smoothstep(front - 0.12, front - 0.004, warped) * inside;
	vec3 ink = mix(u_tintA.rgb, u_tintB.rgb * 0.35, smoothstep(front - 0.12, front, warped));
	col = mix(col, ink, pool * 0.92 * min(1.0, energy * 2.5));
	col = mix(col, u_tintB.rgb * 0.25, exp(-sq((warped - front) / 0.005)) * 0.8 * min(1.0, energy * 2.5));
	col += vec3(1.0) * exp(-sq((warped - front + 0.025) / 0.008)) * 0.22 * energy;
	gl_FragColor = vec4(col, 1.0);
}`;

export const GLSL_BURN =
	GLSL_FX_HEADER +
	`
void main() {
	vec2 px = fragPx();
	float s = u_res.y;
	float p = u_p;
	vec2 q = px / s;
	float field = length(px - u_origin) / s + (fbm(q * 4.0) - 0.5) * 0.35 + (noise(q * 18.0) - 0.5) * 0.05;
	float front = p * 1.8 - 0.15;
	float gone = 1.0 - smoothstep(front - 0.004, front + 0.004, field);
	float live = smoothstep(0.0, 0.04, p) * (1.0 - smoothstep(0.94, 1.0, p));
	float heat = exp(-sq((field - front) / 0.05)) * live;
	vec2 wobble = vec2(noise(q * 20.0 + vec2(0.0, u_time * 3.0)), noise(q * 20.0 + vec2(5.0, u_time * 3.0))) - 0.5;
	vec3 page = toAt(px + wobble * heat * 0.012 * s + u_shift * (1.0 - smoothstep(0.05, 0.55, p))).rgb;
	vec3 list = fromAt(px).rgb;
	list = mix(list, list * vec3(0.3, 0.18, 0.1), smoothstep(front + 0.09, front, field) * (1.0 - gone) * live);
	vec3 col = mix(list, page, gone);
	float edge = field - front;
	col += (vec3(1.0, 0.45, 0.08) * exp(-sq(edge / 0.007)) * 1.8 + vec3(1.0, 0.9, 0.5) * exp(-sq(edge / 0.0025)) * 1.4) * live;
	vec2 grid = vec2(px.x, px.y + u_time * 0.15 * s) / (0.018 * s);
	vec2 local = fract(grid) - 0.5;
	float spark = step(0.92, hash(floor(grid))) * exp(-dot(local, local) * 50.0) * exp(-sq((edge - 0.04) / 0.05));
	col += vec3(1.0, 0.6, 0.2) * spark * 2.2 * live;
	gl_FragColor = vec4(col, 1.0);
}`;

export const GLSL_PIXELS =
	GLSL_FX_HEADER +
	`
void main() {
	vec2 px = fragPx();
	float s = u_res.y;
	float p = u_p;
	float energy = sin(PI * p);
	float size = max(1.0, floor(s * 0.045 * pow(energy, 1.5)));
	vec2 center = (floor(px / size) + 0.5) * size;
	float at = 0.2 + 0.45 * length(center - u_origin) / length(max(u_origin, u_res - u_origin)) + 0.15 * hash(floor(px / (s * 0.045)));
	float take = step(at, p);
	float g = hash(vec2(floor(px.y / (s * 0.012)), floor(u_time * 14.0)));
	vec2 shove = vec2(g > 0.9 ? (g - 0.9) * 0.6 * s * energy : 0.0, 0.0);
	vec2 spot = (size <= 1.0 ? px : center) + shove;
	vec3 col = mix(fromAt(spot).rgb, toAt(spot + u_shift * (1.0 - smoothstep(0.05, 0.55, p))).rgb, take);
	vec2 f = fract(px / size);
	col *= mix(1.0, 0.86 + 0.14 * step(0.07, f.x) * step(0.07, f.y), step(2.0, size));
	col = mix(col, u_tintA.rgb, exp(-sq((p - at) / 0.03)) * 0.55 * step(2.0, size));
	gl_FragColor = vec4(col, 1.0);
}`;
