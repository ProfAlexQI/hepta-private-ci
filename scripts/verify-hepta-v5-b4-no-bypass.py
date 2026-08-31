#!/usr/bin/env python3
"""Fail-closed, Cargo-assisted inventory for Hepta irreversible boundaries."""
import argparse,json,os,pathlib,re,subprocess,sys
from collections import Counter

SCHEMA="hepta.v5.b4.global-no-bypass-inventory.v1"
REPORT="hepta.v5.b4.global-no-bypass-report.v1"
ALLOWED={"PRODUCTION_VIA_VERIFIED_USE","QUALIFICATION_ONLY","FIXTURE_ONLY","DEAD_CODE"}
REQ={"file","symbol","physical_kind","classification","capability","issuer","consumer","final_payload_builder","claim_store_owner","witness_store_owner","reconciliation_owner"}
class Fail(RuntimeError): pass

def load(p):
    try:return json.loads(p.read_text(encoding="utf-8"))
    except Exception as e:raise Fail(f"cannot read {p}: {e}") from e

def under(p,prefixes):
    return any(p==x.rstrip("/") or p.startswith(x.rstrip("/")+"/") for x in prefixes)

def policy(p):
    d=load(p)
    if not isinstance(d,dict) or d.get("schema")!=SCHEMA or d.get("schemaVersion")!=1:raise Fail("inventory schema/version drift")
    s=d.get("sensitiveSymbols")
    if not isinstance(s,dict) or not s:raise Fail("sensitiveSymbols must be non-empty")
    for n,k in s.items():
        if not isinstance(n,str) or not re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*",n) or not isinstance(k,str) or not k:raise Fail(f"invalid sensitive symbol {n!r}")
    impl=d.get("boundaryImplementationFiles")
    if not isinstance(impl,list) or not impl or len(impl)!=len(set(impl)):raise Fail("invalid boundaryImplementationFiles")
    for n in ("qualificationPathPrefixes","fixturePathPrefixes","ignoredPathPrefixes"):
        if not isinstance(d.get(n),list) or any(not isinstance(x,str) or not x for x in d[n]):raise Fail(f"invalid {n}")
    entries=d.get("classifiedCallsites")
    if not isinstance(entries,list):raise Fail("classifiedCallsites must be a list")
    seen=set()
    for e in entries:
        if not isinstance(e,dict) or set(e)!=REQ:raise Fail("classified callsite field-set drift")
        key=(e["file"],e["symbol"])
        if key in seen:raise Fail(f"duplicate classified callsite {key}")
        seen.add(key)
        if e["classification"] not in ALLOWED or e["symbol"] not in s or e["physical_kind"]!=s[e["symbol"]]:raise Fail(f"invalid classified callsite {key}")
        if any(not isinstance(e[x],str) or not e[x] for x in REQ):raise Fail(f"empty binding field {key}")
    a=d.get("authority")
    if not isinstance(a,dict) or not a or any(v is not False for v in a.values()):raise Fail("source inventory must not assert authority")
    return d

def strip(src):
    out=list(src);n=len(src);i=0
    def blank(a,b):
        for x in range(a,b):
            if out[x]!="\n":out[x]=" "
    while i<n:
        if src.startswith("//",i):
            a=i;i=src.find("\n",i+2);i=n if i<0 else i;blank(a,i);continue
        if src.startswith("/*",i):
            a=i;dep=1;i+=2
            while i<n and dep:
                if src.startswith("/*",i):dep+=1;i+=2
                elif src.startswith("*/",i):dep-=1;i+=2
                else:i+=1
            if dep:raise Fail("unterminated Rust block comment")
            blank(a,i);continue
        q=i+2 if src.startswith(("br","rb"),i) else i+1 if src[i]=="r" else -1
        if q>=0:
            x=q
            while x<n and src[x]=="#":x+=1
            if x<n and src[x]=='"':
                end='"'+"#"*(x-q);j=src.find(end,x+1)
                if j<0:raise Fail("unterminated Rust raw string")
                j+=len(end);blank(i,j);i=j;continue
        c=src[i]
        if c in ('"',"'"):
            if c=="'" and i+1<n and (src[i+1].isalpha() or src[i+1]=="_"):
                x=i+2
                while x<n and (src[x].isalnum() or src[x]=="_"):x+=1
                if x>=n or src[x]!="'":i+=1;continue
            a=i;i+=1;esc=False
            while i<n:
                z=src[i]
                if esc:esc=False
                elif z=="\\":esc=True
                elif z==c:i+=1;break
                i+=1
            else:raise Fail("unterminated Rust literal")
            blank(a,i);continue
        i+=1
    return "".join(out)

