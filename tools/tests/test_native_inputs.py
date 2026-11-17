#!/usr/bin/env python3
"""Tests for tools/scripts/native-inputs.py (the keys) and tools/tests/check-native-inputs.py with its
parts in native_check (the check that holds the keys to what the jobs read).

Owners: how OWNERS places a file. Commands, Yaml and Mentions: how the check reads workflow steps.
Each case is a way of running a file or make that, read wrong, would hide a read from the check.
Paths and Contexts: how the check finds the paths a file names and the values that change between
runs.

Goals: what the keys must do, checked on throwaway commits of the working tree, and that the check
catches a job reading what its key leaves out. They need cargo (and make, for the jobs that run
it), so they are skipped without it; Static Checks has both.
"""
import importlib.util
import os
import re
import shutil
import subprocess
import sys
import tempfile
import unittest
from unittest import mock

sys.dont_write_bytecode = True  # no __pycache__ beside the scripts
HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("check_native_inputs", os.path.join(HERE, "check-native-inputs.py"))
check = importlib.util.module_from_spec(spec)
spec.loader.exec_module(check)
from native_check import jobs, ni, paths, wiring  # noqa: E402 (beside this file; after no-bytecode is set)


def step(*lines):
    return ["      - " + lines[0], *("        " + line for line in lines[1:])]


def ran(*lines):
    _, files, guesses, make = jobs.commands(step(*lines), "test")
    return files, [path for _, path in guesses], make


class Owners(unittest.TestCase):
    def test_a_file_no_rule_places_is_read_by_every_artifact(self):
        self.assertEqual(ni.owners("tools/helpers/new.sh"), ni.EVERYONE)
        self.assertEqual(ni.owners("packages/canvas/src-native/new-platform/x.c"), ni.MOBILE)

    def test_the_last_matching_rule_decides(self):
        self.assertEqual(ni.owners("packages/canvas/index.ts"), ni.NONE)
        self.assertEqual(ni.owners("packages/canvas/src-native/canvas-ios/x.ts"), ("canvas-ios",))
        self.assertEqual(ni.owners("packages/canvas/platforms/ios/src/cpp/x.h"), ("canvas-android",))
        self.assertEqual(ni.owners("packages/canvas-media/platforms/linux/lib/x64/x.node"), ni.LINUX)

    def test_a_rule_matches_whole_path_segments(self):
        self.assertTrue(ni.matches("tools/demo", "tools/demo/x.png"))
        self.assertFalse(ni.matches("tools/demo", "tools/demos/x.png"))
        self.assertTrue(ni.matches("packages/*/platforms", "packages/canvas/platforms/android/x.aar"))


