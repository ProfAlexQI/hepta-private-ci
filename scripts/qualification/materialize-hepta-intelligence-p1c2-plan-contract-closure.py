#!/usr/bin/env python3
from pathlib import Path

path = Path("scripts/verify-hepta-intelligence-p1-1c2-reviewed-efficacy.py")
text = path.read_text(encoding="utf-8")
old = '            "exact reference baseline, calibration and efficacy policy",\n'
new = (
    '            "baseline receipt equals the exact embedded P1.1c seed baseline",\n'
    '            "calibration equals `CalibrationContract::qualification_reference()`",\n'
    '            "efficacy policy equals the exact source-only default policy",\n'
)
if text.count(old) != 1:
    raise SystemExit("P1.1c.2 broad reference-plan marker is not unique")
path.write_text(text.replace(old, new, 1), encoding="utf-8")
