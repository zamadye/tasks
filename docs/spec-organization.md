# Spec Organization Plan

Status: Proposal

This document proposes a standard organization for specification documents to prevent drift and maintain consistency.

## Current State

The project has evolved to include multiple specification documents:

| Document | Location | Size | Purpose |
|----------|----------|------|---------|
| Main spec | `spec/spec.md` | 63KB | Comprehensive platform specification |
| GitHub spec | `spec/github.md` | 14KB | GitHub integration details |
| Session runtime spec | `src/runtime/spec.md` | 11KB | Container/supervisor protocol |
| Example walkthrough | `spec/example.md` | 77KB | Worked example (outdated) |

### Issues Identified

1. **Location inconsistency**: The session runtime spec lives in `src/runtime/spec.md` instead of `spec/`.
2. **CLAUDE.md mismatch**: References `spec/session-runtime.md` which doesn't exist.
3. **Outdated content**: `example.md` uses the old "Symphony" name throughout.
4. **No documented conventions**: No standard format for component specs.

## Recommendation: Modular Specs with Conventions

**Keep the modular approach** rather than merging into one document because:

1. The main spec is already 63KB — adding GitHub (14KB) and runtime (11KB) would make it unwieldy.
2. Component specs serve different audiences (GitHub API users vs. container runtime implementers).
3. Smaller documents are easier for AI agents to consume without context overflow.
4. Parallel editing is simpler with separate files.

### Proposed Structure

```
spec/
├── spec.md              # Main platform spec (source of truth)
├── github.md            # Component: GitHub integration
├── session-runtime.md   # Component: Container/supervisor protocol
├── example.md           # Worked example (needs update)
└── README.md            # Index with cross-references
```

### Naming Convention

- `spec.md` — The main spec, always authoritative
- `<component>.md` — Component specs that expand on main spec sections
- Names should match crate or subsystem names where applicable

### Standard Format for Component Specs

Every component spec should include:

```markdown
# <Component Name>

Status: Draft | Provisional | Stable

This document specifies <brief description>. It is a companion to the main spec
(spec.md §X <section name>).

## 1. Overview

<High-level description with ASCII diagram showing relationship to other components>

## 2-N. Technical Sections

<Numbered sections for organization>

## N+1. Open Questions

<Unresolved design decisions>
```

### Cross-Reference Convention

Component specs should reference the main spec using section numbers:

- `spec.md §9 Sessions` — Section reference
- `spec.md §10.2` — Subsection reference
- `session-runtime.md §4.1` — Cross-component reference

This makes relationships explicit and helps identify when specs might be out of sync.

### Drift Prevention

1. **Single source of truth**: The main spec defines concepts; component specs elaborate.
2. **Explicit dependencies**: Component specs declare which main spec sections they expand.
3. **Version status**: Each spec declares its status (Draft/Provisional/Stable).
4. **Open questions**: Unresolved items are tracked in each spec, not left ambiguous.

## Immediate Actions

1. **Move runtime spec**: `src/runtime/spec.md` → `spec/session-runtime.md`
2. **Update CLAUDE.md**: Fix the project structure documentation
3. **Update example.md**: Replace "Symphony" with "Tasks" (or mark as outdated)
4. **Add spec/README.md**: Index document explaining the spec structure

## Future Considerations

- **Automated checks**: CI could verify cross-references are valid section numbers
- **Change tracking**: Consider a changelog section in each spec
- **Diagrams**: Standardize on ASCII diagrams for portability (no external image deps)

## Decision Needed

Should `example.md` be:
1. Updated to use "Tasks" terminology and reflect current architecture
2. Removed entirely (it's 77KB and significantly outdated)
3. Kept as-is with a deprecation notice

The example is valuable for understanding the system flow, but maintaining it alongside the main spec is effort. Recommend option 1 if someone has bandwidth, otherwise option 3 as a stopgap.
