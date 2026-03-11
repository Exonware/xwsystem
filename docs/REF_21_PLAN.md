# eXonware Planning Reference

**Company:** eXonware.com  
**Author:** eXonware Backend Team  
**Email:** connect@exonware.com  
**Version:** 0.0.2  
**Last Updated:** 08-Nov-2025

---

## 📋 AI-Friendly Document

**This document is designed for both human developers and AI assistants.**  
Provides comprehensive guidelines for creating, managing, and executing plans in eXonware projects. This reference document backs the lifecycle defined in `guides/GUIDE_PLAN.md` and routes you to the appropriate standards based on the type of work being planned.

**Related Documents:**
- [GUIDE_MASTER.md](guides/GUIDE_MASTER.md) - Master standards and shared constraints
- [guides/GUIDE_PLAN.md](guides/GUIDE_PLAN.md) - Development lifecycle & phase gates
- [guides/GUIDE_DEV.md](guides/GUIDE_DEV.md) - Development standards and patterns
- [guides/GUIDE_TEST.md](guides/GUIDE_TEST.md) - Testing strategy and implementation
- [guides/GUIDE_DOCS.md](guides/GUIDE_DOCS.md) - Documentation standards
- [guides/GUIDE_PROJECT.md](guides/GUIDE_PROJECT.md) - Project requirements gathering
- [logs/SUMMARY_PLAN.md](logs/SUMMARY_PLAN.md) - Historical planning log
- [logs/plans/](logs/plans/) - Active and completed plan documents

---

## Table of Contents

