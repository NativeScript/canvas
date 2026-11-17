#!/usr/bin/env python3
"""Computes the cache key of each native build output in CI: a hash of the tracked files the output
may be built from, read from git at a commit, so it is the same on every runner and the working tree
does not affect it. .github/actions/native-cache restores a job's output under this key and skips
the build on a hit; this docstring is the one description of what the key covers.

  native-inputs.py ARTIFACT [REV]             the key
  native-inputs.py --explain ARTIFACT [REV]   each input with its git mode and object id (the key
                                              hashes this)
  native-inputs.py --verbose ARTIFACT [REV]   the key, with the --explain lines on stderr
  native-inputs.py --list                     the artifact names
  --job ID, --variant V                       as the action passes them: fail unless job ID is one of
                                              ARTIFACT's, and unless a matrix job passes its matrix

A key covers every tracked file except those the artifact is known not to read, so a file nobody
has placed rebuilds every output rather than none:
  * crates: those in its entry in ARTIFACTS and every workspace crate they reach through path
    dependencies, as `cargo metadata` reports them for the commit's manifests, with every crate dir
    that is not a workspace member and every dir the root Cargo.toml's [patch] and [replace] tables
    point a dependency at (crates/vendor), the root Cargo.toml, .cargo/, rust-toolchain* and
    Cargo.lock. Of the other members, the dependencies each declares
    and its rust-version: Cargo.lock is not committed, so cargo resolves one set of versions for the
    whole workspace, and a requirement in any member can move what every build links. An artifact
    without crates reads none of these;
  * workflows: the blocks of its jobs (one, but for the xcframeworks, which also have a lookup job
    and the jobs that build their parts) and their workflows' settings above `jobs:` (comment lines
    dropped, `run: |` text kept as written), so editing one job rebuilds only what it builds; no
    workflow file;
  * actions: the dirs of the local actions its jobs use (`uses: ./dir`; naming one any other way is
    refused), those they use, and the native-cache action; no other action;
  * every other file whose last matching rule in OWNERS names the artifact, or that no rule matches.

OWNERS and ARTIFACTS are claims about what each job reads. tools/tests/check-native-inputs.py, which
Static Checks runs, holds them to what the jobs are seen to read and checks how the jobs are wired.

Crates.io versions float within the declared ranges, and toolchains float within their pins. Neither
is part of the key. The cache action adds the week to the key so nothing reused is more than a week
old, and the release (npm_release.yml) builds with `fresh: true`.

Needs git, Python 3.8+ and cargo (no network: `metadata --no-deps --offline` fetches nothing).
"""
import argparse
import functools
import hashlib
import io
import json
import os
import posixpath
import re
import subprocess
import sys
import tarfile
import tempfile

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

BUILD_NATIVE = ".github/workflows/build-native.yml"
CACHE_ACTION = ".github/actions/native-cache"
NAPI_CRATES = ["crates/canvas-napi", "crates/canvas-svg-napi", "crates/audio-context-napi",
               "crates/canvas-media-napi", "crates/canvas-gamepad-napi"]


def napi_windows():
    return {"crates": NAPI_CRATES, "jobs": [(BUILD_NATIVE, "napi-windows")]}


def apple(artifact, parts, crate):
    """An xcframework built in parts: its lookup job, the jobs that build the parts, and its own."""
    return {"crates": [crate], "jobs": [(BUILD_NATIVE, f"{artifact}-cache"), (BUILD_NATIVE, parts),
                                        (BUILD_NATIVE, artifact)]}


# crates: the crate dirs it builds. jobs: the workflow and id of each job that builds it, when not
# just build-native.yml's job of the artifact's name. build-napi.sh and build-linux.sh name every
# Node-API crate, so each Node-API build keys them all.
ARTIFACTS = {
    "canvas-android": {"crates": ["crates/canvas-android"]},
    "canvas-ios": apple("canvas-ios", "canvas-apple", "crates/canvas-ios"),
    "canvas-svg-android": {"crates": ["crates/canvas-svg-android", "crates/canvas-svg-c"]},
    "canvas-svg-ios": apple("canvas-svg-ios", "canvas-svg-apple", "crates/canvas-svg-ios"),
    "audio-context-android": {},
    "audio-context-ios": {},
    "canvas-windows": {"crates": NAPI_CRATES},
    "canvas-svg-windows": napi_windows(),
    "audio-context-windows": napi_windows(),
    "canvas-media-windows": napi_windows(),
    "canvas-gamepad-windows": napi_windows(),
    "linux": {"crates": NAPI_CRATES},
    "unit-tests": {"crates": ["crates/canvas-c"], "jobs": [(".github/workflows/pull-request.yml", "unit-tests")]},
}
EVERYONE = tuple(ARTIFACTS)
NONE = ()
ANDROID = ("canvas-android", "canvas-svg-android", "audio-context-android")
APPLE = ("canvas-ios", "canvas-svg-ios", "audio-context-ios")
MOBILE = ANDROID + APPLE
WINDOWS = ("canvas-windows", "canvas-svg-windows", "audio-context-windows", "canvas-media-windows",
           "canvas-gamepad-windows")
