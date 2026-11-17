#!/usr/bin/env python3
"""Holds the claims tools/scripts/native-inputs.py keys the native builds by (OWNERS, ARTIFACTS and
each artifact's crate closure) to what the jobs are seen to read, and checks that every cached job
is wired to its cache. Static Checks runs it. Its parts are in native_check: jobs.py reads the jobs,
paths.py scans the keys, wiring.py checks the wiring and the values that vary between runs.

  check-native-inputs.py [REV]    REV defaults to HEAD

It is a lint, not a proof. A key leaves out only what OWNERS places elsewhere, the crates outside
its closure, the other jobs' blocks and the actions its jobs do not use, so a job can read what its
key misses only across one of those lines, and two passes look for reads across them; a read
written in a form neither pass follows goes unseen. So OWNERS claims that leave files out are kept
narrow, and a file no rule places is in every key.

Reading the job: its steps and job-level keys, its actions' steps (with $GITHUB_ACTION_PATH as the
action's dir), the files they run and the files those mention, what `make -n` prints for the targets
it runs, and the scripts in the package dirs OWNERS gives the artifact, followed in turn. In a job
whose legs build different artifacts, matrix values are this artifact's legs' (the item of a list of
maps that names it in the native-cache `artifact:`). It fails when a file read that way is outside
the key, a crate named that way (by package name or dir) is outside the closure, or the job cannot
be read: a file run that is not tracked, a file a step runs after `cd`, a path above the repo, a
script argument (`bash build`) that is no file, `python -m`, a package runner, cargo building crates
it does not name (these four in followed shell scripts too: any cargo subcommand but those that
build nothing, a plugin's or behind a launcher too, needs -p for each crate, without a glob, and no
--workspace or --all, wherever it runs), an unclosed quote or heredoc, a value continued on plain
lines, `make` in any form but `make [-jN] [VAR=x] <targets>` from the repo root, a Makefile with
conditionals, includes, $(shell) or paths of the dir it runs in, a Makefile that reads a variable it
does not set (or a `?=` default in a path), which the environment would choose, a followed script
that runs make (but the third-party builds in OWN_MAKE) or sources a computed path inside the repo,
a matrix list over several lines, a local action without action.yml, a workflow key after `jobs:`, a
YAML anchor or alias, or a workflow- or job-wide working-directory.

Scanning the key: every text file in the key but prose (docs, notices, git's own files), the
workflows, action manifests and Makefile, and the step and job-key text the first pass reads (and a
followed script from the dir a step runs it in), are searched for paths that lead out of the key. A
path counts relative to the repo or to the file's dir, and when it goes up (`..`), from any dir
above the file; from every dir the text has `cd` to, with each of its values; after a variable the
file sets (`set(DIR ...)`, `DIR=...`, the default in `${DIR:-...}`), with each of its values; after
any other variable or a dir named some other way (`${CMAKE_CURRENT_SOURCE_DIR}`,
`env!("CARGO_MANIFEST_DIR")` with `"/../x"`, `format!("{}/x", root)`, `root + '/x'`), from any of
those dirs, with or without the `..` after it; after a compiler flag (`-I`, `-L`, `-F`); with a glob
or another variable (a matrix value too) standing for one path segment, or for the dir it is in when
that matches nothing; given as the segments of a join() or resolve() call; in the values of a CMake
set() list or a shell variable that is exported or given to a command (`DIR=x cmd`), and in a
heredoc's body; and naming a dir, for every file under it, or a crate dir outside the closure (but
in a Cargo.toml, which cargo metadata reads). In shell text, a shell word's quoted and bare parts
are one word (`cd "$ROOT"/x`), the script's own dir (`$(dirname "$0")`) is its dir, a bare word
given to a reading command (`ls apps`) is a path, and what a line only writes (the arguments of
mkdir, rm and touch, the destination of cp and the like, a redirection's target) or prints (echo
without a redirection) is not. Comments, by each kind of file's syntax, are not paths. Crates'
tests, benches and examples count only for the jobs in CARGO_TESTS, which run `cargo test`. A text
file in the key over 2 MB fails, as it cannot be scanned. This pass holds the claims about build
files, such as the C++ the Android CMake builds compile from the iOS package dir.

A path a file names but its job does not read there is listed in NOT_READ with the reason, for both
passes.

Both fail on values that differ between runs that share a key: a ${{ }} context other than matrix,
steps, env, strategy, runner and github.workspace, in any case and in `.` or `[ ]` form, in the
workflow's settings, the job's keys, its build steps and its actions' steps (where `inputs.` is the
job's own `with:`), the run's GITHUB_ variables (GITHUB_SHA, GITHUB_REF, ...) and git commands that
read the commit, in the text the first pass reads.

And the wiring: each job that uploads an artifact in build-native.yml is one of an artifact's, and
each artifact's first job restores it: a native-cache step with `id: native` right after the
checkout, for its own artifact and the upload's path, with `variant: ${{ toJSON(matrix) }}` in a
matrix job, every later step but the upload skipped on a hit (its `if:` is `steps.native.outputs.hit
!= 'true'`, or that && more, with no ||), and one upload, under its artifact's name. An artifact
built by several jobs (the xcframeworks) has a first job that restores it with `restore-only:
'true'`, only uploads it on a hit, and has the step's `hit` and `key` as outputs. Its other jobs
need the first and have `if: needs.<first>.outputs.hit != 'true'` (or that && more, with no ||), and
the last of them needs every other and restores it as above, with `key: ${{
needs.<first>.outputs.key }}`, and saves it. In a matrix job, the `artifact:` of each leg is one of
the job's entries. No job of an artifact, nor a step of its actions, has a status function in an
`if:` or continue-on-error, which run on after a failure and save what is left. No local action's
descriptions hold `${{ }}`, which stop the runner loading it. For the reader, a make target made of
a matrix value counts once per value the strategy lists.

Needs git, Python 3.8+, cargo and make.
"""
import argparse
import os
import re
import sys