1. [Planning Philosophy](#planning-philosophy)
2. [When to Create a Plan](#when-to-create-a-plan)
3. [Plan Types & Routing](#plan-types--routing)
4. [Plan Structure Template](#plan-structure-template)
5. [Naming Conventions](#naming-conventions)
6. [AI-Friendly Plan Patterns](#ai-friendly-plan-patterns)
7. [Plan Lifecycle Management](#plan-lifecycle-management)
8. [Best Practices](#best-practices)
9. [Plan Templates](#plan-templates)
10. [Quality Checklist](#quality-checklist)
11. [Lifecycle Alignment & Traceability](#lifecycle-alignment--traceability)

---

## Planning Philosophy

### Core Principles

**Planning is thinking made visible.**

1. **Plans are living documents** - Update plans as understanding evolves
2. **Plans are communication tools** - For humans AND AI agents
3. **Plans prevent chaos** - Clear roadmaps reduce errors and rework
4. **Plans enable learning** - Document what worked and what didn't
5. **Plans respect eXonware priorities** - Security, Usability, Maintainability, Performance, Extensibility

### Why We Plan

**Without Planning:**
- ? Scope creep and mission drift
- ? Missed requirements and edge cases
- ? Inconsistent quality across features
- ? No clear success criteria
- ? Wasted effort on wrong solutions

**With Planning:**
- ? Clear objectives and success criteria
- ? Structured approach following standards
- ? Reduced errors through thinking first
- ? Better collaboration (human + AI)
- ? Knowledge transfer and continuity

### eXonware Planning Priorities

**Apply the 5 Priorities to ALL plans:**

1. **Security** - Does the plan address security implications?
2. **Usability** - Will the result be intuitive and user-friendly?
3. **Maintainability** - Is the approach clean and sustainable?
4. **Performance** - Have we considered performance impacts?
5. **Extensibility** - Can the solution be extended later?

---

## When to Create a Plan

### ? Always Create a Plan For:

1. **New Features** - Any significant functionality addition
2. **Refactoring** - Structural code improvements
3. **Bug Fixes (Complex)** - Multi-file or architectural bugs
4. **Testing Initiatives** - New test suites or testing strategies
5. **Documentation Projects** - Major doc updates or restructures
6. **Performance Optimization** - Benchmarking and optimization efforts
7. **Project Milestones** - Major deliverables or releases
8. **Integration Work** - Adding external dependencies or APIs
9. **Architecture Changes** - Design pattern modifications
10. **Migration Tasks** - Version upgrades or API changes

### 💡 Consider a Plan For:

1. **Minor Features** - Small additions with dependencies
2. **Bug Fixes (Moderate)** - Bugs affecting multiple components
3. **Configuration Changes** - Non-trivial config updates
4. **Dependency Updates** - Package upgrades with breaking changes

### ? No Plan Needed For:

1. **Trivial Fixes** - Single-line bug fixes
2. **Documentation Typos** - Simple text corrections
3. **Code Formatting** - Pure style updates
4. **Comment Updates** - Adding/fixing inline comments

---

## Plan Types & Routing

**This section tells you which guides to follow based on plan type.**

### Development Plans (DEV)

**When:** Creating, modifying, or refactoring code

**Primary Guide:** [GUIDE_DEV.md](GUIDE_DEV.md)

**Must Follow:**
- eXonware 5 Priorities (Security ? Usability ? Maintainability ? Performance ? Extensibility)
- Core principles (never remove features, never reinvent wheel, think thoroughly)
- Root cause analysis for bug fixes
- Code quality standards
- Import and dependency management rules
- File path comments at top of every file

**Plan Should Include:**
- Security implications analysis
- Impact on existing features
- Code structure and design patterns
- Performance considerations
- Testing strategy (reference GUIDE_TEST.md)
- Documentation updates needed (reference GUIDE_DOCS.md)

**Example Plan Types:**
- `PLAN_20251106_1700_REFACTOR_CODEC_REGISTRY.md`
- `PLAN_20251107_0900_IMPLEMENT_ASYNC_SERIALIZATION.md`
- `PLAN_20251108_1400_FIX_CACHING_MEMORY_LEAK.md`

---

### Testing Plans (TEST)

**When:** Creating tests, test infrastructure, or test automation

**Primary Guide:** [GUIDE_TEST.md](GUIDE_TEST.md)

**Must Follow:**
- Comprehensive testing approach (unit, integration, e2e)
- pytest standards and conventions
- Test organization and naming
- Runner architecture patterns
- Coverage requirements
- Benchmark testing standards

**Plan Should Include:**
- Test scope and coverage targets
- Test data requirements
- Expected edge cases
- Performance benchmarks
- CI/CD integration
- Test documentation

**Example Plan Types:**
- `PLAN_20251106_1800_IMPLEMENT_CODEC_TEST_SUITE.md`
- `PLAN_20251107_1000_ADD_ASYNC_INTEGRATION_TESTS.md`
- `PLAN_20251108_1500_BENCHMARK_SERIALIZATION_PERFORMANCE.md`

---

### Documentation Plans (DOCS)

**When:** Creating, restructuring, or updating documentation

**Primary Guide:** [GUIDE_DOCS.md](GUIDE_DOCS.md)

**Must Follow:**
- Document naming conventions (GUIDE_, REF_, REPORT_, SUMMARY_)
- File organization (all .md in docs/, except root README.md)
- Template standards for each doc type
- AI-friendly formatting
- Cross-referencing standards
- Version history tracking

**Plan Should Include:**
- Documentation scope and audience
- Template selection
- Cross-references to update
- Index updates needed
- Verification criteria

**Example Plan Types:**
- `PLAN_20251106_1900_CREATE_API_REFERENCE.md`
- `PLAN_20251107_1100_RESTRUCTURE_USAGE_GUIDE.md`
- `PLAN_20251108_1600_UPDATE_ARCHITECTURE_DOCS.md`

---

### Project Plans (PROJECT)

**When:** Defining requirements, milestones, or project scope

**Primary Guide:** [GUIDE_PROJECT.md](GUIDE_PROJECT.md)

**Must Follow:**
- Requirements gathering process
- Functional and non-functional requirements
- eXonware 5 Priorities framework
- Milestone definition
- Stakeholder identification
- Success criteria definition

**Plan Should Include:**
- Vision and goals
- Functional requirements (user stories/use cases)
- Non-functional requirements (security, performance, etc.)
- Constraints (technical, business, architectural)
- Milestones and timeline
- Dependencies and risks

**Example Plan Types:**
- `PLAN_20251106_2000_PROJECT_ASYNC_IMPLEMENTATION.md`
- `PLAN_20251107_1200_MILESTONE_V1_RELEASE.md`
- `PLAN_20251108_1700_REQUIREMENTS_PLUGIN_SYSTEM.md`

---

### Multi-Discipline Plans (MIXED)

**When:** Plan involves multiple disciplines (dev + test + docs)

**Primary Guides:** Multiple (all relevant guides)

**Must Follow:**
- ALL applicable guide standards
- Clear section separation
- Cross-disciplinary coordination
- Comprehensive coverage

**Plan Should Include:**
- Section for each discipline
- Clear routing to applicable guides
- Integration points
- Verification for each component

**Example Plan Types:**
- `PLAN_20251106_2100_FEATURE_COMPLETE_ASYNC_SUPPORT.md`
- `PLAN_20251107_1300_RELEASE_V1_PREPARATION.md`
- `PLAN_20251108_1800_REFACTOR_CORE_ARCHITECTURE.md`

---

## Plan Structure Template

**Use the official template:** [plans/TEMPLATE.md](../logs/plans/TEMPLATE.md)

This comprehensive template includes all required sections for creating plans that follow eXonware standards.

**Standard structure for ALL plans:**

```markdown
# Plan - [Brief Description]

**Date:** DD-Mon-YYYY HH:MM  
**Type:** [DEV|TEST|DOCS|PROJECT|MIXED]  
**Status:** [🔵 Planned|🟢 In Progress|? Complete|⏸️ Paused|? Cancelled]  
**Priority:** [🔴 Critical|🟠 High|🟡 Medium|🟢 Low]  
**Effort:** [S|M|L|XL] (Small/Medium/Large/Extra Large)

**Primary Guide:** [Link to GUIDE_*.md]  
**Related Plans:** [Links if applicable]

---

## 📝 Context

**Why this plan exists:**
[Describe the problem, opportunity, or requirement that prompted this plan]

**Background:**
[Relevant history, previous attempts, related work]

**Trigger:**
[What event/decision/need triggered this plan?]

---

## 🎯 Objectives

**Primary Goal:**
[One sentence describing the main objective]

**Specific Objectives:**
- [ ] Objective 1 (measurable)
- [ ] Objective 2 (measurable)
- [ ] Objective 3 (measurable)

**Success Criteria:**
- [ ] Criterion 1 (how to verify success)
- [ ] Criterion 2 (how to verify success)
- [ ] Criterion 3 (how to verify success)

**Out of Scope:**
- What this plan explicitly does NOT cover

---

## 📋 Approach

**Strategy:**
[High-level approach to achieving objectives]

**Steps:**

### Phase 1: [Phase Name]
**Goal:** [What this phase achieves]

1. [ ] Step 1: [Action item]
   - Details, considerations, or sub-steps
   - Links to relevant code/docs

2. [ ] Step 2: [Action item]
   - Details, considerations, or sub-steps

### Phase 2: [Phase Name]
**Goal:** [What this phase achieves]

1. [ ] Step 1: [Action item]
2. [ ] Step 2: [Action item]

[Add more phases as needed]

---

## 🔍 Analysis

**eXonware 5 Priorities Review:**

1. **Security** (#1 Priority)
   - [ ] Security implications identified
   - [ ] Input validation addressed
   - [ ] Authorization/authentication considered
   - Analysis: [Your analysis]

2. **Usability** (#2 Priority)
   - [ ] User experience impact assessed
   - [ ] API design reviewed
   - [ ] Error messages planned
   - Analysis: [Your analysis]

3. **Maintainability** (#3 Priority)
   - [ ] Code structure planned
   - [ ] Design patterns selected
   - [ ] Documentation strategy defined
   - Analysis: [Your analysis]

4. **Performance** (#4 Priority)
   - [ ] Performance impact estimated
   - [ ] Benchmarks planned
   - [ ] Optimization opportunities identified
   - Analysis: [Your analysis]

5. **Extensibility** (#5 Priority)
   - [ ] Future extension points identified
   - [ ] Plugin/hook architecture considered
   - [ ] API flexibility ensured
   - Analysis: [Your analysis]

---

## ? Validation

**Testing Strategy:**
- [ ] Unit tests required: [scope]
- [ ] Integration tests required: [scope]
- [ ] E2E tests required: [scope]
- [ ] Benchmarks required: [scope]

**Verification Steps:**
1. [ ] Verification step 1
2. [ ] Verification step 2
3. [ ] Verification step 3

**Quality Gates:**
- [ ] All tests pass
- [ ] Coverage meets threshold
- [ ] Benchmarks meet targets
- [ ] Documentation complete
- [ ] Code review approved

---

## 🔗 Dependencies

**Prerequisite Work:**
- [ ] Dependency 1: [description, status]
- [ ] Dependency 2: [description, status]

**Blocked By:**
- [ ] Blocker 1: [description, resolution plan]

**Related Plans:**
- [Link to related plan 1]
- [Link to related plan 2]

**External Dependencies:**
- Package/library dependencies
- Infrastructure requirements
- Third-party services

---

## 📊 Impact Analysis

**Files/Modules Affected:**
- `module/file1.py` - [description of changes]
- `module/file2.py` - [description of changes]

**Breaking Changes:**
- [ ] None
- [ ] API changes (describe)
- [ ] Config changes (describe)
- [ ] Migration needed (describe)

**Documentation Updates Required:**
- [ ] README.md
- [ ] GUIDE_*.md: [specify which]
- [ ] REF_*.md: [specify which]
- [ ] CHANGELOG entry
- [ ] Migration guide (if applicable)

---

## ⚠️ Risks & Mitigation

**Risk 1:** [Description]
- **Likelihood:** [Low|Medium|High]
- **Impact:** [Low|Medium|High]
- **Mitigation:** [Strategy]

**Risk 2:** [Description]
- **Likelihood:** [Low|Medium|High]
- **Impact:** [Low|Medium|High]
- **Mitigation:** [Strategy]

---

## 📝 Notes

**Design Decisions:**
- Decision 1: [rationale]
- Decision 2: [rationale]

**Alternatives Considered:**
- Alternative 1: [why rejected]
- Alternative 2: [why rejected]

**Open Questions:**
- [ ] Question 1
- [ ] Question 2

---

## 📅 Timeline

**Estimated Effort:** [X hours/days/weeks]

**Milestones:**
- [ ] Milestone 1: [date] - [deliverable]
- [ ] Milestone 2: [date] - [deliverable]
- [ ] Milestone 3: [date] - [deliverable]

**Target Completion:** [date]

---

## 📎 References

**Documentation:**
- [GUIDE_DEV.md](GUIDE_DEV.md)
- [Other relevant docs]

**Code:**
- [Links to relevant source files]

**External:**
- [External references, articles, docs]

---

## ? Completion Report

**Completion Date:** [When plan was completed]

**Status:** [? Complete|⏸️ Paused|? Cancelled]

**Outcomes:**
- [ ] Objective 1: [achieved/not achieved, notes]
- [ ] Objective 2: [achieved/not achieved, notes]
- [ ] Objective 3: [achieved/not achieved, notes]

**Lessons Learned:**
- What went well
- What could be improved
- Recommendations for future plans

**Follow-up Actions:**
- [ ] Action 1
- [ ] Action 2

**Related Deliverables:**
- [Link to CHANGE_*.md document]
- [Link to PROJECT_*.md document]
- [Link to code PR/commit]

---

*Plan thoroughly, execute confidently* 🚀
```

---

## Naming Conventions

### Plan Files

**Format:** `PLAN_YYYYMMDD_HHMM_DESCRIPTION.md`

**Rules:**
- All plan files go in `docs/plans/` directory
- Use same naming pattern as PROJECT documents
- YYYYMMDD = Date (e.g., 20251106)
- HHMM = 24-hour time (e.g., 1430)
- DESCRIPTION = Brief snake_case description
- Maximum 50 characters for full filename

**Examples:**
```
? PLAN_20251106_1700_IMPLEMENT_ASYNC_SERIALIZATION.md
? PLAN_20251107_0900_REFACTOR_CODEC_REGISTRY.md
? PLAN_20251108_1400_CREATE_API_REFERENCE_DOCS.md
? PLAN_20251109_1000_BENCHMARK_PERFORMANCE_SUITE.md

? plan_async.md (wrong format)
? async_plan.md (wrong format)
? PLAN_ASYNC_SERIALIZATION.md (missing date/time)
```

### Plan Status Updates

**In Plan Filename:**
- Keep original filename unchanged
- Update status field inside document

**Status Values:**
- 🔵 **Planned** - Not started yet
- 🟢 **In Progress** - Currently being executed
- ? **Complete** - Successfully finished
- ⏸️ **Paused** - Temporarily on hold
- ? **Cancelled** - Abandoned/no longer needed

---

## AI-Friendly Plan Patterns

### Structure Principles

**Make plans easy for AI agents to parse and execute:**

1. **Use Consistent Markdown Hierarchy**
   - `#` for title
   - `##` for major sections
   - `###` for subsections
   - `####` for details

2. **Use Checkboxes Extensively**
   - `- [ ]` for pending tasks
   - `- [x]` for completed tasks
   - Enables progress tracking

3. **Provide Explicit Context**
   - Don't assume knowledge
   - Link to relevant docs
   - Explain abbreviations
   - Define technical terms

4. **Use Structured Data**
   - Tables for comparisons
   - Lists for sequences
   - Code blocks for examples
   - Clear labels and headers

5. **Specify Success Criteria Clearly**
   - Measurable outcomes
   - Verifiable conditions
   - No ambiguity

### Clarity Guidelines

**Write for clarity, not brevity:**

```markdown
? BAD (ambiguous):
- Add caching

? GOOD (specific):
- [ ] Add LRU caching to XWCodec._encode() method
  - Cache size: 1000 entries
  - Eviction policy: Least Recently Used
  - Performance target: 50% reduction in encode time
  - Test coverage: 90%+
```

**Provide Context:**

```markdown
? BAD (no context):
## Objectives
- Improve performance

? GOOD (with context):
## Context
Current serialization of large objects (>1MB) takes 500ms average.
Users report timeout issues in production. Benchmark shows 80% time
spent in repeated encoding operations.

## Objectives
- [ ] Reduce serialization time for 1MB objects from 500ms to <100ms
- [ ] Implement caching to avoid repeated encoding of same data
- [ ] Maintain memory usage under 50MB for cache
```

### Routing Clarity

**Make it obvious which guides apply:**

```markdown
## Applicable Standards

**This is a DEVELOPMENT plan. Follow:**
- ? [GUIDE_DEV.md](GUIDE_DEV.md) - All development rules apply
- ? [GUIDE_TEST.md](GUIDE_TEST.md) - Testing strategy required
- ? [GUIDE_DOCS.md](GUIDE_DOCS.md) - API docs must be updated

**Specific sections to follow:**
- GUIDE_DEV.md sec. Code Quality Standards
- GUIDE_DEV.md sec. Performance & Security
- GUIDE_TEST.md sec. Unit Testing Standards
- GUIDE_DOCS.md sec. API Documentation Template
```

---

## Plan Lifecycle Management

### Creation Phase

1. **Identify Need**
   - Determine if plan is needed (see "When to Create a Plan")
   - Identify plan type (DEV, TEST, DOCS, PROJECT, MIXED)

2. **Create Plan File**
   - Use naming convention: `PLAN_YYYYMMDD_HHMM_DESCRIPTION.md`
   - Save in `docs/plans/` directory
   - Use appropriate template

3. **Fill Out Plan**
   - Complete all required sections
   - Route to appropriate guides
   - Define clear success criteria
   - Add to logs/SUMMARY_PLAN.md

4. **Review Plan**
   - Self-review against quality checklist
   - Get peer review if needed
   - Update based on feedback

### Execution Phase

1. **Mark as In Progress**
   - Update status: 🔵 Planned ? 🟢 In Progress
   - Log start in logs/SUMMARY_PLAN.md

2. **Work Through Steps**
   - Check off completed tasks
   - Update notes as understanding evolves
   - Document decisions and changes

3. **Track Progress**
   - Regular status updates
   - Document blockers
   - Adjust timeline if needed

4. **Create Deliverables**
   - Code changes
   - Tests
   - Documentation
   - CHANGE_*.md or PROJECT_*.md report

### Completion Phase

1. **Mark as Complete**
   - Update status: 🟢 In Progress ? ? Complete
   - Fill out "Completion Report" section
   - Link to deliverables

2. **Document Lessons Learned**
   - What worked well
   - What could be improved
   - Recommendations for future

3. **Update logs/SUMMARY_PLAN.md**
   - Add completion entry
   - Link to plan and deliverables
   - Note any deviations from plan

4. **Archive (Optional)**
   - For very old completed plans
   - Move to `plans/_archive/` if desired
   - Update references

### Pause/Cancel Phase

**Pausing:**
- Update status: 🟢 In Progress → ⏸️ Paused
- Document reason for pause
- Document restart conditions
- Log in logs/SUMMARY_PLAN.md

**Cancelling:**
- Update status: 🟢 In Progress → ❌ Cancelled
- Document reason for cancellation
- Document lessons learned
- Log in logs/SUMMARY_PLAN.md

---

### Active Lifecycle Checklist – PLAN_20251109_1500_ASYNC_PROCESS_FABRIC

**Focus:** Deliver the Async Process Fabric facade while honoring plan → code → test → benchmark → docs checkpoints.

1. **Plan**
   - Maintain `PLAN_20251109_1500_ASYNC_PROCESS_FABRIC.md` under `docs/logs/plans/` with status updates and assumptions.
   - Sync scope entries with `REF_22_PROJECT.md` (iteration table) and `REF_12_IDEA.md` (IDEA-018 status).
2. **Code**
   - Implement `AsyncProcessFabric` in `src/exonware/xwsystem/ipc/async_fabric.py`.
   - Update `ipc/__init__.py` and IPC registries to expose the new facade.
3. **Test**
   - Add unit coverage in `tests/1.unit/ipc_tests/` validating async submission, iteration, and graceful shutdown behaviour.
   - Run IPC unit suites (`pytest tests/1.unit/ipc_tests -k async_fabric`) and capture outcomes in the testing logs.
4. **Benchmark**
   - Document throughput/latency scenarios in `REF_54_BENCH.md`; add or stub a harness in `benchmarks/` as time permits.
   - Record benchmark executions (if any) in `docs/logs/benchmarks/`.
5. **Docs & Logs**
   - Update `REF_15_API.md`, `GUIDE_01_USAGE.md`, `REF_21_PLAN.md`, `REF_13_ARCH.md`, `REF_22_PROJECT.md`, and `REF_12_IDEA.md` to reflect the new facade.
   - Summarise lifecycle execution in `docs/logs/plans/PLAN_20251109_1500_ASYNC_PROCESS_FABRIC.md` and add a change entry in `docs/logs/changes/`.

Use this checklist to keep every lifecycle phase synchronized while the plan remains active.

---

## Best Practices

### Planning Best Practices

1. **Think Before You Write**
   - Spend 20% of time planning, 80% executing
   - A good plan saves 10x the time spent creating it

2. **Start with Why**
   - Always explain context and rationale
   - Link to user needs or business goals

3. **Be Specific**
   - Vague plans lead to vague results
   - Define measurable success criteria

4. **Consider All 5 Priorities**
   - Security, Usability, Maintainability, Performance, Extensibility
   - Don't skip any in your analysis

5. **Plan for Failure**
   - Identify risks upfront
   - Have mitigation strategies
   - Define rollback procedures

6. **Keep Plans Updated**
   - Plans are living documents
   - Update as understanding evolves
   - Document changes and reasons

7. **Link Liberally**
   - Reference relevant docs
   - Link to related plans
   - Connect to deliverables

8. **Use Checklists**
   - Break work into verifiable tasks
   - Track progress visibly
   - Nothing falls through cracks

### AI Collaboration Best Practices

**For Human Planners:**

1. **Write for AI Consumption**
   - Use clear, unambiguous language
   - Provide explicit context
   - Structure data consistently

2. **Make Assumptions Explicit**
   - Don't rely on implicit knowledge
   - Define terms and abbreviations
   - Link to background docs

3. **Specify Constraints Clearly**
   - Technical limitations
   - Business requirements
   - Time/resource constraints

**For AI Agents:**

1. **Follow the Plan Structure**
   - Don't skip sections
   - Fill out all required fields
   - Use templates provided

2. **Route to Correct Guides**
   - Identify plan type correctly
   - Reference all applicable guides
   - Follow specified standards

3. **Provide Rationale**
   - Explain why, not just what
   - Link decisions to priorities
   - Justify approach

4. **Update Progress**
   - Check off completed tasks
   - Document blockers immediately
   - Keep status current

---

## Plan Templates

**Official Template:** [plans/TEMPLATE.md](../logs/plans/TEMPLATE.md)

Use this template for all plan types (DEV, TEST, DOCS, PROJECT, BENCH, MIXED). It includes all necessary sections and follows eXonware documentation standards.

### Quick Reference

| Plan Type | Primary Guide | Template File | Use When |
|-----------|---------------|---------------|----------|
| Development | GUIDE_DEV.md | [plans/TEMPLATE.md](../logs/plans/TEMPLATE.md) | Coding, refactoring, bug fixes |
| Testing | GUIDE_TEST.md | [plans/TEMPLATE.md](../logs/plans/TEMPLATE.md) | Creating tests, test infrastructure |
| Documentation | GUIDE_DOCS.md | [plans/TEMPLATE.md](../logs/plans/TEMPLATE.md) | Writing/updating docs |
| Project | GUIDE_PROJECT.md | [plans/TEMPLATE.md](../logs/plans/TEMPLATE.md) | Requirements, milestones |
| Benchmarking | GUIDE_BENCH.md | [plans/TEMPLATE.md](../logs/plans/TEMPLATE.md) | Performance testing plans |
| Mixed | Multiple | [plans/TEMPLATE.md](../logs/plans/TEMPLATE.md) | Multi-discipline work |

### Template Selector

**Answer these questions to pick the right template:**

1. **What is the primary deliverable?**
   - Code ? DEV-TEMPLATE
   - Tests ? TEST-TEMPLATE
   - Documentation ? DOCS-TEMPLATE
   - Requirements/milestones ? PROJECT-TEMPLATE
   - Multiple ? MIXED-TEMPLATE

2. **Which guide will you follow most?**
   - GUIDE_DEV.md ? DEV-TEMPLATE
   - GUIDE_TEST.md ? TEST-TEMPLATE
   - GUIDE_DOCS.md ? DOCS-TEMPLATE
   - GUIDE_PROJECT.md ? PROJECT-TEMPLATE
   - Multiple ? MIXED-TEMPLATE

3. **What are you primarily changing?**
   - Source code ? DEV-TEMPLATE
   - Test files ? TEST-TEMPLATE
   - .md files ? DOCS-TEMPLATE
   - Requirements ? PROJECT-TEMPLATE
   - Everything ? MIXED-TEMPLATE

### Using Templates

1. **Copy the standard structure** from "Plan Structure Template" section
2. **Customize for plan type** - add type-specific sections
3. **Remove unused sections** - if not applicable
4. **Add type-specific sections** - as needed

**All templates include:**
- Header (Date, Type, Status, Priority, Effort)
- Context (Why, Background, Trigger)
- Objectives (Goals, Success Criteria)
- Approach (Strategy, Steps)
- Analysis (5 Priorities Review)
- Validation (Testing, Verification)
- Dependencies (Prerequisites, Blockers)
- Impact (Files, Breaking Changes, Docs)
- Risks & Mitigation
- Timeline
- Completion Report

---

## Quality Checklist

**Use this checklist before finalizing any plan:**

### Required Elements
- [ ] File named correctly: `PLAN_YYYYMMDD_HHMM_DESCRIPTION.md`
- [ ] File saved in `docs/plans/` directory
- [ ] Header complete (Date, Type, Status, Priority, Effort)
- [ ] Primary guide referenced
- [ ] Context section explains WHY
- [ ] Objectives are specific and measurable
- [ ] Success criteria clearly defined
- [ ] Steps are actionable and detailed
- [ ] All 5 priorities analyzed (Security ? Extensibility)
- [ ] Testing/validation strategy defined
- [ ] Dependencies identified
- [ ] Impact analysis complete
- [ ] Risks identified with mitigation
- [ ] Timeline estimated

### Quality Elements
- [ ] Language is clear and unambiguous
- [ ] No jargon without definitions
- [ ] All links work and point to correct docs
- [ ] Checkboxes used for trackable tasks
- [ ] Tables/lists used for structured data
- [ ] Code examples provided where helpful
- [ ] Assumptions made explicit
- [ ] Out-of-scope items listed
- [ ] Alternative approaches documented

### AI-Friendly Elements
- [ ] Consistent markdown hierarchy
- [ ] Explicit routing to guides
- [ ] Context provided (no implicit knowledge)
- [ ] Technical terms explained
- [ ] Verification criteria measurable
- [ ] No ambiguous language
- [ ] Status indicators clear

### Integration Elements
- [ ] Related plans linked
- [ ] Prerequisite work identified
- [ ] Documentation updates planned
- [ ] logs/SUMMARY_PLAN.md entry created
- [ ] CHANGE_*.md or PROJECT_*.md planned for completion

---

## 📚 Planning Examples

### Example 1: Simple Development Plan

**File:** `PLAN_20251106_1700_ADD_RETRY_LOGIC.md`

**Type:** DEV  
**Scope:** Add retry logic to HTTP requests in XWCodec  
**Primary Guide:** GUIDE_DEV.md

**Key Sections:**
- Context: Users report network failures causing serialization to fail
- Objective: Add exponential backoff retry for transient network errors
- Approach: Implement decorator pattern with configurable retry policy
- Testing: Unit tests for retry scenarios, integration test with mock failures
- Impact: No breaking changes, backward compatible

### Example 2: Documentation Restructure Plan

**File:** `PLAN_20251107_0900_CONSOLIDATE_API_DOCS.md`

**Type:** DOCS  
**Scope:** Consolidate scattered API docs into REF_15_API.md  
**Primary Guide:** GUIDE_DOCS.md

**Key Sections:**
- Context: API docs spread across 5 files, hard to navigate
- Objective: Single REF_15_API.md with all API documentation
- Approach: Extract, consolidate, organize by module, add navigation
- Impact: 5 old files to archive, INDEX.md to update
- Validation: All APIs documented, examples work, links valid

### Example 3: Multi-Discipline Feature Plan

**File:** `PLAN_20251108_1400_ASYNC_SERIALIZATION_COMPLETE.md`

**Type:** MIXED  
**Scope:** Complete async serialization feature  
**Primary Guides:** GUIDE_DEV.md, GUIDE_TEST.md, GUIDE_DOCS.md

**Key Sections:**
- Development: Implement async encode/decode methods
- Testing: Unit tests, integration tests, performance benchmarks
- Documentation: Update GUIDE_01_USAGE.md, REF_15_API.md, add examples
- Project: Links to requirements in REF_22_PROJECT.md
- Timeline: 3 phases over 2 weeks

---

## Lifecycle Alignment & Traceability

**Last status review:** 08-Nov-2025 00:00 UTC  
**Alignment targets:** ECSS-E-ST-40C, ISO/IEC 12207 & 15504, NASA NPR 7150.2, DO-178C Level A

This section maps the lifecycle defined in `guides/GUIDE_PLAN.md` to the compliance processes required for eXonware programmes. Reference it when preparing certification evidence or answering lifecycle audits.

### Phase-to-Process Mapping

| eXonware Phase (GUIDE_PLAN) | Primary Outputs (REF_PLAN) | ECSS-E-ST-40C Activity | ISO/IEC 12207 Process | NASA NPR 7150.2 / DO-178C Alignment | Notes |
|-----------------------------|----------------------------|------------------------|-----------------------|--------------------------------------|-------|
| **I. Ideation** (Capture & Define) | Idea briefs, requirements charters (`REF_12_IDEA.md`, `REF_22_PROJECT.md`) | 5.2 Concept & requirements definition | Stakeholder assessment, requirement elicitation | NPR 7150.2 �4.1 inputs; DO-178C planning data Annex A | Ensure security/usability considerations recorded before formal planning. |
| **II. Planning** (Execution Strategy) | Formal plans (`PLAN_*`), routing decisions (this REF), compliance checklists | 5.3 Project planning, 5.4 SW development planning | Project planning, software implementation planning | NPR 7150.2 �5.1, DO-178C �4.6 (plans) | Plans must cite applicable standards and enumerate verification gates. |
| **III. Development** (Build) | Implementation tasks, CHANGE logs, design notes | 5.5 Detailed design, 5.6 Implementation | Software construction, detailed design | NPR 7150.2 �5.2, DO-178C �5 | Plans capture coding guidelines, design constraints, and acceptance criteria. |
| **IV. Quality Loop** (Verify & Validate) | Test/benchmark sections, evidence routing | 5.7 Verification & validation | Verification, validation, joint reviews | NPR 7150.2 �5.3, DO-178C �6 | Each plan lists mandatory tests/benchmarks with traceable IDs. |
| **V. Release** (Deliver & Support) | Release checklists, follow-up tasks, maintenance notes | 5.8 Delivery & maintenance planning | Transition, operation, maintenance | NPR 7150.2 �5.4, DO-178C �7 | Plans define release authority, rollback strategy, and maintenance actions. |

### Traceability Workflow

1. **Capture Requirement IDs** in the plan header/objectives (link back to `REF_22_PROJECT.md` entries).  
2. **Map Tasks to Deliverables** using the "Related Deliverables" checklist (CHANGE/PROJECT/TEST/BENCH documents).  
3. **Record Verification Assets** in the Validation & Verification subsection with direct links to evidence folders.  
4. **Export Metadata** (requirement IDs → plan tasks → deliverables) to the traceability register in `docs/compliance/traceability/`.  
5. **Close the Loop** in completion reports by confirming each requirement ID has matching implementation and verification links.

### Release Checklist Template

Embed the following table in the release section of each PLAN or CHANGE document. During v0/v1 you can satisfy the items with manual links to guides/refs/logs; once the v2 automation lands, swap over to the generated artefacts.

| Item | Description | Evidence |
|------|-------------|----------|
| Traceability refreshed | Run `python tools/ci/generate_trace_matrix.py --package xwsystem` (plus other packages as needed) when metadata exists, **or** directly link the relevant PLAN/CHANGE/TEST docs. | `xwsystem/docs/compliance/traceability/TRACE_MATRIX.md` *(optional until v2)* |
| Risk register review | Confirm Critical/High risks have mitigation tasks and review dates. | `xwsystem/docs/compliance/risk-assessment/README.md` |
| Formal verification scope | Ensure components in `docs/compliance/verification/README.md` have current proof/test links. | PLAN or CHANGE document |
| Benchmark confirmation | Benchmarks meet SLAs; results logged. | `docs/logs/benchmarks/`, `REF_54_BENCH.md` |
| Documentation bundle | CHANGE/PROJECT/SUMMARY docs updated; release notes and rollback plan captured. | Relevant entries under `docs/logs/` |

Proceed with tagging/publishing only when every row is marked complete and reviewed by the release authority.

### Current Status (08-Nov-2025)

- ✅ Lifecycle-to-standards mapping documented above.  
- ✅ GUIDE_PLAN now references this REF as the canonical planning evidence source.  
- ✅ Automated trace matrix generation available via `tools/ci/generate_trace_matrix.py` and `.github/workflows/traceability.yml`.  
- ✅ Release/maintenance checklist defined (table above).  
- Tracking for ECSS-E-40C-RQ-001 maintained in `docs/compliance/standards/ecss/README.md`.

---

## 📚 Related Documentation

**Essential Reading:**
- [GUIDE_DEV.md](GUIDE_DEV.md) - Development standards
- [GUIDE_TEST.md](GUIDE_TEST.md) - Testing standards
- [GUIDE_DOCS.md](GUIDE_DOCS.md) - Documentation standards
- [GUIDE_PROJECT.md](GUIDE_PROJECT.md) - Requirements gathering

**Planning Artifacts:**
- [logs/SUMMARY_PLAN.md](../logs/SUMMARY_PLAN.md) - Historical planning log
- [logs/plans/](../logs/plans/) - All plan documents

**Related Processes:**
- [logs/SUMMARY_CHANGE.md](../logs/SUMMARY_CHANGE.md) - Track implementations
- [logs/SUMMARY_PROJECT.md](../logs/SUMMARY_PROJECT.md) - Track milestones
- [logs/changes/](../logs/changes/) - Implementation details
- [logs/project/](../logs/project/) - Project reports

---

## 🔄 Version History

**0.0.2** (08-Nov-2025)
- Renamed from GUIDE_PLAN to REF_PLAN and linked to GUIDE_PLAN lifecycle
- Added lifecycle alignment & traceability section for ECSS/ISO/NASA compliance
- Updated related-document links and metadata to reflect new structure

**0.0.1** (06-Nov-2025)
- Initial version
- Established plan types and routing
- Created standard template
- Defined naming conventions
- AI-friendly pattern guidelines

---

*Plan with purpose, execute with precision* 🚀



