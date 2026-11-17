#!/usr/bin/env python3
"""Holds the claims tools/scripts/native-inputs.py keys the native builds by (OWNERS, ARTIFACTS and
each artifact's crate closure) to what the jobs are seen to read, and checks that every cached job
is wired to its cache. Static Checks runs it.

  check-native-inputs.py [REV]    REV defaults to HEAD

A key leaves out only what OWNERS places elsewhere, the crates outside its closure, the other jobs'
blocks and the actions its job does not use, so a job can read what its key misses only across one
of those lines. Two passes look for reads across them.

Reading the job: its steps and job-level keys, its actions' steps, the files they run and the files
those mention, what `make -n` prints for the targets it runs, and the scripts in the package dirs
OWNERS gives the artifact, followed in turn. It fails when a file read that way is outside the key,
a crate named that way (by package name or dir) is outside the closure, or the job cannot be read:
a file run that is not tracked, a file run after `cd`, a path above the repo, a script argument
(`bash build`) that is no file, `python -m`, a package runner, cargo building crates it does not
name, an unclosed quote or heredoc, a value continued on plain lines, `make` in any form but
`make [-jN] [VAR=x] <targets>` from the repo root, a Makefile with conditionals, includes, $(shell)
or paths of the dir it runs in, a Makefile that reads a variable it does not set (or a `?=`
default in a path), which the environment would choose, a followed script that runs make (but the
third-party builds in OWN_MAKE) or sources a computed path, a local action without action.yml, a
workflow key after `jobs:`, a YAML anchor or alias, or a workflow- or job-wide working-directory.

Scanning the key: every text file in the key (up to 2 MB; the larger ones here are binaries) but the
workflows, actions and Makefile, and the step and job-key text the first pass reads, are searched for
paths that lead out of the key. A path counts relative to the repo or to the file's dir; after a
variable the file sets (`set(DIR ...)`, `DIR=...`), with each of its values; after any other
variable, from either place; with a glob or another variable standing for one path segment, or for
the dir it is in when that matches nothing; given as the segments of a join() or resolve() call;
and naming a dir, for every file under it. Comments and the values variables are set to are not
paths; a path made with a matrix value is not checked, since the legs of one job can be different
artifacts (each leg's values are in its key); and crates' tests, benches and examples count only
for the jobs in CARGO_TESTS, which run `cargo test`. This pass holds the claims about build files,
such as the C++ the Android CMake builds compile from the iOS package dir.

A path a file names but its job does not read there (one it writes, say) is listed in NOT_READ with
the reason, for both passes.

Both fail on values that differ between runs that share a key: a ${{ }} context other than matrix,
steps, env, strategy, runner and github.workspace, in any case and in `.` or `[ ]` form, in the
workflow's settings, the job's keys, its build steps and its actions' steps (where `inputs.` is the
job's own `with:`), and the run's GITHUB_ variables (GITHUB_SHA, GITHUB_REF, ...) in the text the
first pass reads.

And the wiring: each job that uploads an artifact in build-native.yml is one of an artifact's, and
one job of each artifact restores it: a native-cache step with `id: native` right after the
checkout, for its own artifact and the upload's path, with `variant: ${{ toJSON(matrix) }}` in a
matrix job, and every later step but the upload skipped on a hit (its `if:` is
`steps.native.outputs.hit != 'true'`, or that && more, with no ||). An artifact built by several
jobs (the xcframeworks) also has a lookup job, which is the checkout and the step with
`lookup-only: 'true'` and has its `hit` and `key` as outputs; each other job that builds a part of
it needs the lookup and has `if: needs.<lookup>.outputs.hit != 'true'` (or that && more, with no
||), and the restore passes `key: ${{ needs.<lookup>.outputs.key }}`, so both use the same week.
For the reader, a make target made of a matrix value counts once per value the strategy lists.

Needs git, Python 3.8+, cargo and make.
"""
import argparse
import collections
import functools
import importlib.util
import os
import posixpath
import re
import subprocess
import sys
import tempfile

sys.dont_write_bytecode = True  # no __pycache__ beside the key module
_spec = importlib.util.spec_from_file_location(
    "native_inputs", os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "scripts", "native-inputs.py"))
ni = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(ni)

# skia-cache names every platform's build scripts, only to hash them.
SKIA_CACHE = ".github/actions/skia-cache"
GUARD = "steps.native.outputs.hit != 'true'"
# Scripts that build a third-party library from its own sources with its own make.
OWN_MAKE = {f"packages/audio-context/src-native/ios/AudioContextNative/AudioContextNative/scripts/{name}"
            for name in ("build_opus_deps.sh", "build_opus_deps_ios.sh")}
SCRIPT_EXT = r"(?:sh|bash|py|js|mjs|cjs|ts|rb|pl|ps1)"
SHELLS = {"sh", "bash", "zsh", "dash", "source", "."}
# The option that makes each interpreter run code given inline rather than a file.
INLINE_CODE = {**{s: {"-c"} for s in SHELLS}, "python": {"-c"}, "python3": {"-c"}, "py": {"-c"},
               "node": {"-e", "--eval", "-p", "--print"}, "ruby": {"-e"}, "perl": {"-e", "-E"},
               "pwsh": {"-Command", "-c", "-EncodedCommand"}, "powershell": {"-Command", "-c", "-EncodedCommand"}}
INTERPRETERS = set(INLINE_CODE)
# Interpreter options whose value is the next argument (so the script is the one after it).
VALUE_OPTIONS = {"-o", "+o", "-O", "+O", "-W", "-X", "-r", "--require", "--import", "--loader",
                 "-ExecutionPolicy", "-ep"}
# Commands that run the command after them, with their options that take a value.
WRAPPERS = {"sudo": {"-u", "-g", "-C", "-D", "-h", "-p", "-r", "-t", "-U"}, "env": {"-u", "-C", "-S"},
            "nice": {"-n"}, "timeout": {"-s", "-k"}, "time": {"-f", "-o"}, "exec": {"-a"},
            "xargs": {"-n", "-I", "-P", "-L", "-s", "-d", "-E", "-a"}, "nohup": set(), "command": set(),
            "arch": set()}
KEYWORDS = {"if", "then", "elif", "else", "while", "until", "do", "!", "{", "}"}
# Commands whose arguments name a program without running it: lookups, output, package installs.
NAMES_ONLY = {"which", "type", "hash", "echo", "printf", "apt", "apt-get", "brew", "choco", "dnf", "yum",
              "pacman", "port", "apk", "winget", "zypper"}