def cargo(repo,supplied):
    if supplied:d=load(supplied)
    else:
        try:
            r=subprocess.run(["cargo","metadata","--manifest-path",str(repo/"codex-rs/Cargo.toml"),"--locked","--no-deps","--format-version","1"],cwd=repo,check=True,capture_output=True,text=True)
            d=json.loads(r.stdout)
        except Exception as e:raise Fail(f"cargo metadata failed: {e}") from e
    if not isinstance(d,dict) or not isinstance(d.get("packages"),list):raise Fail("cargo metadata structure drift")
    return d

def owners(repo,data):
    out=[]
    for p in data["packages"]:
        try:
            root=pathlib.Path(p["manifest_path"]).resolve().parent;root.relative_to(repo)
            targets=[(pathlib.Path(t["src_path"]).resolve(),tuple(sorted(t["kind"]))) for t in p["targets"]]
            out.append((root,p["name"],targets))
        except Exception as e:raise Fail(f"invalid Cargo package metadata: {e}") from e
    return out

def owner(path,items):
    found=[x for x in items if path.is_relative_to(x[0])]
    if not found:return "UNREGISTERED",[]
    depth=max(len(x[0].parts) for x in found);found=[x for x in found if len(x[0].parts)==depth]
    if len({x[1] for x in found})!=1:raise Fail(f"ambiguous Cargo owner for {path}")
    root,name,targets=found[0];rel=path.relative_to(root).as_posix();k=set()
    for sp,kind in targets:
        tr=sp.relative_to(root).as_posix()
        if rel==tr or (rel.startswith("tests/") and tr.startswith("tests/")) or (rel.startswith("src/") and tr.startswith("src/")):k.update(kind)
    return name,sorted(k)

def files(repo,ignored):
    result=[]
    for cur,dirs,names in os.walk(repo,followlinks=False):
        root=pathlib.Path(cur);rr=root.relative_to(repo).as_posix();keep=[]
        for name in dirs:
            rel=name if rr=="." else f"{rr}/{name}";p=root/name
            if name==".git" or under(rel,ignored):continue
            if p.is_symlink():raise Fail(f"symlinked source directory {rel}")
            keep.append(name)
        dirs[:]=keep
        for name in names:
            p=root/name;rel=p.relative_to(repo).as_posix()
            if name.endswith(".rs") and not under(rel,ignored):
                if p.is_symlink():raise Fail(f"symlinked Rust source {rel}")
                result.append(p)
    return sorted(result)

