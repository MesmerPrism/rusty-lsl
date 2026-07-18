import hashlib,json,re
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
lock=json.loads((ROOT/'morphospace/feature.lock.json').read_text(encoding='utf-8'))
assert lock['revision']==14 and lock['lock_fingerprint']=='787827cfe80bd7ff856a304a4f1389070db264d8726a744cd967bbd0948c2c0c'
candidate=dict(lock);candidate['lock_fingerprint']='0'*64
assert hashlib.sha256(json.dumps(candidate,separators=(',',':')).encode()).hexdigest()==lock['lock_fingerprint']
source=(ROOT/'crates/rusty-lsl/src/runtime_activation.rs').read_text(encoding='utf-8')
assert re.search(r'ACCEPTED_FEATURE_LOCK_FINGERPRINT: &str =\s*"'+lock['lock_fingerprint']+'"',source)
assert 'lslc_004i_current_revision_14_admits_and_prior_fingerprint_rejects' in source
assert '7a1088f2dbd46d33734f5136b01b9e4e2298825db5a1e7df2bdbd94b826c773b' in source
for name in ('lslc-003c-lock-bound-runtime-activation-capability.json','lslc-003s-string-sample-activation-descriptor.json'):
 data=json.loads((ROOT/'fixtures/compatibility'/name).read_text(encoding='utf-8')); text=json.dumps(data)
 assert lock['lock_fingerprint'] in text and '7a1088f2' not in text
assert lock['default_activation']=='disabled' and lock['activation_rule']=='selected-lock-and-runtime-input'
assert len(lock['selected_features'])==9
print('LSLC-004I revision-14 runtime lock rebinding passed')
