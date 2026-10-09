#!/usr/bin/env python3
"""The ids of `docs/`: an ADR's number, the ADR index, the rules of `design/` once there are any.

    python3 scripts/docs.py new-adr <slug> <decision>   # docs/adr/NNNN-<slug>.md, the next number
    python3 scripts/docs.py index                       # docs/adr/README.md's table, from the ADRs
    python3 scripts/docs.py check [--open-prs N]        # every id once, every header readable

`new-adr` takes the number after the highest on this branch, on `origin/master` and in the open
pull requests (through `gh`, when it is there). `index` is run by `docs.yml` after a merge, not by
hand: a pull request leaves the table alone, so that two of them never meet in it. `check
--open-prs N` also fails pull request N if an older open one adds an ADR of the same number.
"""

import argparse
import json
import re
import subprocess
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, List, Optional

ROOT = Path(__file__).resolve().parent.parent
ADR_DIR = ROOT / "docs" / "adr"
INDEX = ADR_DIR / "README.md"
# The files that define rules, by the prefix of their ids (docs/README.md, "Identifiers"): none
# since the restart from scratch; a `design/` file is added here with its prefix.
RULES: Dict[str, str] = {}

ADR_FILE = re.compile(r"^(\d{4})-([a-z0-9]+(?:-[a-z0-9]+)*)\.md$")
TITLE = re.compile(r"^# ADR-(\d{4}): (\S.*)$")
STATUS = re.compile(r"^Status: (proposed|accepted|superseded by ADR-(\d{4}))\b")
AMENDS = re.compile(r"^Amends ADR-(\d{4}): (\S.*)$")
RULE_ROW = re.compile(r"^\| *(?:~~)?([A-Z])(\d+)(?:~~)? *\|")
TABLE_HEAD = "| # | Decision | Status |"
TABLE_RULE = "|---|---|---|"
GENERATED = "<!-- written by `python3 scripts/docs.py index` after a merge: never by hand -->"

TEMPLATE = """# ADR-{number:04d}: {decision}

Status: proposed

## Context

## Decision

## Consequences
"""


@dataclass
class Adr:
    number: int
    file: str
    decision: str
    status: str  # as the index shows it: `accepted`, `proposed`, `superseded by D35`
    amends: List[tuple] = field(default_factory=list)  # (number, what)


class DocsError(Exception):
    pass


def read_adr(path: Path) -> Adr:
    name = ADR_FILE.match(path.name)
    if name is None:
        raise DocsError(f"{path.name}: not NNNN-<slug>.md")
    number = int(name.group(1))
    lines = path.read_text(encoding="utf-8").splitlines()
    title = TITLE.match(lines[0]) if lines else None
    if title is None:
        raise DocsError(f"{path.name}: the first line is not `# ADR-NNNN: <decision>`")
    if int(title.group(1)) != number:
        raise DocsError(f"{path.name}: its header says ADR-{title.group(1)}")
    # The header is what comes before the first section: its status, and what it amends.
    header = lines[1:next((i for i, l in enumerate(lines) if l.startswith("## ")), len(lines))]
    statuses = [m for m in map(STATUS.match, header) if m]
    if len(statuses) != 1:
        raise DocsError(f"{path.name}: no `Status: proposed | accepted | superseded by ADR-NNNN`")
    by = statuses[0].group(2)
    status = f"superseded by D{int(by)}" if by else statuses[0].group(1)
    amends = [(int(m.group(1)), m.group(2)) for m in map(AMENDS.match, header) if m]
    return Adr(number, path.name, title.group(2), status, amends)


def read_adrs(directory: Path = ADR_DIR) -> List[Adr]:
    adrs = [read_adr(p) for p in sorted(directory.glob("*.md")) if p.name != "README.md"]
    seen: Dict[int, str] = {}
    for adr in adrs:
        if adr.number in seen:
            raise DocsError(f"ADR-{adr.number:04d} twice: {seen[adr.number]} and {adr.file}")
        seen[adr.number] = adr.file
    for adr in adrs:
        targets = [n for n, _ in adr.amends]
        if adr.status.startswith("superseded by D"):
            targets.append(int(adr.status[len("superseded by D"):]))
        for n in targets:
            if n not in seen or n == adr.number:
                raise DocsError(f"{adr.file}: names ADR-{n:04d}, which is not another ADR")
    return adrs


def index_rows(adrs: List[Adr]) -> List[str]:
    notes: Dict[int, List[str]] = {}
    for adr in adrs:  # by number, so an ADR's notes are in the order of the ones amending it
        for n, what in adr.amends:
            # "stamps by D28", but "a re-export is its package's, not its file's, by D45"
            by = ", by" if "," in what else " by"
            notes.setdefault(n, []).append(f"{what}{by} D{adr.number}")
    rows = []
    for adr in adrs:
        status = "; ".join([adr.status] + notes.get(adr.number, []))
        rows.append(f"| [{adr.number:04d}]({adr.file}) | {adr.decision} | {status} |")
    return rows