def audit(repo,inv,meta):
    repo=repo.resolve(strict=True);p=policy(inv if inv.is_absolute() else repo/inv);cg=cargo(repo,meta);own=owners(repo,cg)
    impl=set(p["boundaryImplementationFiles"]);explicit={(x["file"],x["symbol"]):x for x in p["classifiedCallsites"]};bad=[];occ=[]
    for rel in impl:
        q=repo/rel
        if not q.is_file() or q.is_symlink():bad.append(f"invalid boundary implementation file {rel}")
    syms=p["sensitiveSymbols"];pat=re.compile(r"\b("+"|".join(re.escape(x) for x in sorted(syms,key=len,reverse=True))+r")\b")
    rust=files(repo,p["ignoredPathPrefixes"])
    for path in rust:
        rel=path.relative_to(repo).as_posix()
        try:code=strip(path.read_text(encoding="utf-8"))
        except Exception as e:raise Fail(f"cannot scan {rel}: {e}") from e
        matches=list(pat.finditer(code))
        if not matches:continue
        package,kinds=owner(path.resolve(),own)
        if re.search(r"\b(?:include|include_bytes|include_str|concat_idents)\s*!",code):bad.append(f"generated/included sensitive source {rel}")
        for m in matches:
            sym=m.group(1);key=(rel,sym)
            if rel in impl:cls="VERIFIED_USE_BOUNDARY_IMPLEMENTATION"
            elif key in explicit:cls=explicit[key]["classification"]
            elif under(rel,p["fixturePathPrefixes"]):cls="FIXTURE_ONLY"
            elif under(rel,p["qualificationPathPrefixes"]) or "test" in kinds or "bench" in kinds:cls="QUALIFICATION_ONLY"
            else:cls="UNCLASSIFIED";bad.append(f"unclassified {sym} at {rel}:{code.count(chr(10),0,m.start())+1}")
            occ.append({"file":rel,"line":code.count("\n",0,m.start())+1,"symbol":sym,"physical_kind":syms[sym],"classification":cls,"cargo_package":package,"cargo_target_kinds":kinds})
    observed={(x["file"],x["symbol"]) for x in occ}
    for f,s in sorted(set(explicit)-observed):bad.append(f"stale classified callsite {f}:{s}")
    count=Counter(x["classification"] for x in occ)
    report={"schema":REPORT,"schemaVersion":1,"inventorySchema":SCHEMA,"cargoWorkspaceRoot":pathlib.Path(cg["workspace_root"]).resolve().relative_to(repo).as_posix(),"rustFilesScanned":len(rust),"sensitiveSymbolOccurrences":len(occ),"classificationCounts":dict(sorted(count.items())),"productionCallsites":count["PRODUCTION_VIA_VERIFIED_USE"],"unclassifiedCallsites":count["UNCLASSIFIED"],"occurrences":sorted(occ,key=lambda x:(x["file"],x["line"],x["symbol"])),"failures":bad,"authority":p["authority"],"result":"PASS" if not bad else "FAIL"}
    if bad:raise Fail("; ".join(bad))
    return report

def selftest():
    src='// CheckedModelInvocation invoke_once\nconst S:&str=r#"invoke_once"#;\n/* x /* CheckedModelInvocation */ */\nfn f<\'a>(){let c=\'x\'; let _:CheckedModelInvocation; v.invoke_once();}\n'
    got=[m.group(1) for m in re.finditer(r"\b(CheckedModelInvocation|invoke_once)\b",strip(src))]
    if got!=["CheckedModelInvocation","invoke_once"]:raise AssertionError(got)
    try:strip("/*")
    except Fail:pass
    else:raise AssertionError("unterminated comment accepted")
    print("PASS_HEPTA_V5_B4_SELF_TEST")

def main():
    ap=argparse.ArgumentParser();ap.add_argument("--repo-root",type=pathlib.Path,default=pathlib.Path("."));ap.add_argument("--inventory",type=pathlib.Path,default=pathlib.Path("docs/architecture/HEPTA_V5_B4_GLOBAL_CALLSITE_INVENTORY.json"));ap.add_argument("--metadata",type=pathlib.Path);ap.add_argument("--output",type=pathlib.Path);ap.add_argument("--self-test",action="store_true");a=ap.parse_args()
    try:
        if a.self_test:selftest();return 0
        r=audit(a.repo_root,a.inventory,a.metadata);text=json.dumps(r,indent=2,sort_keys=True)+"\n"
        if a.output:a.output.parent.mkdir(parents=True,exist_ok=True);a.output.write_text(text,encoding="utf-8")
        else:sys.stdout.write(text)
        print("PASS_HEPTA_V5_B4_NO_BYPASS",file=sys.stderr);return 0
    except (Fail,AssertionError,KeyError,ValueError) as e:
        fail=json.dumps({"schema":REPORT,"schemaVersion":1,"result":"FAIL","error":str(e)},indent=2,sort_keys=True)+"\n"
        if a.output:a.output.parent.mkdir(parents=True,exist_ok=True);a.output.write_text(fail,encoding="utf-8")
        print(f"FAIL_HEPTA_V5_B4_NO_BYPASS: {e}",file=sys.stderr);return 1
if __name__=="__main__":raise SystemExit(main())
