# Optimization: Function Call Overhead

**Priority**: 🟢 Lower - Phase 3
**Impact**: 3-5% speedup
**Risk**: High
**Effort**: 4-5 days
**Can work in parallel**: No (should be done after Phase 1+2)

---

## Why This Matters

### The Problem

Every function call has significant overhead:
1. **Environment cloning** - Creating new scope for every call
2. **Frame management** - Push/pop call stack for tracing
3. **Parameter binding** - Destructuring and assignment overhead
4. **Tail-call detection** - Loop checking for continuations

This affects every closure call in map/filter/fold and all recursive functions.

### Real AoC Impact

**Example from Day 16 (2022) - Pathfinding with Memoization**:
```santa
let pressure_gauge = |valves| {
  let recur = memoize |valve, time, open_valves| {
    distances[valve] |> fold(0) |max_pressure, distance, neighbor| {
      if open_valves `bit_and` valve_ids[neighbor] > 0 {
        return max_pressure;  // Early return in nested closure
      }

      let remaining_time = time - distance - 1;
      if remaining_time < 1 {
        return max_pressure;
      }

      max(
        max_pressure,
        released_pressure + recur(neighbor, remaining_time, open_valves `bit_or` valve_ids[neighbor])
      );
    }
  }
}
```

**Call overhead breakdown**:
- `pressure_gauge` closure: 1 call (outer)
- `recur` memoized closure: ~10,000 calls (recursive)
- `fold` closure: ~100,000 calls (inner loop)

**Per call overhead**:
1. Clone environment (Rc::clone) - ~5ns
2. Create new Environment - ~10ns
3. Bind parameters - ~20-50ns (depends on destructuring)
4. Push frame - ~5ns
5. Execute body
6. Check continuation - ~5ns
7. Pop frame - ~5ns

**Total overhead per call**: ~50-80ns
**For 110,000 calls**: ~5-9ms of pure overhead

### Why This is Phase 3 (Not Phase 1)

**Complexity**: Changes core interpreter machinery
**Risk**: High chance of breaking tail-call optimization, memoization, error traces
**Benefit**: Smaller than Phase 1+2 (only 3-5% vs 15-20%)

---

## Current Implementation Analysis

### Function Call Flow

**File**: `lang/src/evaluator/function.rs`, lines 53-106

```rust
pub fn apply(
    &self,
    evaluator: &mut Evaluator,
    arguments: Vec<Rc<Object>>,
    source: &Location,
) -> Result<Rc<Object>, RuntimeErr> {
    match self {
        Function::Closure { parameters, body, environment } => {
            // 1. Clone captured environment (Rc::clone)
            let enclosed_environment = Environment::from(Rc::clone(environment));

            // 2. Create new environment extending captured
            let call_environment = Rc::new(RefCell::new(enclosed_environment));

            // 3. Bind parameters (may involve destructuring)
            bind_parameters(call_environment.clone(), parameters, arguments, source)?;

            // 4. Push call frame (for stack traces)
            evaluator.push_frame(Frame::ClosureCall { source: source.clone() });

            // 5. Execute body
            let mut result = evaluator.eval_statement_in_environment(body, call_environment.clone())?;

            // 6. Tail-call optimization loop
            loop {
                if let Object::Function(Function::Continuation { arguments }) = &*result {
                    // Tail call detected, rebind and re-execute
                    bind_parameters(call_environment.clone(), parameters, arguments.clone(), source)?;
                    result = evaluator.eval_statement_in_environment(body, call_environment.clone())?;
                    continue;
                }
                break;
            }

            // 7. Pop frame
            evaluator.pop_frame();

            Ok(result)
        }
        // ... other function types
    }
}
```

### Overhead Sources

#### 1. Environment Creation (Lines 57-59)
```rust
let enclosed_environment = Environment::from(Rc::clone(environment));
let call_environment = Rc::new(RefCell::new(enclosed_environment));
```

