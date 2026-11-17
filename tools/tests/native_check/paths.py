"""Scans the text files in a key for paths that lead out of it. The second pass of
tools/tests/check-native-inputs.py, whose docstring says what it counts as a path."""
import collections
import functools
import os
import posixpath
import re

from . import ni


# The artifacts whose job runs `cargo test` on its crates, which builds their tests and examples.
CARGO_TESTS = {"unit-tests"}
# Paths that a file in a key names but that its job does not read there: (the file, as an OWNERS
# rule; the whole path, where `*` stands for one segment; why).
NOT_READ = [
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


LIMIT = 2_000_000
# How each kind of file writes comments; prose and git's own files are not read by builds.
C_LIKE = (".rs", ".c", ".cc", ".cpp", ".cxx", ".h", ".hh", ".hpp", ".inl", ".m", ".mm", ".swift", ".kt", ".kts",
          ".java", ".gradle", ".groovy", ".js", ".mjs", ".cjs", ".ts", ".tsx", ".go", ".cs", ".pbxproj", ".json5",
          ".xcconfig", ".modulemap", ".metal", ".glsl", ".hlsl", ".vert", ".frag", ".comp", ".wgsl", ".S", ".rc", ".idl")
HASH = (".sh", ".bash", ".zsh", ".py", ".cmake", ".toml", ".yml", ".yaml", ".rb", ".pl", ".properties", ".podspec",
        ".ps1", ".mk", ".cfg", ".ini", ".conf")
HASH_NAMES = ("CMakeLists.txt", "Makefile", "Podfile", "Gemfile", "Dockerfile")
XML = (".xml", ".plist", ".props", ".targets", ".storyboard", ".xib", ".xcscheme", ".xcworkspacedata", ".html")
PROSE = (".md", ".markdown", ".rst", ".adoc", ".txt")
PROSE_NAMES = re.compile(r"^(LICENSE|NOTICE|THIRD_PARTY_NOTICES|AUTHORS|CHANGELOG)([.-].*)?$|^\.git(ignore|attributes|keep)$")
# A string on one line (kept, so a comment marker in it is text), or a comment (group 1).
STRING = r"\"(?:\\.|[^\"\\\n])*\"|'(?:\\.|[^'\\\n])*'"
C_COMMENT = re.compile(STRING + r"|(/\*.*?\*/|//[^\n]*)", re.S)
# `#` after a space or at a line's start, but not Ruby's `#{`.
HASH_COMMENT = re.compile(STRING + r"|((?:^|(?<=\s))#(?!\{)[^\n]*)", re.M)
XML_COMMENT = re.compile(r"(<!--.*?-->)", re.S)
# Where a file sets a variable to a path: a shell assignment anywhere on a line, CMake's set(), and
# get_filename_component() with DIRECTORY, whose value is the dir above the path it is given.
SHELL_SET = re.compile(r"(?:^|[\s;&|()])(export\s+|(?:local|readonly|declare)\s+(?=\w))?([A-Za-z_]\w*)="
                       r"(\"[^\"]*\"|'[^']*'|[^\s;&|()]*)")
CMAKE_SET = re.compile(r"^\s*(set|get_filename_component)\(\s*([A-Za-z_]\w*)\s+\"?([^\s\")]+)\"?([^)]*)", re.I)
# A variable: ${{ }}, then the named forms (`${X}`, `$(X)`, `$X`), then any other (Ruby's #{x} too).
VARIABLE = re.compile(r"\$\{\{[^}]*\}\}|\$\{([A-Za-z_]\w*)\}|\$\(([A-Za-z_]\w*)\)|\$([A-Za-z_]\w*)"
                      r"|\$\{[^}]*\}|\$\([^)]*\)|#\{[^}]*\}|@[A-Za-z_]\w*@|%[A-Za-z_]\w*%")
# A shell's `cd`/`pushd` and where to.
CD = re.compile(r"(?:^|[\s;&|(])(?:cd|pushd)\s+(\"[^\"]*\"|'[^']*'|[^\s;&|)]+)")
TOKEN = re.compile(r"[^\s'\"`=;,(){}<>\[\]|&:!]*/[^\s'\"`=;,(){}<>\[\]|&:!]*")
# A path given as its segments: path.join(root, 'apps', 'demo'), os.path.join(ROOT, "x").
JOIN = re.compile(r"\b(?:join|resolve)\(([^()]*)\)")
WILD = "\x01"
WILDS = r"[*?" + WILD + "]"


@functools.lru_cache(maxsize=None)
def blobs(rev):
    """{path: (id, size)} of every tracked file at rev."""
    out = {}
    for record in ni.git("ls-tree", "-r", "-l", "-z", rev).split("\0"):
        if record:
            meta, path = record.split("\t", 1)
            _, kind, oid, size = meta.split()
            if kind == "blob":
                out[path] = (oid, int(size))
    return out


@functools.lru_cache(maxsize=None)
def texts(rev):
    """{path: text} of every tracked text file at rev up to LIMIT."""
    ids = {p: oid for p, (oid, size) in blobs(rev).items() if size <= LIMIT}
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
MATCHED = {}  # by rev, the paths each pattern with a glob or variable matches


SHELL_FILES = (".sh", ".bash", ".zsh")


def textual(p):
    """Whether p is of a kind of text file whose comments, or prose, this knows (or has no extension):
    one too large to scan must not be skipped as if it were binary."""
    name = posixpath.basename(p)
    ext = posixpath.splitext(name)[1].lower()
    return not ext or name in HASH_NAMES or ext in C_LIKE + HASH + XML + PROSE + (".json", ".svg", ".csv")


def code(text, p):
    """The file's text without its comments, by the kind of file p is (whole, for a kind whose
    comments are not known: a path read for one is the safe mistake); None for prose."""
    name = posixpath.basename(p)
    ext = posixpath.splitext(name)[1]
    if PROSE_NAMES.match(name) or (ext.lower() in PROSE and name not in HASH_NAMES):
        return None
    syntax = (C_COMMENT if ext in C_LIKE or ext.lower() in C_LIKE else XML_COMMENT if ext.lower() in XML
              else HASH_COMMENT if ext.lower() in HASH or name in HASH_NAMES else None)
    return text if syntax is None else syntax.sub(lambda m: " " if m.group(1) else m.group(0), text)


def expand(text, values):
    """The text with each variable the file sets replaced by each of its values in turn, and any
    other variable (a matrix value too) by WILD."""
    m = VARIABLE.search(text)
    if m is None:
        return [text]
    rests = expand(text[m.end():], values)
    name = m.group(1) or m.group(2) or m.group(3)
    options = (values.get(name) if name else None) or [WILD]
    if len(options) * len(rests) > 64:
        options = [WILD]
    return [text[:m.start()] + option + rest for option in options for rest in rests]


def resolve_token(token, here, entries, matched):
    """The tracked files and dirs a path in a file can name: from the repo or the file's dir, with
    WILD and globs standing for one segment. A path that goes up (`..`), or starts at a dir the file
    does not name (an unknown variable, or `"/../x"` after env!("CARGO_MANIFEST_DIR") or the like),
    may start from any dir above the file, and after an unknown dir, the `..` may lead anywhere. A
    computed name that matches nothing names its dir."""
    token = re.sub(r"^-(?:I|L|F|isystem|iquote|idirafter|include)(?=\S)", "", token)
    if token.startswith("/.."):
        token = WILD + token
    unknown = token.startswith(WILD + "/")
    token = re.sub(r"^(" + WILD + r"/)+", "", token).rstrip(".")
    if token.startswith("/") or "//" in token:
        return set()
    segments = [s for s in token.split("/") if s]
    if not any(re.search(r"[A-Za-z]", s) and not re.search(WILDS, s) for s in segments):
        return set()
    bases = ["", here]
    if unknown or segments[0] == "..":
        bases += [here.rsplit("/", n)[0] for n in range(1, here.count("/") + 1)]
    forms = [segments]
    if unknown and segments[0] == "..":
        forms.append(segments[next(i for i, s in enumerate(segments) if s != ".."):])
    found = set()
    for base, form in ((b, f) for b in bases for f in forms):
        try:
            path = ni.norm(posixpath.join(base, *form))
        except ni.Failure:
            continue
        if not re.search(WILDS, path):
            if path in entries:
                found.add(path)
            continue
        parts = path.split("/")
        if path not in matched:
            pattern = re.compile("^" + "/".join("[^/]*".join(map(re.escape, re.split(WILDS + "+", s))) for s in parts) + "$")
            matched[path] = {p for p in entries if pattern.match(p)}
        literal = "/".join(parts[:next(i for i, s in enumerate(parts) if re.search(WILDS, s))])
        found |= matched[path] or ({literal} if literal in entries else set())
    return found


def names(text, here, entries, matched=None, shell=False):
    """The tracked files and dirs the text's paths name (see resolve_token), with matched a cache
    for the tree entries come from; shell for shell text (see path_tokens)."""
    matched = {} if matched is None else matched
    return set().union(*(resolve_token(token, here, entries, matched) for token in path_tokens(text, shell)))


# Shell commands whose arguments are paths they write, not read (the last of cp-like ones'), and
# commands that read the paths given to them as bare words (`ls apps`).
WRITES_ALL = {"mkdir", "rm", "rmdir", "touch", "chmod"}
WRITES_LAST = {"cp", "mv", "install", "ln", "rsync"}
PRINTS = {"echo", "printf"}
READS = {"ls", "cat", "cp", "mv", "rsync", "tar", "zip", "find", "du", "stat", "head", "tail", "grep", "sed", "awk",
         "cmp", "diff", "sha256sum", "shasum", "md5sum", "xxd", "ln", "source", ".", "bash", "sh", "zsh", "python",
         "python3", "node", "file", "readlink", "realpath", "wc", "install"}


def shell_reads(line):
    """The words of a shell line that can be paths it reads: without what it only writes (the
    arguments of mkdir and the like, the destination of cp and the like, a redirection's target) or
    prints (echo without a redirection), with the bare words a reading command is given marked as
    paths (`./`)."""
    kept = []
    for segment in re.split(r"&&|\|\||[;|]", line):
        words = segment.split()
        while words and re.match(r"^[A-Za-z_]\w*=", words[0]):
            words = words[1:]
        if not words:
            continue
        command = posixpath.basename(words[0].strip("'\""))
        redirected = re.search(r"(?<![0-9&])>", segment)
        if command in PRINTS and not redirected:
            continue
        args = [w for w in words[1:]]
        drop = set()
        for i, w in enumerate(args):
            if w in (">", ">>") and i + 1 < len(args):
                drop.add(i + 1)
            elif re.match(r"^>>?\S", w):
                drop.add(i)
        operands = [i for i, w in enumerate(args) if not w.startswith("-") and i not in drop]
        if command in WRITES_ALL:
            drop.update(operands)
        elif command in WRITES_LAST and operands:
            target = next((i + 1 for i, w in enumerate(args) if w in ("-t", "--target-directory") and i + 1 < len(args)), None)
            drop.add(target if target is not None else operands[-1])
        kept.append(words[0])
        for i, w in enumerate(args):
            if i in drop:
                continue
            bare = command in READS and not w.startswith("-") and "/" not in w and re.search(r"[A-Za-z]", w)
            kept.append("./" + w.strip("'\"") if bare else w)
    return " ".join(kept)


@functools.lru_cache(maxsize=None)
def path_tokens(text, shell=False):
    """The paths the text names: its tokens with a `/` (and in shell text, the bare words a reading
    command is given, without what a line only writes or prints), with the variables it sets
    expanded, the values of the ones it exports and of CMake's set() lists, and each relative path
    also from every dir the text has `cd` to. The text is code: its comments already gone."""
    values, found, dirs = {}, set(), []

    def define(name, value):
        # A value that is not a plain path (a command's output, say) is unknown.
        values.setdefault(name, [])
        values[name] += [v if re.match(r"^[\w./@+" + WILD + "-]*$", v) else WILD
                         for v in expand(value.strip("'\""), values) if v not in values[name]]

    lines = re.sub(r"\\\n", " ", text).split("\n") if shell else text.split("\n")
    for line in lines:
        cmake = CMAKE_SET.match(line)
        if cmake:
            define(cmake.group(2), cmake.group(3) + ("/.." if cmake.group(1).lower() != "set" else ""))
            # A list's values are what is built from (sources); one value names a dir its uses extend.
            line = " ".join(cmake.group(3, 4)) if cmake.group(1).lower() == "set" and cmake.group(4).strip() else ""
        for m in SHELL_SET.finditer(line):
            define(m.group(2), m.group(3))
        exported = " ".join(m.group(3) for m in SHELL_SET.finditer(line) if m.group(1))
        for cd in CD.finditer(line):
            if cd.group(1).strip("'\"") == "-":
                continue
            to = expand(cd.group(1).strip("'\""), values)[0]
            to = to if re.match(r"^[\w./@+" + WILD + "-]*$", to) else WILD
            # As given and from the dir before it, since a popd or subshell may have left that one.
            dirs += [to] + ([posixpath.join(dirs[-1], to)] if dirs and not to.startswith(("/", WILD)) else [])
        rest = SHELL_SET.sub(" ", line) + " " + exported
        for expanded in expand(shell_reads(rest) if shell else rest, values):
            tokens = TOKEN.findall(expanded)
            for m in JOIN.finditer(expanded):
                args = [a.strip() for a in m.group(1).split(",")]
                tokens.append("/".join(a[1:-1] if len(a) > 1 and a[0] == a[-1] and a[0] in "'\"" else WILD for a in args))
            found.update(tokens)
            found.update(posixpath.join(d, t) for d in dirs for t in tokens if not t.startswith(("/", WILD)))
    return tuple(sorted(found))


@functools.lru_cache(maxsize=None)
def named_paths(rev, p):
    text = code(texts(rev)[p], p)
    return frozenset() if text is None else frozenset(
        names(text, posixpath.dirname(p), ni.tree_entries(rev), MATCHED.setdefault(rev, {}), p.endswith(SHELL_FILES)))


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
    """The targets that source names and the artifact's key leaves out, but source itself and the
    dirs it is in, and NOT_READ's. A crate dir counts when it is outside the closure, unless a
    Cargo.toml names it (cargo metadata reads those)."""
    entries, under, problems = ni.tree_entries(rev), files_under(rev), []
    crate_dirs, crates = ni.crate_meta(rev)[2], ni.closure(rev, ni.ARTIFACTS[artifact].get("crates", []))
    for target in sorted(targets):
        if (target == source or target.startswith(".github/workflows/") or here == target
                or here.startswith(target + "/") or not_read(source, target)):
            continue
        if target in crate_dirs:
            if target not in crates and posixpath.basename(source) != "Cargo.toml":
                problems.append(f"{artifact}: {source} names the crate {target}, which is outside its crates' "
                                f"closure; if its job does not read it there, add it to NOT_READ with the reason")
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
    """The paths that the text files in the artifact's key name and its key leaves out, and the files
    in it too large to read that are not binaries. The job pass reads the workflows, actions and
    Makefile for what the job runs of them."""
    tested = set(ni.ARTIFACTS[artifact].get("crates", [])) if artifact in CARGO_TESTS else set()
    problems = [f"{artifact}: {p} is in its key but too large to scan for paths" for p, (_, size) in sorted(blobs(rev).items())
                if size > LIMIT and wanted(p) and textual(p)]
    for p in sorted(texts(rev)):
        if p.startswith(".github/workflows/") or re.match(r"^\.github/actions/[^/]+/action\.ya?ml$", p) or p == "Makefile" or not wanted(p):
            continue
        crate = crate_of(rev, p)
        if crate and p[len(crate) + 1:].split("/")[0] in ("tests", "benches", "examples") and crate not in tested:
            continue
        problems += crossings(artifact, rev, wanted, p, posixpath.dirname(p), named_paths(rev, p))
    return problems