PACKAGE_RUNNERS = {"npm", "npx", "yarn", "pnpm", "nx", "bun", "just"}
CARGO_BUILDS = {"build", "b", "test", "t", "check", "c", "run", "r", "bench", "clippy", "doc", "d", "rustc"}
WORKSPACE_PREFIX = re.compile(r"\$\{\{\s*github\.workspace\s*\}\}/|\$\{?(?:GITHUB_WORKSPACE|ROOT|REPO_ROOT|PWD)\}?/")
# A context in an expression, with the field it reads (`.x` or `['x']`), if any.
CONTEXT = re.compile(r"(?<![\w.-])(github|inputs|vars|secrets|needs|job|runner|matrix|steps|env|strategy)\b"
                     r"(\s*\.\s*[\w-]+|\s*\[\s*'[^']*'\s*\])?", re.I)
# Contexts that are the same on every run of a job leg, so a cached job may use them.
STEADY_CONTEXTS = {"matrix", "steps", "env", "strategy", "runner"}
# The run's variables whose values differ between runs of one commit.
RUN_VARIABLE = re.compile(r"\bGITHUB_(SHA|REF|REF_NAME|REF_TYPE|REF_PROTECTED|HEAD_REF|BASE_REF|EVENT_NAME|"
                          r"EVENT_PATH|RUN_ID|RUN_NUMBER|RUN_ATTEMPT|ACTOR|ACTOR_ID|TRIGGERING_ACTOR|WORKFLOW|"
                          r"WORKFLOW_REF|WORKFLOW_SHA|RETENTION_DAYS)\b")
# The artifacts whose job runs `cargo test` on its crates, which builds their tests and examples.
CARGO_TESTS = {"unit-tests"}
# Paths that a file in a key names but that its job does not read there: (the file, as an OWNERS
# rule; the whole path, where `*` stands for one segment; why).
NOT_READ = [
    ("tools/scripts/build-napi.sh", "packages/*/platforms/windows/*", "it writes each module's build there"),
    ("tools/scripts/build-napi.sh", "packages/*/platforms/windows/*/*",
     "it writes each module's build, and ANGLE's DLLs beside canvas's"),
    ("tools/scripts/download-v8.sh", "packages/canvas/src-native/canvas-android/canvas/src/main/cpp/include",
     "it writes the V8 headers it downloads there"),
    ("tools/scripts/download-v8.sh", "packages/canvas/src-native/canvas-ios/CanvasNative/include",
     "it writes the V8 headers it downloads there"),
    ("crates/canvas-napi/__test__", "packages/canvas", "the tests' titles name the package"),
    ("packages/canvas/src-native/canvas-ios/CanvasNative/.idea/workspace.xml", "packages/canvas/package.json",
     "it is the IDE's state, which no build reads"),
    ("packages/audio-context/src-native/ios/AudioContextNative/AudioContextNative/scripts/build_opus_deps.sh",
     "Makefile", "it runs third_party/Makefile, after a cd"),
]


def not_read(p, target):
    def whole(pattern):
        want, have = pattern.split("/"), target.split("/")
        return len(want) == len(have) and all(re.fullmatch(re.escape(w).replace(r"\*", "[^/]*"), h) for w, h in zip(want, have))
    return any(ni.matches(rule, p) and whole(named) for rule, named, _ in NOT_READ)


def field(step, name):
    """A step's `name:` value, inline or as a block scalar; "" when it has none."""
    for i, line in enumerate(step):
        m = re.match(rf"^\s*(?:-\s+)?{name}:\s*(.*)$", line)
        if not m:
            continue
        value = m.group(1)
        if re.match(r"^[|>][-+0-9]*" + ni.COMMENT + r"\s*$", value):
            body = []
            for rest in step[i + 1:]:
                if rest.strip() and ni.indent(rest) <= ni.key_indent(line):
                    break
                body.append(rest)
            return "\n".join(body)
        following = next((l for l in step[i + 1:] if l.strip()), "")
        if following and ni.indent(following) > ni.key_indent(line):
            raise ni.Failure(f"`{name}:` continues on the next lines; write it as `{name}: |` so it can be read")
        value = re.sub(r"\s+#.*$", "", value).strip()
        if len(value) > 1 and value[0] == value[-1] and value[0] in "'\"":
            value = value[1:-1]
        return value
    return ""


def is_dynamic(token):
    return re.search(r"[$*?{}\[\]]", token) is not None


def looks_like_file(token):
    return "/" in token or re.search(r"\." + SCRIPT_EXT + "$", token) is not None


# What a script calls its own dir: `$(dirname "$0")`, `$(dirname "${BASH_SOURCE[0]}")`,
# `${BASH_SOURCE[0]%/*}`, `$SCRIPT_DIR`.
SCRIPT_DIR_REF = re.compile(r"\$\(\s*dirname\s+\"?\$(?:0|\{BASH_SOURCE\[0\]\}|BASH_SOURCE)\"?\s*\)"
                            r"|\$\{BASH_SOURCE\[0\]%/\*\}|\$\{?SCRIPT_DIR\}?")


def mentions(text, bases, tree, script_dir=None):
    """The tracked files (not dirs) the text names, relative to the repo or to one of bases, and
    with script_dir, to what the script calls its own dir. Workflow files are left out: each job's
    own block is its input, and a script's message naming the ni.workflow (canvas-build.sh does) would
    otherwise rebuild a job for an edit to any other job."""
    found = set()
    text = WORKSPACE_PREFIX.sub("", text.replace("\\", "/"))
    if script_dir is not None:
        text = SCRIPT_DIR_REF.sub("@SCRIPT_DIR@", text)
    for token in re.findall(r"[^\s'\"`=;&|()<>,]+", text):
        token = token.rstrip(".:")
        own = token.startswith("@SCRIPT_DIR@/")
        if own:
            token = token[len("@SCRIPT_DIR@/"):]
        if is_dynamic(token):
            continue
        for base in ([script_dir] if own else ["", *bases]):
            try:
                path = ni.norm(posixpath.join(base, token))
            except ni.Failure:
                continue
            if path in tree and not path.startswith(".github/workflows/"):
                found.add(path)
    return found


