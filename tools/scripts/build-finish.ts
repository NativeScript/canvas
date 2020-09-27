const ngPackage = require('ng-packagr');
const path = require('path');
const fs = require('fs-extra');

const rootDir = path.resolve(path.join(__dirname, '..', '..'));
const nxConfigPath = path.resolve(path.join(rootDir, 'nx.json'));
const nxConfig = JSON.parse(fs.readFileSync(nxConfigPath));
const npmScope = nxConfig.npmScope;

const cmdArgs = process.argv.slice(2);
const packageName = cmdArgs[0];
const publish = cmdArgs[1] === 'publish';

console.log(`Building ${npmScope}/${packageName}...${publish ? 'and publishing.' : ''}`);

function finishPreparation() {
	fs.copy(path.join('dist', 'out-tsc'), path.join('dist', 'packages', packageName))
		.then(() => fs.copy(path.join('tools', 'assets', 'publishing'), path.join('dist', 'packages', packageName)))
		.then(() => console.log(`${npmScope}/${packageName} ready to publish.`))
		.catch((err) => console.error(err));
	// fs.copy(path.join('tools', 'assets', 'publishing'), path.join('dist', 'packages', packageName))
	// 	.then(() => console.log(`${npmScope}/${packageName} ready to publish.`))
	// 	.catch((err) => console.error(err));
}

finishPreparation();