**Cost**:
- `Rc::clone(environment)` - Increment ref count
- `Environment::from()` - Allocate new Environment struct
- `Rc::new(RefCell::new(...))` - Two allocations

**Impact**: Every call, even if environment unchanged

#### 2. Parameter Binding (Line 62)
```rust
bind_parameters(call_environment.clone(), parameters, arguments, source)?;
```

**File**: `lang/src/evaluator/function.rs`, lines 109-180

**Cost**:
- Pattern matching on parameter types
- Destructuring lists/tuples
- Multiple environment lookups and insertions
- Error checking

**Impact**: More complex for destructuring patterns

#### 3. Frame Management (Lines 65, 80)
```rust
evaluator.push_frame(Frame::ClosureCall { source: source.clone() });
// ... execute ...
evaluator.pop_frame();
```

**Cost**:
- Vec push/pop
- Location clone (includes file path, line, column)

**Impact**: Every call, even short-lived closures

#### 4. Tail-Call Detection Loop (Lines 70-78)
```rust
loop {
    if let Object::Function(Function::Continuation { arguments }) = &*result {
        // Re-bind parameters, re-execute
        bind_parameters(...)?;
        result = evaluator.eval_statement_in_environment(...)?;
        continue;
    }
    break;
}
```

**Cost**:
- Pattern match on result every time
- Loop overhead even when not tail-recursive

**Impact**: All calls, beneficial only for tail-recursive functions

---

## Proposed Solutions

### Solution 1: Environment Pooling (Medium Impact, Medium Risk)

Reuse environment objects instead of allocating new ones.

**Approach**:
```rust
pub struct EnvironmentPool {
    available: Vec<Rc<RefCell<Environment>>>,
}

impl EnvironmentPool {
    pub fn acquire(&mut self, outer: EnvironmentRef) -> Rc<RefCell<Environment>> {
        if let Some(mut env) = self.available.pop() {
            // Reuse existing environment
            env.borrow_mut().clear();
            env.borrow_mut().set_outer(Some(outer));
            env
        } else {
            // Allocate new if pool empty
            Rc::new(RefCell::new(Environment::from(outer)))
        }
    }

    pub fn release(&mut self, env: Rc<RefCell<Environment>>) {
        self.available.push(env);
    }
}
```

**Integration**:
```rust
// In Evaluator
pub struct Evaluator {
    frames: Vec<Frame>,
    external_functions: Option<ExternalFnLookup>,
    object_pool: ObjectPool,
    env_pool: EnvironmentPool,  // NEW
}

// In function.rs
let call_environment = evaluator.env_pool.acquire(Rc::clone(environment));
// ... execute ...
evaluator.env_pool.release(call_environment);
```

**Impact**: 5-10% reduction in allocation overhead

**Risk**: Must ensure environment is properly cleared between uses

### Solution 2: Fast Path for Simple Closures (High Impact, High Risk)

Detect closures that don't need full call machinery:
- No parameter destructuring
- No captures (or only read-only captures)
- Single expression body

**Detection**:
```rust
enum ClosureType {
    Simple {
        param_name: String,
        body_expr: Expression,
    },
    Complex {
        parameters: Vec<Pattern>,
        body: Statement,
        environment: EnvironmentRef,
    },
}
```

**Fast path execution**:
```rust
match closure_type {
    ClosureType::Simple { param_name, body_expr } => {
        // Skip environment creation, just bind single parameter
        let temp_env = create_temp_binding(param_name, argument);
        evaluator.eval_expression_with_temp(body_expr, temp_env)
    }
    ClosureType::Complex { ... } => {
        // Full call machinery
        // ... existing code ...
    }
}
```

**Impact**: 15-25% faster for simple closures (very common in map/filter)

**Risk**: Complex to implement correctly, may break edge cases

### Solution 3: Optimize Frame Stack (Low Impact, Low Risk)

