"""Merge feature_test.py JSON results from several environments into one report.

Usage: python summarize.py results/*.json [--md report.md]
"""

from __future__ import annotations

import argparse
import collections
import json

ORDER = ["PASS", "WRONG", "MISSING", "ERROR", "HANG", "INVALID", "N/A"]


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("files", nargs="+")
    ap.add_argument("--md")
    a = ap.parse_args()

    runs = []
    for f in sorted(a.files):
        d = json.load(open(f, encoding="utf-8"))
        e = d["env"]
        label = f"{e['system']}/{e['machine']}/{'musl' if 'musl' in e['libc'] or 'alpine' in e.get('image', '') else 'glibc' if e['system'] == 'Linux' else ''}/py{e['python'].rsplit('.', 1)[0]}/{e['mode']}"
        runs.append((label.replace("//", "/"), d))

    feats: dict[str, dict] = {}
    for label, d in runs:
        for r in d["results"]:
            f = feats.setdefault(r["id"], dict(group=r["group"], name=r["name"], by_env={}))
            f["by_env"][label] = r

    lines = [f"# fast-rich feature report ({len(runs)} runs: " + ", ".join(l for l, _ in runs) + ")", ""]
    totals = collections.Counter()
    inconsistent = []
    group = None
    for fid, f in feats.items():
        statuses = collections.Counter(r["status"] for r in f["by_env"].values() if r["status"] != "N/A")
        overall = min(statuses, key=ORDER.index) if len(statuses) == 1 else "MIXED"
        if not statuses:
            overall = "N/A"
        totals[overall] += 1
        if f["group"] != group:
            group = f["group"]
            lines += ["", f"## {group}", "", "| Feature | Result | Detail |", "|---|---|---|"]
        details = sorted({r["detail"] for r in f["by_env"].values() if r["detail"] and r["status"] != "N/A"})
        detail = " / ".join(details)[:160]
        if overall == "MIXED":
            per = collections.defaultdict(list)
            for env, r in f["by_env"].items():
                per[r["status"]].append(env)
            detail = "; ".join(f"{s}: {', '.join(v)}" for s, v in per.items())
            inconsistent.append(fid)
        lines.append(f"| `{fid}` {f['name']} | {overall} | {detail.replace('|', '¦')} |")
    lines[1:1] = ["", "Totals: " + ", ".join(f"{k}={totals[k]}" for k in ORDER + ['MIXED'] if totals[k]),
                  "Inconsistent across environments: " + (", ".join(inconsistent) or "none"), ""]
    text = "\n".join(lines)
    print(text)
    if a.md:
        open(a.md, "w", encoding="utf-8").write(text)


if __name__ == "__main__":
    main()
