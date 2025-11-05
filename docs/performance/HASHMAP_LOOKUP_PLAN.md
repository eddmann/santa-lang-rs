# Optimization: HashMap Variable Lookup

**Priority**: 🔴 Critical - Phase 1A
**Impact**: 10-15% speedup
**Risk**: Low
**Effort**: 1-2 days
**Can work in parallel**: Yes (independent of other optimizations)

---

## Why This Matters

### The Problem

Every variable access in santa-lang does an **O(n) linear search** through the environment scope chain. This affects literally every identifier lookup in every AoC solution.

### Real AoC Impact

**Example from Day 8 (2022) - Tree Grid Visibility**:
```santa
let grid = parse_grid(input);          // Variable 'grid'
let directions = grid_directions(grid); // Variable 'directions'

grid |> count(|tree_height, position| {
  directions(position)                  // 'directions' lookup
    |> any?(all?(|viewpoint| grid[viewpoint] < tree_height))
                                        // 'grid' lookup in nested loop
});
```

The `grid` variable is looked up **1000+ times** in nested iterations. Each lookup walks through:
1. Current environment (local closure variables)
2. Outer environment (function scope)
3. Outer-outer environment (program scope)

**Current cost per lookup**: O(n) where n = number of variables in scope chain

### Measured Impact

From benchmark analysis:
- **fibonacci**: 639ms baseline → Many recursive calls with environment lookups
- **list_processing**: 7.5ms → Less affected (fewer variables in scope)
- **Real AoC solutions**: Typically have 5-15 variables in program scope

### Why It's O(n)

**Current implementation** (`lang/src/evaluator/environment.rs:89`):
```rust
pub fn get_variable(&self, name: &str) -> Option<Rc<Object>> {
    for (name_, value, _) in &self.store {  // Linear search through Vec
        if name_ == name {
            return Some(Rc::clone(value));
        }
    }
    if let Some(outer) = &self.outer {
        return outer.borrow().get_variable(name);  // Recursive chain walk
    }
    None
}
```

**Worst case**: Variable at program scope accessed from deeply nested closure = O(n × depth)

---

## Current Implementation Analysis

### Data Structure

**File**: `lang/src/evaluator/environment.rs`

```rust
pub struct Environment {
    store: Vec<(String, Rc<Object>, bool)>,  // (name, value, is_mutable)
    sections: Vec<(String, Rc<Section>)>,    // AoC sections (rarely used)
    outer: Option<EnvironmentRef>,           // Parent scope
}
```

### All Methods That Need Updating

1. **get_variable()** - Line 89: O(n) search
2. **set_variable()** - Line 100: O(n) search
3. **define_variable()** - Line 134: Vec push
4. **contains()** - Line 145: O(n) search
5. **keys()** - Line 153: Iterate Vec

---

## Proposed Solution

### Change to HashMap

```rust
use std::collections::HashMap;

pub struct Environment {
    store: HashMap<String, (Rc<Object>, bool)>,  // O(1) lookup
    sections: Vec<(String, Rc<Section>)>,         // Keep Vec (rarely accessed)
    outer: Option<EnvironmentRef>,
}
```

### Updated Methods

#### 1. get_variable() - O(1) lookup

```rust
pub fn get_variable(&self, name: &str) -> Option<Rc<Object>> {
    if let Some((value, _)) = self.store.get(name) {
        return Some(Rc::clone(value));
    }
    if let Some(outer) = &self.outer {
        return outer.borrow().get_variable(name);
    }
    None
}
```

#### 2. set_variable() - O(1) lookup + update

```rust
pub fn set_variable(&mut self, name: String, value: Rc<Object>) -> Result<(), RuntimeErr> {
    if let Some((stored_value, is_mutable)) = self.store.get_mut(&name) {
        if *is_mutable {
            *stored_value = value;
            return Ok(());
        }
        return Err(RuntimeErr::new(
            "Cannot reassign immutable variable",
            Location::default()
        ));
    }

    if let Some(outer) = &self.outer {
        return outer.borrow_mut().set_variable(name, value);
    }

    Err(RuntimeErr::new("Variable not found", Location::default()))
}
```

#### 3. define_variable() - O(1) insert

```rust
pub fn define_variable(&mut self, name: String, value: Rc<Object>, is_mutable: bool) {
    self.store.insert(name, (value, is_mutable));
}
```

#### 4. contains() - O(1) check

```rust
pub fn contains(&self, name: &str) -> bool {
    self.store.contains_key(name)
        || self.outer.as_ref().map_or(false, |outer| outer.borrow().contains(name))
}
```

#### 5. keys() - Convert HashMap keys

```rust
pub fn keys(&self) -> Vec<&str> {
    self.store.keys().map(|s| s.as_str()).collect()
}
```

### Constructor Updates

```rust
impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),  // Changed from Vec::new()
            sections: Vec::new(),
            outer: None,
        }
    }

    pub fn from(outer: EnvironmentRef) -> Self {
        Environment {
            store: HashMap::new(),  // Changed from Vec::new()
            sections: Vec::new(),
            outer: Some(outer),
        }
    }
}
```

---

## Implementation Checklist

### Step 1: Update Data Structure
- [ ] Change `store` field type from `Vec` to `HashMap`
- [ ] Update both constructors (`new()` and `from()`)

### Step 2: Update Methods (5 methods)
- [ ] `get_variable()` - use `.get()`
- [ ] `set_variable()` - use `.get_mut()`
- [ ] `define_variable()` - use `.insert()`
- [ ] `contains()` - use `.contains_key()`
- [ ] `keys()` - use `.keys().map(...).collect()`