class Commands(unittest.TestCase):
    def test_interpreters_and_their_options(self):
        self.assertEqual(ran("run: bash -e tools/scripts/x.sh")[0], {"tools/scripts/x.sh"})
        self.assertEqual(ran("run: bash -o pipefail tools/scripts/x.sh")[0], {"tools/scripts/x.sh"})
        self.assertEqual(ran("run: py -3 tools/x.py")[0], {"tools/x.py"})
        self.assertEqual(ran("run: pwsh -File tools\\scripts\\x.ps1")[0], {"tools/scripts/x.ps1"})
        self.assertEqual(ran("run: bash -c './tools/scripts/x.sh'")[0], {"tools/scripts/x.sh"})
        self.assertEqual(ran("run: bash -c 'echo hi'")[0], set())

    def test_wrappers_and_their_options(self):
        self.assertEqual(ran("run: sudo -E ./tools/x.sh")[0], {"tools/x.sh"})
        self.assertEqual(ran("run: timeout -s KILL 5m ./tools/x.sh")[0], {"tools/x.sh"})
        self.assertEqual(ran("run: env -u X FOO=1 ./tools/x.sh")[0], {"tools/x.sh"})
        self.assertEqual(ran("run: nice -n 10 make android")[2], (["android"], []))
        self.assertEqual(ran("run: arch -arm64 make ios")[2], (["ios"], []))
        for lines in (["run: sudo apt-get install -y make cmake"], ["run: command -v make"], ["run: which make"]):
            with self.subTest(lines=lines):
                self.assertEqual(ran(*lines)[2], None)

    def test_workspace_prefixes(self):
        self.assertEqual(ran('run: bash "$GITHUB_WORKSPACE/tools/x.sh"')[0], {"tools/x.sh"})
        self.assertEqual(ran("run: ${{ github.workspace }}/tools/x.sh")[0], {"tools/x.sh"})

    def test_working_directory_anywhere_in_the_step(self):
        self.assertEqual(ran("working-directory: tools/scripts", "run: sh x.sh")[0], {"tools/scripts/x.sh"})
        self.assertEqual(ran("run: \"./x.sh\"", "working-directory: tools/scripts")[0], {"tools/scripts/x.sh"})

    def test_paths_run_directly(self):
        self.assertEqual(ran("run: ci/foo.sh")[0], {"ci/foo.sh"})
        self.assertEqual(ran("run: |", "  bash \\", "    ci/foo.sh")[0], {"ci/foo.sh"})
        self.assertEqual(ran("run: if true; then ./ci/foo.sh; fi")[0], {"ci/foo.sh"})

    def test_quoting_escapes_heredocs_and_redirections(self):
        self.assertEqual(ran("run: LLVM='C:\\Program Files\\LLVM' && echo \"a; b | c\"")[0], set())
        self.assertEqual(ran("run: bash \"tools/my scripts/x.sh\" 'a;b'")[0], {"tools/my scripts/x.sh"})
        self.assertEqual(ran("run: \"C:/Program Files/x/run.sh\"")[0], set())
        self.assertEqual(ran("run: |", "  echo \"a \\\" b\"", "  bash tools/x.sh")[0], {"tools/x.sh"})
        self.assertEqual(ran("run: |", "  cat > f <<'EOF'", "  it's", "  EOF", "  make android")[2], (["android"], []))
        self.assertEqual(ran("run: make android 2>&1 | tee build.log")[2], (["android"], []))

    def test_a_script_without_extension_must_exist(self):
        self.assertEqual(ran("working-directory: tools", "run: bash build")[1], ["tools/build"])

    def test_refusals(self):
        for lines in (["run: cd tools && ./x.sh"], ["run: python -m tools.x"], ["run: ./../x.sh"],
                      ["run: echo \"unclosed"], ["run: npm run build.canvas.ios"], ["run: |", "  cat <<EOF", "  x"],
                      ["run: cargo test --workspace"], ["run: cargo build --release"], ["run: chrt 10 make android"]):
            with self.subTest(lines=lines):
                self.assertRaises(ni.Failure, ran, *lines)
        self.assertEqual(ran("run: cargo test -p canvas-c --features 2d")[0], set())

    def test_cargo_names_every_crate_it_builds(self):
        for lines in (["working-directory: crates/canvas-c", "run: cargo build"], ["run: cargo build --manifest-path Cargo.toml"],
                      ["run: cargo zigbuild --workspace -p a"], ["run: cargo ndk -t arm64-v8a build"],
                      ["run: cargo -Z unstable-options build"], ["run: rustup run nightly cargo build"],
                      ["run: xcrun cargo build"], ["run: cross build"], ['run: "$REAL_CARGO" build'],
                      ["run: cargo build -p 'canvas-*'"], ['run: OUT="$(cargo build --message-format=json)"'],
                      ['run: echo "`cargo build`"'], ["run: cargo-zigbuild build --workspace"],
                      ["run: cargo-ndk -t arm64-v8a build"], ['run: "$CARGO" ndk -t arm64 build --release'],
                      ["run: cargo build --features new"], ["run: cargo run --release -- help"],
                      ["run: cargo test --profile install"], ["run: docker run img cargo build"]):
            with self.subTest(lines=lines):
                self.assertRaises(ni.Failure, ran, *lines)
        for line in ("run: cargo +nightly build -Z build-std=std --target x -p canvas-ios", "run: cargo build -p $CRATE",
                     "run: cargo install cbindgen@0.29.0 --locked", "run: cargo --version", "run: cargo clean",
                     'run: [ "$MODE" = cross ] && echo cross', "run: cargo-ndk --version",
                     'run: LIB="$(cargo build -p canvas-c --message-format=json)"'):
            with self.subTest(line=line):
                ran(line)

    def test_make(self):
        self.assertEqual(ran("run: make android")[2], (["android"], []))
        self.assertEqual(ran("run: make -j 4 ios tvos")[2], (["ios", "tvos"], []))
        self.assertEqual(ran("run: make -j android")[2], (["android"], []))
        self.assertEqual(ran("run: /usr/bin/make android X=1")[2], (["android"], ["X=1"]))
        self.assertEqual(ran("run: if true; then make android; fi")[2], (["android"], []))
        for lines in (["run: make"], ["run: make -j$(nproc) android"], ["run: make -C x android"],
                      ["run: cd x && make android"], ["working-directory: x", "run: make android"]):
            with self.subTest(lines=lines):
                self.assertRaises(ni.Failure, ran, *lines)

    def test_a_make_target_from_the_matrix_counts_once_per_leg(self):
        values = jobs.matrix_values(["    strategy:", "      matrix:", "        platform: [ios, 'tvos']"])
        self.assertEqual(values, {"platform": ["ios", "tvos"]})
        self.assertEqual(jobs.commands(step("run: make ${{ matrix.platform }}-svg"), "t", values)[3], (["ios-svg", "tvos-svg"], []))
        self.assertRaises(ni.Failure, jobs.commands, step("run: make ${{ matrix.target }}"), "t", values)
        self.assertRaises(ni.Failure, jobs.matrix_values, ["    strategy:", "      matrix:", "        platform: [ios,", "          tvos]"])

    def test_what_the_environment_can_change_in_the_makefile(self):
        self.assertEqual(jobs.make_environment("x:\n\t$(if $(CI),a.sh,b.sh)\n", []), ["CI"])
        self.assertEqual(jobs.make_environment("S ?= a.sh\nx:\n\t./tools/$(S)\n", []), ["S"])
        self.assertEqual(jobs.make_environment("x:\n\t./$(S)\n", ["S=a.sh"]), [])
        self.assertEqual(jobs.make_environment(
            "T ?= nightly\nf = echo $(1)\nx:\n\tcargo +$(T) $(call f,a) $(foreach v,a b,$(v)) $$(HOME)\n", []), [])
        for makefile in ("t:\n\tS=x.sh cargo test\nx:\n\t./$(S)\n", "x:\n\t./$(S:.sh=-2.sh)\n", "x:\n\t./$S\n",
                         "S ?= a\nx:\n\tinclude $(S).mk\n"):
            with self.subTest(makefile=makefile):
                self.assertEqual(jobs.make_environment(makefile, []), ["S"])
        self.assertEqual(jobs.make_environment("x: S := a.sh\ndefine B\necho\nendef\nx:\n\t./$(S) $(B)\n", []), [])