def shell_commands(script, where):
    """The commands in shell text, each a list of its words with quotes removed. Commands end at
    ; & | ( ) ` $( and newlines outside quotes; text inside quotes stays in its word, so a word that
    ran something computed keeps its `$`. A backslash escapes a quote, `$`, a space or another
    backslash, and otherwise reads as `/` (the Windows jobs' paths). Heredoc bodies and redirection
    targets are dropped. An unclosed quote is an error rather than a script swallowed whole."""
    commands, words, word, quote, i = [], [], None, None, 0
    heredocs, drop_next = [], False

    def end_word():
        nonlocal word, drop_next
        if word is not None:
            if drop_next:
                drop_next = False
            else:
                words.append(word)
        word = None

    def end_command():
        nonlocal words, drop_next
        end_word()
        if words:
            commands.append(words)
        words, drop_next = [], False

    while i < len(script):
        c = script[i]
        nxt = script[i + 1] if i + 1 < len(script) else ""
        if quote:
            if c == quote:
                quote = None
            elif c == "\\" and quote == '"' and nxt in '"$`\\':
                word += nxt
                i += 1
            else:
                word += "/" if c == "\\" else c
        elif c == "\\" and nxt and nxt in "'\"$ \\":
            word = (word or "") + nxt
            i += 1
        elif c in "'\"":
            quote, word = c, word or ""
        elif c == "#" and word is None:
            while i < len(script) and script[i] != "\n":
                i += 1
            continue
        elif script.startswith("<<", i) and not script.startswith("<<<", i):
            end_word()
            m = re.match(r"<<-?\s*['\"]?([A-Za-z0-9_]+)['\"]?", script[i:])
            if not m:
                raise ni.Failure(f"{where}: cannot read the heredoc at `{script[i:i + 20].strip()}`")
            heredocs.append(m.group(1))
            i += m.end()
            continue
        elif c in "<>":
            if word is not None and word.isdigit():
                word = None
            end_word()
            drop_next = True
            while i + 1 < len(script) and script[i + 1] in "<>&":
                i += 1
        elif script.startswith(("&&", "||", "$("), i):
            end_command()
            i += 2
            continue
        elif c == "\n":
            end_command()
            while heredocs:
                delimiter = heredocs.pop(0)
                i += 1
                while i < len(script):
                    j = script.find("\n", i)
                    line = script[i:] if j < 0 else script[i:j]
                    i = len(script) if j < 0 else j
                    if line.strip() == delimiter:
                        break
                    i += 1
                else:
                    raise ni.Failure(f"{where}: the heredoc ending `{delimiter}` is not closed")
                i -= 1 if i < len(script) else 0
        elif c in ";&|()`":
            end_command()
        elif c.isspace():
            end_word()
        else:
            word = (word or "") + ("/" if c == "\\" else c)
        i += 1
    if quote:
        raise ni.Failure(f"{where}: a {quote} quote is never closed, so what runs after it cannot be read")
    if heredocs:
        raise ni.Failure(f"{where}: the heredoc ending `{heredocs[0]}` is not closed")
    end_command()
    return commands


def is_absolute(token):
    return token.startswith(("/", "~")) or re.match(r"^[A-Za-z]:/", token) is not None


def matrix_values(strategy):
    """{key: [values]} of the matrix keys a job's strategy lists inline (`platform: [ios, tvos]`)."""
    return {m.group(1): [v.strip().strip("'\"") for v in m.group(2).split(",")]
            for m in (re.match(r"^\s+([\w-]+):\s*\[([^\]]*)\]\s*$", line) for line in strategy) if m}


def each_leg(script, values):
    """The script with each word that holds a matrix value the strategy lists written once per value,
    so `make ${{ matrix.platform }}` reads as one make of every leg's target."""
    def legs(m):
        options = values.get(m.group(2))
        return " ".join(m.group(0).replace(m.group(1), v) for v in options) if options else m.group(0)
    return re.sub(r"[^\s'\"]*(\$\{\{\s*matrix\.([\w-]+)\s*\}\})[^\s'\"]*", legs, script)


def commands(step, where, values=None):
    """What a step's run: text runs: (wd, files, guesses, make targets and variables). files must
    exist; guesses are (what, path) for a script argument without `/` or extension, which has to."""
    wd = field(step, "working-directory")
    files, guesses, makes = set(), [], []
    script = WORKSPACE_PREFIX.sub("", each_leg(re.sub(r"\\\n", " ", field(step, "run")), values or {}))
    for line in script.split("\n"):
        if re.search(r"(^|[\s;&|(!{])(?:\S*/)?g?make\s[^\n]*(\$\(|`|\$\{\{)", line):
            raise ni.Failure(f"{where}: cannot follow make with a computed argument: `{line.strip()}`")
    walk(script, wd, where, files, guesses, makes, False)
    if len(makes) > 1:
        raise ni.Failure(f"{where}: runs make more than once; give each its own step")
    return wd, files, guesses, (makes[0] if makes else None)


def unwrap(tokens):
    """The command a shell command runs, past keywords, assignments and wrappers with their
    options; [] for `command -v`, which only looks one up."""
    while tokens:
        word = tokens[0]
        if word in WRAPPERS:
            tokens = tokens[1:]
            if word == "command" and tokens and tokens[0] in ("-v", "-V"):
                return []
            while tokens and tokens[0].startswith("-"):
                option, tokens = tokens[0], tokens[1:]
                if option in WRAPPERS[word] and tokens:
                    tokens = tokens[1:]
            if word == "timeout" and tokens and re.match(r"^\d+(\.\d+)?[smhd]?$", tokens[0]):
                tokens = tokens[1:]
        elif word in KEYWORDS or re.match(r"^[A-Za-z_][A-Za-z0-9_]*=", word):
            tokens = tokens[1:]
        else:
            break
    return tokens


