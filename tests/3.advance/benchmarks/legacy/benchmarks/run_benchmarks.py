#!/usr/bin/env python3
#exonware/xwsystem/tests/3.advance/benchmarks/legacy/benchmarks/run_benchmarks.py
"""
#exonware/xwsystem/benchmarks/run_benchmarks.py
Execute benchmarks and save results with visualization.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.0.1.387
Generation Date: 01-Nov-2025
"""

import json
import sys
from pathlib import Path
from datetime import datetime
# Add src to path
src_path = Path(__file__).parent.parent / "src"
sys.path.insert(0, str(src_path))
from caching_benchmarks import CacheBenchmark


def save_results(results: dict, filename: str):
    """Save benchmark results to JSON file."""
    output_dir = Path(__file__).parent
    output_file = output_dir / filename
    # Add metadata
    results['benchmark_date'] = datetime.now().isoformat()
    results['version'] = '0.0.1.387'
    results['phase'] = 'baseline'
    with open(output_file, 'w', encoding='utf-8') as f:
        json.dump(results, f, indent=2)
    print(f"\n[SUCCESS] Results saved to: {output_file}")
    return output_file


def print_summary(results: dict):
    """Print human-readable summary."""
    print("\n" + "=" * 80)
    print("BENCHMARK RESULTS SUMMARY")
    print("=" * 80)
    print(f"Date: {results.get('benchmark_date', 'Unknown')}")
    print(f"Version: {results.get('version', 'Unknown')}")
    print(f"Operations: {results.get('num_operations', 0):,}")
    print("=" * 80)
    print("\nPerformance Metrics:")
    print("-" * 80)
    print(f"{'Benchmark':<30s} {'Ops/Sec':>15s} {'P50 (ms)':>12s} {'P95 (ms)':>12s}")
    print("-" * 80)
    for result in results.get('results', []):
        if result.get('success'):
            name = result['name']
            ops = result['operations_per_second']
            p50 = result['latency_p50_ms']
            p95 = result['latency_p95_ms']
            print(f"{name:<30s} {ops:>15,.2f} {p50:>12.4f} {p95:>12.4f}")
    print("-" * 80)


def main():
    """Main execution."""
    print("=" * 80)
    print("CACHING MODULE BENCHMARK RUNNER")
    print("=" * 80)
    print("\nRunning comprehensive benchmarks...")
    print("This may take a few minutes...\n")
    # Run benchmarks
    benchmark = CacheBenchmark(num_operations=10000)
    benchmark.run_all_benchmarks()
    # Get results
    results = benchmark.get_results_dict()
    # Save to file
    output_file = save_results(results, 'baseline_v0.0.1.387.json')
    # Print summary
    print_summary(results)
    print("\n" + "=" * 80)
    print("NEXT STEPS")
    print("=" * 80)
    print("1. Review baseline performance metrics")
    print("2. Implement improvements")
    print("3. Run benchmarks again to compare")
    print("4. Generate comparison report")
    print("=" * 80)
    return 0
if __name__ == "__main__":
    sys.exit(main())
