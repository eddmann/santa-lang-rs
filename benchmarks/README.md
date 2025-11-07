# santa-lang Benchmarks

Docker-based performance benchmarking system for santa-lang-rs.

## Overview

All benchmarks run in isolated Docker containers for reproducible, consistent results across machines. The benchmarking system includes:

1. **Criterion microbenchmarks** - Component-level performance testing
2. **Hyperfine end-to-end benchmarks** - CLI performance with statistical analysis
3. **Version comparison** - Compare performance across git commits/branches

## Quick Start

```bash
# Build benchmark Docker image
make bench/build

# Run benchmarks
make bench/run

# Compare two versions
make bench/compare V1=main V2=HEAD
```

## Available Make Targets

### `make bench/build`

Build the benchmark Docker image with all dependencies (Rust, hyperfine, Python tools).

### `make bench/run`

Run hyperfine benchmarks on all test fixtures. Results saved to `benchmarks/results/`.

**Output:**

- `benchmark_TIMESTAMP.json` - Raw benchmark data
- `benchmark_TIMESTAMP.md` - Formatted markdown report

### `make bench/compare V1=<version1> V2=<version2>`

Compare performance between two git versions in isolated Docker containers.

**Examples:**

```bash
# Compare main branch with current work
make bench/compare V1=main V2=HEAD

# Compare two releases
make bench/compare V1=v1.0.0 V2=v2.0.0

# Compare any two commits
make bench/compare V1=abc123 V2=def456
```

**Output:**

- Comparison table showing performance differences
- Per-fixture JSON results in `benchmarks/results/compare_TIMESTAMP/`

### `make bench/criterion`

Run Criterion microbenchmarks for detailed component analysis.

### `make bench/visualize RESULTS="file1.json file2.json"`

Generate visual charts from benchmark results.

**Examples:**

```bash
# Visualize single run
make bench/visualize RESULTS="benchmarks/results/benchmark_*.json"

# Compare multiple runs
make bench/visualize RESULTS="benchmarks/results/base.json benchmarks/results/pr.json"
```

**Output:**
Charts saved to `benchmarks/results/charts/`

### `make bench/shell`

Open interactive shell in benchmark Docker container for manual testing.

### `make bench/clean`

Remove all benchmark results.

## Test Fixtures

Benchmarks run against test programs in `benchmarks/fixtures/`:

### Synthetic Benchmarks
- **empty.santa** - Interpreter startup overhead
- **fibonacci.santa** - Recursive computation (fib 30)
- **list_processing.santa** - Pipeline operations on 10,000 elements
- **pattern_matching.santa** - Recursive pattern matching

### Real-World AoC Benchmarks

All AoC benchmarks run in test mode (`-t` flag) to validate correctness while measuring performance.

**39 Advent of Code solutions** from recent years:

- **aoc2022_day01-25.santa** - Complete 2022 set (25 solutions): BFS pathfinding, regex parsing, supply stacks, monkey math, grid processing
- **aoc2023_day01-14.santa** - Partial 2023 set (14 solutions): Camel Cards, cosmic expansion, grid traversal, combinations, pattern matching

Each AoC benchmark:
- Contains embedded test cases with expected outputs
- Runs both part_one and part_two solutions
- Validates correctness during benchmark execution
- Represents real-world santa-lang usage patterns

## Understanding Results

### Benchmark Output

```
| Benchmark | Base (ms) | Current (ms) | Change | Status |
|-----------|-----------|--------------|--------|--------|
| empty | 4.20 | 4.18 | -0.5% | ✓ no change |
| fibonacci | 45.60 | 43.20 | -5.3% | 🚀 improved |
```

**Status indicators:**

- ✓ **No change** - < 5% difference
- 🚀 **Improved** - > 5% faster
- ⚠️ **Regressed** - > 5% slower

### Criterion Output

```
fibonacci/recursive     time:   [45.234 ms 45.567 ms 45.923 ms]
                        change: [-2.3% -1.8% -1.2%] (p = 0.00 < 0.05)
                        Performance has improved.
```

## GitHub Actions Integration

The benchmark workflow automatically runs on PRs that modify language or CLI code:

**Features:**

- Builds both PR and base branch in Docker
- Runs identical benchmarks on both versions
- Posts comparison table as PR comment
- Uploads results as artifacts

**Manual trigger:**

1. Go to Actions → Performance Benchmark
2. Click "Run workflow"
3. Choose base branch (default: main)

## Docker Environment

All benchmarks run in a consistent Docker environment:

- **Base image:** rust:1.85.0-bullseye
- **Rust toolchain:** 1.85.0
- **Hyperfine:** 1.18.0
- **Python:** 3.x with matplotlib/numpy
- **Environment variables:**
  - `CARGO_BUILD_JOBS=4` - Consistent parallelism
  - `CARGO_INCREMENTAL=0` - Disable incremental compilation
  - `RUST_BACKTRACE=1` - Full backtraces
- **Hyperfine configuration:**
  - `--shell=none` - Direct execution without shell overhead
  - `--warmup 3` - 3 warmup runs before measurement
  - `--runs 10` - 10 timed runs for statistical significance

**Why `--shell=none`?**

Commands are executed directly without shell intermediary, which:
- Eliminates shell startup overhead (~0.5-1ms)
- Provides accurate measurements for fast benchmarks (< 5ms)
- Prevents hyperfine calibration warnings
- Improves consistency across all fixtures

This is safe for our benchmarks since we don't use shell features (pipes, redirects, etc.).

## Workflows

### Development Workflow

```bash
# Quick check before committing
make bench/run

# Compare with main
make bench/compare V1=main V2=HEAD
```

### Pull Request Workflow

1. **Automatic:** Benchmarks run on PR creation/update
2. **Review:** Check PR comment for performance changes
3. **Investigate:** If regressions, download artifacts for details

### Release Workflow

```bash
# Compare release candidates
make bench/compare V1=v1.0.0 V2=v2.0.0-rc1

# Generate charts
make bench/visualize RESULTS="benchmarks/results/compare_*/empty_v*.json"

# Archive results
cp benchmarks/results/compare_*/comparison.md releases/v2.0.0-benchmarks.md
```

## Adding New Fixtures

1. Create `.santa` file in `benchmarks/fixtures/`:

```santa
// My new benchmark
let compute = |n| [1..n] |> map(|x| x * x) |> sum;
compute(1000)
```

2. Benchmarks will automatically include it:

```bash
make bench/run
```

**Note:** Files starting with `aoc` are automatically run with the `-t` flag to execute tests. All other files run normally.

## Troubleshooting

### Docker image not found

```bash
make bench/build
```

### Permission denied errors

```bash
# Ensure results directory is writable
chmod 755 benchmarks/results
```

### Inconsistent results

Benchmarks can vary due to system load. For most reliable results:

- Close other applications
- Run multiple times
- Use `bench/compare` to compare versions fairly

### Out of disk space

```bash
# Clean old results
make bench/clean

# Clean Docker cache
docker system prune -a
```

## Performance Tips

**For fastest iteration:**

```bash
# Warm up Docker image
make bench/build
# Run benchmarks (uses cached builds)
make bench/run
```

**For most reproducible results:**

```bash
# Clean everything first
make bench/clean
docker system prune -a
# Rebuild and run
make bench/build
make bench/run
```

## Files Structure

```
benchmarks/
├── Dockerfile                    # Docker image definition
├── README.md                     # This file
├── Cargo.toml                   # Criterion benchmarks
├── benches/                      # Criterion benchmark code
│   ├── language.rs              # Lexer/parser benchmarks
│   └── evaluator.rs             # Runtime benchmarks
├── fixtures/                     # Test programs
│   ├── empty.santa
│   ├── fibonacci.santa
│   ├── list_processing.santa
│   └── pattern_matching.santa
├── scripts/                      # Helper scripts
│   ├── compare_results.py       # Compare benchmark results
│   └── visualize_results.py     # Generate charts
└── results/                      # Benchmark outputs (git-ignored)
```

## CI/CD Integration

The benchmarking system integrates with GitHub Actions to automatically measure performance impact of PRs.

### Workflow Configuration

**File:** `.github/workflows/benchmark.yml`