def walk(script, wd, where, files, guesses, makes, after_cd):
    for tokens in shell_commands(script, where):
        command = " ".join(tokens)
        tokens = unwrap(tokens)
        if not tokens:
            continue
        name, args = tokens[0], tokens[1:]
        base = posixpath.basename(name)
        first = next((a for a in args if not a.startswith(("-", "+"))), "")
        # A make that a wrapper this does not know starts (`chrt 10 make`) would read as an argument.
        if base not in ("make", "gmake") and base not in NAMES_ONLY and any(
                posixpath.basename(a) in ("make", "gmake") for a in args):
            raise ni.Failure(f"{where}: cannot follow the make in `{command}`; run `make <targets>` itself")
        if base in PACKAGE_RUNNERS:
            raise ni.Failure(f"{where}: `{command}` runs what a package manifest says; run the script itself")
        if base == "cargo" and first in CARGO_BUILDS and not (wd or after_cd) and (
                {"--workspace", "--all"} & set(args) or not any(
                    a in ("-p", "--package", "--manifest-path") or a.startswith(("--package=", "--manifest-path="))
                    or (a.startswith("-p") and not a.startswith("--")) for a in args)):
            raise ni.Failure(f"{where}: `{command}` builds crates it does not name; name each with -p")
        if name in ("cd", "pushd"):
            after_cd = True
        elif base in ("make", "gmake"):
            makes.append(make_command(args, wd or after_cd, command, where))
        elif base in INTERPRETERS:
            if base in ("python", "python3", "py") and "-m" in args:
                raise ni.Failure(f"{where}: cannot tell which file `{command.strip()}` runs; run the file itself")
            if base in SHELLS and "-c" in args:
                code = args[args.index("-c") + 1] if args.index("-c") + 1 < len(args) else ""
                after_cd = walk(code, wd, where, files, guesses, makes, after_cd)
                continue
            if any(a in INLINE_CODE[base] for a in args):
                continue
            script_arg, skip_value = None, False
            for arg in args:
                if skip_value:
                    skip_value = False
                elif arg in VALUE_OPTIONS:
                    skip_value = True
                elif not arg.startswith(("-", "+")):
                    script_arg = arg
                    break
            if script_arg is None or is_dynamic(script_arg) or is_absolute(script_arg):
                continue
            if after_cd:
                raise ni.Failure(f"{where}: runs `{command.strip()}` after `cd`; give the step a working-directory instead")
            path = ni.norm(posixpath.join(wd, script_arg))
            if looks_like_file(script_arg):
                files.add(path)
            else:
                guesses.append((f"{where}: `{command.strip()}`", path))
        elif "/" in name and not is_absolute(name) and not is_dynamic(name):
            if after_cd:
                raise ni.Failure(f"{where}: runs `{name}` after `cd`; give the step a working-directory instead")
            files.add(ni.norm(posixpath.join(wd, name)))
    return after_cd


def make_command(args, moved, command, where):
    """([targets], [VAR=value]) of `make [-jN] [VAR=value] <targets>` from the repo root."""
    if moved or any(re.match(r"^(-C|-f|--directory|--file|--makefile)", a) for a in args):
        raise ni.Failure(f"{where}: only `make <targets>` from the repo root is followed: `{command.strip()}`")
    targets, variables, i = [], [], 0
    while i < len(args):
        arg = args[i]
        if arg in ("-j", "-l") and i + 1 < len(args) and re.match(r"^\d+$", args[i + 1]):
            i += 1
        elif "=" in arg and not arg.startswith("-"):
            variables.append(arg)
        elif not arg.startswith("-"):
            targets.append(arg)
        i += 1
    if not targets:
        raise ni.Failure(f"{where}: name the targets of `{command.strip()}`, so what it runs can be followed")
    return targets, variables


def make_environment(makefile, variables):
    """The variables a Makefile's commands can take from the environment: those it reads but never
    sets (a function's own, `$(call f,x)`'s $(1) and $(foreach x,...)'s x, are set), and its `?=`
    defaults where they make a path."""
    makefile = makefile.replace("$$", "")
    set_here = set(re.findall(r"^\s*(?:export\s+|override\s+)?([A-Za-z_][\w.-]*)\s*(?::{1,3}=|\+=|\?=|=)", makefile, re.M))
    set_here |= {v.split("=", 1)[0] for v in variables} | set(re.findall(r"\$\(foreach\s+([\w.-]+)\s*,", makefile))
    defaults = set(re.findall(r"^\s*([A-Za-z_][\w.-]*)\s*\?=", makefile, re.M))
    found = []
    for m in re.finditer(r"\$[({]([A-Za-z_][\w.-]*)(?=[)}])", makefile):
        name = m.group(1)
        if name not in set_here and name not in ("MAKE", "MAKEFLAGS", "MAKECMDGOALS", "SHELL"):
            found.append(name)
    for name in defaults:
        if re.search(rf"/\$[({{]{re.escape(name)}[)}}]|\$[({{]{re.escape(name)}[)}}]/", makefile):
            found.append(name)
    return sorted(set(found))


def make_commands(rev, targets, variables):
    """What `make -n` runs, with the commit's Makefile and no inherited variables. A Makefile that
    can run other commands than it prints (conditionals, includes, $(shell)), that names the dir
    make runs in, or whose commands the environment can change, is refused."""
    makefile = ni.show(rev, "Makefile")
    if re.search(r"^\s*(ifeq|ifneq|ifdef|ifndef|-?include|sinclude)\b|^[^#\n]*!="
                 r"|\$[({](shell|origin|wildcard|realpath|abspath|CURDIR)\b", makefile, re.M):
        raise ni.Failure("the Makefile has conditionals, includes, $(shell) or paths of the dir make runs "
                         "in: `make -n` cannot show what a job runs")
    for name in make_environment(makefile, variables):
        raise ni.Failure(f"the Makefile reads {name} from the environment, which `make -n` does not see: set "
                         f"it in the Makefile (with =, or ?= where it names no path) or pass it as {name}=...")
    with tempfile.TemporaryDirectory() as tmp:
        try:
            result = subprocess.run(["make", "-n", "-f", "-", *variables, *targets], input=makefile.encode(),
                                    cwd=tmp, capture_output=True, env={"PATH": os.environ.get("PATH", "")})
        except FileNotFoundError:
            raise ni.Failure("the job runs make, and make is not installed")
        output = result.stdout.decode("utf-8").replace("\r", "")
        if os.path.basename(tmp) in output:
            raise ni.Failure(f"make -n {' '.join(targets)} prints the dir it ran in, so what it runs cannot be followed")
    if result.returncode != 0:
        raise ni.Failure(f"make -n {' '.join(targets)}: {result.stderr.decode(errors='replace').strip()}")
    return output


def named_crates(rev, corpus):
    """The members the text names by package name, as a word of its own (`-p canvas-ios`, not
    packages/canvas/src-native/canvas-android), or by dir, after any prefix (`$ROOT/crates/x`)."""
    _, names, _, _ = ni.crate_meta(rev)
    return {member for name, member in names.items()
            if re.search(rf"(?<![A-Za-z0-9_/.-]){re.escape(name)}(?![A-Za-z0-9_/-])", corpus)
            or re.search(rf"(?<![A-Za-z0-9_-]){re.escape(member)}(?![A-Za-z0-9_-])", corpus)}


