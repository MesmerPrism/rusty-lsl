# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
import json, subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]; SOURCE=ROOT/'crates/rusty-lsl/src/stream_info_runtime_provider.rs'; OVERLAY=ROOT/'fixtures/compatibility/lslc-001u-runtime-assigned-acquisition-evidence-results.json'
TESTS={"evidence_limits_are_explicit","one_call_shared_witness_and_allocations_are_preserved","provider_and_witness_failures_are_typed","runtime_value_bounds_reject_in_fixed_role_order"}
def require(c,m):
    if not c: raise ValueError(m)
def main():
    subprocess.run(['python','tools/check_lslc_001t.py'],cwd=ROOT,check=True)
    source=SOURCE.read_text(encoding='utf-8'); implementation=source.split('#[cfg(test)]',1)[0]
    for marker in ('trait StreamInfoRuntimeProvider','.acquire()','ProviderIdentityMismatch','EpochMismatch','RevisionMismatch','max_runtime_code_points','CreatedAt','Uid','SessionId','Hostname','[StreamInfoVolatileProviderValue; 4]'): require(marker in implementation,f'missing invariant: {marker}')
    require(implementation.count('.acquire()')==1,'provider call count drifted')
    for forbidden in ('unsafe','std::time','SystemTime','Instant','std::env','std::net','gethostname','random','spawn(','StreamInfoVolatileProviderSnapshot::new'): require(forbidden not in implementation,f'ambient surface opened: {forbidden}')
    overlay=json.loads(OVERLAY.read_text(encoding='utf-8')); require(overlay['roles']==['created_at','uid','session_id','hostname'],'role order drifted'); require(set(overlay['focused_rust_tests'])==TESTS,'test inventory drifted'); require(overlay['ambient_acquisition'] is False,'ambient acquisition entered')
    result=subprocess.run(['cargo','test','--workspace','--all-targets','--offline','--locked','stream_info_runtime_provider::tests::'],cwd=ROOT,check=True,capture_output=True,text=True); require('4 passed' in result.stdout,'focused tests did not pass')
    for path,marker in {'AGENTS.md':'check_lslc_001u.ps1','README.md':'StreamInfoRuntimeProvider','docs/ARCHITECTURE.md':'stream_info_runtime_provider','docs/COMPATIBILITY.md':'LSLC-001U','docs/PROVENANCE.md':'LSLC-001U','docs/VALIDATION.md':'check_lslc_001u.ps1','fixtures/compatibility/README.md':'LSLC-001U','morphospace/README.md':'rlsl-lslc-001u-runtime-assigned-acquisition-evidence'}.items(): require(marker in (ROOT/path).read_text(encoding='utf-8'),f'missing route: {path}')
    print('LSLC-001U runtime-assigned acquisition evidence checks passed.'); return 0
if __name__=='__main__': raise SystemExit(main())
