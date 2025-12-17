"""
Data Generator for Serialization Examples

Generates sample data for testing serialization formats.

Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.0.1
Generation Date: October 12, 2025
"""

from typing import Any


def generate_sample_data() -> dict[str, Any]:
    """Generate sample data structure for testing"""
    return {
        'users': [
            {
                'id': 1,
                'name': 'Alice',
                'email': 'alice@example.com',
                'age': 30,
                'active': True,
                'roles': ['admin', 'user']
            },
            {
                'id': 2,
                'name': 'Bob',
                'email': 'bob@example.com',
                'age': 25,
                'active': True,
                'roles': ['user']
            },
            {
                'id': 3,
                'name': 'Charlie',
                'email': 'charlie@example.com',
                'age': 35,
                'active': False,
                'roles': ['user', 'moderator']
            }
        ],
        'posts': [
            {
                'id': 1,
                'title': 'Introduction to xwsystem',
                'author_id': 1,
                'content': 'xwsystem is a powerful serialization library...',
                'tags': ['python', 'serialization', 'xwsystem'],
                'likes': 42
            },
            {
                'id': 2,
                'title': 'Advanced Features',
                'author_id': 1,
                'content': 'Learn about get_at, set_at, and streaming...',
                'tags': ['advanced', 'tutorial'],
                'likes': 38
            }
        ],
        'metadata': {
            'version': '1.0',
            'created': '2025-10-12',
            'total_users': 3,
            'total_posts': 2
        }
    }


def generate_large_dataset(num_items: int = 10000) -> dict[str, Any]:
    """Generate large dataset for streaming/performance testing"""
    return {
        'users': [
            {
                'id': i,
                'name': f'User{i}',
                'email': f'user{i}@example.com',
                'age': 20 + (i % 50),
                'active': i % 3 != 0,
                'score': i * 1.5
            }
            for i in range(num_items)
        ],
        'metadata': {
            'total_users': num_items,
            'generated': True
        }
    }


def generate_nested_data() -> dict[str, Any]:
    """Generate deeply nested data for path access testing"""
    return {
        'company': {
            'name': 'eXonware',
            'departments': {
                'engineering': {
                    'manager': 'Alice',
                    'employees': [
                        {'id': 1, 'name': 'Bob', 'title': 'Senior Engineer'},
                        {'id': 2, 'name': 'Charlie', 'title': 'Engineer'}
                    ],
                    'budget': 100000
                },
                'sales': {
                    'manager': 'David',
                    'employees': [
                        {'id': 3, 'name': 'Eve', 'title': 'Sales Manager'},
                        {'id': 4, 'name': 'Frank', 'title': 'Sales Rep'}
                    ],
                    'budget': 50000
                }
            },
            'metadata': {
                'founded': 2025,
                'employees_total': 4
            }
        }
    }