LINUX = ("linux",)
# The jobs that run make, and those that fetch V8's headers and runtime.
MAKE = ("canvas-android", "canvas-ios", "canvas-svg-android", "canvas-svg-ios")
V8 = ("canvas-android", "canvas-ios", "canvas-svg-android")
SCRIPTS = [
    (("canvas-android",), "build-android"),
    (("canvas-svg-android",), "build-svg-android copy-svg-android"),
    (("canvas-android", "canvas-svg-android"), "build-android-v8-stub"),
    (("canvas-ios",), "canvas-build"),
    (("canvas-svg-ios",), "canvas-svg-build"),
    (("audio-context-ios",), "audio-context-build"),
    (V8, "download-v8"),
    (WINDOWS, "build-napi download-angle"),
    (LINUX, "build-linux"),
    # Release steps, which run on what the builds produced, the apps' download of the release's
    # frameworks (each release bumps its version), and the maintainers' scripts no job runs.
    (NONE, "place-native-artifacts release-ios-binaries download-ios-frameworks release repack android-build "
           "copy-android copy-android-libs busy-trace busy-bench build-headers build-ios canvas-build-debug "
           "canvas-svg-build-debug"),
]
# The package dirs each build reads: its native sources and the platform dir it writes its output
# to (with the package's app-side files there). On Apple, only the binary's dir: the apps' pods
# compile platforms/ios/src, and no framework build reads it.
PACKAGE_DIRS = {
    "canvas-android": ["packages/canvas/src-native/canvas-android", "packages/canvas/platforms/android"],
    "canvas-ios": ["packages/canvas/src-native/canvas-ios", "packages/canvas/platforms/ios/CanvasNativeBinary"],
    "canvas-svg-android": ["packages/canvas-svg/src-native/android", "packages/canvas-svg/platforms/android"],
    "canvas-svg-ios": ["packages/canvas-svg/src-native/ios", "packages/canvas-svg/platforms/ios/CanvasSVGBinary"],
    "audio-context-android": ["packages/audio-context/src-native/android", "packages/audio-context/platforms/android"],
    "audio-context-ios": ["packages/audio-context/src-native/ios",
                          "packages/audio-context/platforms/ios/AudioContextNativeBinary"],
    "canvas-windows": ["packages/canvas/platforms/windows"],
    "canvas-svg-windows": ["packages/canvas-svg/platforms/windows"],
    "audio-context-windows": ["packages/audio-context/platforms/windows"],
    "canvas-media-windows": ["packages/canvas-media/platforms/windows"],
    "canvas-gamepad-windows": ["packages/canvas-gamepad/platforms/windows"],
    "linux": ["packages/*/platforms/linux"],
}

