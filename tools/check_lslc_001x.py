# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
import json, subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
SOURCE=ROOT/'crates/rusty-lsl/src/stream_info_three_owner_snapshot.rs'
OVERLAY=ROOT/'fixtures/compatibility/lslc-001x-three-owner-acquisition-snapshot-composition-results.json'
TESTS={"all_value_allocations_move_into_complete_s_snapshot","consuming_parts_preserve_evidence_and_snapshot","three_witnesses_remain_separate_without_cross_owner_matching","tighter_s_limit_rejection_is_delegated_unchanged"}
def require(condition,message):
    if not condition: raise ValueError(message)
def main():
    subprocess.run(['python','tools/check_lslc_001v.py'],cwd=ROOT,check=True)
    source=SOURCE.read_text(encoding='utf-8'); implementation=source.split('#[cfg(test)]',1)[0]
    for marker in ('StreamInfoThreeOwnerEvidence','StreamInfoThreeOwnerSnapshot','implementation.into_parts()','runtime.into_parts()','transport.into_parts()','StreamInfoVolatileProviderSnapshot::new','StreamInfoVolatileProviderSnapshotInput::new'):
        require(marker in implementation,f'missing invariant: {marker}')
    for forbidden in ('.acquire()','provider_identity()','epoch()','revision()','== expected','std::time','SystemTime','Instant','std::env','std::net','socket','gethostname','random','spawn('):
        require(forbidden not in implementation,f'cross-owner or ambient surface opened: {forbidden}')
    overlay=json.loads(OVERLAY.read_text(encoding='utf-8'))
    require(overlay['witnesses']==['implementation','runtime','transport'],'witness separation drifted')
    require(overlay['cross_owner_inference'] is False,'cross-owner inference entered')
    require(overlay['ambient_acquisition'] is False,'ambient acquisition entered')
    require(set(overlay['focused_rust_tests'])==TESTS,'focused test inventory drifted')
    result=subprocess.run(['cargo','test','--workspace','--all-targets','--offline','--locked','stream_info_three_owner_snapshot::tests::'],cwd=ROOT,check=True,capture_output=True,text=True)
    require('4 passed' in result.stdout,'focused tests did not pass')
    for path,marker in {'AGENTS.md':'check_lslc_001x.ps1','README.md':'StreamInfoThreeOwnerSnapshot','docs/ARCHITECTURE.md':'stream_info_three_owner_snapshot','docs/COMPATIBILITY.md':'LSLC-001X','docs/PROVENANCE.md':'LSLC-001X','docs/VALIDATION.md':'check_lslc_001x.ps1','fixtures/compatibility/README.md':'LSLC-001X','morphospace/README.md':'rlsl-lslc-001x-three-owner-acquisition-snapshot-composition'}.items():
        require(marker in (ROOT/path).read_text(encoding='utf-8'),f'missing route: {path}')
    print('LSLC-001X three-owner acquisition snapshot composition checks passed.')
    return 0
if __name__=='__main__': raise SystemExit(main())