class Yaml(unittest.TestCase):
    def test_steps_with_a_comment_or_compact(self):
        for block in ("  j:\n    steps: # build\n      - run: a\n      - run: b\n",
                      "  j:\n    steps:\n    - run: a\n    - run: b\n"):
            with self.subTest(block=block):
                self.assertEqual(len(ni.steps_of(block, 4, "test")[1]), 2)

    def test_keys_after_the_steps_are_the_jobs(self):
        outside, _ = ni.steps_of("  j:\n    steps:\n      - run: a\n    defaults:\n      run:\n        working-directory: x\n", 4, "t")
        self.assertIn("        working-directory: x", outside)

    def test_no_steps_is_an_error(self):
        self.assertRaises(ni.Failure, ni.steps_of, "  j:\n    steps:\n    runs-on: x\n", 4, "test")

    def test_a_list_over_lines_is_refused_with_the_form_that_reads(self):
        with self.assertRaisesRegex(ni.Failure, re.escape("`needs: [a, b]`")):
            jobs.field(["    needs:", "      - canvas-ios-cache", "      - canvas-apple"], "needs")

    def test_run_block_with_a_comment_keeps_its_body(self):
        self.assertEqual(jobs.field(["      - run: | # build", "          sh x.sh", "          #define A 1"], "run"),
                         "          sh x.sh\n          #define A 1")

    def test_a_value_continued_on_plain_lines_is_an_error(self):
        for first, name in (("      - run:", "run"), ("      - run: echo x &&", "run"),
                            ("        if: steps.native.outputs.hit != 'true' &&", "if")):
            with self.subTest(first=first):
                self.assertRaises(ni.Failure, jobs.field, [first, "          make android"], name)

    def test_scalar_text_is_kept_and_comments_are_not(self):
        lines = [l for l, _ in ni.yaml_lines("a: 1 \n# note\nb: |\n  #define X 1\n\n  y\nc: 2")]
        self.assertEqual(lines, ["a: 1 ", "b: |", "  #define X 1", "", "  y", "c: 2"])


class Mentions(unittest.TestCase):
    def test_files_with_prefixes_not_dirs(self):
        tree = {"tools/scripts/a.sh", "packages/canvas/index.ts", "V8_RELEASE"}
        text = 'x "$ROOT/tools/scripts/a.sh" ${{ github.workspace }}/V8_RELEASE packages/canvas ./tools/scripts/a.sh.'
        self.assertEqual(jobs.mentions(text, [], tree), {"tools/scripts/a.sh", "V8_RELEASE"})

    def test_workflow_files_are_left_to_their_job_blocks(self):
        self.assertEqual(jobs.mentions("see .github/workflows/x.yml", [], {".github/workflows/x.yml"}), set())

    def test_relative_to_a_base_or_the_script_dir(self):
        tree = {"tools/scripts/a.sh", "tools/scripts/lib/b.sh"}
        self.assertEqual(jobs.mentions("sh a.sh", ["tools/scripts"], tree), {"tools/scripts/a.sh"})
        self.assertEqual(jobs.mentions('source "$(dirname "$0")/lib/b.sh"', [], tree, "tools/scripts"), {"tools/scripts/lib/b.sh"})
        self.assertEqual(jobs.mentions('. "$SCRIPT_DIR/lib/b.sh"', [], tree, "tools/scripts"), {"tools/scripts/lib/b.sh"})


ENTRIES = {p: ("tree" if "." not in p.rsplit("/", 1)[-1] else "blob", "") for p in (
    "packages", "packages/canvas", "packages/canvas/platforms", "packages/canvas/platforms/ios",
    "packages/canvas/platforms/ios/src", "packages/canvas/platforms/ios/src/cpp",
    "packages/canvas/platforms/ios/src/cpp/Caches.cpp", "packages/canvas/src-native",
    "packages/canvas/src-native/android", "packages/canvas/src-native/android/CMakeLists.txt",
    "apps", "apps/demo", "apps/demo/audio", "apps/demo/audio/a.wav", "tools", "tools/assets", "tools/assets/icons",
    "tools/assets/icons/a.png")}


def named(text, here="", shell=False):
    return paths.names(text, here, ENTRIES, shell=shell)