# Which artifacts read the files outside the crates and .github: claims that
# tools/tests/check-native-inputs.py holds the jobs to. The last rule that matches a path decides,
# and a path no rule matches is read by every artifact. A rule is a file or dir (`*` stands for one
# path segment), or `*.ext` for every such file.
OWNERS = [
    # The npm packages' TypeScript, docs and manifests, the demos, the JS tooling, and the patches
    # maintainers apply by hand.
    *((f"*.{ext}", NONE) for ext in ("ts", "tsx", "js", "jsx", "mjs", "cjs", "css", "scss", "md", "json")),
    # The repo's own GitHub settings (its workflows and actions are keyed by job).
    (".github", NONE), ("apps", NONE), ("tools/demo", NONE),
    ("tools/assets", NONE), ("tools/schematics", NONE), ("tools/tests", NONE),
    ("tools/workspace-scripts.js", NONE), ("tools/patches", NONE), ("packages/.gitkeep", NONE),
    *((name, NONE) for name in (".editorconfig", ".eslintignore", ".gitignore", ".husky", ".npmrc", ".npsrc",
                                ".nxignore", ".prettierignore", ".prettierrc", ".travis.yml", ".vscode", ".yarn",
                                ".yarnrc.yml", "LICENSE")),
    # Only audio-context-android checks out a submodule (Oboe).
    (".gitmodules", ("audio-context-android",)),
    # Only Apple's builds read Swift packages and pods, which name V8's headers too.
    ("Makefile", MAKE), ("V8_RELEASE", V8), ("NATIVESCRIPT_IOS_RUNTIME_REF", V8), ("nativescript-v8", V8 + APPLE),
    ("CanvasNative.podspec", APPLE), ("NativeScript.podspec", APPLE), ("Package.swift", APPLE),
    *((f"tools/scripts/{name}.sh", who) for who, names in SCRIPTS for name in names.split()),
    # It only computes the keys.
    ("tools/scripts/native-inputs.py", NONE),
    # A package's platform dirs hold the files apps build from and the committed outputs; the
    # builds read none but their own output's (PACKAGE_DIRS).
    ("packages/*/src-native", MOBILE), ("packages/*/platforms", NONE),
    # Its native code ships committed (async-release.aar); no job builds it. The others are JavaScript.
    ("packages/canvas-polyfill", NONE),
    *((f"packages/canvas-{name}", NONE) for name in ("babylon", "chartjs", "phaser", "phaser-ce", "pixi", "three")),
    # No job runs it; the Node-API suites are under the crates.
    ("packages/canvas-gamepad/__test__", NONE),
    *((d, (artifact,)) for artifact, dirs in PACKAGE_DIRS.items() for d in dirs),
    # The C++ the Android CMake builds compile; the iOS frameworks do not (the apps' pods do).
    ("packages/canvas/platforms/ios/src/cpp", ("canvas-android",)),
    ("packages/canvas-svg/platforms/ios/src/cpp", ("canvas-svg-android",)),
    # canvassvg's CMake borrows canvas's JNI headers.
    ("packages/canvas/src-native/canvas-android/canvas/src/main/cpp", ("canvas-android", "canvas-svg-android")),
    # Test fixtures of the Node-API suites, which every Node-API build keys with the crates.
    ("apps/demo/src/assets/file-assets/audio", WINDOWS + LINUX),
    ("tools/demo/canvas/assets/webgpu/pano.mp4", WINDOWS + LINUX),
]

RUST_ROOT = re.compile(r"^(Cargo\.toml|Cargo\.lock|rust-toolchain(\.toml)?|\.cargo/.*)$")
# What cargo metadata says of a dependency that can change the versions resolved.
RESOLVES = ("name", "req", "source", "registry", "kind", "target", "optional", "uses_default_features", "features",
            "rename")
LOCAL_USE = re.compile(r"^\s*(?:-\s+)?uses:\s*['\"]?\./([^\s'\"@#]+)")
COMMENT = r"(?:\s+#.*)?"


class Failure(Exception):
    pass


def git(*args, data=None, binary=False):
    # From the repo's root: ls-tree and archive take paths relative to the dir git runs in.
    result = subprocess.run(["git", "-c", "core.quotepath=off", *args], input=data, capture_output=True, cwd=ROOT)
    if result.returncode != 0:
        raise Failure(f"git {' '.join(args[:2])}: {result.stderr.decode(errors='replace').strip()}")
    return result.stdout if binary else result.stdout.decode("utf-8")


@functools.lru_cache(maxsize=None)
def show(rev, path):
    return git("show", f"{rev}:{path}").replace("\r", "")


@functools.lru_cache(maxsize=None)
def tree_entries(rev):
    """{path: (type, `mode id`)} of every tree, blob and submodule at rev."""
    entries = {}
    for record in git("ls-tree", "-r", "-t", "-z", rev).split("\0"):
        if record:
            meta, path = record.split("\t", 1)
            mode, kind, oid = meta.split()
            entries[path] = (kind, f"{mode} {oid}")
    return entries


@functools.lru_cache(maxsize=None)
def tracked(rev):
    return frozenset(p for p, (kind, _) in tree_entries(rev).items() if kind != "tree")


def blob_id(text):
    data = text.encode("utf-8")
    return hashlib.sha1(b"blob %d\0" % len(data) + data).hexdigest()


def norm(path):
    """Resolves . and .. in a repo-relative path; a path above the repo is an error."""
    parts = []
    for part in path.replace("\\", "/").split("/"):
        if part in ("", "."):
            continue
        if part == "..":
            if not parts:
                raise Failure(f"'{path}' points above the repo")
            parts.pop()
        else:
            parts.append(part)
    return "/".join(parts)