Reduce frame overhead for short-lived calls.

**Approach**:
```rust
// Add flag to skip frame for certain calls
pub fn apply_no_trace(
    &self,
    evaluator: &mut Evaluator,
    arguments: Vec<Rc<Object>>,
    source: &Location,
) -> Result<Rc<Object>, RuntimeErr> {
    // Skip push_frame/pop_frame for builtins or known-safe closures
    match self {
        Function::Builtin { .. } => {
            // Execute without frame
        }
        _ => {
            self.apply(evaluator, arguments, source)
        }
    }
}
```

**Impact**: 2-3% reduction in overhead

**Risk**: Lose stack traces for errors (acceptable for builtins)

### Solution 4: Optimize Tail-Call Detection (Low Impact, Low Risk)

Skip continuation check when not needed.

**Approach**:
```rust
// Mark functions as tail-recursive during parsing/evaluation
pub struct Closure {
    parameters: Vec<Pattern>,
    body: Statement,
    environment: EnvironmentRef,
    is_tail_recursive: bool,  // NEW: set during definition
}

// In apply:
let result = evaluator.eval_statement_in_environment(body, call_environment.clone())?;

if self.is_tail_recursive {
    // Only check for continuation if function can be tail-recursive
    loop {
        if let Object::Function(Function::Continuation { arguments }) = &*result {
            // ... tail call handling ...
        }
        break;
    }
}
```

**Impact**: 1-2% reduction in overhead

**Risk**: Low (conservative: default to checking if unsure)

---

## Recommended Implementation Priority

### Phase 3A: Low-Risk Quick Wins (2-3 days)

1. **Optimize Frame Stack** (Solution 3)
   - Skip frames for builtin calls
   - Use lightweight frame for simple closures

2. **Optimize Tail-Call Detection** (Solution 4)
   - Skip check when function doesn't use recursion
   - Mark tail-recursive functions during definition

**Expected**: 2-3% improvement, low risk

### Phase 3B: Environment Pooling (2-3 days)

3. **Environment Pooling** (Solution 1)
   - Implement pool
   - Integrate into function calls
   - Extensive testing

**Expected**: +5-10% improvement, medium risk

### Phase 3C: Simple Closure Fast Path (Optional, 3-4 days)

4. **Fast Path for Simple Closures** (Solution 2)
   - Only if Phase 3A+3B show good results
   - Detect simple closures during parsing
   - Implement fast path

**Expected**: +15-25% improvement on map/filter, high risk

---

## Implementation Checklist

### Phase 3A: Quick Wins

#### Frame Optimization
- [ ] Add `apply_no_trace` method to Function
- [ ] Update builtin calls to skip frames
- [ ] Test error traces still work for user code

#### Tail-Call Optimization
- [ ] Add `is_tail_recursive` flag to Closure
- [ ] Detect tail recursion during function definition
- [ ] Skip continuation check when not recursive
- [ ] Test tail recursion still works

### Phase 3B: Environment Pooling

- [ ] Create EnvironmentPool struct
- [ ] Add pool to Evaluator
- [ ] Implement acquire/release
- [ ] Update function.rs to use pool
- [ ] Add clear() method to Environment
- [ ] Test environment isolation (no leakage between calls)
- [ ] Benchmark allocation reduction

### Phase 3C: Simple Closure Fast Path (Optional)

- [ ] Add ClosureType enum
- [ ] Detect simple closures during parsing
- [ ] Implement fast path execution
- [ ] Extensive testing for edge cases
- [ ] Benchmark impact on map/filter workloads

---

## Testing Strategy

### 1. Correctness Tests

**Critical areas**:
- [ ] Closure captures work correctly
- [ ] Parameter destructuring works
- [ ] Tail recursion still optimized
- [ ] Error stack traces show correct call sites
- [ ] Nested closures work
- [ ] Memoization still caches correctly

### 2. Specific Test Cases

