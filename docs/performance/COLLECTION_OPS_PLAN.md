# Optimization: Collection Operation Overhead

**Priority**: 🟡 Medium - Phase 2A
**Impact**: 5-10% speedup
**Risk**: Medium
**Effort**: 3-4 days
**Can work in parallel**: Yes (but benefits from Phase 1 being done first)

---

## Why This Matters

### The Problem

Collection operations (`map`, `filter`, `fold`, `zip`) are the **most frequently used operations** in AoC solutions (~90% usage). Current implementation has three performance issues:

1. **Persistent data structure overhead** in hot loops
2. **Function call overhead** for every element
3. **Excessive Rc cloning** during iteration

### Real AoC Impact

**Example from Day 8 (2022) - Tree Grid**:
```santa
let parse_grid = |input| {
  zip(0.., lines(input))                      // Create lazy zip
    |> flat_map(|[y, row]| {                 // Function call per line
      zip(0.., row)                           // Nested zip
        |> map(|[x, tree]| [[y, x], int(tree)]); // Function call per char
    })
    |> dict;                                  // Collect to persistent HashMap
}
```

For a 99x99 grid:
- **10,000 closure calls** (flat_map + map)
- **10,000 Vector push_back operations** (persistent structure)
- **20,000 Rc clones** (y, x, tree values passed to closures)

**Example from Day 3 (2022) - Rucksack**:
```santa
parse_groups(input)
  |> map(compartments >> common_item >> get(_, priorities))
  |> sum;
```

With 300 lines of input:
- **300 closure calls** for outer map
- **900 closure calls** total (nested operations)
- **600+ Rc clones** for values

### Current Performance Characteristics

From `lang/src/evaluator/builtins/collection.rs`:

**map implementation** (line 44-86):
```rust
builtin! {
    map(mapper, collection) [evaluator, source] match {
        (Object::Function(mapper), Object::List(list)) => {
            let mut elements = Vector::new();  // Persistent structure
            for element in list {
                // Function call overhead + Rc clone
                elements.push_back(
                    mapper.apply(evaluator, vec![Rc::clone(element)], source)?
                );
            }
            Ok(Rc::new(Object::List(elements)))
        }
    }
}
```

**Problems**:
1. `Vector::new()` is `im_rc::Vector` (RRB-tree, not plain Vec)
2. `push_back()` creates new tree nodes (structural sharing overhead)
3. `mapper.apply()` has full function call overhead
4. `Rc::clone(element)` for every element

---

## Current Implementation Analysis

### Persistent Data Structure Overhead

**im_rc::Vector** (used for all Lists):
- Structure: RRB-tree (Relaxed Radix Balanced Tree)
- `push_back`: O(1)* amortized but creates tree nodes
- Memory: More overhead than `Vec` (tree structure)
- Benefit: O(1) clone, structural sharing
- Cost: Every mutation creates nodes even when building

**When building collections**:
```rust
let mut elements = Vector::new();  // RRB-tree
for item in items {
    elements.push_back(result);    // Creates tree nodes
}
```

vs **what we could do**:
```rust
let mut elements = Vec::with_capacity(items.len());  // Plain array
for item in items {
    elements.push(result);         // Array append
}
Vector::from(elements)             // Convert once at end
```

### Function Call Overhead

**Every element goes through full function machinery**:

```rust
// In map implementation
mapper.apply(evaluator, vec![Rc::clone(element)], source)?

// apply() does:
// 1. Create new environment
// 2. Bind parameters
// 3. Push frame
// 4. Execute body
// 5. Pop frame
// 6. Check for continuation
```

For a 10,000 element list: **10,000 full function calls**

### Rc Cloning Overhead

Pattern throughout builtins:
```rust
for element in list {
    let result = f.apply(evaluator, vec![Rc::clone(element)], source)?;
    //                                    ^^^^^^^^^^^^^^^^^ Clone for every element
}
```

