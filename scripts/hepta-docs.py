#!/usr/bin/env python3
"""Closed-world verifier for the canonical Hepta V8 development system."""
from __future__ import annotations
import argparse, hashlib, json, os, re, subprocess, sys
from collections import Counter, defaultdict, deque
from datetime import datetime, timedelta, timezone
from pathlib import Path
from typing import Any

ROOT=Path(__file__).resolve().parents[1]
PLAN_ID='HEPTA-GLOBAL-MODULAR-DEVELOPMENT-PLAN'; VERSION='8.0.0'
REPO='TrillionniumFoundation/hepta-private-ci'; REPO_ID=1320694176; DEFAULT_BRANCH='integration/vnext-main-20260811'
AUTHORITY_KEYS=['runtimeAuthority','productionCaller','productionWriter','modelInvocation','providerDispatch','toolExecution','networkConnect','externalFilesystemMutation','secretOperation','matrixSend','externalEffect','fleetMutation','canonicalSelection','merge','operatorAcceptance','promotion','release']
FILES={'current':'docs/CURRENT.json','system':'docs/governance/DOCUMENT_SYSTEM.json','architecture':'docs/architecture/ARCHITECTURE.json','modules':'docs/modules/MODULES.json','contracts':'docs/contracts/CONTRACTS.json','protocols':'docs/contracts/PROTOCOL_SCHEMAS.json','data':'docs/data/DATA_AUTHORITY.json','work':'docs/delivery/WORK_PACKAGES.json','development':'docs/delivery/DEVELOPMENT_DAG.json','activation':'docs/delivery/ACTIVATION_DAG.json','evidence_dag':'docs/delivery/EVIDENCE_DAG.json','paths':'docs/delivery/PATH_OWNERSHIP.json','objectives':'docs/control-plane/OBJECTIVES.json','ndu':'docs/control-plane/NDU.json','optimization':'docs/control-plane/OPTIMIZATION.json','prompt':'docs/intelligence/PROMPT_INTERVENTIONS.json','learning':'docs/learning/LEARNING_SYSTEM.json','experiments':'docs/learning/EXPERIMENTS.json','artifacts':'docs/learning/ARTIFACTS.json','claims':'docs/evidence/CLAIMS.json','qualification':'docs/evidence/QUALIFICATION.json','evidence':'docs/evidence/INDEX.json','threats':'docs/security/THREAT_MODEL.json'}
SCHEMAS={'current':'hepta.selected-development-source.v2','system':'hepta.document-system.v4','architecture':'hepta.architecture-model.v5','modules':'hepta.module-registry.v5','contracts':'hepta.contract-registry.v2','protocols':'hepta.protocol-schema-registry.v2','data':'hepta.data-authority-registry.v2','work':'hepta.work-package-registry.v4','development':'hepta.development-dag.v3','activation':'hepta.activation-dag.v3','evidence_dag':'hepta.evidence-dag.v3','paths':'hepta.path-ownership.v2','objectives':'hepta.global-objective-registry.v2','ndu':'hepta.ndu-registry.v2','optimization':'hepta.optimization-registry.v1','prompt':'hepta.prompt-intervention-registry.v2','learning':'hepta.learning-system.v2','experiments':'hepta.experiment-registry.v1','artifacts':'hepta.learning-artifact-registry.v1','claims':'hepta.claim-registry.v1','qualification':'hepta.qualification-registry.v2','evidence':'hepta.evidence-index.v5','threats':'hepta.threat-model.v2'}
RECEIPT_SCHEMA='hepta.development-docs-execution-receipt.v5'

class DuplicateKey(ValueError): pass

def die(msg): raise SystemExit('FAIL_HEPTA_DEVELOPMENT_DOCS_V8: '+msg)
def need(ok,msg):
    if not ok: die(msg)

def pairs(items):
    out={}
    for k,v in items:
        if k in out: raise DuplicateKey(k)
        out[k]=v
    return out

def load(rel):
    try: return json.loads((ROOT/rel).read_text(encoding='utf-8'),object_pairs_hook=pairs)
    except Exception as exc: die(f'{rel}: {exc}')

def load_path(path):
    target=Path(path)
    if not target.is_absolute(): target=ROOT/target
    try: return json.loads(target.read_text(encoding='utf-8'),object_pairs_hook=pairs)
    except Exception as exc: die(f'{target}: {exc}')