def matches(rule, path):
    if rule.startswith("*."):
        return path.endswith(rule[1:])
    want, have = rule.split("/"), path.split("/")
    return len(have) >= len(want) and all(w in ("*", h) for w, h in zip(want, have))


@functools.lru_cache(maxsize=None)
def owners(path):
    """The artifacts that read a file outside the crates and .github, by OWNERS (cached: after
    changing OWNERS, call owners.cache_clear())."""
    found = EVERYONE
    for rule, who in OWNERS:
        if matches(rule, path):
            found = who
    return found


def indent(line):
    return len(line) - len(line.lstrip(" "))


def key_indent(line):
    """The column of a line's key, past any `- ` list marker."""
    stripped = line.lstrip(" ")
    return indent(line) + len(stripped) - len(stripped.lstrip("- "))


def yaml_lines(text):
    """(line, in_scalar) for each line, without YAML comments and blank lines outside block scalars
    (`key: |` text), which are kept as written."""
    out, scalar = [], None
    for line in text.split("\n"):
        if scalar is not None:
            if not line.strip() or indent(line) > scalar:
                out.append((line, True))
                continue
            scalar = None
        if not line.strip() or line.lstrip().startswith("#"):
            continue
        out.append((line, False))
        if re.search(r":\s*[|>][-+0-9]*" + COMMENT + r"\s*$", line):
            scalar = key_indent(line)
    return out


@functools.lru_cache(maxsize=None)
def workflow(path, rev):
    """(header, {job id: block}) of a workflow; refuses what a job block would not show."""
    problems, header, jobs, in_jobs, job = [], [], {}, False, None
    for line, in_scalar in yaml_lines(show(rev, path)):
        if not in_scalar and re.search(r"(:|-)\s+[&*][A-Za-z0-9_][A-Za-z0-9_-]*(\s|$)|<<:", line):
            problems.append(f"YAML anchor or alias: {line.strip()}")
        if re.match(r"^jobs:" + COMMENT + r"\s*$", line):
            in_jobs = True
            continue
        if not in_jobs:
            if "working-directory:" in line and not in_scalar:
                problems.append(f"workflow-wide working-directory: {line.strip()}")
            header.append(line)
            continue
        if not in_scalar and re.match(r"^\S", line):
            problems.append(f"top-level key after jobs: {line.strip()}")
        m = None if in_scalar else re.match(r"^  ([A-Za-z0-9_-]+):" + COMMENT + r"\s*$", line)
        if m:
            job = m.group(1)
            jobs[job] = []
        if job:
            jobs[job].append(line)
    if problems:
        raise Failure(f"{path}: " + "; ".join(problems))
    return "\n".join(header) + "\n", {j: "\n".join(lines) + "\n" for j, lines in jobs.items()}


def jobs_of(artifact):
    return ARTIFACTS[artifact].get("jobs", [(BUILD_NATIVE, artifact)])


def job_blocks(artifact, rev):
    """[(workflow path, job id, the workflow's header, the job's block)] of each job that builds it."""
    out = []
    for path, job_id in jobs_of(artifact):
        header, jobs = workflow(path, rev)
        if job_id not in jobs:
            raise Failure(f"no job '{job_id}' in {path} at {rev}")
        out.append((path, job_id, header, jobs[job_id]))
    return out


def steps_of(text, key_column, where):
    """Splits a job block or an action's runs: section at its `steps:` key into (its other lines, the
    job-level keys before and after the steps, and [step lines]). Steps may sit two columns in or,
    compact, at the key's own column."""
    lines = text.split("\n")
    start = next((i for i, line in enumerate(lines)
                  if re.match(rf"^{' ' * key_column}steps:" + COMMENT + r"\s*$", line)), None)
    if start is None:
        raise Failure(f"{where}: no steps")
    rest = [line for line in lines[start + 1:] if line.strip()]
    item = next((" " * c + "- " for c in (key_column + 2, key_column) if rest and rest[0].startswith(" " * c + "- ")), None)
    steps, end = [], len(lines)
    for i, line in enumerate(lines[start + 1:], start + 1):
        if item and line.startswith(item):
            steps.append([])
        elif line.strip() and indent(line) <= key_column:
            end = i
            break
        if steps:
            steps[-1].append(line)
    if not steps:
        raise Failure(f"{where}: no steps could be read under `steps:`")
    return lines[:start] + lines[end:], steps


