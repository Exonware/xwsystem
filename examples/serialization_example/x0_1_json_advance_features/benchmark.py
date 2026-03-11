#!/usr/bin/env python3
#exonware/xwsystem/examples/serialization_example/x0_1_json_advance_features/benchmark.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.0.1.389
Generation Date: November 9, 2025
JSON Advanced Features Demo - 3GB Social Media Data
Demonstrates advanced serializer features on a massive JSON file:
- atomic_update_path: Update single paths without loading full file
- atomic_read_path: Read single paths efficiently
- query: JSONPath queries for complex searches
- merge: Deep merge operations
- JSON References ($ref) for relationships
Goal: All operations complete in milliseconds even on 3GB files.
"""

import os
import sys
import time
import psutil
import json
from pathlib import Path
from typing import Any, Optional
from datetime import datetime, timedelta
import random
import string
# Add src to path for development
xwsystem_root = Path(__file__).parent.parent.parent.parent / "src"
sys.path.insert(0, str(xwsystem_root))
from exonware.xwsystem.io.serialization.formats.text.json import JsonSerializer
from exonware.xwsystem.io.serialization.serializer import XWSerializer


class SocialMediaDataGenerator:
    """Generate realistic social media data with JSON references."""

    def __init__(self, target_size_gb: float = 3.0):
        """
        Initialize data generator.
        Args:
            target_size_gb: Target file size in GB
        """
        self.target_size_gb = target_size_gb
        self.target_size_bytes = int(target_size_gb * 1024 * 1024 * 1024)
        self.users: list[dict[str, Any]] = []
        self.user_ids: list[str] = []

    def _generate_user_id(self) -> str:
        """Generate a unique user ID."""
        return f"user_{random.randint(1000000, 9999999)}"

    def _generate_username(self) -> str:
        """Generate a random username."""
        adjectives = ["cool", "awesome", "epic", "legendary", "mega", "super", "ultra"]
        nouns = ["ninja", "warrior", "hero", "star", "king", "queen", "master", "pro"]
        return f"{random.choice(adjectives)}_{random.choice(nouns)}_{random.randint(100, 999)}"

    def _generate_profile(self, user_id: str) -> dict[str, Any]:
        """Generate user profile data."""
        return {
            "id": user_id,
            "username": self._generate_username(),
            "display_name": f"{random.choice(['John', 'Jane', 'Alex', 'Sam', 'Taylor'])} {random.choice(['Smith', 'Johnson', 'Williams', 'Brown', 'Jones'])}",
            "bio": f"Passionate about {random.choice(['technology', 'music', 'art', 'sports', 'travel', 'food'])}. Living life to the fullest!",
            "location": f"{random.choice(['New York', 'Los Angeles', 'Chicago', 'Houston', 'Phoenix', 'Philadelphia'])}, {random.choice(['USA', 'Canada', 'UK', 'Australia'])}",
            "website": f"https://example.com/{user_id}",
            "joined_date": (datetime.now() - timedelta(days=random.randint(1, 3650))).isoformat(),
            "verified": random.choice([True, False]),
            "followers_count": random.randint(0, 1000000),
            "following_count": random.randint(0, 5000),
            "posts_count": random.randint(0, 10000),
            "profile_image_url": f"https://example.com/avatars/{user_id}.jpg",
            "cover_image_url": f"https://example.com/covers/{user_id}.jpg",
            "privacy": random.choice(["public", "private"]),
            "language": random.choice(["en", "es", "fr", "de", "ja", "zh"]),
            "timezone": random.choice(["UTC", "EST", "PST", "GMT", "JST"]),
        }

    def _generate_post(self, user_id: str, post_id: str) -> dict[str, Any]:
        """Generate a social media post."""
        content_types = ["text", "image", "video", "link", "poll"]
        content_type = random.choice(content_types)
        post = {
            "id": post_id,
            "author_ref": {"$ref": f"#/users/{user_id}"},
            "content_type": content_type,
            "text": self._generate_post_text(),
            "created_at": (datetime.now() - timedelta(hours=random.randint(0, 720))).isoformat(),
            "likes_count": random.randint(0, 50000),
            "comments_count": random.randint(0, 5000),
            "shares_count": random.randint(0, 1000),
            "views_count": random.randint(0, 100000),
        }
        if content_type == "image":
            post["media"] = {
                "type": "image",
                "url": f"https://example.com/media/{post_id}.jpg",
                "width": random.choice([1080, 1920, 2048]),
                "height": random.choice([1080, 1920, 2048]),
            }
        elif content_type == "video":
            post["media"] = {
                "type": "video",
                "url": f"https://example.com/media/{post_id}.mp4",
                "duration": random.randint(10, 600),
                "thumbnail": f"https://example.com/thumbnails/{post_id}.jpg",
            }
        elif content_type == "link":
            post["link"] = {
                "url": f"https://example.com/articles/{post_id}",
                "title": f"Interesting Article {post_id}",
                "description": f"Check out this amazing article about {random.choice(['technology', 'science', 'art', 'culture'])}",
            }
        return post

    def _generate_post_text(self) -> str:
        """Generate realistic post text."""
        templates = [
            "Just had an amazing day! {emoji}",
            "Check out this {topic}! What do you think?",
            "Feeling grateful today. {message}",
            "New project announcement! Stay tuned for updates.",
            "Can't believe it's been {time} since {event}.",
            "Sharing some thoughts on {topic}. Let me know your opinion!",
        ]
        template = random.choice(templates)
        emojis = ["😊", "🎉", "🚀", "💪", "🌟", "✨", "🎯", "🔥"]
        topics = ["technology", "innovation", "creativity", "life", "adventure", "success"]
        messages = ["Life is beautiful", "Every day is a gift", "Making progress", "Living the dream"]
        events = ["we started", "I joined", "this happened", "we launched"]
        times = ["a year", "6 months", "3 months", "a month", "a week"]
        text = template.format(
            emoji=random.choice(emojis),
            topic=random.choice(topics),
            message=random.choice(messages),
            event=random.choice(events),
            time=random.choice(times),
        )
        return text

    def generate_data(self) -> dict[str, Any]:
        """
        Generate social media data structure targeting ~1GB.
        Structure:
        {
            "metadata": {...},
            "users": [
                {
                    "id": "user_123",
                    "profile": {...},
                    "following": [{"$ref": "#/users/user_456"}, ...],
                    "followers": [{"$ref": "#/users/user_789"}, ...],
                    "posts": [...]
                }
            ]
        }
        """
        print(f"\n📊 Generating {self.target_size_gb}GB social media data...")
        print("=" * 70)
        # Estimate: Each user with profile + relationships + posts ≈ 50-100KB
        # For 3GB: Need ~30,000-60,000 users
        estimated_users = max(30000, int(self.target_size_bytes / 100000))  # Conservative estimate
        print(f"   Target size: {self.target_size_gb:.2f} GB ({self.target_size_bytes:,} bytes)")
        print(f"   Estimated users: {estimated_users:,}")
        print(f"   Generating data...")
        start_time = time.time()
        current_size = 0
        # Generate metadata
        metadata = {
            "version": "1.0",
            "generated_at": datetime.now().isoformat(),
            "total_users": 0,
            "total_posts": 0,
            "total_relationships": 0,
            "description": "Large-scale social media dataset with JSON references",
        }
        users = []
        user_index_map = {}  # Map user_id to index for $ref generation
        # Generate users in batches to monitor progress
        batch_size = 1000
        posts_per_user = 10  # Average posts per user
        for batch_start in range(0, estimated_users, batch_size):
            batch_end = min(batch_start + batch_size, estimated_users)
            for i in range(batch_start, batch_end):
                user_id = self._generate_user_id()
                self.user_ids.append(user_id)
                user_index_map[user_id] = len(users)
                # Generate profile
                profile = self._generate_profile(user_id)
                # Generate following relationships (using $ref)
                following_count = random.randint(0, min(500, len(users)))  # Follow up to 500 existing users
                following = []
                if users:  # Only if we have existing users
                    following_ids = random.sample(
                        [u["id"] for u in users[-min(1000, len(users)):]],  # Follow recent users
                        min(following_count, min(1000, len(users)))
                    )
                    following = [{"$ref": f"#/users/{fid}"} for fid in following_ids]
                # Generate followers (will be populated by later users)
                followers = []  # Will be populated as we add more users
                # Generate posts
                posts = []
                for p in range(posts_per_user):
                    post_id = f"post_{user_id}_{p}"
                    posts.append(self._generate_post(user_id, post_id))
                user_data = {
                    "id": user_id,
                    "profile": profile,
                    "following": following,
                    "followers": followers,  # Will be populated later
                    "posts": posts,
                }
                users.append(user_data)
                # Update followers for users this user follows (bidirectional)
                for ref in following:
                    if "$ref" in ref:
                        ref_id = ref["$ref"].split("/")[-1]
                        # Find user and add this user to their followers
                        for u in users:
                            if u["id"] == ref_id:
                                u["followers"].append({"$ref": f"#/users/{user_id}"})
                                break
            # Estimate current size
            temp_data = {
                "metadata": metadata,
                "users": users,
            }
            temp_json = json.dumps(temp_data, indent=2)
            current_size = len(temp_json.encode('utf-8'))
            elapsed = time.time() - start_time
            progress = (batch_end / estimated_users) * 100
            size_gb = current_size / (1024 ** 3)
            print(f"   Progress: {progress:.1f}% | Users: {batch_end:,}/{estimated_users:,} | "
                  f"Size: {size_gb:.2f} GB | Time: {elapsed:.1f}s", end='\r')
            # Stop if we've reached target size
            if current_size >= self.target_size_bytes * 0.95:  # 95% of target
                break
        print()  # New line after progress
        # Update metadata
        metadata["total_users"] = len(users)
        metadata["total_posts"] = sum(len(u["posts"]) for u in users)
        metadata["total_relationships"] = sum(len(u["following"]) for u in users)
        data = {
            "metadata": metadata,
            "users": users,
        }
        elapsed = time.time() - start_time
        final_size = len(json.dumps(data, indent=2).encode('utf-8'))
        final_size_gb = final_size / (1024 ** 3)
        print(f"\n✅ Data generation complete!")
        print(f"   Users: {len(users):,}")
        print(f"   Posts: {metadata['total_posts']:,}")
        print(f"   Relationships: {metadata['total_relationships']:,}")
        print(f"   Final size: {final_size_gb:.2f} GB ({final_size:,} bytes)")
        print(f"   Generation time: {elapsed:.2f} seconds")
        return data


class PerformanceMonitor:
    """Monitor memory usage and operation times."""

    def __init__(self):
        self.process = psutil.Process(os.getpid())
        self.baseline_memory = self._get_memory_mb()

    def _get_memory_mb(self) -> float:
        """Get current memory usage in MB."""
        return self.process.memory_info().rss / (1024 * 1024)

    def get_memory_delta(self) -> float:
        """Get memory delta from baseline in MB."""
        return self._get_memory_mb() - self.baseline_memory

    def format_size(self, bytes_size: int) -> str:
        """Format bytes to human-readable size."""
        for unit in ['B', 'KB', 'MB', 'GB', 'TB']:
            if bytes_size < 1024.0:
                return f"{bytes_size:.2f} {unit}"
            bytes_size /= 1024.0
        return f"{bytes_size:.2f} PB"

    def measure_operation(self, operation_name: str, operation_func):
        """Measure operation time and memory."""
        print(f"\n🔍 {operation_name}")
        print("-" * 70)
        # Warm up (if needed)
        memory_before = self._get_memory_mb()
        # Measure operation
        start_time = time.perf_counter()
        result = operation_func()
        end_time = time.perf_counter()
        memory_after = self._get_memory_mb()
        memory_delta = memory_after - memory_before
        elapsed_ms = (end_time - start_time) * 1000
        print(f"   ⏱️  Time: {elapsed_ms:.2f} ms")
        print(f"   💾 Memory delta: {memory_delta:+.2f} MB")
        print(f"   📊 Memory usage: {memory_after:.2f} MB")
        if elapsed_ms < 1000:
            print(f"   ✅ FAST: Completed in {elapsed_ms:.2f} ms (< 1 second)")
        elif elapsed_ms < 5000:
            print(f"   ⚠️  MODERATE: Completed in {elapsed_ms/1000:.2f} seconds")
        else:
            print(f"   ❌ SLOW: Completed in {elapsed_ms/1000:.2f} seconds")
        return result, elapsed_ms, memory_delta


def main():
    """Main demo function."""
    print("=" * 70)
    print("🚀 JSON Advanced Features Demo - 1GB Social Media Data")
    print("=" * 70)
    print("\nDemonstrating:")
    print("  • atomic_update_path: Update paths without loading full file")
    print("  • atomic_read_path: Read paths efficiently")
    print("  • query: JSONPath queries for complex searches")
    print("  • merge: Deep merge operations")
    print("  • JSON References ($ref) for relationships")
    print("\nGoal: All operations complete in milliseconds!")
    print("=" * 70)
    # Setup
    data_dir = Path(__file__).parent / "data"
    data_dir.mkdir(exist_ok=True)
    json_file = data_dir / "social_media_1gb.json"
    monitor = PerformanceMonitor()
    serializer = JsonSerializer()
    xw_serializer = XWSerializer()
    # Check if file exists
    if json_file.exists():
        file_size = json_file.stat().st_size
        file_size_gb = file_size / (1024 ** 3)
        print(f"\n📁 Found existing file: {json_file}")
        print(f"   Size: {monitor.format_size(file_size)} ({file_size_gb:.2f} GB)")
        print(f"   ✅ Using existing file for demo")
    else:
        print(f"\n⚠️  File not found: {json_file}")
        print(f"   Please run generate_data.py first to create the 1GB JSON file:")
        print(f"   python generate_data.py")
        print(f"\n   Or the demo will generate it now (this may take several minutes)...")
        response = input("\n   Generate file now? (y/N): ").strip().lower()
        if response == 'y':
            # Generate data
            generator = SocialMediaDataGenerator(target_size_gb=1.0)
            data = generator.generate_data()
            # Save to file
            print(f"\n💾 Saving to {json_file}...")
            save_start = time.time()
            serializer.save_file(data, json_file, indent=2)
            save_time = time.time() - save_start
            file_size = json_file.stat().st_size
            file_size_gb = file_size / (1024 ** 3)
            print(f"   ✅ Saved in {save_time:.2f} seconds")
            print(f"   📊 File size: {monitor.format_size(file_size)} ({file_size_gb:.2f} GB)")
        else:
            print("\n   Exiting. Please generate the file first.")
            return
    # Get file info
    file_size = json_file.stat().st_size
    file_size_gb = file_size / (1024 ** 3)
    print(f"\n📊 File Information:")
    print(f"   Path: {json_file}")
    print(f"   Size: {monitor.format_size(file_size)} ({file_size_gb:.2f} GB)")
    print(f"   Memory baseline: {monitor.baseline_memory:.2f} MB")
    # ========================================================================
    # DEMONSTRATION 1: atomic_read_path - Read single paths efficiently
    # ========================================================================
    def demo_atomic_read_path():
        """Demonstrate atomic_read_path."""
        # Read metadata
        metadata = serializer.atomic_read_path(json_file, "/metadata")
        # Read specific user profile
        first_user_id = serializer.atomic_read_path(json_file, "/users/0/id")
        profile = serializer.atomic_read_path(json_file, f"/users/0/profile")
        # Read nested path
        username = serializer.atomic_read_path(json_file, f"/users/0/profile/username")
        followers_count = serializer.atomic_read_path(json_file, f"/users/0/profile/followers_count")
        return {
            "metadata": metadata,
            "first_user_id": first_user_id,
            "username": username,
            "followers_count": followers_count,
        }
    result, elapsed_ms, mem_delta = monitor.measure_operation(
        "1. atomic_read_path - Reading multiple paths",
        demo_atomic_read_path
    )
    print(f"   📖 Read metadata, user ID, username, and followers count")
    print(f"   👤 Username: {result['username']}")
    print(f"   👥 Followers: {result['followers_count']:,}")
    # ========================================================================
    # DEMONSTRATION 2: atomic_update_path - Update paths atomically
    # ========================================================================
    def demo_atomic_update_path():
        """Demonstrate atomic_update_path."""
        # Update user's bio
        new_bio = f"Updated bio at {datetime.now().isoformat()} - Advanced features demo!"
        serializer.atomic_update_path(
            json_file,
            "/users/0/profile/bio",
            new_bio,
            backup=True
        )
        # Update followers count
        serializer.atomic_update_path(
            json_file,
            "/users/0/profile/followers_count",
            999999,
            backup=False
        )
        # Update verification status
        serializer.atomic_update_path(
            json_file,
            "/users/0/profile/verified",
            True,
            backup=False
        )
        # Verify updates
        updated_bio = serializer.atomic_read_path(json_file, "/users/0/profile/bio")
        updated_followers = serializer.atomic_read_path(json_file, "/users/0/profile/followers_count")
        updated_verified = serializer.atomic_read_path(json_file, "/users/0/profile/verified")
        return {
            "bio": updated_bio,
            "followers": updated_followers,
            "verified": updated_verified,
        }
    result, elapsed_ms, mem_delta = monitor.measure_operation(
        "2. atomic_update_path - Updating multiple paths",
        demo_atomic_update_path
    )
    print(f"   ✏️  Updated bio, followers count, and verification status")
    print(f"   📝 Bio: {result['bio'][:60]}...")
    print(f"   👥 Followers: {result['followers']:,}")
    print(f"   ✅ Verified: {result['verified']}")
    # ========================================================================
    # DEMONSTRATION 3: query - JSONPath queries
    # ========================================================================
    def demo_query():
        """Demonstrate JSONPath queries."""
        # Find all verified users
        verified_users = serializer.query(json_file, "$.users[?(@.profile.verified == true)]")
        # Find users with > 100k followers
        popular_users = serializer.query(json_file, "$.users[?(@.profile.followers_count > 100000)]")
        # Find users from specific location
        ny_users = serializer.query(json_file, "$.users[?(@.profile.location =~ /New York.*/)]")
        # Get all usernames
        usernames = serializer.query(json_file, "$.users[*].profile.username")
        return {
            "verified_count": len(verified_users),
            "popular_count": len(popular_users),
            "ny_count": len(ny_users),
            "total_usernames": len(usernames),
        }
    result, elapsed_ms, mem_delta = monitor.measure_operation(
        "3. query - JSONPath queries",
        demo_query
    )
    print(f"   🔍 Found {result['verified_count']:,} verified users")
    print(f"   🌟 Found {result['popular_count']:,} popular users (>100k followers)")
    print(f"   🗽 Found {result['ny_count']:,} users from New York")
    print(f"   📋 Retrieved {result['total_usernames']:,} usernames")
    # ========================================================================
    # DEMONSTRATION 4: merge - Deep merge operations
    # ========================================================================
    def demo_merge():
        """Demonstrate merge operations."""
        # Create updates
        updates = {
            "metadata": {
                "last_updated": datetime.now().isoformat(),
                "update_reason": "Advanced features demo",
            },
            "users": [
                {
                    "id": serializer.atomic_read_path(json_file, "/users/0/id"),
                    "profile": {
                        "status": "active",
                        "last_active": datetime.now().isoformat(),
                    }
                }
            ]
        }
        # Perform merge
        serializer.merge(json_file, updates, deep=True)
        # Verify merge
        last_updated = serializer.atomic_read_path(json_file, "/metadata/last_updated")
        user_status = serializer.atomic_read_path(json_file, "/users/0/profile/status")
        return {
            "last_updated": last_updated,
            "user_status": user_status,
        }
    result, elapsed_ms, mem_delta = monitor.measure_operation(
        "4. merge - Deep merge operations",
        demo_merge
    )
    print(f"   🔀 Merged metadata and user profile updates")
    print(f"   📅 Last updated: {result['last_updated']}")
    print(f"   👤 User status: {result['user_status']}")
    # ========================================================================
    # DEMONSTRATION 5: XWSerializer delegation
    # ========================================================================
    def demo_xw_serializer():
        """Demonstrate XWSerializer delegation."""
        # XWSerializer automatically detects format and delegates
        username = xw_serializer.atomic_read_path(json_file, "/users/0/profile/username")
        xw_serializer.atomic_update_path(
            json_file,
            "/users/0/profile/bio",
            f"Updated via XWSerializer at {datetime.now().isoformat()}",
            backup=False
        )
        return {"username": username}
    result, elapsed_ms, mem_delta = monitor.measure_operation(
        "5. XWSerializer - Automatic format detection and delegation",
        demo_xw_serializer
    )
    print(f"   🎯 XWSerializer automatically detected JSON format")
    print(f"   👤 Username: {result['username']}")
    # ========================================================================
    # SUMMARY
    # ========================================================================
    print("\n" + "=" * 70)
    print("📊 PERFORMANCE SUMMARY")
    print("=" * 70)
    final_memory = monitor._get_memory_mb()
    total_memory_delta = final_memory - monitor.baseline_memory
    print(f"\n📁 File Information:")
    print(f"   Size: {monitor.format_size(file_size)} ({file_size_gb:.2f} GB)")
    print(f"   Path: {json_file}")
    print(f"\n💾 Memory Usage:")
    print(f"   Baseline: {monitor.baseline_memory:.2f} MB")
    print(f"   Final: {final_memory:.2f} MB")
    print(f"   Delta: {total_memory_delta:+.2f} MB")
    if total_memory_delta < file_size_gb * 1024 * 0.1:  # Less than 10% of file size
        print(f"   ✅ EXCELLENT: Memory efficient (delta < 10% of file size)")
    elif total_memory_delta < file_size_gb * 1024 * 0.5:  # Less than 50%
        print(f"   ✅ GOOD: Memory efficient (delta < 50% of file size)")
    else:
        print(f"   ⚠️  Memory usage is significant")
    print(f"\n✅ All advanced features demonstrated successfully!")
    print(f"   Operations completed efficiently on {file_size_gb:.2f} GB file")
    print(f"   Memory overhead: {total_memory_delta:.2f} MB")
    print("\n" + "=" * 70)
    print("🎉 Demo Complete!")
    print("=" * 70)
if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("\n\n⚠️  Demo interrupted by user")
        sys.exit(1)
    except Exception as e:
        print(f"\n\n❌ Error: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)