class Paths(unittest.TestCase):
    def test_a_long_line(self):
        self.assertEqual(len(paths.path_tokens(" ".join(f"$X{i}/a" for i in range(1200)), True)), 1)
        self.assertEqual(paths.path_tokens("a" * 200_000, True), ())

    def test_relative_to_the_file_and_after_a_variable(self):
        cpp = "packages/canvas/platforms/ios/src/cpp"
        here = "packages/canvas/src-native/android"
        self.assertEqual(named("include_directories(../../platforms/ios/src/cpp)", here), {cpp})
        self.assertEqual(named("set(DIR ../../platforms/ios/src/cpp)\nadd_library(x ${DIR}/Caches.cpp)", here), {cpp + "/Caches.cpp"})
        self.assertEqual(named("include(${PROJECT_ROOT}/../../packages/canvas/platforms/ios/src/cpp)"), {cpp})

    def test_from_a_dir_above_the_file(self):
        cpp = "packages/canvas/platforms/ios/src/cpp"
        here = "packages/canvas/src-native/android"
        self.assertEqual(named("include_directories(${CMAKE_CURRENT_SOURCE_DIR}/../../platforms/ios/src/cpp)", here), {cpp})
        self.assertEqual(named('include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../platforms/ios/src/cpp/Caches.cpp"))', here),
                         {cpp + "/Caches.cpp"})
        self.assertEqual(named('Path::new(root).join("../../platforms/ios/src/cpp")', here + "/src"), {cpp})

    def test_values_set_in_case_branches(self):
        text = "  a) PKG=packages/canvas ;;\n  b) PKG=tools/assets ;;\nesac\ncp $PKG/icons/a.png out"
        self.assertEqual(named(text), {"tools/assets/icons/a.png"})

    def test_dirs_globs_and_computed_names(self):
        self.assertEqual(named('cp -R tools/assets/ "$DEST/"'), {"tools/assets"})
        self.assertEqual(named("for f in tools/assets/*; do :; done"), {"tools/assets/icons"})
        self.assertEqual(named('cp "$ROOT/tools/assets/$NAME.svg" x'), {"tools/assets"})
        self.assertEqual(named('ROOT="$(cd "$(dirname "$0")/.." && pwd)"\ncp "$ROOT/tools/assets/$NAME.svg" x'), {"tools/assets"})

    def test_a_path_given_as_segments(self):
        self.assertEqual(named("const a = path.join(root, 'apps', 'demo', 'audio');"), {"apps/demo/audio"})

    def test_comments_by_the_kind_of_file(self):
        self.assertEqual(named(paths.code("x(); // mirrors tools/assets\n/* tools/assets */ y(\"tools/assets\");", "a.rs")),
                         {"tools/assets"})
        self.assertEqual(named(paths.code("# tools/assets\necho x # tools/assets\nls ${#a}", "a.sh")), set())
        self.assertIsNone(paths.code("see tools/assets", "packages/x/platforms/README.md"))

    def test_a_comment_marker_in_a_string_is_text(self):
        for text in ('g("x/*/y");\nread("tools/assets/icons/a.png");\n/* z */', 'f("a//b", read("tools/assets/icons/a.png"));'):
            with self.subTest(text=text):
                self.assertEqual(named(paths.code(text, "a.rs")), {"tools/assets/icons/a.png"})
        self.assertEqual(named(paths.code('s.source = "#{root}/tools/assets/icons/a.png"', "a.podspec")), {"tools/assets/icons/a.png"})
        self.assertEqual(paths.code("#include tools/assets", "a.unknown"), "#include tools/assets")

    def test_after_a_cd(self):
        self.assertEqual(named("cd packages/canvas/src-native/android\ncp ../../platforms/ios/src/cpp/Caches.cpp x"),
                         {"packages/canvas/src-native/android", "packages/canvas/platforms/ios/src/cpp/Caches.cpp"})

    def test_the_words_made_of_a_matrix_value(self):
        self.assertEqual(jobs.apart("cp a/b x/${{ matrix.n }}/y z"), ("cp a/b   z", "x/${{ matrix.n }}/y"))

    def test_an_artifacts_own_leg_of_a_shared_matrix(self):
        strategy = ["    strategy:", "      matrix:", "        module:", "          - name: canvas-svg", "            crate: svg-napi",
                    "          - name: audio", "            crate: audio-napi", "        arch: [x64, arm64]"]
        self.assertEqual(jobs.matrix_values(strategy, "audio-windows", "${{ matrix.module.name }}-windows"),
                         {"module.name": ["audio"], "module.crate": ["audio-napi"], "arch": ["x64", "arm64"]})

    def test_shell_writes_and_prints_are_not_reads(self):
        self.assertEqual(named('mkdir -p tools/assets && cp x/y tools/assets/icons/a.png; rm -rf tools/assets\n'
                               'echo tools/assets/icons/a.png; cat x > tools/assets/icons/a.png', shell=True), set())
        self.assertEqual(named("cp tools/assets/icons/a.png out; echo tools/assets >> list", shell=True),
                         {"tools/assets/icons/a.png", "tools/assets"})

    def test_shell_bare_names_exports_and_lists(self):
        self.assertEqual(named("ls apps; tar czf x.tgz tools", shell=True), {"apps", "tools"})
        self.assertEqual(named('export ASSETS="$PWD/tools/assets"'), {"tools/assets"})
        self.assertEqual(named("set(SRC a.c tools/assets/icons/a.png)"), {"tools/assets/icons/a.png"})
        self.assertEqual(named("pushd tools; popd; pushd apps; cp demo/audio/a.wav /tmp/x", shell=True), {"apps/demo/audio/a.wav"})
        self.assertEqual(named("OTHER_CFLAGS = -I$(SRCROOT)/../../platforms/ios/src/cpp", "packages/canvas/src-native/android"),
                         {"packages/canvas/platforms/ios/src/cpp"})

    def test_quoted_words_and_the_scripts_own_dir(self):
        caches = "packages/canvas/platforms/ios/src/cpp/Caches.cpp"
        self.assertIn(caches, named('cd "$(dirname "$0")/../packages/canvas/src-native/android"\n'
                                    'cp ../../platforms/ios/src/cpp/Caches.cpp x', "tools", shell=True))
        self.assertIn(caches, named('cd "$ROOT"/packages/canvas/src-native/android\ncat ../../platforms/ios/src/cpp/Caches.cpp', shell=True))
        self.assertIn("tools/assets", named('X="$ROOT"/tools/assets; cp -R "$X" out', shell=True))
        self.assertIn("tools/assets/icons/a.png", named('case $X in a) D=packages/canvas ;; b) D=tools ;; esac\n'
                                                        'cd "$D"\ncat assets/icons/a.png', shell=True))

    def test_a_default_a_dir_named_some_other_way_and_segments_over_lines(self):
        self.assertIn("tools/assets/icons/a.png", named('F="${F:-$ROOT/tools/assets/icons/a.png}"\ngrep x "$F"', shell=True))
        self.assertEqual(named('let p = format!("{}/tools/assets/icons/a.png", root);'), {"tools/assets/icons/a.png"})
        self.assertEqual(named("const a = root + '/apps/demo/audio';"), {"apps/demo/audio"})
        self.assertEqual(named("const a = path.join(\n  root,\n  'apps', 'demo', 'audio');"), {"apps/demo/audio"})

    def test_a_value_set_is_read_where_a_command_is_given_it(self):
        self.assertEqual(named("cat > cfg <<EOF\nicons=tools/assets/icons\nEOF", shell=True), {"tools/assets/icons"})
        self.assertEqual(named("ICONS=tools/assets/icons ./gen", shell=True), {"tools/assets/icons"})
        # A dir the script goes on to name parts of is not read whole.
        self.assertEqual(named("D=tools/assets\ncp $D/icons/a.png out", shell=True), {"tools/assets/icons/a.png"})

    def test_cmake_dirs_by_mode(self):
        cpp = "packages/canvas/platforms/ios/src/cpp"
        here = "packages/canvas/src-native/android"
        self.assertEqual(named('get_filename_component(P "../.." ABSOLUTE)\ninclude(${P}/platforms/ios/src/cpp)', here), {cpp})
        self.assertEqual(named('get_filename_component(P "../../x" DIRECTORY)\ninclude(${P}/platforms/ios/src/cpp)', here), {cpp})

    def test_a_matrix_value_is_one_segment_or_each_listed_value(self):
        self.assertEqual(named("cp packages/${{ matrix.name }}/platforms x"), {"packages/canvas/platforms"})
        self.assertEqual(jobs.each_leg("cp packages/${{ matrix.m.name }}/x/${{ matrix.arch }}", {"m.name": ["a"], "arch": ["x64", "arm64"]}),
                         "cp packages/a/x/x64 packages/a/x/arm64")
        strategy = ["    strategy:", "      matrix:", "        module:", "          - name: canvas-svg", "            crate: svg",
                    "          - name: audio", "        arch: [x64, arm64]", "        include:", "          - arch: x64",
                    "            target: x86_64"]
        self.assertEqual(jobs.matrix_values(strategy), {"module.name": ["canvas-svg", "audio"], "module.crate": ["svg"],
                                                          "arch": ["x64", "arm64"], "target": ["x86_64"]})