@functools.lru_cache(maxsize=None)
def crate_dirs(rev):
    """Every dir with a Cargo.toml at rev, from git alone, so outputs without crates need no cargo."""
    return tuple(sorted(p[: -len("/Cargo.toml")] for p in tracked(rev) if p.endswith("/Cargo.toml")))


@functools.lru_cache(maxsize=None)
def crate_meta(rev):
    """(deps, names, crate dirs, requirements) at rev: each workspace member's path dependencies and
    each package name's dir, as `cargo metadata` reports them for the commit's manifests, every dir
    with a Cargo.toml, and what decides the versions cargo resolves for the workspace: each member's
    declared dependencies and rust-version."""
    tree = tracked(rev)
    root = sorted({"Cargo.toml", *(p for p in tree if RUST_ROOT.match(p))})
    archive = git("archive", "--format=tar", rev, "--", *root, *crate_dirs(rev), binary=True)
    with tempfile.TemporaryDirectory() as tmp:
        with tarfile.open(fileobj=io.BytesIO(archive)) as tar:
            if hasattr(tarfile, "data_filter"):
                tar.extractall(tmp, filter="data")
            else:
                tar.extractall(tmp)
        try:
            # Run outside the repo, so a committed rust-toolchain file does not make rustup install
            # its toolchain just to read manifests.
            result = subprocess.run(["cargo", "metadata", "--no-deps", "--format-version", "1", "--offline",
                                     "--manifest-path", os.path.join(tmp, "Cargo.toml")],
                                    capture_output=True, stdin=subprocess.DEVNULL, cwd=tempfile.gettempdir())
        except FileNotFoundError:
            raise Failure("cargo is not installed")
    if result.returncode != 0:
        raise Failure(f"cargo metadata at {rev}: {result.stderr.decode(errors='replace').strip()}")
    meta = json.loads(result.stdout)
    workspace = meta["workspace_root"].replace("\\", "/").rstrip("/") + "/"

    def rel(path):
        path = path.replace("\\", "/")
        if not path.startswith(workspace):
            raise Failure(f"path dependency outside the workspace: {path}")
        return path[len(workspace):]

    deps, names, requirements = {}, {}, {}
    for package in meta["packages"]:
        member = rel(posixpath.dirname(package["manifest_path"].replace("\\", "/")))
        deps[member] = [rel(dep["path"]) for dep in package["dependencies"] if dep.get("path")]
        names[package["name"]] = member
        # Named fields, so a cargo that reports more of them does not move every key.
        declared = [[dep.get(k) for k in RESOLVES] for dep in package["dependencies"]]
        requirements[member] = {"rust_version": package.get("rust_version"), "features": package.get("features"),
                                "dependencies": sorted(declared, key=json.dumps)}
    return deps, names, crate_dirs(rev), requirements


def closure(rev, start):
    """The crate dirs an artifact building start reads: start, the members they reach by path, every
    crate dir that is not a member, and the dirs the root manifest patches dependencies to (cargo
    metadata reports a patched dependency as a registry one, with no path). Nothing when start is
    empty."""
    if not start:
        return set()
    deps, _, dirs, _ = crate_meta(rev)
    seen, queue = set(), list(start)
    while queue:
        crate = queue.pop()
        if crate in seen:
            continue
        if crate not in deps:
            raise Failure(f"{crate} is not a workspace member at {rev}")
        seen.add(crate)
        queue.extend(deps[crate])
    return seen | {d for d in dirs if d not in deps} | patched(rev)


def patched(rev):
    """The dirs the root Cargo.toml's [patch.*] and [replace] tables point dependencies at."""
    found, table = set(), ""
    for line in show(rev, "Cargo.toml").split("\n"):
        header = re.match(r"^\s*\[+\s*([^\]]+?)\s*\]", line)
        if header:
            table = header.group(1)
        elif re.match(r"^(patch\.|replace$)", table):
            found.update(norm(p) for p in re.findall(r"\bpath\s*=\s*\"([^\"]+)\"", line))
    return found


def used_actions(rev, text, where):
    """The local actions a job uses, and those they use, with the native-cache action."""
    found, todo, tree = {CACHE_ACTION}, [text], tracked(rev)
    while todo:
        for line in todo.pop().split("\n"):
            m = LOCAL_USE.match(line)
            if m is None and re.search(r"\buses['\"]?\s*:\s*['\"]?\.\.?/", line):
                raise Failure(f"{where}: write a local action as `uses: ./dir` on its own line: {line.strip()}")
            p = norm(m.group(1)) if m else None
            if p is None or p in found:
                continue
            found.add(p)
            manifest = next((f"{p}/{n}" for n in ("action.yml", "action.yaml") if f"{p}/{n}" in tree), None)
            if manifest is None:
                raise Failure(f"{where}: {p} is used as an action but has no action.yml at {rev}")
            todo.append(show(rev, manifest))
    return found


