#!/usr/bin/env python3
#exonware/xwsystem/examples/serialization_example/x5_key_value/benchmark.py
"""
Key-Value Operations Benchmark
Tests database-backed serializers with key-value operations:
- LMDB: Memory-mapped key-value store
- SQLite3: SQL database operations
- Direct key access (get/put)
- Prefix scanning
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.0.1
Generation Date: October 12, 2025
"""

import sys
import time
from pathlib import Path
# Add common to path
parent_dir = Path(__file__).parent.parent
sys.path.insert(0, str(parent_dir))
# Add xwsystem to path
xwsystem_root = Path(__file__).parent.parent.parent.parent / "src"
sys.path.insert(0, str(xwsystem_root))
from x0_common import generate_sample_data, print_section
from exonware.xwsystem.serialization import LmdbSerializer, Sqlite3Serializer


def test_lmdb_operations():
    """Test LMDB key-value operations"""
    print("\n### LMDB Key-Value Operations ###")
    serializer = LmdbSerializer(map_size=1024**3)  # 1GB
    data = generate_sample_data()
    data_dir = Path(__file__).parent / "data" / "lmdb_db"
    data_dir.mkdir(parents=True, exist_ok=True)
    # Put operations
    print("\nput() operations:")
    try:
        start = time.perf_counter()
        for user in data['users']:
            serializer.put(f"user:{user['id']}", user, data_dir)
        for post in data['posts']:
            serializer.put(f"post:{post['id']}", post, data_dir)
        elapsed = (time.perf_counter() - start) * 1000
        total_items = len(data['users']) + len(data['posts'])
        print(f"  ✓ Stored {total_items} items in {elapsed:.2f}ms")
        print(f"  ✓ Avg time per item: {elapsed/total_items:.2f}ms")
    except Exception as e:
        print(f"  ✗ Error: {e}")
    # Get operations
    print("\nget() operations:")
    try:
        start = time.perf_counter()
        user1 = serializer.get("user:1", data_dir)
        user2 = serializer.get("user:2", data_dir)
        elapsed = (time.perf_counter() - start) * 1000
        print(f"  ✓ Retrieved 2 users in {elapsed:.2f}ms")
        print(f"  ✓ user:1 = {user1['name']}")
        print(f"  ✓ user:2 = {user2['name']}")
    except Exception as e:
        print(f"  ✗ Error: {e}")
    # Prefix scanning
    print("\nkeys() with prefix:")
    try:
        start = time.perf_counter()
        user_keys = list(serializer.keys(data_dir, prefix="user:"))
        elapsed = (time.perf_counter() - start) * 1000
        print(f"  ✓ Found {len(user_keys)} user keys in {elapsed:.2f}ms")
        for key in user_keys:
            print(f"    - {key}")
    except Exception as e:
        print(f"  ✗ Error: {e}")
    # items() iteration
    print("\nitems() iteration:")
    try:
        start = time.perf_counter()
        count = 0
        for key, value in serializer.items(data_dir, prefix="post:"):
            count += 1
        elapsed = (time.perf_counter() - start) * 1000
        print(f"  ✓ Iterated {count} posts in {elapsed:.2f}ms")
    except Exception as e:
        print(f"  ✗ Error: {e}")
    # Delete operation
    print("\ndelete() operation:")
    try:
        start = time.perf_counter()
        deleted = serializer.delete("user:3", data_dir)
        elapsed = (time.perf_counter() - start) * 1000
        print(f"  ✓ Deleted user:3 in {elapsed:.2f}ms")
        # Verify
        result = serializer.get("user:3", data_dir, default=None)
        print(f"  ✓ Verified deletion: {result is None}")
    except Exception as e:
        print(f"  ✗ Error: {e}")
    serializer.close()


def test_sqlite_operations():
    """Test SQLite3 key-value operations"""
    print("\n### SQLite3 Operations ###")
    serializer = Sqlite3Serializer()
    data = generate_sample_data()
    data_dir = Path(__file__).parent / "data"
    data_dir.mkdir(exist_ok=True)
    db_file = data_dir / "test.db"
    # Save data
    print("\nSave operations:")
    try:
        start = time.perf_counter()
        serializer.save_file(str(db_file), data)
        elapsed = (time.perf_counter() - start) * 1000
        print(f"  ✓ Saved data to SQLite in {elapsed:.2f}ms")
    except Exception as e:
        print(f"  ✗ Error: {e}")
    # Load data
    print("\nLoad operations:")
    try:
        start = time.perf_counter()
        loaded = serializer.load_file(str(db_file))
        elapsed = (time.perf_counter() - start) * 1000
        print(f"  ✓ Loaded data from SQLite in {elapsed:.2f}ms")
        print(f"  ✓ Users loaded: {len(loaded.get('users', []))}")
        print(f"  ✓ Posts loaded: {len(loaded.get('posts', []))}")
    except Exception as e:
        print(f"  ✗ Error: {e}")


def main():
    """Main benchmark runner"""
    print_section("KEY-VALUE OPERATIONS BENCHMARK")
    print("\nKey-value serializers provide database-like operations")
    print("with very fast reads through memory mapping (LMDB)!")
    test_lmdb_operations()
    test_sqlite_operations()
    print_section("SUMMARY")
    print("\n✓ Key-Value operations enable:")
    print("  - Direct key-based access (O(1) lookups)")
    print("  - Memory-mapped reads (LMDB)")
    print("  - Prefix-based scanning")
    print("  - Database-like operations on files")
    print("  - ACID transactions (SQLite)")
    data_dir = Path(__file__).parent / "data"
    print(f"\n✓ Output saved to: {data_dir}")
if __name__ == "__main__":
    main()
