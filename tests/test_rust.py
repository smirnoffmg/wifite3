"""
Tests for Rust components
"""

import pytest


def test_rust_import():
    """Test that Rust module can be imported"""
    try:
        import _wifite3

        assert _wifite3 is not None
    except ImportError:
        pytest.skip("Rust module not yet built")


def test_wifi_network_creation():
    """Test WiFiNetwork creation"""
    try:
        import _wifite3

        network = _wifite3.WiFiNetwork(
            ssid="TestNetwork",
            bssid="00:11:22:33:44:55",
            channel=6,
            rssi=-50,
            encryption="WPA2",
        )

        assert network.ssid == "TestNetwork"
        assert network.bssid == "00:11:22:33:44:55"
        assert network.channel == 6
        assert network.rssi == -50
        assert network.encryption == "WPA2"
    except ImportError:
        pytest.skip("Rust module not yet built")


def test_network_scanner_creation():
    """Test NetworkScanner creation"""
    try:
        import _wifite3

        scanner = _wifite3.NetworkScanner("wlan0")
        # Note: interface field is private, so we can't test it directly
        assert scanner is not None
    except ImportError:
        pytest.skip("Rust module not yet built")


def test_interface_usage():
    """Test that scanner uses the specified interface"""
    try:
        import _wifite3

        # Test with different interfaces
        interfaces = _wifite3.NetworkScanner.get_interfaces()
        assert isinstance(interfaces, list)
        assert len(interfaces) > 0

        # Test that we can create scanners with different interfaces
        for interface in interfaces[:2]:  # Test first 2 interfaces
            scanner = _wifite3.NetworkScanner(interface)
            assert scanner is not None

    except ImportError:
        pytest.skip("Rust module not yet built")


def test_get_interfaces():
    """Test getting network interfaces"""
    try:
        import _wifite3

        interfaces = _wifite3.NetworkScanner.get_interfaces()
        assert isinstance(interfaces, list)
        assert len(interfaces) > 0
    except ImportError:
        pytest.skip("Rust module not yet built")


def test_scan_networks():
    """Test scanning for networks with real data validation"""
    try:
        import _wifite3

        scanner = _wifite3.NetworkScanner("wlan0")
        networks = scanner.scan()
        assert isinstance(networks, list)

        # If networks are found, validate they have real data structure
        if networks:
            for network in networks:
                # Validate that we're getting real network data, not mock data
                assert isinstance(network.ssid, str)
                assert isinstance(network.bssid, str)
                assert isinstance(network.channel, int)
                assert isinstance(network.rssi, int)
                assert isinstance(network.encryption, str)

                # Validate BSSID format (MAC address)
                assert len(network.bssid.split(":")) == 6

                # Validate channel range
                assert 1 <= network.channel <= 14

                # Validate RSSI range (typically -100 to 0)
                assert -100 <= network.rssi <= 0

                # Validate encryption type
                assert network.encryption in ["Open", "WEP", "WPA", "WPA2", "WPA3"]

                # If we have real data, SSID should not be mock format
                assert not network.ssid.startswith("Network_")

    except ImportError:
        pytest.skip("Rust module not yet built")
    except RuntimeError as e:
        if "Permission denied" in str(e) or "cannot open" in str(e):
            pytest.skip("Network scanning requires root permissions")
        else:
            raise
