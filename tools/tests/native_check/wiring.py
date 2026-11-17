"""Checks that every native job is wired to its cache and reads no value that changes between runs
of one key. Part of tools/tests/check-native-inputs.py, whose docstring says what it checks."""
import re

from . import ni
from .jobs import field, sections


GUARD = "steps.native.outputs.hit != 'true'"
# A context in an expression, with the field it reads (`.x` or `['x']`), if any.
CONTEXT = re.compile(r"(?<![\w.-])(github|inputs|vars|secrets|needs|job|runner|matrix|steps|env|strategy)\b"
                     r"(\s*\.\s*[\w-]+|\s*\[\s*'[^']*'\s*\])?", re.I)
# Contexts that are the same on every run of a job leg, so a cached job may use them.
STEADY_CONTEXTS = {"matrix", "steps", "env", "strategy", "runner"}
# The run's variables whose values differ between runs of one commit.
RUN_VARIABLE = re.compile(r"\bGITHUB_(SHA|REF|REF_NAME|REF_TYPE|REF_PROTECTED|HEAD_REF|BASE_REF|EVENT_NAME|"
                          r"EVENT_PATH|RUN_ID|RUN_NUMBER|RUN_ATTEMPT|ACTOR|ACTOR_ID|TRIGGERING_ACTOR|WORKFLOW|"
                          r"WORKFLOW_REF|WORKFLOW_SHA|RETENTION_DAYS)\b")