class Contexts(unittest.TestCase):
    def test_contexts_that_change_between_runs(self):
        for expression in ("github.sha", "toJSON(github)", "github['run_id']", "GitHub.SHA", "vars.X", "inputs.fresh"):
            with self.subTest(expression=expression):
                self.assertTrue(wiring.varying([expression]))
        self.assertEqual(wiring.varying(["matrix.target", "steps.native.outputs.hit != 'true'", "runner.os",
                                        "env.X", "github.workspace", "hashFiles('Cargo.toml')"]), [])

    def test_an_actions_inputs_are_its_jobs(self):
        self.assertEqual(wiring.varying(["inputs.save == 'true'"], in_action=True), [])

    def test_git_commands_that_read_the_commit(self):
        for line in ("git describe --tags", 'git -C "$ROOT" rev-parse --short HEAD', "git --no-pager log -1",
                     "git rev-list -1 HEAD", "git rev-parse @", "git rev-parse --short @", "git rev-parse origin/master",
                     "git ls-tree HEAD", "git cat-file -p HEAD", "git diff HEAD~1 --stat", "git branch --show-current",
                     'V=$(git rev-parse --short=8 "$REF")'):
            with self.subTest(line=line):
                self.assertTrue(wiring.reads_commit(line))
        for line in ("git rev-parse --show-toplevel", "if git rev-parse --show-toplevel >/dev/null 2>&1; then",
                     "git rev-parse --show-toplevel 2> /dev/null", "R=$(git rev-parse --show-toplevel)",
                     "git ls-files Cargo.toml", "git hash-object --stdin", "git diff --exit-code",
                     "git config --system core.longpaths true", "# the git mode of each entry"):
            with self.subTest(line=line):
                self.assertIsNone(wiring.reads_commit(line))


IDENTITY = {f"GIT_{who}_{what}": value for who in ("AUTHOR", "COMMITTER")
            for what, value in (("NAME", "native-inputs test"), ("EMAIL", "test@invalid"))}


def git(*args, env=None, data=None):
    run = subprocess.run(["git", *args], input=data, capture_output=True,
                         env={**os.environ, **IDENTITY, **(env or {})})
    if run.returncode:
        raise AssertionError(f"git {' '.join(args)}: {run.stderr.decode().strip()}")
    return run.stdout.decode().strip()