def write_index(text: str, rows: List[str]) -> str:
    """`text` with the table after `TABLE_HEAD` replaced by `rows`, and marked generated."""
    lines = text.splitlines()
    try:
        head = lines.index(TABLE_HEAD)
    except ValueError:
        raise DocsError(f"{INDEX.name}: no `{TABLE_HEAD}`") from None
    end = head + 2
    while end < len(lines) and lines[end].startswith("|"):
        end += 1
    start = head - 2 if head >= 2 and lines[head - 2] == GENERATED else head
    return "\n".join(lines[:start] + [GENERATED, "", TABLE_HEAD, TABLE_RULE] + rows + lines[end:]) + "\n"


def check_rules(root: Path = ROOT, rules: Dict[str, str] = RULES) -> None:
    for prefix, name in rules.items():
        seen = set()
        for line in (root / name).read_text(encoding="utf-8").splitlines():
            row = RULE_ROW.match(line)
            if row is None or row.group(1) != prefix:
                continue
            if row.group(2) in seen:
                raise DocsError(f"{name}: {prefix}{row.group(2)} twice")
            seen.add(row.group(2))


def git(*args: str) -> Optional[str]:
    try:
        done = subprocess.run(["git", *args], cwd=ROOT, capture_output=True, text=True)
    except OSError:
        return None
    return done.stdout if done.returncode == 0 else None


def open_pr_adrs() -> Optional[Dict[int, List[tuple]]]:
    """The ADR files each open pull request adds, by its number; None without `gh`."""
    try:
        done = subprocess.run(
            ["gh", "pr", "list", "--state", "open", "--limit", "200", "--json", "number,files"],
            cwd=ROOT, capture_output=True, text=True)
    except OSError:
        return None
    if done.returncode != 0:
        return None
    adrs = {}
    for pr in json.loads(done.stdout):
        for f in pr.get("files") or []:
            path = Path(f["path"])
            name = ADR_FILE.match(path.name)
            if path.parent.as_posix() == "docs/adr" and name and f.get("additions", 0) > 0:
                adrs.setdefault(pr["number"], []).append((int(name.group(1)), path.name))
    return adrs


def numbers_in(names: List[str]) -> List[int]:
    return [int(m.group(1)) for m in map(ADR_FILE.match, names) if m]


def new_adr(slug: str, decision: str) -> Path:
    if not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", slug):
        raise DocsError(f"{slug}: a slug is lowercase words joined by `-`")
    taken = numbers_in([p.name for p in ADR_DIR.glob("*.md")])
    master = git("ls-tree", "--name-only", "origin/master", "docs/adr/")
    if master is None:
        print("note: no origin/master; numbers on this branch only", file=sys.stderr)
    else:
        taken += numbers_in([Path(p).name for p in master.split()])
    prs = open_pr_adrs()
    if prs is None:
        print("note: no `gh`; the open pull requests' numbers not looked at", file=sys.stderr)
    else:
        taken += [n for files in prs.values() for n, _ in files]
    number = max(taken, default=0) + 1
    path = ADR_DIR / f"{number:04d}-{slug}.md"
    path.write_text(TEMPLATE.format(number=number, decision=decision), encoding="utf-8")
    return path


def check_open_prs(pr: int, adrs: List[Adr], prs: Dict[int, List[tuple]]) -> None:
    """Fails if an open pull request older than `pr` adds an ADR of a number `adrs` has otherwise."""
    mine = {a.number: a.file for a in adrs}
    for other, files in sorted(prs.items()):
        if other >= pr:
            continue  # the younger pull request takes another number
        for n, name in files:
            if mine.get(n) not in (None, name):
                raise DocsError(f"ADR-{n:04d} is #{other}'s ({name}): take the next with "
                                f"`python3 scripts/docs.py new-adr`, and rename {mine[n]}")


def main(argv: List[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    commands = parser.add_subparsers(dest="command", required=True)
    new = commands.add_parser("new-adr", help="start docs/adr/NNNN-<slug>.md")
    new.add_argument("slug")
    new.add_argument("decision")
    commands.add_parser("index", help="write docs/adr/README.md's table")
    check = commands.add_parser("check", help="every id once, every ADR header readable")
    check.add_argument("--open-prs", type=int, metavar="N",
                       help="fail pull request N if an older open one took its ADR number")
    args = parser.parse_args(argv)
    try:
        if args.command == "new-adr":
            print(new_adr(args.slug, args.decision).relative_to(ROOT).as_posix())
        elif args.command == "index":
            rows = index_rows(read_adrs())
            INDEX.write_text(write_index(INDEX.read_text(encoding="utf-8"), rows), encoding="utf-8")
        else:
            adrs = read_adrs()
            check_rules()
            if args.open_prs is not None:
                prs = open_pr_adrs()
                if prs is None:
                    raise DocsError("--open-prs needs `gh`, logged in")
                check_open_prs(args.open_prs, adrs, prs)
    except DocsError as e:
        print(f"error: {e}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
