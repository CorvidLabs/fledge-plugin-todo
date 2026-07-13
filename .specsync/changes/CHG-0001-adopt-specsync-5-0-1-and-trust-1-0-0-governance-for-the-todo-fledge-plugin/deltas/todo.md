## MODIFIED
### SPEC SECTION Change Log
| Version | Date | Changes |
|---------|------|---------|
| 1 | 2026-07-12 | Document existing Todo scanner behavior for SpecSync 5 adoption. |
| 2 | 2026-07-13 | Reconciled existing scanner documentation and stable requirement IDs for SpecSync 5.0.1 governance; runtime behavior is unchanged. |

### REQUIREMENT REQ-todo-005
JSON mode SHALL emit schema version 1 with the action, root, searched markers, count, and normalized match records.

Acceptance Criteria
- The deterministic JSON smoke parses the envelope, checks every required top-level key, and checks the required keys on every emitted match record.

### REQUIREMENT REQ-todo-006
Blocking mode SHALL exit 1 when the final result set is non-empty and 0 when it is empty.

Acceptance Criteria
- The deterministic blocking smoke checks exit 1 for a temporary marker fixture and exit 0 after replacing it with a clean fixture.
