#!/usr/bin/env python3
import importlib.util, subprocess, unittest
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
SPEC=importlib.util.spec_from_file_location("gate",ROOT/"tools/check_lslc_003m.py"); gate=importlib.util.module_from_spec(SPEC); SPEC.loader.exec_module(gate)
class Tests(unittest.TestCase):
 @classmethod
 def setUpClass(c):
  c.source=subprocess.run(["git","show",f"{gate.BASE}:README.md"],cwd=ROOT,capture_output=True,check=True).stdout; c.archive=(ROOT/"docs/history/README-THROUGH-LSLC-003L.md").read_bytes(); c.readme=(ROOT/"README.md").read_bytes()
 def test_live(c): gate.validate(c.source,c.archive,c.readme)
 def test_archive_omission(c):
  with c.assertRaises(gate.RouterError): gate.validate(c.source,c.archive[:-1],c.readme)
 def test_archive_change(c):
  with c.assertRaises(gate.RouterError): gate.validate(c.source,b"X"+c.archive[1:],c.readme)
 def test_archive_duplicate(c):
  with c.assertRaises(gate.RouterError): gate.validate(c.source,c.archive+c.archive,c.readme)
 def test_each_route_omission(c):
  for m in gate.ROUTES:
   with c.subTest(m=m), c.assertRaises(gate.RouterError): gate.validate(c.source,c.archive,c.readme.replace(m,b"missing",1))
 def test_each_capability_omission(c):
  for m in gate.CAPABILITIES:
   with c.subTest(m=m), c.assertRaises(gate.RouterError): gate.validate(c.source,c.archive,c.readme.replace(m,b"missing",1))
 def test_overclaim(c):
  with c.assertRaises(gate.RouterError): gate.validate(c.source,c.archive,c.readme+b"production-ready")
if __name__=="__main__": unittest.main()
