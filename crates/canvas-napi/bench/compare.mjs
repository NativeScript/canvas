// Compares two bench/run.mjs result files: per scenario, the median change. Exits 1 when any
// scenario got slower than the threshold (default 15%), so it can gate a change.
//
//   node bench/compare.mjs bench/results/<before>.json bench/results/<after>.json [--threshold 0.15]
import fs from 'node:fs';

const [beforeFile, afterFile] = process.argv.slice(2).filter((a) => !a.startsWith('--'));
const thresholdIndex = process.argv.indexOf('--threshold');
const threshold = thresholdIndex > 0 ? Number(process.argv[thresholdIndex + 1]) : 0.15;
if (!beforeFile || !afterFile) {
	console.error('usage: compare.mjs <before.json> <after.json> [--threshold 0.15]');
	process.exit(2);
}

const load = (file) => new Map(JSON.parse(fs.readFileSync(file, 'utf8')).results.map((r) => [`${r.group}.${r.name}`, r]));
const before = load(beforeFile);
const after = load(afterFile);

let regressions = 0;
for (const [key, now] of after) {
	const then = before.get(key);
	if (!then) {
		console.log(`${key.padEnd(44)} ${now.median.toFixed(1).padStart(9)} ns   (new)`);
		continue;
	}
	const change = (now.median - then.median) / then.median;
	const slower = change > threshold;
	regressions += slower ? 1 : 0;
	const sign = change >= 0 ? '+' : '';
	console.log(`${key.padEnd(44)} ${then.median.toFixed(1).padStart(9)} -> ${now.median.toFixed(1).padStart(9)} ns  ${`${sign}${(change * 100).toFixed(0)}%`.padStart(6)}${slower ? '  SLOWER' : ''}`);
}
process.exit(regressions > 0 ? 1 : 0);
