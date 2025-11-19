# Refactoring Plan - Project Structure Improvements

## Status: PROPOSED (Not Yet Implemented)

This document outlines a plan to improve the project structure as the codebase
grows.

## Current State

The project is well-organized but has some areas that could be improved:

- `main.rs` is 527 lines (should be <100 lines)
- `ai/config.rs` is 452 lines (could be split)
- Some modules are growing large and could benefit from subdirectories

## Proposed Changes

### Phase 1: Extract Commands (HIGH PRIORITY)

**Goal:** Reduce main.rs from 527 to ~100 lines

**Changes:**

1. Create `src/commands/` module
2. Move `investigate_logs()` to `src/commands/investigate.rs`
3. Move `handle_config()` to `src/commands/config.rs`
4. Keep only CLI parsing and command dispatch in `main.rs`

**Benefits:**

- Testable command logic
- Cleaner main.rs
- Easier to add new commands

**Estimated Effort:** 2-3 hours **Risk:** Low (no logic changes, just moving
code)

### Phase 2: Split AI Config (MEDIUM PRIORITY)

**Goal:** Split ai/config.rs into focused modules

**Changes:**

1. Create `src/ai/config/` directory
2. Split into:
   - `ai_config.rs` - AI provider configuration
   - `analysis_config.rs` - Analysis settings
   - `mcp_config.rs` - MCP settings
   - `mod.rs` - Re-exports

**Benefits:**

- Single responsibility per file
- Easier to maintain
- Clearer ownership

**Estimated Effort:** 1-2 hours **Risk:** Low (mostly moving code)

### Phase 3: Reorganize AI Module (LOWER PRIORITY)

**Goal:** Better organization of AI module

**Changes:**

1. Create `src/ai/analysis/` for parallel, retry, progress, statistics
2. Create `src/ai/extraction/` for json_extractor
3. Keep providers/ as is

**Benefits:**

- Logical grouping
- Easier navigation
- Clearer module boundaries

**Estimated Effort:** 2-3 hours **Risk:** Medium (many imports to update)

### Phase 4: Reorganize Parser Module (FUTURE)

**Goal:** Better organization of parser module

**Changes:**

1. Create `src/parser/processing/` for parallel, encoding, metadata, stack_trace
2. Create `src/parser/analysis/` for detector, timestamp, statistics
3. Keep formats/ as is

**Benefits:**

- Clear separation of concerns
- Easier to find related code
- Better for future additions

**Estimated Effort:** 2-3 hours **Risk:** Medium (many imports to update)

## Implementation Guidelines

### Before Starting

1. Ensure all tests pass
2. Create a feature branch
3. Commit frequently

### During Refactoring

1. Move one file at a time
2. Update imports
3. Run tests after each change
4. Keep old structure until fully migrated

### After Completion

1. Run full test suite
2. Update documentation
3. Update PROJECT_STRUCTURE.md
4. Create PR for review

## Testing Strategy

For each phase:

1. Run unit tests: `cargo test --lib`
2. Run integration tests: `cargo test --test '*'`
3. Run doc tests: `cargo test --doc`
4. Build release: `cargo build --release`
5. Manual smoke test: Test main commands

## Rollback Plan

If issues arise:

1. Revert to previous commit
2. Identify specific problem
3. Fix and retry
4. If blocked, defer to next version

## Success Criteria

### Phase 1

- [ ] main.rs < 150 lines
- [ ] All tests pass
- [ ] Commands are testable
- [ ] No behavior changes

### Phase 2

- [ ] ai/config.rs split into 3 files
- [ ] Each file < 200 lines
- [ ] All tests pass
- [ ] No behavior changes

### Phase 3

- [ ] AI module has 3 subdirectories
- [ ] Related code grouped together
- [ ] All tests pass
- [ ] No behavior changes

### Phase 4

- [ ] Parser module has 3 subdirectories
- [ ] Related code grouped together
- [ ] All tests pass
- [ ] No behavior changes

## Timeline

- **Phase 1:** Next sprint (high value, low risk)
- **Phase 2:** Following sprint (good value, low risk)
- **Phase 3:** Future (nice to have)
- **Phase 4:** Future (nice to have)

## Notes

- This is a living document
- Update as we learn more
- Phases can be adjusted based on priorities
- Focus on value and risk

## Decision

**Recommendation:** Implement Phase 1 (commands extraction) in next development
cycle.

**Rationale:**

- High impact (makes main.rs much cleaner)
- Low risk (just moving code)
- Enables better testing
- Foundation for future improvements

**Defer:** Phases 3-4 until we see clear need or pain points.