Add to `lang/src/evaluator/tests/function.rs`:

```rust
#[test]
fn test_environment_isolation() {
    // Ensure pooled environments don't leak between calls
    let code = r#"
        let f = |x| {
            let y = x + 1;
            y
        };
        let a = f(10);
        let b = f(20);
        [a, b]
    "#;
    // Should be [11, 21], not [11, 22] if 'y' leaked
}

#[test]
fn test_tail_recursion_still_works() {
    let code = r#"
        let factorial = |n, acc| {
            if n == 0 { return acc; }
            factorial(n - 1, n * acc)
        };
        factorial(1000, 1)
    "#;
    // Should not stack overflow
}

#[test]
fn test_nested_closure_captures() {
    let code = r#"
        let outer = |x| {
            let inner = |y| x + y;
            inner(5)
        };
        outer(10)
    "#;
    // Should be 15
}
```

### 3. Benchmark: Function Call Overhead

Add `benchmarks/fixtures/function_calls.santa`:
```santa
// Stress test function call overhead

// Simple closure (candidate for fast path)
let simple = |x| x * 2;
let test_simple = 1..10000 |> map(simple) |> sum;

// Closure with capture
let multiplier = 3;
let with_capture = |x| x * multiplier;
let test_capture = 1..10000 |> map(with_capture) |> sum;

// Tail-recursive function
let sum_to = |n, acc| {
  if n == 0 { return acc; }
  sum_to(n - 1, acc + n)
};
let test_recursive = sum_to(1000, 0);

[test_simple, test_capture, test_recursive]
```

**Expected improvements**:
- Phase 3A: 2-3% faster
- Phase 3B: +5-10% faster
- Phase 3C: +15-25% faster (simple closures)

### 4. Real AoC Solutions

Focus on function-heavy solutions:

```bash
# Day 16: Heavy recursion with memoization
time target/release/santa-cli ~/Projects/advent-of-code/2022/santa-lang/aoc2022_day16.santa

# Day 11: Nested fold operations
time target/release/santa-cli ~/Projects/advent-of-code/2022/santa-lang/aoc2022_day11.santa

# Day 8: Nested map operations
time target/release/santa-cli ~/Projects/advent-of-code/2022/santa-lang/aoc2022_day08.santa
```

---

## Expected Performance Impact

### Phase 3A: Quick Wins

| Benchmark | After Phase 1+2 | After Phase 3A | Improvement |
|-----------|-----------------|----------------|-------------|
| fibonacci | ~505ms | ~495ms | 2% |
| function_calls | N/A | Create new | Baseline |
| Day 16 | ~85% baseline | ~83% | 2-3% |

### Phase 3B: Environment Pooling

| Benchmark | After Phase 3A | After Phase 3B | Improvement |
|-----------|----------------|----------------|-------------|
| fibonacci | ~495ms | ~470ms | 5% |
| function_calls | Baseline | -7% | 7% |
| Day 16 | ~83% baseline | ~78% | 5-6% |

### Phase 3C: Simple Closure Fast Path (Optional)

| Benchmark | After Phase 3B | After Phase 3C | Improvement |
|-----------|----------------|----------------|-------------|
| list_processing | ~5.4ms | ~4.9ms | 8-10% |
| Day 8 (nested maps) | ~78% baseline | ~70% | 10-12% |

### Combined Impact (All Phases)

| Benchmark | Baseline | After Phase 1+2+3 | Total Improvement |
|-----------|----------|-------------------|-------------------|
| fibonacci | 639ms | ~470ms | 25-28% |
| list_processing | 7.5ms | ~4.9ms | 35-38% |
| Day 11 | Baseline | - | 30-35% |
| Day 16 | Baseline | - | 22-28% |

---

## Success Criteria

### Phase 3A Must Have ✅
- All existing tests pass
- Tail recursion still works
- Error traces still accurate
- At least 1% improvement

