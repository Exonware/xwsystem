# xwsystem benchmarks

Benchmark campaigns live in date-title folders. Each folder has:
- **scripts/** — Script(s) that run the benchmark
- **data/** — JSON, configs, and other data files
- **benchmarks/** — Markdown reports (BENCH_*.md, INDEX.md)

## Campaigns

| Folder | Description |
|--------|-------------|
| [20260209-benchmark xwsystem json vs others](20260209-benchmark%20xwsystem%20json%20vs%20others/) | JSON serialization and comparison benchmarks |
| [20260209-benchmark xwsystem caching vs others](20260209-benchmark%20xwsystem%20caching%20vs%20others/) | Caching and legacy benchmark reports (migrated from docs/logs/benchmarks) |
| [20260210-benchmark xwsystem serialization msgpack yaml pickle](20260210-benchmark%20xwsystem%20serialization%20msgpack%20yaml%20pickle/) | MsgPack, YAML, Pickle serialization (xwsystem vs others) |
| [20260210-benchmark xwsystem atomic io](20260210-benchmark%20xwsystem%20atomic%20io/) | Atomic file I/O vs plain write |
| [20260210-benchmark xwsystem operations diff merge patch](20260210-benchmark%20xwsystem%20operations%20diff%20merge%20patch/) | Diff, merge, patch operations throughput |
| [20260210-benchmark xwsystem data structures trie unionfind](20260210-benchmark%20xwsystem%20data%20structures%20trie%20unionfind/) | Trie (TrieNode) and UnionFind throughput |
| [20260210-benchmark xwsystem object pool](20260210-benchmark%20xwsystem%20object%20pool/) | ObjectPool get/release vs direct instantiation |
| [20260210-benchmark xwsystem validation](20260210-benchmark%20xwsystem%20validation/) | check_data_depth and validate_path_input throughput |
| [20260210-benchmark xwsystem threading locks](20260210-benchmark%20xwsystem%20threading%20locks/) | EnhancedRLock vs threading.RLock |
| [20260210-benchmark xwsystem async fabric](20260210-benchmark%20xwsystem%20async%20fabric/) | Async Process Fabric: task submission, queue, shared memory (REF_54_BENCH scaffold) |

*Per GUIDE_54_BENCH: benchmarks at project root; date at start (YYYYMMDD), then title.*
