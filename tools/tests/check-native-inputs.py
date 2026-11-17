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

Scanning the key: every text file in the key but prose (docs, notices, git's own files), the
workflows, actions and Makefile, and the step and job-key text the first pass reads, are searched
for paths that lead out of the key. A path counts relative to the repo or to the file's dir, and
when it goes up (`..`), from any dir above the file; after a variable the file sets (`set(DIR
...)`, `DIR=...`), with each of its values; after any other variable (`${CMAKE_CURRENT_SOURCE_DIR}`,
`env!("CARGO_MANIFEST_DIR")` with `"/../x"`), from any of those dirs, with or without the `..`
after it; with a glob or another variable (a matrix value too) standing for one path segment, or
for the dir it is in when that matches nothing; given as the segments of a join() or resolve()
call; and naming a dir, for every file under it, or a crate dir outside the closure (but in a
Cargo.toml, which cargo metadata reads). Comments, by each kind of file's syntax, and the values
variables are set to are not paths. In a job whose legs build different artifacts, a path made with
a matrix value counts when no artifact of the job reads it. Crates' tests, benches and examples
count only for the jobs in CARGO_TESTS, which run `cargo test`. A file in the key over 2 MB must be
of a binary type (BINARY). This pass holds the claims about build files, such as the C++ the
Android CMake builds compile from the iOS package dir.

A path a file names but its job does not read there (one it writes, say) is listed in NOT_READ with
the reason, for both passes.

Both fail on values that differ between runs that share a key: a ${{ }} context other than matrix,
steps, env, strategy, runner and github.workspace, in any case and in `.` or `[ ]` form, in the
workflow's settings, the job's keys, its build steps and its actions' steps (where `inputs.` is the
job's own `with:`), and the run's GITHUB_ variables (GITHUB_SHA, GITHUB_REF, ...) in the text the
first pass reads.

And the wiring: each job that uploads an artifact in build-native.yml is one of an artifact's, and
each artifact's first job restores it: a native-cache step with `id: native` right after the
checkout, for its own artifact and the upload's path, with `variant: ${{ toJSON(matrix) }}` in a
matrix job, and every later step but the upload skipped on a hit (its `if:` is
`steps.native.outputs.hit != 'true'`, or that && more, with no ||). An artifact built by several
jobs (the xcframeworks) has a first job that restores it with `restore-only: 'true'`, only uploads
it on a hit, and has the step's `hit` and `key` as outputs. Its other jobs need the first and have
`if: needs.<first>.outputs.hit != 'true'` (or that && more, with no ||), and the last of them
restores it as above, with `key: ${{ needs.<first>.outputs.key }}`, and saves it. For the reader,
a make target made of a matrix value counts once per value the strategy lists.

Needs git, Python 3.8+, cargo and make.
"""
import argparse
import functools
import os
import re
import sys

sys.dont_write_bytecode = True  # no __pycache__ beside the parts or the key module
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))  # the parts, however this is loaded

from native_check import ni  # noqa: E402
from native_check.jobs import apart, discover, each_leg, named_crates  # noqa: E402
from native_check.paths import CARGO_TESTS, MATCHED, code, crossings, names, not_read, scan  # noqa: E402
from native_check.wiring import RUN_VARIABLE, check_wiring, expressions, moves_cache, varying  # noqa: E402



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
    # A path made with a matrix value is one leg's: in a job whose legs build different artifacts
    # (the variant keys each leg), it counts when no artifact of the job reads it. Any other path
    # counts against this artifact's key.
    entries, cache = ni.tree_entries(rev), MATCHED.setdefault(rev, {})
    for read in texts_read:
        legs = [a for a in ni.ARTIFACTS if read.job in ni.jobs_of(a)] if read.job else [artifact]
        filters = [key_filter(a, rev) for a in legs]
        plain, made = apart(read.text)
        for text, wants in ((plain, wanted), (each_leg(made, read.values), lambda p: any(f(p) for f in filters))):
            found = names(code(text, "step.yml"), read.wd, entries, cache) - set(reads)
            problems += crossings(artifact, rev, wants, read.where, read.wd, found)
    return problems + scan(artifact, rev, wanted)


@functools.lru_cache(maxsize=None)
def key_filter(artifact, rev):
    return ni.key_filter(artifact, rev)


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
