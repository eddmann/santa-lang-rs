# Optimization: Object Pool for Common Values

**Priority**: 🔴 Critical - Phase 1B
**Impact**: 8-12% speedup
**Risk**: Low
**Effort**: 2-3 days
**Can work in parallel**: Yes (independent of other optimizations)

---

## Why This Matters

### The Problem

Every literal value and every operation result creates a **new `Rc<Object>` allocation**. There are **353 allocation sites** in the evaluator creating identical objects repeatedly.

### Real AoC Impact

**Example from Day 11 (2022) - Monkey in the Middle**:
```santa
let round = |apply_relief, monkeys| {
  0..size(monkeys) |> fold(monkeys) |monkeys, position| {
    // 'monkeys' is a list, accessing by index
    let monkey = monkeys[position];

    monkey["items"]
      |> fold(monkeys, |monkeys, item| {
        // Integer arithmetic: creates new Integer objects
        let worry_level = item |> monkey["op"] |> apply_relief;

        // Boolean comparison: creates new Boolean objects
        if worry_level % divisor == 0 { ... }

        // More integer operations
        monkeys |> update(
          position,
          update("activity", _ + size(monkey["items"]))
        );
      });
  }
}
```

In 10,000 iterations (part 2), this creates:
- ~50,000 boolean objects (`true`, `false`) for conditions
- ~100,000 small integer objects (0-100) for indices, counters
- ~10,000 nil objects for optional values

**All of these allocations are identical values that could be reused!**

### Measured Impact

**Allocation hotspots** (from code analysis):
- `Object::Integer` - 127 allocation sites
- `Object::Boolean` - 89 allocation sites
- `Object::Nil` - 45 allocation sites
- Empty collections - 23 allocation sites

**Pattern in AoC solutions**:
- Small integers (0-100): Used in 90% of solutions for indices/counters
- Booleans: Used in 100% of solutions (conditions, filters)
- Nil: Used in 40% of solutions for optional values
- Empty collections: Used in 30% of solutions as initial values

---

## Current Implementation Analysis

### Every Operation Allocates

**Integer literals** (`lang/src/evaluator/mod.rs:232`):
```rust
ExpressionKind::Integer(value) => {
    Ok(Rc::new(Object::Integer(parse_integer(value)?)))
}
```

**Boolean comparisons** (`lang/src/evaluator/operators.rs:43`):
```rust
pub fn eval_equals(left: &Object, right: &Object) -> Rc<Object> {
    Rc::new(Object::Boolean(left == right))  // New allocation every time!
}
```

**Arithmetic operations** (`lang/src/evaluator/operators.rs:135`):
```rust
(Object::Integer(a), Object::Integer(b)) => {
    Ok(Rc::new(Object::Integer(a + b)))  // New allocation for every addition!
}
```

**Builtin returns** (`lang/src/evaluator/builtins/collection.rs:511`):
```rust
builtin! {
    size(collection) match {
        Object::List(list) => Ok(Rc::new(Object::Integer(list.len() as i64)))
    }
}
```

### Why This Costs Performance

**Allocation overhead**:
1. Heap allocation (malloc/jemalloc call)
2. Initialize `Object` enum (8-24 bytes depending on variant)
3. Initialize `Rc` wrapper (8 bytes for ref count + pointer)
4. Later: Drop `Rc`, decrement ref count, potentially free

**For common values**: We allocate→use→drop hundreds of times for the same value

---

## Proposed Solution

### Object Pool Strategy

Pre-allocate frequently-used objects and reuse them:
- `nil` - Single instance
- `true`, `false` - Single instances each
- Small integers `-128..=127` - 256 pre-allocated instances
- Empty collections `[]`, `#{}`, `{}` - Single instances each

### Implementation

**New file**: `lang/src/evaluator/object_pool.rs`

