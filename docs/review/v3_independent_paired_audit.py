#!/usr/bin/env python3
"""Post hoc independent numerical audit of committed V3 parent summaries.
Stdlib only. No Hiyal runner or existing jq decision-rule code is used.
Usage: python3 docs/review/v3_independent_paired_audit.py [repo-root]
"""
import json, math, sys
from pathlib import Path

N = 48
T975_47 = 2.011741
MECHANISMS = ("random-turnover", "belief-deletion", "full-feedback")
PERIODS = (2, 10, 20)

def summarize(values):
    if len(values) != N or any(not math.isfinite(v) for v in values):
        raise ValueError("expected 48 finite independent-parent values")
    mu = math.fsum(values) / N
    sd = math.sqrt(math.fsum((x-mu)**2 for x in values)/(N-1))
    half = T975_47 * sd / math.sqrt(N)
    return {"mean": mu, "sd": sd, "t_ci_95": [mu-half, mu+half]}

def sign_test(values):
    plus = sum(x>0 for x in values)
    minus = sum(x<0 for x in values)
    n = plus+minus
    p = min(1., 2*sum(math.comb(n,i) for i in range(min(plus,minus)+1))/2**n) if n else 1.
    return {"plus":plus,"minus":minus,"zeros":len(values)-n,"p_two_sided":p}

def audit(root):
    output=[]
    for cell in range(9):
        mechanism=MECHANISMS[cell%3]
        period=PERIODS[cell//3]
        stage="v3-stage-a-20261022" if cell%3==2 else "v3-stage-b-20261022"
        folder=root/"experiments"/"002-reputation-lock-in"/"results"/stage/f"cell-{cell}"/"cell-summaries"
        wanted={f"parent-{130000+cell*100+i}.json" for i in range(N)}
        found={f.name for f in folder.glob("*.json")}
        if wanted!=found:
            raise ValueError(f"cell {cell}: missing/extra/duplicate parent files: {wanted^found}")
        columns={key:[] for key in ("treated_nps","null_nps","paired_nps","paired_inherited","paired_gap")}
        for i in range(N):
            seed=130000+100*cell+i
            path=folder/f"parent-{seed}.json"
            j=json.loads(path.read_text(encoding="utf-8"))
            if (j["parent_seed"],j["rewire_every"],j["mechanism"]) != (seed,period,mechanism):
                raise ValueError(f"cell/seed/mechanism mismatch: {path}")
            t=j["information_only"]["delta_0_2"]["late"]
            n=j["information_only"]["delta_0"]["late"]
            if (t["from"],t["through"],n["from"],n["through"]) != (501,600,501,600):
                raise ValueError(f"late measurement windows differ: {path}")
            tn=t["new_link_exposure"]+t["conditional_selection"]
            nn=n["new_link_exposure"]+n["conditional_selection"]
            if any(abs(a-b)>1e-11 for a,b in [(tn,t["new_link_plus_selection"]),(nn,n["new_link_plus_selection"]),(tn-nn,j["late_false_prior_minus_null"]["new_link_plus_selection"])]):
                raise ValueError(f"accounting mismatch: {path}")
            for key, val in (("treated_nps",tn),("null_nps",nn),("paired_nps",tn-nn),("paired_inherited",t["inherited_exposure"]-n["inherited_exposure"]),("paired_gap",t["signed_gap"]-n["signed_gap"])):
                columns[key].append(float(val))
        if mechanism=="random-turnover" and any(abs(v)>1e-11 for key in ("paired_nps","paired_inherited","paired_gap") for v in columns[key]):
            raise ValueError(f"random-turnover treated/control forks not identical: {cell}")
        out={"cell":cell,"mechanism":mechanism,"rewire_every":period,"independent_parents":N}
        for key,values in columns.items():
            out[key]=summarize(values)
        out["paired_sign_test"]=sign_test(columns["paired_nps"])
        output.append(out)
    # Exact two-sided sign tests under zero median, Holm adjustment across nine cells.
    order=sorted(range(9),key=lambda i: output[i]["paired_sign_test"]["p_two_sided"])
    running=0.
    for rank,cell in enumerate(order):
        running=max(running,(9-rank)*output[cell]["paired_sign_test"]["p_two_sided"])
        output[cell]["paired_sign_test"]["holm_adjusted_p"]=min(1.,running)
    return {"status":"POST_HOC_SYNTHETIC_PARENT_SUMMARY_REANALYSIS_NOT_INDEPENDENT_HUMAN_REVIEW",
            "method":"48 independent parents per cell; paired treated-minus-no-false-prior NPS; unadjusted t CIs; exact sign test with nine-comparison Holm adjustment",
            "warning":"Does not replay raw Hiyal journals or prove human causality, novelty or equivalence; the components are accounting quantities, not identified natural mediator effects.",
            "cells":output}

if __name__=="__main__":
    root=Path(sys.argv[1]) if len(sys.argv)==2 else Path(__file__).resolve().parents[2]
    print(json.dumps(audit(root),indent=2,allow_nan=False))