# Git commands whose output is the commit, which differs between commits that share a key.
READS_COMMIT = re.compile(r"\bgit\s+(?:describe|log|show|rev-parse\s+(?:--\S+\s+)*(?:HEAD|@)\b)")


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
    artifact's jobs cache it. Its first job restores it: a native-cache step with `id: native` right
    after the checkout, for that artifact and the upload's path, `variant: ${{ toJSON(matrix) }}` in
    a matrix job, and every later step but the upload skipped on a hit. An artifact built by several
    jobs is restored with `restore-only: 'true'` by a first job that only uploads it on a hit and has
    the step's `hit` and `key` as outputs; its other jobs need that job and are skipped on its hit,
    and the last of them restores it as above, under the first job's key, and saves it. No job of an
    artifact uses a value that changes from run to run where it could change what is built, nor a
    status function (success(), always(), cancelled(), failure()) in an `if:`, which would run a
    step or job after one it follows failed. Each build-native.yml job that restores an output
    uploads it once, under its artifact's name. And no local action's descriptions hold `${{ }}`:
    the runner evaluates them and cannot load the action."""
    problems, artifacts_of, parsed = [], {}, {}
    for manifest in sorted(p for p in ni.tracked(rev) if re.match(r"^\.github/actions/[^/]+/action\.ya?ml$", p)):
        column = None
        for line in ni.show(rev, manifest).split("\nruns:")[0].split("\n"):
            if column is not None and line.strip() and ni.indent(line) <= column:
                column = None
            if re.match(r"^\s*description:", line):
                column = ni.indent(line)
            if column is not None and "${{" in line:
                problems.append(f"{manifest}: an expression in an action's description stops the runner loading it: {line.strip()}")
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
            parsed[path, job_id] = (where, outside, steps, cache, uploads)
            # Steps that only move caches, like the upload, do not change what is built; nor do the
            # header's and the job's keys other than these.
            builds = [s for s in steps if s is not cache and s not in uploads and not moves_cache(s)]
            job_settings = settings + sections(outside, 4, {"runs-on", "env", "defaults", "strategy", "container", "services"})
            for context in varying(expressions(job_settings, builds)):
                problems.append(f"{where}: uses {context}, which can change between runs that share a key")
            for condition in [job_key(outside, "if"), *(field(s, "if") for s in steps)]:
                if STATUS.search(condition):
                    problems.append(f"{where}: `if: {condition.strip()}` holds a status function, which runs it after "
                                    f"what it follows failed")
            # It runs what follows a failure, and saves the cache with whatever the failed step left.
            if job_key(outside, "continue-on-error") or any(field(s, "continue-on-error") for s in steps):
                problems.append(f"{where}: continue-on-error runs it on after a failure, and the cache saves what is left")
            if cache is None:
                continue
            if field(cache, "id") != "native":
                problems.append(f"{where}: the native-cache step needs `id: native`")
            artifact = field(cache, "artifact")
            if "${{" not in artifact and artifact not in artifacts_of[path, job_id]:
                problems.append(f"{where}: native-cache artifact '{artifact}' is not one of this job's entries")
            if any(not re.match(r"^actions/checkout@", field(s, "uses")) for s in steps[:steps.index(cache)]):
                problems.append(f"{where}: only the checkout may come before the native-cache step")
            if any(re.match(r"^\s+matrix:", l) for l in outside) and re.sub(r"\s", "", field(cache, "variant")) != "${{toJSON(matrix)}}":
                problems.append(f"{where}: a matrix job passes `variant: ${{{{ toJSON(matrix) }}}}`")
    for artifact in ni.ARTIFACTS:
        jobs = ni.jobs_of(artifact)
        if any(job not in parsed for job in jobs):
            continue
        cached = [job for job in jobs if parsed[job][3] is not None]
        firsts = [job for job in cached if not field(parsed[job][3], "key")]
        if len(firsts) != 1:
            problems.append(f"{artifact}: one of its jobs restores it under the key it computes")
            continue
        first = firsts[0]
        where, outside, steps, cache, uploads = parsed[first]
        if len(jobs) == 1:
            if field(cache, "restore-only"):
                problems.append(f"{where}: an output built by one job is restored and saved there, without restore-only")
            problems += restored(*parsed[first], uploaded=first[0] == ni.BUILD_NATIVE)
            continue
        if field(cache, "restore-only") != "true":
            problems.append(f"{where}: {artifact} is built by several jobs, so its first job passes `restore-only: 'true'`")
        outputs = dict(m.groups() for m in (re.match(r"^\s+([\w-]+):\s*(.*?)\s*$", l)
                                            for l in sections(outside, 4, {"outputs"})[1:]) if m)
        for name in ("hit", "key"):
            if re.sub(r"\s", "", outputs.get(name, "")) != f"${{{{steps.native.outputs.{name}}}}}":
                problems.append(f"{where}: its outputs are `{name}: ${{{{ steps.native.outputs.{name} }}}}`")
        after = steps[steps.index(cache) + 1:]
        if len(after) != 1 or after[0] not in uploads or bare(field(after[0], "if")) != "steps.native.outputs.hit == 'true'":
            problems.append(f"{where}: after the restore it only uploads, once, with `if: steps.native.outputs.hit == 'true'`")
        problems += uploaded_as(where, cache, after[:1])
        lookup = first[1]
        gate = f"needs.{lookup}.outputs.hit != 'true'"
        for job in jobs:
            if job != first and (lookup not in needs(parsed[job][1]) or not guarded(bare(job_key(parsed[job][1], "if")), gate)):
                problems.append(f"{parsed[job][0]}: it builds {artifact}, so it needs {lookup} and its `if:` is "
                                f"{gate}, or that && more, with no ||")
        lasts = [job for job in cached if job != first]
        if len(lasts) != 1:
            problems.append(f"{artifact}: one of the jobs that build it restores it under {lookup}'s key, to save it")
            continue
        last_where, last_outside, _, last_cache, _ = parsed[lasts[0]]
        missing = sorted(j for _, j in jobs if (_, j) != lasts[0] and j not in needs(last_outside))
        if missing:
            problems.append(f"{last_where}: it saves {artifact}, so it needs every job that builds a part: {', '.join(missing)}")
        for name, want in (("key", f"${{{{needs.{lookup}.outputs.key}}}}"), ("artifact", field(cache, "artifact")),
                           ("path", field(cache, "path")), ("restore-only", "")):
            if re.sub(r"\s", "", field(last_cache, name)) != re.sub(r"\s", "", want):
                problems.append(f"{last_where}: its native-cache step's {name} must be {want or 'unset'}")
        problems += restored(*parsed[lasts[0]], uploaded=lasts[0][0] == ni.BUILD_NATIVE)
    if problems:
        raise ni.Failure("\n  ".join(["native-cache wiring:", *sorted(set(problems), key=problems.index)]))


STATUS = re.compile(r"\b(success|always|cancelled|failure)\s*\(", re.I)


def restored(where, outside, steps, cache, uploads, uploaded):
    """The problems of a job that restores an output and builds it on a miss: it skips every step
    but the upload on a hit, and when uploaded, uploads what it caches once, on a hit too."""
    problems = []
    for step in steps[steps.index(cache) + 1:]:
        if step in uploads:
            if bare(field(step, "if")):
                problems.append(f"{where}: the upload must run on a hit too")
        elif not guarded(bare(field(step, "if")), GUARD):
            problems.append(f"{where}: `{step[0].strip()}` runs on a hit (its `if:` must be "
                            f"{GUARD}, or that && more, with no ||)")
    if uploaded and len(uploads) != 1:
        problems.append(f"{where}: it uploads what it restores once")
    return problems + uploaded_as(where, cache, uploads)


def uploaded_as(where, cache, uploads):
    """The problems of uploads that are not the cached output under its artifact's name (with the
    leg's suffix in a matrix job)."""
    problems = []
    for upload in uploads:
        if field(upload, "path").rstrip("/") != field(cache, "path").rstrip("/"):
            problems.append(f"{where}: uploads {field(upload, 'path')} but caches {field(cache, 'path')}")
        if not re.sub(r"\s", "", field(upload, "name")).startswith(re.sub(r"\s", "", field(cache, "artifact"))):
            problems.append(f"{where}: uploads {field(upload, 'name')}, not {field(cache, 'artifact')}")
    return problems


def bare(condition):
    return re.sub(r"^\$\{\{\s*|\s*\}\}$", "", " ".join(condition.split()))


def guarded(condition, guard):
    """Whether an `if:` holds the guard, alone or && more, with no ||."""
    return re.match(rf"^{re.escape(guard)}\s*(&&|$)", condition) is not None and "||" not in condition


def job_key(outside, name):
    """A job-level key's value; '' when the job has none. A value continued on plain lines is an
    error, as in a step."""
    lines = sections(outside, 4, {name})
    return field(lines, name) if lines else ""


def needs(outside):
    return {n.strip() for n in job_key(outside, "needs").strip("[]").split(",") if n.strip()}