def discover(artifact, rev):
    """What the artifact's job is seen to read: ({path: where it was found}, all the text read, its
    actions' steps as [(manifest, step)], and the steps' and job keys' text whose mentions count, as
    [(where, text, its working-directory)])."""
    blocks = ni.job_blocks(artifact, rev)
    tree = ni.tracked(rev)
    reads, corpus, todo, guesses, makes, actions, executed, wd_of = {}, [b for *_, b in blocks], [], [], [], [], set(), {}
    action_steps, texts_read = [], []

    def own(p):
        who = ni.owners(p)
        return p.startswith("packages/") and artifact in who and who != ni.EVERYONE

    def reach(p, how, wd=""):
        if p not in reads:
            reads[p] = how
            wd_of[p] = wd
            todo.append(p)

    def take(step, step_where, scan, values=None):
        wd, files, step_guesses, make = commands(step, step_where, values)
        text = "\n".join(step)
        corpus.append(text)
        # What a cache key hashes is not read by the build.
        text = re.sub(r"hashFiles\([^)]*\)", "", text)
        actions.extend(ni.norm(m.group(1)) for m in map(ni.LOCAL_USE.match, step) if m)
        for p in files:
            if p not in tree:
                raise ni.Failure(f"{step_where}: runs {p}, which is not tracked at {rev}")
            executed.add(p)
            reach(p, step_where, wd)
        if scan:
            texts_read.append((step_where, text, wd))
            for p in mentions(text, [wd], tree):
                reach(p, step_where, wd)
        guesses.extend(step_guesses)
        if make:
            makes.append((step_where, make))

    jobs = set()
    for path, job_id, _, block in blocks:
        where = f"{path}#{job_id}"
        jobs.add(where)
        outside, steps = ni.steps_of(block, 4, where)
        for line in outside:
            if re.match(r"^\s*(?:-\s+)?working-directory:", line):
                raise ni.Failure(f"{where}: job-wide working-directory: {line.strip()}")
        # A matrix names every leg's values, each leg reads its own, and its key has them (variant).
        matrix = sections(outside, 4, {"strategy"})
        texts_read.append((f"{where} job keys", "\n".join(l for l in outside if l not in matrix), ""))
        for p in mentions("\n".join(outside), [], tree):
            reach(p, f"{where} job keys")
        for step in steps:
            take(step, where, True, matrix_values(matrix))
    for p in sorted(tree):
        if own(p) and p.endswith((".sh", ".bash")):
            reach(p, f"a script in {artifact}'s dirs")

    followed = {ni.CACHE_ACTION}
    while todo or guesses or makes or actions:
        if makes:
            make_where, (targets, variables) = makes.pop()
            if make_where not in jobs:
                raise ni.Failure(f"{make_where} runs make; run make from the job instead")
            output = make_commands(rev, targets, variables)
            take(["run: |", *("  " + l for l in output.split("\n"))], f"make -n {' '.join(targets)}", True)
            reach("Makefile", make_where)
            continue
        if actions:
            p = actions.pop()
            if p in followed:
                continue
            followed.add(p)
            manifest = next((f"{p}/{n}" for n in ("action.yml", "action.yaml") if f"{p}/{n}" in tree), None)
            if manifest is None:
                raise ni.Failure(f"{p} is used as an action but has no action.yml at {rev}")
            lines = [l for l, _ in ni.yaml_lines(ni.show(rev, manifest))]
            runs = next((i for i, l in enumerate(lines) if re.match(r"^runs:" + ni.COMMENT + r"\s*$", l)), None)
            if runs is None:
                raise ni.Failure(f"{manifest} has no runs:")
            # A JavaScript or Docker action's code lives in its dir, which is in the key whole.
            if any(re.match(r"^\s+using:\s*['\"]?composite", l) for l in lines[runs:]):
                for step in ni.steps_of("\n".join(lines[runs:]), 2, manifest)[1]:
                    take(step, manifest, p != SKIA_CACHE)
                    action_steps.append((manifest, step))
            continue
        if not todo:
            for what, p in guesses:
                if p not in tree:
                    raise ni.Failure(f"{what}: {p} does not exist, so what it runs is unknown")
                executed.add(p)
                reach(p, what.split(": ")[0])
            guesses = []
            continue
        p = todo.pop()
        if not (p in executed or re.search(r"\." + SCRIPT_EXT + "$", p) or p.startswith("tools/scripts/")):
            continue
        data = ni.git("show", f"{rev}:{p}", binary=True)
        if b"\0" in data:
            continue
        text = "\n".join(l for l in data.decode("utf-8", errors="replace").replace("\r", "").split("\n")
                         if not l.lstrip().startswith("#"))
        if p not in OWN_MAKE:
            if re.search(r"^\s*(?:\S*/)?g?make\s", text, re.M):
                raise ni.Failure(f"{p} runs make; run make from the job instead")
            for m in re.finditer(r"^\s*(?:source|\.)\s+(\S+)", text, re.M):
                source = SCRIPT_DIR_REF.sub("@", WORKSPACE_PREFIX.sub("", m.group(1).strip("'\"")))
                if is_dynamic(source):
                    raise ni.Failure(f"{p}: cannot tell which file `{m.group(0).strip()}` reads")
        corpus.append(text)
        here = posixpath.dirname(p)
        for found in mentions(text, [wd_of.get(p, ""), here], tree, here):
            reach(found, p, wd_of.get(p, ""))
    return reads, "\n".join(corpus), action_steps, texts_read