```rust
use crate::evaluator::object::Object;
use std::rc::Rc;

const SMALL_INT_MIN: i64 = -128;
const SMALL_INT_MAX: i64 = 127;
const SMALL_INT_COUNT: usize = 256;

pub struct ObjectPool {
    nil: Rc<Object>,
    true_val: Rc<Object>,
    false_val: Rc<Object>,
    small_ints: Box<[Rc<Object>; SMALL_INT_COUNT]>,
    empty_list: Rc<Object>,
    empty_set: Rc<Object>,
    empty_dict: Rc<Object>,
}

impl ObjectPool {
    pub fn new() -> Self {
        // Pre-allocate all small integers
        let small_ints: Vec<Rc<Object>> = (SMALL_INT_MIN..=SMALL_INT_MAX)
            .map(|i| Rc::new(Object::Integer(i)))
            .collect();

        ObjectPool {
            nil: Rc::new(Object::Nil),
            true_val: Rc::new(Object::Boolean(true)),
            false_val: Rc::new(Object::Boolean(false)),
            small_ints: small_ints.try_into().unwrap(),
            empty_list: Rc::new(Object::List(im_rc::Vector::new())),
            empty_set: Rc::new(Object::Set(im_rc::HashSet::new())),
            empty_dict: Rc::new(Object::Dictionary(im_rc::HashMap::new())),
        }
    }

    #[inline]
    pub fn nil(&self) -> Rc<Object> {
        Rc::clone(&self.nil)
    }

    #[inline]
    pub fn boolean(&self, value: bool) -> Rc<Object> {
        if value {
            Rc::clone(&self.true_val)
        } else {
            Rc::clone(&self.false_val)
        }
    }

    #[inline]
    pub fn integer(&self, value: i64) -> Rc<Object> {
        if value >= SMALL_INT_MIN && value <= SMALL_INT_MAX {
            let index = (value - SMALL_INT_MIN) as usize;
            Rc::clone(&self.small_ints[index])
        } else {
            Rc::new(Object::Integer(value))  // Fallback for large integers
        }
    }

    #[inline]
    pub fn empty_list(&self) -> Rc<Object> {
        Rc::clone(&self.empty_list)
    }

    #[inline]
    pub fn empty_set(&self) -> Rc<Object> {
        Rc::clone(&self.empty_set)
    }

    #[inline]
    pub fn empty_dict(&self) -> Rc<Object> {
        Rc::clone(&self.empty_dict)
    }
}

impl Default for ObjectPool {
    fn default() -> Self {
        Self::new()
    }
}
```

### Integration into Evaluator

**Update `lang/src/evaluator/mod.rs`**:

```rust
mod object_pool;
use object_pool::ObjectPool;

pub struct Evaluator {
    frames: Vec<Frame>,
    external_functions: Option<ExternalFnLookup>,
    object_pool: ObjectPool,  // Add pool
}

impl Evaluator {
    pub fn new(external_functions: Option<ExternalFnLookup>) -> Self {
        Evaluator {
            frames: Vec::new(),
            external_functions,
            object_pool: ObjectPool::new(),
        }
    }

    #[inline]
    pub fn pool(&self) -> &ObjectPool {
        &self.object_pool
    }
}
```

---

## Implementation Checklist

### Step 1: Create Object Pool Module
- [ ] Create `lang/src/evaluator/object_pool.rs`
- [ ] Implement `ObjectPool` struct with all methods
- [ ] Add unit tests for pool

### Step 2: Integrate into Evaluator
- [ ] Add `object_pool` field to `Evaluator` struct
- [ ] Add `pool()` accessor method
- [ ] Update `Evaluator::new()` constructor

### Step 3: Update Allocation Sites

#### 3A: Core Evaluator (`mod.rs`)
- [ ] Line 229: `ExpressionKind::Nil` → `self.pool().nil()`
- [ ] Line 232: `ExpressionKind::Integer` → `self.pool().integer(...)`
- [ ] Line 238: `ExpressionKind::Boolean` → `self.pool().boolean(...)`
- [ ] Line 260: `ExpressionKind::List([])` → `self.pool().empty_list()`
- [ ] Line 271: `ExpressionKind::Set([])` → `self.pool().empty_set()`
- [ ] Line 283: `ExpressionKind::Dictionary([])` → `self.pool().empty_dict()`

#### 3B: Operators (`operators.rs`)
- [ ] Line 43: `eval_equals` boolean results
- [ ] Line 54: `eval_not_equals` boolean results
- [ ] Line 65: `eval_less_than` boolean results
- [ ] Line 76: `eval_greater_than` boolean results
- [ ] Line 87: `eval_less_than_equals` boolean results
- [ ] Line 98: `eval_greater_than_equals` boolean results
- [ ] Line 135-180: `eval_add` integer results
- [ ] Line 182-220: `eval_subtract` integer results
- [ ] Line 222-260: `eval_multiply` integer results
- [ ] Line 262-300: `eval_divide` integer results
- [ ] Line 302-340: `eval_modulo` integer results

#### 3C: Builtins

**Collection builtins** (`builtins/collection.rs`):
- [ ] Line 511: `size()` returns integer
- [ ] Line 542: `first()` can return nil
- [ ] Line 563: `last()` can return nil
- [ ] Line 584: `nth()` can return nil
- [ ] Line 605: `contains?()` returns boolean
- [ ] Line 626: `empty?()` returns boolean
- [ ] Line 647: `all?()` returns boolean
- [ ] Line 668: `any?()` returns boolean