### Step 3: Verify No Other Dependencies
- [ ] Search for `environment.store` access outside the module
- [ ] Check if any code pattern-matches on `Environment` struct

---

## Testing Strategy

### 1. Existing Test Suite (Must Pass)

```bash
# Core language tests
cargo test --package santa-lang

# Specific environment tests
cargo test --package santa-lang environment

# All evaluator tests (use variables extensively)
cargo test --package santa-lang evaluator
```

### 2. Benchmark Comparison

```bash
# Baseline (before changes)
git stash
make bench/run
# Note: fibonacci=639ms, list_processing=7.5ms

# After changes
git stash pop
make bench/run

# Compare
make bench/compare V1=main V2=HEAD
```

**Expected improvements**:
- fibonacci: 10-15% faster (many variable lookups in recursion)
- list_processing: 5-10% faster (fewer variables, but still present)

### 3. Create Worst-Case Benchmark

Add `benchmarks/fixtures/variable_lookup.santa`:
```santa
// Worst case: many variables in scope, accessed in hot loop
let a = 1; let b = 2; let c = 3; let d = 4; let e = 5;
let f = 6; let g = 7; let h = 8; let i = 9; let j = 10;

1..1000
  |> map(|x| a + b + c + d + e + f + g + h + i + j + x)
  |> sum
```

**Expected**: 20-30% improvement (many variables accessed repeatedly)

### 4. Real AoC Solution Testing

Test on actual AoC problems with variable-heavy patterns:

```bash
cargo build --release --bin santa-cli

# Day 8: Heavy 'grid' variable access
time target/release/santa-cli ~/Projects/advent-of-code/2022/santa-lang/aoc2022_day08.santa

# Day 11: Nested variable lookups in folds
time target/release/santa-cli ~/Projects/advent-of-code/2022/santa-lang/aoc2022_day11.santa

# Day 16: Memoization with many variable captures
time target/release/santa-cli ~/Projects/advent-of-code/2022/santa-lang/aoc2022_day16.santa
```

### 5. Correctness Validation

Key scenarios to verify:
- [ ] Variable shadowing works correctly
- [ ] Mutable variables can be reassigned
- [ ] Immutable variables cannot be reassigned
- [ ] Scope chain traversal finds variables in outer scopes
- [ ] `contains()` works across scope chain
- [ ] `keys()` returns correct variable names

---

## Expected Performance Impact

### Benchmark Predictions

| Benchmark | Before | After | Improvement |
|-----------|--------|-------|-------------|
| fibonacci | 639ms | ~550ms | 10-15% |
| list_processing | 7.5ms | ~6.8ms | 5-10% |
| variable_lookup | N/A | Create new | Baseline |
| pattern_matching | 2.0ms | ~1.9ms | 0-5% |

### Real AoC Solutions

Based on variable usage patterns:
- **Day 1-5** (simple): 5-8% improvement
- **Day 8-12** (moderate): 10-15% improvement
- **Day 16-20** (complex): 15-20% improvement

### Why HashMap Won't Regress Small Cases

**Concern**: HashMap overhead for 1-3 variables?

**Analysis**:
- Rust's `HashMap` is well-optimized for small sizes
- Uses FNV hash for small keys (fast)
- Typical environment has 5-15 variables (HashMap sweet spot)
- Even with 1-2 variables: ~2-3ns vs ~1ns (negligible)

**Validation**: Our benchmarks will catch any regression

---

## Success Criteria

### Must Have ✅
- All existing tests pass (100%)
- No regressions on any benchmark (within 2% variance)
- At least 8% improvement on fibonacci or variable_lookup

### Should Have 📊
- 10%+ improvement on variable-heavy benchmarks
- Real AoC solutions show measurable speedup
- Code is cleaner and more maintainable

### Nice to Have 🎯
- 15%+ improvement overall
- Improvements visible across all benchmarks

---

## Rollback Plan

If issues arise:

### Quick Revert
```bash
git checkout HEAD -- lang/src/evaluator/environment.rs
cargo test --package santa-lang
```

### Validation
```bash
# After revert, confirm baseline restored
make bench/run
# Should match original: fibonacci=639ms
```

### Risk Mitigation
- Changes isolated to single file (`environment.rs`)
- No API changes visible outside module
- All existing code using environment unchanged
- HashMap is well-tested Rust standard library type

---

## Integration Notes

### Dependencies
- **None**: This change is independent and can be done in parallel

### Conflicts
- May conflict with code that pattern-matches `Environment` struct
- Search for: `Environment { store, sections, outer }` patterns
- Update to: `Environment { store, sections, outer }` (no change needed, works with both)

### After This Change
- Object Pool optimization will benefit (fewer lookups during allocation)
- Collection operations will be faster (variables accessed in builtins)
- Function calls slightly faster (parameter binding uses environment)

---

## Timeline

- **Day 1 Morning**: Implement HashMap changes
- **Day 1 Afternoon**: Run tests, fix any issues
- **Day 2 Morning**: Run benchmarks, validate improvements
- **Day 2 Afternoon**: Test on real AoC solutions, document results

**Total**: 1-2 days

---

## References

- Current implementation: `lang/src/evaluator/environment.rs`
- Used by: `lang/src/evaluator/mod.rs` (eval_expression, eval_statement)
- Tests: `lang/src/evaluator/tests/` (all tests use environments)