### Phase 3B Must Have ✅
- Environment isolation maintained
- No memory leaks in pool
- At least 3% additional improvement

### Phase 3C Must Have ✅
- Simple closures detected correctly
- All edge cases handled
- At least 5% improvement on map/filter

### Overall Phase 3 Should Have 📊
- 3-5% total improvement
- No regressions on any benchmark
- Combined Phase 1+2+3: 30-40% improvement

---

## Rollback Plan

### Quick Revert
```bash
# Revert specific phase
git checkout HEAD -- lang/src/evaluator/function.rs
git checkout HEAD -- lang/src/evaluator/mod.rs

# Verify baseline
cargo test --package santa-lang
make bench/run
```

### Partial Rollback
- Can disable environment pooling by not using pool
- Can disable fast path by always using complex path
- Can re-enable frames for all calls

### Risk Mitigation
- Implement in separate phases (can stop after 3A or 3B)
- Each phase has independent rollback
- Extensive testing before proceeding to next phase

---

## Why This is Phase 3

### Complexity
- **Phase 1**: Simple data structure changes (HashMap, Object pool)
- **Phase 2**: Collection operation updates (localized to builtins)
- **Phase 3**: Core interpreter machinery (affects everything)

### Risk
- **Phase 1**: Low (additive, fallback available)
- **Phase 2**: Medium (more code to update)
- **Phase 3**: High (can break tail calls, traces, closures)

### Benefit
- **Phase 1**: 15-20% improvement
- **Phase 2**: +8-15% improvement
- **Phase 3**: +3-5% improvement (smaller incremental gain)

### Dependencies
- Phase 3 benefits from Phase 1+2 being done first
- Profiling data from Phase 1+2 helps prioritize Phase 3 work
- Can make better decisions about fast paths with Phase 1+2 baseline

---

## Advanced Optimizations (Future)

### Inline Trivial Closures

For closures like `|x| x` (identity), skip call entirely:
```rust
// Detect during parsing
if is_identity_closure(mapper) {
    return collection;  // No-op map
}
```

### Specialize Common Patterns

Recognize patterns like `map(|x| x * 2)` and generate specialized code:
```rust
enum SpecializedOp {
    MapMultiply(i64),
    MapAdd(i64),
    FilterEquals(Rc<Object>),
}

// Execute with no function calls
match specialized_op {
    MapMultiply(n) => {
        for element in list {
            if let Object::Integer(i) = &**element {
                result.push(Object::Integer(i * n));
            }
        }
    }
}
```

### JIT Compilation

For hot functions (detected by profiling), compile to native code.

**Complexity**: Very high
**Benefit**: 5-10x speedup for numeric code

---

## Integration Notes

### Dependencies
- **Should be done after**: Phase 1, Phase 2
- **Independent of**: Other optimizations

### Ordering
- Phase 3A can be done anytime (low risk)
- Phase 3B should wait for Phase 1+2 data
- Phase 3C should wait for Phase 3A+3B results

### After This Change
- Function call overhead minimized
- Further speedups require bytecode compiler or JIT

---

## Timeline

### Phase 3A: Quick Wins
- **Day 1**: Frame optimization
- **Day 2**: Tail-call optimization
- **Day 3**: Testing

### Phase 3B: Environment Pooling
- **Day 4-5**: Implement pool
- **Day 6**: Integration and testing

### Phase 3C: Simple Closure Fast Path (Optional)
- **Day 7-8**: Implement detection and fast path
- **Day 9**: Extensive testing

**Total**: 4-5 days (3A+3B), up to 9 days (if including 3C)

---

## References

- Main file: `lang/src/evaluator/function.rs` (apply method, lines 53-106)
- Parameter binding: `lang/src/evaluator/function.rs` (lines 109-180)
- Frame management: `lang/src/evaluator/mod.rs` (push_frame, pop_frame)
- Tail-call detection: `lang/src/evaluator/continuation.rs`
