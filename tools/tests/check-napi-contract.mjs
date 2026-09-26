// Checks the Node-API module (crates/canvas-napi) against the surface the V8 bindings register
// (packages/canvas/platforms/ios/src/cpp/**), which is what packages/canvas is written against.
//
//   node tools/tests/check-napi-contract.mjs [--json] [--strict] [--class Name]
//
// For every C++ class (SetClassName) its methods and accessors must exist on the napi class of
// the same name (prototype or static); module-level members (canvasMod->Set) must exist on
// CanvasModule. ALL_CAPS constants are reported separately: packages/canvas defines its own.
// --strict exits 1 when anything but constants is missing.
import fs from 'node:fs';
import path from 'node:path';
import url from 'node:url';

const here = path.dirname(url.fileURLToPath(import.meta.url));
const root = path.resolve(here, '../..');
const cppRoot = path.join(root, 'packages/canvas/platforms/ios/src/cpp');

const args = process.argv.slice(2);
const asJson = args.includes('--json');
const strict = args.includes('--strict');
const onlyClass = args.includes('--class') ? args[args.indexOf('--class') + 1] : null;

function walk(dir, out = []) {
	for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
		const full = path.join(dir, entry.name);
		if (entry.isDirectory()) walk(full, out);
		else if (entry.name.endsWith('.cpp')) out.push(full);
	}
	return out;
}

const MEMBER = /(?:->Set|->SetNativeDataProperty|->SetAccessor|->SetLazyDataProperty|->SetAccessorProperty)\(\s*ConvertToV8String\(\s*isolate\s*,\s*"([^"]+)"/g;
// Fast API methods: SetFastMethod(isolate, tmpl, "name", ...) / SetFastMethodWithOverLoads(...).
const FAST_METHOD = /SetFastMethod(?:WithOverLoads)?\(\s*(?:isolate|context)\s*,\s*\w+\s*,\s*"([^"]+)"/g;
const CONSTANT = /->Set\(\s*isolate\s*,\s*"([^"]+)"/g;
const MODULE_MEMBER = /canvasMod->Set\(\s*context\s*,\s*ConvertToV8String\(\s*isolate\s*,\s*"([^"]+)"/g;
const CLASS_NAME = /SetClassName\(\s*ConvertToV8String\(\s*isolate\s*,\s*"([^"]+)"/g;
// e.g. WebGLRenderingContext::SetMethods(isolate, tmpl) inside WebGL2RenderingContext's file.
const INHERITED = /(\w+)::Set(?:Methods|Props|Constants)\(\s*isolate/g;

function matches(re, text) {
	return [...text.matchAll(re)].map((m) => m[1]);
}

const files = walk(cppRoot);
const byFile = new Map();
for (const file of files) {
	const text = fs.readFileSync(file, 'utf8');
	byFile.set(file, {
		classes: matches(CLASS_NAME, text),
		members: new Set([...matches(MEMBER, text), ...matches(FAST_METHOD, text)]),
		constants: new Set(matches(CONSTANT, text)),
		module: matches(MODULE_MEMBER, text),
		inherits: matches(INHERITED, text),
	});
}

// Class -> members, following Set{Methods,Props,Constants} calls into other classes' files.
const classFile = new Map();
for (const [file, info] of byFile) {
	for (const name of info.classes) {
		if (!classFile.has(name)) classFile.set(name, file);
	}
}
function surface(name, seen = new Set()) {
	const file = classFile.get(name);
	const members = new Set();
	const constants = new Set();
	if (!file || seen.has(name)) return { members, constants };
	seen.add(name);
	const info = byFile.get(file);
	info.members.forEach((m) => members.add(m));
	info.constants.forEach((c) => constants.add(c));
	for (const base of info.inherits) {
		if (base === name) continue;
		const inherited = surface(base, seen);
		inherited.members.forEach((m) => members.add(m));
		inherited.constants.forEach((c) => constants.add(c));
	}
	return { members, constants };
}

const { load } = await import(url.pathToFileURL(path.join(root, 'crates/canvas-napi/__test__/addon.mjs')).href);
const CanvasModule = load();

function has(ctor, name) {
	return name in ctor || (ctor.prototype && name in ctor.prototype);
}

const report = { module: { missing: [] }, classes: {} };
const moduleMembers = new Set();
for (const info of byFile.values()) info.module.forEach((m) => moduleMembers.add(m));
report.module.missing = [...moduleMembers].filter((m) => !(m in CanvasModule)).sort();

// Ignore object-literal keys that the member regex can also hit (only on ->Set(context, ...)).
const NOT_MEMBERS = new Set(['constructor']);

let failures = report.module.missing.length;
for (const name of [...classFile.keys()].sort()) {
	if (onlyClass && name !== onlyClass) continue;
	const { members, constants } = surface(name);
	const ctor = CanvasModule[name];
	if (typeof ctor !== 'function') {
		report.classes[name] = { exported: false, members: members.size };
		failures++;
		continue;
	}
	const missing = [...members].filter((m) => !NOT_MEMBERS.has(m) && !has(ctor, m)).sort();
	const missingConstants = [...constants].filter((c) => !has(ctor, c)).sort();
	failures += missing.length;
	report.classes[name] = { exported: true, members: members.size, missing, missingConstants: missingConstants.length };
}

if (asJson) {
	console.log(JSON.stringify(report, null, 2));
} else {
	if (report.module.missing.length) {
		console.log(`CanvasModule: missing ${report.module.missing.join(', ')}`);
	}
	for (const [name, r] of Object.entries(report.classes)) {
		if (!r.exported) {
			console.log(`${name}: NOT EXPORTED (${r.members} members)`);
		} else if (r.missing.length || r.missingConstants) {
			const constants = r.missingConstants ? ` (+${r.missingConstants} constants)` : '';
			console.log(`${name}: ${r.missing.length}/${r.members} missing${constants}${r.missing.length ? ': ' + r.missing.join(', ') : ''}`);
		}
	}
	console.log(failures ? `${failures} missing` : 'contract OK');
}
process.exit(strict && failures ? 1 : 0);
