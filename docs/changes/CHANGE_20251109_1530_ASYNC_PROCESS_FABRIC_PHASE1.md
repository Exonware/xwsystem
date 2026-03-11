# Feature - Async Process Fabric Phase 1

**Library:** exonware-xwsystem  
**Version:** 0.0.1.389  
**Date:** 09-Nov-2025 15:30  
**Author:** eXonware Backend Team

---

## Overview

Delivered the first milestone for IDEA-018 by introducing the `AsyncProcessFabric` facade. The fabric unifies process pools, async message queues, and shared memory helpers under a single async session API. Documentation was refreshed across the reference set and benchmark scenarios recorded for follow-up execution.

**Change Type:** Feature  
**Impact Level:** Medium

---

## Motivation

### Problem Statement

IPC primitives existed in isolation; downstream services duplicated orchestration logic and lacked consistent async lifecycle management.

### Goals

1. Provide an async-first facade that wraps existing IPC building blocks.
2. Surface the new API through `exonware.xwsystem.ipc` and document usage.
3. Align plan/bench/reference documents with the lifecycle (plan → code → test → benchmark → docs).

### Non-Goals

- Implementing telemetry hooks (scheduled for Phase 2).
- Executing performance benchmarks (scenarios documented; harness pending).

---

## Changes Made

### Code Changes

**Files Added:**
- `src/exonware/xwsystem/ipc/async_fabric.py` – AsyncProcessFabric facade tying together process pools, queues, and shared memory.

**Files Modified:**
- `src/exonware/xwsystem/ipc/__init__.py` – Export `AsyncProcessFabric`.
- `tests/1.unit/test_ipc.py` – Added AsyncProcessFabric tests and helper callables.

### Documentation Changes

**Updated:**
- `docs/REF_PROJECT.md` – Current iteration + usage guidance references.
- `docs/REF_PLAN.md` – Active lifecycle checklist for PLAN_20251109_1500.
- `docs/REF_ARCH.md` – IPC module architecture section (Async fabric).
- `docs/REF_API.md` / `docs/REF_USAGE.md` – Public/API usage for AsyncProcessFabric.
- `docs/REF_BENCH.md` – Benchmark scenarios for async fabric.
- `docs/REF_IDEA.md` – IDEA-018 status update (phase 1 delivered).
- `docs/logs/plans/PLAN_20251109_1500_ASYNC_PROCESS_FABRIC.md` – Status + progress log entry.

**Added:**
- `docs/logs/changes/CHANGE_20251109_1530_ASYNC_PROCESS_FABRIC_PHASE1.md` (this file).

---

## Technical Details

### Implementation Approach

- Created `AsyncProcessFabric` with session-scoped orchestration over `AsyncProcessPool`, `AsyncMessageQueue`, and `SharedMemoryManager`.
- Provided helper methods for task submission, result iteration, publish/consume with optional channel filtering, and shared-memory lifecycle helpers.
- Leveraged async context managers to ensure deterministic cleanup; introduced buffering logic for channel filtering to avoid message starvation.
- Updated tests to validate submission pipelines, queue filtering, and shared memory reuse.

**Key Decisions:**
1. Use lazy factories so consumers can inject alternative pool/queue implementations without modifying the facade.
2. Treat channel filtering as a best-effort convenience while keeping the queue single-backed for phase 1.

**Alternatives Considered:**
- Building a fully pluggable bus abstraction (deferred; current facade keeps scope manageable).
- Reusing `AsyncProcessPool` directly in downstream code (rejected for lack of queue/shared memory coordination).

### Architecture Impact

**Before:**
```
ProcessPool / AsyncProcessPool - used independently
AsyncMessageQueue - manual wiring per service
SharedMemoryManager - ad-hoc usage
```

**After:**
```
AsyncProcessFabric.session()
    ├─ AsyncProcessPool (task orchestration)
    ├─ AsyncMessageQueue (publish/consume facade)
    └─ SharedMemoryManager (segment lifecycle)
```

---

## eXonware 5 Priorities Analysis

1. **Security:** Impact – None. Reuses existing IPC primitives; no new trust boundaries introduced.
2. **Usability:** Impact – Positive. Provides a single async API with documented usage examples.
3. **Maintainability:** Impact – Positive. Consolidates orchestration logic, reducing duplication.
4. **Performance:** Impact – Neutral. Benchmarks pending; facade adds minimal overhead.
5. **Extensibility:** Impact – Positive. Factory overrides enable alternate implementations in future iterations.

---

## Testing

### Test Coverage

**Tests Added:**
- `tests/1.unit/test_ipc.py::TestAsyncProcessFabric` – Validates task submission, channel-aware publish/consume, and shared memory helpers.

**Test Results:**
```
Layer        | Tests | Passed | Failed | Coverage
-------------|-------|--------|--------|----------
1.unit (targeted) | N/A | N/A | N/A | N/A
```

> Attempted `python -m pytest xwsystem/tests/1.unit/test_ipc.py` but pytest is not available in the current environment (`ModuleNotFoundError: pytest`). Manual verification required once pytest is installed.

### Manual Testing

- Local async session invocation via REPL to confirm facade API behaviour (submit/iter/publish/share).

---

## Performance Impact

Benchmarks not executed in this iteration (scenarios documented in `REF_BENCH.md`). No measurable performance data captured.

---

## Migration Guide

**Breaking Changes:** None. Existing IPC imports continue to function; the new facade is additive.

**Upgrade Path:** Consumers can opt into the new API via:
```python
from exonware.xwsystem.ipc import AsyncProcessFabric
```
No additional configuration required.

---

## Impact Analysis

| Component | Impact | Notes |
|-----------|--------|-------|
| IPC module | Medium | Adds async orchestration facade and exports. |
| Documentation set | Medium | Reference docs updated with new iteration guidance. |

---