For 10,000 elements: **10,000 Rc clones + 10,000 Rc drops**

---

## Proposed Solutions

### Solution 1: Transient Collections (High Impact, Medium Risk)

Use mutable `Vec` for building, convert to persistent at end.

**Before**:
```rust
let mut elements = Vector::new();  // Persistent from start
for element in list {
    elements.push_back(result);    // Tree node creation
}
Ok(Rc::new(Object::List(elements)))
```

**After**:
```rust
let mut elements = Vec::with_capacity(list.len());  // Mutable builder
for element in list {
    elements.push(result);         // Plain array push
}
Ok(Rc::new(Object::List(Vector::from(elements))))  // Convert once
```

**Impact**: 10-20% faster collection building

### Solution 2: Inline Small Closures (High Impact, High Risk)

Detect single-expression closures and inline them.

**Pattern to detect**:
```santa
|x| x * 2              // Single expression, no closure capture
|x| x % 2 == 0        // Single expression
|[a, b]| a + b        // Single expression with destructuring
```

**Implementation**:
```rust
enum FastPath {
    None,
    SimpleExpression(Expression),  // Single expr, inline it
}

// In map:
match &mapper {
    Function::Closure { parameters, body, .. } if is_simple(body) => {
        // Fast path: evaluate expression directly without full apply()
        for element in list {
            let result = eval_with_binding(evaluator, parameters, element, body)?;
            elements.push(result);
        }
    }
    _ => {
        // Slow path: full function call
        for element in list {
            elements.push(mapper.apply(...)?);
        }
    }
}
```

**Impact**: 15-25% faster for simple closures (very common in AoC)

### Solution 3: Reduce Rc Cloning (Medium Impact, Low Risk)

Pass elements by reference when possible.

**Current**:
```rust
mapper.apply(evaluator, vec![Rc::clone(element)], source)?
```

**Optimized**:
```rust
// For closures that don't capture the value long-term:
mapper.apply(evaluator, vec![element.as_ref()], source)?
```

**Limitation**: Requires tracking if closure captures value

**Alternative**: Pre-allocate argument vector
```rust
// Outside loop:
let mut args = Vec::with_capacity(1);

// Inside loop:
args.clear();
args.push(Rc::clone(element));
mapper.apply(evaluator, args, source)?;
```

**Impact**: 5-10% reduction in cloning overhead

---

## Implementation Plan

### Phase 2A.1: Transient Collections

#### Files to Modify
- `lang/src/evaluator/builtins/collection.rs`

#### Operations to Update

**map** (line 44-86):
```rust
builtin! {
    map(mapper, collection) [evaluator, source] match {
        (Object::Function(mapper), Object::List(list)) => {
            let mut elements = Vec::with_capacity(list.len());  // NEW
            for element in list {
                elements.push(mapper.apply(evaluator, vec![Rc::clone(element)], source)?);
            }
            Ok(Rc::new(Object::List(Vector::from(elements))))  // Convert once
        }
    }
}
```

**filter** (line 88-131):
```rust
builtin! {
    filter(predicate, collection) [evaluator, source] match {
        (Object::Function(predicate), Object::List(list)) => {
            // Estimate capacity (assume ~50% pass filter)
            let mut elements = Vec::with_capacity(list.len() / 2);
            for element in list {
                let result = predicate.apply(evaluator, vec![Rc::clone(element)], source)?;
                if is_truthy(&result) {
                    elements.push(Rc::clone(element));
                }
            }
            Ok(Rc::new(Object::List(Vector::from(elements))))
        }
    }
}
```

**Similar updates needed for**:
- `flat_map` (line 320-362)
- `chunk` (line 364-406)
- `zip` (line 835-896)
- `take` (line 543-585)
- `drop` (line 587-629)

### Phase 2A.2: Reusable Argument Buffers

Add to builtins that call functions frequently:

