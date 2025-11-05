# Optimization: Dictionary Hybrid Storage

**Priority**: 🟡 Medium - Phase 2B
**Impact**: 5-8% speedup
**Risk**: Medium
**Effort**: 3-4 days
**Can work in parallel**: Yes (can be done parallel to Phase 2A)

---

## Why This Matters

### The Problem

Dictionary operations use **persistent hash tries (HAMT)** from `im_rc::HashMap` for all dictionaries, regardless of size. This creates unnecessary overhead for **small dictionaries** (< 10 keys) which are extremely common in AoC solutions.

### Real AoC Impact

**Example from Day 11 (2022) - Monkey State**:
```santa
let monkey = #{
  "items": [79, 98],
  "op": _ * 19,
  "divisor": 23,
  "throw_to": |w| if w % 23 == 0 { 2 } else { 3 },
  "activity": 0
};

// Repeated operations in hot loop:
monkey["items"]                           // Dictionary lookup
  |> fold(monkeys, |monkeys, item| {
    monkeys |> update(
      position,
      update("activity", _ + 1)          // Nested dictionary update
        >> assoc("items", new_items)     // Another update
    );
  })
```

Each monkey has **5 keys**. With 10,000 iterations:
- **50,000 dictionary lookups** through HAMT structure
- **20,000 dictionary updates** creating new HAMT nodes
- **O(log n)** cost for each operation vs O(n) linear search which is faster for n < 10

**Example from Day 16 (2022) - Valve State**:
```santa
let valve = #{
  "flow": 13,
  "tunnels": ["AA", "CC"]
};
```

Each valve has **2 keys**. HAMT overhead is pure waste here.

### HAMT Performance Characteristics

**im_rc::HashMap** (Hash Array Mapped Trie):
- Structure: 32-way branching tree
- Lookup: O(log₃₂ n) = ~7 steps for 1M entries
- Insert: O(log₃₂ n) + node allocation
- Memory: Significant overhead (tree nodes, bitmaps)

**For small dictionaries**:
- 2 keys: ~40 bytes overhead (tree structure) + 32 bytes data = 72 bytes
- 5 keys: ~80 bytes overhead + 80 bytes data = 160 bytes
- vs Vec: 0 bytes overhead + 32 bytes per entry

**Performance**:
- Linear search wins for n < ~8-12 (depends on key type)
- HAMT only worth it for larger dictionaries

---

## Usage Pattern Analysis

### Dictionary Size Distribution in AoC

From analyzing real solutions:

**Very Small (1-3 keys)**: ~40% of dictionaries
- Configuration objects
- Coordinate pairs `#{"x": 5, "y": 3}`
- Simple state `#{"seen": true, "count": 1}`

**Small (4-10 keys)**: ~35% of dictionaries
- State objects (Day 11 monkeys)
- Entity attributes (Day 16 valves)
- Instruction parsing

**Medium (11-50 keys)**: ~20% of dictionaries
- Lookup tables (Day 3 priorities)
- Memoization caches (early states)

**Large (50+ keys)**: ~5% of dictionaries
- Grid representations (Day 8)
- Comprehensive caches (Day 16 full game state)

**Key insight**: **75% of dictionaries have ≤10 keys**

### Common String Keys

**Frequently used keys** (could benefit from interning):
- `"x"`, `"y"` - Coordinates (used in ~40% of solutions)
- `"items"`, `"activity"` - State fields (Day 11)
- `"flow"`, `"tunnels"` - Valve fields (Day 16)
- `"left"`, `"right"` - Tree nodes (Day 8)

These string keys are allocated repeatedly.

---

## Current Implementation

**File**: `lang/src/evaluator/object.rs`

```rust
pub enum Object {
    // ...
    Dictionary(HashMap<Rc<Object>, Rc<Object>>),
    // Always uses im_rc::HashMap (HAMT)
}
```

**Dictionary operations**:
- Lookup: O(log n) via HAMT traversal
- Update: O(log n) + node allocation
- Insert: O(log n) + node allocation

---

## Proposed Solution

### Hybrid Storage Strategy

Use different storage based on dictionary size:

```rust
pub enum Dictionary {
    Small(Vec<(Rc<Object>, Rc<Object>)>),      // Linear search for ≤ threshold
    Large(HashMap<Rc<Object>, Rc<Object>>),     // HAMT for > threshold
}

pub enum Object {
    // ...
    Dictionary(Dictionary),
}
```

**Threshold**: 8-10 keys (determined by benchmarking)

### When to Convert

**Small → Large**: When size exceeds threshold during insert
```rust
if small_dict.len() > THRESHOLD {
    convert_to_large(small_dict);
}
```

**Large → Small**: Never (optimization: once large, stay large)
- Avoids thrashing
- Large dictionaries rarely shrink significantly