def check_reads(artifact, rev):
    """What the artifact's job reads that its key leaves out, by both passes, and the values it
    reads that differ between runs that share its key."""
    reads, corpus, action_steps, texts_read = discover(artifact, rev)
    wanted = ni.key_filter(artifact, rev)
    problems = [f"{artifact}: {how} reads {p}, which its key leaves out (OWNERS gives it to "
                f"{', '.join(ni.owners(p)) or 'no artifact'})" for p, how in sorted(reads.items())
                if not wanted(p) and not not_read(how, p)]
    crates = ni.closure(rev, ni.ARTIFACTS[artifact].get("crates", []))
    for crate in sorted(named_crates(rev, corpus) - crates):
        problems.append(f"{artifact}: its job names {crate}, which is not among its crates in ARTIFACTS")
    if artifact not in CARGO_TESTS and re.search(r"\bcargo\s+(?:\+\S+\s+)?(?:test|t|bench)\b", corpus):
        problems.append(f"{artifact}: its job runs cargo test, which builds its crates' tests and examples: "
                        f"add it to CARGO_TESTS")
    for name in sorted(set(RUN_VARIABLE.findall(corpus))):
        problems.append(f"{artifact}: its job reads GITHUB_{name}, which can change between runs that share a key")
    for manifest, step in action_steps:
        if not moves_cache(step):
            for context in varying(expressions([], [step]), in_action=True):
                problems.append(f"{artifact}: {manifest} uses {context}, which can change between runs that share a key")
    entries = ni.tree_entries(rev)
    for where, text, wd in texts_read:
        problems += crossings(artifact, rev, wanted, where, wd, names(text, wd, entries) - set(reads))
    return problems + scan(artifact, rev, wanted)


def moves_cache(step):
    return re.match(r"^actions/cache(/(restore|save))?@", field(step, "uses")) is not None


def expressions(lines, steps):
    """The ${{ }} expressions in the lines and steps, with each step's bare `if:`."""
    texts = [*lines, *("\n".join(step) for step in steps)]
    return [e for text in texts for e in re.findall(r"\$\{\{(.*?)\}\}", text)] + [field(step, "if") for step in steps]


def varying(found, in_action=False):
    """The contexts the expressions read that can differ between runs of a job leg. In an action,
    `inputs.` is the job's own `with:`, which is checked where the job is."""
    out = []
    for expression in found:
        for m in CONTEXT.finditer(expression):
            name, key = m.group(1).lower(), re.sub(r"[\s.\[\]']", "", m.group(2) or "").lower()
            if not (name in STEADY_CONTEXTS or (name, key) == ("github", "workspace") or (name == "inputs" and in_action)):
                out.append(" ".join(m.group(0).split()))
    return out


def check_wiring(rev):
    """Each job that uploads an artifact in build-native.yml is one of an artifact's, and each
    artifact's jobs cache it. One job restores it: a native-cache step with `id: native` right after
    the checkout, for that artifact and the upload's path, `variant: ${{ toJSON(matrix) }}` in a
    matrix job, and every later step but the upload skipped on a hit. An artifact built by several
    jobs also has a lookup job (checkout, then the step with `lookup-only: 'true'`, whose `hit` and
    `key` are the job's outputs); its other jobs need it and are skipped on its hit, and the restore
    takes its key. No job of an artifact uses a value that changes from run to run where it could
    change what is built."""
    problems, artifacts_of, parsed = [], {}, {}
    for artifact in ni.ARTIFACTS:
        for job in ni.jobs_of(artifact):
            artifacts_of.setdefault(job, []).append(artifact)
    for path in sorted({p for p, _ in artifacts_of} | {ni.BUILD_NATIVE}):
        header, jobs = ni.workflow(path, rev)
        settings = [l for l in sections(header.split("\n"), 0, {"env", "defaults"})
                    if l.strip() != "NATIVE_CACHE_FRESH: ${{ inputs.fresh }}"]
        for job_id, block in jobs.items():
            where = f"{path}#{job_id}"
            try:
                outside, steps = ni.steps_of(block, 4, where)
            except ni.Failure:
                if (path, job_id) in artifacts_of:
                    raise
                continue
            cache = next((s for s in steps if re.search(r"uses:\s*\./\.github/actions/native-cache\b", "\n".join(s))), None)
            uploads = [s for s in steps if re.match(r"^actions/upload-artifact@", field(s, "uses"))]
            if (path, job_id) not in artifacts_of:
                if path == ni.BUILD_NATIVE and (uploads or cache):
                    problems.append(f"{where}: builds an artifact, but no entry in ARTIFACTS lists this job")
                continue
            parsed[path, job_id] = (outside, cache)
            # Steps that only move caches, like the upload, do not change what is built; nor do the
            # header's and the job's keys other than these.
            builds = [s for s in steps if s is not cache and s not in uploads and not moves_cache(s)]
            job_settings = settings + sections(outside, 4, {"runs-on", "env", "defaults", "strategy", "container", "services"})
            for context in varying(expressions(job_settings, builds)):
                problems.append(f"{where}: uses {context}, which can change between runs that share a key")
            if cache is None:
                continue
            if field(cache, "id") != "native":
                problems.append(f"{where}: the native-cache step needs `id: native`")
            artifact = field(cache, "artifact")
            if "${{" not in artifact and artifact not in artifacts_of[path, job_id]:
                problems.append(f"{where}: native-cache artifact '{artifact}' is not one of this job's entries")
            if any(not re.match(r"^actions/checkout@", field(s, "uses")) for s in steps[:steps.index(cache)]):
                problems.append(f"{where}: only the checkout may come before the native-cache step")
            if field(cache, "lookup-only") == "true":
                if steps[steps.index(cache) + 1:]:
                    problems.append(f"{where}: a lookup job ends with its lookup")
                outputs = dict(m.groups() for m in (re.match(r"^\s+([\w-]+):\s*(.*?)\s*$", l)
                                                    for l in sections(outside, 4, {"outputs"})[1:]) if m)
                for name in ("hit", "key"):
                    if re.sub(r"\s", "", outputs.get(name, "")) != f"${{{{steps.native.outputs.{name}}}}}":
                        problems.append(f"{where}: a lookup job's outputs are `{name}: ${{{{ steps.native.outputs.{name} }}}}`")
                continue
            if any(re.match(r"^\s+matrix:", l) for l in outside) and re.sub(r"\s", "", field(cache, "variant")) != "${{toJSON(matrix)}}":
                problems.append(f"{where}: a matrix job passes `variant: ${{{{ toJSON(matrix) }}}}`")
            for upload in uploads:
                if field(upload, "path").rstrip("/") != field(cache, "path").rstrip("/"):
                    problems.append(f"{where}: uploads {field(upload, 'path')} but caches {field(cache, 'path')}")
            for step in steps[steps.index(cache) + 1:]:
                condition = bare(field(step, "if"))
                if step in uploads:
                    if condition:
                        problems.append(f"{where}: the upload must run on a hit too")
                elif not guarded(condition, GUARD):
                    problems.append(f"{where}: `{step[0].strip()}` runs on a hit (its `if:` must be "
                                    f"{GUARD}, or that && more, with no ||)")
    for artifact in ni.ARTIFACTS:
        jobs = ni.jobs_of(artifact)
        if any(job not in parsed for job in jobs):
            continue
        lookups = [job for job in jobs if parsed[job][1] and field(parsed[job][1], "lookup-only") == "true"]
        restores = [job for job in jobs if parsed[job][1] and job not in lookups]
        parts = [job for job in jobs if parsed[job][1] is None]
        if len(restores) != 1:
            problems.append(f"{artifact}: one of its jobs restores it with the native-cache step")
        if not lookups and not parts:
            if restores and field(parsed[restores[0]][1], "key"):
                problems.append(f"{artifact}: its native-cache step computes its own key")
            continue
        if len(lookups) != 1:
            problems.append(f"{artifact}: it is built by several jobs, so one of them looks it up (`lookup-only: 'true'`)")
            continue
        lookup = lookups[0][1]
        gate = f"needs.{lookup}.outputs.hit != 'true'"
        for path, job_id in parts:
            outside = parsed[path, job_id][0]
            if lookup not in needs(outside) or not guarded(bare(job_key(outside, "if")), gate):
                problems.append(f"{path}#{job_id}: builds part of {artifact}, so it needs {lookup} and its `if:` "
                                f"is {gate}, or that && more, with no ||")
        for path, job_id in restores:
            outside, cache = parsed[path, job_id]
            if lookup not in needs(outside) or re.sub(r"\s", "", field(cache, "key")) != f"${{{{needs.{lookup}.outputs.key}}}}":
                problems.append(f"{path}#{job_id}: restores {artifact} under the key {lookup} looked up: it needs "
                                f"{lookup} and passes `key: ${{{{ needs.{lookup}.outputs.key }}}}`")
    if problems:
        raise ni.Failure("\n  ".join(["native-cache wiring:", *sorted(set(problems), key=problems.index)]))