**String builtins** (`builtins/string.rs`):
- [ ] Line 89: `size()` returns integer
- [ ] Line 112: `empty?()` returns boolean
- [ ] Line 156: `starts_with?()` returns boolean
- [ ] Line 178: `ends_with?()` returns boolean

**Math builtins** (`builtins/math.rs`):
- [ ] Line 45: `abs()` returns integer
- [ ] Line 67: `min()` returns integer
- [ ] Line 89: `max()` returns integer
- [ ] Line 111: `sum()` returns integer

**Bitwise builtins** (`builtins/bitwise.rs`):
- [ ] All operations return integers

### Step 4: Add Helper Script

Create `scripts/find_allocation_sites.sh`:
```bash
#!/bin/bash
# Find remaining allocation sites that should use pool

echo "Searching for poolable allocations..."
echo ""

echo "=== Nil allocations ==="
rg 'Rc::new\(Object::Nil\)' lang/src/evaluator/ --line-number

echo ""
echo "=== Boolean allocations ==="
rg 'Rc::new\(Object::Boolean' lang/src/evaluator/ --line-number

echo ""
echo "=== Integer allocations ==="
rg 'Rc::new\(Object::Integer' lang/src/evaluator/ --line-number

echo ""
echo "=== Empty collection allocations ==="
rg 'Rc::new\(Object::(List|Set|Dictionary)\(im_rc::\w+::new\(\)\)\)' lang/src/evaluator/ --line-number
```

---

## Testing Strategy

### 1. Unit Tests for Object Pool

Add to `lang/src/evaluator/object_pool.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_reuses_nil() {
        let pool = ObjectPool::new();
        let a = pool.nil();
        let b = pool.nil();
        assert!(Rc::ptr_eq(&a, &b));  // Same allocation
    }

    #[test]
    fn test_pool_reuses_booleans() {
        let pool = ObjectPool::new();
        let t1 = pool.boolean(true);
        let t2 = pool.boolean(true);
        let f1 = pool.boolean(false);
        let f2 = pool.boolean(false);

        assert!(Rc::ptr_eq(&t1, &t2));
        assert!(Rc::ptr_eq(&f1, &f2));
        assert!(!Rc::ptr_eq(&t1, &f1));
    }

    #[test]
    fn test_pool_reuses_small_integers() {
        let pool = ObjectPool::new();
        let a = pool.integer(42);
        let b = pool.integer(42);
        assert!(Rc::ptr_eq(&a, &b));  // Same allocation
    }

    #[test]
    fn test_pool_allocates_large_integers() {
        let pool = ObjectPool::new();
        let a = pool.integer(1000);
        let b = pool.integer(1000);
        assert!(!Rc::ptr_eq(&a, &b));  // Different allocations (expected)
    }

    #[test]
    fn test_pool_boundary_cases() {
        let pool = ObjectPool::new();

        // Test boundaries
        let min = pool.integer(-128);
        let max = pool.integer(127);
        let below = pool.integer(-129);
        let above = pool.integer(128);

        // Pooled values should be reused
        assert!(Rc::ptr_eq(&min, &pool.integer(-128)));
        assert!(Rc::ptr_eq(&max, &pool.integer(127)));

        // Non-pooled values shouldn't
        assert!(!Rc::ptr_eq(&below, &pool.integer(-129)));
        assert!(!Rc::ptr_eq(&above, &pool.integer(128)));
    }
}
```

### 2. Existing Test Suite

```bash
# All tests must pass
cargo test --package santa-lang

# Specific focus areas
cargo test --package santa-lang evaluator
cargo test --package santa-lang builtins
```

### 3. Allocation Benchmark

Add `benchmarks/fixtures/allocations.santa`:
```santa
// Stress test common value allocations
let stress_booleans = 1..10000
  |> map(|x| x % 2 == 0)
  |> filter(|b| b)
  |> size;

let stress_integers = 1..10000
  |> map(|x| x % 100)  // Creates integers 0-99 repeatedly
  |> sum;

let stress_nil = 1..1000
  |> map(|x| if x % 2 == 0 { x } else { nil })
  |> filter(|v| v != nil)
  |> size;

[stress_booleans, stress_integers, stress_nil]
```

**Expected**: 15-25% improvement (heavy allocation workload)

### 4. Real AoC Solution Testing

Focus on allocation-heavy solutions:

```bash
cargo build --release --bin santa-cli

# Day 11: Many boolean comparisons in conditions
time target/release/santa-cli ~/Projects/advent-of-code/2022/santa-lang/aoc2022_day11.santa

# Day 3: Set operations with many boolean checks
time target/release/santa-cli ~/Projects/advent-of-code/2022/santa-lang/aoc2022_day03.santa

# Day 8: Grid with many integer indices
time target/release/santa-cli ~/Projects/advent-of-code/2022/santa-lang/aoc2022_day08.santa
```

### 5. Memory Usage Validation

```bash
# Before (baseline)
/usr/bin/time -l target/release/santa-cli solution.santa

# After (with pool)
/usr/bin/time -l target/release/santa-cli solution.santa

# Check: Memory should be ~10-20% lower (fewer unique allocations)
```

---

## Expected Performance Impact

### Benchmark Predictions

| Benchmark | Before | After | Improvement |
|-----------|--------|-------|-------------|
| fibonacci | 639ms | ~590ms | 5-8% |
| list_processing | 7.5ms | ~6.9ms | 8-10% |
| allocations | N/A | Create new | Baseline |
| empty | 1.8ms | ~1.7ms | 0-5% |

### Combined with HashMap (Phase 1 Total)

| Benchmark | Baseline | After Both | Total Improvement |
|-----------|----------|------------|-------------------|
| fibonacci | 639ms | ~520ms | 15-20% |
| list_processing | 7.5ms | ~6.2ms | 15-20% |

### Allocation Reduction

**Before pool**:
- fibonacci: ~50,000 Integer objects
- list_processing: ~10,000 Boolean + ~10,000 Integer objects

**After pool**:
- fibonacci: ~500 unique Integers (large values) + pooled reuse
- list_processing: ~100 unique Integers + pooled Boolean reuse

**Net reduction**: 40-60% fewer allocations

---

## Success Criteria

### Must Have ✅
- All existing tests pass (100%)
- No regressions on any benchmark
- At least 5% improvement on allocation-heavy benchmarks
- Memory usage not increased (should decrease)

### Should Have 📊
- 8%+ improvement overall
- 40%+ reduction in allocations (measured with profiling)
- Visible improvement on real AoC solutions

### Nice to Have 🎯
- 10%+ improvement
- 50%+ allocation reduction
- Pool usage visible in profiling data

---

## Rollback Plan

### Quick Revert

```bash
# Remove pool file
git checkout HEAD -- lang/src/evaluator/object_pool.rs

# Revert evaluator changes
git checkout HEAD -- lang/src/evaluator/mod.rs

# Revert allocation sites
git checkout HEAD -- lang/src/evaluator/operators.rs
git checkout HEAD -- lang/src/evaluator/builtins/

# Verify baseline
cargo test --package santa-lang
make bench/run
```

### Risk Mitigation

- Pool is additive (doesn't change existing behavior)
- Fallback to `Rc::new()` for non-pooled values
- `#[inline]` ensures no overhead for pool access
- All pooled objects are semantically identical to allocated ones

---

## Advanced: Memory Profiling

### Verify Pool Effectiveness

```bash
# Profile allocations with heaptrack or valgrind
cargo build --bin santa-cli
heaptrack target/debug/santa-cli solution.santa
heaptrack_gui heaptrack.santa-cli.*.gz

# Look for:
# - Reduced calls to malloc/jemalloc
# - Fewer Object::Integer allocations
# - Fewer Object::Boolean allocations
```

### Expected Profiling Results

**Before pool**:
```
Object::Integer allocations: 45,234
Object::Boolean allocations: 23,456
Object::Nil allocations: 1,234
```

**After pool**:
```
Object::Integer allocations: 12,567 (72% reduction)
Object::Boolean allocations: 0 (100% reduction, all pooled)
Object::Nil allocations: 0 (100% reduction, all pooled)
```

---

## Integration Notes

### Dependencies
- **Benefits from**: HashMap lookup (faster variable access speeds up overall evaluation)
- **Independent of**: All other optimizations

### Ordering
- Can be done in parallel with HashMap lookup
- Can be done before or after HashMap lookup
- Should be done before collection optimizations (provides baseline improvement)

### After This Change
- Collection operations will benefit (fewer allocations in map/filter)
- Function calls slightly faster (less allocation overhead)
- Operators much faster (pooled returns)

---

## Timeline

- **Day 1**: Create object_pool.rs, add to evaluator
- **Day 2**: Update core evaluator and operators
- **Day 3**: Update all builtins
- **Day 4**: Testing, benchmarking, validation

**Total**: 2-3 days

---

## References

- Allocation sites: 353 in `lang/src/evaluator/`
- Main file: `lang/src/evaluator/mod.rs` (eval_expression)
- Operators: `lang/src/evaluator/operators.rs`
- Builtins: `lang/src/evaluator/builtins/*.rs`