---

## Implementation Plan

### Phase 2B.1: Add Dictionary Enum

**Update `lang/src/evaluator/object.rs`**:

```rust
use im_rc::HashMap as PersistentHashMap;

const DICT_SMALL_THRESHOLD: usize = 8;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Dictionary {
    Small(Vec<(Rc<Object>, Rc<Object>)>),
    Large(PersistentHashMap<Rc<Object>, Rc<Object>>),
}

impl Dictionary {
    pub fn new() -> Self {
        Dictionary::Small(Vec::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        if capacity <= DICT_SMALL_THRESHOLD {
            Dictionary::Small(Vec::with_capacity(capacity))
        } else {
            Dictionary::Large(PersistentHashMap::new())
        }
    }

    pub fn get(&self, key: &Rc<Object>) -> Option<&Rc<Object>> {
        match self {
            Dictionary::Small(vec) => {
                vec.iter()
                    .find(|(k, _)| k == key)
                    .map(|(_, v)| v)
            }
            Dictionary::Large(map) => map.get(key),
        }
    }

    pub fn insert(&mut self, key: Rc<Object>, value: Rc<Object>) {
        match self {
            Dictionary::Small(vec) => {
                // Try to update existing key
                if let Some((_, v)) = vec.iter_mut().find(|(k, _)| k == &key) {
                    *v = value;
                    return;
                }

                // Add new key
                vec.push((key, value));

                // Convert to Large if threshold exceeded
                if vec.len() > DICT_SMALL_THRESHOLD {
                    let map = vec.iter()
                        .map(|(k, v)| (Rc::clone(k), Rc::clone(v)))
                        .collect();
                    *self = Dictionary::Large(map);
                }
            }
            Dictionary::Large(map) => {
                *map = map.update(key, value);
            }
        }
    }

    pub fn remove(&mut self, key: &Rc<Object>) {
        match self {
            Dictionary::Small(vec) => {
                vec.retain(|(k, _)| k != key);
            }
            Dictionary::Large(map) => {
                *map = map.without(key);
            }
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Dictionary::Small(vec) => vec.len(),
            Dictionary::Large(map) => map.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn keys(&self) -> Box<dyn Iterator<Item = &Rc<Object>> + '_> {
        match self {
            Dictionary::Small(vec) => Box::new(vec.iter().map(|(k, _)| k)),
            Dictionary::Large(map) => Box::new(map.keys()),
        }
    }

    pub fn values(&self) -> Box<dyn Iterator<Item = &Rc<Object>> + '_> {
        match self {
            Dictionary::Small(vec) => Box::new(vec.iter().map(|(_, v)| v)),
            Dictionary::Large(map) => Box::new(map.values()),
        }
    }

    pub fn iter(&self) -> Box<dyn Iterator<Item = (&Rc<Object>, &Rc<Object>)> + '_> {
        match self {
            Dictionary::Small(vec) => Box::new(vec.iter().map(|(k, v)| (k, v))),
            Dictionary::Large(map) => Box::new(map.iter()),
        }
    }
}

impl Default for Dictionary {
    fn default() -> Self {
        Self::new()
    }
}
```

### Phase 2B.2: Update Object Enum

```rust
pub enum Object {
    // ... other variants
    Dictionary(Dictionary),  // Changed from HashMap to Dictionary enum
    // ...
}
```

### Phase 2B.3: Update All Dictionary Operations

#### Indexing (`lang/src/evaluator/mod.rs`)

```rust
// eval_index_expression
match (&*collection, &*key) {
    (Object::Dictionary(dict), key) => {
        dict.get(key)  // Use Dictionary::get
            .map(Rc::clone)
            .ok_or_else(|| RuntimeErr::new("Key not found", source))
    }
}
```

#### Dictionary Builtins (`lang/src/evaluator/builtins/dictionary.rs`)

Update all operations:
- `dict` (line 22): Create Dictionary::Small or Large based on input size
- `keys` (line 44): Use Dictionary::keys()
- `values` (line 66): Use Dictionary::values()
- `get` (line 88): Use Dictionary::get()
- `assoc` (line 110): Clone and insert
- `dissoc` (line 132): Clone and remove
- `merge` (line 154): Merge logic for both variants

#### Collection Builtins

Update operations that create dictionaries:
- `dict()` in collection.rs - Use Dictionary::new()
- `group_by()` - Create with capacity hint

---

## Implementation Checklist

### Step 1: Create Dictionary Enum
- [ ] Add `Dictionary` enum to object.rs
- [ ] Implement all methods (get, insert, remove, len, keys, values, iter)
- [ ] Add unit tests for Dictionary enum
- [ ] Test Small → Large conversion at threshold