def bare(condition):
    return re.sub(r"^\$\{\{\s*|\s*\}\}$", "", " ".join(condition.split()))


def guarded(condition, guard):
    """Whether an `if:` holds the guard, alone or && more, with no ||."""
    return re.match(rf"^{re.escape(guard)}\s*(&&|$)", condition) is not None and "||" not in condition


def job_key(outside, name):
    """A job-level key's inline value; '' when the job has none."""
    lines = sections(outside, 4, {name})
    return field(lines[:1], name) if lines else ""


def needs(outside):
    return {n.strip() for n in job_key(outside, "needs").strip("[]").split(",") if n.strip()}


def sections(lines, column, keys):
    """The lines under the given keys at a column, the key lines included."""
    out, current = [], None
    for line in lines:
        if line.strip() and ni.indent(line) <= column:
            m = re.match(r"^\s*([A-Za-z0-9_-]+):", line) if ni.indent(line) == column else None
            current = m.group(1) if m else None
        if current in keys:
            out.append(line)
    return out


COMMENT_LINE = re.compile(r"^\s*(#|//|/\*|\*|<!--)")
# Where a file sets a variable to a path: a shell assignment anywhere on a line, CMake's set(), and
# get_filename_component() with DIRECTORY, whose value is the dir above the path it is given.
SHELL_SET = re.compile(r"(?:^|[\s;&|()])(?:export\s+|local\s+|readonly\s+|declare\s+)?([A-Za-z_]\w*)="
                       r"(\"[^\"]*\"|'[^']*'|[^\s;&|()]*)")
CMAKE_SET = re.compile(r"^\s*(set|get_filename_component)\(\s*([A-Za-z_]\w*)\s+\"?([^\s\")]+)", re.I)
# A variable: ${{ }}, then the named forms (`${X}`, `$(X)`, `$X`), then any other.
VARIABLE = re.compile(r"\$\{\{[^}]*\}\}|\$\{([A-Za-z_]\w*)\}|\$\(([A-Za-z_]\w*)\)|\$([A-Za-z_]\w*)"
                      r"|\$\{[^}]*\}|\$\([^)]*\)|@[A-Za-z_]\w*@|%[A-Za-z_]\w*%")
# A matrix value is one leg's, which its key holds (variant); a path made with one is not checked,
# as the legs of one job can be different artifacts.
MATRIX_VALUE = re.compile(r"\$\{\{\s*matrix\.[^}]*\}\}")
TOKEN = re.compile(r"[^\s'\"`=;,(){}<>\[\]|&:!]*/[^\s'\"`=;,(){}<>\[\]|&:!]*")
# A path given as its segments: path.join(root, 'apps', 'demo'), os.path.join(ROOT, "x").
JOIN = re.compile(r"\b(?:join|resolve)\(([^()]*)\)")
WILD, LEG = "\x01", "\x02"
WILDS = r"[*?" + WILD + "]"


@functools.lru_cache(maxsize=None)
def texts(rev):
    """{path: text} of every tracked text file at rev up to 2 MB (those above it here are binaries)."""
    ids = {}
    for record in ni.git("ls-tree", "-r", "-l", "-z", rev).split("\0"):
        if record:
            meta, path = record.split("\t", 1)
            _, kind, oid, size = meta.split()
            if kind == "blob" and int(size) <= 2_000_000:
                ids[path] = oid
    missing = sorted({oid for oid in ids.values() if oid not in BLOBS})
    data = ni.git("cat-file", "--batch", data="".join(oid + "\n" for oid in missing).encode(), binary=True)
    i = 0
    for oid in missing:
        end = data.index(b"\n", i)
        size = int(data[i:end].split()[2])
        blob = data[end + 1:end + 1 + size]
        i = end + 2 + size
        BLOBS[oid] = None if b"\0" in blob[:8000] else blob.decode("utf-8", errors="replace").replace("\r", "")
    return {p: BLOBS[oid] for p, oid in ids.items() if BLOBS[oid] is not None}


BLOBS = {}  # each blob's text, None for a binary, kept across revs
MATCHED = {}  # the paths each pattern with a glob or variable matches, by (pattern, tree)


def expand(text, values):
    """The text with each variable the file sets replaced by each of its values in turn, and any
    other variable by WILD (a matrix value by LEG)."""
    text = MATRIX_VALUE.sub(LEG, text)
    m = VARIABLE.search(text)
    if m is None:
        return [text]
    rests = expand(text[m.end():], values)
    name = m.group(1) or m.group(2) or m.group(3)
    options = (values.get(name) if name else None) or [WILD]
    if len(options) * len(rests) > 64:
        options = [WILD]
    return [text[:m.start()] + option + rest for option in options for rest in rests]


