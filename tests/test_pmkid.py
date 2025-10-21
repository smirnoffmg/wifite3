"""
Tests for PMKID capture functionality
"""

import pytest


def test_pmkid_capture_creation():
    """Test PMKIDCapture creation"""
    try:
        import _wifite3

        pmkid_capture = _wifite3.PMKIDCapture(
            ssid="TestNetwork",
            bssid="00:11:22:33:44:55",
            client_mac="aa:bb:cc:dd:ee:ff",
            pmkid="1234567890abcdef1234567890abcdef",
        )

        assert pmkid_capture.ssid == "TestNetwork"
        assert pmkid_capture.bssid == "00:11:22:33:44:55"
        assert pmkid_capture.client_mac == "aa:bb:cc:dd:ee:ff"
        assert pmkid_capture.pmkid == "1234567890abcdef1234567890abcdef"
        assert "WPA*01*" in pmkid_capture.hashcat_format
        assert "TestNetwork" in pmkid_capture.hashcat_format

    except ImportError:
        pytest.skip("Rust module not yet built")


def test_pmkid_hashcat_format():
    """Test PMKID hashcat format generation"""
    try:
        import _wifite3

        pmkid_capture = _wifite3.PMKIDCapture(
            ssid="TestNetwork",
            bssid="00:11:22:33:44:55",
            client_mac="aa:bb:cc:dd:ee:ff",
            pmkid="1234567890abcdef1234567890abcdef",
        )

        hashcat_format = pmkid_capture.get_hashcat_format()
        assert hashcat_format.startswith("WPA*01*")
        assert "1234567890abcdef1234567890abcdef" in hashcat_format
        assert "001122334455" in hashcat_format  # BSSID without colons
        assert "aabbccddeeff" in hashcat_format  # Client MAC without colons
        assert "TestNetwork" in hashcat_format

    except ImportError:
        pytest.skip("Rust module not yet built")


def test_pmkid_summary():
    """Test PMKID capture summary"""
    try:
        import _wifite3

        pmkid_capture = _wifite3.PMKIDCapture(
            ssid="TestNetwork",
            bssid="00:11:22:33:44:55",
            client_mac="aa:bb:cc:dd:ee:ff",
            pmkid="1234567890abcdef1234567890abcdef",
        )

        summary = pmkid_capture.get_summary()
        assert "TestNetwork" in summary
        assert "00:11:22:33:44:55" in summary
        assert "aa:bb:cc:dd:ee:ff" in summary

    except ImportError:
        pytest.skip("Rust module not yet built")


def test_pmkid_capture_functionality():
    """Test PMKID capture functionality"""
    try:
        import _wifite3

        scanner = _wifite3.NetworkScanner("wlan0")

        # Test PMKID capture with short duration
        pmkid_captures = scanner.capture_pmkid(1)  # 1 second capture

        assert isinstance(pmkid_captures, list)

        # If PMKID captures are found, validate structure
        if pmkid_captures:
            for capture in pmkid_captures:
                assert isinstance(capture.ssid, str)
                assert isinstance(capture.bssid, str)
                assert isinstance(capture.client_mac, str)
                assert isinstance(capture.pmkid, str)
                assert isinstance(capture.hashcat_format, str)

                # Validate MAC address format
                assert len(capture.bssid.split(":")) == 6
                assert len(capture.client_mac.split(":")) == 6

                # Validate PMKID format (32 hex characters)
                assert len(capture.pmkid) == 32
                assert all(c in "0123456789abcdef" for c in capture.pmkid.lower())

                # Validate hashcat format
                assert capture.hashcat_format.startswith("WPA*01*")

    except ImportError:
        pytest.skip("Rust module not yet built")
    except RuntimeError as e:
        if "Permission denied" in str(e) or "cannot open" in str(e):
            pytest.skip("PMKID capture requires root permissions")
        else:
            raise


def test_cli_pmkid_option():
    """Test CLI PMKID option"""
    from click.testing import CliRunner

    from wifite3.cli import main

    runner = CliRunner()
    result = runner.invoke(main, ["--pmkid", "--help"])
    # The help should show the PMKID option
    assert result.exit_code == 0


def test_cli_pmkid_duration():
    """Test CLI PMKID with duration option"""
    from click.testing import CliRunner

    from wifite3.cli import main

    runner = CliRunner()
    result = runner.invoke(main, ["--pmkid", "--duration", "5"])
    assert result.exit_code == 0
    assert "Capturing PMKID for 5 seconds" in result.output
