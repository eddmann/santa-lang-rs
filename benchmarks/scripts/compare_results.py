#!/usr/bin/env python3
"""Compare benchmark results from compare directory."""
import json
import sys
from pathlib import Path

def main():
    if len(sys.argv) < 4:
        print("Usage: compare_results.py <results_dir> <v1_name> <v2_name>")
        sys.exit(1)

    results_dir = Path(sys.argv[1])
    v1_name = sys.argv[2]
    v2_name = sys.argv[3]

    # Find all fixture pairs
    fixtures = set()
    for f in results_dir.glob("*_v1.json"):
        fixtures.add(f.stem.replace("_v1", ""))

    print(f"\n## Performance Comparison: {v1_name} vs {v2_name}\n")
    print("| Benchmark | {} (ms) | {} (ms) | Change | Status |".format(v1_name, v2_name))
    print("|-----------|" + "-" * (len(v1_name) + 6) + "|" + "-" * (len(v2_name) + 6) + "|--------|--------|")

    for fixture in sorted(fixtures):
        v1_file = results_dir / f"{fixture}_v1.json"
        v2_file = results_dir / f"{fixture}_v2.json"

        if not v1_file.exists() or not v2_file.exists():
            continue

        with open(v1_file) as f:
            v1_data = json.load(f)
        with open(v2_file) as f:
            v2_data = json.load(f)

        v1_mean = v1_data['results'][0]['mean'] * 1000
        v2_mean = v2_data['results'][0]['mean'] * 1000
        change = ((v2_mean - v1_mean) / v1_mean) * 100

        if abs(change) < 5:
            status = "✓ no change"
        elif change < 0:
            status = "🚀 improved"
        else:
            status = "⚠️ regressed"

        print(f"| {fixture:20} | {v1_mean:>7.2f} | {v2_mean:>7.2f} | {change:+6.1f}% | {status} |")

    print("\n### Legend")
    print("- ✓ No significant change (< 5%)")
    print("- 🚀 Performance improved (> 5% faster)")
    print("- ⚠️ Performance regressed (> 5% slower)\n")

if __name__ == "__main__":
    main()
