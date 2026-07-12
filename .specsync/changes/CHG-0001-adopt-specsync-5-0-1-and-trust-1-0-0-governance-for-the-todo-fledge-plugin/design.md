---
change: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-todo-fledge-plugin
artifact: design
---

# Design

Retain CI and Pages unchanged, then add an independently required trust job. The standard Trust profile blocks risk, keeps provenance soft, enforces 100% contract coverage, and disables Trust-managed Atlas because Pages remains separately managed. A native Fledge lane composes Rust checks, ShellCheck, manifest validation, JSON parsing, and blocking-mode smoke behavior without recursive governance calls.

