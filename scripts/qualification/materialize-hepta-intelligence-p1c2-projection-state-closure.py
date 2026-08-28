#!/usr/bin/env python3
from pathlib import Path

path = Path("codex-rs/hepta-memory-p1-1c2-qualification/src/evaluation.rs")
text = path.read_text(encoding="utf-8")
old = """    let final_label_bindings_match = projection_audit.coverage_complete
        && projection_audit.bindings_match
        && final_label_blockers.is_empty();
"""
new = """    let final_label_bindings_match = projection_audit.eligible_for_reviewed_evaluation
        && final_label_blockers.is_empty();
"""
if text.count(old) != 1:
    raise SystemExit("P1.1c.2 final-label projection-state marker is not unique")
path.write_text(text.replace(old, new, 1), encoding="utf-8")