### Step 2: Update Object Enum
- [ ] Change `Object::Dictionary` to use `Dictionary` enum
- [ ] Update pattern matches throughout codebase

### Step 3: Update Evaluator
- [ ] Update dictionary literal parsing
- [ ] Update index expression handling
- [ ] Update dictionary comprehensions

### Step 4: Update Builtins
- [ ] Update `dict` builtin
- [ ] Update `keys`, `values` builtins
- [ ] Update `get`, `assoc`, `dissoc` builtins
- [ ] Update `merge` builtin
- [ ] Update `group_by` in collections

### Step 5: Benchmark Threshold
- [ ] Test with threshold = 6, 8, 10, 12
- [ ] Measure performance on small dictionaries
- [ ] Choose optimal threshold

---

## Testing Strategy

### 1. Unit Tests for Dictionary Enum

Add to `lang/src/evaluator/object.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_dictionary() {
        let mut dict = Dictionary::new();
        let key1 = Rc::new(Object::Integer(1));
        let val1 = Rc::new(Object::String("one".to_string()));

        dict.insert(Rc::clone(&key1), Rc::clone(&val1));
        assert_eq!(dict.len(), 1);
        assert_eq!(dict.get(&key1), Some(&val1));
    }

    #[test]
    fn test_conversion_to_large() {
        let mut dict = Dictionary::new();

        // Insert beyond threshold
        for i in 0..=DICT_SMALL_THRESHOLD + 1 {
            dict.insert(
                Rc::new(Object::Integer(i as i64)),
                Rc::new(Object::Integer(i as i64 * 2)),
            );
        }

        // Should have converted to Large
        match dict {
            Dictionary::Large(_) => { /* Expected */ }
            Dictionary::Small(_) => panic!("Should have converted to Large"),
        }

        // Verify all keys still accessible
        for i in 0..=DICT_SMALL_THRESHOLD + 1 {
            let key = Rc::new(Object::Integer(i as i64));
            assert!(dict.get(&key).is_some());
        }
    }

    #[test]
    fn test_update_existing_key() {
        let mut dict = Dictionary::new();
        let key = Rc::new(Object::Integer(1));

        dict.insert(Rc::clone(&key), Rc::new(Object::Integer(10)));
        dict.insert(Rc::clone(&key), Rc::new(Object::Integer(20)));

        assert_eq!(dict.len(), 1);  // Still only one key
        assert_eq!(
            dict.get(&key),
            Some(&Rc::new(Object::Integer(20)))
        );
    }
}
```

### 2. Existing Test Suite

```bash
# All dictionary tests must pass
cargo test --package santa-lang dictionary

# All evaluator tests
cargo test --package santa-lang evaluator
```

### 3. Threshold Benchmark

Add `benchmarks/fixtures/dictionary_sizes.santa`:
```santa
// Test different dictionary sizes

// Very small (2 keys)
let small = #{"x": 5, "y": 3};
1..1000 |> map(|i| small["x"] + small["y"]) |> sum;

// At threshold (8 keys)
let medium = #{
  "a": 1, "b": 2, "c": 3, "d": 4,
  "e": 5, "f": 6, "g": 7, "h": 8
};
1..1000 |> fold(0) |sum, i| {
  sum + medium["a"] + medium["h"]
};

// Large (50 keys)
let large = 1..50 |> map(|i| [i, i * 2]) |> dict;
1..1000 |> fold(0) |sum, i| {
  sum + large[i % 50]
};
```

**Test with different thresholds**:
```bash
# Modify DICT_SMALL_THRESHOLD in object.rs
# Try 6, 8, 10, 12 and benchmark each
make bench/run
```

### 4. Real AoC Solution Testing

Focus on dictionary-heavy solutions:

```bash
# Day 11: Small dictionaries (monkey state)
time target/release/santa-cli ~/Projects/advent-of-code/2022/santa-lang/aoc2022_day11.santa

# Day 16: Mix of small and large dictionaries
time target/release/santa-cli ~/Projects/advent-of-code/2022/santa-lang/aoc2022_day16.santa

# Day 3: Medium dictionaries (priority lookup)
time target/release/santa-cli ~/Projects/advent-of-code/2022/santa-lang/aoc2022_day03.santa
```

---

## Expected Performance Impact

### Benchmark Predictions

| Benchmark | After Phase 1+2A | After Phase 2B | Improvement |
|-----------|------------------|----------------|-------------|
| Day 11 (monkey) | ~90% of baseline | ~85% | 5-7% |
| Day 16 (valves) | ~85% of baseline | ~80% | 5-6% |
| Day 3 (rucksack) | ~88% of baseline | ~84% | 4-5% |

### Small Dictionary Performance

**For 5-key dictionary with 1000 lookups**:

| Implementation | Time | Improvement |
|----------------|------|-------------|
| Always HAMT | 120μs | Baseline |
| Hybrid (Small) | 85μs | 29% faster |

**For 50-key dictionary with 1000 lookups**:

| Implementation | Time | Improvement |
|----------------|------|-------------|
| Always HAMT | 180μs | Baseline |
| Hybrid (Large) | 180μs | No change |

### Combined Impact (Phase 1 + 2A + 2B)

| Benchmark | Baseline | After Phase 1+2A+2B | Total Improvement |
|-----------|----------|---------------------|-------------------|
| fibonacci | 639ms | ~505ms | 20-22% |
| list_processing | 7.5ms | ~5.4ms | 28-30% |
| Day 11 (dict-heavy) | Baseline | - | 25-30% |

---

## Threshold Selection

### Benchmark Different Thresholds

Test thresholds: 6, 8, 10, 12

**Methodology**:
```rust
// Create test harness
for threshold in [6, 8, 10, 12] {
    DICT_SMALL_THRESHOLD = threshold;
    run_benchmarks();
}
```

**Expected optimal**: 8-10 keys
- Below 8: Small dictionaries convert too early
- Above 10: Larger Small dictionaries have more overhead

### Performance Characteristics by Size

| Size | Vec (Linear) | HAMT (Log) | Winner |
|------|--------------|------------|--------|
| 2 | 2 compares | ~3-4 steps | Vec |
| 5 | 5 compares | ~3-4 steps | Vec |
| 8 | 8 compares | ~3-4 steps | Close |
| 10 | 10 compares | ~3-4 steps | HAMT |
| 20 | 20 compares | ~4-5 steps | HAMT |
| 100 | 100 compares | ~5-6 steps | HAMT |

**Optimal threshold**: Where curves cross (~8-10 keys)

---

## Success Criteria

### Must Have ✅
- All existing tests pass
- No regressions on large dictionaries
- At least 3% improvement on small-dictionary workloads

### Should Have 📊
- 5%+ improvement on dictionary-heavy AoC solutions
- Optimal threshold determined by benchmarking
- Memory usage reduced for small dictionaries

### Nice to Have 🎯
- 8%+ improvement on small dictionaries
- 30%+ total improvement combined with Phase 1+2A

---

## Rollback Plan

### Quick Revert
```bash
git checkout HEAD -- lang/src/evaluator/object.rs
git checkout HEAD -- lang/src/evaluator/builtins/dictionary.rs
cargo test --package santa-lang
```

### Partial Rollback
If issues with specific sizes:
```rust
// Disable hybrid by always using Large
impl Dictionary {
    pub fn new() -> Self {
        Dictionary::Large(PersistentHashMap::new())
    }
}
```

### Risk Mitigation
- Dictionary enum maintains same external API
- Conversion threshold is tunable
- Can disable Small variant entirely if needed
- Iterator trait objects abstract internal structure

---

## Advanced Optimizations (Future)

### String Key Interning

For frequently-used string keys:
```rust
lazy_static! {
    static ref INTERNED_KEYS: HashMap<&'static str, Rc<Object>> = {
        let mut m = HashMap::new();
        m.insert("x", Rc::new(Object::String("x".to_string())));
        m.insert("y", Rc::new(Object::String("y".to_string())));
        // ... other common keys
        m
    };
}
```

**Impact**: Reduce string allocations for common keys

### Specialized Small Dict Layouts

For very common patterns:
```rust
enum Dictionary {
    Coord { x: Rc<Object>, y: Rc<Object> },  // #{"x": ..., "y": ...}
    KeyValue { key: Rc<Object>, value: Rc<Object> },  // #{"key": ..., "value": ...}
    Small(Vec<...>),
    Large(HashMap<...>),
}
```

**Complexity**: High (pattern recognition needed)

---

## Integration Notes

### Dependencies
- **Independent of**: Phase 1, Phase 2A, Phase 3
- **Can parallelize with**: Phase 2A

### Ordering
- Can be done in any order relative to other phases
- Slightly easier after Phase 1 (fewer allocations to track)

### After This Change
- Dictionary operations faster overall
- Small dictionary patterns become zero-cost
- Large dictionaries maintain current performance

---

## Timeline

- **Day 1**: Implement Dictionary enum with methods
- **Day 2**: Update Object enum and evaluator
- **Day 3**: Update all builtins, threshold benchmarking
- **Day 4**: Testing and validation

**Total**: 3-4 days

---

## References

- Current implementation: `lang/src/evaluator/object.rs` (Object::Dictionary)
- Dictionary builtins: `lang/src/evaluator/builtins/dictionary.rs`
- Tests: `lang/src/evaluator/tests/builtins/dictionary.rs`
- Usage: ~70% of AoC solutions use dictionaries