@unittest.skipUnless(shutil.which("cargo"), "needs cargo")
class Goals(unittest.TestCase):
    """The keys do what the change set out to: a TypeScript edit rebuilds nothing, a crate edit
    every platform that links it, a Windows script edit only Windows, a Makefile edit every make
    job, and a file nobody has placed everything. The check fails a job that reads what its key
    leaves out. The base is HEAD with the working tree on top, untracked files too, so a local run
    tests the change being made; CI's tree is clean."""

    @classmethod
    def setUpClass(cls):
        cls.tmp = tempfile.TemporaryDirectory()
        cls.index = {"GIT_INDEX_FILE": os.path.join(cls.tmp.name, "index")}
        git("read-tree", "HEAD", env=cls.index)
        git("add", "-A", env=cls.index)
        cls.base = git("commit-tree", git("write-tree", env=cls.index), "-p", "HEAD", "-m", "test base", env=cls.index)
        cls.keys = cls.all_keys(cls.base)

    @classmethod
    def tearDownClass(cls):
        cls.tmp.cleanup()

    @staticmethod
    def all_keys(rev):
        return {a: ni.key_inputs(a, rev) for a in ni.ARTIFACTS}

    def show(self, path):
        return git("show", f"{self.base}:{path}")

    def commit(self, path, text, mode="100644", base=None):
        base = base or self.base
        git("read-tree", base, env=self.index)
        blob = git("hash-object", "-w", "--stdin", data=text.encode())
        git("update-index", "--add", "--cacheinfo", f"{mode},{blob},{path}", env=self.index)
        return git("commit-tree", git("write-tree", env=self.index), "-p", base, "-m", "probe", env=self.index)

    def moved(self, rev):
        keys = self.all_keys(rev)
        return {a for a in keys if keys[a] != self.keys[a]}

    def moved_by(self, path):
        return self.moved(self.commit(path, self.show(path) + "\n# probe\n"))

    def edit(self, path, old, new):
        text = self.show(path)
        self.assertIn(old, text)
        return self.commit(path, text.replace(old, new, 1))

    def test_typescript(self):
        self.assertEqual(self.moved_by("packages/canvas/index.ts"), set())

    def test_crate(self):
        moved = self.moved_by("crates/canvas-core/src/lib.rs")
        self.assertLessEqual({"canvas-android", "canvas-ios", "canvas-windows", "linux", "unit-tests"}, moved)
        self.assertFalse({"audio-context-android", "audio-context-ios"} & moved)

    def test_windows_script(self):
        self.assertEqual(self.moved_by("tools/scripts/build-napi.sh"), set(ni.WINDOWS))

    def test_makefile(self):
        self.assertEqual(self.moved_by("Makefile"), set(ni.MAKE))

    def test_a_new_file_nobody_placed(self):
        self.assertEqual(self.moved(self.commit("tools/helpers/prep.sh", "echo prep\n")), set(ni.ARTIFACTS))

    def test_a_scripts_mode(self):
        path = "tools/scripts/build-napi.sh"
        self.assertEqual(self.moved(self.commit(path, self.show(path), "100755")), set(ni.WINDOWS))

    def test_a_requirement_in_any_member_moves_every_build_with_crates(self):
        manifest = "crates/canvas-gamepad-napi/Cargo.toml"
        pinned = self.edit(manifest, "[dependencies]", "[dependencies]\nlibc = \"=0.2.150\"")
        self.assertEqual(self.moved(pinned), {a for a in ni.ARTIFACTS if ni.ARTIFACTS[a].get("crates")})
        self.assertFalse({"canvas-ios", "unit-tests"} & self.moved_by(manifest))

    def test_a_patched_crate_keys_every_build_with_crates_also_as_a_member(self):
        member = self.edit("Cargo.toml", "members = [", 'members = [\n    "crates/vendor/cpal",')
        device = "crates/vendor/cpal/src/host/wasapi/device.rs"
        edited = self.commit(device, self.show(device) + "\n// probe\n", base=member)
        for artifact in ("linux", "audio-context-windows"):
            with self.subTest(artifact=artifact):
                self.assertNotEqual(ni.key_inputs(artifact, member), ni.key_inputs(artifact, edited))

    def test_a_key_from_another_dir(self):
        script = os.path.join(HERE, "..", "scripts", "native-inputs.py")
        out = subprocess.run([sys.executable, script, "audio-context-ios", self.base], cwd=HERE, capture_output=True, text=True)
        self.assertEqual(out.returncode, 0, out.stderr)

    def test_a_local_action_named_another_way_is_refused(self):
        rev = self.edit(ni.BUILD_NATIVE, "      - uses: ./.github/actions/setup-android-native",
                        "      - { uses: ./.github/actions/setup-android-native }")
        self.assertRaises(ni.Failure, ni.key_inputs, "canvas-android", rev)

    def problems(self, artifact, rev):
        return check.check_reads(artifact, rev)

    def test_check_fails_a_read_the_key_leaves_out(self):
        rev = self.edit(ni.BUILD_NATIVE, "run: ./tools/scripts/build-napi.sh ${{ matrix.target }} release-napi",
                        "run: ./tools/scripts/build-napi.sh ${{ matrix.target }} release-napi && cat V8_RELEASE")
        self.assertTrue(any("V8_RELEASE" in p for p in self.problems("canvas-windows", rev)))

    def test_check_fails_a_dir_a_script_copies(self):
        script = "tools/scripts/build-napi.sh"
        rev = self.commit(script, self.show(script) + '\ncp -R tools/assets/ "$DEST/"\n')
        self.assertTrue(any("tools/assets" in p for p in self.problems("canvas-windows", rev)))

    def test_check_fails_a_read_after_a_quoted_cd(self):
        script = "tools/scripts/build-napi.sh"
        rev = self.commit(script, self.show(script) + '\ncd "$(dirname "$0")/../../packages/canvas-svg/src-native/ios"\n'
                          "cp ../../../canvas/src-native/canvas-ios/CanvasNative/include/canvas_ios.h x\n")
        self.assertTrue(any("canvas_ios.h" in p for p in self.problems("canvas-windows", rev)))

    def test_check_fails_a_read_in_a_quoted_substitution(self):
        script = "tools/scripts/build-napi.sh"
        rev = self.commit(script, self.show(script) + '\nV8_TAG="$(cat "$REPO/V8_RELEASE")"\n')
        self.assertTrue(any("V8_RELEASE" in p for p in self.problems("canvas-windows", rev)))

    def test_a_large_fixture_of_a_crates_tests_is_not_scanned(self):
        rev = self.commit("crates/canvas-svg/tests/big.svg", "<svg/>\n" * 300_000)
        self.assertFalse(any("too large" in p for p in self.problems("canvas-svg-android", rev)))

    def test_check_fails_a_file_a_build_step_hashes(self):
        workflow = ".github/workflows/build-native.yml"
        text = self.show(workflow)
        build = "run: ./tools/scripts/build-napi.sh ${{ matrix.target }} release-napi\n"
        self.assertEqual(text.count(build), 1)
        rev = self.commit(workflow, text.replace(build, build.replace("run: ", "run: echo ${{ hashFiles('V8_RELEASE') }} && ")))
        self.assertTrue(any("V8_RELEASE" in p for p in self.problems("canvas-windows", rev)))

    def test_a_module_build_napi_writes_is_no_read(self):
        script = "tools/scripts/build-napi.sh"
        rev = self.commit(script, self.show(script).replace('echo "$DEST/$NAME.node"', 'ls -l "$DEST/$NAME.node"; du -sh "$DEST"'))
        self.assertEqual(self.problems("canvas-windows", rev), [])
        rev = self.commit(script, self.show(script) + '\ncp "$ROOT/packages/canvas/platforms/windows/THIRD_PARTY_NOTICES.txt" "$DEST/x"\n')
        self.assertTrue(any("THIRD_PARTY_NOTICES" in p for p in self.problems("canvas-svg-windows", rev)))

    def test_check_fails_a_step_of_an_action_that_runs_after_a_failure(self):
        action = ".github/actions/setup-windows-native/action.yml"
        text = self.show(action)
        at = text.index("    - name:", text.index("runs:"))
        for key, needle in (("continue-on-error: true", "continue-on-error"), ("if: always()", "status function")):
            with self.subTest(key=key):
                rev = self.commit(action, text[:at] + f"    - name: probe\n      {key}\n      shell: bash\n      run: 'true'\n" + text[at:])
                self.assertTrue(any(needle in p for p in self.problems("canvas-windows", rev)))

    def test_check_fails_a_value_of_the_run(self):
        script = "tools/scripts/build-napi.sh"
        for line, needle in (('echo "$GITHUB_SHA" > stamp', "GITHUB_SHA"), ("git describe --tags > stamp", "commit")):
            with self.subTest(line=line):
                rev = self.commit(script, self.show(script) + f"\n{line}\n")
                self.assertTrue(any(needle in p for p in self.problems("canvas-windows", rev)))

    def test_a_followed_script_is_held_to_the_refusals(self):
        script = "tools/scripts/build-napi.sh"
        for line in ("npx nx run canvas:build", "cargo build --workspace", "python3 -m gen", '[ -n "$X" ] && . "$DIR/$NAME.sh"'):
            with self.subTest(line=line):
                self.assertRaises(ni.Failure, self.problems, "canvas-windows", self.commit(script, self.show(script) + f"\n{line}\n"))
        self.problems("canvas-windows", self.commit(script, self.show(script) + '\n[ -f "$HOME/.cargo/env" ] && . "$HOME/.cargo/env"\n'))

    def test_check_holds_the_claims_about_fixtures(self):
        rule = "apps/demo/src/assets/file-assets/audio"
        saved = list(ni.OWNERS)
        ni.OWNERS[:] = [r for r in ni.OWNERS if r[0] != rule]
        ni.owners.cache_clear()
        try:
            self.assertEqual(len(ni.OWNERS), len(saved) - 1)
            self.assertTrue(any(rule in p for p in self.problems("canvas-windows", self.base)))
        finally:
            ni.OWNERS[:] = saved
            ni.owners.cache_clear()

    @unittest.skipUnless(shutil.which("make"), "needs make")
    def test_check_holds_the_claims_about_build_files(self):
        rule = "packages/canvas/platforms/ios/src/cpp"
        saved = list(ni.OWNERS)
        ni.OWNERS[:] = [r for r in ni.OWNERS if r[0] != rule]
        ni.owners.cache_clear()
        try:
            self.assertTrue(any(rule in p for p in self.problems("canvas-android", self.base)))
        finally:
            ni.OWNERS[:] = saved
            ni.owners.cache_clear()

    def test_the_wiring(self):
        guarded = "      - name: Build Rust libraries\n        if: steps.native.outputs.hit != 'true'\n        run: make android"
        for name, new in (("unguarded", "      - name: Build Rust libraries\n        run: make android"),
                          ("|| guard", guarded.replace("!= 'true'", "!= 'true' || always()")),
                          ("status function", guarded.replace("!= 'true'", "!= 'true' && !success()")),
                          ("status function in another case", guarded.replace("!= 'true'", "!= 'true' && Always()")),
                          ("continue-on-error", guarded + "\n        continue-on-error: true"),
                          ("run context", guarded.replace("run: make android", "run: echo ${{ github.sha }} && make android"))):
            with self.subTest(name=name):
                self.assertRaises(ni.Failure, wiring.check_wiring, self.edit(ni.BUILD_NATIVE, guarded, new))
        upload = "      - uses: actions/upload-artifact@v4\n        with:\n          name: canvas-android\n"
        for name, new in (("renamed upload", upload.replace("name: canvas-android", "name: android")), ("no upload", "")):
            with self.subTest(name=name):
                self.assertRaises(ni.Failure, wiring.check_wiring, self.edit(
                    ni.BUILD_NATIVE, upload + "          path: packages/canvas/platforms/android/canvas-release.aar\n"
                    "          if-no-files-found: error\n", new and new + "          path: packages/canvas/platforms/android/"
                    "canvas-release.aar\n          if-no-files-found: error\n"))
        self.assertRaises(ni.Failure, wiring.check_wiring,
                          self.edit(ni.BUILD_NATIVE, "          variant: ${{ toJSON(matrix) }}\n", ""))
        leg = "          - name: canvas-gamepad\n            crate: canvas-gamepad-napi\n            file: canvasgamepad.node\n"
        self.assertRaises(ni.Failure, wiring.check_wiring, self.edit(ni.BUILD_NATIVE, leg, leg + leg.replace("canvas-gamepad\n", "canvas-foo\n")))
        # GitHub evaluates expressions in an action's input descriptions and fails to load it.
        self.assertRaises(ni.Failure, wiring.check_wiring, self.edit(
            f"{ni.CACHE_ACTION}/action.yml", "    description: The job's entry", "    description: ${{ inputs.x }} The job's entry"))
        wiring.check_wiring(self.base)

    def test_the_wiring_of_an_artifact_built_in_parts(self):
        gate = "    needs: canvas-ios-cache\n    if: needs.canvas-ios-cache.outputs.hit != 'true'\n"
        combine = "    needs: [canvas-ios-cache, canvas-apple]\n    if: needs.canvas-ios-cache.outputs.hit != 'true'\n"
        for name, old, new in (("ungated part", gate, "    needs: canvas-ios-cache\n"),
                               ("|| gate", gate, gate.replace("!= 'true'", "!= 'true' || always()")),
                               ("own key", "          key: ${{ needs.canvas-ios-cache.outputs.key }}\n", ""),
                               ("saving first job", "          artifact: canvas-ios\n          path: canvas-ios.tar.gz\n"
                                                    "          restore-only: 'true'\n",
                                "          artifact: canvas-ios\n          path: canvas-ios.tar.gz\n"),
                               ("first job uploading on a miss", "        if: steps.native.outputs.hit == 'true'\n"
                                                                 "        with:\n          name: canvas-ios\n",
                                "        with:\n          name: canvas-ios\n"),
                               ("last job running after a failed part", combine,
                                combine.replace("'true'\n", "'true' && !cancelled()\n")),
                               ("last job not needing the parts", combine, combine.replace(", canvas-apple]", "]")),
                               ("if: continued on the next line", combine,
                                combine.replace("'true'\n", "'true' &&\n      always()\n"))):
            with self.subTest(name=name):
                self.assertRaises(ni.Failure, wiring.check_wiring, self.edit(ni.BUILD_NATIVE, old, new))
        self.assertIn(f"{ni.BUILD_NATIVE}#canvas-apple", ni.key_inputs("canvas-ios", self.base))

    def test_check_fails_a_build_file_naming_a_crate_outside_the_closure(self):
        rev = self.commit("packages/audio-context/src-native/android/extra.gradle", 'cargo { module = "../../../../crates/canvas-core" }\n')
        self.assertTrue(any("crates/canvas-core" in p for p in self.problems("audio-context-android", rev)))

    def test_a_shared_job_reads_with_each_artifacts_own_matrix_values(self):
        build = "        run: ./tools/scripts/build-napi.sh ${{ matrix.target }} release-napi ${{ matrix.module.crate }}"
        for line, flagged in (("cp -R packages/canvas-media/platforms/windows/x64/ x", True),
                              ("cp packages/canvas-media/platforms/windows/${{ matrix.arch }}/canvasmedia.node x", True),
                              ("cp -R packages/${{ matrix.module.name }}/platforms/windows/x64/ x", False)):
            with self.subTest(line=line):
                rev = self.edit(ni.BUILD_NATIVE, build, build.replace("run: ", f"run: {line} && "))
                self.assertEqual(any("canvas-media/platforms/windows" in p for p in self.problems("canvas-svg-windows", rev)), flagged)

    def test_an_output_without_crates_needs_no_cargo(self):
        with mock.patch.object(ni, "crate_meta", side_effect=AssertionError("ran cargo")):
            ni.key_inputs("audio-context-android", self.base)


if __name__ == "__main__":
    sys.exit(unittest.main(verbosity=1))