def git(*args,check=True):
    p=subprocess.run(['git','-C',str(ROOT),*args],text=True,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
    if check and p.returncode: die('git '+' '.join(args)+': '+p.stderr.strip())
    return p.stdout.strip()

def tracked():
    try: return sorted(x for x in git('ls-files','-z').split('\0') if x)
    except SystemExit: return sorted(str(p.relative_to(ROOT)).replace('\\','/') for p in ROOT.rglob('*') if p.is_file())

def parse_utc(value,label):
    need(isinstance(value,str) and value.strip(),label+' missing')
    raw=value.strip()
    if raw.endswith('Z'): raw=raw[:-1]+'+00:00'
    try: dt=datetime.fromisoformat(raw)
    except ValueError as exc: die(label+' invalid: '+str(exc))
    need(dt.tzinfo is not None,label+' timezone')
    return dt.astimezone(timezone.utc)

def normalized_utc(dt):
    return dt.astimezone(timezone.utc).isoformat().replace('+00:00','Z')

def validate_observation(value,policy,now=None):
    observed=parse_utc(value,'observation timestamp')
    checked=(now or datetime.now(timezone.utc)).astimezone(timezone.utc)
    skew=int(policy['maximumClockSkewSeconds']); ttl=int(policy['dynamicReceiptTtlSeconds'])
    need(skew>=0 and 0<ttl<=604800,'dynamic observation bounds')
    delta=(observed-checked).total_seconds()
    if policy['futureTimestampAllowed'] is False:
        need(delta<=skew,'future observation exceeds clock skew')
    age=(checked-observed).total_seconds()
    need(age<=ttl,'stale observation exceeds TTL')
    return {
        'status':'PASS_HEPTA_DYNAMIC_TIME_EVIDENCE',
        'observedAt':normalized_utc(observed),
        'checkedAt':normalized_utc(checked),
        'ageSeconds':max(0,int(age)),
        'futureSkewSeconds':max(0,int(delta)),
        'maximumClockSkewSeconds':skew,
        'ttlSeconds':ttl,
        'futureTimestampAllowed':policy['futureTimestampAllowed'],
    }

def event_context():
    event={}
    event_path=os.environ.get('GITHUB_EVENT_PATH','')
    if event_path and Path(event_path).is_file():
        try: event=json.loads(Path(event_path).read_text(encoding='utf-8'),object_pairs_hook=pairs)
        except Exception as exc: die('GitHub event: '+str(exc))
    pr=event.get('pull_request') or {}
    return {
        'event':event,
        'pr':pr,
        'number':pr.get('number') or event.get('number'),
        'base':(pr.get('base') or {}).get('sha'),
        'source':(pr.get('head') or {}).get('sha'),
        'eventMerge':pr.get('merge_commit_sha'),
    }

def shape_paths(v,p='$'):
    out={p}
    if isinstance(v,dict):
        for k,c in v.items(): out|=shape_paths(c,f'{p}.{k}')
    elif isinstance(v,list):
        out.add(p+'[]')
        for c in v: out|=shape_paths(c,p+'[]')
    else: out.add(p+':'+type(v).__name__)
    return out

def shape_sha(v): return hashlib.sha256('\n'.join(sorted(shape_paths(v))).encode()).hexdigest()

def prefix(x):
    for s in ('/**','/*'):
        if x.endswith(s): return x[:-len(s)].rstrip('/')
    return x.rstrip('/')

def overlaps(a,b):
    a,b=prefix(a),prefix(b)
    return a==b or a.startswith(b+'/') or b.startswith(a+'/')

def acyclic(nodes,edges,label):
    ns=set(nodes); need(len(ns)==len(nodes),label+' duplicate node')
    ind={n:0 for n in ns}; out=defaultdict(list); seen=set()
    for e in edges:
        a,b=e.get('from'),e.get('to')
        need(a in ns and b in ns and a!=b and (a,b) not in seen,f'{label} edge {a}->{b}')
        seen.add((a,b)); ind[b]+=1; out[a].append(b)
    q=deque(sorted(n for n in ns if ind[n]==0)); count=0
    while q:
        n=q.popleft(); count+=1
        for x in sorted(out[n]):
            ind[x]-=1
            if ind[x]==0: q.append(x)
    need(count==len(ns),label+' cycle')

def reach(nodes,edges):
    out=defaultdict(list)
    for e in edges: out[e['from']].append(e['to'])
    ans={n:set() for n in nodes}
    for n in nodes:
        stack=list(out[n])
        while stack:
            x=stack.pop()
            if x in ans[n]: continue
            ans[n].add(x); stack.extend(out[x])
    return ans

def status_text(d):
    states=Counter(x['state'] for x in d['work']['packages']); cur=d['current']
    lines=['# Hepta Selected Development Source Status','',
           '> Generated by `python3 scripts/hepta-docs.py generate-status`. Do not edit by hand.','',
           f"**Plan:** `{PLAN_ID}` v{VERSION}",
           f"**Repository:** `{cur['repository']['fullName']}`",
           f"**Exact base:** `{cur['repository']['exactBaseHead']}` / `{cur['repository']['exactBaseTree']}`",
           f"**Target branch:** `{cur['candidate']['targetBranch']}`",
           f"**Current package:** `{cur['currentWorkPackage']}`",'',
           'Dynamic Git, CI, review, operator, selection, promotion and release facts are external exact-candidate receipts and are not cached in this file.','',
           '## Registry closure','',
           f"- Modules: **{len(d['modules']['modules'])}**",
           f"- Contracts: **{len(d['contracts']['contracts'])}**",
           f"- Critical protocols: **{len(d['protocols']['protocols'])}**",
           f"- Durable data domains: **{len(d['data']['domains'])}**",
           f"- Work packages: **{len(d['work']['packages'])}**",'',
           '## Work-package states','', '| State | Count |','|---|---:|']
    for k,v in sorted(states.items()): lines.append(f'| `{k}` | {v} |')
    lines+=['','## Baseline claims','', '| Claim | Current level |','|---|---|']
    for k,v in cur['baselineClaims'].items(): lines.append(f'| `{k}` | `{v}` |')
    lines+=['','## Authority posture','',
            'Every canonical authority flag is present and false. Source presence, a generated file, a queued workflow or a fixture is not selection, merge, operator acceptance, promotion or release.','']
    return '\n'.join(lines)

def verify_legacy(system,paths):
    rules=[(x['id'],re.compile(x['regex'],re.I)) for x in system['forbiddenLegacyPathRules']]; hits=[]
    for path in paths:
        if path in system['canonicalPaths']: continue
        for name,rx in rules:
            if rx.search(path): hits.append((path,name)); break
    need(not hits,'legacy paths '+repr(hits[:20]))
    allowed=set(system['referenceScanPolicy']['allowedReferenceFiles'])
    exts=set(system['referenceScanPolicy']['scanExtensions'])
    rr=[(x['id'],re.compile(x['regex'],re.I)) for x in system['forbiddenLegacyReferenceRules']]; hits=[]
    for path in paths:
        if path in allowed or Path(path).suffix.lower() not in exts: continue
        try: text=(ROOT/path).read_text(encoding='utf-8',errors='ignore')
        except OSError: continue
        for name,rx in rr:
            if rx.search(text): hits.append((path,name)); break
    need(not hits,'dangling legacy references '+repr(hits[:20]))

def verify_cleanup_base(system):
    policy=system['knownLegacyDeletion']
    probe=subprocess.run(['git','-C',str(ROOT),'rev-parse','--is-inside-work-tree'],text=True,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
    if probe.returncode!=0:
        return {'evaluated':False,'reason':'not_a_git_worktree','expectedDeletionCount':policy['exactPathCount']}
    base=policy['exactBaseHead']; base_tree=policy['exactBaseTree']
    need(git('rev-parse',base+'^{commit}')==base,'cleanup base commit')
    need(git('rev-parse',base+'^{tree}')==base_tree,'cleanup base tree')
    for path,expected in policy['exactGitObjects'].items():
        need(git('rev-parse',base+':'+path)==expected,'cleanup base object '+path)
    snap=policy['copiedSnapshotPath']
    snapshot=[x for x in git('ls-tree','-r','--name-only',base,'--',snap).splitlines() if x]
    need(len(snapshot)==policy['copiedSnapshotDescendantCount'],'cleanup snapshot descendant count')
    need(len(snapshot)==len(set(snapshot)) and all(x.startswith(snap+'/') for x in snapshot),'cleanup snapshot inventory')
    expected=sorted(snapshot+policy['directPaths'])
    need(len(expected)==policy['exactPathCount'] and len(expected)==len(set(expected)),'cleanup exact inventory count')
    raw=git('diff','--name-status','--no-renames',base+'..HEAD','--')
    deleted=[]; unexpected_status=[]
    for line in raw.splitlines():
        if not line: continue
        parts=line.split('\t',1); need(len(parts)==2,'cleanup diff row')
        status,path=parts
        if status=='D': deleted.append(path)
        elif status.startswith('R') or status.startswith('C'): unexpected_status.append(line)
    need(not unexpected_status,'cleanup rename/copy status '+repr(unexpected_status[:10]))
    need(sorted(deleted)==expected,'cleanup deletion set mismatch')
    code_exts={'.rs','.py','.toml','.yaml','.yml','.sh','.bzl','.bazel','.js','.ts','.tsx','.go','.c','.cc','.h','.hpp'}
    retained=[]
    for path in tracked():
        if path=='docs/governance/DOCUMENT_SYSTEM.json' or Path(path).suffix.lower() not in code_exts: continue
        try: retained.append((path,(ROOT/path).read_text(encoding='utf-8',errors='ignore')))
        except OSError: pass
    consumer_hits=[]
    for old_path in expected:
        if not old_path.lower().endswith('.json'): continue
        terms=(old_path,Path(old_path).name)
        for path,text in retained:
            if any(term in text for term in terms):
                consumer_hits.append({'retainedPath':path,'deletedJson':old_path})
    need(not consumer_hits,'deleted JSON consumer '+repr(consumer_hits[:10]))
    ancestor=subprocess.run(['git','-C',str(ROOT),'merge-base','--is-ancestor',base,'HEAD'],stdout=subprocess.PIPE,stderr=subprocess.PIPE)
    need(ancestor.returncode==0,'cleanup base is not ancestor')
    head=git('rev-parse','HEAD'); tree=git('rev-parse','HEAD^{tree}'); parents=git('rev-list','--parents','-n','1','HEAD').split()[1:]
    return {'evaluated':True,'baseHead':base,'baseTree':base_tree,'head':head,'tree':tree,'parents':parents,
            'snapshotDescendantCount':len(snapshot),'expectedDeletionCount':len(expected),'observedDeletionCount':len(deleted),
            'exactObjectCount':len(policy['exactGitObjects']),'retainedDeletedJsonConsumerHits':0,
            'inventorySha256':hashlib.sha256(('\n'.join(expected)+'\n').encode()).hexdigest()}

def verify()->int:
    req=['README.md','docs/DEVELOPMENT.md','docs/STATUS.md','scripts/hepta-docs.py','.github/workflows/hepta-development-docs.yml',*FILES.values()]
    for rel in req: need((ROOT/rel).is_file(),'missing '+rel)
    d={k:load(v) for k,v in FILES.items()}
    for k,s in SCHEMAS.items(): need(d[k].get('schema')==s,'schema '+k)
    for k,v in d.items():
        need(v.get('planId')==PLAN_ID and v.get('planVersion')==VERSION,'plan binding '+k)
        f=v.get('authorityFlags')
        need(isinstance(f,dict) and list(f)==AUTHORITY_KEYS,k+' authority key closure')
        need(not any(f.values()),k+' positive authority')
    cur=d['current']; r=cur['repository']
    need((r['id'],r['fullName'],r['defaultBranch'])==(REPO_ID,REPO,DEFAULT_BRANCH),'repository identity')
    need(r['exactBaseHead']=='b621768b70a09d56626bb8a2c331e3dc424e6a4d' and r['exactBaseTree']=='f2e82fd525d337efae355adf6f19398812d4180c','base identity')
    policy=cur['dynamicObservationPolicy']
    need(policy['cachedGitOrCiStatusAllowed'] is False and policy['externalExactCandidateReceiptRequired'] is True,'dynamic observation authority')
    need(policy['futureTimestampAllowed'] is False and int(policy['maximumClockSkewSeconds'])==300,'dynamic observation future policy')
    need(0<int(policy['dynamicReceiptTtlSeconds'])<=604800,'dynamic observation TTL')
    system=d['system']
    need(system['canonicalHumanDevelopmentDocument']=='docs/DEVELOPMENT.md','human authority')
    need(set(system['canonicalPaths'])==set(req),'canonical path set')
    closures={x['path']:x for x in system['registryShapeClosures']}
    need(set(closures)==set(FILES.values())-{'docs/governance/DOCUMENT_SYSTEM.json'},'shape closure coverage')
    for k,rel in FILES.items():
        if k=='system': continue
        row=closures[rel]
        need(row['topLevelKeys']==list(d[k]),'top-level closure '+rel)
        need(row['recursiveShapeSha256']==shape_sha(d[k]),'recursive closure '+rel)
    paths=tracked(); verify_legacy(system,paths)
    cleanup=verify_cleanup_base(system)
    need(cleanup['evaluated'] or not (ROOT/'.git').exists(),'cleanup inventory not evaluated')
    mods=d['modules']['modules']; mids={m['id'] for m in mods}
    need(len(mids)==len(mods),'module IDs')
    roots={}; writers={}
    for m in mods:
        need(m['owner'] and m['deputy'] and m['rootBindings'],'module ownership '+m['id'])
        for rb in m['rootBindings']:
            need(rb['path'] not in roots,'root owner '+rb['path']); roots[rb['path']]=m['id']
        for dom in m['writes']:
            need(dom not in writers,'writer '+dom); writers[dom]=m['id']
        for dep in m['uses']: need(dep in mids,'module dependency '+m['id']+'->'+dep)
    acyclic([m['id'] for m in mods],[{'from':x,'to':m['id']} for m in mods for x in m['uses']],'module graph')
    contracts=d['contracts']['contracts']; cids={x['id'] for x in contracts}
    need(len(cids)==len(contracts),'contract IDs')
    for c in contracts:
        need(c['producer'] in mids and set(c['consumers'])<=mids and c['bounded'] and c['authorityDelta']=='none','contract '+c['id'])
    protocols=d['protocols']['protocols']; pids={x['id'] for x in protocols}
    need(len(pids)==len(protocols),'protocol IDs')
    for p in protocols:
        need(p['maximumEncodedBytes']>0 and p['denyUnknownCriticalFields'] and p['fields'],'protocol '+p['id'])
        names=[x['name'] for x in p['fields']]
        need(len(names)==len(set(names)),'protocol duplicate field '+p['id'])
        for f in p['fields']:
            if f['type'] in {'utf8','bounded_array','bounded_object','bounded_vector','bounded_fixed_point_vector','bounded_probability_vector','id128','sha256'}:
                need('maxBytes' in f or f['type'] not in {'utf8','bounded_array','bounded_object','bounded_vector','bounded_fixed_point_vector','bounded_probability_vector'},'unbounded field '+p['id']+'.'+f['name'])
    domains=d['data']['domains']
    need(len(domains)==len({x['id'] for x in domains}),'data IDs')
    for x in domains: need(x['authoritativeWriter'] in mids and x['schemaOwner']==x['authoritativeWriter'],'data writer '+x['id'])
    packages=d['work']['packages']; pkgids={p['id'] for p in packages}
    need(len(pkgids)==len(packages),'package IDs')
    counts=Counter(p['module'] for p in packages)
    qprofiles={x['id'] for x in d['qualification']['profiles']}
    for p in packages:
        need(p['module'] in mids and p['owner'] and p['deputy'],'package module '+p['id'])
        need(p['qualificationProfile'] in qprofiles,'package profile '+p['id'])
        need(type(p['sourceMutationAllowed']) is bool,'package mutation '+p['id'])
        need(bool(p['allowedWritePaths'])==p['sourceMutationAllowed'],'package paths '+p['id'])
        need(p['deliverables'] and p['acceptanceCriteria'] and p['resourceBudget'] and p['rollback'] and p['stopConditions'],'package envelope '+p['id'])
        for c in p['consumesContracts']+p['producesContracts']: need(c in cids,'package contract '+p['id']+' '+c)
        for f in ('developmentAfter','activationAfter','evidenceAfter'):
            for dep in p[f]: need(dep in pkgids and dep!=p['id'],f+' '+p['id'])
    need(set(counts)==mids,'module package coverage')
    for name,field in [('development','developmentAfter'),('activation','activationAfter'),('evidence_dag','evidenceAfter')]:
        dag=d[name]
        need(set(dag['nodes'])==pkgids,name+' nodes')
        expected={(x,p['id']) for p in packages for x in p[field]}
        actual={(x['from'],x['to']) for x in dag['edges']}
        need(expected==actual,name+' edges'); acyclic(dag['nodes'],dag['edges'],name)
    dev=reach(d['development']['nodes'],d['development']['edges'])
    act=reach(d['activation']['nodes'],d['activation']['edges'])
    leases={(x['packageA'],x['packageB']) for x in d['paths']['activeLeases']}; un=[]
    for i,a in enumerate(packages):
        for b in packages[i+1:]:
            if not any(overlaps(x,y) for x in a['allowedWritePaths'] for y in b['allowedWritePaths']): continue
            ordered=b['id'] in dev[a['id']] or a['id'] in dev[b['id']] or b['id'] in act[a['id']] or a['id'] in act[b['id']]
            if not ordered and (a['id'],b['id']) not in leases and (b['id'],a['id']) not in leases: un.append((a['id'],b['id']))
    need(not un,'path conflicts '+repr(un[:20]))
    evid={x['id'] for x in d['evidence']['evidenceTypes']}
    for ladder in d['claims']['ladders']:
        levels=[x['id'] for x in ladder['levels']]
        need(ladder['current'] in levels,'claim current '+ladder['id'])
        for level in ladder['levels']: need(set(level['requires'])<=evid,'claim evidence '+ladder['id']+'.'+level['id'])
    for q in d['qualification']['profiles']:
        need(set(q['requires'])<=evid and q['mandatoryTestClasses'] and q['independentDecisionRoles'],'qualification '+q['id'])
    for x in d['objectives']['hardConstraints']: need(x['learnable'] is False,'learnable hard constraint '+x['id'])
    for p in d['objectives']['baselineProfiles']:
        need(abs(sum(float(x['baselineWeight']) for x in p['dimensions'])-1)<1e-9,'weights '+p['id'])
    need(d['ndu']['allowedSubjectClasses']==['system','domain','agent','episode'],'NDU subject scope')
    need(d['prompt']['security']['untrustedUpgradeToInstructionAllowed'] is False,'prompt trust')
    need(d['learning']['currentRunMutationAllowed'] is False and d['learning']['plasticity']['topologyOnlineActivationAllowed'] is False,'online mutation')
    need(d['artifacts']['loadPolicy']['currentRunReplacementAllowed'] is False and d['artifacts']['loadPolicy']['mixedArtifactGenerationAllowed'] is False,'artifact load')
    need(d['learning']['longitudinal']['minimumIndependentSnapshots']>=3 and d['learning']['longitudinal']['minimumCalendarWindows']>=2,'longitudinal minimum')
    for t in d['threats']['threats']:
        need(t['owner'] in mids and t['prevent'] and t['detect'] and t['respond'],'threat '+t['id'])
    need((ROOT/'docs/STATUS.md').read_text()==status_text(d),'STATUS stale')
    wf=(ROOT/'.github/workflows/hepta-development-docs.yml').read_text()
    for token in ['source-head:','merge-candidate:','github.event.pull_request.head.sha','github.event.pull_request.base.sha',
                  'git merge-tree --write-tree','git commit-tree','persist-credentials: false',
                  'python3 scripts/hepta-docs.py verify','python3 scripts/hepta-docs.py inventory-legacy',
                  'python3 scripts/hepta-docs.py cleanup-inventory','python3 scripts/hepta-docs.py self-test',
                  'python3 scripts/hepta-docs.py receipt-verify','include-hidden-files: true',
                  'actions/upload-artifact@ea165f8d65b6e75b540449e92b4886f43607fa02','contents: read']:
        need(token in wf,'workflow '+token)
    for token in ['contents: write','git push','update-ref','pull-requests: write','paths-ignore:',
                  'github.event.pull_request.merge_commit_sha']:
        need(token not in wf,'workflow mutation or stale identity '+token)
    print(json.dumps({'status':'PASS_HEPTA_DEVELOPMENT_DOCS_V8','planVersion':VERSION,'modules':len(mods),
                      'contracts':len(contracts),'protocols':len(protocols),'dataDomains':len(domains),
                      'workPackages':len(packages),'legacyPaths':0,'unresolvedPathConflicts':0},sort_keys=True))
    return 0

def generate():
    d={k:load(v) for k,v in FILES.items()}
    (ROOT/'docs/STATUS.md').write_text(status_text(d))
    print('WROTE docs/STATUS.md')
    return 0

def inventory():
    s=load(FILES['system']); paths=tracked()
    rules=[(x['id'],re.compile(x['regex'],re.I)) for x in s['forbiddenLegacyPathRules']]; hits=[]
    for path in paths:
        if path in s['canonicalPaths']: continue
        for name,rx in rules:
            if rx.search(path): hits.append({'path':path,'rule':name}); break
    print(json.dumps({'schema':'hepta.legacy-development-inventory.v2','count':len(hits),'matches':hits},indent=2))
    return 1 if hits else 0

def cleanup_inventory(output):
    system=load(FILES['system'])
    payload={'schema':'hepta.development-docs-cleanup-inventory.v1','planVersion':VERSION,
             'cleanup':verify_cleanup_base(system),'authorityGranted':False}
    need(payload['cleanup']['evaluated'],'cleanup inventory requires Git worktree')
    target=Path(output)
    if not target.is_absolute(): target=ROOT/target
    target.parent.mkdir(parents=True,exist_ok=True)
    target.write_text(json.dumps(payload,sort_keys=True)+'\n')
    print(json.dumps(payload,sort_keys=True))
    return 0

def receipt(kind,expected_sha,output):
    need(kind in {'source-head','merge-candidate'},'receipt kind')
    actual=git('rev-parse','HEAD'); tree=git('rev-parse','HEAD^{tree}')
    need(not expected_sha or actual==expected_sha,'receipt expected SHA')
    ctx=event_context(); pr=ctx['pr']; base=ctx['base']; source=ctx['source']; event_merge=ctx['eventMerge']; number=ctx['number']
    parents=git('rev-list','--parents','-n','1','HEAD').split()[1:]
    if pr:
        need(base and source and number,'pull-request event identity')
        if kind=='source-head':
            need(actual==source,'source receipt head')
            need(parents==[base],'source receipt direct parent')
        else:
            need(actual==expected_sha,'merge receipt expected head')
            need(len(parents)==2 and parents[0]==base and parents[1]==source,'merge receipt ordered parents')
    elif kind=='merge-candidate':
        die('merge receipt requires pull-request event')
    base_tree=git('rev-parse',base+'^{tree}') if base else None
    source_tree=git('rev-parse',source+'^{tree}') if source else tree
    merge_candidate=actual if kind=='merge-candidate' else None
    merge_tree=tree if kind=='merge-candidate' else None
    cleanup=verify_cleanup_base(load(FILES['system']))
    need(cleanup['evaluated'],'receipt cleanup inventory')
    now=datetime.now(timezone.utc); policy=load(FILES['current'])['dynamicObservationPolicy']
    verified_at=normalized_utc(now); time_evidence=validate_observation(verified_at,policy,now)
    payload={
        'schema':RECEIPT_SCHEMA,'planVersion':VERSION,'kind':kind,'repository':REPO,'pullRequest':number,
        'baseCommit':base,'baseTree':base_tree,'sourceHead':source or actual,'sourceTree':source_tree,
        'mergeCandidate':merge_candidate,'mergeTree':merge_tree,
        'eventMergeCandidate':event_merge,'eventMergeCandidateTrusted':False,
        'commit':actual,'tree':tree,'parents':parents,'expectedCommit':expected_sha,
        'workflowPath':'.github/workflows/hepta-development-docs.yml',
        'workflowSha256':hashlib.sha256((ROOT/'.github/workflows/hepta-development-docs.yml').read_bytes()).hexdigest(),
        'verifierSha256':hashlib.sha256((ROOT/'scripts/hepta-docs.py').read_bytes()).hexdigest(),
        'verifiedAt':verified_at,'timeEvidence':time_evidence,
        'maximumClockSkewSeconds':policy['maximumClockSkewSeconds'],
        'dynamicReceiptTtlSeconds':policy['dynamicReceiptTtlSeconds'],
        'cleanup':cleanup,'authorityGranted':False,
    }
    target=Path(output)
    if not target.is_absolute(): target=ROOT/target
    target.parent.mkdir(parents=True,exist_ok=True)
    target.write_text(json.dumps(payload,sort_keys=True)+'\n')
    print(json.dumps(payload,sort_keys=True))
    return 0

def receipt_verify(input_path,kind,expected_sha):
    need(kind in {'source-head','merge-candidate'},'receipt verification kind')
    need(bool(expected_sha),'receipt verification expected SHA')
    payload=load_path(input_path); actual=git('rev-parse','HEAD'); tree=git('rev-parse','HEAD^{tree}')
    parents=git('rev-list','--parents','-n','1','HEAD').split()[1:]
    need(actual==expected_sha,'receipt verification checkout SHA')
    need(payload.get('schema')==RECEIPT_SCHEMA,'receipt schema')
    need(payload.get('planVersion')==VERSION and payload.get('kind')==kind,'receipt plan/kind')
    need(payload.get('repository')==REPO and payload.get('authorityGranted') is False,'receipt authority')
    need(payload.get('expectedCommit')==expected_sha and payload.get('commit')==actual,'receipt commit')
    need(payload.get('tree')==tree and payload.get('parents')==parents,'receipt tree/parents')
    need(payload.get('workflowPath')=='.github/workflows/hepta-development-docs.yml','receipt workflow path')
    need(payload.get('workflowSha256')==hashlib.sha256((ROOT/'.github/workflows/hepta-development-docs.yml').read_bytes()).hexdigest(),'receipt workflow digest')
    need(payload.get('verifierSha256')==hashlib.sha256((ROOT/'scripts/hepta-docs.py').read_bytes()).hexdigest(),'receipt verifier digest')
    policy=load(FILES['current'])['dynamicObservationPolicy']
    fresh=validate_observation(payload.get('verifiedAt'),policy)
    stored=payload.get('timeEvidence')
    need(isinstance(stored,dict) and stored.get('status')=='PASS_HEPTA_DYNAMIC_TIME_EVIDENCE','receipt time evidence status')
    for key in ('observedAt','maximumClockSkewSeconds','ttlSeconds','futureTimestampAllowed'):
        need(stored.get(key)==fresh.get(key),'receipt time evidence '+key)
    need(payload.get('maximumClockSkewSeconds')==policy['maximumClockSkewSeconds'],'receipt clock skew policy')
    need(payload.get('dynamicReceiptTtlSeconds')==policy['dynamicReceiptTtlSeconds'],'receipt TTL policy')
    need(payload.get('eventMergeCandidateTrusted') is False,'event merge candidate trust')
    ctx=event_context(); pr=ctx['pr']; base=ctx['base']; source=ctx['source']; number=ctx['number']
    if pr:
        need(payload.get('pullRequest')==number,'receipt PR number')
        need(payload.get('baseCommit')==base and payload.get('sourceHead')==source,'receipt base/source')
        need(payload.get('baseTree')==git('rev-parse',base+'^{tree}'),'receipt base tree')
        need(payload.get('sourceTree')==git('rev-parse',source+'^{tree}'),'receipt source tree')
        if kind=='source-head':
            need(actual==source and parents==[base],'verified source identity')
            need(payload.get('mergeCandidate') is None and payload.get('mergeTree') is None,'source merge identity')
        else:
            need(parents==[base,source],'verified merge ordered parents')
            need(payload.get('mergeCandidate')==actual and payload.get('mergeTree')==tree,'verified merge identity')
    else:
        need(kind=='source-head' and payload.get('sourceHead')==actual,'push source identity')
    cleanup=verify_cleanup_base(load(FILES['system']))
    need(payload.get('cleanup',{}).get('inventorySha256')==cleanup.get('inventorySha256'),'receipt cleanup inventory')
    need(payload.get('cleanup',{}).get('observedDeletionCount')==cleanup.get('observedDeletionCount'),'receipt cleanup count')
    print(json.dumps({'status':'PASS_HEPTA_EXECUTION_RECEIPT_V5','kind':kind,'commit':actual,'tree':tree,
                      'verifiedAt':fresh['checkedAt'],'authorityGranted':False},sort_keys=True))
    return 0

def self_test():
    cases=[]
    try:
        json.loads('{"a":1,"a":2}',object_pairs_hook=pairs); raise AssertionError
    except DuplicateKey: cases.append('duplicate_key')
    try:
        acyclic(['a','b'],[{'from':'a','to':'b'},{'from':'b','to':'a'}],'fixture'); raise AssertionError
    except SystemExit: cases.append('cycle')
    need(overlaps('a/**','a/b') and not overlaps('a/b','a2/b'),'overlap fixture'); cases.append('overlap')
    need(shape_sha({'a':1})!=shape_sha({'a':'1'}),'shape fixture'); cases.append('shape')
    need(list({k:False for k in AUTHORITY_KEYS})==AUTHORITY_KEYS,'authority fixture'); cases.append('authority')
    fixed=datetime(2026,9,1,tzinfo=timezone.utc)
    policy={'futureTimestampAllowed':False,'maximumClockSkewSeconds':300,'dynamicReceiptTtlSeconds':86400}
    validate_observation(normalized_utc(fixed+timedelta(seconds=300)),policy,fixed); cases.append('bounded_future_time')
    for name,observed in [('reject_future_time',fixed+timedelta(seconds=301)),('reject_stale_time',fixed-timedelta(seconds=86401))]:
        try:
            validate_observation(normalized_utc(observed),policy,fixed); raise AssertionError
        except SystemExit: cases.append(name)
    print(json.dumps({'status':'PASS_HEPTA_DEVELOPMENT_DOCS_V8_SELF_TEST','cases':cases,'authorityGranted':False},sort_keys=True))
    return 0

def main():
    ap=argparse.ArgumentParser(); sp=ap.add_subparsers(dest='cmd',required=True)
    for name in ['verify','generate-status','inventory-legacy','self-test']: sp.add_parser(name)
    cp=sp.add_parser('cleanup-inventory'); cp.add_argument('--output',required=True)
    rp=sp.add_parser('receipt'); rp.add_argument('--kind',required=True); rp.add_argument('--expected-sha',default=''); rp.add_argument('--output',required=True)
    vp=sp.add_parser('receipt-verify'); vp.add_argument('--input',required=True); vp.add_argument('--kind',required=True); vp.add_argument('--expected-sha',required=True)
    args=ap.parse_args()
    if args.cmd=='verify': return verify()
    if args.cmd=='generate-status': return generate()
    if args.cmd=='inventory-legacy': return inventory()
    if args.cmd=='self-test': return self_test()
    if args.cmd=='cleanup-inventory': return cleanup_inventory(args.output)
    if args.cmd=='receipt': return receipt(args.kind,args.expected_sha,args.output)
    return receipt_verify(args.input,args.kind,args.expected_sha)

if __name__=='__main__':
    try: raise SystemExit(main())
    except BrokenPipeError: raise SystemExit(1)
