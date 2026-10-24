#!/usr/bin/env python3
"""Independent, stdlib-only arithmetic audit of archived synthetic result tables.

Reads committed historical results, not Hiyal's Rust analysis or jq decision rules.
Neither regenerates nor verifies raw journals. Safe for routine CI.
"""
import csv
import importlib.util
import json
import math
import sys
from pathlib import Path

# The downstream evidence pipeline refuses dirty source trees. Loading a local
# reference module must never write an untracked __pycache__ into the checkout.
sys.dont_write_bytecode = True

ROOT = Path(__file__).resolve().parents[1]
ONE = ROOT / "experiments/001-emergence-of-inequality/results/replicated-74000"
TWO = ROOT / "experiments/002-reputation-lock-in/results/v2-36-paired-20261021"


def require(condition, message):
    if not condition:
        raise AssertionError(message)


def close(actual, expected, context, tolerance=1e-10):
    require(
        math.isfinite(actual) and math.isfinite(expected)
        and abs(actual - expected) <= tolerance,
        f"{context}: computed {actual!r}, recorded {expected!r}",
    )


def sample_sd(values):
    mean = math.fsum(values) / len(values)
    return math.sqrt(math.fsum((v - mean) ** 2 for v in values) / (len(values) - 1))


def audit_001():
    with (ONE / "hiyal-trajectories.csv").open(newline="", encoding="utf-8") as stream:
        rows = list(csv.DictReader(stream))
    with (ONE / "hiyal-summary.csv").open(newline="", encoding="utf-8") as stream:
        summaries = list(csv.DictReader(stream))
    require(len(rows) == 720, "001: expected all 720 committed trajectory rows")
    require(len(summaries) == 18, "001: expected 18 configured comparisons")
    seen = set()
    endpoints = {}
    for r in rows:
        identity = tuple(r[k] for k in ("actors", "initial", "offset", "horizon", "seed", "arm", "step"))
        require(identity not in seen, f"001: repeated trajectory key {identity!r}")
        seen.add(identity)
        require(r["arm"] in ("control", "treatment"), "001: unknown arm")
        require(r["offset"] == "" if r["arm"] == "control" else r["offset"] != "",
                "001: treatment offset/control encoding inconsistent")
        require(74000 <= int(r["seed"]) <= 74009, "001: seed outside planned range")
        for field in ("gini", "top_share", "rank_persistence", "mean_rank_movement"):
            require(math.isfinite(float(r[field])), f"001: invalid {field}")
        if int(r["step"]) == int(r["horizon"]):
            key = tuple(r[k] for k in ("actors", "initial", "horizon", "seed", "arm", "offset"))
            require(key not in endpoints, f"001: duplicate endpoint {key!r}")
            endpoints[key] = float(r["gini"])

    checked = set()
    negatives = []
    largest_difference = 0.0
    for s in summaries:
        cell = tuple(s[k] for k in ("actors", "initial", "offset", "horizon"))
        require(cell not in checked, f"001: duplicated summary cell {cell!r}")
        checked.add(cell)
        control, treated, differences = [], [], []
        for seed in range(74000, 74010):
            base = (s["actors"], s["initial"], s["horizon"], str(seed))
            c = endpoints[base + ("control", "")]
            t = endpoints[base + ("treatment", s["offset"])]
            control.append(c)
            treated.append(t)
            differences.append(t - c)
        require(int(s["pairs"]) == 10, f"001: cell {cell!r} incorrectly counts pairs")
        for field, observed in (
            ("control_mean_gini", math.fsum(control) / 10),
            ("treatment_mean_gini", math.fsum(treated) / 10),
            ("paired_delta", math.fsum(differences) / 10),
        ):
            expected = float(s[field])
            close(observed, expected, f"001 {cell!r} {field}")
            largest_difference = max(largest_difference, abs(observed - expected))
        if math.fsum(differences) < 0:
            negatives.append(cell)

    require(len(endpoints) == 240, "001: expected 60 distinct control and 180 treatment endpoints")
    # Exact *count-variance* result of the symmetric Polya urn, not a Gini result.
    variance_ratio = (500 + 20 * (10 + 1)) / (1 + 20 * (10 + 1))
    close(variance_ratio, 720 / 221, "001 exact count-variance ratio", 1e-14)
    return {
        "comparison_cells": len(checked),
        "trajectory_rows": len(rows),
        "endpoint_histories": len(endpoints),
        "max_difference_from_published_decimal_csv": largest_difference,
        "negative_paired_mean_cells": [list(x) for x in negatives],
        "exact_urn_count_variance_ratio": variance_ratio,
    }