def resolve_token(token, here, entries):
    """The tracked files and dirs a path in a file can name: from the repo or the file's dir, with
    WILD and globs standing for one segment, and from either after an unknown variable (with the
    `..` after it, which leads from somewhere unknown). A computed name that matches nothing names
    the dir it is in."""
    token = re.sub(r"^(" + WILD + r"/(\.\./)*)+", "", token).rstrip(".")
    if token.startswith("/") or "//" in token or LEG in token:
        return set()
    segments = [s for s in token.split("/") if s]
    if not any(re.search(r"[A-Za-z]", s) and not re.search(WILDS, s) for s in segments):
        return set()
    found = set()
    for base in ("", here):
        try:
            path = ni.norm(posixpath.join(base, *segments))
        except ni.Failure:
            continue
        if not re.search(WILDS, path):
            if path in entries:
                found.add(path)
            continue
        parts = path.split("/")
        if (path, id(entries)) not in MATCHED:
            pattern = re.compile("^" + "/".join("[^/]*".join(map(re.escape, re.split(WILDS + "+", s))) for s in parts) + "$")
            MATCHED[path, id(entries)] = {p for p in entries if pattern.match(p)}
        matched = MATCHED[path, id(entries)]
        literal = "/".join(parts[:next(i for i, s in enumerate(parts) if re.search(WILDS, s))])
        found |= matched or ({literal} if literal in entries else set())
    return found


def names(text, here, entries):
    """The tracked files and dirs the text's paths name, from the repo or from here."""
    return set().union(*(resolve_token(token, here, entries) for token in path_tokens(text)))


@functools.lru_cache(maxsize=None)
def path_tokens(text):
    """The paths the text names outside comments and the values it sets variables to, with the
    variables it sets expanded."""
    values, found = {}, set()

    def define(name, value):
        # A value that is not a plain path (a command's output, say) is unknown.
        values.setdefault(name, [])
        values[name] += [v if re.match(r"^[\w./@+" + WILD + "-]*$", v) else WILD
                         for v in expand(value.strip("'\""), values) if v not in values[name]]

    for line in text.split("\n"):
        if COMMENT_LINE.match(line):
            continue
        cmake = CMAKE_SET.match(line)
        if cmake:
            define(cmake.group(2), cmake.group(3) + ("/.." if cmake.group(1).lower() != "set" else ""))
            continue
        for m in SHELL_SET.finditer(line):
            define(m.group(1), m.group(2))
        for expanded in expand(SHELL_SET.sub(" ", line), values):
            tokens = TOKEN.findall(expanded)
            for m in JOIN.finditer(expanded):
                args = [a.strip() for a in m.group(1).split(",")]
                tokens.append("/".join(a[1:-1] if len(a) > 1 and a[0] == a[-1] and a[0] in "'\"" else WILD for a in args))
            found.update(tokens)
    return tuple(sorted(found))


@functools.lru_cache(maxsize=None)
def named_paths(rev, p):
    return frozenset(names(texts(rev)[p], posixpath.dirname(p), ni.tree_entries(rev)))


@functools.lru_cache(maxsize=None)
def files_under(rev):
    under = collections.defaultdict(list)
    for p, (kind, _) in ni.tree_entries(rev).items():
        d = posixpath.dirname(p) if kind != "tree" else None
        while d:
            under[d].append(p)
            d = posixpath.dirname(d)
    return under


def crate_of(rev, p):
    return next((d for d in sorted(ni.crate_meta(rev)[2], key=len, reverse=True) if p.startswith(d + "/")), None)


def crossings(artifact, rev, wanted, source, here, targets):
    """The targets that source names and the artifact's key leaves out, but crate dirs (which name
    crates), source itself and the dirs it is in, and NOT_READ's."""
    entries, under, problems = ni.tree_entries(rev), files_under(rev), []
    crate_dirs, crates = ni.crate_meta(rev)[2], ni.closure(rev, ni.ARTIFACTS[artifact].get("crates", []))
    for target in sorted(targets):
        if (target in crate_dirs or target == source or target.startswith(".github/workflows/")
                or here == target or here.startswith(target + "/") or not_read(source, target)):
            continue
        left_out = [f for f in (under[target] if entries[target][0] == "tree" else [target]) if not wanted(f)]
        if left_out:
            first, crate = left_out[0], crate_of(rev, left_out[0])
            whose = (f"{crate} is outside its crates' closure" if crate and crate not in crates else
                     f"OWNERS gives {first} to {', '.join(ni.owners(first)) or 'no artifact'}")
            what = target if left_out == [target] else f"{len(left_out)} of the files under {target}"
            problems.append(f"{artifact}: {source} names {what}, which its key leaves out ({whose}); if its "
                            f"job does not read it there, add it to NOT_READ with the reason")
    return problems


def scan(artifact, rev, wanted):
    """The paths that the text files in the artifact's key name and its key leaves out. The job
    pass reads the workflows, actions and Makefile for what the job runs of them."""
    tested = set(ni.ARTIFACTS[artifact].get("crates", [])) if artifact in CARGO_TESTS else set()
    problems = []
    for p in sorted(texts(rev)):
        if p.startswith(".github/") or p == "Makefile" or not wanted(p):
            continue
        crate = crate_of(rev, p)
        if crate and p[len(crate) + 1:].split("/")[0] in ("tests", "benches", "examples") and crate not in tested:
            continue
        problems += crossings(artifact, rev, wanted, p, posixpath.dirname(p), named_paths(rev, p))
    return problems


def main(argv):
    parser = argparse.ArgumentParser(prog="check-native-inputs.py",
                                     description="hold the native cache keys to what their jobs read")
    parser.add_argument("rev", nargs="?", default="HEAD")
    rev, problems = ni.resolve(parser.parse_args(argv).rev), []
    for artifact in ni.ARTIFACTS:
        try:
            ni.key_inputs(artifact, rev)
            problems.extend(check_reads(artifact, rev))
        except ni.Failure as error:
            problems.append(f"{artifact}: {error}")
    if problems:
        raise ni.Failure("\n  ".join(["keys:", *problems]))
    check_wiring(rev)
    print(f"{len(ni.ARTIFACTS)} keys resolve and hold what their jobs read, and every cached job is wired right")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main(sys.argv[1:]))
    except ni.Failure as error:
        sys.stderr.write(f"check-native-inputs: {error}\n")
        sys.exit(1)
