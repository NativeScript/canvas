"""Reads what a native job runs: its steps, the actions they use, the scripts and files they run,
and what `make -n` prints, with the refusals for what cannot be read. The first pass of
tools/tests/check-native-inputs.py, whose docstring says what it checks."""
import collections
import os
import posixpath
import re
import subprocess
import tempfile

from . import ni


# skia-cache names every platform's build scripts, only to hash them.
SKIA_CACHE = ".github/actions/skia-cache"
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
    """{key: [values]} of the values a job's strategy lists for each matrix key: `platform: [ios,
    tvos]`, a list of scalars, a list of maps (`module.name`), and include's maps (their own keys)."""
    lines = [l for l in strategy if l.strip()]
    start = next((i for i, l in enumerate(lines) if re.match(r"^\s+matrix:\s*$", l)), None)
    if start is None:
        return {}
    values, key, column = {}, None, None
    for line in lines[start + 1:]:
        if ni.indent(line) <= ni.indent(lines[start]):
            break
        column = column if column is not None else ni.indent(line)
        m = re.match(r"^\s*(-\s+)?([\w-]+):\s*(.*?)\s*$", line) or re.match(r"^\s*(-\s+)()(.+?)\s*$", line)
        if not m:
            continue
        dash, name, value = m.groups()
        if value.startswith(("[", "{")) and not value.endswith(("]", "}")):
            raise ni.Failure(f"write the matrix's `{name}:` on one line, so its values can be read: {line.strip()}")
        if ni.indent(line) == column and not dash:
            key = name
            if value.startswith("["):
                values[key] = [v.strip().strip("'\"") for v in value.strip("[]").split(",") if v.strip()]
            continue
        if key is None:
            continue
        if dash and not name:
            values.setdefault(key, []).append(value.strip("'\""))
            continue
        field_key = name if key == "include" else f"{key}.{name}"
        if value and not value.startswith(("[", "{")):
            values.setdefault(field_key, [])
            if value.strip("'\"") not in values[field_key]:
                values[field_key].append(value.strip("'\""))
    return values


MATRIX = re.compile(r"\$\{\{\s*matrix\.([\w.-]+)\s*\}\}")
MATRIX_WORD = re.compile(r"[^\s'\"]*\x03[\w.-]+\x03[^\s'\"]*")


def marked(script):
    """The script with each matrix value marked without the spaces `${{ }}` holds, so it stays in
    its word."""
    return MATRIX.sub(lambda m: f"\x03{m.group(1)}\x03", script)


def unmarked(script):
    return re.sub(r"\x03([\w.-]+)\x03", lambda m: "${{ matrix." + m.group(1) + " }}", script)


def apart(script):
    """(the script without its words that hold a matrix value, and those words, a line each)."""
    script = marked(script)
    return unmarked(MATRIX_WORD.sub(" ", script)), unmarked("\n".join(MATRIX_WORD.findall(script)))


def each_leg(script, values):
    """The script with each word that holds a matrix value the strategy lists written once per value,
    so `make ${{ matrix.platform }}` reads as one make of every leg's target."""
    script = marked(script)
    for key, options in values.items():
        mark = f"\x03{key}\x03"
        script = re.sub(r"[^\s'\"]*" + re.escape(mark) + r"[^\s'\"]*",
                        lambda m: " ".join(m.group(0).replace(mark, v) for v in options), script)
    return unmarked(script)


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
    sets (a function's own, `$(call f,x)`'s $(1) and $(foreach x,...)'s x, are set, as are a
    target's own, `t: X := y`, and `define X`), and its `?=` defaults where they make a path."""
    makefile = makefile.replace("$$", "")
    # Not a recipe's (tab-indented) shell assignment, which sets nothing for make.
    assignment = r"(?:export[ \t]+|override[ \t]+)?([A-Za-z_][\w.-]*)[ \t]*(?::{1,3}=|\+=|\?=|=)"
    set_here = set(re.findall(r"^(?!\t)[ ]*" + assignment, makefile, re.M))
    set_here |= set(re.findall(r"^(?!\t)[^:=#\n]+:(?!=)[ \t]*" + assignment, makefile, re.M))
    set_here |= set(re.findall(r"^define\s+([A-Za-z_][\w.-]*)", makefile, re.M))
    set_here |= {v.split("=", 1)[0] for v in variables} | set(re.findall(r"\$\(foreach\s+([\w.-]+)\s*,", makefile))
    defaults = set(re.findall(r"^\s*([A-Za-z_][\w.-]*)\s*\?=", makefile, re.M))
    found = []
    # $(X), ${X}, $(X:a=b) and $X.
    for m in re.finditer(r"\$[({]([A-Za-z_][\w.-]*)(?=[:)}])|\$([A-Za-z_])", makefile):
        name = m.group(1) or m.group(2)
        if name not in set_here and name not in ("MAKE", "MAKEFLAGS", "MAKECMDGOALS", "SHELL"):
            found.append(name)
    for name in defaults:
        # Next to a path's characters: `dir/$(X)`, `$(X)/lib.a`, `$(X).mk`, `$(X)-out`.
        if re.search(rf"[\w./-]\$[({{]{re.escape(name)}[)}}]|\$[({{]{re.escape(name)}[)}}][\w./-]", makefile):
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
    actions' steps as [(manifest, step)], and the steps' and job keys' text whose paths count, as
    [Read])."""
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

    def take(step, step_where, scan, values=None, job=None):
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
            texts_read.append(Read(step_where, job, text, wd, values or {}))
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
        values = matrix_values(matrix)
        texts_read.append(Read(f"{where} job keys", (path, job_id), "\n".join(l for l in outside if l not in matrix), "", values))
        for p in mentions("\n".join(outside), [], tree):
            reach(p, f"{where} job keys")
        for step in steps:
            take(step, where, True, values, (path, job_id))
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


# Text the job pass read: where it is, its job (path, id) if it is a job's, its working-directory,
# and the values its job's matrix lists.
Read = collections.namedtuple("Read", "where job text wd values")


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