sys.dont_write_bytecode = True  # no __pycache__ beside the parts or the key module
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))  # the parts, however this is loaded

from native_check import ni  # noqa: E402
from native_check.jobs import discover, each_leg, field, named_crates  # noqa: E402
from native_check.paths import CARGO_TESTS, MATCHED, code, crossings, names, not_read, scan  # noqa: E402
from native_check.wiring import READS_COMMIT, RUN_VARIABLE, STATUS, check_wiring, expressions, moves_cache, varying  # noqa: E402


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
    if READS_COMMIT.search(corpus):
        problems.append(f"{artifact}: its job reads the commit ({READS_COMMIT.search(corpus).group(0)}), which can "
                        f"change between runs that share a key")
    for manifest, step in action_steps:
        # As in the job: a step that runs after a failure lets the job pass and save what is left.
        if STATUS.search(field(step, "if")):
            problems.append(f"{artifact}: {manifest}: `if: {field(step, 'if').strip()}` holds a status function, which "
                            f"runs it after what it follows failed")
        if field(step, "continue-on-error"):
            problems.append(f"{artifact}: {manifest}: continue-on-error runs it on after a failure, and the cache saves what is left")
        if not moves_cache(step):
            for context in varying(expressions([], [step]), in_action=True):
                problems.append(f"{artifact}: {manifest} uses {context}, which can change between runs that share a key")
    # Matrix values are this artifact's legs' (see discover), so every path counts against its key.
    entries, cache = ni.tree_entries(rev), MATCHED.setdefault(rev, {})
    for read in texts_read:
        found = names(code(each_leg(read.text, read.values), "step.yml"), read.wd, entries, cache, shell=True)
        problems += crossings(artifact, rev, wanted, read.where, read.wd, found - set(reads))
    return problems + scan(artifact, rev, wanted)


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
