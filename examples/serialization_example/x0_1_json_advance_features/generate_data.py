#!/usr/bin/env python3
"""
Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.0.1.389
Generation Date: November 9, 2025

Generate 1GB JSON file for advanced features demo.

This script generates a large JSON file with social media data that can be
used by the benchmark.py demo. The file is saved to the data/ directory.
"""

import os
import sys
import time
import json
from pathlib import Path
from typing import Any
from datetime import datetime, timedelta
import random

# Add src to path for development
xwsystem_root = Path(__file__).parent.parent.parent.parent / "src"
sys.path.insert(0, str(xwsystem_root))

from exonware.xwsystem.io.serialization.formats.text.json import JsonSerializer


class SocialMediaDataGenerator:
    """Generate realistic social media data with JSON references."""
    
    def __init__(self, target_size_gb: float = 1.0):
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
        # For 1GB: Need ~10,000-20,000 users
        estimated_users = max(10000, int(self.target_size_bytes / 100000))  # Conservative estimate
        
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
        
        # Generate users in batches to monitor progress
        batch_size = 1000
        posts_per_user = 10  # Average posts per user
        
        for batch_start in range(0, estimated_users, batch_size):
            batch_end = min(batch_start + batch_size, estimated_users)
            
            for i in range(batch_start, batch_end):
                user_id = self._generate_user_id()
                self.user_ids.append(user_id)
                
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


def main():
    """Generate 1GB JSON file."""
    print("=" * 70)
    print("🚀 Generate 1GB Social Media JSON File")
    print("=" * 70)
    
    # Setup paths
    data_dir = Path(__file__).parent / "data"
    data_dir.mkdir(exist_ok=True)
    json_file = data_dir / "social_media_1gb.json"
    
    # Check if file already exists
    if json_file.exists():
        file_size = json_file.stat().st_size
        file_size_gb = file_size / (1024 ** 3)
        print(f"\n📁 Found existing file: {json_file}")
        print(f"   Size: {file_size_gb:.2f} GB ({file_size:,} bytes)")
        
        response = input("\n   Regenerate file? (y/N): ").strip().lower()
        if response != 'y':
            print("   Using existing file.")
            return
        else:
            print("   Regenerating...")
            json_file.unlink()
    
    # Generate data
    generator = SocialMediaDataGenerator(target_size_gb=1.0)
    data = generator.generate_data()
    
    # Save to file
    print(f"\n💾 Saving to {json_file}...")
    serializer = JsonSerializer()
    save_start = time.time()
    serializer.save_file(data, json_file, indent=2)
    save_time = time.time() - save_start
    
    file_size = json_file.stat().st_size
    file_size_gb = file_size / (1024 ** 3)
    
    print(f"   ✅ Saved in {save_time:.2f} seconds")
    print(f"   📊 File size: {file_size_gb:.2f} GB ({file_size:,} bytes)")
    print(f"   📁 Location: {json_file}")
    
    print("\n" + "=" * 70)
    print("🎉 File generation complete!")
    print("=" * 70)
    print(f"\nYou can now run benchmark.py to test advanced features on this file.")


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("\n\n⚠️  Generation interrupted by user")
        sys.exit(1)
    except Exception as e:
        print(f"\n\n❌ Error: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)

