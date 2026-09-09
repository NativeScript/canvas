#!/usr/bin/env python3
"""Static invariants for the Apple/Android V8 bridge sources.

This branch builds against a single vendored V8 (14.9, tools/scripts/download-v8.sh)
rather than supporting 10.3 and 14 from one tree, and its fast-call registration
depends on a few things the compiler will not catch:

  * V8 14 requires an explicit tag on internal-field and external pointer
    accessors. A read tagged differently from its write returns null rather
    than failing to build, so an untagged accessor is a runtime bug.
  * Fast-call overload sets are registered through a helper that deduces the
    array extent. If that helper ever goes back to taking a bare pointer, the
    extent is silently lost and only the first overload of each set survives
    (see the NUM(&method_overloads) bug fixed alongside this file).
  * Dual-version shims re-entering the tree would compile fast calls out on
    V8 >= 14, which is the only V8 this branch targets.

Run: python3 tools/tests/check-v8-bridge-invariants.py
"""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SRC = ROOT / 'packages/canvas/platforms/ios/src/cpp'
HELPERS = SRC / 'Helpers.h'

failures = []


def fail(message, hits):
    failures.append((message, hits))


def sources():
    for path in sorted([*SRC.rglob('*.cpp'), *SRC.rglob('*.h')]):
        yield path, path.read_text(encoding='utf-8', errors='replace')


def strip_comments(text):
    text = re.sub(r'/\*.*?\*/', '', text, flags=re.S)
    return re.sub(r'//[^\n]*', '', text)


def scan(pattern, predicate=None):
    """Yield (path, lineno, line) for lines matching pattern, skipping comments."""
    hits = []
    for path, text in sources():
        for number, line in enumerate(text.splitlines(), 1):
            if line.lstrip().startswith(('//', '*', '/*')):
                continue
            if re.search(pattern, line) and (predicate is None or predicate(line)):
                hits.append((path.relative_to(ROOT), number, line.strip()))
    return hits


# 1. Internal-field pointer accessors must carry the shared wrapper tag.
#    ObjectWrapperImpl::kInternalFieldTag is the one tag every wrapper uses;
#    an untagged call reads back null under V8 14.
untagged = scan(r'(Get|Set)Aligned\w*InternalField\s*\(',
                lambda line: 'kInternalFieldTag' not in line)
if untagged:
    fail('internal-field accessor without ObjectWrapperImpl::kInternalFieldTag', untagged)

# 2. v8::External values must carry the external pointer tag on both sides.
ext_new = scan(r'v8::External::New\s*\(',
               lambda line: 'kExternalPointerTypeTagDefault' not in line)
if ext_new:
    fail('v8::External::New without v8::kExternalPointerTypeTagDefault', ext_new)

# 3. APIs V8 14 removed or replaced must not come back.
removed = {
    r'->Utf8Length\s*\(': 'Utf8Length was replaced by Utf8LengthV2',
    r'->WriteUtf8\s*\(': 'WriteUtf8 was replaced by WriteUtf8V2',
    r'\bSetAccessor\s*\(': 'SetAccessor was replaced by SetNativeDataProperty',
}
for pattern, why in removed.items():
    hits = scan(pattern)
    if hits:
        fail(why, hits)

# 4. No dual-version shims: this branch has a single vendored V8 14.9, so a
#    `#if V8_MAJOR_VERSION` fork means one of the two arms is dead, and the
#    upstream shims disable fast calls on exactly the version we ship.
for pattern, why in {
    r'V8_MAJOR_VERSION': 'V8 version fork in a single-V8 branch (see Common.h notes)',
    r'CANVAS_FAST_FUNCTION': 'CANVAS_FAST_FUNCTION compiles fast calls out on V8 >= 14',
    r'canvas::(GetAlignedPointer|SetAlignedPointer|Receiver|NewExternal|ExternalValue|Utf8Length|WriteUtf8|SetAccessor)\b':
        'dual-version canvas:: shim; call the V8 14 API directly',
}.items():
    hits = scan(pattern)
    if hits:
        fail(why, hits)

# 5. The overload helper must take the array by reference so its extent is
#    deduced. A bare `const v8::CFunction *` parameter loses the length.
helpers = strip_comments(HELPERS.read_text(encoding='utf-8'))
signature = re.search(
    r'SetFastMethodWithOverLoads\s*\([^)]*?const\s+v8::CFunction\s*(?P<form>\(\s*&\s*\w+\s*\)\s*\[\s*\w+\s*\]|\*\s*\w+)',
    helpers, re.S)
if signature is None:
    fail('could not find SetFastMethodWithOverLoads signature in Helpers.h', [])
elif signature.group('form').lstrip().startswith('*'):
    fail('SetFastMethodWithOverLoads takes a bare pointer, so the overload count '
         'is lost and only the first overload of each set is registered',
         [(HELPERS.relative_to(ROOT), helpers[:signature.start()].count('\n') + 1,
           signature.group(0).split('\n')[-1].strip())])

# 6. Every declared overload set must actually be registered, and every
#    registration must name a set declared as an array in the same file.
for path, text in sources():
    body = strip_comments(text)
    declared = set(re.findall(r'const\s+v8::CFunction\s+(\w+_overloads_)\s*\[\s*\]', body))
    used = set(re.findall(r'SetFastMethodWithOverLoads\s*\([^;]*?\b(\w+_overloads_)\b', body, re.S))
    rel = path.relative_to(ROOT)
    for name in sorted(declared - used):
        fail(f'overload set declared but never registered: {name}', [(rel, 0, name)])
    for name in sorted(used - declared):
        # registered here but declared elsewhere -> the extent would not deduce
        fail(f'overload set registered but not declared as an array in this file: {name}',
             [(rel, 0, name)])

if failures:
    for message, hits in failures:
        print(f'FAIL: {message}', file=sys.stderr)
        for rel, number, line in hits[:20]:
            where = f'{rel}:{number}' if number else str(rel)
            print(f'    {where}: {line}', file=sys.stderr)
        if len(hits) > 20:
            print(f'    ... and {len(hits) - 20} more', file=sys.stderr)
    print(f'\n{len(failures)} invariant(s) violated', file=sys.stderr)
    sys.exit(1)

print(f'v8 bridge invariants hold across {len(list(sources()))} sources')