def audit_002_v2():
    paired = json.loads((TWO / "paired-summary-with-uncertainty.json").read_text(encoding="utf-8"))
    h1 = json.loads((TWO / "h1-decomposition.json").read_text(encoding="utf-8"))
    rows = paired["per_pair"]
    seeds = [int(x["seed"]) for x in rows]
    require(len(rows) == 36 and len(set(seeds)) == 36, "002 V2: expected 36 unique parent pairs")
    require(seeds == paired["seeds"] and paired["parent_pairs"] == 36, "002 V2: planned seeds mismatch")
    expected = list(range(98000, 98036))
    require(seeds == expected, "002 V2: unexpected seed cohort")
    prior, null, diffs = [], [], []
    for r in rows:
        p = float(r["prior_information_late_gap"])
        n = float(r["no_prior_information_late_gap"])
        effect = float(r["matched_seed_prior_effect"])
        close(p - n, effect, f"002 V2 seed {r['seed']} paired prior effect")
        prior.append(p)
        null.append(n)
        diffs.append(p - n)
    for label, values in (("prior_gap", prior), ("no_prior_gap", null),
                          ("matched_difference", diffs)):
        close(math.fsum(values) / 36, float(paired[label]["mean"]), f"002 V2 {label} mean")
        close(sample_sd(values), float(paired[label]["sd"]), f"002 V2 {label} sample SD")
    by_delta = {float(g["delta"]): g for g in h1["groups"]}
    require(set(by_delta) == {0.0, 0.2}, "002 V2: missing prior/null groups")
    for delta, values in ((0.0, null), (0.2, prior)):
        g = by_delta[delta]
        require(g["parents"] == 36 and g["late_window"] == [501, 600],
                "002 V2: group parent count or late window inconsistent")
        close(math.fsum(values) / 36, g["gap"]["mean"], f"002 V2 delta={delta} gap")
        close(g["new_link_exposure"]["mean"] + g["selection"]["mean"],
              g["new_link_plus_selection"]["mean"], f"002 V2 delta={delta} decomposition")
    return {
        "independent_parent_pairs": 36,
        "paired_prior_effect_mean": math.fsum(diffs) / 36,
        "negative_prior_effect_pairs": sum(v < 0 for v in diffs),
    }


def audit_002_v3():
    path = ROOT / "docs/review/v3_independent_paired_audit.py"
    spec = importlib.util.spec_from_file_location("v3_paired_reference", path)
    require(spec is not None and spec.loader is not None, "002 V3: cannot load independent audit")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    result = module.audit(ROOT)  # independent of Hiyal and original jq decision rules
    cells = result["cells"]
    require(len(cells) == 9, "002 V3: missing factorial cells")
    require(sum(c["independent_parents"] for c in cells) == 432,
            "002 V3: incorrect independent parent total")
    # Regression references from the 2026-10-22 post-hoc paired-summary audit.
    references = {
        1: -0.032624813236699905, 2: -0.07467631408418554,
        4: -0.018626129232844922, 5: 0.010723085500099915,
        7: -0.007744023345922931, 8: 0.004511849445892842,
    }
    for cell in cells:
        index = cell["cell"]
        require(index in range(9), "002 V3: unexpected cell number")
        if index in (0, 3, 6):
            close(cell["paired_gap"]["mean"], 0.0, f"002 V3 cell {index} null effect", 1e-12)
            close(cell["paired_nps"]["mean"], 0.0, f"002 V3 cell {index} null NPS", 1e-12)
        else:
            close(cell["paired_nps"]["mean"], references[index],
                  f"002 V3 cell {index} paired NPS")
    close(cells[2]["paired_sign_test"]["holm_adjusted_p"], 0.04963681336950999,
          "002 V3 exploratory Holm sign test")
    return {
        "cells": len(cells), "independent_parents": 432,
        "paired_nps_by_cell": [round(c["paired_nps"]["mean"], 9) for c in cells],
        "scope": "post-hoc parent-summary arithmetic, NOT Hiyal journal replay",
    }


def main():
    report = {
        "status": "PASS_COMMITTED_TABLE_ARITHMETIC_ONLY",
        "source": "published historical exploratory synthetic outputs",
        "experiment_001": audit_001(),
        "experiment_002_v2": audit_002_v2(),
        "experiment_002_v3": audit_002_v3(),
        "limitations": [
            "Does not execute Rust/Hiyal or replay historical raw journals",
            "Does not independently validate the behavioral model specification",
            "Does not establish confirmatory, empirical or human-population findings",
        ],
    }
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
