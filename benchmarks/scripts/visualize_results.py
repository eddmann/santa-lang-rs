#!/usr/bin/env python3
"""Visualize benchmark results."""
import json
import sys
import glob
from pathlib import Path

try:
    import matplotlib.pyplot as plt
    import numpy as np
except ImportError:
    print("Error: matplotlib is required")
    sys.exit(1)


def main():
    if len(sys.argv) < 2:
        print("Usage: visualize_results.py <result1.json> [result2.json ...] --output <dir> --labels <label1> <label2>")
        sys.exit(1)

    args = sys.argv[1:]
    output_dir = Path("charts")
    labels = []
    file_patterns = []

    i = 0
    while i < len(args):
        if args[i] == "--output":
            output_dir = Path(args[i + 1])
            i += 2
        elif args[i] == "--labels":
            # Collect labels until next flag or end
            i += 1
            while i < len(args) and not args[i].startswith("--"):
                labels.append(args[i])
                i += 1
        else:
            file_patterns.append(args[i])
            i += 1

    output_dir.mkdir(parents=True, exist_ok=True)

    # Expand glob patterns and collect files
    files = []
    for pattern in file_patterns:
        matched = glob.glob(pattern)
        if matched:
            files.extend([Path(f) for f in matched])
        else:
            files.append(Path(pattern))

    if not files:
        print("No files found")
        sys.exit(1)

    # Group files by fixture name (for per-fixture JSON files)
    fixture_data = {}
    for file in files:
        with open(file) as f:
            data = json.load(f)

        # Determine label from filename or use provided labels
        if '_v1' in file.stem:
            version = labels[0] if len(labels) > 0 else 'v1'
        elif '_v2' in file.stem:
            version = labels[1] if len(labels) > 1 else 'v2'
        else:
            version = labels[len(fixture_data)] if len(labels) > len(fixture_data) else file.stem

        # Extract fixture name
        fixture_name = file.stem.replace('_v1', '').replace('_v2', '')

        if fixture_name not in fixture_data:
            fixture_data[fixture_name] = {}

        fixture_data[fixture_name][version] = data['results'][0]['mean'] * 1000

    # Create grouped data for plotting
    fixtures = sorted(fixture_data.keys())
    versions = sorted(set(v for f in fixture_data.values() for v in f.keys()))

    fig, ax = plt.subplots(figsize=(12, 6))
    x = np.arange(len(fixtures))
    width = 0.8 / len(versions)
    colors = plt.cm.Set2(np.linspace(0, 1, len(versions)))

    for i, version in enumerate(versions):
        means = [fixture_data[f].get(version, 0) for f in fixtures]
        offset = width * i - (width * len(versions) / 2) + width / 2
        bars = ax.bar(x + offset, means, width, label=version, alpha=0.8, color=colors[i])

        # Add value labels on bars
        for bar in bars:
            height = bar.get_height()
            if height > 0:
                ax.text(bar.get_x() + bar.get_width()/2., height,
                       f'{height:.1f}',
                       ha='center', va='bottom', fontsize=8)

    ax.set_xlabel('Benchmark', fontsize=12)
    ax.set_ylabel('Time (ms)', fontsize=12)
    ax.set_title('Performance Comparison', fontsize=14, fontweight='bold')
    ax.set_xticks(x)
    ax.set_xticklabels(fixtures, rotation=45, ha='right')
    ax.legend()
    ax.grid(axis='y', alpha=0.3)

    plt.tight_layout()
    output_file = output_dir / "comparison.png"
    plt.savefig(output_file, dpi=300, bbox_inches='tight')
    print(f"Chart saved: {output_file}")


if __name__ == "__main__":
    main()
