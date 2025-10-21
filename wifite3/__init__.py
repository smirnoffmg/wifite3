"""
Wifite3 - Modern WiFi penetration testing tool
"""

__version__ = "0.1.0"
__author__ = "Wifite3 Team"
__description__ = "Modern WiFi penetration testing tool with Python + Rust"

from .cli import main

# Import Rust module if available
try:
    import _wifite3

    __all__ = ["main", "_wifite3"]
except ImportError:
    __all__ = ["main"]
