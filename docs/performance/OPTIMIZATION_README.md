# Santa-Lang Interpreter Performance Optimizations

**Branch**: `conductor/interpreter-perf-review`
**Goal**: 30-40% overall speedup through targeted optimizations
**Status**: Analysis complete, ready for parallel implementation

---

## Quick Start

Pick a plan file and start implementing. Each file is self-contained with all the information needed:

| Plan File | Priority | Impact | Risk | Effort | Can Parallelize? |
|-----------|----------|--------|------|--------|------------------|
| [HASHMAP_LOOKUP_PLAN.md](HASHMAP_LOOKUP_PLAN.md) | 🔴 Critical | 10-15% | Low | 1-2 days | ✅ Yes |
| [OBJECT_POOL_PLAN.md](OBJECT_POOL_PLAN.md) | 🔴 Critical | 8-12% | Low | 2-3 days | ✅ Yes |
| [COLLECTION_OPS_PLAN.md](COLLECTION_OPS_PLAN.md) | 🟡 Medium | 5-10% | Medium | 3-4 days | ✅ Yes (after Phase 1) |
| [DICTIONARY_HYBRID_PLAN.md](DICTIONARY_HYBRID_PLAN.md) | 🟡 Medium | 5-8% | Medium | 3-4 days | ✅ Yes (independent) |
| [FUNCTION_CALL_PLAN.md](FUNCTION_CALL_PLAN.md) | 🟢 Lower | 3-5% | High | 4-5 days | ❌ No (do last) |

---

## Recommended Approach

### Phase 1: Quick Wins (1-2 weeks) → 15-20% speedup

**Can work in parallel**:
- Person A: [HASHMAP_LOOKUP_PLAN.md](HASHMAP_LOOKUP_PLAN.md) (1-2 days)
- Person B: [OBJECT_POOL_PLAN.md](OBJECT_POOL_PLAN.md) (2-3 days)

**Merge both**, then benchmark:
```bash
make bench/compare V1=main V2=HEAD
# Target: 15-20% improvement
```

### Phase 2: Collection & Dictionary (2-3 weeks) → +8-15% speedup

**Can work in parallel** (after Phase 1 merged):
- Person A: [COLLECTION_OPS_PLAN.md](COLLECTION_OPS_PLAN.md) (3-4 days)
- Person B: [DICTIONARY_HYBRID_PLAN.md](DICTIONARY_HYBRID_PLAN.md) (3-4 days)

**Merge both**, then benchmark:
```bash
make bench/compare V1=main V2=HEAD
# Target: 25-35% total improvement
```

### Phase 3: Function Calls (1 week) → +3-5% speedup

**Sequential** (after Phase 1+2):
- One person: [FUNCTION_CALL_PLAN.md](FUNCTION_CALL_PLAN.md) (4-5 days)

**Final benchmark**:
```bash
make bench/compare V1=main V2=HEAD
# Target: 30-40% total improvement
```

---

## What Each Plan Contains

Every `*_PLAN.md` file includes:

1. **Why This Matters** - Problem statement with real AoC examples
2. **Current Implementation Analysis** - Code walkthrough with line numbers
3. **Proposed Solution** - Complete implementation with before/after code
4. **Implementation Checklist** - Step-by-step tasks
5. **Testing Strategy** - What to test, how to validate
6. **Expected Performance Impact** - Concrete benchmark predictions
7. **Success Criteria** - Must have / Should have / Nice to have
8. **Rollback Plan** - How to revert if issues arise
9. **Timeline** - Estimated effort breakdown

---

## Baseline Performance

Established on 2025-11-05:

```
fibonacci:        639.6ms ± 23.1ms (recursive computation)
list_processing:    7.5ms ±  0.8ms (10k elements: filter+map+fold)
pattern_matching:   2.0ms ±  0.2ms (destructuring patterns)
empty:              1.8ms ±  0.4ms (startup overhead)
```

---

## How to Use These Plans

### Working Solo

1. Pick the highest priority plan that interests you
2. Read through entire plan first (understand why + how)
3. Follow implementation checklist
4. Run tests frequently
5. Benchmark at end
6. Move to next plan if successful

### Working in Parallel

1. **Phase 1**: Split HashMap (Person A) and Object Pool (Person B)
2. **Merge Phase 1**: Combine and benchmark together
3. **Phase 2**: Split Collections (Person A) and Dictionary (Person B)
4. **Merge Phase 2**: Combine and benchmark together
5. **Phase 3**: One person tackles Function Calls

### Critical Success Factors

- ✅ Run `cargo test --package santa-lang` after every change
- ✅ Benchmark with `make bench/compare V1=main V2=HEAD`
- ✅ Test on real AoC solutions (paths in plans)
- ✅ Don't proceed to next phase if current phase fails criteria
- ✅ Use rollback plan if issues arise

---

## Expected Cumulative Results

| After Phase | Benchmark | Expected Time | Improvement |
|-------------|-----------|---------------|-------------|
| **Baseline** | fibonacci | 639ms | - |
|  | list_processing | 7.5ms | - |
| **Phase 1** | fibonacci | ~520ms | 15-20% |
|  | list_processing | ~6.2ms | 15-20% |
| **Phase 2** | fibonacci | ~505ms | 20-22% |
|  | list_processing | ~5.4ms | 28-30% |
| **Phase 3** | fibonacci | ~470ms | 25-28% |
|  | list_processing | ~4.9ms | 35-38% |

---

## Real AoC Impact

Based on analysis of 30+ real solutions:

**Most affected problems** (will see biggest speedups):
- Day 8 (2022): Grid operations with nested maps → 20-30% faster
- Day 11 (2022): Dictionary updates in loops → 25-30% faster
- Day 16 (2022): Recursive memoization → 22-28% faster

**Least affected problems** (still improved):
- Day 1 (2022): Simple map/sum → 10-15% faster
- Day 2 (2022): Pattern matching → 8-12% faster

**Hot operations improved**:
- `map`, `filter`, `fold` - Used in ~90% of solutions
- Dictionary lookups - Used in ~70% of solutions
- Variable access - Used in 100% of solutions
- Integer operations - Used in 100% of solutions

---

## Safety & Rollback

Every plan includes:
- **Rollback instructions** - How to revert if problems
- **Risk assessment** - What could go wrong
- **Validation tests** - How to verify correctness
- **Success criteria** - When to proceed vs rollback

**Golden rule**: If a phase doesn't meet "Must Have" criteria, stop and investigate before proceeding.

---

## Questions?

Each plan is comprehensive and self-contained. Read the relevant plan file for:
- Why this optimization matters
- How to implement it
- What to test
- When to rollback

**After completing any phase**: Post benchmark results and observations. This helps others working in parallel.

---

## Analysis Basis

These plans are based on:
- ✅ Deep analysis of interpreter implementation
- ✅ Review of 30+ real AoC solutions
- ✅ Baseline performance benchmarks
- ✅ Identification of hot paths and bottlenecks
- ✅ Risk assessment for each optimization

**Confidence level**: High. Recommendations are based on real code analysis and usage patterns, not speculation.