def key_filter(artifact, rev):
    """Whether a tracked path is one of the artifact's inputs."""
    actions = set().union(*(used_actions(rev, block, f"{path}#{job_id}") for path, job_id, _, block in job_blocks(artifact, rev)))
    crates = closure(rev, ARTIFACTS[artifact].get("crates", []))
    dirs = sorted(crate_dirs(rev), key=len, reverse=True)

    def wanted(p):
        if p.startswith(".github/workflows/"):
            return False
        if any(p.startswith(a + "/") for a in actions):
            return True
        if p.startswith(".github/actions/"):
            return False
        crate = next((d for d in dirs if p.startswith(d + "/")), None)
        if crate is not None:
            return crate in crates
        if RUST_ROOT.match(p):
            return bool(crates)
        return artifact in owners(p)
    return wanted


def key_inputs(artifact, rev):
    """The artifact's inputs as `kind id path` lines: each wanted file by its mode and id, a dir whose
    files are all wanted by its tree's, the blocks of its jobs and their workflows' headers, and with
    crates, what the workspace's resolution reads."""
    wanted, entries = key_filter(artifact, rev), tree_entries(rev)
    children = {}
    for p in entries:
        children.setdefault(posixpath.dirname(p), []).append(p)

    @functools.lru_cache(maxsize=None)
    def whole(p):
        if entries[p][0] != "tree":
            return wanted(p)
        return all([whole(c) for c in children[p]])

    lines = {}
    for path, job_id, header, block in job_blocks(artifact, rev):
        lines[f"{path}#{job_id}"] = f"block {blob_id(block)}"
        lines[f"{path}#header"] = f"block {blob_id(header)}"
    if ARTIFACTS[artifact].get("crates"):
        lines["Cargo resolution"] = f"deps {blob_id(json.dumps(crate_meta(rev)[3], sort_keys=True))}"
    todo = [""]
    while todo:
        for p in children.get(todo.pop(), []):
            if whole(p):
                lines[p] = entries[p][1]
            elif entries[p][0] == "tree":
                todo.append(p)
    return "".join(f"{lines[p]} {p}\n" for p in sorted(lines))


def resolve(rev):
    try:
        return git("rev-parse", "--verify", "--quiet", f"{rev}^{{commit}}").strip()
    except Failure:
        raise Failure(f"no commit '{rev}'")


def main(argv):
    parser = argparse.ArgumentParser(prog="native-inputs.py", description="the cache keys of the native build outputs")
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--list", action="store_true")
    mode.add_argument("--explain", action="store_true")
    mode.add_argument("--verbose", action="store_true")
    parser.add_argument("--job")
    parser.add_argument("--variant")
    parser.add_argument("artifact", nargs="?")
    parser.add_argument("rev", nargs="?", default="HEAD")
    args = parser.parse_args(argv)
    out = sys.stdout.buffer
    if args.list:
        out.write("".join(f"{a}\n" for a in ARTIFACTS).encode())
        return 0
    if args.artifact not in ARTIFACTS:
        raise Failure(f"unknown artifact '{args.artifact}' (one of: {' '.join(ARTIFACTS)})")
    rev = resolve(args.rev)
    if args.job is not None:
        block = next((b for _, job_id, _, b in job_blocks(args.artifact, rev) if job_id == args.job), None)
        if block is None:
            raise Failure(f"job '{args.job}' does not build '{args.artifact}'")
        if (args.variant or "").strip() in ("", "null") and any(re.match(r"^\s+matrix:", l) for l in steps_of(block, 4, args.job)[0]):
            raise Failure(f"{args.job} is a matrix job: pass `variant: ${{{{ toJSON(matrix) }}}}`, or its legs share one key")
    lines = key_inputs(args.artifact, rev)
    if args.explain:
        out.write(lines.encode())
    else:
        if args.verbose:
            sys.stderr.buffer.write(lines.encode())
        out.write((blob_id(lines)[:32] + "\n").encode())
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main(sys.argv[1:]))
    except Failure as error:
        sys.stderr.write(f"native-inputs: {error}\n")
        sys.exit(1)
