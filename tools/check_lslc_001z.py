# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
import json, subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
SOURCE=ROOT/'crates/rusty-lsl/src/stream_info_three_owner_observed_document.rs'
OVERLAY=ROOT/'fixtures/compatibility/lslc-001z-three-owner-observed-document-composition-results.json'
TESTS={'lslc_001z_one_call_produces_document_and_retains_three_witnesses','lslc_001z_r_rejection_remains_stage_typed'}
def require(condition,message):
    if not condition: raise ValueError(message)
def main():
    for checker in ('check_lslc_001n.py','check_lslc_001p.py','check_lslc_001q.py','check_lslc_001r.py','check_lslc_001x.py'):
        subprocess.run(['python',f'tools/{checker}'],cwd=ROOT,check=True)
    source=SOURCE.read_text(encoding='utf-8'); implementation=source.split('#[cfg(test)]',1)[0]
    for marker in ('StreamInfoVolatileXml::compose','StreamInfoOrderedXml::compose','StreamInfoObservedDocument::project','snapshot.into_parts()','StreamInfoThreeOwnerEvidence'):
        require(marker in implementation,f'missing composition invariant: {marker}')
    for forbidden in ('::acquire(', 'provider_identity()', '.epoch()', '.revision()', 'std::time', 'SystemTime', 'Instant', 'std::env', 'std::net', 'socket', 'gethostname', 'random', 'spawn('):
        require(forbidden not in implementation,f'ambient or authority surface opened: {forbidden}')
    overlay=json.loads(OVERLAY.read_text(encoding='utf-8'))
    require(overlay['stages']==['P','Q','R'],'composition stage inventory drifted')
    require(overlay['witnesses']==['implementation','runtime','transport'],'witness separation drifted')
    require(overlay['wire_claim'] is False and overlay['runtime_activation'] is False,'inert boundary drifted')
    require(set(overlay['focused_rust_tests'])==TESTS,'focused test inventory drifted')
    result=subprocess.run(['cargo','test','--workspace','--all-targets','--offline','--locked','stream_info_three_owner_observed_document::tests::'],cwd=ROOT,check=True,capture_output=True,text=True)
    require('2 passed' in result.stdout,'focused tests did not pass')
    for path,marker in {'AGENTS.md':'check_lslc_001z.ps1','README.md':'StreamInfoThreeOwnerObservedDocument','docs/ARCHITECTURE.md':'stream_info_three_owner_observed_document','docs/COMPATIBILITY.md':'LSLC-001Z','docs/PROVENANCE.md':'LSLC-001Z','docs/VALIDATION.md':'check_lslc_001z.ps1','fixtures/compatibility/README.md':'LSLC-001Z','morphospace/README.md':'rlsl-lslc-001z-three-owner-observed-document-composition'}.items():
        require(marker in (ROOT/path).read_text(encoding='utf-8'),f'missing route: {path}')
    print('LSLC-001Z three-owner observed document composition checks passed.')
    return 0
if __name__=='__main__': raise SystemExit(main())
