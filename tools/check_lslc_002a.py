# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
import json, subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / 'crates/rusty-lsl/src/stream_info_observed_document_parser.rs'
FIXTURES = ROOT / 'fixtures/compatibility/lslc-002a-bounded-observed-document-shape-parser-fixtures.json'
CLASSES = {'valid', 'damaged', 'truncated', 'oversized', 'non-canonical'}

def require(condition, message):
    if not condition:
        raise ValueError(message)

def materialize(base, operation):
    kind = operation['kind']
    if kind == 'identity':
        return base
    if kind == 'replace-first':
        require(base.count(operation['old']) >= 1, 'fixture replacement source is absent')
        return base.replace(operation['old'], operation['new'], 1)
    if kind == 'remove-final-utf8-bytes':
        encoded = base.encode('utf-8')
        return encoded[:-operation['count']].decode('utf-8')
    raise ValueError(f'unknown fixture operation: {kind}')

def first_difference(left, right):
    a, b = left.encode('utf-8'), right.encode('utf-8')
    for index, pair in enumerate(zip(a, b)):
        if pair[0] != pair[1]:
            return index
    return min(len(a), len(b))

def main():
    fixture = json.loads(FIXTURES.read_text(encoding='utf-8'))
    base = fixture['base_document']
    require(len(base.encode('utf-8')) == fixture['base_document_utf8_bytes'], 'base UTF-8 length drifted')
    cases = fixture['cases']
    require({case['class'] for case in cases} == CLASSES, 'required fixture classes drifted')
    require(len({case['case_id'] for case in cases}) == len(cases), 'fixture IDs must be unique')
    for case in cases:
        candidate = materialize(base, case['operation'])
        expected = case['expected']
        if expected.get('error') == 'InputLimitExceeded':
            require(len(candidate.encode('utf-8')) == expected['actual'], f"{case['case_id']} actual length drifted")
            require(case['operation']['max_input_bytes'] == expected['expected_max'], f"{case['case_id']} bound drifted")
        elif 'byte_offset' in expected:
            if case['class'] in {'damaged', 'non-canonical'}:
                require(first_difference(base, candidate) == expected['byte_offset'], f"{case['case_id']} first offset drifted")
            elif case['class'] == 'truncated':
                require(len(candidate.encode('utf-8')) == expected['byte_offset'], f"{case['case_id']} truncation offset drifted")
    require(all(value is False for value in fixture['claims'].values()), 'inert/non-interoperability claims drifted')

    source = SOURCE.read_text(encoding='utf-8')
    implementation = source.split('#[cfg(test)]', 1)[0]
    for marker in ('FIELD_END_TAGS', '[Range<usize>; FIELD_NAMES.len()]', 'core::array::from_fn', 'is_xml_char', 'try_reserve'):
        if marker == 'try_reserve':
            require(marker not in implementation, 'parser must need no fallible structural allocation')
        else:
            require(marker in implementation, f'missing parser invariant: {marker}')
    for forbidden in ('Vec<', 'Vec::', 'format!(', 'with_capacity', 'find_subslice', 'std::fs', 'std::net', 'socket', 'unsafe'):
        require(forbidden not in implementation, f'unbounded, ambient, or authority surface opened: {forbidden}')

    result = subprocess.run(
        ['cargo', 'test', '--workspace', '--all-targets', '--offline', '--locked', 'stream_info_observed_document_parser::tests::'],
        cwd=ROOT, check=True, capture_output=True, text=True,
    )
    require('6 passed' in result.stdout, 'focused parser tests did not pass')
    routes = {
        'AGENTS.md': 'check_lslc_002a.ps1',
        'README.md': 'ParsedStreamInfoObservedDocument',
        'docs/ARCHITECTURE.md': 'stream_info_observed_document_parser',
        'docs/COMPATIBILITY.md': 'LSLC-002A',
        'docs/PROVENANCE.md': 'LSLC-002A',
        'docs/VALIDATION.md': 'check_lslc_002a.ps1',
        'fixtures/compatibility/README.md': 'LSLC-002A',
        'morphospace/README.md': 'rlsl-lslc-002a-bounded-observed-document-shape-parser',
    }
    for path, marker in routes.items():
        require(marker in (ROOT / path).read_text(encoding='utf-8'), f'missing route: {path}')
    print('LSLC-002A bounded observed document shape parser checks passed.')
    return 0

if __name__ == '__main__':
    raise SystemExit(main())
