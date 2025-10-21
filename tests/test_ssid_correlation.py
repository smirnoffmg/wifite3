"""
Tests for SSID correlation functionality in PMKID capture
"""

import pytest


def test_ssid_correlation_cache():
    """Test SSID correlation cache creation and lookup"""
    try:
        import _wifite3

        # Test that we can create a scanner with SSID correlation
        scanner = _wifite3.NetworkScanner("wlan0")

        # Test that the scanner has SSID correlation capability
        # This will be implemented in the Rust code
        assert hasattr(scanner, "get_ssid_cache")

        # Test empty cache initially
        cache = scanner.get_ssid_cache()
        assert isinstance(cache, dict)
        assert len(cache) == 0

    except ImportError:
        pytest.skip("Rust module not yet built")
    except RuntimeError as e:
        if "Interface" in str(e):
            pytest.skip("Network interface not available")


def test_ssid_correlation_during_capture():
    """Test that SSID correlation works during PMKID capture"""
    try:
        import _wifite3

        scanner = _wifite3.NetworkScanner("wlan0")

        # Test that PMKID capture with correlation returns proper SSIDs
        # This test will pass once we implement the correlation logic
        pmkid_captures = scanner.capture_pmkid_with_correlation(1)  # 1 second

        # If we get captures, they should have proper SSIDs (not "Unknown")
        if pmkid_captures:
            for capture in pmkid_captures:
                assert capture.ssid != "Unknown"
                assert len(capture.ssid) > 0
                # SSID should be a reasonable network name
                assert not capture.ssid.startswith("Network_")  # Not mock data

    except ImportError:
        pytest.skip("Rust module not yet built")
    except RuntimeError as e:
        if "Interface" in str(e):
            pytest.skip("Network interface not available")


def test_ssid_cache_management():
    """Test SSID cache management and cleanup"""
    try:
        import _wifite3

        scanner = _wifite3.NetworkScanner("wlan0")

        # Test cache operations
        cache = scanner.get_ssid_cache()
        assert isinstance(cache, dict)

        # Test cache clearing
        scanner.clear_ssid_cache()
        cache_after_clear = scanner.get_ssid_cache()
        assert len(cache_after_clear) == 0

    except ImportError:
        pytest.skip("Rust module not yet built")
    except RuntimeError as e:
        if "Interface" in str(e):
            pytest.skip("Network interface not available")
