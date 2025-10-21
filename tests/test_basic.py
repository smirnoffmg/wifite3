"""
Basic tests for Wifite3
"""

from click.testing import CliRunner

from wifite3.cli import main


def test_cli_help():
    """Test that CLI shows help"""
    runner = CliRunner()
    result = runner.invoke(main, ["--help"])
    assert result.exit_code == 0
    assert "Wifite3" in result.output


def test_cli_verbose():
    """Test verbose mode"""
    runner = CliRunner()
    result = runner.invoke(main, ["--verbose"])
    assert result.exit_code == 0
    assert "Verbose mode enabled" in result.output


def test_cli_scan():
    """Test scan mode"""
    runner = CliRunner()
    result = runner.invoke(main, ["--scan"])
    assert result.exit_code == 0
    assert "Scanning for networks" in result.output


def test_cli_interface():
    """Test interface option"""
    runner = CliRunner()
    result = runner.invoke(main, ["-i", "wlan0"])
    assert result.exit_code == 0
