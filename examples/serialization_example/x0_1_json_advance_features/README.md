# JSON Advanced Features Demo

**Company:** eXonware.com  
**Author:** eXonware Backend Team  
**Email:** connect@exonware.com  
**Version:** 0.0.1.389  
**Generation Date:** November 9, 2025

## Overview

Demonstrates advanced serializer features on a massive 1GB JSON file containing social media data. Shows how to efficiently work with huge files using path-based operations, queries, and merges without loading the entire file into memory.

## Features Demonstrated

- **atomic_update_path**: Update single paths without loading full file
- **atomic_read_path**: Read single paths efficiently  
- **query**: JSONPath queries for complex searches
- **merge**: Deep merge operations
- **JSON References ($ref)**: Relationship references between users

## Goal

All operations complete in **milliseconds** even on 1GB files!

## Data Structure

The generated JSON file contains:

```json
{
  "metadata": {
    "version": "1.0",
    "generated_at": "...",
    "total_users": 30000+,
    "total_posts": 300000+,
    "total_relationships": 15000000+
  },
  "users": [
    {
      "id": "user_1234567",
      "profile": {
        "username": "cool_ninja_123",
        "display_name": "John Smith",
        "bio": "...",
        "location": "New York, USA",
        "followers_count": 50000,
        "verified": true,
        ...
      },
      "following": [
        {"$ref": "#/users/user_4567890"},
        ...
      ],
      "followers": [
        {"$ref": "#/users/user_7890123"},
        ...
      ],
      "posts": [
        {
          "id": "post_user_1234567_0",
          "author_ref": {"$ref": "#/users/user_1234567"},
          "content_type": "image",
          "text": "...",
          "likes_count": 1000,
          ...
        },
        ...
      ]
    },
    ...
  ]
}
```

## Usage

### Step 1: Generate the 1GB JSON file

First, generate the 1GB JSON file:

```bash
cd x0_1_json_advance_features
python generate_data.py
```

This will create `data/social_media_1gb.json` (~1GB file with social media data).

### Step 2: Run the benchmark

Then run the benchmark to test advanced features:

```bash
python benchmark.py
```

The benchmark will automatically detect and use the pre-generated file. If the file doesn't exist, it will prompt you to generate it.

## Performance Metrics

The demo measures and displays:

- **Operation Time**: Milliseconds for each operation
- **Memory Usage**: Baseline, delta, and current memory
- **File Size**: Actual file size in GB
- **Memory Efficiency**: Comparison to file size

## Example Output

```
🚀 JSON Advanced Features Demo - 3GB Social Media Data
======================================================================

📊 File Information:
   Size: 3.00 GB (3,221,225,472 bytes)
   Memory baseline: 45.23 MB

🔍 1. atomic_read_path - Reading multiple paths
----------------------------------------------------------------------
   ⏱️  Time: 234.56 ms
   💾 Memory delta: +12.34 MB
   📊 Memory usage: 57.57 MB
   ✅ FAST: Completed in 234.56 ms (< 1 second)
   📖 Read metadata, user ID, username, and followers count
   👤 Username: cool_ninja_123
   👥 Followers: 50,000

🔍 2. atomic_update_path - Updating multiple paths
----------------------------------------------------------------------
   ⏱️  Time: 456.78 ms
   💾 Memory delta: +15.67 MB
   📊 Memory usage: 73.24 MB
   ✅ FAST: Completed in 456.78 ms (< 1 second)
   ✏️  Updated bio, followers count, and verification status
   ...
```

## Requirements

- Python 3.8+
- psutil (for memory monitoring)
- jsonpointer (for path operations)
- jsonpath-ng (for JSONPath queries)

## Files

- **`generate_data.py`**: Script to generate the 1GB JSON file
- **`benchmark.py`**: Main demo script that tests all advanced features
- **`data/social_media_1gb.json`**: Generated 1GB JSON file (created by generate_data.py)

## Notes

- First run `generate_data.py` to create the 1GB file (takes several minutes)
- The benchmark will automatically use the existing file if found
- All operations use atomic file writes for data safety
- Memory usage is monitored to ensure efficiency
- The file contains realistic social media data with JSON references ($ref)

