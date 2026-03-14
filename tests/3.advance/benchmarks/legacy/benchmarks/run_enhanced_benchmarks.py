#!/usr/bin/env python3
#exonware/xwsystem/tests/3.advance/benchmarks/legacy/benchmarks/run_enhanced_benchmarks.py
"""
#exonware/xwsystem/benchmarks/run_enhanced_benchmarks.py
Execute enhanced benchmarks and save results for comparison.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.0.1.388
Generation Date: 01-Nov-2025
"""

import json
import sys
from pathlib import Path
from datetime import datetime
# Add src to path
src_path = Path(__file__).parent.parent / "src"
sys.path.insert(0, str(src_path))
from enhanced_caching_benchmarks import EnhancedCacheBenchmark


def save_results(results: dict, filename: str):
    """Save benchmark results to JSON file."""
    output_dir = Path(__file__).parent
    output_file = output_dir / filename
    # Add metadata
    results['benchmark_date'] = datetime.now().isoformat()
    results['version'] = '0.0.1.388'
    results['phase'] = 'improved'
    with open(output_file, 'w', encoding='utf-8') as f:
        json.dump(results, f, indent=2)
    print(f"\n✅ Results saved to: {output_file}")
    return output_file


def print_summary(results: dict):
    """Print human-readable summary."""
    print("\n" + "=" * 80)
    print("ENHANCED BENCHMARK RESULTS SUMMARY")
    print("=" * 80)
    print(f"Date: {results.get('benchmark_date', 'Unknown')}")
    print(f"Version: {results.get('version', 'Unknown')}")
    print(f"Operations: {results.get('num_operations', 0):,}")
    print("=" * 80)
    print("\nPerformance Metrics:")
    print("-" * 80)
    print(f"{'Benchmark':<35s} {'Ops/Sec':>15s} {'P50 (ms)':>12s} {'P95 (ms)':>12s}")
    print("-" * 80)
    for result in results.get('results', []):
        if result.get('success'):
            name = result['name']
            ops = result['operations_per_second']
            p50 = result['latency_p50_ms']
            p95 = result['latency_p95_ms']
            print(f"{name:<35s} {ops:>15,.2f} {p50:>12.4f} {p95:>12.4f}")
    print("-" * 80)


def compare_with_baseline(improved_results: dict):
    """Compare with baseline and show improvements."""
    baseline_file = Path(__file__).parent / 'baseline_v0.0.1.387.json'
    if not baseline_file.exists():
        print("\n⚠️  Baseline not found - cannot compare")
        return
    with open(baseline_file) as f:
        baseline = json.load(f)
    baseline_results = {r['name']: r for r in baseline.get('results', [])}
    improved_results_map = {r['name']: r for r in improved_results.get('results', [])}
    print("\n" + "=" * 80)
    print("IMPROVEMENTS vs BASELINE")
    print("=" * 80)
    print(f"{'Benchmark':<35s} {'Baseline':>15s} {'Improved':>15s} {'Speedup':>10s}")
    print("-" * 80)
    improvements = []
    for name in baseline_results:
        if name in improved_results_map:
            baseline_ops = baseline_results[name]['operations_per_second']
            improved_ops = improved_results_map[name]['operations_per_second']
            speedup = improved_ops / baseline_ops if baseline_ops > 0 else 1.0
            improvements.append((name, baseline_ops, improved_ops, speedup))
    # Sort by speedup
    improvements.sort(key=lambda x: x[3], reverse=True)
    for name, baseline_ops, improved_ops, speedup in improvements:
        indicator = "🚀" if speedup > 1.5 else "✅" if speedup >= 1.0 else "⚠️"
        print(
            f"{indicator} {name:<32s} {baseline_ops:>15,.0f} {improved_ops:>15,.0f} "
            f"{speedup:>9.2f}x"
        )
    print("=" * 80)
    # Highlight key improvements
    print("\nKEY IMPROVEMENTS:")
    for name, baseline_ops, improved_ops, speedup in improvements[:3]:
        if speedup > 1.1:
            print(f"  🎯 {name}: {speedup:.1f}x faster")


def main():
    """Main execution."""
    print("=" * 80)
    print("ENHANCED CACHING MODULE BENCHMARK RUNNER")
    print("=" * 80)
    print("\nRunning comprehensive benchmarks for v0.0.1.388...")
    print("This may take a few minutes...\n")
    # Run enhanced benchmarks
    benchmark = EnhancedCacheBenchmark(num_operations=10000)
    benchmark.run_all_enhanced_benchmarks()
    # Get results
    results = benchmark.get_results_dict()
    # Save to file
    output_file = save_results(results, 'improved_v0.0.1.388.json')
    # Print summary
    print_summary(results)
    # Compare with baseline
    compare_with_baseline(results)
    print("\n" + "=" * 80)
    print("NEXT STEPS")
    print("=" * 80)
    print("1. Review improved performance metrics")
    print("2. Generate comparison report:")
    print("   python benchmarks/comparison_report.py")
    print("=" * 80)
    return 0
if __name__ == "__main__":
    sys.exit(main())