```rust
// Helper for reducing cloning
fn map_with_reused_args(
    mapper: &Function,
    list: &Vector<Rc<Object>>,
    evaluator: &mut Evaluator,
    source: &Location,
) -> Result<Vec<Rc<Object>>, RuntimeErr> {
    let mut elements = Vec::with_capacity(list.len());
    let mut args = Vec::with_capacity(1);  // Reuse buffer

    for element in list {
        args.clear();
        args.push(Rc::clone(element));
        elements.push(mapper.apply(evaluator, args.clone(), source)?);
    }

    Ok(elements)
}
```

### Phase 2A.3: Fast Path for Simple Closures (Optional, High Risk)

**Only if time permits and Phase 2A.1-2 show good results**

Add fast path detection:
```rust
fn is_simple_closure(func: &Function) -> Option<&Expression> {
    match func {
        Function::Closure { parameters, body, environment } if environment.borrow().store.is_empty() => {
            // No captures, check if body is single expression
            if let Statement::Expression(expr) = body {
                Some(expr)
            } else {
                None
            }
        }
        _ => None
    }
}
```

---

## Implementation Checklist

### Step 1: Convert to Transient Collections
- [ ] Update `map` to use `Vec::with_capacity` + `Vector::from`
- [ ] Update `filter` with capacity hint
- [ ] Update `flat_map`
- [ ] Update `chunk`
- [ ] Update `take`
- [ ] Update `drop`
- [ ] Update `zip`

### Step 2: Add Argument Buffer Reuse
- [ ] Create helper function for reusable args
- [ ] Apply to `map`
- [ ] Apply to `filter`
- [ ] Apply to `fold`
- [ ] Apply to `flat_map`

### Step 3: Benchmark and Validate
- [ ] Run tests
- [ ] Benchmark improvements
- [ ] Profile allocation reduction
- [ ] Validate on real AoC solutions

### Step 4: (Optional) Simple Closure Fast Path
- [ ] Only proceed if Step 1-2 show 5%+ improvement
- [ ] Implement detection
- [ ] Add fast path to `map`
- [ ] Extensive testing (high risk)

---

## Testing Strategy

### 1. Existing Test Suite
```bash
# All collection tests must pass
cargo test --package santa-lang builtins::collection

# All evaluator tests
cargo test --package santa-lang evaluator
```

### 2. New Collection Benchmark

Add `benchmarks/fixtures/collection_intensive.santa`:
```santa
// Stress test collection operations
let data = 1..10000;

let test_map = data |> map(|x| x * 2) |> sum;

let test_filter = data |> filter(|x| x % 2 == 0) |> sum;

let test_combined = data
  |> filter(|x| x % 3 == 0)
  |> map(|x| x * x)
  |> filter(|x| x < 100000)
  |> sum;

[test_map, test_filter, test_combined]
```

**Expected**: 10-15% improvement

### 3. Real AoC Solutions

Focus on collection-heavy problems:

```bash
# Day 1: Heavy use of map, sum
time target/release/santa-cli ~/Projects/advent-of-code/2022/santa-lang/aoc2022_day01.santa

# Day 8: Nested maps with flat_map
time target/release/santa-cli ~/Projects/advent-of-code/2022/santa-lang/aoc2022_day08.santa

# Day 3: Map, filter, chunk combinations
time target/release/santa-cli ~/Projects/advent-of-code/2022/santa-lang/aoc2022_day03.santa
```

### 4. Memory Profiling

```bash
# Check allocation reduction
heaptrack target/debug/santa-cli solution.santa

# Look for:
# - Fewer Vector allocations during building
# - Reduced tree node allocations
# - Same or fewer final Vector instances
```

---

## Expected Performance Impact

### Benchmark Predictions

| Benchmark | After Phase 1 | After Phase 2A | Improvement |
|-----------|---------------|----------------|-------------|
| list_processing | ~6.2ms | ~5.5ms | 8-12% |
| fibonacci | ~520ms | ~510ms | 2-3% |
| collection_intensive | N/A | Create new | Baseline |