**Triggers:**

- **Automatic:** Pull requests to `main` that modify:
  - `lang/**` - Core language implementation
  - `runtime/cli/**` - CLI runtime
  - `Cargo.toml` / `Cargo.lock` - Dependencies
  - `benchmarks/**` - Benchmark code
  - `.github/workflows/benchmark.yml` - Workflow itself
  - `Makefile` - Build configuration

- **Manual:** Workflow dispatch with optional base branch selection

### Workflow Steps

1. **Setup**
   - Checkout repository with full history (`fetch-depth: 0`)
   - Set up Docker Buildx
   - Determine base and current refs for comparison

2. **Build**
   - Build benchmark Docker image
   - Cache layers for faster subsequent runs

3. **Benchmark Execution**
   - Run `make bench/compare V1=<base> V2=HEAD`
   - Benchmarks run in isolated Docker containers
   - Each fixture runs with 3 warmup + 10 timed runs

4. **Results Processing**
   - Generate comparison table
   - Create performance charts (if matplotlib available)
   - Determine status indicators (✓/🚀/⚠️)

5. **Reporting**
   - Post comparison table as PR comment
   - Link to downloadable artifacts
   - Update existing comment on subsequent runs

6. **Artifact Upload**
   - JSON results for each fixture and version
   - Comparison tables and charts
   - Retained for 30 days

### Interpreting CI Results

When benchmarks complete, a comment appears on the PR:

```
## Performance Comparison: main vs HEAD

| Benchmark        | main (ms) | HEAD (ms) | Change  | Status       |
|------------------|-----------|-----------|---------|--------------|
| empty            |     4.20  |     4.18  |  -0.5%  | ✓ no change  |
| fibonacci        |    45.60  |    43.20  |  -5.3%  | 🚀 improved  |
| list_processing  |    12.30  |    13.50  |  +9.8%  | ⚠️ regressed |
| pattern_matching |    28.70  |    28.90  |  +0.7%  | ✓ no change  |
```

**Status Thresholds:**
- ✓ **No change** - Less than 5% difference
- 🚀 **Improved** - More than 5% faster
- ⚠️ **Regressed** - More than 5% slower

**Action Items:**
- **Improvements** - Great! Consider mentioning in PR description
- **No change** - Expected for non-performance changes
- **Regressions** - Investigate before merging:
  1. Download artifacts for detailed analysis
  2. Run locally: `make bench/compare V1=main V2=HEAD`
  3. Check if regression is acceptable tradeoff for other benefits
  4. Consider optimization if regression is significant

### Manual CI Runs

To manually trigger benchmarks on a branch:

1. Go to **Actions** → **Performance Benchmark**
2. Click **Run workflow**
3. Select your branch
4. (Optional) Specify a different base branch to compare against
5. Click **Run workflow**

### Downloading Artifacts

1. Navigate to the workflow run in Actions tab
2. Scroll to **Artifacts** section
3. Download `benchmark-results` (JSON + tables)
4. Download `performance-chart` (PNG visualization)

### Customizing CI Behavior

**Change regression threshold:**

Edit `benchmarks/scripts/compare_results.py`:
```python
if abs(change) < 5:  # Change this value
    status = "✓ no change"
```

**Add/remove monitored paths:**

Edit `.github/workflows/benchmark.yml`:
```yaml
paths:
  - 'lang/**'
  - 'your/new/path/**'
```

**Adjust benchmark runs:**

Edit `Makefile` `bench/compare` target:
```bash
hyperfine --shell=none --warmup 3 --runs 10  # Adjust these values
```

**Note:** The `--shell=none` flag is recommended for accurate measurements. Remove it only if your benchmarks require shell features (pipes, redirects, etc.).

## Quick Reference

```bash
# Build
make bench/build             # Build Docker image

# Run
make bench/run               # Run benchmarks
make bench/criterion         # Run Criterion benchmarks

# Compare
make bench/compare V1=main V2=HEAD  # Compare versions

# Visualize
make bench/visualize RESULTS="*.json"  # Generate charts

# Utilities
make bench/shell             # Interactive Docker shell
make bench/clean             # Clean results
```
