#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/utils/web.py
"""
Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.1.0.3
Generation Date: January 2025

Web utility functions for XSystem.
"""

from typing import Optional
from urllib.parse import urlparse

# Prevent BeautifulSoup from trying to import lxml (which has Python 2 syntax issues)
# Block lxml import before BeautifulSoup tries to import it
import sys
if 'lxml' not in sys.modules:
    # Create a dummy module to prevent lxml from being imported
    class DummyModule:
        pass
    sys.modules['lxml'] = DummyModule()
    sys.modules['lxml.etree'] = DummyModule()
    sys.modules['lxml.html'] = DummyModule()


def validate_url_accessible(url: str, timeout: int = 10) -> bool:
    """
    Validate that a URL is accessible.
    
    Args:
        url: URL to validate
        timeout: Request timeout in seconds
    
    Returns:
        True if URL is accessible (status 200), False otherwise
    
    Examples:
        >>> validate_url_accessible("https://www.example.com")
        True
        >>> validate_url_accessible("https://invalid-url-that-does-not-exist.com")
        False
    """
    try:
        parsed_url = urlparse(url)
        if not all([parsed_url.scheme in ["http", "https"], parsed_url.netloc]):
            return False
        
        # Optional dependency: requests
        import importlib.util
        _requests_spec = importlib.util.find_spec('requests')
        if _requests_spec is None:
            # If requests is not available, we can't validate
            return False
        import requests
        
        try:
            # First try a HEAD request (more efficient)
            response = requests.head(url, allow_redirects=True, timeout=timeout)
        except requests.RequestException:
            # If HEAD request fails, try a GET request
            response = requests.get(url, allow_redirects=True, timeout=timeout)
        
        # Check if the status code indicates success
        return response.status_code == 200
        
    except (requests.RequestException, Exception):
        return False


def extract_webpage_text(url: str) -> str:
    """
    Extract text content from a webpage.
    
    Args:
        url: URL to extract text from
    
    Returns:
        Extracted text content
    
    Examples:
        >>> text = extract_webpage_text("https://www.example.com")
        >>> len(text) > 0
        True
    """
    from urllib.request import urlopen
    from bs4 import BeautifulSoup
    
    html = urlopen(url).read()
    soup = BeautifulSoup(html, features="html.parser")
    
    # Remove script and style elements
    for script in soup(["script", "style"]):
        script.extract()
    
    # Get text
    text = soup.get_text()
    
    # Break into lines and remove leading/trailing space
    lines = (line.strip() for line in text.splitlines())
    chunks = (phrase.strip() for line in lines for phrase in line.split("  "))
    # Drop blank lines
    return '\n'.join(chunk for chunk in chunks if chunk)


__all__ = [
    'validate_url_accessible',
    'extract_webpage_text',
]