### Combined Impact (Phase 1 + 2A)

| Benchmark | Baseline | After Phase 1+2A | Total Improvement |
|-----------|----------|------------------|-------------------|
| list_processing | 7.5ms | ~5.5ms | 25-30% |
| fibonacci | 639ms | ~510ms | 18-22% |

### Real AoC Solutions

- **Day 1** (heavy map/sum): 10-15% faster
- **Day 8** (nested maps): 15-20% faster
- **Day 3** (filter/map/chunk): 12-18% faster

---

## Success Criteria

### Must Have ✅
- All existing tests pass
- No regressions on any benchmark
- At least 5% improvement on collection-intensive workloads

### Should Have 📊
- 8%+ improvement on list_processing
- 10%+ improvement on real AoC collection-heavy solutions
- Measurable allocation reduction

### Nice to Have 🎯
- 12%+ improvement
- 30%+ fewer allocations during collection building
- Visible improvement across all collection operations

---

## Rollback Plan

### Quick Revert
```bash
git checkout HEAD -- lang/src/evaluator/builtins/collection.rs
cargo test --package santa-lang
```

### Validation After Revert
```bash
make bench/compare V1=HEAD V2=main
# Should show baseline performance restored
```

### Risk Mitigation
- Changes isolated to collection.rs
- Each operation updated independently
- Can revert individual operations if issues
- `Vector::from(Vec)` is well-tested conversion

---

## Why This is Phase 2 (Not Phase 1)

**Dependencies**:
- Benefits more after Phase 1 (HashMap + Object Pool reduce other overhead)
- Phase 1 improvements make collection overhead more visible

**Risk**:
- More complex than Phase 1 (affects many operations)
- Requires careful testing of each builtin
- Transient→persistent conversion must be correct

**Incremental Value**:
- Phase 1 already provides 15-20% speedup
- Phase 2 adds another 8-12% on top
- Can stop after Phase 1 if needed

---

## Advanced Optimizations (Future)

### Lazy Evaluation Improvements

Some operations could stay lazy longer:
```santa
data
  |> map(f)        // Lazy
  |> filter(g)     // Lazy (fusion with map)
  |> take(100)     // Lazy (early termination)
  |> list          // Materialize only here
```

**Current**: Each operation materializes fully
**Future**: Chain operations, materialize once

**Complexity**: High (requires redesigning collection builtins)

### SIMD for Simple Operations

For closures like `|x| x * 2`, could use SIMD:
```rust
// Detect pattern: numeric operation on elements
if is_simd_compatible(mapper) {
    return simd_map(list, mapper);  // Vectorized execution
}
```

**Complexity**: Very high (requires unsafe, platform-specific)

---

## Integration Notes

### Dependencies
- **Benefits from**: Phase 1 (HashMap + Object Pool)
- **Independent of**: Phase 2B (Dictionary hybrid), Phase 3

### Ordering
- Should be done after Phase 1
- Can be done in parallel with Phase 2B if different people
- Should be done before Phase 3 (provides baseline improvement)

### After This Change
- Function call overhead becomes more visible (candidate for Phase 3)
- Dictionary operations become more visible (candidate for Phase 2B)

---

## Timeline

- **Day 1**: Implement transient collections for map, filter
- **Day 2**: Update remaining operations (flat_map, chunk, zip)
- **Day 3**: Add argument buffer reuse
- **Day 4**: Testing, benchmarking, profiling
- **(Optional) Day 5**: Simple closure fast path (if time permits)

**Total**: 3-4 days

---

## References

- Main file: `lang/src/evaluator/builtins/collection.rs` (1779 lines)
- Persistent structures: `im_rc::Vector`, `im_rc::HashMap`
- Tests: `lang/src/evaluator/tests/builtins/collection.rs`
- Usage: ~90% of AoC solutions use map/filter/fold
