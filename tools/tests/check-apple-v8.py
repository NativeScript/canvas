#!/usr/bin/env python3
"""Syntax-check every Apple bridge source against a selected runtime framework."""
import argparse
import concurrent.futures
from pathlib import Path
import subprocess
import sys

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--runtime-framework', type=Path, required=True, help='Path to NativeScript.framework')
parser.add_argument('--canvas-framework', type=Path, required=True, help='Path to CanvasNative.framework')
parser.add_argument('--sdk', default='iphonesimulator')
parser.add_argument('--target', default='arm64-apple-ios13.0-simulator')
parser.add_argument('--jobs', type=int, default=4)
args = parser.parse_args()
root = Path(__file__).resolve().parents[2]
src = root / 'packages/canvas/platforms/ios/src'
for framework in [args.runtime_framework, args.canvas_framework]:
    if not framework.is_dir():
        parser.error(f'Framework does not exist: {framework}')
sdk = subprocess.check_output(['xcrun', '--sdk', args.sdk, '--show-sdk-path'], text=True).strip()
command = ['xcrun', 'clang++', '-fsyntax-only', '-std=c++20', '-x', 'objective-c++', '-target', args.target, '-isysroot', sdk]
for framework in [args.runtime_framework, args.canvas_framework]:
    command += ['-F', str(framework.resolve().parent)]
command += ['-I', str(args.runtime_framework.resolve() / 'Headers/include'), '-I', str(args.runtime_framework.resolve() / 'Headers')]
for directory in [src, *sorted(p for p in src.rglob('*') if p.is_dir())]:
    command += ['-I', str(directory)]
files = sorted([*src.rglob('*.cpp'), *src.glob('*.mm')])
def check(source):
    result = subprocess.run(command + [str(source)], capture_output=True, text=True)
    return source.relative_to(src), result
failed = 0
with concurrent.futures.ThreadPoolExecutor(max_workers=args.jobs) as pool:
    for source, result in pool.map(check, files):
        print(f'{source}: {"FAIL" if result.returncode else "PASS"}', flush=True)
        if result.returncode:
            failed += 1
            print(result.stderr, file=sys.stderr)
print(f'{len(files) - failed}/{len(files)} translation units passed')
sys.exit(bool(failed))
